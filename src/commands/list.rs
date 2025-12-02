use crate::errors::{AenshError, AenshResult};
use colored::*;
use std::fs;

pub fn run(args: &[String]) -> AenshResult<()> {
    let target = args.get(0).map(|s| s.as_str()).unwrap_or(".");
    let entries = fs::read_dir(target)
        .map_err(|e| AenshError::Io(format!("não consigo listar {}: {}", target, e)))?;

    let mut items = Vec::new();
    for entry in entries {
        let entry = entry.map_err(|e| AenshError::Io(e.to_string()))?;
        let meta = entry
            .metadata()
            .map_err(|e| AenshError::Io(format!("erro em metadata: {}", e)))?;
        items.push((entry.file_name(), meta.is_dir()));
    }

    items.sort_by(|a, b| a.0.to_string_lossy().cmp(&b.0.to_string_lossy()));

    for (name, is_dir) in items {
        let display = name.to_string_lossy();
        if is_dir {
            println!("{}", format!("{}/", display).bright_blue());
        } else {
            println!("{}", display.bright_white());
        }
    }
    Ok(())
}
