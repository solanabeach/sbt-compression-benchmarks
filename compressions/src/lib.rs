use std::process::exit;
use std::time::Duration;
use rdkafka::config::FromClientConfig;
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::error::KafkaError;
use rdkafka::util::Timeout;
use rdkafka::{ClientConfig, Message, Offset, TopicPartitionList};
pub mod timeit;
use rdkafka::producer::{FutureProducer, FutureRecord};


// pub fn read_files() -> io::Result<()> {
//     use std::path::PathBuf;
//     // let datapath = Path::new("").join(std::env::current_exe().unwrap()).join("samples");
//     let datapath = "/home/rxz/dev/sb-actix-lib/sample-data";
//     println!("attemptint to open {}", datapath);
//     let mut rd = read_dir(datapath)?
//         .map(|readdir| readdir.map(|p| p.path()))
//         .collect::<io::Result<Vec<PathBuf>>>()?;

//     for blockpath in rd.iter() {
//         println!("------------------------------------------------------------------");
//         println!("Processing blockpath: {}", blockpath.display());

//         let mut reader = BufReader::new(File::open(blockpath)?);
//         let mut block = String::new();

//         reader.read_to_string(&mut block);
//         let mut block_parsed: Value = serde_json::from_str(&block)?;
//         // println!("block_parsed: {:?}", block_parsed);
//     }

//     Ok(())
// }

pub enum KafkaCompressionType{
    Null,
    Gzip,
    Snappy,
    Lz4,
    Zstd
}

fn instantiate_cons_prod( kfk_compression_type:KafkaCompressionType, compression_level:u64)->Result<(StreamConsumer, FutureProducer), KafkaError>{
    let bootstrap_hosts = "127.0.0.1:9095";
    let default_timeout = Timeout::After(Duration::from_millis(15000));
    let group_id = "compression-benchmarks";

    let mut compression_type:&str;

    match kfk_compression_type{
        Null   => compression_type = "none",
        Gzip   => compression_type = "gzip",
        Snappy => compression_type = "snappy",
        Lz4    => compression_type = "lz4",
        Zstd   => compression_type = "zstd"
    }

    let mut producer_config = ClientConfig::new();

    producer_config.set("bootstrap.servers"           , bootstrap_hosts );
    producer_config.set("group.id"                    , group_id        );
    producer_config.set("statistics.interval.ms"      , "500"           );
    producer_config.set("compression.type"            , compression_type);
    producer_config.set("compression.level"            , u64::to_string(&compression_level));
    producer_config.set("enable.idempotence"          , "true"          ); // required for keeping msgs sequential
    producer_config.set("queue.buffering.max.messages", "10000000"      ); // 100k msgs buffered
    producer_config.set("queue.buffering.max.kbytes"  , "2047483647"    ); // 2GB buffered
    producer_config.set("message.max.bytes"           , "500000000"     ); // max message size 500MB
    let producer =
        FutureProducer::from_config(&producer_config).expect("Failed to created producer");

    let mut consumer_config = ClientConfig::new();
    consumer_config.set("bootstrap.servers"          , bootstrap_hosts);
    consumer_config.set("group.id"                   , group_id       );
    consumer_config.set("socket.send.buffer.bytes"   , "5048576"      );
    consumer_config.set("socket.receive.buffer.bytes", "5048576"      );
    consumer_config.set("queued.max.messages.kbytes" , "2096151"      );
    let consumer: StreamConsumer =
        StreamConsumer::from_config(&consumer_config).expect("Couldn't create consumer");
    Ok((consumer, producer))
}


// compression.level = -1 .. 12 	default = -1 	medium
// Compression level parameter for algorithm selected by configuration property compression.codec. 
// Higher values will result in better compression at the cost of more CPU usage.
// Usable range is algorithm-dependent: [0-9] for gzip; [0-12] for lz4; only 0 for snappy; 
// -1 = codec-dependent default compression level.
// Type: integer

pub async fn unencoded_to_gzip(compression_level:usize) {

    let default_timeout          = Timeout::After(Duration::from_millis(15000));

    let (consumer,producer) = instantiate_cons_prod( KafkaCompressionType::Gzip,compression_level as u64).unwrap();
    let source_topic        = "compression_data_control";
    let destination_topic   = format!("compression_gzip_{}", compression_level);

    let mut topic_partition_list = TopicPartitionList::new();
    topic_partition_list.add_partition(source_topic, 0);
    topic_partition_list.set_partition_offset(source_topic, 0, Offset::Beginning)
    .expect("failed to set partition message");
    consumer.assign(&topic_partition_list).expect("Couldn't assign consumer");

    let mut total_elapsed = std::time::Instant::now();
    let mut message_count = 0;
    let commit_times = async {
        let mut times_to_commit:Vec<u128> = vec![]; // let this be a proxy for userspace 

        loop {
            let msg = consumer.recv().await.unwrap();
            // println!("Offset :{:?}", msg.offset());
            // let offset           = msg.offset().to_le_bytes();
            let inner_instant = std::time::Instant::now();
            let r:FutureRecord<_, [u8]> = FutureRecord::to(&destination_topic).payload(msg.payload().unwrap()).key(&());
            let future           = producer.send(r, Timeout::After(Duration::from_millis(15000)));
            if  future.await.map_or(-1, |_| {println!("Processed {} successfully.", &message_count); 0}) < 0{
                println!("Message {} processing failed.", message_count);
            }else{
                times_to_commit.push(inner_instant.elapsed().as_millis());
            }

            message_count+=1;
            if message_count == 5000{
                println!("Migrated all 5000 mesages to topic {}. Exiting.", destination_topic);
                break
            }
        }
        times_to_commit
    }.await;

    let migration_total   = total_elapsed.elapsed();
    let migration_msg_avg = commit_times.iter().fold(0, |acc,next| acc+next)/commit_times.len() as u128;
    

}



