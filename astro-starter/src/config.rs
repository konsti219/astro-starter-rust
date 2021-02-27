use std::{
    fs,
    net::{IpAddr, Ipv4Addr, SocketAddr},
    path::Path,
};

extern crate yaml_rust;
use yaml_rust::YamlLoader;

pub mod starter_config {
    use std::path::PathBuf;

    use super::*;

    #[derive(Debug)]
    enum ServerType {
        Local,
        Remote,
    }
    #[derive(Debug)]
    struct ServerConfig {
        id: String,
        server_type: ServerType,
        name: String,
        server_addr: SocketAddr,
        console_addr: SocketAddr,
        console_password: String,
        whitelist: bool,
        save_interval: usize,
        backup_saves: bool,
        backup_interval: usize,
        enable_astrochat_integration: bool,
    }
    #[derive(Debug)]
    pub struct StarterConfig {
        webserver_port: u16,
        servers: Vec<ServerConfig>,
    }

    impl StarterConfig {
        pub fn new(work_dir: &PathBuf) -> StarterConfig {
            let config_path = Path::new(&work_dir).join("starter_config.yml");
            let config_path = config_path
                .to_str()
                .expect("Error converting path in StarterConfig::new");
            let docs =
                YamlLoader::load_from_str(config_path).expect("Could not parse yaml config file");

            StarterConfig {
                webserver_port: 5000,
                servers: vec![],
            }
        }
    }
}
