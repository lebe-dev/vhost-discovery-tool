use regex::Regex;

pub struct VhostDiscoveryConfig {
    /// Pattern for vhost section start
    ///
    /// Example for nginx:
    ///
    /// ```nginx
    /// server {
    /// ```
    pub section_start: Regex,

    /// Pattern for redirect to another url
    pub redirect_to_url: Regex,

    /// Pattern for vhost port
    pub port: Regex,

    /// Pattern for vhost domain
    pub domain: Regex,

    /// Scan sub-directories for vhost files
    pub include_subdirs: bool
}