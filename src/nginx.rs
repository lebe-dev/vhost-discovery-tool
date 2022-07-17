use std::path::Path;

use anyhow::Context;
use regex::Regex;

use crate::domain::VirtualHost;
use crate::webserver::{get_vhost_config_file_list, get_virtual_hosts_from_file};

pub fn get_nginx_vhosts(nginx_vhosts_path: &Path,
                        recursive: bool) -> anyhow::Result<Vec<VirtualHost>> {

    debug!("get virtual hosts from nginx configs");
    debug!("configs path '{}'", nginx_vhosts_path.display());

    let mut vhosts: Vec<VirtualHost> = Vec::new();

    if nginx_vhosts_path.exists() && nginx_vhosts_path.is_dir() {

        let vhost_files = get_vhost_config_file_list(
            nginx_vhosts_path, recursive).context("couldn't get files from path")?;

        let section_start_regex = get_nginx_vhost_section_start_regex();
        let redirect_with_301_regex = get_nginx_redirect_with_301_regex();
        let port_search_regex = get_nginx_vhost_port_regex();
        let domain_search_regex = get_domain_search_regex_for_nginx_vhost();

        for vhost_file in vhost_files {
            let vhost_file_path = vhost_file.as_path();

            if let Ok(nginx_vhosts) = get_virtual_hosts_from_file(
                vhost_file_path,
                &section_start_regex,
                &redirect_with_301_regex,
                &port_search_regex,
                &domain_search_regex,
            ) {
                for nginx_vhost in nginx_vhosts {
                    debug!("{}", nginx_vhost.to_string());
                    vhosts.push(nginx_vhost);
                }

            } else { error!("couldn't get virtual hosts from file") }
        }

    } else { info!("nginx vhosts path doesn't exist, skip") }

    Ok(vhosts)
}

pub fn get_domain_search_regex_for_nginx_vhost() -> Regex {
    return Regex::new("(?:^|^[^#]+)server_name[\\s\t]+([a-z0-9.\\s\\-]+);").unwrap();
}

pub fn get_nginx_vhost_section_start_regex() -> Regex {
    return Regex::new("(?:^|^[^#]+)server[\\s\t]+\\{").unwrap();
}

pub fn get_nginx_redirect_with_301_regex() -> Regex {
    return Regex::new("^[\t\\s]*return[\\s\\t]+301[\\s\t]+http.*[\\s\t]*$").unwrap();
}

pub fn get_nginx_vhost_port_regex() -> Regex {
    return Regex::new("^[\\s\t]*listen[\\s\t]+(\\d{1,3}\\.\\d{1,3}\\.\\d{1,3}\\.\\d{1,3}:)?(?P<port>\\d+)[\\s\t]*[ssl\\s|http2\\s]*;.*$").unwrap();
}

#[cfg(test)]
pub mod nginx_tests {
    use std::path::Path;

    use crate::{DEFAULT_HTTP_PORT, DEFAULT_HTTPS_PORT, VirtualHost};
    use crate::nginx::{get_domain_search_regex_for_nginx_vhost, get_nginx_redirect_with_301_regex, get_nginx_vhost_port_regex, get_nginx_vhost_section_start_regex, get_nginx_vhosts};
    use crate::test_utils::assert_vhost_in_vec;
    use crate::webserver::get_virtual_hosts_from_file;

    #[test]
    fn support_ip_and_port() {
        let section_start_regex = get_nginx_vhost_section_start_regex();
        let redirect_with_301_regex = get_nginx_redirect_with_301_regex();
        let port_search_regex = get_nginx_vhost_port_regex();
        let domain_search_regex = get_domain_search_regex_for_nginx_vhost();

        let vhost_file_path = Path::new("tests/nginx-vhosts/listen.conf");

        match get_virtual_hosts_from_file(
            vhost_file_path,
            &section_start_regex,
            &redirect_with_301_regex,
            &port_search_regex,
            &domain_search_regex,
        ) {
            Ok(vhosts) => {
                println!("{:?}", vhosts);
                assert_eq!(vhosts.len(), 2);

                let expected_domain1 = "qweqwe.ru";

                let expected_vhost1 = VirtualHost {
                    domain: expected_domain1.to_string(),
                    port: 2345
                };

                assert_eq!(expected_vhost1.to_string(), vhosts.first().unwrap().to_string());

                let expected_domain2 = "www.megatron2000.ru";

                let expected_vhost2 = VirtualHost {
                    domain: expected_domain2.to_string(),
                    port: 443
                };

                assert_eq!(expected_vhost2.to_string(), vhosts.last().unwrap().to_string());
            },
            Err(_) => panic!("vhosts vec was expected")
        }
    }

