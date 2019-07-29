use ybase::nexus::entry::YBaseEngine;
use ybase::storage::fixmap::engine::FixMapStorageEngine;

fn main() {
    println!("Hello, rust!");
    let engine = YBaseEngine{engine: Box::new(FixMapStorageEngine::new(4 * 1024 * 1024))};
    engine.run();
}
