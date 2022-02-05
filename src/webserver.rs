pub mod webserver {
    use std::{fs, io};
    use std::fs::{DirEntry, File};
    use std::io::{BufRead, BufReader};
    use std::path::{Path, PathBuf};

    use regex::Regex;

    use crate::domain::domain::VirtualHost;

    const VHOST_CONFIG_FILE_EXTENSION: &str = ".conf";

    pub fn get_vhost_config_file_list(vhost_root_path: &Path,
                                      recursive: bool) -> Result<Vec<PathBuf>,io::Error> {

        let paths = fs::read_dir(&vhost_root_path)?;

        let mut vhost_files: Vec<PathBuf> = Vec::new();

        for path_entry in paths {
            let dir_entry = path_entry.unwrap();

            match dir_entry.file_type() {
                Ok(file_type) => {
                    if file_type.is_dir() && recursive {
                        let path_entry = dir_entry.path();
                        let path_subdir_entry = path_entry.as_path();

                        match get_vhost_config_file_list(path_subdir_entry, recursive) {
                            Ok(mut vhosts) => vhost_files.append(&mut vhosts),
                            Err(e) => error!("{}", e)
                        }
                    }

                    if file_type.is_file() || file_type.is_symlink() {
                        match get_vhost_file_from_dir(vhost_root_path, &dir_entry) {
                            Some(file_path) => vhost_files.push(file_path),
                            None => {}
                        }
                    }
                }
                Err(e) => error!("{}", e)
            }
        }

        Ok(vhost_files)
    }

    pub fn get_virtual_hosts_from_file(
        vhost_file: &Path, section_start_pattern: Regex, redirect_with_301_pattern: Regex,
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

        let mut previous_row: Option<String> = None;

        for line in buffered.lines() {
            let row = line.unwrap_or(String::new());
            trace!("row '{}'", row);

            if section_start_pattern.is_match(&row) {
                if domain.is_none() && port.is_some() {
                    domain = None;
                    port = None;
                }

                if domain.is_some() && port.is_some() && !redirect_to_url {
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
                    trace!("redirect 301 pattern has been matched");

                    match previous_row {
                        Some(previous_row_value) => {
                            trace!("previous row value: '{}'", previous_row_value);
                            if !previous_row_value.contains("location /") {
                                debug!(
                                    "previous row doesn't contain 'location /', \
                                    redirect 301 was detected, skip vhost"
                                );
                                redirect_to_url = true;
                                domain = None;
                                port = None;
                                inside_server_section = false;
                            }
                        }
                        None => {
                            debug!("redirect detected, skip vhost");
                            redirect_to_url = true;
                            domain = None;
                            port = None;
                            inside_server_section = false;
                        }
                    }
                }

                if port.is_none() && port_search_pattern.is_match(&row) {
                    trace!("port wasn't detected yet, port pattern has been matched");
                    let vhost_port_str = get_first_group_match_as_string(
                        &row, &port_search_pattern
                    );

                    trace!("vhost port: '{}'", vhost_port_str);

                    if let Ok(vhost_port) = vhost_port_str.parse() {
                        debug!("port found {}", vhost_port);
                        port = Some(vhost_port);

                    } else { error!("unable to parse port value '{}'", vhost_port_str); }
                }

                if domain.is_none() && domain_search_pattern.is_match(&row) {
                    let domains_row = get_first_group_match_as_string(
                        &row, &domain_search_pattern
                    );
                    let sanitized_domains_row = domains_row.replace(r"[\s\t]{2}", " ");
                    let domains: Vec<&str> = sanitized_domains_row.split(" ").collect::<Vec<&str>>();
                    if let Some(domain_value) = domains.first() {
                        debug!("domain found {}", domain_value);
                        domain = Some(domain_value.to_string());
                    }
                }
            }

            previous_row = Some(row)
        }

        if port.is_some() && domain.is_some() && !redirect_to_url {
            hosts.push(get_virtual_host(domain, port));
        }

        Ok(hosts)
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
