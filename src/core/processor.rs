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
const DATA_SIZE: usize = 1_000_000_000;
const POOL_SIZE: usize = 1000;
const CHUNK_SIZE: usize = 1_000_000;

pub fn run(map: Arc<Mutex<HashMap<String, Temperature>>>) -> Result<(), Box<dyn Error>> {
    let pool = ThreadPool::new(POOL_SIZE);
    let file = File::open(FILENAME)?;
    let reader = BufReader::new(file);
    let mut chunk_lines = Vec::with_capacity(CHUNK_SIZE);
    let mut lines = reader.lines();

    while let Some(line) = lines.next() {
        let line = line?;
        chunk_lines.push(line);

        if chunk_lines.len() >= CHUNK_SIZE {
            let chunk = chunk_lines.clone();
            let map_clone = Arc::clone(&map);

            pool.execute(move || {
                process_chunk(chunk, map_clone)?;
                Ok(())
            })?;

            chunk_lines.clear();
        }
    }

    if !chunk_lines.is_empty() {
        let map_clone = Arc::clone(&map);
        pool.execute(move || {
            process_chunk(chunk_lines, map_clone)?;
            Ok(())
        })?;
    }

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
