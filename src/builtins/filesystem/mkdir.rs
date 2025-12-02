use crate::core::errors::{AenshError, AenshResult};
use colored::*;
use std::fs;

pub fn run(args: &[String]) -> AenshResult<()> {
    if args.is_empty() {
        return Err(AenshError::Validation("uso: mkdir <diretório>".into()));
    }

    for dir in args {
        fs::create_dir(dir).map_err(|e| {
            AenshError::Io(format!("não consigo criar {}: {}", dir, e))
        })?;
        println!("{} Diretório {} criado", "✓".bright_green(), dir.bright_cyan());
    }
    Ok(())
}
