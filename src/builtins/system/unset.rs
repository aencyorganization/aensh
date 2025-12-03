use std::env;
use colored::*;
use crate::core::errors::AenshResult;

pub fn run(args: &[String]) -> AenshResult<()> {
    if args.is_empty() {
        eprintln!("{} Uso: unset NOME", "Erro:".red());
        return Ok(());
    }

    for name in args {
        env::remove_var(name);
        println!("{} Variável '{}' removida", "✓".green(), name.bright_cyan());
    }

    Ok(())
}
