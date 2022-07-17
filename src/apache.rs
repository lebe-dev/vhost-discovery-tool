use std::path::Path;
use std::process::exit;

use regex::Regex;

use crate::domain::VirtualHost;
use crate::ERROR_EXIT_CODE;
use crate::webserver::webserver::{get_vhost_config_file_list, get_virtual_hosts_from_file};

pub fn get_apache_vhosts(vhosts_path: &Path, recursive: bool) -> Vec<VirtualHost> {
    debug!("get virtual hosts from apache configs");
    debug!("configs path '{}'", vhosts_path.display());

    let mut vhosts: Vec<VirtualHost> = Vec::new();

    if vhosts_path.is_dir() && vhosts_path.exists() {
        match get_vhost_config_file_list(vhosts_path, recursive) {
            Ok(vhost_files) => {
                for vhost_file in vhost_files {
                    let vhost_file_path = vhost_file.as_path();

                    debug!("analyze vhost file '{}'", vhost_file_path.display());

                    let section_start_regex = get_apache_vhost_port_regex();
                    let redirect_to_url_regex = get_apache_redirect_to_http_regex();
                    let port_search_regex = get_apache_vhost_port_regex();
                    let domain_search_regex = get_domain_search_regex_for_apache_vhost();

                    if let Ok(apache_vhosts) = get_virtual_hosts_from_file(
                        vhost_file_path,
                        section_start_regex,
                        redirect_to_url_regex,
                        port_search_regex,
                        domain_search_regex,
                    ) {
                        for apache_vhost in apache_vhosts {
                            debug!("{}", apache_vhost.to_string());
                            vhosts.push(apache_vhost);
                        }

                    } else { error!("couldn't get virtual hosts from file") }
                }
            }
            Err(_) => {
                error!("couldn't get vhost file list from '{}', \
                        possible reason: lack of permissions", vhosts_path.display());
                exit(ERROR_EXIT_CODE)
            }
        }
    }

    return vhosts;
}

fn get_domain_search_regex_for_apache_vhost() -> Regex {
    return Regex::new("(?:^|^[^#]+)ServerName[\\s\t]+([a-zA-Z0-9.-]+)$").unwrap();
}

fn get_apache_redirect_to_http_regex() -> Regex {
    return Regex::new("(?:^|^[^#]+)Redirect[\\s\t]+/[\\s\t]+http").unwrap();
}

fn get_apache_vhost_port_regex() -> Regex {
    return Regex::new("(?:^|^[^#]+)<VirtualHost[\\s\t]+.*:(?P<port>\\d+)>").unwrap();
}

#[cfg(test)]
pub mod apache_tests {
    use std::path::Path;

    use crate::apache::get_apache_vhosts;
    use crate::test_utils::assert_vhost_in_vec;

    #[test]
    fn get_virtual_hosts_from_apache_file() {
        let vhosts_path = Path::new("tests/apache-vhosts");

        let vhosts = get_apache_vhosts(vhosts_path, false);

        for vhost in &vhosts {
            println!("{}", vhost.to_string());
        }

        let expected_size: usize = 4;
        assert_eq!(&vhosts.len(), &expected_size);

        assert_vhost_in_vec(&vhosts, "collections.museum.ru", 8081);
        assert_vhost_in_vec(&vhosts, "whatever.ru", 443);
        assert_vhost_in_vec(&vhosts, "whatever.ru", 5380);
        assert_vhost_in_vec(&vhosts, "demo.company.ru", 1480);
    }
}
