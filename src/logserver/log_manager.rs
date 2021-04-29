use std::sync::Arc;
use tokio::sync::Mutex;
use crate::logger::logger::RsdsLog;

pub type MutexedSchedulerStatus = Arc<Mutex<RsdsLog<K, V>>>;

pub fn init_status() -> MutexedSchedulerStatus {
            Arc::new(Mutex::new(SchedulerStatus::new("343".to_string(), 45, 65, "email".to_string())))
}