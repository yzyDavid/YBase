use crate::storage::storage_engine::StorageEngine;
use crate::error::{Result, RuntimeError};

extern crate serde;
extern crate uuid;
extern crate nix;

use uuid::Uuid;
use nix::libc;

use serde::{Serialize, Deserialize};
use serde_json;

use std::path;
use crate::nexus::value::Value;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
pub struct FixMapConfig {
    page_size: u64,
    root_dir: path::PathBuf,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
struct Schema {
    fields: Vec<String>
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
struct Meta {
    table_name: String,
    pages: Vec<String>,
}

pub struct FixMapStorageEngine {
    config: FixMapConfig,
}

impl FixMapStorageEngine {
    pub fn new(config: FixMapConfig) -> Self {
        FixMapStorageEngine { config }
    }

    fn alloc_page(&self) {
        let page_name = Uuid::new_v4().to_hyphenated().to_string() + ".page";
        let mut page_path = self.config.root_dir.clone();
        page_path.push(page_name);
    }

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

    fn insert(&self, v: Value) -> Result<()> {
        unimplemented!()
    }
}
