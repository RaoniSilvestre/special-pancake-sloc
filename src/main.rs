use sloc_rust::{config::Configuration, display::Display, search::Searcher};

fn main() {
    let configuration = Configuration::configure();
    let mut seacher = Searcher::new(configuration);
    seacher.search();
    Display::print_info(seacher.result);
}
