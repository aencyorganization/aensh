use crate::core::errors::{AenshError, AenshResult};
use colored::*;
use std::fs;

pub fn run(args: &[String]) -> AenshResult<()> {
    if args.is_empty() {
        return Err(AenshError::Validation("uso: rm <arquivo/diretório>".into()));
    }

    for path in args {
        let metadata = fs::metadata(path)
            .map_err(|e| AenshError::Io(format!("não consigo acessar {}: {}", path, e)))?;

        if metadata.is_dir() {
            fs::remove_dir_all(path).map_err(|e| {
                AenshError::Io(format!("não consigo remover diretório {}: {}", path, e))
            })?;
        } else {
            fs::remove_file(path).map_err(|e| {
                AenshError::Io(format!("não consigo remover arquivo {}: {}", path, e))
            })?;
        }
        println!("{} {} removido", "✓".bright_green(), path.bright_yellow());
    }
    Ok(())
}
