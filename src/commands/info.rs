use crate::errors::AenshResult;
use colored::*;
use std::env;

pub fn run(_args: &[String]) -> AenshResult<()> {
    let user = env::var("USER").unwrap_or_else(|_| "user".into());
    let host = gethostname::gethostname().to_string_lossy().to_string();

    println!(
        "{}",
        format!("Usuário: {} | Máquina: {}", user, host)
            .bright_white()
            .on_bright_black()
    );
    println!("{}", "Aensh 0.1.0 - Shell totalmente em Rust".bright_cyan());
    Ok(())
}
