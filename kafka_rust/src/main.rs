use kafka::consumer::{Consumer, FetchOffset, GroupOffsetStorage};
use kafka::error::Error as KafkaError;
use std::{thread, time};
use std::str::from_utf8;


fn main() {
    let broker = "localhost:9092".to_owned();
    let topic = "test".to_owned();
    let group = "test-group".to_owned();

    if let Err(e) = consume_messages(group, topic, vec![broker]) {
        println!("Failed consuming messages: {}", e);
    }


}


fn consume_messages(group: String, topic: String, brokers: Vec<String>) -> Result<(), KafkaError> {
    let mut con = Consumer::from_hosts(brokers)
    .with_topic(topic)
    .with_group(group)
    .with_fallback_offset(FetchOffset::Earliest)
    .with_offset_storage(GroupOffsetStorage::Kafka)
    .create()?;

    loop {
        let mss = con.poll()?;
        if mss.is_empty() {
            println!("No messages available!");
            thread::sleep(time::Duration::from_millis(1000));
        }

        for ms in mss.iter() {
            for m in ms.messages() {
                println!("{}:{}@{} {:?}", ms.topic(), ms.partition(), m.offset, std::str::from_utf8(m.value));
            }
            let _ = con.consume_messageset(ms);
        }

        con.commit_consumed()?;
    }

}
