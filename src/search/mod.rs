use crate::FileInfo;
use core::panic;
use std::{collections::HashMap, fs::*, io::Read};
pub mod display;
pub mod search;
pub mod structs;

fn calculate_info(
    path: &str, informações: &mut HashMap<String, FileInfo>
) -> std::io::Result<()> {
    let result_arquivo = File::open(path);

    let _ = if let Ok(mut arquivo) = result_arquivo {
        let mut conteudo = String::new();

        match arquivo.read_to_string(&mut conteudo) {
            Ok(_) => {
                let extensão = get_extension(path);
                println!("{},{extensão}", path.rfind(".").unwrap());
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
            Err(e) => println!("Erro no arquivo {}\nDescrição do erro: {}", path, e),
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
        Some(0) => {
            return "no_extension";
        }
        Some(pos) => {
            return &path[pos + 0..];
        }
        None => {
            panic!("None no rfind na função get_extension");
        }
    }
}

#[test]
fn test_get_extension() {
    let x = get_extension("/wordle/LICENSE");
    assert_eq!("no_extension", x);
}
