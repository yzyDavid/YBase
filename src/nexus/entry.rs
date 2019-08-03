use crate::storage::storage_engine::StorageEngine;

pub struct YBaseEngine {
    pub storage_engine: Box<dyn StorageEngine>
}

impl YBaseEngine {
    pub fn new(storage_engine: Box<dyn StorageEngine>) -> Self {
        Self { storage_engine }
    }
    pub fn run(&mut self) {
        self.storage_engine.init();
    }
}