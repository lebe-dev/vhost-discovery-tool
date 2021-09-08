pub mod filter {
    use crate::{DEFAULT_HTTP_PORT, DEFAULT_HTTPS_PORT};
    use crate::domain::domain::VirtualHost;
    use regex::Regex;

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
