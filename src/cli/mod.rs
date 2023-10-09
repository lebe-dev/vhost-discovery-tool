use clap::ArgMatches;

use crate::{DOMAIN_IGNORE_MASKS_OPTION, INCLUDE_CUSTOM_PORTS_OPTION, INCLUDE_DOMAINS_WITH_WWW, RECURSIVE_OPTION, VHOST_FILE_EXTENSIONS_DEFAULT_VALUE};

pub struct AppConfig {
    pub include_domains_with_www: bool,
    pub include_custom_domains: bool,
    pub recursive_mode: bool,

    pub domain_ignore_masks: Vec<String>,

    pub vhost_file_extensions: Vec<String>
}

pub fn get_app_config(arg_matches: &ArgMatches) -> AppConfig {

    let domain_ignore_masks_row: &str = if arg_matches.is_present(DOMAIN_IGNORE_MASKS_OPTION) {
        arg_matches.value_of(DOMAIN_IGNORE_MASKS_OPTION).unwrap()
    } else { "" };

    let vhost_file_extensions_row: &str = if arg_matches.is_present(VHOST_FILE_EXTENSIONS_DEFAULT_VALUE) {
        arg_matches.value_of(VHOST_FILE_EXTENSIONS_DEFAULT_VALUE).unwrap()
    } else { VHOST_FILE_EXTENSIONS_DEFAULT_VALUE };

    let vhost_file_extensions = vhost_file_extensions_row.split(",")
        .collect::<Vec<&str>>()
        .iter()
        .map(|fe|fe.to_string())
        .collect::<Vec<String>>();

    AppConfig {
        include_domains_with_www: arg_matches.occurrences_of(INCLUDE_DOMAINS_WITH_WWW) > 0,
        include_custom_domains: arg_matches.occurrences_of(INCLUDE_CUSTOM_PORTS_OPTION) > 0,
        recursive_mode: arg_matches.occurrences_of(RECURSIVE_OPTION) > 0,
        domain_ignore_masks: domain_ignore_masks_row.split(",").collect::<Vec<&str>>()
            .iter()
            .map(|fe|fe.to_string())
            .collect::<Vec<String>>(),
        vhost_file_extensions,
    }
}