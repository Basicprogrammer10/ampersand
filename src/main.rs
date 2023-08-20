use std::{env, path::PathBuf};

use afire::Server;
use anyhow::{Context, Result};

use app::App;
mod app;
mod application;
mod config;
mod misc;
mod routes;

fn main() -> Result<()> {
    let config_path = env::args().nth(1).context("No config path provided")?;
    let app = App::new(PathBuf::from(config_path))?;

    println!(
        "Loaded {} applications from {:?}",
        app.applications.len(),
        app.config.config_path
    );
    for i in &app.applications {
        println!(" - {}", i.config.name);
    }

    // TODO: Try to startup any containers that are not running

    println!(
        "\nStarting server on {}:{}",
        app.config.server.host, app.config.server.port
    );

    let threads = app.config.server.threads;
    let mut server = Server::new(&app.config.server.host, app.config.server.port).state(app);
    routes::attach(&mut server);

    server.start_threaded(threads).unwrap();
    Ok(())
}
