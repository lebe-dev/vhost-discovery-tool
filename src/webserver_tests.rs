#[cfg(test)]
mod webserver_tests {
    use std::path::Path;

    use crate::webserver::webserver::{get_apache_vhost_port_regex, get_apache_vhost_section_start_regex, get_domain_from_vhost_file, get_domain_search_regex_for_apache_vhost, get_domain_search_regex_for_nginx_vhost, get_nginx_vhost_port_regex, get_nginx_vhost_section_start_regex, get_vhost_server_port, get_virtual_hosts_from_file, VHOST_DEFAULT_HOSTNAME};

    #[test]
    fn get_virtual_hosts_from_nginx_file() {
        let vhost_file = Path::new("tests/nginx-vhosts/vhost2.conf");
        let section_start_regex = get_nginx_vhost_section_start_regex();
        let port_search_regex = get_nginx_vhost_port_regex();
        let domain_search_regex = get_domain_search_regex_for_nginx_vhost();

        let vhosts = get_virtual_hosts_from_file(
            vhost_file, section_start_regex, port_search_regex, domain_search_regex
        );

        let expected_size: usize = 3;
        assert_eq!(&vhosts.len(), &expected_size);

        let first_vhost = &vhosts.first().unwrap();

        assert_eq!(first_vhost.port, 38101);
        assert_eq!(first_vhost.domain, "collections.company.ru");

        let second_vhost = &vhosts.get(1).unwrap();

        assert_eq!(second_vhost.port, 27239);
        assert_eq!(second_vhost.domain, VHOST_DEFAULT_HOSTNAME);

        let last_vhost = &vhosts.last().unwrap();

        assert_eq!(last_vhost.port, 23512);
        assert_eq!(last_vhost.domain, "collections.company.ru");
    }

    #[test]
    fn get_virtual_hosts_from_apache_file() {
        let vhost_file = Path::new("tests/apache-vhosts/vhost2.conf");
        let section_start_regex = get_apache_vhost_section_start_regex();
        let port_search_regex = get_apache_vhost_port_regex();
        let domain_search_regex = get_domain_search_regex_for_apache_vhost();

        let vhosts = get_virtual_hosts_from_file(
            vhost_file, section_start_regex, port_search_regex, domain_search_regex
        );

        for vhost in &vhosts {
            println!("port: '{}', domain: '{}'", vhost.port, vhost.domain);
        }

        let expected_size: usize = 3;
        assert_eq!(&vhosts.len(), &expected_size);

        let first_vhost = &vhosts.first().unwrap();

        assert_eq!(first_vhost.port, 80);
        assert_eq!(first_vhost.domain, "collections.e-gallery.ru");

        let second_vhost = &vhosts.get(1).unwrap();

        assert_eq!(second_vhost.port, 5380);
        assert_eq!(second_vhost.domain, "crab.corp.ru");

        let last_vhost = &vhosts.last().unwrap();

        assert_eq!(last_vhost.port, 1480);
        assert_eq!(last_vhost.domain, "demo.company.ru");
    }

    #[test]
    fn get_domain_search_regex_for_apache_vhost_should_match_valid_servername_values() {
        let regex = get_domain_search_regex_for_apache_vhost();
        assert!(regex.is_match("    ServerName vp.ugramuseum.ru"));
        assert!(regex.is_match("ServerName   vp123.Ugra2mus-eum.ru"));
        assert_eq!(regex.is_match("ServerName 1 vp.Ugra2mu-seum.ru   "), false);
    }

    #[test]
    fn get_domain_from_vhost_file_with_apache_pattern_should_return_domain_name() {
        let vhost_file = Path::new("tests/apache-vhosts/vhost1.conf");
        let domain_search_regex = get_domain_search_regex_for_apache_vhost();
        let domain = get_domain_from_vhost_file(vhost_file, domain_search_regex);

        assert_eq!("collections.museum.ru", domain.unwrap());
    }

    #[test]
    fn get_domain_from_vhost_file_with_nginx_pattern_should_return_domain_name() {
        let vhost_file = Path::new("tests/nginx-vhosts/vhost2.conf");
        let domain_search_regex = get_domain_search_regex_for_nginx_vhost();
        let domain = get_domain_from_vhost_file(vhost_file, domain_search_regex);

        assert_eq!("collections.museum.ru", domain.unwrap());
    }

    #[test]
    fn get_nginx_vhost_server_port_should_return_port() {
        let vhost_file = Path::new("tests/nginx-vhosts/vhost2.conf");
        let search_regex = get_nginx_vhost_port_regex();

        let port = get_vhost_server_port(vhost_file, search_regex);

        assert_eq!(port, 38101);
    }

    #[test]
    fn get_apache_vhost_server_port_should_return_port() {
        let vhost_file = Path::new("tests/apache-vhosts/vhost1.conf");
        let search_regex = get_apache_vhost_port_regex();

        let port = get_vhost_server_port(vhost_file, search_regex);

        assert_eq!(port, 8081);
    }
}