    #[test]
    fn support_ssl_and_http2() {
        let section_start_regex = get_nginx_vhost_section_start_regex();
        let redirect_with_301_regex = get_nginx_redirect_with_301_regex();
        let port_search_regex = get_nginx_vhost_port_regex();
        let domain_search_regex = get_domain_search_regex_for_nginx_vhost();

        let vhost_file_path = Path::new("tests/nginx-vhosts/ssl-and-http2.conf");

        match get_virtual_hosts_from_file(
            vhost_file_path,
            &section_start_regex,
            &redirect_with_301_regex,
            &port_search_regex,
            &domain_search_regex,
        ) {
            Ok(vhosts) => {
                println!("{:?}", vhosts);
                assert_eq!(vhosts.len(), 4);

                let expected_domain = "zabbix.com";

                let expected_vhost1 = VirtualHost {
                    domain: expected_domain.to_string(),
                    port: 443
                };

                assert_eq!(expected_vhost1.to_string(), vhosts.first().unwrap().to_string());

                let expected_vhost2 = VirtualHost {
                    domain: expected_domain.to_string(),
                    port: 10555
                };

                assert_eq!(expected_vhost2.to_string(), vhosts.get(1).unwrap().to_string());

                let expected_vhost3 = VirtualHost {
                    domain: expected_domain.to_string(),
                    port: 2928
                };

                assert_eq!(expected_vhost3.to_string(), vhosts.get(2).unwrap().to_string());

                let expected_vhost4 = VirtualHost {
                    domain: expected_domain.to_string(),
                    port: 32318
                };

                assert_eq!(expected_vhost4.to_string(), vhosts.get(3).unwrap().to_string());
            },
            Err(_) => panic!("vhosts vec was expected")
        }
    }

    #[test]
    fn skip_vhosts_with_return_301() {
        let section_start_regex = get_nginx_vhost_section_start_regex();
        let redirect_with_301_regex = get_nginx_redirect_with_301_regex();
        let port_search_regex = get_nginx_vhost_port_regex();
        let domain_search_regex = get_domain_search_regex_for_nginx_vhost();

        let vhost_file_path = Path::new("tests/nginx-vhosts/return-301.conf");

        match get_virtual_hosts_from_file(
            vhost_file_path,
            &section_start_regex,
            &redirect_with_301_regex,
            &port_search_regex,
            &domain_search_regex,
        ) {
            Ok(vhosts) => {
                println!("{:?}", vhosts);
                assert_eq!(vhosts.len(), 1);

                let expected_vhost = VirtualHost {
                    domain: "dhl.de".to_string(),
                    port: 80
                };

                assert_eq!(expected_vhost.to_string(), vhosts.first().unwrap().to_string());
            },
            Err(_) => panic!("vhosts vec was expected")
        }
    }

    #[test]
    fn vhosts_should_be_extracted_from_multiply_files_from_path() {
        let nginx_vhost_path = Path::new("tests/nginx-multi-files");

        let vhosts = get_nginx_vhosts(&nginx_vhost_path, false).unwrap();

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
    fn comments_should_be_respected() {
        let section_start_regex = get_nginx_vhost_section_start_regex();
        let redirect_with_301_regex = get_nginx_redirect_with_301_regex();
        let port_search_regex = get_nginx_vhost_port_regex();
        let domain_search_regex = get_domain_search_regex_for_nginx_vhost();

        let vhost_file_path = Path::new("tests/nginx-vhosts/comments.conf");

        match get_virtual_hosts_from_file(
            vhost_file_path,
            &section_start_regex,
            &redirect_with_301_regex,
            &port_search_regex,
            &domain_search_regex,
        ) {
            Ok(vhosts) => {
                println!("{:?}", vhosts);
                assert_eq!(vhosts.len(), 1);

                let expected_vhost = VirtualHost {
                    domain: "whatever.ru".to_string(),
                    port: 80
                };

                assert_eq!(expected_vhost.to_string(), vhosts.first().unwrap().to_string());
            },
            Err(_) => panic!("vhosts vec was expected")
        }
    }

    #[test]
    fn ignore_vhost_server_without_server_name_property() {
        let section_start_regex = get_nginx_vhost_section_start_regex();
        let redirect_with_301_regex = get_nginx_redirect_with_301_regex();
        let port_search_regex = get_nginx_vhost_port_regex();
        let domain_search_regex = get_domain_search_regex_for_nginx_vhost();

        let vhost_file_path = Path::new(
            "tests/nginx-vhosts/without-server-name-property.conf"
        );

        match get_virtual_hosts_from_file(
            vhost_file_path,
            &section_start_regex,
            &redirect_with_301_regex,
            &port_search_regex,
            &domain_search_regex,
        ) {
            Ok(vhosts) => {
                println!("{:?}", vhosts);
                assert!(vhosts.is_empty());
            },
            Err(_) => panic!("vhosts vec was expected")
        }
    }
}
