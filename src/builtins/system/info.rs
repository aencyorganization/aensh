use crate::core::errors::AenshResult;
use colored::*;
use std::env;

pub fn run(_args: &[String]) -> AenshResult<()> {
    let user = env::var("USER").unwrap_or_else(|_| "user".into());
    let host = gethostname::gethostname().to_string_lossy().to_string();

    println!("\n{}", "═".repeat(50).bright_cyan());
    println!("{}", "  Aensh - A Modern Shell in Rust".bright_magenta().bold());
    println!("{}", "═".repeat(50).bright_cyan());
    println!("{} {}", "Versão:".bright_yellow().bold(), "0.1.0".bright_white());
    println!("{} {}", "Usuário:".bright_yellow().bold(), user.bright_green());
    println!("{} {}", "Máquina:".bright_yellow().bold(), host.bright_cyan());
    println!("{} {}", "Linguagem:".bright_yellow().bold(), "Rust 🦀".bright_red());
    println!("{}", "═".repeat(50).bright_cyan());
    println!();
    Ok(())
}
