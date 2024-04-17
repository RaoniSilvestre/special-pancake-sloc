use crate::search::calculate_info;
use crate::search::read_dir;
use crate::FileInfo;
use std::collections::HashMap;
use std::fs::DirEntry;
use std::fs::Metadata;
use std::fs::ReadDir;

const LOCKED_DIRS: [&str; 3] = ["/target", "/.git", "/node_modules"];

pub fn search_tree(path: &str, informações: &mut HashMap<String, FileInfo>) {
    let entries_result = read_dir(path);

    match entries_result {
        Ok(entries) => iterate_over_dir(entries, informações),
        Err(e) => eprintln!("Erro! {}", e),
    }
}

fn iterate_over_dir(entries: ReadDir, informações: &mut HashMap<String, FileInfo>) {
    for entry_result in entries {
        match entry_result {
            Ok(entry) => get_metadata(entry, informações),
            Err(e) => eprintln!("Erro! {}", e),
        }
    }
}

fn get_metadata(entry: DirEntry, informações: &mut HashMap<String, FileInfo>) {
    let metadata_result = entry.metadata();

    match metadata_result {
        Ok(metadata) => calculate_file_lines(entry, metadata, informações),
        Err(e) => eprintln!("Erro! {}", e),
    }
}

fn calculate_file_lines(
    entry: DirEntry,
    metadata: Metadata,
    informações: &mut HashMap<String, FileInfo>,
) {
    let entry_path = entry.path();

    let entry_path_str = entry_path
        .to_str()
        .expect("Não foi possível converter entry_path : PathBuf -> entry_path_str &str");

    if metadata.is_file() {
        match calculate_info(entry_path_str, informações) {
            Ok(_) => (),
            Err(_) => panic!("Erro grave no calculate_file_lines ln 52"),
        };
    }
    if metadata.is_dir() && !is_forbidden_directories(entry_path_str) {
        search_tree(&entry_path_str, informações)
    }
}

fn is_forbidden_directories(entry_path_str: &str) -> bool {
    for element in LOCKED_DIRS {
        if entry_path_str.contains(element) {
            return true;
        }
    }
    false
}
