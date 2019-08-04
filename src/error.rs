#[derive(Debug)]
pub struct RuntimeError(String);

impl RuntimeError {
    pub fn new(msg: &str) -> Self {
        RuntimeError { 0: String::from(msg) }
    }
}

impl std::error::Error for RuntimeError {}

impl std::fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.0.as_str())
    }
}

pub type Result<T> = std::result::Result<T, RuntimeError>;

impl From<std::io::Error> for RuntimeError {
    fn from(err: std::io::Error) -> Self {
        RuntimeError(err.to_string())
    }
}

impl From<serde_json::Error> for RuntimeError {
    fn from(err: serde_json::Error) -> Self {
        RuntimeError(err.to_string())
    }
}

impl From<std::ffi::NulError> for RuntimeError {
    fn from(err: std::ffi::NulError) -> Self {
        RuntimeError(err.to_string())
    }
}

impl From<std::ffi::OsString> for RuntimeError {
    fn from(os_string: std::ffi::OsString) -> Self {
        RuntimeError(String::from("convert to c string failed!"))
    }
}
