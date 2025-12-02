use crate::commands::SUPPORTED_COMMANDS;
use crate::errors::AenshResult;
use colored::*;

pub fn run(_args: &[String]) -> AenshResult<()> {
    println!("\n{} {}\n", "Aensh".bright_blue().bold(), "Comandos".bright_cyan());
    println!("{}", "Comandos nativos:".bright_yellow().bold());

    for (cmd, desc) in SUPPORTED_COMMANDS {
        println!("  {} - {}", cmd.bright_green(), desc);
    }

    println!(
        "\n{}",
        "Todos os comandos são implementados internamente. Use o prefixo 'a'!".bright_white()
    );
    Ok(())
}
