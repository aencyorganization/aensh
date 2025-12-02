use crate::core::errors::{AenshError, AenshResult};
use colored::*;
use std::fs;

pub fn run(args: &[String]) -> AenshResult<()> {
    if args.len() < 2 {
        return Err(AenshError::Validation("uso: mv <origem> <destino>".into()));
    }

    let source = &args[0];
    let dest = &args[1];

    fs::rename(source, dest)
        .map_err(|e| AenshError::Io(format!("não consigo mover {}: {}", source, e)))?;

    println!("{} {} movido para {}", "✓".bright_green(), source.bright_cyan(), dest.bright_cyan());
    Ok(())
}
