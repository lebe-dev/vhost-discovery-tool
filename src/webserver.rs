use std::{fs, io};
use std::fs::{DirEntry, File};
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

use anyhow::{anyhow, Context};
use regex::Regex;

use crate::domain::VirtualHost;
use crate::vhost::VhostDiscoveryConfig;

pub fn get_vhosts(path: &Path, config: &VhostDiscoveryConfig,
                  fall_on_parse_errors: bool) -> anyhow::Result<Vec<VirtualHost>> {
    info!("getting vhosts from path '{}'..", path.display());

    let mut results: Vec<VirtualHost> = Vec::new();

    if path.is_dir() && path.exists() {
        let vhost_files = get_vhost_config_file_list(
            path, &config.file_extensions, config.include_subdirs)
            .context("couldn't get vhost files from path")?;

        for vhost_file in vhost_files {
            let vhost_file_path = vhost_file.as_path();

            debug!("processing file '{}'", vhost_file_path.display());

            match get_virtual_hosts_from_file(
                vhost_file_path,
                &config.section_start, &config.redirect_to_url,
                &config.port, &config.domain
            ) {
                Ok(vhosts) => {
                    for vhost in vhosts {
                        debug!("{}", vhost.to_string());
                        results.push(vhost);
                    }
                }
                Err(e) => {
                    error!("unable to get virtual hosts from file path '{}': {}",
                        vhost_file_path.display(), e);

                    if fall_on_parse_errors {
                        break;
                    }
                }
            }

        }

        Ok(results)

    } else {
        warn!("vhosts path '{}' doesn't exist", path.display());
        Err(anyhow!("vhosts path doesn't exist"))
    }

}

pub fn get_vhost_config_file_list(vhost_root_path: &Path, file_extensions: &Vec<String>,
                                  recursive: bool) -> Result<Vec<PathBuf>,io::Error> {

    let paths = fs::read_dir(&vhost_root_path)?;

    let mut vhost_files: Vec<PathBuf> = Vec::new();

    for path_entry in paths {
        let dir_entry = path_entry?;

        match dir_entry.file_type() {
            Ok(file_type) => {
                if file_type.is_dir() && recursive {
                    let path_entry = dir_entry.path();
                    let path_subdir_entry = path_entry.as_path();

                    match get_vhost_config_file_list(path_subdir_entry, &file_extensions, recursive) {
                        Ok(mut vhosts) => vhost_files.append(&mut vhosts),
                        Err(e) => error!("{}", e)
                    }
                }

                if file_type.is_file() || file_type.is_symlink() {
                    match get_vhost_file_from_dir(vhost_root_path, &file_extensions, &dir_entry) {
                        Some(file_path) => vhost_files.push(file_path),
                        None => {}
                    }
                }
            }
            Err(e) => error!("{}", e)
        }
    }

    debug!("vhost files collected: {:?}", vhost_files);

    Ok(vhost_files)
}

