use clap::{arg, ArgMatches, Command};

pub fn get_opts() -> ArgMatches {
    Command::new("sloc-rust")
        .about("Pega informações de todos os arquivos na pasta escolhida.")
        .arg(arg!(<PATH>).help("Caminho para fazer busca!"))
        .arg(
            arg!(--recursive "Ativa busca recursiva!")
                .short('r')
                .required(false),
        )
        .get_matches()
}
