use lazy_static::lazy_static;
use std::{env, net::IpAddr, path::PathBuf};

lazy_static! {
    static ref KEYCLOAK_PUBLIC_KEY: String = parse_required_string("KEYCLOAK_PUBLIC_KEY");
}

#[derive(Debug, Clone)]
pub struct Config {
    pub host: IpAddr,
    pub port: u16,
    pub keycloak_public_key: &'static str,
    pub images_path: PathBuf,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            host: parse_ip_addr("BIND_HOST", [0, 0, 0, 0].into()),
            port: parse_port("BIND_PORT", 8090),
            keycloak_public_key: KEYCLOAK_PUBLIC_KEY.as_str(),
            images_path: parse_required_pathbuf("IMAGES_PATH"),
        }
    }
}

pub fn parse_ip_addr(env_var: &str, default: IpAddr) -> IpAddr {
    let addr = env::var(env_var);
    if let Ok(addr) = addr {
        addr.parse()
            .unwrap_or_else(|_| panic!("Provided {} is not a valid ip address: {}", env_var, addr))
    } else {
        default
    }
}

pub fn parse_port(env_var: &str, default: u16) -> u16 {
    let port = env::var(env_var);
    if let Ok(port) = port {
        port.parse()
            .unwrap_or_else(|_| panic!("Provided {} is not a valid port: {}", env_var, port))
    } else {
        default
    }
}

pub fn parse_string(env_var: &str, default: &str) -> String {
    env::var(env_var).unwrap_or(default.to_string())
}

pub fn parse_required_string(env_var: &str) -> String {
    let value = parse_string(env_var, "");
    if value != "" {
        value
    } else {
        panic!("Setting {} is mandatory and should not be empty", env_var)
    }
}

pub fn parse_required_pathbuf(env_var: &str) -> PathBuf {
    let path = parse_required_string(env_var);
    path.parse()
        .unwrap_or_else(|_| panic!("Provided {} is not a valid path: {}", env_var, path))
}
