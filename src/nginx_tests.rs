#[cfg(test)]
pub mod nginx_tests {
    use std::path::Path;

    use crate::DEFAULT_HTTPS_PORT;
    use crate::nginx::nginx::get_nginx_vhosts;
    use crate::test_utils::test_utils::assert_vhost_in_vec;

    const SAMPLE_DOMAIN: &str = "whatever.ru";
    const SAMPLE_DOMAIN2: &str = "gallery.whatever.ru";

    #[test]
    fn get_nginx_vhosts_from_path() {
        let nginx_vhost_path = Path::new("tests/nginx-vhosts");

        let vhosts = get_nginx_vhosts(&nginx_vhost_path);

        vhosts.iter().for_each(|vhost| println!("{}", vhost.to_string()));

        let expected_size: usize = 2;
        assert_eq!(&vhosts.len(), &expected_size);

        assert_vhost_in_vec(&vhosts, SAMPLE_DOMAIN, DEFAULT_HTTPS_PORT);
        assert_vhost_in_vec(&vhosts, SAMPLE_DOMAIN2, 23512);
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
