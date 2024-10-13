use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use super::Processor;
use crate::infra::FileInfo;

impl Processor {
    pub fn process(file: File) -> FileInfo {
        let buffer = BufReader::new(file);
        Self::iterate_lines(buffer)
    }

    fn iterate_lines(buffer: BufReader<File>) -> FileInfo {
        let mut file_info = FileInfo::default();

        for buffer_result in buffer.lines() {
            match buffer_result {
                Ok(line) => file_info += Self::process_line(line),
                Err(error) => eprintln!("Erro na iterate_lines: {error}"),
            }
        }

        file_info
    }

    fn process_line(line: String) -> FileInfo {
        let mut file_info = FileInfo::default();
        let trimmed_line = line.trim();

        if trimmed_line.starts_with("#") || trimmed_line.starts_with("//") {
            file_info.comment += 1;
        } else if trimmed_line.is_empty() {
            file_info.whitespace += 1;
        } else {
            file_info.code += 1;
        }

        file_info
    }
}
