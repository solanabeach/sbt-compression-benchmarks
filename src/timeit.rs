

use std::{time::{Instant, Duration}, fs::File, 
io::{ Error, Write, Read,BufRead, BufReader}};


pub fn write_results(results_path:&str,total:u128, msg_avg:u128 )->Result<(), Error>{
    let mut output = File::create(results_path)?;
    write!(output, "migration_total\t: {}\nmigration_msg_avg\t: {}", total,msg_avg)?;
    println!("Wrote to {}", results_path);
    Ok(())
}

pub fn timer()->Result<(), Error>{

    let then =  Instant::now();
    ( ||{std::thread::sleep(Duration::from_secs(3))} )();
    println!("seconds elapsed : {:?}", then.elapsed().as_secs());


    Ok(())


}