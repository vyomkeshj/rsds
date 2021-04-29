use std::time::Duration;

use log::info;

use rdkafka::config::ClientConfig;
use rdkafka::message::OwnedHeaders;
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::util::get_rdkafka_version;

use structopt::clap::App;
use serde::{Serialize, Deserialize};
use crate::logger::logger::Logger;


/*impl KafkaLogger for dyn Logger {

    fn log(key: &str, value: Box<dyn Serialize>) {

    }

}
*/
//let topic = "quickstart-events";
//let brokers = "127.0.0.1:9092";

pub async fn produce(brokers: &str, topic_name: &str/*, log: Box<dyn Loggable>*/) { //fixme: why?
    let producer: &FutureProducer = &ClientConfig::new()
        .set("bootstrap.servers", brokers)
        .set("message.timeout.ms", "5000")
        .create()
        .expect("Producer creation error");

    // This loop is non blocking: all messages will be sent one after the other, without waiting
    // for the results.
    let futures = async move {
            // The send operation on the topic returns a future, which will be
            // completed once the result or failure from Kafka is received.
            let delivery_status = producer
                .send(
                    FutureRecord::to(topic_name)
                        .payload("log.get_value()") //fixme: this is the stuff that is sent
                        .key("log.get_key()"),
                    Duration::from_secs(0),
                )
                .await;
            delivery_status
        }
        .collect::<Vec<_>>();

    // This loop will wait until all delivery statuses have been received.
    for future in futures {
        info!("Future completed. Result: {:?}", future.await);
    }
}

