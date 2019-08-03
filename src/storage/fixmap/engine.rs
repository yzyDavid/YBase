use crate::storage::storage_engine::StorageEngine;
use crate::error::{Result, RuntimeError};

extern crate serde;
extern crate uuid;
extern crate nix;

use uuid::Uuid;


use serde::{Serialize, Deserialize};
use serde_json;

use std::path;
use crate::nexus::value::Value;
use std::collections::HashMap;
use std::thread::current;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
struct FieldDef {
    name: String,
    value_type: String
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Schema {
    fields: Vec<FieldDef>
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Meta {
    table_name: String,
    schema: Schema,
    page_size: u64,
    root_dir: path::PathBuf,
    pages: Vec<String>,
}

pub struct FixMapStorageEngine {
    meta: Meta,
}

impl FixMapStorageEngine {
    pub fn new(meta: Meta) -> Self {
        FixMapStorageEngine { meta }
    }

    fn init(&mut self) {}

    fn alloc_page(&mut self) -> Result<()> {
        let page_name = Uuid::new_v4().to_hyphenated().to_string() + ".page";
        self.meta.pages.push(page_name.clone());
        let mut page_path = self.meta.root_dir.clone();
        page_path.push(page_name);
        Ok(())
    }

    fn mmap_file(&mut self, file: &PathBuf) {}

    pub fn parse_meta(meta_file: path::PathBuf) -> Result<Meta> {
        let content = std::fs::read_to_string(meta_file)?;
        let meta: Meta = serde_json::from_str(&content)?;
        // TODO: check fields legal

        Ok(meta)
    }
}

impl StorageEngine for FixMapStorageEngine {
    fn name(&self) -> String {
        String::from("FixMapStorageEngine")
    }

    fn insert(&self, _v: Value) -> Result<()> {
        unimplemented!()
    }
}
