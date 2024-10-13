use std::collections::BTreeMap;

use super::Display;
use crate::infra::FileInfo;

impl Display {
    fn ln() {
        println!(
        "--------------------------------------------------------------------------------------------------------"
    );
    }

    pub fn print_info(informações: BTreeMap<String, FileInfo>) {
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

        println!(
            "| {:<36} | {:<18} | {:<18} | {:<18} |",
            "TOTAL", total_file_info.code, total_file_info.comment, total_file_info.whitespace
        );

        Self::ln();
    }
}
