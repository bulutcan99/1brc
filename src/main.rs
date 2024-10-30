#![feature(slice_split_once)]

use core::temperature::Temperature;
use std::{collections::HashMap, error::Error};
pub mod core;

fn main() -> Result<(), Box<dyn Error>> {
    let start_time = std::time::Instant::now();
    if let Err(e) = core::processor::run() {
        eprintln!("Error in run function: {:?}", e);
    }
    println!("Time elapsed: {:?}", start_time.elapsed());
    print!("Finished!");
    Ok(())
}
