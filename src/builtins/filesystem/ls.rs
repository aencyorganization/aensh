use crate::core::errors::{AenshError, AenshResult};
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
        items.push((entry.file_name(), meta.is_dir(), meta.len()));
    }

    items.sort_by(|a, b| a.0.to_string_lossy().cmp(&b.0.to_string_lossy()));

    for (name, is_dir, size) in items {
        let display = name.to_string_lossy();
        if is_dir {
            println!("{} {}", "📁".bright_blue(), format!("{}/", display).bright_blue().bold());
        } else {
            let size_str = format_size(size);
            println!("{} {} {}", "📄".bright_white(), display.bright_white(), size_str.bright_black());
        }
    }
    Ok(())
}

fn format_size(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB"];
    let mut size = bytes as f64;
    let mut unit_idx = 0;

    while size >= 1024.0 && unit_idx < UNITS.len() - 1 {
        size /= 1024.0;
        unit_idx += 1;
    }

    format!("({:.1}{})", size, UNITS[unit_idx])
}
