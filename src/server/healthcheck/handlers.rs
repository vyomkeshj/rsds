use std::convert::Infallible;
use crate::server::healthcheck::status_manager::MutexedSchedulerStatus;
use crate::server::healthcheck::core_dump::SchedulerStatus;


pub async fn get_status(scheduler_status: MutexedSchedulerStatus) -> Result<impl warp::Reply, Infallible> {
    let status = scheduler_status.lock().await;
    let status: SchedulerStatus = status.clone();
    Ok(warp::reply::json(&status))
}