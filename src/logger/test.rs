use std::time::Duration;

use log::info;

use rdkafka::config::ClientConfig;
use rdkafka::message::OwnedHeaders;
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::util::get_rdkafka_version;

use structopt::clap::App;

#[tokio::main]
async fn main() {
    let matches = App::new("starting rsds_kafka logger").get_matches();
    //setup_logger(true, matches.value_of("log-conf"));


    let topic = "quickstart-events";
    let brokers = "127.0.0.1:9092";



    //produce(brokers, topic, "message").await;
}