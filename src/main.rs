use core::temperature::Temperature;
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    sync::Arc,
};

use futures::lock::Mutex;
use simple_logger::SimpleLogger;

pub mod core;

const FILENAME: &str = "measurements.txt";
const TOTAL_LINES: usize = 1_000_000_000;
const CHUNK_SIZE: usize = 1000;

async fn process_chunk(chunk: Vec<String>, map: Arc<Mutex<HashMap<String, Temperature>>>) {}

async fn run(map: Arc<Mutex<HashMap<String, Temperature>>>) -> Result<(), anyhow::Error> {
    let file = File::open(FILENAME)?;
    let readers = BufReader::new(file);
    let mut chunk_lines = Vec::with_capacity(CHUNK_SIZE);
    let mut lines = readers.lines();
    for line in lines {}
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    SimpleLogger::new().init().unwrap();
    let map: Arc<Mutex<HashMap<String, Temperature>>> = Arc::new(Mutex::new(HashMap::new()));
    let mut count: u32 = 0;
    let start_time = std::time::Instant::now();
    log::info!("Reading file");
    run(map, &mut count).await?;

    log::info!("Processed {} lines", count);
    log::info!("Time elapsed: {:?}", start_time.elapsed());
    Ok(())
}
