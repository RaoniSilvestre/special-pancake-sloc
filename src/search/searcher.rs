use crate::config::Configuration;
use crate::infra::FileInfo;
use crate::process::Processor;

use super::Searcher;

use std::collections::BTreeMap;
use std::fs::read_dir;
use std::fs::DirEntry;
use std::fs::File;
use std::fs::Metadata;
use std::fs::ReadDir;
use std::io::Error;
use std::io::Read;
use std::path::PathBuf;

impl Searcher {
    pub fn new(configuration: Configuration) -> Searcher {
        let ignored_directories = vec!["/target", "/.git", "/node_modules"]
            .into_iter()
            .map(String::from)
            .collect();

        Searcher {
            root_path: configuration.path,
            is_recursive: configuration.recursive,
            ignored_directories,
            result: BTreeMap::new(),
        }
    }

    pub fn search(&mut self) {
        match read_dir(&self.root_path) {
            Ok(directory_iterator) => self.iterate(directory_iterator),
            Err(error) => eprintln!("{error}"),
        }
    }

    fn search_dir(&mut self, directory: PathBuf) {
        match read_dir(directory) {
            Ok(directory_iterator) => self.iterate(directory_iterator),
            Err(error) => eprintln!("{error}"),
        }
    }

    fn iterate(&mut self, directory_iterator: ReadDir) {
        for iterator_item in directory_iterator {
            self.process_item(iterator_item)
        }
    }

    fn process_item(&mut self, iterator_item: Result<DirEntry, Error>) {
        match iterator_item {
            Ok(file) => self.process_file(file),
            Err(ref iterator_error) => {
                eprintln!("Erro: {iterator_error}\nNo arquivo: {iterator_item:?}")
            }
        }
    }

    fn process_file(&mut self, dir_entry: DirEntry) {
        let metadata = Self::get_metadata(&dir_entry);

        if metadata.is_file() && Self::is_readable(dir_entry.path()) {
            let processed_file_info = match File::open(dir_entry.path()) {
                Ok(file) => Processor::process(file),
                Err(_) => FileInfo::default(),
            };

            let extension = dir_entry
                .path()
                .extension()
                .map(|ext| ext.to_string_lossy().into_owned())
                .unwrap_or_default();

            self.update_hasmap(extension, processed_file_info);
        }

        if metadata.is_dir()
            && self.is_recursive
            && !self.is_forbidden_directories(dir_entry.path())
        {
            self.search_dir(dir_entry.path());
        }
    }

    fn update_hasmap(&mut self, extension: String, file_info: FileInfo) {
        let key_from_informações_result = self.result.get_mut(&extension);

        match key_from_informações_result {
            Some(file_info_old) => *file_info_old += file_info,
            None => {
                self.result.insert(extension, file_info);
            }
        }
    }

    fn get_metadata(file: &DirEntry) -> Metadata {
        match file.metadata() {
            Ok(metadata) => metadata,
            Err(metadata_error) => panic!("Erro ao pegar metadata: {metadata_error}"),
        }
    }

    fn is_readable(file_path: PathBuf) -> bool {
        File::open(file_path)
            .and_then(|mut file| {
                let mut buffer = String::new();
                file.read_to_string(&mut buffer)
            })
            .is_ok()
    }

    fn is_forbidden_directories(&self, entry_path: PathBuf) -> bool {
        if let Some(path_str) = entry_path.to_str() {
            for element in &self.ignored_directories {
                if path_str.contains(element) {
                    return true;
                }
            }
        }
        false
    }
}
