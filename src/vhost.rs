use regex::Regex;

pub struct VhostDiscoveryConfig {
    pub section_start: Regex,
    pub redirect_to_url: Regex,
    pub port: Regex,
    pub domain: Regex,
    pub include_subdirs: bool
}