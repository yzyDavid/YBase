extern crate log;

use ybase::{
    nexus::entry::YBaseEngine,
    storage::fixmap::engine::FixMapStorageEngine,
    error::Result,
};

use log::{info};

fn main() -> Result<()> {
    info!("starting YBase now...");
    let config_path = std::path::PathBuf::from("./config.json");
    let config = FixMapStorageEngine::parse_config(config_path)?;
    let storage_engine = Box::from(FixMapStorageEngine::new(config));
    let engine = YBaseEngine::new(storage_engine);
    info!("init Ok, running...");
    engine.run();
    Ok(())
}
