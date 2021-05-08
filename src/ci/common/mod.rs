mod uuid;
mod util;

pub use util::*;
pub type CIResult<Value> = std::result::Result<Value, Box<dyn std::error::Error>>;
pub type UUID = uuid::UUID;


