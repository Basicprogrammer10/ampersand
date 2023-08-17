use std::path::PathBuf;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub config_path: PathBuf,
    pub server: ServerConfig,
}

#[derive(Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub threads: usize,
}

#[derive(Deserialize)]
pub struct ApplicationConfig {
    pub name: String,
    pub package: String,
    pub api_secret: String,
}
