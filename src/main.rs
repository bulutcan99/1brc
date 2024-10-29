use core::temperature::Temperature;
use std::{
    collections::HashMap,
    error::Error,
    sync::{Arc, Mutex},
};
pub mod core;

fn main() -> Result<(), Box<dyn Error>> {
    let map: Arc<Mutex<HashMap<String, Temperature>>> = Arc::new(Mutex::new(HashMap::new()));
    core::processor::run(map)?;
    Ok(())
}
