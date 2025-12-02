use crate::errors::{AenshError, AenshResult};
use colored::*;
use std::fs;

pub fn run(args: &[String]) -> AenshResult<()> {
    let Some(path) = args.get(0) else {
        return Err(AenshError::Validation("use: ashow <arquivo>".into()));
    };

    let data = fs::read_to_string(path)
        .map_err(|e| AenshError::Io(format!("não consigo abrir {}: {}", path, e)))?;
    println!("{}", data.bright_white());
    Ok(())
}
