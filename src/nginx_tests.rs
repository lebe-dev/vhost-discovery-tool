#[cfg(test)]
pub mod nginx_tests {
    use std::path::Path;

    use crate::nginx::nginx::get_nginx_vhosts;

    const SAMPLE_DOMAIN: &str = "whatever.ru";
    const SAMPLE_DOMAIN2: &str = "gallery.whatever.ru";

    #[test]
    fn get_nginx_vhosts_from_path() {
        let nginx_vhost_path = Path::new("tests/nginx-vhosts");

        let vhosts = get_nginx_vhosts(&nginx_vhost_path);

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

    #[test]
    fn ignore_vhost_server_without_server_name_property() {
        let nginx_vhost_path = Path::new("tests/nginx-vhosts");

        let vhosts = get_nginx_vhosts(&nginx_vhost_path);

        vhosts.iter().for_each(|vhost| println!("{}", vhost.to_string()));

        let expected_size: usize = 2;
        assert_eq!(&vhosts.len(), &expected_size);
    }
}
