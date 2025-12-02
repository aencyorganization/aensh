use crate::core::errors::{AenshError, AenshResult};
use colored::*;
use std::fs;

pub fn run(args: &[String]) -> AenshResult<()> {
    if args.len() < 2 {
        return Err(AenshError::Validation("uso: cp <origem> <destino>".into()));
    }

    let source = &args[0];
    let dest = &args[1];

    let metadata = fs::metadata(source)
        .map_err(|e| AenshError::Io(format!("não consigo acessar {}: {}", source, e)))?;

    if metadata.is_dir() {
        copy_dir_recursive(source, dest)?;
    } else {
        fs::copy(source, dest)
            .map_err(|e| AenshError::Io(format!("não consigo copiar {}: {}", source, e)))?;
    }

    println!("{} {} copiado para {}", "✓".bright_green(), source.bright_cyan(), dest.bright_cyan());
    Ok(())
}

fn copy_dir_recursive(src: &str, dst: &str) -> AenshResult<()> {
    fs::create_dir_all(dst)
        .map_err(|e| AenshError::Io(format!("não consigo criar {}: {}", dst, e)))?;

    for entry in fs::read_dir(src)
        .map_err(|e| AenshError::Io(format!("não consigo ler {}: {}", src, e)))?
    {
        let entry = entry.map_err(|e| AenshError::Io(e.to_string()))?;
        let path = entry.path();
        let file_name = entry.file_name();
        let dest_path = format!("{}/{}", dst, file_name.to_string_lossy());

        if path.is_dir() {
            copy_dir_recursive(path.to_str().unwrap(), &dest_path)?;
        } else {
            fs::copy(&path, &dest_path)
                .map_err(|e| AenshError::Io(format!("não consigo copiar {:?}: {}", path, e)))?;
        }
    }
    Ok(())
}
