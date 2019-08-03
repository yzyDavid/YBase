use crate::storage::storage_engine::StorageEngine;
use crate::error::{Result, RuntimeError};

extern crate serde;
extern crate uuid;
extern crate nix;

use uuid::Uuid;


use serde::{Serialize, Deserialize};
use serde_json;
use log::info;
use std::path;
use crate::nexus::value::Value;
use std::collections::HashMap;
use std::thread::current;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
struct FieldDef {
    name: String,
    value_type: String,
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

struct RuntimeContext {}

pub struct FixMapStorageEngine {
    meta_path: PathBuf,
    meta: Meta,
    context: RuntimeContext,
}

impl FixMapStorageEngine {
    pub fn new(meta_path: PathBuf, meta: Meta) -> Self {
        FixMapStorageEngine { meta_path, meta, context: RuntimeContext {} }
    }

    fn alloc_page(&mut self) -> Result<()> {
        let page_name = Uuid::new_v4().to_hyphenated().to_string() + ".page";
        info!("allocating page with name: [{}]", &page_name);
        self.meta.pages.push(page_name.clone());
        let mut page_path = self.meta.root_dir.clone();
        page_path.push(page_name);
        self.mmap_file(&page_path);
        Ok(())
    }

    fn mmap_file(&mut self, file: &PathBuf) {
        info!("mmap file: [{:?}]", &file);
    }

    pub fn parse_meta(meta_file: path::PathBuf) -> Result<Meta> {
        let content = std::fs::read_to_string(meta_file)?;
        let meta: Meta = serde_json::from_str(&content)?;
        // TODO: check fields legal

        Ok(meta)
    }

    fn sync_meta(&self) {
        serde_json::to_string(&self.meta);
    }
}

impl StorageEngine for FixMapStorageEngine {
    fn name(&self) -> String {
        String::from("FixMapStorageEngine")
    }

    fn init(&mut self) {
        info!("start init");
        for page in self.meta.pages.clone().iter() {
            let mut page_path = self.meta.root_dir.clone();
            page_path.push(page);
            self.mmap_file(&page_path);
        }
    }

    fn insert(&self, _v: Value) -> Result<()> {
        unimplemented!()
    }
}
