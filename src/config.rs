use std::path::PathBuf;

use docker_api::opts::RegistryAuth;
use serde::Deserialize;

use crate::misc::serde_defs::RegistryAuthDef;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub config_path: PathBuf,
    pub docker_address: String,
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
    pub package_auth: Option<RegistryAuthDef>,
    pub api_secret: String,
    pub docker_args: Vec<String>,
}
