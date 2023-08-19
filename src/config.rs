use std::path::PathBuf;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub config_path: PathBuf,
    pub server: ServerConfig,
}

#[derive(Deserialize, Debug)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub threads: usize,
}

#[derive(Deserialize, Debug)]
pub struct ApplicationConfig {
    pub name: String,
    pub package: String,
    pub api_secret: String,
    pub docker_args: Vec<String>,
}
