use ybase::{
    nexus::entry::{YBaseEngine},
    storage::fixmap::engine::FixMapStorageEngine
};

fn main() {
    info!("starting YBase now...");
    let engine = YBaseEngine{engine: Box::new(FixMapStorageEngine::new(std::path::PathBuf::from(""))};
    engine.run();
}
