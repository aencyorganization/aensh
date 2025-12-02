use crate::core::errors::AenshResult;
use crate::core::plugins::PluginManager;
use colored::*;

pub fn run(args: &[String]) -> AenshResult<()> {
    if args.is_empty() {
        return show_help();
    }

    match args[0].as_str() {
        "list" => list_plugins(),
        "add" => {
            if args.len() < 4 {
                eprintln!("{} Uso: plugin add <nome> <comando> <descrição>", "Erro:".red());
                return Ok(());
            }
            add_plugin(&args[1], &args[2], &args[3..].join(" "))
        }
        "remove" => {
            if args.len() < 2 {
                eprintln!("{} Uso: plugin remove <nome>", "Erro:".red());
                return Ok(());
            }
            remove_plugin(&args[1])
        }
        "help" | "--help" | "-h" => show_help(),
        other => {
            eprintln!("{} Subcomando desconhecido: {}", "Erro:".red(), other);
            show_help()
        }
    }
}

fn show_help() -> AenshResult<()> {
    println!("\n{}", "Plugin Manager".bright_cyan().bold());
    println!("{}", "═".repeat(40));
    println!();
    println!("{}", "SUBCOMANDOS:".yellow().bold());
    println!("  {} - Lista todos os plugins instalados", "list".green());
    println!("  {} - Adiciona um novo plugin", "add <nome> <comando> <descrição>".green());
    println!("  {} - Remove um plugin", "remove <nome>".green());
    println!("  {} - Mostra esta ajuda", "help".green());
    println!();
    println!("{}", "EXEMPLOS:".yellow().bold());
    println!("  plugin list");
    println!("  plugin add myplugin /path/to/script \"Meu plugin personalizado\"");
    println!("  plugin remove myplugin");
    println!();
    println!("{}", "NOTA:".yellow().bold());
    println!("  Plugins também podem ser adicionados colocando scripts executáveis em:");
    println!("  {}", "~/.config/aensh/plugins/".bright_white());
    println!();
    Ok(())
}

fn list_plugins() -> AenshResult<()> {
    let manager = PluginManager::new();
    let plugins = manager.list();

    if plugins.is_empty() {
        println!("{}", "Nenhum plugin instalado.".yellow());
        println!("Use {} para adicionar plugins.", "plugin add".green());
        return Ok(());
    }

    println!("\n{}", "Plugins Instalados".bright_cyan().bold());
    println!("{}", "═".repeat(40));
    
    for plugin in plugins {
        println!("  {} - {}", plugin.name.bright_green().bold(), plugin.description);
        println!("    Comando: {}", plugin.command.bright_white());
    }
    
    println!();
    Ok(())
}

fn add_plugin(name: &str, command: &str, description: &str) -> AenshResult<()> {
    let mut manager = PluginManager::new();
    
    match manager.register(name.to_string(), command.to_string(), description.to_string()) {
        Ok(_) => {
            println!("{} Plugin '{}' adicionado com sucesso!", "✓".green(), name.bright_green());
        }
        Err(e) => {
            eprintln!("{} {}", "✗".red(), e);
        }
    }
    
    Ok(())
}

fn remove_plugin(name: &str) -> AenshResult<()> {
    let mut manager = PluginManager::new();
    
    match manager.unregister(name) {
        Ok(_) => {
            println!("{} Plugin '{}' removido com sucesso!", "✓".green(), name.bright_green());
        }
        Err(e) => {
            eprintln!("{} {}", "✗".red(), e);
        }
    }
    
    Ok(())
}
