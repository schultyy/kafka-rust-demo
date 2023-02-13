use kafka::consumer::{Consumer, FetchOffset};

fn main() {
    let mut consumer =
       Consumer::from_hosts(vec!("broker:9092".to_owned()))
          .with_topic("hnstories".to_owned())
          .with_fallback_offset(FetchOffset::Earliest)
          .create()
          .unwrap();
    loop {
      for ms in consumer.poll().unwrap().iter() {
        for m in ms.messages() {
          let str = String::from_utf8_lossy(m.value);
          println!("{:?}",str);
        }
        let _ = consumer.consume_messageset(ms);
      }
      consumer.commit_consumed().unwrap();
    }
}
