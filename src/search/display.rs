use crate::FileInfo;
use std::collections::HashMap;
fn ln() {
    println!(
        "--------------------------------------------------------------------------------------------------------"
    );
}

fn update_vec(vetor: &mut Vec<(String, f32)>, new_entry: (String, f32)) {
    if vetor.len() < 3 {
        vetor.push(new_entry)
    } else {
        for (index, elem) in vetor.iter().enumerate() {
            if elem.1 < new_entry.1 {
                vetor.remove(index);
                vetor.push(new_entry);
                break;
            }
        }
    }
}

pub fn print_info(informações: &HashMap<String, FileInfo>) {
    let extensão = "extensão";
    let comments = "comentários";
    let code = "códigos";
    let whitespace = "espaços em branco";
    let mut total_file_info = FileInfo::new();
    let mut most_used_files: Vec<(String, f32)> = Vec::new();

    ln();

    println!(
        "| {:<36} | {:<18} | {:<18} | {:<18} |",
        extensão, code, comments, whitespace
    );

    ln();

    for (chave, valor) in informações {
        total_file_info = total_file_info + *valor;

        update_vec(&mut most_used_files, (chave.to_string(), valor.code as f32));

        println!(
            "| {:<36} | {:<18} | {:<18} | {:<18} |",
            chave, valor.code, valor.comment, valor.whitespace
        )
    }

    println!(
        "| {:<36} | {:<18} | {:<18} | {:<18} |",
        "TOTAL", total_file_info.code, total_file_info.comment, total_file_info.whitespace
    );

    ln();
    most_used_files = my_cool_sort(most_used_files);

    println!("\n\n");
    ln();
    println!(
        "| {:<36} | {:<18} | {:<18} | {:<18} |",
        "MOST USED EXT", most_used_files[2].0, most_used_files[1].0, most_used_files[0].0
    );

    let most_used_files_percentage: Vec<f32> = {
        most_used_files
            .iter()
            .map(|a| a.1 / total_file_info.code as f32)
            .collect()
    };

    ln();
    println!(
        "| {:<36} | {:17.3}% | {:>17.3}% | {:>17.3}% |",
        "PORCENTAGEM",
        most_used_files_percentage[2],
        most_used_files_percentage[1],
        most_used_files_percentage[0]
    );

    ln();
}

fn my_cool_sort(mut vetor: Vec<(String, f32)>) -> Vec<(String, f32)> {
    vetor.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    vetor
}
