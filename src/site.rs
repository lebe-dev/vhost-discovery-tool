pub mod site {
    use crate::{DEFAULT_HTTP_PORT, DEFAULT_HTTPS_PORT, WWW_SEARCH_PATTERN};
    use crate::domain::domain::{Site, VirtualHost};

    pub fn get_domains_from_vhosts(vhosts: Vec<VirtualHost>,
                                   include_domains_with_www: bool) -> Vec<Site> {
        let sites: Vec<Site> = vhosts.iter()
            .filter(|vhost| {
                let domain_in_lowercase = vhost.domain.to_lowercase();

                let domain_starts_with_www = domain_in_lowercase.starts_with(WWW_SEARCH_PATTERN);

                (!include_domains_with_www && !domain_starts_with_www) ||
                (include_domains_with_www && domain_starts_with_www) ||
                !domain_starts_with_www

            }).map(get_domain_from_vhost).collect();

        return sites;
    }

    pub fn get_url(domain: &str, vhost_port: i32) -> String {
        match vhost_port {
            DEFAULT_HTTP_PORT => String::from(format!("http://{}", domain)),
            DEFAULT_HTTPS_PORT => String::from(format!("https://{}", domain)),
            _ => String::from(format!("http://{}:{}", domain, vhost_port))
        }
    }

    fn get_domain_from_vhost(vhost: &VirtualHost) -> Site {
        let url = get_url(&vhost.domain, vhost.port);
        Site { name: get_site_name(&vhost.domain, vhost.port), url }
    }

    fn get_site_name(domain: &str, port: i32) -> String {
        if port == DEFAULT_HTTP_PORT {
            String::from(format!("{}_http", domain))
        } else if port == DEFAULT_HTTPS_PORT {
            String::from(domain)
        } else {
            String::from(format!("{}:{}", domain, port))
        }
    }
}
