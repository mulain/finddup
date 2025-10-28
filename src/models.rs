use serde::Serialize;
use std::path::PathBuf;

#[derive(Debug, Serialize)]
pub struct FileInfo {
    pub path: PathBuf,
    pub size: u64,
    pub hash: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct DuplicateGroup {
    pub hash: String,
    pub files: Vec<PathBuf>,
}
