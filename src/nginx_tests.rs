#[cfg(test)]
pub mod nginx_tests {
    use std::path::Path;

    use crate::{DEFAULT_HTTP_PORT, DEFAULT_HTTPS_PORT, VirtualHost};
    use crate::nginx::nginx::{get_domain_search_regex_for_nginx_vhost, get_nginx_redirect_with_301_regex, get_nginx_vhost_port_regex, get_nginx_vhost_section_start_regex, get_nginx_vhosts};
    use crate::test_utils::test_utils::assert_vhost_in_vec;
    use crate::webserver::webserver::get_virtual_hosts_from_file;

    #[test]
    fn skip_vhosts_with_return_301() {
        let section_start_regex = get_nginx_vhost_section_start_regex();
        let redirect_with_301_regex = get_nginx_redirect_with_301_regex();
        let port_search_regex = get_nginx_vhost_port_regex();
        let domain_search_regex = get_domain_search_regex_for_nginx_vhost();

        let vhost_file_path = Path::new("tests/nginx-vhosts/return-301.conf");

        match get_virtual_hosts_from_file(
            vhost_file_path,
            section_start_regex,
            redirect_with_301_regex,
            port_search_regex,
            domain_search_regex,
        ) {
            Ok(vhosts) => {
                println!("{:?}", vhosts);
                assert_eq!(vhosts.len(), 1);

                let expected_vhost = VirtualHost {
                    domain: "dhl.de".to_string(),
                    port: 80
                };

                assert_eq!(expected_vhost.to_string(), vhosts.first().unwrap().to_string());
            },
            Err(_) => panic!("vhosts vec was expected")
        }
    }

    #[test]
    fn vhosts_should_be_extracted_from_multiply_files_from_path() {
        let nginx_vhost_path = Path::new("tests/nginx-multi-files");

        let vhosts = get_nginx_vhosts(&nginx_vhost_path);

        vhosts.iter().for_each(|vhost| println!("{}", vhost.to_string()));

        println!("{:?}", vhosts);

        let expected_len: usize = 4;
        assert_eq!(vhosts.len(), expected_len);

        assert_vhost_in_vec(&vhosts, "beta.tesla.com", 12398);
        assert_vhost_in_vec(&vhosts, "rust-lang.org", DEFAULT_HTTP_PORT);
        assert_vhost_in_vec(&vhosts, "whatever.ru", DEFAULT_HTTP_PORT);
        assert_vhost_in_vec(&vhosts, "tesla.com", DEFAULT_HTTPS_PORT);
    }

    #[test]
    fn comments_should_be_respected() {
        let section_start_regex = get_nginx_vhost_section_start_regex();
        let redirect_with_301_regex = get_nginx_redirect_with_301_regex();
        let port_search_regex = get_nginx_vhost_port_regex();
        let domain_search_regex = get_domain_search_regex_for_nginx_vhost();

        let vhost_file_path = Path::new("tests/nginx-vhosts/comments.conf");

        match get_virtual_hosts_from_file(
            vhost_file_path,
            section_start_regex,
            redirect_with_301_regex,
            port_search_regex,
            domain_search_regex,
        ) {
            Ok(vhosts) => {
                println!("{:?}", vhosts);
                assert_eq!(vhosts.len(), 1);

                let expected_vhost = VirtualHost {
                    domain: "whatever.ru".to_string(),
                    port: 80
                };

                assert_eq!(expected_vhost.to_string(), vhosts.first().unwrap().to_string());
            },
            Err(_) => panic!("vhosts vec was expected")
        }
    }

    #[test]
    fn ignore_vhost_server_without_server_name_property() {
        let section_start_regex = get_nginx_vhost_section_start_regex();
        let redirect_with_301_regex = get_nginx_redirect_with_301_regex();
        let port_search_regex = get_nginx_vhost_port_regex();
        let domain_search_regex = get_domain_search_regex_for_nginx_vhost();

        let vhost_file_path = Path::new(
        "tests/nginx-vhosts/without-server-name-property.conf"
        );

        match get_virtual_hosts_from_file(
            vhost_file_path,
            section_start_regex,
            redirect_with_301_regex,
            port_search_regex,
            domain_search_regex,
        ) {
            Ok(vhosts) => {
                println!("{:?}", vhosts);
                assert!(vhosts.is_empty());
            },
            Err(_) => panic!("vhosts vec was expected")
        }
    }
}
