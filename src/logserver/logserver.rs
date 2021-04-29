use warp;
use crate::server::core::CoreRef;
use crate::logserver::{log_manager, routes};

//todo: initialize status manager from outside and pass reference

/**
*   get things from the put endpoint and store it into a database
*/
#[tokio::main]
pub async fn run_dashboard() {
    let logger_route = log_manager::init_status();
    let logger_route = routes::register_logger_route(logger_route);

    print!("Starting logserver at http://127.0.0.1:3047/status");

    warp::serve(logger_route)
        .run(([127, 0, 0, 1], 3047))
        .await;
}
