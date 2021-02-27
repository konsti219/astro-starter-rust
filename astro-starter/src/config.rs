use std::net::{IpAddr, Ipv4Addr, SocketAddr};

extern crate yaml_rust;
use yaml_rust::YamlLoader;

pub mod starter_config {
    use std::{fs::read_to_string, path::PathBuf};

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
        pub fn new(config_path: &PathBuf) -> Result<StarterConfig, &'static str> {
            let config_path = config_path.to_str().unwrap();

            let config_content = match read_to_string(config_path) {
                Ok(c) => c,
                Err(_) => return Err("Failed to read config file."),
            };
            let docs = YamlLoader::load_from_str(&config_content)
                .expect("Could not parse yaml config file.");

            let doc = &docs[0];

            // print full doc
            println!("{:?}", doc);

            // handle owner
            if doc["owner"].is_badvalue() {
                return Err("Missing global owner field.");
            }
            let owner = match doc["owner"].as_str() {
                Some(o) => o,
                None => return Err("Global owner field could not be parsed as string."),
            };
            println!("owner: {:?}", owner);

            // handle webserver_port
            if doc["webserver_port"].is_badvalue() {
                return Err("Missing global webserver_port field.");
            }
            let webserver_port = match doc["webserver_port"].as_i64() {
                Some(o) => o,
                None => return Err("Global webserver_port field could not be parsed as integer."),
            } as u16;
            println!("webserver_port: {:?}", webserver_port);

            // hadle servers
            if doc["servers"].is_badvalue() {
                return Err("Missing global servers field.");
            }

            Ok(StarterConfig {
                webserver_port,
                servers: vec![],
            })
        }
    }
}
