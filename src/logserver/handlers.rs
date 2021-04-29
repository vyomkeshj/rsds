use std::convert::Infallible;
use crate::logserver::log_manager::MutexedSchedulerStatus;


pub async fn put_log(scheduler_status: MutexedSchedulerStatus) -> Result<impl warp::Reply, Infallible> {
    let status = scheduler_status.lock().await;
    let status: MutexedSchedulerStatus = status.clone();
    Ok(warp::reply::json(&status))
}