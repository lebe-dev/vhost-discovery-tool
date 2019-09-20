#[cfg(test)]
mod main_tests {
    use crate::{DEFAULT_HTTP_PORT, DEFAULT_HTTPS_PORT, get_site_name, get_url};

    const CUSTOM_VHOST_PORT: i32 = 5382;

    #[test]
    fn get_site_name_with_standard_port_should_return_name_without_port() {
        let domain = "superco.ru";

        assert_eq!(get_site_name(domain, DEFAULT_HTTP_PORT), domain);
        assert_eq!(get_site_name(domain, DEFAULT_HTTPS_PORT), domain);
    }

    #[test]
    fn get_site_name_with_nonstandard_port_should_return_name_with_port() {
        let domain = "sub.diggers.ru";
        let expected_domain = format!("{}:{}", domain, CUSTOM_VHOST_PORT);

        assert_eq!(get_site_name(domain, CUSTOM_VHOST_PORT), expected_domain);
    }

    #[test]
    fn get_url_should_return_url_with_https_for_443_port() {
        let domain = "quarkoman.com";
        let expected_url = format!("https://{}", domain);

        assert_eq!(get_url(domain, DEFAULT_HTTPS_PORT), expected_url)
    }

    #[test]
    fn get_url_should_return_url_without_port_for_default_http_port() {
        let domain = "quarkoman.com";
        let expected_url = format!("http://{}", domain);

        assert_eq!(get_url(domain, DEFAULT_HTTP_PORT), expected_url)
    }

    #[test]
    fn get_url_should_return_url_with_port_when_custom_port_provided() {
        let domain = "quarkoman.com";
        let expected_url = format!("http://{}:{}", domain, CUSTOM_VHOST_PORT);

        assert_eq!(get_url(domain, CUSTOM_VHOST_PORT), expected_url)
    }
}