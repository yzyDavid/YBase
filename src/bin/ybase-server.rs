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
    let meta_path = std::path::PathBuf::from("./meta.json");
    let meta = FixMapStorageEngine::parse_meta(meta_path)?;
    let storage_engine = Box::from(FixMapStorageEngine::new(meta));
    let engine = YBaseEngine::new(storage_engine);
    info!("init Ok, running...");
    engine.run();
    Ok(())
}
