use afire::{Method, Response, Server};

use crate::app::App;

pub fn attach(server: &mut Server<App>) {
    server.stateful_route(Method::POST, "/api/{app}/deployments", |app, req| {
        let app_name = req.param("app").unwrap();
        let Some(application)  = app.get_application(&app_name) else {
            return Response::new()
                .text(format!("Application `{app_name}` not found."))
                .status(404);
        };

        let Some(auth) = req.headers.get("Authorization") else {
            return Response::new()
                .text("No authorization header provided.")
                .status(401);
        };

        if auth != application.config.api_secret {
            return Response::new()
                .text("Invalid authorization header provided.")
                .status(401);
        }

        app.deploy(&application);

        Response::new()
    });
}
