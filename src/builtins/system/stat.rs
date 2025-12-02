use crate::core::errors::{AenshError, AenshResult};
use colored::*;
use std::fs;

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

pub fn run(args: &[String]) -> AenshResult<()> {
    if args.is_empty() {
        return Err(AenshError::Validation("uso: stat <arquivo>".into()));
    }

    let path = &args[0];
    let metadata = fs::metadata(path)
        .map_err(|e| AenshError::Io(format!("não consigo acessar {}: {}", path, e)))?;

    println!("\n{} {}\n", "📊 Informações de:".bright_cyan().bold(), path.bright_yellow());

    println!("{} {}", "Tipo:".bright_white().bold(), 
        if metadata.is_dir() { "Diretório 📁" } else { "Arquivo 📄" }.bright_green());
    
    println!("{} {} bytes", "Tamanho:".bright_white().bold(), 
        format_size(metadata.len()).bright_cyan());
    
    println!("{} {}", "Permissões:".bright_white().bold(), 
        format_permissions(metadata.permissions().mode()).bright_magenta());

    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt;
        println!("{} {}", "Inode:".bright_white().bold(), 
            metadata.ino().to_string().bright_white());
        println!("{} {}", "Links:".bright_white().bold(), 
            metadata.nlink().to_string().bright_white());
    }

    println!();
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

    format!("{:.2} {}", size, UNITS[unit_idx])
}

#[cfg(unix)]
fn format_permissions(mode: u32) -> String {
    let user = (mode >> 6) & 7;
    let group = (mode >> 3) & 7;
    let other = mode & 7;

    format!("{}{}{}", perm_to_string(user), perm_to_string(group), perm_to_string(other))
}

#[cfg(not(unix))]
fn format_permissions(_mode: u32) -> String {
    "N/A".to_string()
}

fn perm_to_string(perm: u32) -> String {
    let r = if perm & 4 != 0 { "r" } else { "-" };
    let w = if perm & 2 != 0 { "w" } else { "-" };
    let x = if perm & 1 != 0 { "x" } else { "-" };
    format!("{}{}{}", r, w, x)
}
