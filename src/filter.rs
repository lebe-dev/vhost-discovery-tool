pub mod filter {
    use crate::{DEFAULT_HTTP_PORT, DEFAULT_HTTPS_PORT};
    use crate::domain::domain::VirtualHost;

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
            if vhost.port == DEFAULT_HTTP_PORT || vhost.port == DEFAULT_HTTPS_PORT {
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

    fn vec_contains_same_domain_with_port(vhosts: &Vec<VirtualHost>,
                                              domain: &String, port: i32) -> bool {
        vhosts.iter()
              .find(|vhost| &vhost.domain == domain && vhost.port == port).is_some()
    }
}
