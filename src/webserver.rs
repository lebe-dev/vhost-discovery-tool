pub mod webserver {
    use std::{fs, io};
    use std::fs::{DirEntry, File};
    use std::io::{BufRead, BufReader};
    use std::path::{Path, PathBuf};

    use regex::Regex;

    use crate::domain::domain::VirtualHost;

    const VHOST_CONFIG_FILE_EXTENSION: &str = ".conf";

    pub fn get_vhost_config_file_list(vhost_root_path: &Path) -> Result<Vec<PathBuf>,io::Error> {
        let paths = fs::read_dir(&vhost_root_path)?;

        let mut vhost_files: Vec<PathBuf> = Vec::new();

        for path in paths {
            let file = path.unwrap();

            match get_vhost_file_from_dir(vhost_root_path, &file) {
                Some(file_path) => vhost_files.push(file_path),
                None => {}
            }
        }

        Ok(vhost_files)
    }

    pub fn get_virtual_hosts_from_file(vhost_file: &Path,
                                       section_start_pattern: Regex,
                                       redirect_with_301_pattern: Regex,
                                       port_search_pattern: Regex,
                                       domain_search_pattern: Regex) -> Result<Vec<VirtualHost>, io::Error> {
        let mut hosts: Vec<VirtualHost> = Vec::new();

        let vhost_file_name = vhost_file.to_str().unwrap();

        info!("get virtual hosts from file '{}'", vhost_file_name);

        let input = File::open(vhost_file)?;
        let buffered = BufReader::new(input);

        let mut inside_server_section = false;
        let mut redirect_to_url = false;
        let mut port: Option<i32> = None;
        let mut domain: Option<String> = None;

        for line in buffered.lines() {
            let row = line.unwrap_or(String::new());

            trace!("row '{}'", row);

            if section_start_pattern.is_match(&row) {
                if domain.is_none() && port.is_some() {
                    domain = None;
                    port = None;
                }

                if port.is_some() && domain.is_some() && !redirect_to_url {
                    let vhost = get_virtual_host(domain, port);

                    hosts.push(vhost);

                    domain = None;
                    port = None;
                }

                inside_server_section = true;
                redirect_to_url = false;
            }

            if inside_server_section {
                if redirect_with_301_pattern.is_match(&row) {
                    debug!("redirect detected");
                    redirect_to_url = true;
                    inside_server_section = false;

                    domain = None;
                    port = None;
                }

                if port.is_none() && port_search_pattern.is_match(&row) {
                    let vhost_port_str = get_first_group_match_as_string(&row, &port_search_pattern);
                    if let Ok(vhost_port) = vhost_port_str.parse() {
                        debug!("port found {}", vhost_port);
                        port = Some(vhost_port);

                    } else { error!("unable to parse port value '{}'", vhost_port_str); }
                }

                if domain.is_none() && domain_search_pattern.is_match(&row) {
                    let domain_name = get_first_group_match_as_string(&row, &domain_search_pattern);
                    debug!("domain found {}", domain_name);
                    domain = Some(domain_name);
                }
            }

        }

        if port.is_some() && domain.is_some() && !redirect_to_url {
            hosts.push(get_virtual_host(domain, port));
        }

        Ok(hosts)
    }

    pub fn get_domain_search_regex_for_apache_vhost() -> Regex {
        return Regex::new("ServerName[\\s\t]+([a-zA-Z0-9.-]+)$").unwrap();
    }

    pub fn get_apache_redirect_to_http_regex() -> Regex {
        return Regex::new("Redirect[\\s\t]+/[\\s\t]+http").unwrap();
    }

    pub fn get_apache_vhost_port_regex() -> Regex {
        return Regex::new("<VirtualHost[\\s\t]+.*:(\\d+)>").unwrap();
    }

    fn get_vhost_file_from_dir(vhost_root_path: &Path,
                               dir_entry: &DirEntry) -> Option<PathBuf> {
        let mut result: Option<PathBuf> = None;

        if let Ok(file_type) = dir_entry.file_type() {
            if file_type.is_file() || file_type.is_symlink() {
                if let Ok(file_name) = dir_entry.file_name().into_string() {
                    if file_name.ends_with(VHOST_CONFIG_FILE_EXTENSION) {
                        let vhost_file = vhost_root_path.join(file_name);
                        result = Some(vhost_file)
                    }
                }
            }
        }

        result
    }

    fn get_first_group_match_as_string(row: &str, pattern: &Regex) -> String {
        let groups = pattern.captures_iter(&row).next().unwrap();
        String::from(&groups[1])
    }

    fn get_virtual_host(domain: Option<String>, port: Option<i32>) -> VirtualHost {
        let domain_name = domain.unwrap();
        VirtualHost {
            domain: domain_name.to_owned(), port: port.unwrap()
        }
    }
}
