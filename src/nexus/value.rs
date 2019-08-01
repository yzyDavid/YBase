#[derive(Debug)]
pub enum Value {
    Int(i64),
    Float(f64)
}

// primary key, record
pub struct Record(u64, Vec<Value>);
