use crate::search::calculate::calculate_file_info;

use crate::FileInfo;

use std::collections::HashMap;
use std::fs::read_dir;
use std::fs::DirEntry;
use std::fs::File;
use std::fs::Metadata;
use std::fs::ReadDir;
use std::io::Read;

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
            Ok(entry) => {
                get_metadata(entry, informações);
            }
            Err(e) => eprintln!("Erro! {}", e),
        }
    }
}

fn get_metadata(entry: DirEntry, informações: &mut HashMap<String, FileInfo>) {
    let metadata_result = entry.metadata();

    match metadata_result {
        Ok(metadata) => call_calculate_file_info(entry, metadata, informações),
        Err(e) => eprintln!("Erro! {}", e),
    }
}

fn call_calculate_file_info(
    entry: DirEntry,
    metadata: Metadata,
    informações: &mut HashMap<String, FileInfo>,
) {
    let entry_path = entry.path();

    let entry_path_str = entry_path
        .to_str()
        .expect("Não foi possível converter entry_path : PathBuf -> entry_path_str &str");

    if metadata.is_file() && is_readable(entry_path_str) {
        calculate_file_info(entry_path_str, informações)
    }
    if metadata.is_dir() && !is_forbidden_directories(entry_path_str) {
        search_tree(&entry_path_str, informações)
    }
}

fn is_readable(file_path: &str) -> bool {
    let file = File::open(file_path);
    let mut buffer = String::new();
    match file {
        Ok(mut file) => {
            let wasread = file.read_to_string(&mut buffer);
            match wasread {
                Ok(_) => true,
                Err(_) => false,
            }
        }
        Err(_) => false,
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
