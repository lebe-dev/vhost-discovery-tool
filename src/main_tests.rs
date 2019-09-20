#[cfg(test)]
mod main_tests {
    use crate::{DEFAULT_HTTP_PORT, DEFAULT_HTTPS_PORT, get_site_name};

    #[test]
    fn get_site_name_with_standard_port_should_return_name_without_port() {
        let domain = "superco.ru";

        assert_eq!(get_site_name(domain, DEFAULT_HTTP_PORT), domain);
        assert_eq!(get_site_name(domain, DEFAULT_HTTPS_PORT), domain);
    }

    #[test]
    fn get_site_name_with_nonstandard_port_should_return_name_with_port() {
        let domain = "sub.diggers.ru";
        let custom_port = 5382;

        let expected_domain = format!("{}:{}", domain, custom_port);

        assert_eq!(get_site_name(domain, custom_port), expected_domain);
    }
}