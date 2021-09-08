#[macro_use]
extern crate log;
extern crate log4rs;
extern crate serde_json;

use std::env;
use std::path::Path;

use clap::{App, Arg, ArgMatches};
use serde_json::json;

use crate::apache::apache::get_apache_vhosts;
use crate::domain::domain::{Site, VirtualHost};
use crate::filter::filter::{filter_vhosts, filter_by_domain_masks};
use crate::logging::logging::get_logging_config;
use crate::nginx::nginx::get_nginx_vhosts;
use crate::site::site::get_sites_from_vhosts;

mod logging;

mod main_tests;

mod webserver;
mod webserver_tests;
mod nginx;

mod domain;

mod apache;

mod site;

mod filter;
mod filter_tests;
mod site_tests;
mod nginx_tests;
mod apache_tests;
mod test_utils;
mod test_samples;

const DEFAULT_HTTP_PORT: i32 = 80;
const DEFAULT_HTTPS_PORT: i32 = 443;

const INCLUDE_DOMAINS_WITH_WWW: &str = "include-www";
const INCLUDE_CUSTOM_PORTS_OPTION: &str = "include-custom-ports";

const DOMAIN_IGNORE_MASKS_OPTION: &str = "ignore-by-masks";

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
    let matches = App::new("Virtual Host Discovery Tool")
        .version("1.4.2")
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
        ).arg(
            Arg::with_name(DOMAIN_IGNORE_MASKS_OPTION)
                .short(DOMAIN_IGNORE_MASKS_OPTION)
                .help("set ignore masks for domains. Use ',' char as value separator. Example: house,ads")
                .long(DOMAIN_IGNORE_MASKS_OPTION)
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

    init_logging(&matches);
    init_working_dir(&matches);

    let include_domains_with_www = matches.occurrences_of(INCLUDE_DOMAINS_WITH_WWW) > 0;
    let include_custom_domains = matches.occurrences_of(INCLUDE_CUSTOM_PORTS_OPTION) > 0;

    let domain_ignore_masks_row: &str = if matches.is_present(DOMAIN_IGNORE_MASKS_OPTION) {
        matches.value_of(DOMAIN_IGNORE_MASKS_OPTION).unwrap()
    } else { "" };

    let domain_ignore_masks: Vec<&str> = domain_ignore_masks_row.split(",").collect();

    info!("[~] collect virtual hosts..");
    info!("- include domains with custom ports: {}", include_custom_domains);
    let mut vhosts: Vec<VirtualHost> = Vec::new();

    let nginx_vhosts_path: &Path = get_nginx_vhosts_path(&matches);
    debug!("- nginx vhosts root: '{}'", nginx_vhosts_path.display());

    let mut nginx_vhosts = get_nginx_vhosts(nginx_vhosts_path);
    vhosts.append(&mut nginx_vhosts);

    let apache_vhosts_path: &Path = get_apache_vhosts_path(&matches);
    debug!("apache vhosts root: '{}'", apache_vhosts_path.display());

    let mut apache_vhosts = get_apache_vhosts(apache_vhosts_path);
    vhosts.append(&mut apache_vhosts);

    let mut filtered_vhosts = filter_vhosts(&vhosts, include_custom_domains);
    filtered_vhosts = filter_by_domain_masks(&filtered_vhosts, &domain_ignore_masks);

    let sites: Vec<Site> = get_sites_from_vhosts(filtered_vhosts, include_domains_with_www);

    let json;

    if matches.is_present(USE_DATA_PROPERTY_ARGUMENT) {
        json = get_low_level_discovery_json_with_data_property(sites);

    } else {
        json = get_low_level_discovery_json(sites);
    };

    println!("{}", json);
}

fn init_logging(matches: &ArgMatches) {
    let logging_level: &str = if matches.is_present(LOG_LEVEL_ARGUMENT) {
        matches.value_of(LOG_LEVEL_ARGUMENT).unwrap()
    } else { LOG_LEVEL_DEFAULT_VALUE };

    let logging_config = get_logging_config(logging_level);
    log4rs::init_config(logging_config).unwrap();
}

fn init_working_dir(matches: &ArgMatches) {
    let working_directory: &Path = get_argument_path_value(
        &matches, WORK_DIR_ARGUMENT, WORK_DIR_SHORT_ARGUMENT, WORKDIR);

    debug!("working directory '{}'", &working_directory.display());

    env::set_current_dir(&working_directory).expect("unable to set working directory");
}

fn get_argument_path_value<'a>(matches: &'a ArgMatches, long_argument: &str,
                               short_argument: &str, default_path: &'a str) -> &'a Path {
    let mut path: &Path = Path::new(default_path);

    if matches.is_present(long_argument) {
        let vhosts_path_value = matches.value_of(long_argument)
                                             .unwrap_or(default_path);
        path = Path::new(vhosts_path_value)

    } else {
        if matches.is_present(short_argument) {
            let vhosts_path_value = matches.value_of(short_argument)
                                                 .unwrap_or(default_path);
            path = Path::new(vhosts_path_value)
        }
    }

    return path;
}

fn get_nginx_vhosts_path<'a>(matches: &'a ArgMatches) -> &'a Path {
    get_argument_path_value(&matches, NGINX_VHOSTS_PATH_ARGUMENT,
        NGINX_VHOSTS_PATH_SHORT_ARGUMENT, NGINX_VHOSTS_PATH)
}

fn get_apache_vhosts_path<'a>(matches: &'a ArgMatches) -> &'a Path {
    get_argument_path_value(&matches, APACHE_VHOSTS_PATH_ARGUMENT,
                            APACHE_VHOSTS_PATH_SHORT_ARGUMENT, APACHE_VHOSTS_PATH)
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
