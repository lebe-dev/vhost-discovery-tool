#[macro_use]
extern crate log;
extern crate log4rs;
extern crate serde_json;

use std::env;
use std::path::Path;

use clap::{App, Arg, ArgMatches};
use serde_json::json;

use crate::apache::get_apache_discovery_config;
use crate::cli::get_app_config;
use crate::domain::{Site, VirtualHost};
use crate::filter::{filter_by_domain_masks, filter_vhosts};
use crate::logging::get_logging_config;
use crate::nginx::get_nginx_discovery_config;
use crate::site::get_domains_from_vhosts;
use crate::webserver::get_vhosts;

mod logging;

mod webserver;

mod vhost;

mod nginx;

mod domain;

mod apache;

mod site;

mod filter;

mod cli;

#[cfg(test)]
mod test_utils;

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

const VHOST_FILE_EXTENSIONS_ARGUMENT: &str = "file-extensions";
const VHOST_FILE_EXTENSIONS_DEFAULT_VALUE: &str = ".conf,.vhost";

const NGINX_VHOSTS_PATH_ARGUMENT: &str = "nginx-vhosts-path";
const NGINX_VHOSTS_PATH_SHORT_ARGUMENT: &str = "n";

const RECURSIVE_OPTION: &str = "r";

const APACHE_VHOSTS_PATH_ARGUMENT: &str = "apache-vhosts-path";
const APACHE_VHOSTS_PATH_SHORT_ARGUMENT: &str = "a";

const USE_DATA_PROPERTY_ARGUMENT: &str = "use-data-property";

const LOG_LEVEL_ARGUMENT: &str = "log-level";
const LOG_LEVEL_DEFAULT_VALUE: &str = "info";

