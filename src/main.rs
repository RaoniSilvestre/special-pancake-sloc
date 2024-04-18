use search::display::print_info;
use search::search::search_tree;
use search::structs::FileInfo;

use std::collections::HashMap;

mod cli;
mod search;

fn main() {
    let matches = cli::get_opts();

    match matches.get_one::<String>("PATH") {
        Some(path) => {
            let mut informações: HashMap<String, FileInfo> = HashMap::new();

            search_tree(path, &mut informações, matches.get_flag("recursive"));
            print_info(&informações);
        }
        None => {
            panic!("<PATH> necessário para rodar programa!")
        }
    }
}
