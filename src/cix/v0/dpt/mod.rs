use serde::{Serialize, Deserialize};
use super::common::*;

#[derive(Serialize, Deserialize)]
#[derive(Debug, Clone)]
pub enum Command {
    Post(ID, ArtifactKind, FileMessage),
    Fetch(ID, ArtifactKind),
}

#[derive(Serialize, Deserialize)]
#[derive(Debug, Clone)]
pub enum Report {
    Post(ID, Result<(),String>),
    Fetch(ID, ArtifactKind, FileMessage),
    Error(Error),
}

#[derive(Serialize, Deserialize)]
#[derive(Debug, Clone)]
pub enum Error {
    /// Storage is full or missing.
    StorageUnavailable,
}

