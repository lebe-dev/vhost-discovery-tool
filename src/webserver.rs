pub mod webserver {
    use std::{fs, io};
    use std::ffi::OsString;
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    use std::path::{Path, PathBuf};

    use regex::Regex;

    const VHOST_CONFIG_FILE_EXTENSION: &str = ".conf";

    pub struct VirtualHost {
        pub domain: String,
        pub port: i32
    }

    pub fn get_vhost_config_file_list(vhost_root_path: &Path) -> Result<Vec<PathBuf>,io::Error> {
        let paths = fs::read_dir(&vhost_root_path)?;

        let mut vhost_files: Vec<PathBuf> = Vec::new();

        for path in paths {
            let file = path.unwrap();
            let file_type = file.file_type()?;

            if file_type.is_file() {
                let file_name = file.file_name().into_string().unwrap();

                if file_name.ends_with(VHOST_CONFIG_FILE_EXTENSION) {
                    let vhost_file = vhost_root_path.join(file_name);
                    vhost_files.push(vhost_file);
                }
            }
        }

        Ok(vhost_files)
    }

    pub fn get_virtual_hosts_from_file(vhost_file: &Path,
                                       section_start_pattern: Regex,
                                       port_search_pattern: Regex,
                                       domain_search_pattern: Regex) -> Vec<VirtualHost> {
        let mut hosts: Vec<VirtualHost> = Vec::new();

        info!("get virtual hosts from file '{}'", vhost_file.display());
        let input = File::open(vhost_file).unwrap();
        let buffered = BufReader::new(input);

        let mut inside_section = false;
        let mut port: Option<i32> = None;
        let mut domain: Option<String> = None;

        for line in buffered.lines() {
            let row = line.unwrap();

            if section_start_pattern.is_match(&row) {
                if domain.is_none() && port.is_some() {
                    let hostname: OsString = gethostname::gethostname();
                    let hostname_as_domain = hostname.into_string().unwrap();

                    if &hostname_as_domain != "localhost" {
                        let vhost = VirtualHost {
                            domain: String::from(hostname_as_domain), port: port.unwrap()
                        };

                        hosts.push(vhost);
                    }

                    port = None;
                    domain = None;
                }

                inside_section = true;
            }

            if inside_section && port.is_none() && port_search_pattern.is_match(&row) {
                let groups = port_search_pattern.captures_iter(&row).next().unwrap();
                let vhost_port_str = String::from(&groups[1]);
                let vhost_port: i32 = vhost_port_str.parse().unwrap();
                port = Some(vhost_port);
            }

            if inside_section && domain.is_none() && domain_search_pattern.is_match(&row) {
                let groups = domain_search_pattern.captures_iter(&row).next().unwrap();
                let domain_name = String::from(&groups[1]);
                domain = Some(domain_name);
            }

            if port.is_some() && domain.is_some() {
                let hostname: OsString = gethostname::gethostname();
                let hostname_as_domain = hostname.into_string().unwrap();

                if &hostname_as_domain != "localhost" {
                    let vhost = VirtualHost {
                        domain: String::from(hostname_as_domain), port: port.unwrap()
                    };

                    hosts.push(vhost);
                }

                port = None;
                domain = None;

                inside_section = false;
            }
        }

        return hosts
    }

    pub fn get_domain_search_regex_for_nginx_vhost() -> Regex {
        return Regex::new("server_name[\\s\t]+([a-z0-9.\\-]+);").unwrap();
    }

    pub fn get_domain_search_regex_for_apache_vhost() -> Regex {
        return Regex::new("ServerName[\\s\t]+([a-zA-Z0-9.-]+)$").unwrap();
    }

    pub fn get_nginx_vhost_section_start_regex() -> Regex {
        return Regex::new("server[\\s\t]+\\{").unwrap();
    }

    pub fn get_apache_vhost_section_start_regex() -> Regex {
        return Regex::new("<VirtualHost[\\s\t]+.*:\\d+>").unwrap();
    }

    pub fn get_nginx_vhost_port_regex() -> Regex {
        return Regex::new("[\\s\t]+listen[\\s\t]+(\\d+)( ssl)?;").unwrap();
    }

    pub fn get_apache_vhost_port_regex() -> Regex {
        return Regex::new("<VirtualHost[\\s\t]+.*:(\\d+)>").unwrap();
    }
}