pub fn get_virtual_hosts_from_file(
    vhost_file: &Path, section_start_pattern: &Regex, redirect_with_301_pattern: &Regex,
    port_search_pattern: &Regex,
    domain_search_pattern: &Regex) -> anyhow::Result<Vec<VirtualHost>> {

    let mut hosts: Vec<VirtualHost> = Vec::new();

    let vhost_file_name = vhost_file.to_str().unwrap();

    info!("get virtual hosts from file '{vhost_file_name}'");

    let input = File::open(vhost_file)?;
    let buffered = BufReader::new(input);

    let mut inside_server_section = false;
    let mut redirect_to_url = false;
    let mut port: Option<i32> = None;
    let mut domain: Option<String> = None;

    let mut previous_row: Option<String> = None;

    for line in buffered.lines() {
        let row = line.unwrap_or(String::new());
        trace!("row '{row}'");

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
                        trace!("previous row value: '{previous_row_value}'");
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
                let vhost_port_str = find_group_with_port_value(
                    &row, &port_search_pattern
                );

                trace!("vhost port: '{vhost_port_str}'");

                if let Ok(vhost_port) = vhost_port_str.parse() {
                    debug!("port found {}", vhost_port);
                    port = Some(vhost_port);

                } else { error!("couldn't parse port value '{vhost_port_str}'"); }
            }

            if domain.is_none() && domain_search_pattern.is_match(&row) {
                let domains_row = get_first_group_match_as_string(
                    &row, &domain_search_pattern
                );
                let sanitized_domains_row = domains_row.replace(r"[\s\t]{2}", " ");
                let domains: Vec<&str> = sanitized_domains_row.split(" ").collect::<Vec<&str>>();
                if let Some(domain_value) = domains.first() {
                    debug!("domain found {domain_value}");
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

fn get_vhost_file_from_dir(vhost_root_path: &Path, file_extensions: &Vec<String>,
                           dir_entry: &DirEntry) -> Option<PathBuf> {
    let mut result: Option<PathBuf> = None;

    if let Ok(file_type) = dir_entry.file_type() {
        if file_type.is_file() || file_type.is_symlink() {
            if let Ok(file_name) = dir_entry.file_name().into_string() {

                for file_ext in file_extensions.iter() {

                    if file_name.ends_with(file_ext) {
                        let vhost_file = vhost_root_path.join(file_name);
                        result = Some(vhost_file);
                        break;
                    }

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

fn find_group_with_port_value(row: &str, pattern: &Regex) -> String {
    let mut port = String::new();

    for caps in pattern.captures_iter(&row) {
        port = format!("{}", &caps["port"]);
    }

    return port
}

fn get_virtual_host(domain: Option<String>, port: Option<i32>) -> VirtualHost {
    let domain_name = domain.unwrap();
    VirtualHost {
        domain: domain_name.to_owned(), port: port.unwrap()
    }
}

#[cfg(test)]
mod get_vhosts_tests {
    use std::path::Path;

    use crate::{DEFAULT_HTTP_PORT, DEFAULT_HTTPS_PORT, get_nginx_discovery_config};
    use crate::test_utils::assert_vhost_in_vec;
    use crate::webserver::get_vhosts;

    #[test]
    fn vhosts_should_be_extracted_from_multiply_files_from_path() {
        let nginx_vhost_path = Path::new("test-data/nginx-multi-files");
        let config = get_nginx_discovery_config(
            false, &vec![".conf".to_string()]);

        let vhosts = get_vhosts(&nginx_vhost_path, &config, false).unwrap();

        vhosts.iter().for_each(|vhost| println!("{}", vhost.to_string()));

        println!("{:?}", vhosts);

        let expected_len: usize = 4;
        assert_eq!(vhosts.len(), expected_len);

        assert_vhost_in_vec(&vhosts, "beta.tesla.com", 12398);
        assert_vhost_in_vec(&vhosts, "rust-lang.org", DEFAULT_HTTP_PORT);
        assert_vhost_in_vec(&vhosts, "whatever.ru", DEFAULT_HTTP_PORT);
        assert_vhost_in_vec(&vhosts, "tesla.com", DEFAULT_HTTPS_PORT);
    }

    #[test]
    fn return_error_for_invalid_path() {
        let config = get_nginx_discovery_config(true, &vec![".conf".to_string()]);
        let path = Path::new("does-not-exist");
        assert!(get_vhosts(path, &config, false).is_err())
    }
}

#[cfg(test)]
mod webserver_tests {
    use std::path::Path;

    use crate::nginx::{get_domain_search_regex_for_nginx_vhost, get_nginx_redirect_with_301_regex, get_nginx_vhost_port_regex, get_nginx_vhost_section_start_regex};
    use crate::webserver::{get_vhost_config_file_list, get_virtual_hosts_from_file};

    #[test]
    fn support_recursive_mode() {
        let vhost_root_path = Path::new("tests/nginx-multi-files");
        let files = get_vhost_config_file_list(vhost_root_path,  &vec![".conf".to_string()], true).unwrap();

        let expected_size: usize = 3;
        assert_eq!(&files.len(), &expected_size);
    }

    #[test]
    fn get_vhost_config_file_list_should_return_file_names() {
        let vhost_root_path = Path::new("tests/apache-vhosts");
        let files = get_vhost_config_file_list(vhost_root_path, &vec![".conf".to_string()], false).unwrap();

        let expected_size: usize = 2;
        assert_eq!(&files.len(), &expected_size);
    }

    #[test]
    fn get_vhost_config_file_list_should_return_error_for_unknown_path() {
        let unknown_path = Path::new("unknown-path");
        assert!(get_vhost_config_file_list(unknown_path, &vec![".conf".to_string()], false).is_err());
    }

    #[test]
    fn include_vhosts_with_redirect_inside_location() {
        let vhost_file = Path::new("tests/nginx-vhosts/vhost2.conf");
        let vhosts = get_virtual_hosts_from_file(
            vhost_file, &get_nginx_vhost_section_start_regex(),
            &get_nginx_redirect_with_301_regex(),
            &get_nginx_vhost_port_regex(),
            &get_domain_search_regex_for_nginx_vhost()
        ).unwrap();

        let expected_domain = "goodhost.ru";

        let result = vhosts.iter().find(
            |vhost|vhost.domain == expected_domain
        ).unwrap();

        assert_eq!(result.domain, expected_domain);
        assert_eq!(result.port, 443);
    }
}
