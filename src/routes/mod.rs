use afire::Server;

use crate::app::App;

mod deployments;

pub fn attach(server: &mut Server<App>) {
    deployments::attach(server);
}
