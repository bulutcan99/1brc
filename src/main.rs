use std::{collections::HashMap, fs::File, io::BufReader};

use anyhow::Error;

pub mod core;

fn main() -> Result<(), Error> {
    let file = File::open("measurements.txt")?;
    let reader = BufReader::new(file);
    let map = HashMap::new();
    Ok(())
}
