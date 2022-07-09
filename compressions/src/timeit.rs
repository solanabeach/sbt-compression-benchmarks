

use std::{time::{Instant, Duration}, fs::File, 
io::{ Error, Write, Read,BufRead, BufReader}};


pub fn write_results(results_path:&str)->Result<(), Error>{
    let path       = "/home/rxz/dev/sb-actix-lib/gzip_1.txt";
    let mut output = File::create(path)?;
    write!(output, "migration_total\t: {}\nmigration_msg_avg\t: {}", 20,30)?;
    println!("Wrote to {}", path);
    Ok(())

}

pub fn timer()->Result<(), Error>{

    let then =  Instant::now();
    ( ||{std::thread::sleep(Duration::from_secs(3))} )();
    println!("seconds elapsed : {:?}", then.elapsed().as_secs());


    Ok(())


}