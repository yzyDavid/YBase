#[derive(Debug)]
pub enum Value {
    Int(i64),
    Float(f64)
}

// primary key, record
pub struct Record(u64, Vec<Value>);

pub const VALID_TYPE: [&str; 2] = ["Int", "Float"];
