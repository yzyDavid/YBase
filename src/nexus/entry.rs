use crate::storage::storage_engine::StorageEngine;

pub struct YBaseEngine {
    pub engine: Box<dyn StorageEngine>
}

impl YBaseEngine {
    pub fn run(&self) {}
}