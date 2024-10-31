use fxhash::FxHashMap;
use rayon::prelude::*;
use std::error::Error;
use std::fs::File;
use std::io::Read;

use crate::core::helper;
use crate::core::temperature::{Temperature, Value32};

const FILENAME: &str = "measurements.txt";

pub fn run() -> Result<(), Box<dyn Error>> {
    let filename = FILENAME.to_string();
    let stat = std::fs::metadata(&filename).unwrap();
    let mut data = Vec::with_capacity(stat.len() as usize + 1);
    let mut file = File::open(&filename).map_err(|e| format!("Error opening file: {}", e))?;
    file.read_to_end(&mut data)
        .map_err(|e| format!("Error reading file: {}", e))?;

    if data.last() != Some(&b'\n') {
        return Err("File must end with a newline character".into());
    }

    let mut h = data
        .par_split(|&c| c == b'\n')
        .filter(|line| !line.is_empty())
        .map(|line| {
            let (name, value) = line.split_once(|&c| c == b';').unwrap();
            let mut local_map = FxHashMap::default();
            let key = helper::generate_hash_key(name);
            local_map
                .entry(key)
                .or_insert_with(|| (Temperature::default(), name))
                .0
                .add(Value32::parse(value));
            local_map
        })
        .reduce(FxHashMap::default, |mut acc, local_map| {
            for (key, (temp, name)) in local_map {
                acc.entry(key)
                    .or_insert_with(|| (Temperature::default(), name))
                    .0
                    .merge(&temp)
            }
            acc
        });

    let mut v: Vec<_> = h.into_iter().collect();
    v.sort_unstable_by_key(|p| p.0);

    v.par_iter().for_each(|(_key, (r, name))| {
        println!(
            "{}: {}/{}/{}",
            std::str::from_utf8(name).unwrap(),
            Value32::format(&r.min),
            Value32::format(&r.average()),
            Value32::format(&r.max)
        );
    });

    eprintln!("Num records: {}", v.len());

    Ok(())
}
