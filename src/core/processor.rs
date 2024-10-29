use super::{temperature::Temperature, thread_pool::ThreadPool};
use std::{
    collections::HashMap,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    sync::{Arc, Mutex},
    time::Duration,
};

const FILENAME: &str = "measurements-mini.txt";
const POOL_SIZE: usize = 10;

pub fn run(map: Arc<Mutex<HashMap<String, Temperature>>>) -> Result<(), Box<dyn Error>> {
    let file = File::open(FILENAME)?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    // No need to count if we are directly iterating over `lines`
    let total_lines = lines.by_ref().count(); // `total_lines` is returned as `usize`
    let chunk_size = total_lines / POOL_SIZE; // Number of lines per thread

    // To start again from the beginning, we need a new `BufReader`
    let file = File::open(FILENAME)?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    let mut pool = ThreadPool::new(POOL_SIZE);
    let mut chunk_lines = Vec::with_capacity(chunk_size);

    while let Some(line) = lines.next() {
        let line = line?; // Perform error handling
        chunk_lines.push(line);

        // If enough lines have been collected, send them to a thread
        if chunk_lines.len() >= chunk_size {
            let chunk = chunk_lines.clone();
            let map_clone = Arc::clone(&map);
            pool.execute(move || {
                process_chunk(chunk, map_clone)?;
                Ok(())
            })?;

            chunk_lines.clear(); // Clear collected lines
        }
    }

    // Process remaining lines
    if !chunk_lines.is_empty() {
        let map_clone = Arc::clone(&map);
        pool.execute(move || {
            process_chunk(chunk_lines, map_clone)?;
            Ok(())
        })?;
    }

    pool.shutdown(None)?;

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
