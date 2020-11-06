#[cfg(test)]
mod webserver_tests {
    use std::path::Path;

    use crate::nginx::nginx::{get_domain_search_regex_for_nginx_vhost, get_nginx_vhost_section_start_regex};
    use crate::webserver::webserver::{get_apache_redirect_to_http_regex, get_apache_vhost_port_regex, get_domain_search_regex_for_apache_vhost, get_nginx_redirect_with_301_regex, get_nginx_vhost_port_regex, get_vhost_config_file_list, get_virtual_hosts_from_file};

    const SAMPLE_DOMAIN: &str = "whatever.ru";
    const SAMPLE_DOMAIN2: &str = "gallery.whatever.ru";

    const NGINX_SAMPLE_VHOST_FILE: &str = "tests/nginx-vhosts/vhost2.conf";

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
    fn get_virtual_hosts_from_nginx_file() {
        let vhost_file = Path::new(NGINX_SAMPLE_VHOST_FILE);
        let section_start_regex = get_nginx_vhost_section_start_regex();
        let redirect_with_301_regex = get_nginx_redirect_with_301_regex();
        let port_search_regex = get_nginx_vhost_port_regex();
        let domain_search_regex = get_domain_search_regex_for_nginx_vhost();

        if let Ok(vhosts) = get_virtual_hosts_from_file(
            vhost_file, section_start_regex,
            redirect_with_301_regex,
            port_search_regex, domain_search_regex
        ) {
            vhosts.iter().for_each(|vhost| println!("{}", vhost.to_string()));

            let expected_size: usize = 2;
            assert_eq!(&vhosts.len(), &expected_size);

            let first_vhost = &vhosts.first().unwrap();

            assert_eq!(first_vhost.port, 443);
            assert_eq!(first_vhost.domain, SAMPLE_DOMAIN);

            let last_vhost = &vhosts.last().unwrap();

            assert_eq!(last_vhost.port, 23512);
            assert_eq!(last_vhost.domain, SAMPLE_DOMAIN2);
        }
    }

    #[test]
    fn ignore_vhost_server_without_server_name_property() {
        let vhost_file = Path::new(NGINX_SAMPLE_VHOST_FILE);

        let section_start_regex = get_nginx_vhost_section_start_regex();
        let redirect_with_301_regex = get_nginx_redirect_with_301_regex();
        let port_search_regex = get_nginx_vhost_port_regex();
        let domain_search_regex = get_domain_search_regex_for_nginx_vhost();

        if let Ok(vhosts) = get_virtual_hosts_from_file(
            vhost_file, section_start_regex,
            redirect_with_301_regex,
            port_search_regex, domain_search_regex
        ) {
            assert_eq!(vhosts.len(), 2);

            let first_vhost = vhosts.first().unwrap();
            assert_eq!(first_vhost.domain, SAMPLE_DOMAIN);

            let last_vhost = vhosts.last().unwrap();
            assert_eq!(last_vhost.domain, SAMPLE_DOMAIN2);
        }
    }

    #[test]
    fn get_virtual_hosts_from_apache_file() {
        let vhost_file = Path::new("tests/apache-vhosts/vhost2.conf");
        let section_start_regex = get_apache_vhost_port_regex();
        let redirect_to_http = get_apache_redirect_to_http_regex();
        let port_search_regex = get_apache_vhost_port_regex();
        let domain_search_regex = get_domain_search_regex_for_apache_vhost();

        if let Ok(vhosts) = get_virtual_hosts_from_file(
            vhost_file,
            section_start_regex,
            redirect_to_http,
            port_search_regex,
            domain_search_regex
        ) {

            for vhost in &vhosts {
                println!("{}", vhost.to_string());
            }

            let expected_size: usize = 3;
            assert_eq!(&vhosts.len(), &expected_size);

            let first_vhost = &vhosts.first().unwrap();

            assert_eq!(first_vhost.port, 443);
            assert_eq!(first_vhost.domain, "whatever.ru");

            let second_vhost = &vhosts.get(1).unwrap();

            assert_eq!(second_vhost.port, 5380);
            assert_eq!(second_vhost.domain, "whatever.ru");

            let last_vhost = &vhosts.last().unwrap();

            assert_eq!(last_vhost.port, 1480);
            assert_eq!(last_vhost.domain, "demo.company.ru");
        }
    }

    #[test]
    fn get_domain_search_regex_for_apache_vhost_should_match_valid_servername_values() {
        let regex = get_domain_search_regex_for_apache_vhost();
        assert!(regex.is_match("    ServerName distrib.company.ru"));
        assert!(regex.is_match("ServerName   vp123.Cgro2Mp-aNy.ru"));
        assert_eq!(regex.is_match("ServerName 1 cp.coM2mu-any.ru   "), false);
    }

    #[test]
    fn test_get_nginx_vhost_port_regex_pattern() {
        let regex = get_nginx_vhost_port_regex();

        assert!(regex.is_match("   listen 80;"));
        assert!(regex.is_match("    listen 443 ssl;"));
        assert!(regex.is_match("     listen 443  ssl;"));
        assert!(!regex.is_match(""));
        assert!(!regex.is_match("listen abcasdwd932 ssl;"));
    }

    #[test]
    fn test_get_nginx_redirect_to_https_regex_pattern() {
        let regex = get_nginx_redirect_with_301_regex();
        assert!(regex.is_match(" return 301 https://whatever.ru;"));
        assert!(regex.is_match("return 301 https://whatever.ru;     "));
        assert!(!regex.is_match("return 302 https://whatever.ru;"));
    }

    #[test]
    fn test_get_domain_search_regex_for_nginx_vhost() {
        let regex = get_domain_search_regex_for_nginx_vhost();

        assert!(regex.is_match("server_name abc;"));
        assert!(regex.is_match("  server_name  abc; "));

        assert!(!regex.is_match("server_name  abc"));
        assert!(!regex.is_match("server_nameabc;"));
        assert!(!regex.is_match(""));
    }
}