pub async fn funnel_data_off() {
    let topic = "modern_blocks_json";
    let bootstrap_hosts = "127.0.0.1:9095";
    let default_timeout = Timeout::After(Duration::from_millis(15000));

    let mut producer_config = ClientConfig::new();
    producer_config.set("bootstrap.servers", bootstrap_hosts);
    producer_config.set("group.id", "compression-benchmarks");
    producer_config.set("statistics.interval.ms", "500");
    // producer_config.set("compression.type", "gzip");
    producer_config.set("compression.type", "none");
    producer_config.set("enable.idempotence", "true"); // required for keeping msgs sequential
    producer_config.set("queue.buffering.max.messages", "10000000"); // 100k msgs buffered
    producer_config.set("queue.buffering.max.kbytes", "2047483647"); // 2GB buffered
    producer_config.set("message.max.bytes", "500000000"); // max message size 500MB
    let producer =
        FutureProducer::from_config(&producer_config).expect("Failed to created producer");

    let mut consumer_config = ClientConfig::new();
    consumer_config.set("bootstrap.servers", bootstrap_hosts);
    consumer_config.set("group.id", "default");
    consumer_config.set("socket.send.buffer.bytes", "5048576");
    consumer_config.set("socket.receive.buffer.bytes", "5048576");
    consumer_config.set("queued.max.messages.kbytes", "2096151");
    let consumer: StreamConsumer =
        StreamConsumer::from_config(&consumer_config).expect("Couldn't create consumer");
    let mut topic_partition_list = TopicPartitionList::new();
    topic_partition_list.add_partition(topic, 0);
    match topic_partition_list.set_partition_offset(topic, 0, Offset::OffsetTail(1374)) {
        Ok(_) => {}
        Err(e) => {
            println!("Error setting partition offset: {}", e);
            return;
        }
    };
    consumer
        .assign(&topic_partition_list)
        .expect("Couldn't assign consumer");

    let mut count = 0;
    async {
        loop {
            let msg = consumer.recv().await.unwrap();
            println!("Offset :{:?}", msg.offset());
            let dest_topic = "compression_data_control";
            let offset     =  msg.offset().to_le_bytes();
            let r:FutureRecord<_, [u8]> = FutureRecord::to(dest_topic)
                .payload(msg.payload().unwrap()).key(&());

            let future = producer.send(r, Timeout::After(Duration::from_millis(15000)));
            future.await.map_or(-1, |_| {println!("Processed offset {:?} successfully.", &offset); 0});
            count+=1;
            if count == 5000{
                println!("Migrated 5000 mesages to topic {}. Exiting.", dest_topic);
                exit(0);
            }
        }
    }.await;
}

// pub async fn gzip() {
//     let bootstrap_hosts = "127.0.0.1:9095";

//     let mut config = ClientConfig::new();
//     config.set("bootstrap.servers", bootstrap_hosts);
//     config.set("group.id", "compression-benchmarks");
//     config.set("statistics.interval.ms", "500");
//     config.set("compression.type", "none");
//     config.set("enable.idempotence", "true"); // required for keeping msgs sequential
//     config.set("queue.buffering.max.messages", "10000000"); // 100k msgs buffered
//     config.set("queue.buffering.max.kbytes", "2047483647"); // 2GB buffered
//     config.set("message.max.bytes", "500000000"); // max message size 500MB

//     let producer = FutureProducer::from_config(&config).expect("Failed to created producer");
//     let mut r = FutureRecord::to("gzip_benchmark");
//     let _payload = &b"hello world".to_vec();
//     let _key = &75_u64.to_le_bytes();
//     r.payload = Some(_payload);
//     r.partition = Some(0);
//     r.key = Some(_key);

//     async {
//         let res = producer.send(r, Timeout::After(Duration::from_millis(15000)));
//         let res = res.await.map_or(-1, |_| {
//             println!("Delivered message");
//             1
//         });
//     }
//     .await
// }
