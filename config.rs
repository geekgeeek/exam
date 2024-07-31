use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
pub struct BackendConfig {
    pub address: String,
    pub port: u16,
}

#[derive(Deserialize)]
pub struct Config {
    pub load_balancer: String,
    pub backend_servers: Vec<BackendConfig>,
}

pub fn load_config(filename: &str) -> Result<Config, Box<dyn std::error::Error>> {
    let config_str = fs::read_to_string(filename)?;
    let config: Config = toml::from_str(&config_str)?;
    if config.backend_servers.is_empty() {
        return Err("No backend servers specified".into());
    }
    Ok(config)
}
