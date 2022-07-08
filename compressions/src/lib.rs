use std::collections::HashMap;
use std::fs::{read_dir, File};
use std::io::{self, BufReader, Read};
use std::time::Duration;

use rdkafka::config::FromClientConfig;
use rdkafka::consumer::{StreamConsumer, Consumer};
use rdkafka::message::ToBytes;
use rdkafka::util::Timeout;
use rdkafka::{ClientConfig, TopicPartitionList, Message, Offset};
use sb_backend_3_actix::actors::basic::AtrIOWrite;
use sb_backend_3_actix::actors::basic::{AtrStringToSerde, AtrUtf8ToString};
use sb_backend_3_actix::actors::kafka::{AtrKafkaConsumer, MsgJumpToOffset};
use sb_backend_3_actix::actors::sb_actor::SBActor;
use sb_backend_3_actix::actors::sb_atr_wrapper::ActorWrap;

use rdkafka::producer::{BaseProducer, FutureProducer, FutureRecord};
use sb_backend_3_actix::messages::MsgVoid;
use serde_json::Value;

// #[actix::main]
// pub async fn get_data() {
//     println!("Hihih from libr.s");
//     let topic: &'static str = "modern_blocks_json";
//     let actor_iowrite = AtrIOWrite::new().wrap();
//     let consumer_w =
//         AtrKafkaConsumer::new_simple_hosts(topic, actor_iowrite, "localhost:9095").wrap();

//     consumer_w.do_send(MsgJumpToOffset { offset: 5_000_000 });
//     loop {
//         let mut ival = actix::clock::interval(Duration::from_millis(1000));
//         ival.tick().await;
//         consumer_w.do_send(MsgVoid {});
//     }
// }

pub fn read_files() -> io::Result<()> {
    use std::path::PathBuf;
    // let datapath = Path::new("").join(std::env::current_exe().unwrap()).join("samples");
    let datapath = "/home/rxz/dev/sb-actix-lib/sample-data";
    println!("attemptint to open {}", datapath);
    let mut rd = read_dir(datapath)?
        .map(|readdir| readdir.map(|p| p.path()))
        .collect::<io::Result<Vec<PathBuf>>>()?;

    for blockpath in rd.iter() {
        println!("------------------------------------------------------------------");
        println!("Processing blockpath: {}", blockpath.display());

        let mut reader = BufReader::new(File::open(blockpath)?);
        let mut block = String::new();

        reader.read_to_string(&mut block);
        let mut block_parsed: Value = serde_json::from_str(&block)?;
        // println!("block_parsed: {:?}", block_parsed);
    }

    Ok(())
}

pub async fn funnel_data_off() {
    let topic           = "modern_blocks_json";
    let partion         = 0;
    let bootstrap_hosts = "127.0.0.1:9095";

    let mut config = ClientConfig::new();
    config.set("bootstrap.servers", bootstrap_hosts);
    config.set("group.id", "default");
    config.set("socket.send.buffer.bytes", "5048576");
    config.set("socket.receive.buffer.bytes", "5048576");
    config.set("queued.max.messages.kbytes", "2096151");


    let consumer: StreamConsumer = StreamConsumer::from_config(&config).expect("Couldn't create consumer");
    let mut topic_partition_list = TopicPartitionList::new();
    topic_partition_list.add_partition(topic, 0);
    match topic_partition_list.set_partition_offset(topic, 0, Offset::OffsetTail(5000)){
        Ok(_) => {},
        Err(e) => {
            println!("Error setting partition offset: {}", e);
            return;
        }
    };
    consumer.assign(&topic_partition_list).expect("Couldn't assign consumer");
    let default_timeout = Timeout::After(Duration::from_millis(15000));
    // match consumer.seek(&topic, 0,rdkafka::Offset::OffsetTail(5000), default_timeout.clone()){
    //     Ok(_) => {
    //         println!("Seeked to offset 5000000");
    //     }
    //     Err(e) => {
    //         println!("Error seeking to offset 5000000: {:?}", e);
    //     }
    // }
    let msg         = consumer.recv().await.unwrap();
    let msg_payload = msg.payload().unwrap();
    println!("Offset :{:?}", msg.offset());
    // println!("Key :{:?}", u64::from_le_bytes(msg.key().unwrap().clone().split_at(8).0.try_into().unwrap()));
}

pub async fn gzip() {
    let topic           = "gzip_benchmark";
    let partion         = 0;
    let bootstrap_hosts = "127.0.0.1:9095";

    let mut config = ClientConfig::new();
    config.set("bootstrap.servers", bootstrap_hosts);
    config.set("group.id", "compression-benchmarks");
    config.set("statistics.interval.ms", "500");
    config.set("compression.type", "gzip");
    config.set("enable.idempotence", "true"); // required for keeping msgs sequential
    config.set("queue.buffering.max.messages", "10000000"); // 100k msgs buffered
    config.set("queue.buffering.max.kbytes", "2047483647"); // 2GB buffered
    config.set("message.max.bytes", "500000000"); // max message size 500MB

    let producer    = FutureProducer::from_config(&config).expect("Failed to created producer");
    let mut r       = FutureRecord::to("gzip_benchmark");
    let _payload    = &b"hello world".to_vec();
    let _key        = &75_u64.to_le_bytes();
        r.payload   = Some(_payload);
        r.partition = Some(0);
        r.key       = Some(_key);

    async {
        let res = producer.send(r, Timeout::After(Duration::from_millis(15000)));
        let res = res.await.map_or(-1, |_| {
            println!("Delivered message");
            1
        });
    }
    .await
}
