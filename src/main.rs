use std::{env, path::PathBuf};

use anyhow::{Context, Result};

use app::App;

mod app;
mod application;
mod config;

fn main() -> Result<()> {
    let config_path = env::args().nth(1).context("No config path provided")?;
    let app = App::new(PathBuf::from(config_path))?;

    Ok(())
}
