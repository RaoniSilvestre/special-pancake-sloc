use core::panic;
use std::ops::Add;
use std::{collections::HashMap, fs::*, io::Read};

#[derive(Debug, Clone, Copy)]
pub struct FileInfo {
    pub code: u32,
    pub whitespace: u32,
    pub comment: u32,
}

impl Add for FileInfo {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        FileInfo {
            code: self.code + other.code,
            whitespace: self.whitespace + other.whitespace,
            comment: self.comment + other.comment,
        }
    }
}

pub fn search_tree(path: &str, informações: &mut HashMap<String, FileInfo>) {
    if let Ok(entries) = read_dir(path) {
        for entry in entries {
            if let Ok(entry) = entry {
                if let Ok(metadata) = entry.metadata() {
                    let entry_path = entry.path();

                    let entry_path_str = entry_path
                        .to_str()
                        .expect("Não foi possível... num sei oq -> &str");

                    if metadata.is_file() {
                        match calculate_info(entry_path_str, informações) {
                            Ok(_) => (),
                            Err(_) => panic!("Erro grave no calculate_info ln 44"),
                        };
                    }
                    if metadata.is_dir()
                        && !entry_path_str.contains("/target")
                        && !entry_path_str.contains("/.git")
                        && !entry_path_str.contains("/node_modules")
                    {
                        search_tree(&entry_path_str, informações)
                    }
                } else {
                    println!("Deu ruim pra pegar entry");
                }
            }
        }
    }
}

fn calculate_info(
    path: &str, informações: &mut HashMap<String, FileInfo>
) -> std::io::Result<()> {
    let result_arquivo = File::open(path);

    let _ = if let Ok(mut arquivo) = result_arquivo {
        let mut conteudo = String::new();

        match arquivo.read_to_string(&mut conteudo) {
            Ok(_) => {
                let extensão = get_extension(path);

                let mut file_info: FileInfo = FileInfo {
                    code: 0,
                    whitespace: 0,
                    comment: 0,
                };

                for line in conteudo.lines() {
                    if line.trim().contains("#") || line.trim().contains("//") {
                        file_info.comment += 1;
                    } else if line.trim().is_empty() {
                        file_info.whitespace += 1;
                    } else {
                        file_info.code += 1;
                    }
                }

                if let Some(file_info_old) = informações.get_mut(extensão) {
                    *file_info_old = *file_info_old + file_info
                } else {
                    informações.insert(extensão.to_string(), file_info);
                }
            }
            Err(_) => println!("Erro no arquivo {}", path),
        };

        Ok(())
    } else {
        Err("Erro no arquivo!")
    };
    Ok(())
}

fn get_extension(path: &str) -> &str {
    let posição_do_ponto = path.rfind(".");

    match posição_do_ponto {
        Some(pos) => {println!("{&path[pos + 1..]}");

        return &path[pos + 1..]};
        None => "no_extension",
    }
}

#[test]
fn test_get_extension() {
    let x = get_extension("/wordle/LICENSE");
    assert_eq!("no_extension", x);
}

pub fn print_info(informações: &HashMap<String, FileInfo>) {
    let extensão = "extensão";
    let comments = "comentários";
    let code = "códigos";
    let whitespace = "espaços em branco";
    println!(
        "-------------------------------------------------------------------------------------------------------"
    );
    println!(
        "| {:<36} | {:<18} | {:<18} | {:<18} |",
        extensão, code, comments, whitespace
    );
    println!(
        "--------------------------------------------------------------------------------------------------------"
    );
    for (chave, valor) in informações {
        println!(
            "| {:<36} | {:<18} | {:<18} | {:<18} |",
            chave, valor.code, valor.comment, valor.whitespace
        )
    }
    println!(
        "--------------------------------------------------------------------------------------------------------"
    );
}
