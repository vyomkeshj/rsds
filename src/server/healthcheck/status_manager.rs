use std::sync::Arc;
use tokio::sync::Mutex;
use crate::server::healthcheck::core_dump::SchedulerStatus;


pub type MutexedSchedulerStatus = Arc<Mutex<SchedulerStatus>>;

pub fn init_status() -> MutexedSchedulerStatus {
            Arc::new(Mutex::new(SchedulerStatus::new("343".to_string(),
                                                     45,
                                                     65
                                                     , "email".to_string()
            )))

}
