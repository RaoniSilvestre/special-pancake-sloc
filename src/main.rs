use clap::{arg, Command};
use core::panic;
use search::print_info;
use search::search_tree;
use search::FileInfo;
use std::collections::HashMap;
mod search;
fn main() {
    let matches = Command::new("sloc-rust")
        .about("Pega informações de todos os arquivos na pasta escolhida.")
        .arg(arg!(<PATH>).help("Caminho para fazer busca!"))
        .arg(
            arg!(--recursive "Ativa busca recursiva!")
                .short('r')
                .required(false),
        )
        .get_matches();

    match matches.get_one::<String>("PATH") {
        Some(path) => {
            let mut informações: HashMap<String, FileInfo> = HashMap::new();
            search_tree(path, &mut informações);
            print_info(&informações);
        }
        None => {
            panic!("<PATH> necessário para rodar programa!")
        }
    }
}
