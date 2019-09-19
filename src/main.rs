#[macro_use]
extern crate log;
extern crate log4rs;

use std::env;
use std::path::Path;
use std::process::exit;

use serde_json::json;

use crate::logging::logging::get_logging_config;
use crate::webserver::webserver::{get_apache_vhost_port_regex, get_apache_vhost_section_start_regex, get_domain_search_regex_for_apache_vhost, get_domain_search_regex_for_nginx_vhost, get_nginx_vhost_port_regex, get_nginx_vhost_section_start_regex, get_vhost_config_file_list, get_virtual_hosts_from_file, VirtualHost};

mod logging;

mod webserver;
mod webserver_tests;

const VERSION: &str = "1.0.0";

const VERTICAL_LINE: &str = "-----------------------------------";

const WITHOUT_ARGUMENTS: usize = 1;
const ONE_ARGUMENT: usize = 2;

const ERROR_EXIT_CODE: i32 = 1;

fn main() {
    let logging_config = get_logging_config();
    log4rs::init_config(logging_config).unwrap();

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
        let mut vhosts: Vec<VirtualHost> = Vec::new();

        // APACHE

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
                Err(error) => {
                    error!("unable to get vhost file list from '{}', \
                           possible reason: lack of permissions", apache_vhost_base_path.display());
                    exit(ERROR_EXIT_CODE)
                }
            }
        }

        // NGINX

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
                Err(error) => {
                    error!("unable to get vhost file list from '{}', \
                           possible reason: lack of permissions", nginx_vhost_base_path.display());
                    exit(ERROR_EXIT_CODE)
                }
            }
        }

        let urls: Vec<String> = vhosts.iter().map(|vhost| get_url(&vhost.domain, vhost.port)).collect();

        show_low_level_discovery_json(urls);

    } else {
        show_usage();
        exit(ERROR_EXIT_CODE);
    }
}

fn get_url(domain: &str, vhost_port: i32) -> String {
    if vhost_port == 80 {
        String::from(format!("http://{}", domain))

    } else if vhost_port == 443 {
        String::from(format!("https://{}", domain))

    } else {
        String::from(format!("http://{}:{}", domain, vhost_port))
    }
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

fn show_low_level_discovery_json(urls: Vec<String>) {
    let json_structure = json!({"data": urls});

    let json = serde_json::to_string(&json_structure).unwrap();

    println!("{}", json);
}