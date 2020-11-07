#[cfg(test)]
pub mod apache_tests {
    use std::path::Path;

    use crate::apache::apache::get_apache_vhosts;
    use crate::test_utils::test_utils::assert_vhost_in_vec;

    #[test]
    fn get_virtual_hosts_from_apache_file() {
        let vhosts_path = Path::new("tests/apache-vhosts");

        let vhosts = get_apache_vhosts(vhosts_path);

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
