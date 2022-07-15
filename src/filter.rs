pub mod filter {
    use regex::Regex;

    use crate::{DEFAULT_HTTP_PORT, DEFAULT_HTTPS_PORT};
    use crate::domain::domain::VirtualHost;

    pub fn filter_by_domain_masks(vhosts: &Vec<VirtualHost>,
                                  mask_patterns: &Vec<&str>) -> Vec<VirtualHost> {

        let mut results: Vec<VirtualHost> = Vec::new();

        for vhost in vhosts {
            let mut permitted = true;

            for mask in mask_patterns {
                debug!("mask regexp '{}'", mask);

                if mask.len() > 0 {
                    match Regex::new(mask) {
                        Ok(mask_pattern) => {
                            if mask_pattern.is_match(&vhost.domain) {
                                debug!(
                                    "vhost domain '{}' has been filtered by pattern '{}'",
                                    vhost.domain, mask
                                );
                                permitted = false;
                                break
                            }
                        }
                        Err(e) => error!("invalid filter mask pattern: {} [skip]", e)
                    }
                }

            }

            if permitted {
                results.push(vhost.to_owned())
            }
        }

        return results
    }

    pub fn filter_vhosts(vhosts: &Vec<VirtualHost>, include_custom_domains: bool) -> Vec<VirtualHost> {
        let mut results: Vec<VirtualHost> = Vec::new();

        for vhost in vhosts {
            if vhost_add_permitted(vhost, &results, include_custom_domains) {
                debug!("+ add vhost '{}'", vhost.to_string());
                results.push(vhost.to_owned());
            }
        }

        return results
    }

    fn vhost_add_permitted(vhost: &VirtualHost, buffer: &Vec<VirtualHost>,
                           include_custom_ports: bool) -> bool {

        let mut permitted = false;

        if include_custom_ports {
            if !vec_contains_same_domain_with_port(buffer, &vhost.domain, vhost.port) {
                permitted = true;
            }

        } else {
            if vhost_has_standard_port(vhost.port) {
                if !vec_contains_same_domain_with_port(buffer, &vhost.domain, vhost.port) {
                    permitted = true;
                }
            }
        }

        if permitted {
            debug!("+ add vhost '{}'", vhost.to_string());
        }

        permitted
    }

    fn vhost_has_standard_port(port: i32) -> bool {
        port == DEFAULT_HTTP_PORT || port == DEFAULT_HTTPS_PORT
    }

    fn vec_contains_same_domain_with_port(vhosts: &Vec<VirtualHost>,
                                              domain: &String, port: i32) -> bool {
        vhosts.iter()
              .find(|vhost| &vhost.domain == domain && vhost.port == port).is_some()
    }
}

#[cfg(test)]
mod filter_tests {
    use crate::{DEFAULT_HTTP_PORT, DEFAULT_HTTPS_PORT};
    use crate::domain::domain::VirtualHost;
    use crate::filter::filter::{filter_by_domain_masks, filter_vhosts};

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
