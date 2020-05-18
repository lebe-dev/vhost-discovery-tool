#[macro_use]
extern crate log;
extern crate log4rs;
extern crate serde_json;

use std::env;
use std::path::Path;
use std::process::exit;

use clap::{App, Arg, ArgMatches};
use serde::Serialize;
use serde_json::json;

use crate::logging::logging::get_logging_config;
use crate::webserver::webserver::{get_apache_vhost_port_regex, get_apache_vhost_section_start_regex, get_domain_search_regex_for_apache_vhost, get_domain_search_regex_for_nginx_vhost, get_nginx_vhost_port_regex, get_nginx_vhost_section_start_regex, get_vhost_config_file_list, get_virtual_hosts_from_file, VirtualHost};

mod logging;

mod main_tests;

mod webserver;
mod webserver_tests;

const DEFAULT_HTTP_PORT: i32 = 80;
const DEFAULT_HTTPS_PORT: i32 = 443;

const INCLUDE_DOMAINS_WITH_WWW: &str = "include-www";
const INCLUDE_CUSTOM_PORTS_OPTION: &str = "include-custom-ports";

const WWW_SEARCH_PATTERN: &str = "www.";

const WORKDIR: &str = "/etc/zabbix";

const WORK_DIR_ARGUMENT: &str = "work-dir";
const WORK_DIR_SHORT_ARGUMENT: &str = "d";

const NGINX_VHOSTS_PATH: &str = "/etc/nginx/conf.d";
const APACHE_VHOSTS_PATH: &str = "/etc/httpd/conf.d";

const NGINX_VHOSTS_PATH_ARGUMENT: &str = "nginx-vhosts-path";
const NGINX_VHOSTS_PATH_SHORT_ARGUMENT: &str = "n";
const APACHE_VHOSTS_PATH_ARGUMENT: &str = "apache-vhosts-path";
const APACHE_VHOSTS_PATH_SHORT_ARGUMENT: &str = "a";

const USE_DATA_PROPERTY_ARGUMENT: &str = "use-data-property";

const LOG_LEVEL_ARGUMENT: &str = "log-level";
const LOG_LEVEL_DEFAULT_VALUE: &str = "info";

const ERROR_EXIT_CODE: i32 = 1;

