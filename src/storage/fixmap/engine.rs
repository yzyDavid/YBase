use crate::storage::storage_engine::StorageEngine;
use crate::error::Result;

extern crate serde;

use serde::{Serialize, Deserialize};
use serde_json;

use std::path;
use std::error::Error;

#[derive(Serialize, Deserialize, Debug)]
struct FixMapConfig {
    page_size: u64,
    root_dir: path::PathBuf,
}

pub struct FixMapStorageEngine {
    config: FixMapConfig,
}

impl FixMapStorageEngine {
    pub fn new(config_path: path::PathBuf) -> Result<Self> {
        let config = Self::parse_config(config_path)?;
        Ok(FixMapStorageEngine { config })
    }

    fn alloc_page(&self) {}

    fn parse_config(config_path: path::PathBuf) -> Result<FixMapConfig> {
        let content = std::fs::read_to_string(config_path)?;
        let config = serde_json::from_str(content.as_str())?;
    }
}

impl StorageEngine for FixMapStorageEngine {
    fn name(&self) -> String {
        String::from("FixMapStorageEngine")
    }
}
