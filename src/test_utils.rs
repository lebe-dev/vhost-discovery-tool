#[cfg(test)]
pub mod test_utils {
    use crate::domain::domain::VirtualHost;

    pub fn assert_vhost_in_vec(vhosts: &Vec<VirtualHost>, domain: &str, port: i32) {
        let vhost_found = vhosts.iter().find(|vhost| vhost.domain == domain && vhost.port == port);
        println!("expect domain: '{}'", domain);
        println!("expect port: {}", port);
        assert!(vhost_found.is_some());
    }
}
