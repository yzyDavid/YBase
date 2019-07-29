use crate::storage::storage_engine::StorageEngine;

pub struct FixMapStorageEngine {
    page_size: u64
}

impl FixMapStorageEngine {
    pub fn new(page_size: u64) -> FixMapStorageEngine {
        FixMapStorageEngine {page_size}
    }

    fn alloc_page() {

    }
}

impl StorageEngine for FixMapStorageEngine {
    fn name(&self) -> String {
        String::from("FixmapStorageEngine")
    }
}
