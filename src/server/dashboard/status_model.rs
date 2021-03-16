use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SchedulerStatus {
    pub scheduler_address: String,
    pub number_of_workers: i32,
    pub number_of_tasks: i32,
    pub current_status: String,
}

impl SchedulerStatus {

    #[inline]
    pub fn new(addr: String,
               num_workers: i32,
               num_tasks: i32,
               status: String,
    ) -> Self {
        SchedulerStatus { scheduler_address: addr, number_of_workers: num_workers, number_of_tasks: num_tasks, current_status: status }}

}

