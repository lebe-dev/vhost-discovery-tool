#[macro_use]
extern crate log;
extern crate log4rs;
extern crate serde_json;

use std::env;
use std::path::Path;
use std::process::exit;

use clap::{App, Arg};
use serde::Serialize;
use serde_json::json;

use crate::logging::logging::get_logging_config;
use crate::webserver::webserver::{get_apache_vhost_port_regex, get_apache_vhost_section_start_regex, get_domain_search_regex_for_apache_vhost, get_domain_search_regex_for_nginx_vhost, get_nginx_vhost_port_regex, get_nginx_vhost_section_start_regex, get_vhost_config_file_list, get_virtual_hosts_from_file, VirtualHost};

mod logging;

mod main_tests;

mod webserver;
mod webserver_tests;

const VERSION: &str = "1.0.0";

const VERTICAL_LINE: &str = "-----------------------------------";

const WITHOUT_ARGUMENTS: usize = 1;
const ONE_ARGUMENT: usize = 2;

const DEFAULT_HTTP_PORT: i32 = 80;
const DEFAULT_HTTPS_PORT: i32 = 443;

const INCLUDE_CUSTOM_PORTS_OPTION: &str = "include-custom-ports";

const ERROR_EXIT_CODE: i32 = 1;

fn main() {
    let logging_config = get_logging_config();
    log4rs::init_config(logging_config).unwrap();

    let matches = App::new("Site Discovery Flea")
                                    .version("1.0.0")
                                    .author("Eugene Lebedev <duke.tougu@gmail.com>")
                                    .about("Discover site configs for nginx and apache. \
                                            Then generate urls and show output in \
                                            Zabbix Low Level Discovery format")
                                    .arg(
                                        Arg::with_name(INCLUDE_CUSTOM_PORTS_OPTION)
                                                .help("include domains with custom ports")
                                    )
                                    .get_matches();

    let include_custom_domains = matches.occurrences_of(INCLUDE_CUSTOM_PORTS_OPTION) > 0;

    let args: Vec<String> = env::args().collect();

    if args.len() == ONE_ARGUMENT {
        let first_argument = &args[1].to_lowercase();

        if is_version_command(first_argument) { show_version() }
        else if is_help_command(first_argument) { show_usage() }
        else {
            show_usage();
            exit(ERROR_EXIT_CODE);
        }

    } else if args.len() == WITHOUT_ARGUMENTS {
        info!("[~] collect virtual hosts..");
        info!("include domains with custom ports: {}", include_custom_domains);
        let mut vhosts: Vec<VirtualHost> = Vec::new();

        let nginx_vhosts = get_nginx_vhosts();

        for nginx_vhost in nginx_vhosts {
            if nginx_vhost.port == DEFAULT_HTTP_PORT {
                if !vector_contains_same_domain_with_ssl(&vhosts, &nginx_vhost.domain) {
                    vhosts.push(nginx_vhost)
                }
            } else {

                if include_custom_domains && nginx_vhost.port != DEFAULT_HTTPS_PORT {
                    if !vector_contains_same_domain_with_default_http_port(&vhosts, &nginx_vhost.domain) {
                        vhosts.push(nginx_vhost)
                    }

                } else {
                    vhosts.push(nginx_vhost)
                }
            }
        }

        let apache_vhosts = get_apache_vhosts();

        for apache_vhost in apache_vhosts {
            if apache_vhost.port == DEFAULT_HTTP_PORT {
                if !vector_contains_same_domain_with_ssl(&vhosts, &apache_vhost.domain) {
                    vhosts.push(apache_vhost)
                }
            } else {
                if include_custom_domains && apache_vhost.port != DEFAULT_HTTPS_PORT {
                    if !vector_contains_same_domain_with_default_http_port(&vhosts, &apache_vhost.domain) {
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

    } else {
        show_usage();
        exit(ERROR_EXIT_CODE);
    }
}

fn get_site_name(domain: &str, port: i32) -> String {
    if port == DEFAULT_HTTP_PORT || port == DEFAULT_HTTPS_PORT {
        String::from(domain)

    } else {
        String::from(format!("{}:{}", domain, port))
    }
}

fn get_nginx_vhosts() -> Vec<VirtualHost> {
    let mut vhosts: Vec<VirtualHost> = Vec::new();

    let nginx_vhost_base_path = Path::new("/etc/nginx/conf.d");

    if nginx_vhost_base_path.is_dir() && nginx_vhost_base_path.exists() {
        match get_vhost_config_file_list(nginx_vhost_base_path) {
            Ok(apache_vhost_files) => {

                for vhost_file in apache_vhost_files {
                    let vhost_file_path = vhost_file.as_path();

                    let section_start_regex = get_nginx_vhost_section_start_regex();
                    let port_search_regex = get_nginx_vhost_port_regex();
                    let domain_search_regex = get_domain_search_regex_for_nginx_vhost();

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
                           possible reason: lack of permissions", nginx_vhost_base_path.display());
                exit(ERROR_EXIT_CODE)
            }
        }
    }

    return vhosts
}

fn get_apache_vhosts() -> Vec<VirtualHost> {
    let mut vhosts: Vec<VirtualHost> = Vec::new();

    let apache_vhost_base_path = Path::new("/etc/httpd/conf.d");

    if apache_vhost_base_path.is_dir() && apache_vhost_base_path.exists() {
        match get_vhost_config_file_list(apache_vhost_base_path) {
            Ok(apache_vhost_files) => {

                for vhost_file in apache_vhost_files {
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
                        possible reason: lack of permissions", apache_vhost_base_path.display());
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

fn is_version_command(arg: &str) -> bool {
    arg == "-v" || arg == "--version"
}

fn is_help_command(arg: &str) -> bool {
    arg == "-h" || arg == "--help"
}

fn show_usage() {
    println!("{}", VERTICAL_LINE);
    println!(" SITE DISCOVERY FLEA v{}", VERSION);
    println!("{}", VERTICAL_LINE);
    println!("Discover site configs for nginx and apache. Then generate urls and show output in Zabbix Low Level Discovery format");
    println!();
    println!("usage:");
    println!("<without arguments> - discover and generate site urls");
    println!("-v - show version");
}

fn show_version() { println!("{}", VERSION) }

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