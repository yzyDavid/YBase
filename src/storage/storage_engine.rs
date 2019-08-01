use crate::error::Result;
use crate::nexus::value::Value;

pub trait StorageEngine {
    fn name(&self) -> String;

    fn insert(&self, v: Value) -> Result<()>;
}