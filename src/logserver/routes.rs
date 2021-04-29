use std::convert::Infallible;
use warp::{self, Filter};

use crate::server::healthcheck::handlers;
use crate::logserver::log_manager::MutexedSchedulerStatus;


pub fn register_logger_route(
    db: MutexedSchedulerStatus,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    get_scheduler_status(db)
}

/// GET /status
fn get_scheduler_status(
    server_status: MutexedSchedulerStatus,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("log")
        .and(warp::put())
        .and(provide_status(server_status))      //fixme: remove
        .and_then(handlers::get_status)
}

fn provide_status(status: MutexedSchedulerStatus) -> impl Filter<Extract = (MutexedSchedulerStatus,), Error = Infallible> + Clone {
    warp::any().map(move || status.clone())
}
