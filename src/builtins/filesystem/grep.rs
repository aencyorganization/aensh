use crate::core::errors::{AenshError, AenshResult};
use colored::*;
use std::fs;

pub fn run(args: &[String]) -> AenshResult<()> {
    if args.len() < 2 {
        return Err(AenshError::Validation("uso: grep <padrão> <arquivo>".into()));
    }

    let pattern = &args[0];
    let file_path = &args[1];

    let content = fs::read_to_string(file_path)
        .map_err(|e| AenshError::Io(format!("não consigo abrir {}: {}", file_path, e)))?;

    let mut found = false;
    for (line_num, line) in content.lines().enumerate() {
        if line.contains(pattern) {
            found = true;
            // Destacar o padrão encontrado
            let highlighted = line.replace(pattern, &format!("{}", pattern.bright_yellow().bold()));
            println!("{} {}", format!("{}:", line_num + 1).bright_cyan(), highlighted);
        }
    }

    if !found {
        println!("{} Nenhuma correspondência encontrada para '{}'", "ℹ".bright_blue(), pattern);
    }

    Ok(())
}
