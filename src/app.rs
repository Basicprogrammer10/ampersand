use std::{fs, path::PathBuf};

use anyhow::Result;
use docker_api::{opts::PullOpts, Docker};
use futures::executor::block_on_stream;
use tokio::runtime::Runtime;

use crate::{
    application::Application,
    config::{ApplicationConfig, Config},
};

pub struct App {
    pub config: Config,
    pub applications: Vec<Application>,
    docker: Docker,
    runtime: Runtime,
}

impl App {
    pub fn new(config_path: PathBuf) -> Result<Self> {
        let config_raw = fs::read_to_string(&config_path)?;
        let config = toml::from_str::<Config>(&config_raw)?;

        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_time()
            .enable_io()
            .build()?;

        let docker = Docker::new(&config.docker_address)?;
        // if let Err(e) = runtime.block_on(docker.info()) {
        //     return Err(anyhow::anyhow!("Failed to connect to docker: {}", e));
        // }

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
            docker,
            runtime,
        })
    }

    pub fn get_application(&self, name: &str) -> Option<&Application> {
        self.applications.iter().find(|i| i.config.name == name)
    }

    // todo:
    // - Pull new image from the defined package
    // - Stop the current container
    // - Rename the current container to like {app}-old
    // - Run the new container with the specified docker args
    pub fn deploy(&self, application: &Application) -> Result<()> {
        println!("Deploying application `{}`", application.config.name);

        // Pull new image
        let mut pull_opts = PullOpts::builder().image(&application.config.package);
        if let Some(auth) = &application.config.package_auth {
            pull_opts = pull_opts.auth(auth.clone().into());
        }

        let images = self.docker.images();
        let pull = images.pull(&pull_opts.build());
        let mut stream = block_on_stream(pull);
        while let Some(i) = stream.next() {
            println!("{:?}", i);
        }

        Ok(())
    }
}
