pub mod nginx {
    use std::path::Path;
    use std::process::exit;

    use crate::ERROR_EXIT_CODE;
    use crate::webserver::webserver::{get_domain_search_regex_for_nginx_vhost, get_nginx_redirect_with_301_regex, get_nginx_vhost_port_regex, get_nginx_vhost_section_start_regex, get_vhost_config_file_list, get_virtual_hosts_from_file, VirtualHost};

    pub fn get_nginx_vhosts(nginx_vhosts_path: &Path) -> Vec<VirtualHost> {
        debug!("get virtual hosts from nginx configs");
        debug!("configs path '{}'", nginx_vhosts_path.display());

        let mut vhosts: Vec<VirtualHost> = Vec::new();

        if nginx_vhosts_path.is_dir() && nginx_vhosts_path.exists() {
            match get_vhost_config_file_list(nginx_vhosts_path) {
                Ok(vhost_files) => {
                    for vhost_file in vhost_files {
                        debug!("analyze vhost file '{}'", vhost_file.display());

                        let section_start_regex = get_nginx_vhost_section_start_regex();
                        let redirect_with_301_regex = get_nginx_redirect_with_301_regex();
                        let port_search_regex = get_nginx_vhost_port_regex();
                        let domain_search_regex = get_domain_search_regex_for_nginx_vhost();

                        let vhost_file_path = vhost_file.as_path();

                        if let Ok(nginx_vhosts) = get_virtual_hosts_from_file(
                            vhost_file_path,
                            section_start_regex,
                            redirect_with_301_regex,
                            port_search_regex,
                            domain_search_regex,
                        ) {
                            for nginx_vhost in nginx_vhosts {
                                debug!("{}", nginx_vhost.to_string());
                                vhosts.push(nginx_vhost);
                            }

                        } else {
                            error!("unable to get virtual hosts form file")
                        }
                    }
                }
                Err(_error) => {
                    error!("unable to get vhost file list from '{}', \
                       possible reason: lack of permissions", nginx_vhosts_path.display());
                    exit(ERROR_EXIT_CODE)
                }
            }
        }

        return vhosts;
    }
}
