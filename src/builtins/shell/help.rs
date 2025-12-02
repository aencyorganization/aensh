use crate::builtins::SUPPORTED_COMMANDS;
use crate::core::errors::AenshResult;
use crate::core::plugins::PluginManager;
use colored::*;

pub fn run(_args: &[String]) -> AenshResult<()> {
    println!("\n{} {}\n", "Aensh".bright_blue().bold(), "Comandos".bright_cyan());
    
    println!("{}", "📚 Comandos de Shell:".bright_yellow().bold());
    print_commands(&["help", "exit", "quit", "plugin"]);
    
    println!("\n{}", "🗂️  Navegação:".bright_yellow().bold());
    print_commands(&["cd", "pwd"]);
    
    println!("\n{}", "📁 Sistema de Arquivos:".bright_yellow().bold());
    print_commands(&["ls", "cat", "mkdir", "touch", "rm", "cp", "mv", "find", "grep", "tree"]);
    
    println!("\n{}", "⚙️  Sistema:".bright_yellow().bold());
    print_commands(&["echo", "clear", "info", "whoami", "date", "stat"]);

    // Show plugins
    let manager = PluginManager::new();
    let plugins = manager.list();
    if !plugins.is_empty() {
        println!("\n{}", "🔌 Plugins:".bright_yellow().bold());
        for plugin in plugins {
            println!("  {} - {}", plugin.name.bright_green().bold(), plugin.description);
        }
    }

    println!("\n{}", "💡 Dicas:".bright_yellow().bold());
    println!("  • Use {} para encadear comandos", "&&".bright_cyan());
    println!("  • Use {} para piping", "|".bright_cyan());
    println!("  • Use {} para navegar no histórico", "↑/↓".bright_cyan());
    println!("  • Use {} para mover o cursor", "←/→".bright_cyan());

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
