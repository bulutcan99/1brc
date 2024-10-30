use crate::core::temperature::Value32;

use super::temperature::Temperature;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::Read;

const FILENAME: &str = "measurements.txt";

pub fn run() -> Result<(), Box<dyn Error>> {
    let filename = FILENAME.to_string();
    let mut data = Vec::new();

    let mut file = File::open(&filename).map_err(|e| format!("Error opening file: {}", e))?;
    file.read_to_end(&mut data)
        .map_err(|e| format!("Error reading file: {}", e))?;

    if data.last() != Some(&b'\n') {
        return Err("File must end with a newline character".into());
    }

    let mut h: HashMap<&[u8], Temperature> = HashMap::new();

    for line in data.split(|&c| c == b'\n') {
        if line.is_empty() {
            continue;
        }

        let (name, value) = match line.split_once(|&c| c == b';') {
            Some(pair) => pair,
            None => {
                eprintln!(
                    "Invalid line format: {:?}",
                    std::str::from_utf8(line).unwrap()
                );
                continue;
            }
        };

        h.entry(name)
            .or_insert_with(Temperature::default)
            .add(Value32::parse(value));
    }
    let mut v: Vec<_> = h.into_iter().collect();
    v.sort_unstable_by_key(|p| p.0);

    for (name, r) in &v {
        println!(
            "{}: {}/{}/{}",
            std::str::from_utf8(name).unwrap(),
            Value32::format(&r.min),
            Value32::format(&r.average()),
            Value32::format(&r.max)
        );
    }

    eprintln!("Num records: {}", v.len());

    Ok(())
}
