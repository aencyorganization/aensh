use crate::core::errors::{AenshError, AenshResult};
use colored::*;
use std::fs;

pub fn run(args: &[String]) -> AenshResult<()> {
    if args.is_empty() {
        return Err(AenshError::Validation("uso: find <diretório> [padrão]".into()));
    }

    let start_dir = &args[0];
    let pattern = args.get(1).map(|s| s.as_str()).unwrap_or("");

    find_recursive(start_dir, pattern)?;
    Ok(())
}

fn find_recursive(dir: &str, pattern: &str) -> AenshResult<()> {
    let entries = fs::read_dir(dir)
        .map_err(|e| AenshError::Io(format!("não consigo ler {}: {}", dir, e)))?;

    for entry in entries {
        let entry = entry.map_err(|e| AenshError::Io(e.to_string()))?;
        let path = entry.path();
        let file_name = entry.file_name();
        let file_name_str = file_name.to_string_lossy();

        // Se não há padrão, mostra tudo
        // Se há padrão, filtra por nome
        let matches = pattern.is_empty() || file_name_str.contains(pattern);

        if matches {
            let meta = entry
                .metadata()
                .map_err(|e| AenshError::Io(format!("erro em metadata: {}", e)))?;

            if meta.is_dir() {
                println!("{} {}", "📁".bright_blue(), path.display().to_string().bright_blue());
            } else {
                println!("{} {}", "📄".bright_white(), path.display().to_string().bright_white());
            }
        }

        // Recursivamente buscar em subdiretórios
        if path.is_dir() {
            let _ = find_recursive(path.to_str().unwrap_or(""), pattern);
        }
    }

    Ok(())
}
