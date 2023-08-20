use anyhow::Result;

use crate::config::ApplicationConfig;

pub struct Application {
    pub config: ApplicationConfig,
}
impl Application {
    pub fn new(config: ApplicationConfig) -> Result<Self> {
        Ok(Self { config })
    }
}
