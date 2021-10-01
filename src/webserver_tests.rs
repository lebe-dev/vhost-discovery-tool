#[cfg(test)]
mod webserver_tests {
    use std::path::Path;
    use crate::nginx::nginx::{get_domain_search_regex_for_nginx_vhost, get_nginx_redirect_with_301_regex, get_nginx_vhost_port_regex, get_nginx_vhost_section_start_regex};

    use crate::webserver::webserver::{get_vhost_config_file_list, get_virtual_hosts_from_file};

    #[test]
    fn get_vhost_config_file_list_should_return_file_names() {
        let vhost_root_path = Path::new("tests/apache-vhosts");
        let files = get_vhost_config_file_list(vhost_root_path).unwrap();

        let expected_size: usize = 2;
        assert_eq!(&files.len(), &expected_size);
    }

    #[test]
    fn get_vhost_config_file_list_should_return_error_for_unknown_path() {
        let unknown_path = Path::new("unknown-path");
        assert!(get_vhost_config_file_list(unknown_path).is_err());
    }

    #[test]
    fn include_vhosts_with_redirect_inside_location() {
        let vhost_file = Path::new("tests/nginx-vhosts/vhost2.conf");
        let vhosts = get_virtual_hosts_from_file(
            vhost_file, get_nginx_vhost_section_start_regex(),
            get_nginx_redirect_with_301_regex(),
            get_nginx_vhost_port_regex(),
            get_domain_search_regex_for_nginx_vhost()
        ).unwrap();

        let expected_domain = "goodhost.ru";

        let result = vhosts.iter().find(
            |vhost|vhost.domain == expected_domain
        ).unwrap();

        assert_eq!(result.domain, expected_domain);
        assert_eq!(result.port, 443);
    }
}
