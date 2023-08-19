use std::{fs, path::PathBuf};

use anyhow::Result;

use crate::{
    application::Application,
    config::{ApplicationConfig, Config},
};

pub struct App {
    config: Config,
    applications: Vec<Application>,
}

impl App {
    pub fn new(config_path: PathBuf) -> Result<Self> {
        let config_raw = fs::read_to_string(&config_path)?;
        let config = toml::from_str::<Config>(&config_raw)?;

        let mut applications = Vec::new();
        for i in fs::read_dir(&config.config_path)? {
            let path = i?.path();
            if path == config.config_path {
                continue;
            }

            let raw = fs::read_to_string(path)?;
            let app_config = toml::from_str::<ApplicationConfig>(&raw)?;
            applications.push(Application::new(app_config)?);
        }

        Ok(Self {
            config,
            applications,
        })
    }
}
