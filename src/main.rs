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

const INCLUDE_CUSTOM_PORTS_OPTION: &str = "include-custom-ports";
const WORKDIR: &str = "/etc/zabbix";

const WORK_DIR_ARGUMENT: &str = "work-dir";
const NGINX_VHOSTS_PATH: &str = "/etc/nginx/conf.d";
const APACHE_VHOSTS_PATH: &str = "/etc/httpd/conf.d";

const NGINX_VHOSTS_PATH_ARGUMENT: &str = "nginx-vhosts-path";
const APACHE_VHOSTS_PATH_ARGUMENT: &str = "apache-vhosts-path";

const ERROR_EXIT_CODE: i32 = 1;

fn main() {
    let matches = App::new("Site Discovery Flea")
                                    .version("1.1.0")
                                    .author("Eugene Lebedev <duke.tougu@gmail.com>")
                                    .about("Discover site configs for nginx and apache. \
                                            Then generate urls and show output in \
                                            Zabbix Low Level Discovery format")
                                    .arg(
                                        Arg::with_name(WORK_DIR_ARGUMENT)
                                            .short("d")
                                            .help("set working directory")
                                            .long(WORK_DIR_ARGUMENT)
                                            .takes_value(true).required(false)
                                    )
                                    .arg(
                                        Arg::with_name(INCLUDE_CUSTOM_PORTS_OPTION)
                                                .long("include-custom-ports")
                                                .help("include domains with custom ports")
                                    )
                                    .arg(
                                        Arg::with_name(NGINX_VHOSTS_PATH_ARGUMENT)
                                            .short("n")
                                            .help("set nginx vhosts root path")
                                            .long(NGINX_VHOSTS_PATH_ARGUMENT)
                                            .takes_value(true).required(false)
                                    )
                                    .arg(
                                        Arg::with_name(APACHE_VHOSTS_PATH_ARGUMENT)
                                            .short("a")
                                            .help("set apache vhosts root path")
                                            .long(APACHE_VHOSTS_PATH_ARGUMENT)
                                            .takes_value(true).required(false)
                                    )
                                    .get_matches();

    let working_directory: &Path = get_argument_path_value(
        &matches, WORK_DIR_ARGUMENT, WORKDIR);

    env::set_current_dir(&working_directory).expect("unable to set working directory");

    let logging_config = get_logging_config(working_directory.to_str().unwrap());
    log4rs::init_config(logging_config).unwrap();

    let include_custom_domains = matches.occurrences_of(INCLUDE_CUSTOM_PORTS_OPTION) > 0;

    info!("[~] collect virtual hosts..");
    info!("include domains with custom ports: {}", include_custom_domains);
    let mut vhosts: Vec<VirtualHost> = Vec::new();

    let nginx_vhosts_path: &Path = get_argument_path_value(
        &matches, NGINX_VHOSTS_PATH_ARGUMENT, NGINX_VHOSTS_PATH);

    let nginx_vhosts = get_nginx_vhosts(nginx_vhosts_path);

    for nginx_vhost in nginx_vhosts {
        if nginx_vhost.port == DEFAULT_HTTP_PORT {
            if !vector_contains_same_domain_with_ssl(&vhosts, &nginx_vhost.domain) {
                vhosts.push(nginx_vhost)
            }
        } else {

            if nginx_vhost.port != DEFAULT_HTTPS_PORT {
                if include_custom_domains &&
                   !vector_contains_same_domain_with_default_http_port(&vhosts, &nginx_vhost.domain) {
                    vhosts.push(nginx_vhost)
                }

            } else {
                vhosts.push(nginx_vhost)
            }
        }
    }

    let apache_vhosts_path: &Path = get_argument_path_value(
        &matches, APACHE_VHOSTS_PATH_ARGUMENT, APACHE_VHOSTS_PATH);

    let apache_vhosts = get_apache_vhosts(apache_vhosts_path);

    for apache_vhost in apache_vhosts {
        if apache_vhost.port == DEFAULT_HTTP_PORT {
            if !vector_contains_same_domain_with_ssl(&vhosts, &apache_vhost.domain) {
                vhosts.push(apache_vhost)
            }
        } else {
            if apache_vhost.port != DEFAULT_HTTPS_PORT {
                if include_custom_domains &&
                   !vector_contains_same_domain_with_default_http_port(&vhosts, &apache_vhost.domain) {
                    vhosts.push(apache_vhost)
                }

            } else {
                vhosts.push(apache_vhost)
            }
        }
    }

    let sites: Vec<Site> = vhosts.iter().map(|vhost| {
        let url = get_url(&vhost.domain, vhost.port);

        Site {
            name: get_site_name(&vhost.domain, vhost.port),
            url
        }
    }).collect();

    show_low_level_discovery_json(sites);
}

fn get_argument_path_value<'a>(matches: &'a ArgMatches, argument: &str, default_path: &'a str) -> &'a Path {
    let path: &Path = if matches.is_present(argument) {
        let apache_vhosts_path_value = matches.value_of(argument).unwrap();
        Path::new(apache_vhosts_path_value)

    } else { Path::new(default_path) };

    return path;
}

fn get_site_name(domain: &str, port: i32) -> String {
    if port == DEFAULT_HTTP_PORT || port == DEFAULT_HTTPS_PORT {
        String::from(domain)

    } else {
        String::from(format!("{}:{}", domain, port))
    }
}

fn get_nginx_vhosts(nginx_vhosts_path: &Path) -> Vec<VirtualHost> {
    let mut vhosts: Vec<VirtualHost> = Vec::new();

    if nginx_vhosts_path.is_dir() && nginx_vhosts_path.exists() {
        match get_vhost_config_file_list(nginx_vhosts_path) {
            Ok(vhost_files) => {

                for vhost_file in vhost_files {
                    let section_start_regex = get_nginx_vhost_section_start_regex();
                    let port_search_regex = get_nginx_vhost_port_regex();
                    let domain_search_regex = get_domain_search_regex_for_nginx_vhost();

                    let vhost_file_path = vhost_file.as_path();

                    let nginx_vhosts = get_virtual_hosts_from_file(
                        vhost_file_path,
                        section_start_regex,
                        port_search_regex,
                        domain_search_regex
                    );

                    for nginx_vhost in nginx_vhosts {
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

    return vhosts
}

fn get_apache_vhosts(vhosts_path: &Path) -> Vec<VirtualHost> {
    let mut vhosts: Vec<VirtualHost> = Vec::new();

    if vhosts_path.is_dir() && vhosts_path.exists() {
        match get_vhost_config_file_list(vhosts_path) {
            Ok(vhost_files) => {

                for vhost_file in vhost_files {
                    let vhost_file_path = vhost_file.as_path();

                    let section_start_regex = get_apache_vhost_section_start_regex();
                    let port_search_regex = get_apache_vhost_port_regex();
                    let domain_search_regex = get_domain_search_regex_for_apache_vhost();

                    let apache_vhosts = get_virtual_hosts_from_file(
                        vhost_file_path,
                        section_start_regex,
                        port_search_regex,
                        domain_search_regex
                    );

                    for apache_vhost in apache_vhosts {
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

    return vhosts
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

#[derive(Clone, Serialize)]
struct Site {
    #[serde(rename(serialize = "{#NAME}"))]
    pub name: String,
    #[serde(rename(serialize = "{#URL}"))]
    pub url: String
}

fn show_low_level_discovery_json(sites: Vec<Site>) {
    let json_structure = json!({"data": sites});
    let json = serde_json::to_string(&json_structure).unwrap();
    println!("{}", json);
}
