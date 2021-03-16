use std::convert::Infallible;
use warp::{self, Filter};

use crate::server::dashboard::status_manager::MutexedSchedulerStatus;
use crate::server::dashboard::handlers;


pub fn register_dashboard_routes(
    db: MutexedSchedulerStatus,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    get_scheduler_status(db)
}

/// GET /status
fn get_scheduler_status(
    server_status: MutexedSchedulerStatus,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("status")
        .and(warp::get())
        .and(provide_status(server_status))
        .and_then(handlers::get_status)
}

fn provide_status(status: MutexedSchedulerStatus) -> impl Filter<Extract = (MutexedSchedulerStatus,), Error = Infallible> + Clone {
    warp::any().map(move || status.clone())
}
