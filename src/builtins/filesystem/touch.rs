use crate::core::errors::{AenshError, AenshResult};
use colored::*;
use std::fs::OpenOptions;

pub fn run(args: &[String]) -> AenshResult<()> {
    if args.is_empty() {
        return Err(AenshError::Validation("uso: touch <arquivo>".into()));
    }

    for file in args {
        OpenOptions::new()
            .write(true)
            .create(true)
            .open(file)
            .map_err(|e| AenshError::Io(format!("não consigo criar {}: {}", file, e)))?;
        println!("{} Arquivo {} criado", "✓".bright_green(), file.bright_cyan());
    }
    Ok(())
}
