pub mod apache {
    use regex::Regex;

    pub fn get_domain_search_regex_for_apache_vhost() -> Regex {
        return Regex::new("ServerName[\\s\t]+([a-zA-Z0-9.-]+)$").unwrap();
    }

    pub fn get_apache_redirect_to_http_regex() -> Regex {
        return Regex::new("Redirect[\\s\t]+/[\\s\t]+http").unwrap();
    }

    pub fn get_apache_vhost_port_regex() -> Regex {
        return Regex::new("<VirtualHost[\\s\t]+.*:(\\d+)>").unwrap();
    }
}
