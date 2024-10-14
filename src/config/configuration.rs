use std::path::PathBuf;

use super::{Arguments, Configuration};
use clap::Parser;

impl Configuration {
    pub fn configure() -> Configuration {
        let cli = Arguments::parse();
        Configuration::new(PathBuf::from(&cli.path), cli.recursive)
    }

    pub fn new(path: PathBuf, recursive: bool) -> Configuration {
        Configuration { path, recursive }
    }
}