fn main() {
    let matches = App::new("Virtual Host Discovery Tool")
        .version("1.5.4")
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
            Arg::with_name(VHOST_FILE_EXTENSIONS_ARGUMENT)
                .long(VHOST_FILE_EXTENSIONS_ARGUMENT)
                .help("specify file extensions, default: .conf and .vhost")
        )
        .arg(
            Arg::with_name(RECURSIVE_OPTION)
                .short(RECURSIVE_OPTION)
                .help("scan vhost-files in subdirectories")
                .takes_value(false).required(false)
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
                .help("set ignore masks for domains. Use ',' \
                        char as value separator. Example: house,ads")
                .long(DOMAIN_IGNORE_MASKS_OPTION)
                .default_value("^localhost$")
                .takes_value(true).required(false)
        )
        .arg(
            Arg::with_name(USE_DATA_PROPERTY_ARGUMENT)
                .help("use low level discovery format with 'data' \
                        property. example: { \"data\": [] }")
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

    let app_config = get_app_config(&matches);

    info!("[~] collect virtual hosts..");
    info!("- include domains with custom ports: {}", &app_config.include_custom_domains);
    let mut vhosts: Vec<VirtualHost> = Vec::new();

    let nginx_vhosts_path: &Path = get_nginx_vhosts_path(&matches);
    debug!("- nginx vhosts root: '{}'", nginx_vhosts_path.display());

    let nginx_discovery_config = get_nginx_discovery_config(
        app_config.recursive_mode, &app_config.vhost_file_extensions);
    let mut nginx_vhosts = get_vhosts(nginx_vhosts_path, &nginx_discovery_config)
        .expect("couldn't get vhosts from nginx");

    debug!("nginx vhosts collected:");
    debug!("{:?}", nginx_vhosts);

    vhosts.append(&mut nginx_vhosts);

    let apache_vhosts_path: &Path = get_apache_vhosts_path(&matches);
    debug!("apache vhosts root: '{}'", apache_vhosts_path.display());

    let apache_discovery_config = get_apache_discovery_config(
        app_config.recursive_mode, &app_config.vhost_file_extensions);
    let mut apache_vhosts = get_vhosts(apache_vhosts_path, &apache_discovery_config)
        .expect("couldn't get vhosts from apache");

    debug!("apache vhosts collected:");
    debug!("{:?}", apache_vhosts);
    vhosts.append(&mut apache_vhosts);

    let mut filtered_vhosts = filter_vhosts(&vhosts, app_config.include_custom_domains);
    filtered_vhosts = filter_by_domain_masks(&filtered_vhosts, &app_config.domain_ignore_masks);

    let sites: Vec<Site> = get_domains_from_vhosts(filtered_vhosts, app_config.include_domains_with_www);

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

    env::set_current_dir(&working_directory).expect("couldn't set working directory");
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

#[cfg(test)]
mod main_tests {
    use crate::{DEFAULT_HTTP_PORT, DEFAULT_HTTPS_PORT, get_low_level_discovery_json, get_low_level_discovery_json_with_data_property};
    use crate::domain::{Site, VirtualHost};
    use crate::site::{get_domains_from_vhosts, get_url};

    const CUSTOM_VHOST_PORT: i32 = 5382;

    #[test]
    fn get_url_should_return_url_with_https_for_443_port() {
        let domain = "quarkoman.com";
        let expected_url = format!("https://{}", domain);

        assert_eq!(get_url(domain, DEFAULT_HTTPS_PORT), expected_url)
    }

    #[test]
    fn get_url_should_return_url_without_port_for_default_http_port() {
        let domain = "quarkoman.com";
        let expected_url = format!("http://{}", domain);

        assert_eq!(get_url(domain, DEFAULT_HTTP_PORT), expected_url)
    }

    #[test]
    fn get_url_should_return_url_with_port_when_custom_port_provided() {
        let domain = "quarkoman.com";
        let expected_url = format!("http://{}:{}", domain, CUSTOM_VHOST_PORT);

        assert_eq!(get_url(domain, CUSTOM_VHOST_PORT), expected_url)
    }

    #[test]
    fn get_low_level_discovery_json_should_return_valid_json() {
        let mut vhosts: Vec<VirtualHost> = Vec::new();

        let domain = String::from("meduttio.uk");

        let vhost = VirtualHost {
            domain: String::from(&domain),
            port: DEFAULT_HTTPS_PORT
        };

        vhosts.push(vhost);

        let sites: Vec<Site> = get_domains_from_vhosts(vhosts, true);

        let expected_json: &str = r#"[{"{#NAME}":"meduttio.uk","{#URL}":"https://meduttio.uk"}]"#;

        let json = get_low_level_discovery_json(sites);

        assert_eq!(json, expected_json);
    }

    #[test]
    fn get_sites_vector_from_vhosts_should_return_domains_with_www_if_option_is_true() {
        let mut vhosts: Vec<VirtualHost> = Vec::new();

        let domain1 = String::from("meduttio.uk");

        let vhost1 = VirtualHost {
            domain: String::from(&domain1),
            port: DEFAULT_HTTPS_PORT
        };

        let domain2 = String::from("www.meduttio.uk");

        let vhost2 = VirtualHost {
            domain: String::from(&domain2),
            port: DEFAULT_HTTP_PORT
        };

        vhosts.push(vhost1);
        vhosts.push(vhost2);

        let sites: Vec<Site> = get_domains_from_vhosts(vhosts, true);

        assert_eq!(2, sites.len());

        let first_result = sites.first();
        assert_eq!(domain1, first_result.unwrap().name);

        let last_result = sites.last();
        assert_eq!("www.meduttio.uk_http", last_result.unwrap().name);
    }

    #[test]
    fn get_sites_vector_from_vhosts_should_return_domains_without_www_if_option_is_false() {
        let mut vhosts: Vec<VirtualHost> = Vec::new();

        let domain1 = String::from("meduttio.uk");

        let vhost1 = VirtualHost {
            domain: String::from(&domain1),
            port: DEFAULT_HTTPS_PORT
        };

        let domain2 = String::from("www.meduttio.uk");

        let vhost2 = VirtualHost {
            domain: String::from(&domain2),
            port: DEFAULT_HTTP_PORT
        };

        vhosts.push(vhost1);
        vhosts.push(vhost2);

        let sites: Vec<Site> = get_domains_from_vhosts(vhosts, false);

        assert_eq!(1, sites.len());

        let first_result = sites.first();
        assert_eq!(domain1, first_result.unwrap().name);
    }

    #[test]
    fn get_low_level_discovery_json_with_data_property_return_valid_json() {
        let mut vhosts: Vec<VirtualHost> = Vec::new();

        let domain = String::from("meduttio.uk");

        let vhost = VirtualHost {
            domain: String::from(&domain),
            port: DEFAULT_HTTPS_PORT
        };

        vhosts.push(vhost);

        let sites: Vec<Site> = get_domains_from_vhosts(vhosts, true);

        let expected_json: &str =
            r#"{"data":[{"{#NAME}":"meduttio.uk","{#URL}":"https://meduttio.uk"}]}"#;

        let json = get_low_level_discovery_json_with_data_property(sites);

        assert_eq!(json, expected_json);
    }
}