fn main() {
    let matches = App::new("Site Discovery Flea")
        .version("1.3.2")
        .author("Eugene Lebedev <duke.tougu@gmail.com>")
        .about("Discover site configs for nginx and apache. \
                                            Then generate urls and show output in \
                                            Zabbix Low Level Discovery format")
        .arg(
            Arg::with_name(WORK_DIR_ARGUMENT)
                .short(WORK_DIR_SHORT_ARGUMENT)
                .help("set working directory")
                .long(WORK_DIR_ARGUMENT)
                .takes_value(true).required(false)
        )
        .arg(
            Arg::with_name(INCLUDE_DOMAINS_WITH_WWW)
                .long(INCLUDE_DOMAINS_WITH_WWW)
                .help("include domains with www")
        )
        .arg(
            Arg::with_name(INCLUDE_CUSTOM_PORTS_OPTION)
                .long(INCLUDE_CUSTOM_PORTS_OPTION)
                .help("include domains with custom ports")
        )
        .arg(
            Arg::with_name(NGINX_VHOSTS_PATH_ARGUMENT)
                .short(NGINX_VHOSTS_PATH_SHORT_ARGUMENT)
                .help("set nginx vhosts root path")
                .long(NGINX_VHOSTS_PATH_ARGUMENT)
                .takes_value(true).required(false)
        )
        .arg(
            Arg::with_name(APACHE_VHOSTS_PATH_ARGUMENT)
                .short(APACHE_VHOSTS_PATH_SHORT_ARGUMENT)
                .help("set apache vhosts root path")
                .long(APACHE_VHOSTS_PATH_ARGUMENT)
                .takes_value(true).required(false)
        )
        .arg(
            Arg::with_name(USE_DATA_PROPERTY_ARGUMENT)
                .help("use low level discovery format with 'data' property. example: { \"data\": [] }")
                .long(USE_DATA_PROPERTY_ARGUMENT)
                .takes_value(false).required(false)
        )
        .arg(
            Arg::with_name(LOG_LEVEL_ARGUMENT)
                .help("set logging level. possible values: debug, info, error, warn, trace")
                .long(LOG_LEVEL_ARGUMENT)
                .case_insensitive(true)
                .takes_value(true).required(false)
                .default_value(LOG_LEVEL_DEFAULT_VALUE)
        )
        .get_matches();

    let working_directory: &Path = get_argument_path_value(
        &matches, WORK_DIR_ARGUMENT, WORK_DIR_SHORT_ARGUMENT, WORKDIR);

    debug!("working directory '{}'", &working_directory.display());

    env::set_current_dir(&working_directory).expect("unable to set working directory");

    let logging_level: &str = if matches.is_present(LOG_LEVEL_ARGUMENT) {
        matches.value_of(LOG_LEVEL_ARGUMENT).unwrap()
    } else { LOG_LEVEL_DEFAULT_VALUE };

    let logging_config = get_logging_config(logging_level);
    log4rs::init_config(logging_config).unwrap();

    let include_domains_with_www = matches.occurrences_of(INCLUDE_DOMAINS_WITH_WWW) > 0;
    let include_custom_domains = matches.occurrences_of(INCLUDE_CUSTOM_PORTS_OPTION) > 0;

    info!("[~] collect virtual hosts..");
    info!("- include domains with custom ports: {}", include_custom_domains);
    let mut vhosts: Vec<VirtualHost> = Vec::new();

    let nginx_vhosts_path: &Path = get_argument_path_value(
        &matches, NGINX_VHOSTS_PATH_ARGUMENT,
        NGINX_VHOSTS_PATH_SHORT_ARGUMENT, NGINX_VHOSTS_PATH);

    debug!("- nginx vhosts root: '{}'", nginx_vhosts_path.display());

    let nginx_vhosts = get_nginx_vhosts(nginx_vhosts_path);

    for nginx_vhost in nginx_vhosts {
        if nginx_vhost.port == DEFAULT_HTTP_PORT {
            if !vector_contains_same_domain_with_ssl(&vhosts, &nginx_vhost.domain) {
                debug!("+ add vhost '{}'", nginx_vhost.to_string());
                vhosts.push(nginx_vhost)
            }
        } else {
            if nginx_vhost.port != DEFAULT_HTTPS_PORT {
                if include_custom_domains &&
                    !vector_contains_same_domain_with_default_http_port(&vhosts, &nginx_vhost.domain) {
                    debug!("+ add vhost '{}'", nginx_vhost.to_string());
                    vhosts.push(nginx_vhost)
                }
            } else {
                debug!("+ add vhost '{}'", nginx_vhost.to_string());
                vhosts.push(nginx_vhost)
            }
        }
    }

    let apache_vhosts_path: &Path = get_argument_path_value(
        &matches, APACHE_VHOSTS_PATH_ARGUMENT,
        APACHE_VHOSTS_PATH_SHORT_ARGUMENT, APACHE_VHOSTS_PATH);

    debug!("apache vhosts root: '{}'", apache_vhosts_path.display());

    let apache_vhosts = get_apache_vhosts(apache_vhosts_path);

    for apache_vhost in apache_vhosts {
        if apache_vhost.port == DEFAULT_HTTP_PORT {
            if !vector_contains_same_domain_with_ssl(&vhosts, &apache_vhost.domain) {
                debug!("+ add vhost '{}'", apache_vhost.to_string());
                vhosts.push(apache_vhost)
            }
        } else {
            if apache_vhost.port != DEFAULT_HTTPS_PORT {
                if include_custom_domains &&
                    !vector_contains_same_domain_with_default_http_port(&vhosts, &apache_vhost.domain) {
                    debug!("+ found vhost '{}'", apache_vhost.to_string());
                    vhosts.push(apache_vhost)
                }
            } else {
                debug!("+ found vhost '{}'", apache_vhost.to_string());
                vhosts.push(apache_vhost)
            }
        }
    }

    let sites: Vec<Site> = get_sites_vector_from_vhosts(vhosts, include_domains_with_www);

    let json;

    if matches.is_present(USE_DATA_PROPERTY_ARGUMENT) {
        json = get_low_level_discovery_json_with_data_property(sites);
    } else {
        json = get_low_level_discovery_json(sites);
    };

    println!("{}", json);
}

fn get_sites_vector_from_vhosts(vhosts: Vec<VirtualHost>, include_domains_with_www: bool) -> Vec<Site> {
    let sites: Vec<Site> = vhosts.iter()
        .filter(|vhost| {
        let domain_in_lowercase = vhost.domain.to_lowercase();

        if domain_in_lowercase.starts_with(WWW_SEARCH_PATTERN) && !include_domains_with_www {
            false

        } else {
            true
        }

    }).map(|vhost| {
        let url = get_url(&vhost.domain, vhost.port);

        Site {
            name: get_site_name(&vhost.domain, vhost.port),
            url,
        }
    }).collect();

    return sites;
}

fn get_argument_path_value<'a>(matches: &'a ArgMatches, long_argument: &str,
                               short_argument: &str, default_path: &'a str) -> &'a Path {
    let mut path: &Path = Path::new(default_path);

    if matches.is_present(long_argument) {
        let vhosts_path_value = matches.value_of(long_argument).unwrap();
        path = Path::new(vhosts_path_value)
    } else {
        if matches.is_present(short_argument) {
            let vhosts_path_value = matches.value_of(short_argument).unwrap();
            path = Path::new(vhosts_path_value)
        }
    }

    return path;
}

