use std::net::{IpAddr, Ipv4Addr, SocketAddr};

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
struct StarterConfig {
    webserver_port: u16,
    servers: Vec<ServerConfig>,
}

fn main() {
    println!("Do stuff");
}
