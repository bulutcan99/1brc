use super::temperature::Temperature;
use async_std::{
    fs::File,
    io::{prelude::BufReadExt, BufReader},
    stream::StreamExt,
    sync::Mutex,
};
use std::{collections::HashMap, error::Error, sync::Arc};

const FILENAME: &str = "measurements.txt";
const CHUNK_SIZE: usize = 1000;

async fn process_chunk(chunk: Vec<String>, map: Arc<Mutex<HashMap<String, Temperature>>>) {}

pub async fn run(map: Arc<Mutex<HashMap<String, Temperature>>>) -> Result<(), Box<dyn Error>> {
    let file = File::open(FILENAME).await?;
    let reader = BufReader::new(file);
    let mut chunk_lines = Vec::with_capacity(CHUNK_SIZE);
    let mut lines = reader.lines();

    while let Some(line) = lines.next().await {
        let line = line?;
        chunk_lines.push(line);

        if chunk_lines.len() >= CHUNK_SIZE {
            process_chunk(chunk_lines.clone(), map.clone()).await;
            chunk_lines.clear();
        }
    }

    if !chunk_lines.is_empty() {
        process_chunk(chunk_lines, map).await;
    }

    Ok(())
}
