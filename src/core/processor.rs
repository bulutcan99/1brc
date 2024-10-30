use super::helper;
use super::temperature::Temperature;
use crate::core::temperature::Value32;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::str;

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

    let mut h: HashMap<u64, (Temperature, &[u8])> = HashMap::new();

    for line in data.split(|&c| c == b'\n') {
        if line.is_empty() {
            continue;
        }

        let (name, value) = line.split_once(|&c| c == b';').unwrap();

        h.entry(helper::generate_hash_key(name))
            .or_insert((Temperature::default(), name))
            .0
            .add(Value32::parse(value));
    }

    let mut v: Vec<_> = h.into_iter().collect();
    v.sort_unstable_by_key(|p| p.0);

    for (_key, (r, name)) in &v {
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
