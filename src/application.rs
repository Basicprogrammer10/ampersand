use anyhow::Result;

use crate::config::ApplicationConfig;

pub struct Application {
    config: ApplicationConfig,
}
impl Application {
    pub fn new(config: ApplicationConfig) -> Result<Self> {
        todo!()
    }
}
