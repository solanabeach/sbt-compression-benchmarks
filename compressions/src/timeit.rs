


use std::time::{Instant, Duration};
pub fn timer(){

    let then =  Instant::now();
    ( ||{std::thread::sleep(Duration::from_secs(3))} )();
    println!("seconds elapsed : {:?}", then.elapsed().as_secs());

}