#[cfg(test)]
mod main_tests {
    use crate::{DEFAULT_HTTP_PORT, DEFAULT_HTTPS_PORT, get_low_level_discovery_json, get_site_name, get_url, Site, vector_contains_same_domain_with_default_http_port, vector_contains_same_domain_with_ssl};
    use crate::webserver::webserver::VirtualHost;

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

    #[test]
    fn vector_contains_same_domain_with_ssl_should_return_true_if_vhost_with_ssl_found() {
        let mut vhosts: Vec<VirtualHost> = Vec::new();

        let domain = String::from("zebra.uk");

        let vhost = VirtualHost {
            domain: String::from(&domain),
            port: DEFAULT_HTTPS_PORT
        };

        vhosts.push(vhost);

        assert!(vector_contains_same_domain_with_ssl(&vhosts, &domain))
    }

    #[test]
    fn vector_contains_same_domain_with_default_http_port_should_return_true_if_vhost_with_standard_http_port_found() {
        let mut vhosts: Vec<VirtualHost> = Vec::new();

        let domain = String::from("meduttio.uk");

        let vhost = VirtualHost {
            domain: String::from(&domain),
            port: DEFAULT_HTTP_PORT
        };

        vhosts.push(vhost);

        assert!(vector_contains_same_domain_with_default_http_port(&vhosts, &domain))
    }

    #[test]
    fn get_low_level_discovery_json_should_return_valid_json() {
        let mut vhosts: Vec<VirtualHost> = Vec::new();

        let domain = String::from("meduttio.uk");

        let vhost = VirtualHost {
            domain: String::from(&domain),
            port: DEFAULT_HTTP_PORT
        };

        vhosts.push(vhost);

        let sites: Vec<Site> = vhosts.iter().map(|vhost| {
            let url = get_url(&vhost.domain, vhost.port);

            Site {
                name: get_site_name(&vhost.domain, vhost.port),
                url
            }
        }).collect();

        let expected_json: &str = r#"[{"{#NAME}":"meduttio.uk","{#URL}":"http://meduttio.uk"}]"#;

        let json = get_low_level_discovery_json(sites);

        assert_eq!(json, expected_json);
    }
}
