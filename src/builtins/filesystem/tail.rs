use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::VecDeque;
use colored::*;
use crate::core::errors::{AenshError, AenshResult};

pub fn run(args: &[String]) -> AenshResult<()> {
    if args.is_empty() {
        eprintln!("{} Uso: tail [-n NUM] <arquivo>", "Erro:".red());
        return Ok(());
    }

    let mut lines = 10usize;
    let mut file_arg = 0;

    // Parse arguments
    if args.len() >= 2 && args[0] == "-n" {
        lines = args[1].parse().unwrap_or(10);
        file_arg = 2;
    }

    if file_arg >= args.len() {
        eprintln!("{} Arquivo não especificado", "Erro:".red());
        return Ok(());
    }

    let path = &args[file_arg];
    let file = File::open(path)
        .map_err(|e| AenshError::Io(format!("Falha ao abrir '{}': {}", path, e)))?;

    let reader = BufReader::new(file);
    let mut buffer: VecDeque<String> = VecDeque::with_capacity(lines);
    
    for line in reader.lines().flatten() {
        if buffer.len() >= lines {
            buffer.pop_front();
        }
        buffer.push_back(line);
    }

    for line in buffer {
        println!("{}", line);
    }

    Ok(())
}
