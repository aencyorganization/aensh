use crate::core::errors::{AenshError, AenshResult};
use colored::*;
use std::fs;

pub fn run(args: &[String]) -> AenshResult<()> {
    let start_dir = args.get(0).map(|s| s.as_str()).unwrap_or(".");
    let max_depth = args
        .get(1)
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(usize::MAX);

    println!("{}", start_dir.bright_cyan().bold());
    print_tree(start_dir, "", 0, max_depth)?;
    Ok(())
}

fn print_tree(dir: &str, prefix: &str, depth: usize, max_depth: usize) -> AenshResult<()> {
    if depth >= max_depth {
        return Ok(());
    }

    let entries = fs::read_dir(dir)
        .map_err(|e| AenshError::Io(format!("não consigo ler {}: {}", dir, e)))?;

    let mut items: Vec<_> = entries
        .filter_map(|e| e.ok())
        .collect();

    items.sort_by(|a, b| {
        let a_name = a.file_name();
        let b_name = b.file_name();
        a_name.cmp(&b_name)
    });

    for (idx, entry) in items.iter().enumerate() {
        let is_last = idx == items.len() - 1;
        let connector = if is_last { "└── " } else { "├── " };
        let next_prefix = if is_last { "    " } else { "│   " };

        let file_name = entry.file_name();
        let file_name_str = file_name.to_string_lossy();

        let meta = entry
            .metadata()
            .map_err(|e| AenshError::Io(format!("erro em metadata: {}", e)))?;

        if meta.is_dir() {
            println!("{}{}{} {}", prefix, connector, "📁".bright_blue(), file_name_str.bright_blue().bold());
            let path = entry.path();
            let _ = print_tree(path.to_str().unwrap_or(""), &format!("{}{}", prefix, next_prefix), depth + 1, max_depth);
        } else {
            let size = format_size(meta.len());
            println!("{}{}{} {} {}", prefix, connector, "📄".bright_white(), file_name_str.bright_white(), size.bright_black());
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
