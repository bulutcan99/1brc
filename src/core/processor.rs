use super::temperature::Temperature;
use rayon::prelude::*;
use std::{
    collections::HashMap,
    error::Error,
    sync::{Arc, Mutex},
};
use tokio::fs::File as TokioFile;
use tokio::io::{AsyncBufReadExt, BufReader as TokioBufReader};

const FILENAME: &str = "measurements.txt";
const CHUNK_SIZE: usize = 1_000_000;

pub async fn run(map: Arc<Mutex<HashMap<String, Temperature>>>) -> Result<(), Box<dyn Error>> {
    let file = TokioFile::open(FILENAME).await?;
    let reader = TokioBufReader::new(file);
    let mut lines = reader.lines();

    let mut chunk_lines = Vec::with_capacity(CHUNK_SIZE);

    while let Some(line) = lines.next_line().await? {
        chunk_lines.push(line);

        if chunk_lines.len() >= CHUNK_SIZE {
            let chunk = chunk_lines.clone();
            let map_clone = Arc::clone(&map);
            tokio::task::spawn(async move {
                if let Err(e) = process_chunk(chunk, map_clone).await {
                    eprintln!("Error processing chunk: {:?}", e);
                }
            });
            chunk_lines.clear();
        }
    }

    if !chunk_lines.is_empty() {
        let map_clone = Arc::clone(&map);
        tokio::task::spawn(async move {
            if let Err(e) = process_chunk(chunk_lines, map_clone).await {
                eprintln!("Error processing remaining lines: {:?}", e);
            }
        });
    }

    println!("Finished processing all lines");
    Ok(())
}

async fn process_chunk(
    chunk: Vec<String>,
    map: Arc<Mutex<HashMap<String, Temperature>>>,
) -> Result<(), Box<dyn std::error::Error>> {
    chunk.par_iter().for_each(|data| {
        let parts: Vec<&str> = data.split(';').collect();
        if parts.len() < 2 {
            eprintln!("Invalid data format: {}", data);
            return; // Geçersiz veri durumunda döngüyü sonlandır
        }

        let key = parts[0].to_string();
        let temp: f64 = parts[1]
            .parse()
            .map_err(|e| {
                eprintln!("Failed to parse temperature for {}: {}", key, e);
                e
            })
            .unwrap_or(0.0);

        let mut temp_map = map.lock().unwrap();
        if let Some(temperature) = temp_map.get_mut(&key) {
            temperature.update(temp);
        } else {
            temp_map.insert(key.clone(), Temperature::new(temp));
        }
    });

    Ok(())
}
