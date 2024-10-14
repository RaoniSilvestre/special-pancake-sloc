use sloc_rust::{config::Configuration, display::Tabela, search::Searcher};

fn main() {
    let configuration = Configuration::configure();
    let mut seacher = Searcher::new(configuration);
    seacher.search();
    Tabela::print_info(seacher.result);
}
