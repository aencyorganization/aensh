use std::fs;
use colored::*;
use crate::core::errors::{AenshError, AenshResult};

pub fn run(args: &[String]) -> AenshResult<()> {
    if args.is_empty() {
        eprintln!("{} Uso: wc [-l|-w|-c] <arquivo>", "Erro:".red());
        return Ok(());
    }

    let mut show_lines = true;
    let mut show_words = true;
    let mut show_chars = true;
    let mut file_arg = 0;

    // Parse flags
    for (i, arg) in args.iter().enumerate() {
        match arg.as_str() {
            "-l" => {
                show_lines = true;
                show_words = false;
                show_chars = false;
                file_arg = i + 1;
            }
            "-w" => {
                show_lines = false;
                show_words = true;
                show_chars = false;
                file_arg = i + 1;
            }
            "-c" => {
                show_lines = false;
                show_words = false;
                show_chars = true;
                file_arg = i + 1;
            }
            _ => break,
        }
    }

    if file_arg >= args.len() {
        file_arg = 0;
    }

    let path = &args[file_arg];
    let content = fs::read_to_string(path)
        .map_err(|e| AenshError::Io(format!("Falha ao ler '{}': {}", path, e)))?;

    let lines = content.lines().count();
    let words = content.split_whitespace().count();
    let chars = content.len();

    let mut output = Vec::new();
    
    if show_lines {
        output.push(format!("{}", lines));
    }
    if show_words {
        output.push(format!("{}", words));
    }
    if show_chars {
        output.push(format!("{}", chars));
    }

    println!("{} {}", output.join(" "), path.bright_cyan());

    Ok(())
}
