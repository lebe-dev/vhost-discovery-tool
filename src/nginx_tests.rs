#[cfg(test)]
pub mod nginx_tests {
    use std::path::Path;

    use crate::nginx::nginx::{get_domain_search_regex_for_nginx_vhost, get_nginx_redirect_with_301_regex, get_nginx_vhost_port_regex, get_nginx_vhost_section_start_regex};
    use crate::webserver::webserver::get_virtual_hosts_from_file;

    const NGINX_SAMPLE_VHOST_FILE: &str = "tests/nginx-vhosts/vhost2.conf";

    const SAMPLE_DOMAIN: &str = "whatever.ru";
    const SAMPLE_DOMAIN2: &str = "gallery.whatever.ru";

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
}
