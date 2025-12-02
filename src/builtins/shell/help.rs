use crate::builtins::SUPPORTED_COMMANDS;
use crate::core::errors::AenshResult;
use colored::*;

pub fn run(_args: &[String]) -> AenshResult<()> {
    println!("\n{} {}\n", "Aensh".bright_blue().bold(), "Comandos".bright_cyan());
    
    println!("{}", "📚 Comandos de Shell:".bright_yellow().bold());
    print_commands(&["help", "exit", "quit"]);
    
    println!("\n{}", "🗂️  Navegação:".bright_yellow().bold());
    print_commands(&["cd", "pwd"]);
    
    println!("\n{}", "📁 Sistema de Arquivos:".bright_yellow().bold());
    print_commands(&["ls", "cat", "mkdir", "touch", "rm", "cp", "mv", "find", "grep", "tree"]);
    
    println!("\n{}", "⚙️  Sistema:".bright_yellow().bold());
    print_commands(&["echo", "clear", "info", "whoami", "date", "stat"]);

    println!(
        "\n{}\n",
        "Use 'help' para ver esta mensagem novamente.".bright_white()
    );
    Ok(())
}

fn print_commands(names: &[&str]) {
    for (cmd, desc) in SUPPORTED_COMMANDS {
        if names.contains(cmd) {
            println!("  {} - {}", cmd.bright_green().bold(), desc);
        }
    }
}
