mod uuid;
mod util;

pub use util::*;
pub type CIResult<Value> = std::result::Result<Value, Box<dyn std::error::Error>>;
pub type UUID = uuid::UUID;

/// Common error for `Option::None` case where a value is required.
#[derive(Debug,Clone)]
pub struct MissingError;
impl std::error::Error for MissingError {}
impl std::fmt::Display for MissingError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "some value expected but missing.")
    }
}