fn get_site_name(domain: &str, port: i32) -> String {
    if port == DEFAULT_HTTP_PORT {
        String::from(format!("{}_http", domain))
    } else if port == DEFAULT_HTTPS_PORT {
        String::from(domain)
    } else {
        String::from(format!("{}:{}", domain, port))
    }
}

fn get_nginx_vhosts(nginx_vhosts_path: &Path) -> Vec<VirtualHost> {
    debug!("get virtual hosts from nginx configs");
    debug!("configs path '{}'", nginx_vhosts_path.display());

    let mut vhosts: Vec<VirtualHost> = Vec::new();

    if nginx_vhosts_path.is_dir() && nginx_vhosts_path.exists() {
        match get_vhost_config_file_list(nginx_vhosts_path) {
            Ok(vhost_files) => {
                for vhost_file in vhost_files {
                    debug!("analyze vhost file '{}'", vhost_file.display());

                    let section_start_regex = get_nginx_vhost_section_start_regex();
                    let port_search_regex = get_nginx_vhost_port_regex();
                    let domain_search_regex = get_domain_search_regex_for_nginx_vhost();

                    let vhost_file_path = vhost_file.as_path();

                    let nginx_vhosts = get_virtual_hosts_from_file(
                        vhost_file_path,
                        section_start_regex,
                        port_search_regex,
                        domain_search_regex,
                    );

                    for nginx_vhost in nginx_vhosts {
                        debug!("{}", nginx_vhost.to_string());
                        vhosts.push(nginx_vhost);
                    }
                }
            }
            Err(_error) => {
                error!("unable to get vhost file list from '{}', \
                       possible reason: lack of permissions", nginx_vhosts_path.display());
                exit(ERROR_EXIT_CODE)
            }
        }
    }

    return vhosts;
}

fn get_apache_vhosts(vhosts_path: &Path) -> Vec<VirtualHost> {
    debug!("get virtual hosts from apache configs");
    debug!("configs path '{}'", vhosts_path.display());

    let mut vhosts: Vec<VirtualHost> = Vec::new();

    if vhosts_path.is_dir() && vhosts_path.exists() {
        match get_vhost_config_file_list(vhosts_path) {
            Ok(vhost_files) => {
                for vhost_file in vhost_files {
                    let vhost_file_path = vhost_file.as_path();

                    debug!("analyze vhost file '{}'", vhost_file_path.display());

                    let section_start_regex = get_apache_vhost_section_start_regex();
                    let port_search_regex = get_apache_vhost_port_regex();
                    let domain_search_regex = get_domain_search_regex_for_apache_vhost();

                    let apache_vhosts = get_virtual_hosts_from_file(
                        vhost_file_path,
                        section_start_regex,
                        port_search_regex,
                        domain_search_regex,
                    );

                    for apache_vhost in apache_vhosts {
                        debug!("{}", apache_vhost.to_string());
                        vhosts.push(apache_vhost);
                    }
                }
            }
            Err(_) => {
                error!("unable to get vhost file list from '{}', \
                        possible reason: lack of permissions", vhosts_path.display());
                exit(ERROR_EXIT_CODE)
            }
        }
    }

    return vhosts;
}

fn get_url(domain: &str, vhost_port: i32) -> String {
    if vhost_port == DEFAULT_HTTP_PORT {
        String::from(format!("http://{}", domain))
    } else if vhost_port == DEFAULT_HTTPS_PORT {
        String::from(format!("https://{}", domain))
    } else {
        String::from(format!("http://{}:{}", domain, vhost_port))
    }
}

fn vector_contains_same_domain_with_ssl(vhosts: &Vec<VirtualHost>, domain: &String) -> bool {
    let mut result = false;

    let vhost_found = vhosts.iter().find(
        |vhost| &vhost.domain == domain && vhost.port == DEFAULT_HTTPS_PORT
    ).is_some();

    if vhost_found {
        result = true;
    }

    result
}

fn vector_contains_same_domain_with_default_http_port(vhosts: &Vec<VirtualHost>,
                                                      domain: &String) -> bool {
    let mut result = false;

    let vhost_found = vhosts.iter().find(
        |vhost| &vhost.domain == domain && vhost.port == DEFAULT_HTTP_PORT
    ).is_some();

    if vhost_found {
        result = true;
    }

    result
}

fn get_low_level_discovery_json(sites: Vec<Site>) -> String {
    let json_structure = json!(sites);
    let json = serde_json::to_string(&json_structure).unwrap();
    return json;
}

fn get_low_level_discovery_json_with_data_property(sites: Vec<Site>) -> String {
    let json_structure = json!({"data": sites});
    let json = serde_json::to_string(&json_structure).unwrap();
    return json;
}

#[derive(Clone, Serialize)]
struct Site {
    #[serde(rename(serialize = "{#NAME}"))]
    pub name: String,
    #[serde(rename(serialize = "{#URL}"))]
    pub url: String,
}
