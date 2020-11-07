#[cfg(test)]
mod webserver_tests {
    use std::path::Path;

    use crate::nginx::nginx::{get_domain_search_regex_for_nginx_vhost, get_nginx_redirect_with_301_regex, get_nginx_vhost_port_regex};
    use crate::webserver::webserver::get_vhost_config_file_list;

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
