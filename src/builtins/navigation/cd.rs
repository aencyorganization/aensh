use crate::core::errors::{AenshError, AenshResult};
use colored::*;
use nix::unistd::chdir;
use std::env;

pub fn run(args: &[String]) -> AenshResult<()> {
    let target = args
        .get(0)
        .map(|s| s.as_str())
        .unwrap_or("~");

    let resolved = if target == "~" {
        env::var("HOME").unwrap_or_else(|_| "/".to_string())
    } else {
        target.to_string()
    };

    chdir(resolved.as_str()).map_err(|e| {
        AenshError::Io(format!(
            "não consigo mudar para {}: {}",
            resolved.bright_yellow(),
            e
        ))
    })?;

    Ok(())
}
