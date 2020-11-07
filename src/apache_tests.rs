#[cfg(test)]
pub mod apache_tests {
    use std::path::Path;

    use crate::apache::apache::{get_apache_redirect_to_http_regex, get_apache_vhost_port_regex, get_domain_search_regex_for_apache_vhost};
    use crate::webserver::webserver::get_virtual_hosts_from_file;

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
}
