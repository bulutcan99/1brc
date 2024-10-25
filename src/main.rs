use core::temperature::Temperature;
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

use simple_logger::SimpleLogger;

pub mod core;

fn main() -> Result<(), anyhow::Error> {
    SimpleLogger::new().init().unwrap();
    let file = File::open("measurements.txt")?;
    let readers = BufReader::new(file);
    let mut map: HashMap<String, Temperature> = HashMap::new();
    let mut count = 0;
    let start_time = std::time::Instant::now();
    log::info!("Reading file");

    for line in readers.lines() {
        count += 1;
        let line = line?;
        let parts: Vec<&str> = line.split(';').collect();
        if parts.len() < 2 {
            continue;
        }
        let key = parts[0].to_owned();
        let value = parts[1].parse::<f64>()?;
        map.entry(key)
            .and_modify(|prev| prev.update(value))
            .or_insert(Temperature::new(value));
    }

    log::info!("Processed {} lines", count);
    log::info!("Time elapsed: {:?}", start_time.elapsed());
    Ok(())
}
