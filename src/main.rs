use async_std::{main, sync::Mutex};
use core::temperature::Temperature;
use std::{collections::HashMap, error::Error, sync::Arc};
pub mod core;

#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let map: Arc<Mutex<HashMap<String, Temperature>>> = Arc::new(Mutex::new(HashMap::new()));
    let start_time = std::time::Instant::now();

    core::processor::run(map).await?;

    println!("Time spent: {:?}", start_time.elapsed());
    Ok(())
}
