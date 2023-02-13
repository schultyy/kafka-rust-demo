use kafka::producer::{Producer, Record, RequiredAcks};
use std::time::Duration;
use crate::hn::HNSearchResult;

mod hn;

fn send_to_kafka(host: &str, topic: &str, payload: Vec<HNSearchResult>) {
    let mut producer = Producer::from_hosts(vec![host.to_owned()])
        .with_ack_timeout(Duration::from_secs(1))
        .with_required_acks(RequiredAcks::One)
        .create()
        .unwrap();

    for search_result in payload {
        let buffer = serde_json::to_string(&search_result).unwrap();

        producer
            .send(&Record::from_value(topic, buffer.as_bytes()))
            .unwrap();
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stories = hn::fetch_hn_stories("Ruby".into(), 100).await?;
    println!("Fetched {} stories", stories.hits.len());
    send_to_kafka("broker:9092", "hnstories", stories.hits);
    Ok(())
}
