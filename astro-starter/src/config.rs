use std::net::{IpAddr, Ipv4Addr, SocketAddr};

extern crate yaml_rust;
use yaml_rust::{yaml, YamlLoader};

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
            // println!("{:?}", doc);

            // handle owner
            if doc["owner"].is_badvalue() {
                return Err("Missing global owner field.");
            }
            let owner = match doc["owner"].as_str() {
                Some(o) => o,
                None => return Err("Global owner field could not be parsed as string."),
            };

            // handle webserver_port
            if doc["webserver_port"].is_badvalue() {
                return Err("Missing global webserver_port field.");
            }
            let webserver_port = match doc["webserver_port"].as_i64() {
                Some(o) => o,
                None => return Err("Global webserver_port field could not be parsed as integer."),
            } as u16;

            // hadle servers
            if doc["servers"].is_badvalue() {
                return Err("Missing global servers field.");
            }

            let mut servers: Vec<ServerConfig> = Vec::new();
            match doc["servers"] {
                yaml::Yaml::Array(ref servers_c) => {
                    for server_c in servers_c {
                        servers.push(match handle_server(server_c) {
                            Ok(s) => s,
                            Err(e) => return Err(e),
                        });
                    }
                }
                _ => return Err("Global servers field is not array."),
            };

            Ok(StarterConfig {
                webserver_port,
                servers,
            })
        }
    }

    fn handle_server(server: &yaml::Yaml) -> Result<ServerConfig, &'static str> {
        println!("{:?}", server);

        if server["id"].is_badvalue() {
            return Err("Missing id field for a server.");
        }
        let id = match server["id"].as_str() {
            Some(o) => o,
            None => return Err("id field could not be parsed as string."),
        };
        println!("id: {:?}", id);

        Ok(ServerConfig {
            id: String::from(id),
            server_type: ServerType::Local,
            name: String::from("x"),
            server_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080),
            console_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080),
            console_password: String::from("x"),
            whitelist: false,
            save_interval: 900,
            backup_saves: true,
            backup_interval: 3600,
            enable_astrochat_integration: false,
        })
    }
}
