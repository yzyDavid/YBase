extern crate log;
extern crate env_logger;

use ybase::{
    nexus::entry::YBaseEngine,
    storage::fixmap::engine::FixMapStorageEngine,
    error::Result,
};

use log::{info};

fn main() -> Result<()> {
    env_logger::from_env(env_logger::Env::default().default_filter_or("info")).init();
    info!("starting YBase now...");
    let config_path = std::path::PathBuf::from("./config.json");
    let config = FixMapStorageEngine::parse_config(config_path)?;
    let storage_engine = Box::from(FixMapStorageEngine::new(config));
    let engine = YBaseEngine::new(storage_engine);
    info!("init Ok, running...");
    engine.run();
    Ok(())
}
