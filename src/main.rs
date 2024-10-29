use core::temperature::Temperature;
use std::{
    collections::HashMap,
    error::Error,
    sync::{Arc, Mutex},
};
pub mod core;

fn main() -> Result<(), Box<dyn Error>> {
    let map: Arc<Mutex<HashMap<String, Temperature>>> = Arc::new(Mutex::new(HashMap::new()));
    let start_time = std::time::Instant::now();
    println!("Start time: {:?}", start_time.clone());
    if let Err(e) = core::processor::run(map) {
        eprintln!("Error in run function: {:?}", e);
    }
    println!("Time elapsed: {:?}", start_time.elapsed());
    Ok(())
}
