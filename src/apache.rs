use regex::Regex;

use crate::vhost::VhostDiscoveryConfig;

pub fn get_apache_discovery_config(include_subdirs: bool, file_extensions: &Vec<String>) -> VhostDiscoveryConfig {
    VhostDiscoveryConfig {
        section_start: get_apache_vhost_port_regex(),
        redirect_to_url: get_apache_redirect_to_http_regex(),
        port: get_apache_vhost_port_regex(),
        domain: get_domain_search_regex_for_apache_vhost(),
        include_subdirs,
        file_extensions: file_extensions.clone()
    }
}

pub fn get_domain_search_regex_for_apache_vhost() -> Regex {
    return Regex::new("(?:^|^[^#]+)ServerName[\\s\t]+([a-zA-Z0-9.-]+)$").unwrap();
}

pub fn get_apache_redirect_to_http_regex() -> Regex {
    return Regex::new("(?:^|^[^#]+)Redirect[\\s\t]+/[\\s\t]+http").unwrap();
}

pub fn get_apache_vhost_port_regex() -> Regex {
    return Regex::new("(?:^|^[^#]+)<VirtualHost[\\s\t]+.*:(?P<port>\\d+)>").unwrap();
}

#[cfg(test)]
pub mod apache_tests {
    use std::path::Path;

    use crate::{get_apache_discovery_config, get_vhosts};
    use crate::test_utils::assert_vhost_in_vec;

    #[test]
    fn get_virtual_hosts_from_apache_file() {
        let vhosts_path = Path::new("test-data/apache-vhosts");

        let config = get_apache_discovery_config(false, &vec![".conf".to_string()]);

        let vhosts = get_vhosts(vhosts_path, &config, false).unwrap();

        for vhost in &vhosts {
            println!("{}", vhost.to_string());
        }

        let expected_size: usize = 4;
        assert_eq!(&vhosts.len(), &expected_size);

        assert_vhost_in_vec(&vhosts, "collections.museum.ru", 8081);
        assert_vhost_in_vec(&vhosts, "whatever.ru", 443);
        assert_vhost_in_vec(&vhosts, "whatever.ru", 5380);
        assert_vhost_in_vec(&vhosts, "demo.company.ru", 1480);
    }
}
