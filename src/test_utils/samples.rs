use crate::{DEFAULT_HTTPS_PORT, VirtualHost};

pub const SAMPLE_DOMAIN1: &str = "sumanguru.fi";
pub const SAMPLE_DOMAIN2: &str = "dhl.de";
pub const SAMPLE_DOMAIN3: &str = "tinyops.ru";
pub const SAMPLE_DOMAIN4: &str = "www.google.com";

pub fn get_4_sample_vhosts() -> Vec<VirtualHost> {
    let vhost1 = VirtualHost { domain: SAMPLE_DOMAIN1.to_string(), port: DEFAULT_HTTPS_PORT };
    let vhost2 = VirtualHost { domain: SAMPLE_DOMAIN2.to_string(), port: DEFAULT_HTTPS_PORT };
    let vhost3 = VirtualHost { domain: SAMPLE_DOMAIN3.to_string(), port: DEFAULT_HTTPS_PORT };
    let vhost4 = VirtualHost { domain: SAMPLE_DOMAIN4.to_string(), port: DEFAULT_HTTPS_PORT };
    vec![vhost1.clone(), vhost2.clone(), vhost3.clone(), vhost4.clone()]
}