use std::{collections::BTreeMap, fmt::Display};

use super::Tabela;
use crate::infra::FileInfo;

impl Tabela {
    fn ln() {
        println!(
        "--------------------------------------------------------------------------------------------------------"
    );
    }

    pub fn print_info<T: Display>(informações: BTreeMap<T, FileInfo>) {
        let extensão = "extensão";
        let comments = "comentários";
        let code = "códigos";
        let whitespace = "espaços em branco";

        let mut total_file_info = FileInfo::default();

        Self::ln();

        println!(
            "| {:<36} | {:<18} | {:<18} | {:<18} |",
            extensão, code, comments, whitespace
        );

        Self::ln();

        for (chave, valor) in informações {
            total_file_info += valor;

            println!(
                "| {:<36} | {:<18?} | {:<18?} | {:<18?} |",
                chave, valor.code, valor.comment, valor.whitespace
            )
        }

        Self::ln();

        println!(
            "| {:<36} | {:<18} | {:<18} | {:<18} |",
            "TOTAL", total_file_info.code, total_file_info.comment, total_file_info.whitespace
        );

        Self::ln();
    }
}
