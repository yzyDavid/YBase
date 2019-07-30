use crate::storage::storage_engine::StorageEngine;
use crate::error::{Result, RuntimeError};

extern crate serde;

use serde::{Serialize, Deserialize};
use serde_json;

use std::path;

#[derive(Serialize, Deserialize, Debug)]
pub struct FixMapConfig {
    page_size: u64,
    root_dir: path::PathBuf,
}

pub struct FixMapStorageEngine {
    config: FixMapConfig,
}

impl FixMapStorageEngine {
    pub fn new(config: FixMapConfig) -> Self {
        FixMapStorageEngine { config }
    }

    fn alloc_page(&self) {}

    pub fn parse_config(config_path: path::PathBuf) -> Result<FixMapConfig> {
        let content = std::fs::read_to_string(config_path)?;
        let config: FixMapConfig = serde_json::from_str(&content)?;
        Ok(config)
    }
}

impl StorageEngine for FixMapStorageEngine {
    fn name(&self) -> String {
        String::from("FixMapStorageEngine")
    }
}
