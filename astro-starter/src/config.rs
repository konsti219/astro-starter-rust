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
        pub fn new(config_path: &PathBuf) -> Result<StarterConfig, String> {
            let config_path = config_path.to_str().unwrap();

            let config_content = match read_to_string(config_path) {
                Ok(c) => c,
                Err(_) => return Err(String::from("Failed to read config file.")),
            };
            let docs = YamlLoader::load_from_str(&config_content)
                .expect("Could not parse yaml config file.");

            let doc = &docs[0];

            // print full doc
            // println!("{:?}", doc);

            // handle owner
            if doc["owner"].is_badvalue() {
                return Err(String::from("Missing global owner field."));
            }
            let owner = match doc["owner"].as_str() {
                Some(o) => o,
                None => {
                    return Err(String::from(
                        "Global owner field could not be parsed as string.",
                    ))
                }
            };

            // handle webserver_port
            if doc["webserver_port"].is_badvalue() {
                return Err(String::from("Missing global webserver_port field."));
            }
            let webserver_port = match doc["webserver_port"].as_i64() {
                Some(o) => o,
                None => {
                    return Err(String::from(
                        "Global webserver_port field could not be parsed as integer.",
                    ))
                }
            } as u16;

            // hadle servers
            if doc["servers"].is_badvalue() {
                return Err(String::from("Missing global servers field."));
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
                _ => return Err(String::from("Global servers field is not array.")),
            };

            Ok(StarterConfig {
                webserver_port,
                servers,
            })
        }
    }

    fn handle_server(server: &yaml::Yaml) -> Result<ServerConfig, String> {
        println!("{:?}", server);

        // handle id
        if server["id"].is_badvalue() {
            return Err(String::from("Missing id field for a server."));
        }
        let id = match server["id"].as_str() {
            Some(v) => v,
            None => return Err(String::from("id field could not be parsed as string.")),
        };
        println!("id: {:?}", id);

        // handle server_type
        if server["type"].is_badvalue() {
            return Err(String::from("Missing server_type field, id: ") + &id);
        }
        let server_type = match server["type"].as_str() {
            Some(v) => {
                if v == "local" {
                    ServerType::Local
                } else if v == "remote" {
                    ServerType::Remote
                } else {
                    return Err(
                        String::from("server_type field needs to be local/remore, id: ") + &id,
                    );
                }
            }
            None => {
                return Err(
                    String::from("server_type field could not be parsed as string, id: ") + &id,
                )
            }
        };
        println!("server_type: {:?}", server_type);

        // handle name
        if server["name"].is_badvalue() {
            return Err(String::from("Missing name field, id: ") + &id);
        }
        let name = match server["name"].as_str() {
            Some(v) => v,
            None => {
                return Err(String::from("name field could not be parsed as string, id: ") + &id)
            }
        };
        println!("name: {:?}", name);

        // https://api.ipify.org/

        // handle
        /*if server["_"].is_badvalue() {
            return Err(String::from("Missing _ field, id: ") + &id);
        }
        let _ = match server["_"].as_str() {
            Some(v) => v,
            None => return Err(String::from("_ field could not be parsed as string, id: ") + &id),
        };*/
        //println!("_: {:?}", x);

        Ok(ServerConfig {
            id: String::from(id),
            server_type,
            name: String::from(name),
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
