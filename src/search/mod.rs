use crate::infra::FileInfo;
use std::{collections::BTreeMap, path::PathBuf};

mod searcher;

#[derive(Debug)]
pub struct Searcher {
    root_path: PathBuf,
    ignored_directories: Vec<String>,
    is_recursive: bool,
    pub result: BTreeMap<String, FileInfo>,
}
