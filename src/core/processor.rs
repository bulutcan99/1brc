use crate::core::pool::ThreadPool;

use super::temperature::Temperature;
use std::{
    collections::HashMap,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    sync::{Arc, Mutex},
};

const FILENAME: &str = "measurements.txt";
const POOL_SIZE: usize = 10;

pub fn run(map: Arc<Mutex<HashMap<String, Temperature>>>) -> Result<(), Box<dyn Error>> {
    let file = File::open(FILENAME)?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    let total_lines = lines.by_ref().count();
    let chunk_size = total_lines / POOL_SIZE; // Number of lines per thread

    let file = File::open(FILENAME)?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    let pool = Arc::new(ThreadPool::new(POOL_SIZE));
    let mut chunk_lines = Vec::with_capacity(chunk_size);

    while let Some(line) = lines.next() {
        let line = line?;
        chunk_lines.push(line);
        let pool = Arc::clone(&pool);
        if chunk_lines.len() >= chunk_size {
            let chunk = chunk_lines.clone();
            let map_clone = Arc::clone(&map);
            pool.execute(move || {
                if let Err(e) = process_chunk(chunk, map_clone) {
                    eprintln!("Error processing chunk: {:?}", e);
                }
                Ok(())
            })?;
            chunk_lines.clear();
        }
    }
    if !chunk_lines.is_empty() {
        let map_clone = Arc::clone(&map);
        pool.execute(move || {
            if let Err(e) = process_chunk(chunk_lines, map_clone) {
                eprintln!("Error processing remaining lines: {:?}", e);
            }
            Ok(())
        })?;
    }
    println!("Finished processing all lines");
    Ok(())
}

fn process_chunk(
    chunk: Vec<String>,
    map: Arc<Mutex<HashMap<String, Temperature>>>,
) -> Result<(), Box<dyn std::error::Error>> {
    for data in chunk {
        let parts: Vec<&str> = data.split(';').collect();
        if parts.len() < 2 {
            eprintln!("Invalid data format: {}", data);

            continue;
        }

        let key = parts[0].to_string();
        let temp: f64 = parts[1].parse().map_err(|e| {
            eprintln!("Failed to parse temperature for {}: {}", key, e);
            e
        })?;

        let mut temp_map = map.lock().unwrap();
        if let Some(temperature) = temp_map.get_mut(&key) {
            temperature.update(temp);
        } else {
            temp_map.insert(key.clone(), Temperature::new(temp));
        }
    }
    Ok(())
}
