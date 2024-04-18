use std::fs::File;
use std::io::Read;

use std::collections::HashMap;

use crate::FileInfo;

pub fn calculate_file_info(path: &str, informações: &mut HashMap<String, FileInfo>) {
    let arquivo_result = File::open(path);

    match arquivo_result {
        Ok(mut arquivo) => read_arquivo(&mut arquivo, path, informações),
        Err(e) => eprintln!("Erro! ln 13 calculate.rs {}\n {}", e, path),
    }
}

fn read_arquivo(arquivo: &mut File, path: &str, informações: &mut HashMap<String, FileInfo>) {
    let mut buffer = String::new();

    let file_to_string_try = arquivo.read_to_string(&mut buffer);

    match file_to_string_try {
        Ok(_) => {
            let file_info_new = iterate_over_lines(buffer);

            update_hash_info(file_info_new, informações, path)
        }
        Err(_e) => (), // eprintln!("\nErro ao converter para string!\nNo arquivo {}\nMotivo: {}\n",path, e) ,
    }
}

fn iterate_over_lines(conteudo: String) -> FileInfo {
    let mut file_info = FileInfo::new();

    for line in conteudo.lines() {
        if line.trim().contains("#") || line.trim().contains("//") {
            file_info.comment += 1;
        } else if line.trim().is_empty() {
            file_info.whitespace += 1;
        } else {
            file_info.code += 1;
        };
    }

    file_info
}

fn update_hash_info(
    file_info_new: FileInfo,
    informações: &mut HashMap<String, FileInfo>,
    path: &str,
) {
    let extensão = get_extension(path);

    let key_from_informações_result = informações.get_mut(extensão);

    match key_from_informações_result {
        Some(file_info_old) => *file_info_old = *file_info_old + file_info_new,
        None => {
            informações.insert(extensão.to_string(), file_info_new);
        }
    }
}

fn get_extension(path: &str) -> &str {
    let mut extension = path;
    let posição_da_barra_opt = path.rfind("/");

    extension = match posição_da_barra_opt {
        Some(posição_da_barra) => &extension[posição_da_barra..],
        None => &extension,
    };

    let posição_do_ponto = extension.rfind(".");

    match posição_do_ponto {
        Some(0) => {
            return "no_extension";
        }
        Some(pos) if pos < 36 => {
            return &extension[pos..];
        }
        _ => return "no_extension",
    }
}

#[test]
fn test_get_extension() {
    let x = get_extension("/wordle/LICENSE");
    assert_eq!("no_extension", x);
}
