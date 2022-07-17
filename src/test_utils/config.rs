use crate::apache::{get_apache_redirect_to_http_regex, get_apache_vhost_port_regex, get_domain_search_regex_for_apache_vhost};
use crate::nginx::{get_domain_search_regex_for_nginx_vhost, get_nginx_redirect_with_301_regex, get_nginx_vhost_port_regex, get_nginx_vhost_section_start_regex};
use crate::vhost::VhostDiscoveryConfig;

pub fn get_nginx_discovery_config(include_subdirs: bool) -> VhostDiscoveryConfig {
    VhostDiscoveryConfig {
        section_start_regex: get_nginx_vhost_section_start_regex(),
        redirect_to_url: get_nginx_redirect_with_301_regex(),
        port: get_nginx_vhost_port_regex(),
        domain: get_domain_search_regex_for_nginx_vhost(),
        include_subdirs
    }
}

pub fn get_apache_discovery_config(include_subdirs: bool) -> VhostDiscoveryConfig {
    VhostDiscoveryConfig {
        section_start_regex: get_apache_vhost_port_regex(),
        redirect_to_url: get_apache_redirect_to_http_regex(),
        port: get_apache_vhost_port_regex(),
        domain: get_domain_search_regex_for_apache_vhost(),
        include_subdirs
    }
}