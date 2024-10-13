use clap::Parser;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Configuration {
    pub path: PathBuf,
    pub recursive: bool,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Arguments {
    #[arg(short = 'r', long)]
    pub recursive: Option<bool>,

    #[arg(short = 'p', long)]
    pub path: String,
}

pub mod config;
