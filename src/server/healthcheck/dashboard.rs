use warp;
use crate::server::healthcheck::{status_manager, routes};
use crate::server::core::CoreRef;

//todo: initialize status manager from outside and pass reference
#[tokio::main]
pub async fn run_dashboard() {
    let status_manager = status_manager::init_status();
    let dashboard_routes = routes::register_dashboard_routes(status_manager);

    print!("Starting healthcheck at http://127.0.0.1:3047/status");

    warp::serve(dashboard_routes)
        .run(([127, 0, 0, 1], 3047))
        .await;
}
