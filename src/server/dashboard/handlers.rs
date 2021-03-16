use std::convert::Infallible;

use crate::server::dashboard::status_model::SchedulerStatus;
use crate::server::dashboard::status_manager::MutexedSchedulerStatus;

/// Returns DashboardStatus JSON
pub async fn get_status(scheduler_status: MutexedSchedulerStatus) -> Result<impl warp::Reply, Infallible> {
    let status = scheduler_status.lock().await;
    let status: SchedulerStatus = status.clone();
    Ok(warp::reply::json(&status))
}