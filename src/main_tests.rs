#[cfg(test)]
mod main_tests {
    use crate::{DEFAULT_HTTP_PORT, DEFAULT_HTTPS_PORT, get_low_level_discovery_json, get_low_level_discovery_json_with_data_property};
    use crate::domain::domain::{Site, VirtualHost};
    use crate::site::site::{get_domains_from_vhosts, get_url};

    const CUSTOM_VHOST_PORT: i32 = 5382;

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
    fn get_low_level_discovery_json_should_return_valid_json() {
        let mut vhosts: Vec<VirtualHost> = Vec::new();

        let domain = String::from("meduttio.uk");

        let vhost = VirtualHost {
            domain: String::from(&domain),
            port: DEFAULT_HTTPS_PORT
        };

        vhosts.push(vhost);

        let sites: Vec<Site> = get_domains_from_vhosts(vhosts, true);

        let expected_json: &str = r#"[{"{#NAME}":"meduttio.uk","{#URL}":"https://meduttio.uk"}]"#;

        let json = get_low_level_discovery_json(sites);

        assert_eq!(json, expected_json);
    }

    #[test]
    fn get_sites_vector_from_vhosts_should_return_domains_with_www_if_option_is_true() {
        let mut vhosts: Vec<VirtualHost> = Vec::new();

        let domain1 = String::from("meduttio.uk");

        let vhost1 = VirtualHost {
            domain: String::from(&domain1),
            port: DEFAULT_HTTPS_PORT
        };

        let domain2 = String::from("www.meduttio.uk");

        let vhost2 = VirtualHost {
            domain: String::from(&domain2),
            port: DEFAULT_HTTP_PORT
        };

        vhosts.push(vhost1);
        vhosts.push(vhost2);

        let sites: Vec<Site> = get_domains_from_vhosts(vhosts, true);

        assert_eq!(2, sites.len());

        let first_result = sites.first();
        assert_eq!(domain1, first_result.unwrap().name);

        let last_result = sites.last();
        assert_eq!("www.meduttio.uk_http", last_result.unwrap().name);
    }

    #[test]
    fn get_sites_vector_from_vhosts_should_return_domains_without_www_if_option_is_false() {
        let mut vhosts: Vec<VirtualHost> = Vec::new();

        let domain1 = String::from("meduttio.uk");

        let vhost1 = VirtualHost {
            domain: String::from(&domain1),
            port: DEFAULT_HTTPS_PORT
        };

        let domain2 = String::from("www.meduttio.uk");

        let vhost2 = VirtualHost {
            domain: String::from(&domain2),
            port: DEFAULT_HTTP_PORT
        };

        vhosts.push(vhost1);
        vhosts.push(vhost2);

        let sites: Vec<Site> = get_domains_from_vhosts(vhosts, false);

        assert_eq!(1, sites.len());

        let first_result = sites.first();
        assert_eq!(domain1, first_result.unwrap().name);
    }

    #[test]
    fn get_low_level_discovery_json_with_data_property_return_valid_json() {
        let mut vhosts: Vec<VirtualHost> = Vec::new();

        let domain = String::from("meduttio.uk");

        let vhost = VirtualHost {
            domain: String::from(&domain),
            port: DEFAULT_HTTPS_PORT
        };

        vhosts.push(vhost);

        let sites: Vec<Site> = get_domains_from_vhosts(vhosts, true);

        let expected_json: &str =
            r#"{"data":[{"{#NAME}":"meduttio.uk","{#URL}":"https://meduttio.uk"}]}"#;

        let json = get_low_level_discovery_json_with_data_property(sites);

        assert_eq!(json, expected_json);
    }
}
