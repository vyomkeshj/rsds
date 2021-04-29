/**
    Has the log trait which contains the log <K,V> pair
    provides a macro that allows the user to choose a logger backend (KafkaLogger/logserver_logger)

    or how the logger is set up depends on a macro
*/

use serde::{Deserialize, Serialize};
use tokio::time;
use tokio::time::Instant;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct RsdsLog<K, V: Serialize + Deserialize> {
    log_key: K,
    log_value: V,
    log_timestamp: time::Instant
}

pub trait Logger {
    fn log(key: &str, value:Box<dyn Serialize>);   //logs data to the selected logger
}
