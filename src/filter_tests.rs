#[cfg(test)]
mod filter_tests {
    use crate::{DEFAULT_HTTP_PORT, DEFAULT_HTTPS_PORT};
    use crate::domain::domain::VirtualHost;
    use crate::filter::filter::{filter_vhosts, filter_by_domain_masks};

    const DOMAIN: &str = "cronbox.ru";
    const DOMAIN2: &str = "tinyops.ru";
    const DOMAIN3: &str = "fancy-ads.com";
    const DOMAIN4: &str = "ads.megacorp.de";

    #[test]
    fn filter_by_domain_masks_should_exclude_domains_which_contain_at_least_one_mask() {
        let vhost1 = VirtualHost { domain: DOMAIN2.to_string(), port: DEFAULT_HTTP_PORT };
        let vhost2 = VirtualHost { domain: DOMAIN3.to_string(), port: DEFAULT_HTTPS_PORT };
        let vhost3 = VirtualHost { domain: DOMAIN.to_string(), port: 5384 };
        let vhost4 = VirtualHost { domain: DOMAIN4.to_string(), port: DEFAULT_HTTPS_PORT };
        let vhost5 = VirtualHost { domain: "localhost".to_string(), port: DEFAULT_HTTPS_PORT };

        let vhosts: Vec<VirtualHost> = vec![
            vhost1.clone(), vhost2.clone(), vhost3.clone(), vhost4.clone(), vhost5.clone()
        ];

        let masks: Vec<&str> = vec![".de$", "^fancy", "ops", "^localhost$"];

        let results = filter_by_domain_masks(&vhosts, &masks);

        assert_eq!(results.len(), 1);

        let first_result = results.get(0).unwrap();
        assert_eq!(first_result.domain, vhost3.domain);
    }

    #[test]
    fn filter_by_domain_masks_should_ignore_blank_masks() {
        let vhost1 = VirtualHost { domain: DOMAIN2.to_string(), port: DEFAULT_HTTP_PORT };
        let vhost2 = VirtualHost { domain: DOMAIN3.to_string(), port: DEFAULT_HTTPS_PORT };
        let vhost3 = VirtualHost { domain: DOMAIN.to_string(), port: 5384 };
        let vhost4 = VirtualHost { domain: DOMAIN4.to_string(), port: DEFAULT_HTTPS_PORT };

        let vhosts: Vec<VirtualHost> = vec![
            vhost1.clone(), vhost2.clone(), vhost3.clone(), vhost4.clone()
        ];

        let masks: Vec<&str> = vec!["", "qwerty"];

        let results = filter_by_domain_masks(&vhosts, &masks);

        assert_eq!(results.len(), 4);
    }

    #[test]
    fn result_without_custom_ports_should_contain_only_http_or_https_ports() {
        let vhost1 = VirtualHost { domain: DOMAIN.to_string(), port: 7435 };
        let vhost2 = VirtualHost { domain: DOMAIN2.to_string(), port: DEFAULT_HTTP_PORT };
        let vhost3 = VirtualHost { domain: DOMAIN.to_string(), port: DEFAULT_HTTPS_PORT };

        let vhosts: Vec<VirtualHost> = vec![vhost1.clone(), vhost2.clone(), vhost3.clone()];

        let results = filter_vhosts(&vhosts, false);

        assert_eq!(results.len(), 2);

        let vhost2_found = results.iter().find(
            |vhost| vhost.domain == vhost2.domain && vhost.port == vhost2.port
        );

        assert!(vhost2_found.is_some());

        let vhost3_found = results.iter().find(
            |vhost| vhost.domain == vhost3.domain && vhost.port == vhost3.port
        );

        assert!(vhost3_found.is_some());
    }

    #[test]
    fn result_should_not_contain_duplicates_without_custom_ports() {
        let vhost1 = VirtualHost { domain: DOMAIN.to_string(), port: DEFAULT_HTTPS_PORT };
        let vhost2 = VirtualHost { domain: DOMAIN2.to_string(), port: DEFAULT_HTTP_PORT };
        let vhost3 = VirtualHost { domain: DOMAIN.to_string(), port: DEFAULT_HTTPS_PORT };

        let vhosts: Vec<VirtualHost> = vec![vhost1.clone(), vhost2.clone(), vhost3.clone()];

        let results = filter_vhosts(&vhosts, false);

        assert_eq!(results.len(), 2);

        let vhost1_found = results.iter().find(
            |vhost| vhost.domain == vhost1.domain && vhost.port == vhost1.port
        );

        assert!(vhost1_found.is_some());

        let vhost2_found = results.iter().find(
            |vhost| vhost.domain == vhost2.domain && vhost.port == vhost2.port
        );

        assert!(vhost2_found.is_some());
    }

    #[test]
    fn result_should_not_contain_duplicates_with_custom_ports() {
        let custom_port = 4113;

        let vhost1 = VirtualHost { domain: DOMAIN.to_string(), port: custom_port };
        let vhost2 = VirtualHost { domain: DOMAIN2.to_string(), port: DEFAULT_HTTPS_PORT };
        let vhost3 = VirtualHost { domain: DOMAIN.to_string(), port: custom_port };

        let vhosts: Vec<VirtualHost> = vec![vhost1.clone(), vhost2.clone(), vhost3.clone()];

        let results = filter_vhosts(&vhosts, true);

        assert_eq!(results.len(), 2);

        let vhost1_found = results.iter().find(
            |vhost| vhost.domain == vhost1.domain && vhost.port == vhost1.port
        );

        assert!(vhost1_found.is_some());

        let vhost2_found = results.iter().find(
            |vhost| vhost.domain == vhost2.domain && vhost.port == vhost2.port
        );

        assert!(vhost2_found.is_some());
    }
}
