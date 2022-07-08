use std::time::Duration;

use sb_backend_3_actix::actors::basic::{AtrStringToSerde, AtrUtf8ToString};
use sb_backend_3_actix::actors::kafka::{ AtrKafkaConsumer, MsgJumpToOffset };
use sb_backend_3_actix::actors::sb_actor::SBActor;
use sb_backend_3_actix::actors::sb_atr_wrapper::ActorWrap;
use sb_backend_3_actix::actors::{basic::AtrIOWrite};

use actix;
use sb_backend_3_actix::messages::MsgVoid;

#[actix::main]
pub async fn get_data(){

    println!("Hihih from libr.s");
    let topic                           :& 'static str= "modern_blocks_json";
    let actor_iowrite = AtrIOWrite      ::new().wrap();
    let consumer_w    = AtrKafkaConsumer::new_simple_hosts(topic, actor_iowrite, "localhost:9095").wrap();

    consumer_w.do_send(MsgJumpToOffset { offset: 5_000_000 });
    loop {
        let mut ival = actix::clock::interval(Duration::from_millis(1000));
        ival.tick().await;
        consumer_w.do_send( MsgVoid{} );
    }
}
