use crate::core::errors::{AenshError, AenshResult};
use colored::*;
use std::fs;

pub fn run(args: &[String]) -> AenshResult<()> {
    if args.is_empty() {
        return Err(AenshError::Validation("uso: cat <arquivo>".into()));
    }

    for path in args {
        let data = fs::read_to_string(path)
            .map_err(|e| AenshError::Io(format!("não consigo abrir {}: {}", path, e)))?;
        println!("{}", data.bright_white());
    }
    Ok(())
}
