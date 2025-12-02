mod core;
mod builtins;

use std::env;
use colored::*;

use core::{
    show_banner, build_prompt, 
    parse_command_chain, execute_chain,
    Config, PluginManager, ReadLine
};

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn print_help() {
    println!("{}", "Aensh - A Modern Shell in Rust".bright_cyan().bold());
    println!("Versão: {}", VERSION);
    println!();
    println!("{}", "USO:".yellow().bold());
    println!("    aensh [OPÇÕES]");
    println!();
    println!("{}", "OPÇÕES:".yellow().bold());
    println!("    {}          Mostra esta mensagem de ajuda", "--help".green());
    println!("    {}       Mostra a versão", "--version".green());
    println!("    {} Define Aensh como shell padrão ao iniciar terminal", "--default true".green());
    println!("    {}  Remove Aensh como shell padrão", "--default false".green());
    println!();
    println!("{}", "EXEMPLOS:".yellow().bold());
    println!("    aensh                    # Inicia o shell");
    println!("    aensh --default true     # Define como shell padrão");
    println!("    aensh --default false    # Remove como shell padrão");
    println!();
    println!("{}", "RECURSOS:".yellow().bold());
    println!("    • Navegação com setas (histórico e cursor)");
    println!("    • Piping de comandos (cmd1 | cmd2)");
    println!("    • Encadeamento com && (cmd1 && cmd2)");
    println!("    • Sistema de plugins personalizados");
    println!();
    println!("{}", "DIRETÓRIOS:".yellow().bold());
    println!("    Plugins: ~/.config/aensh/plugins/");
    println!("    Config:  ~/.config/aensh/config.json");
    println!("    History: ~/.aensh_history");
}

fn print_version() {
    println!("Aensh {}", VERSION);
}

fn handle_args() -> bool {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        return false; // No args, run shell
    }
    
    match args[1].as_str() {
        "--help" | "-h" => {
            print_help();
            true
        }
        "--version" | "-v" => {
            print_version();
            true
        }
        "--default" => {
            if args.len() < 3 {
                eprintln!("{} --default requer 'true' ou 'false'", "Erro:".red());
                return true;
            }
            
            let mut config = Config::load();
            match args[2].as_str() {
                "true" | "1" | "yes" => {
                    match config.set_default_shell(true) {
                        Ok(_) => println!("{} Aensh definido como shell padrão!", "✓".green()),
                        Err(e) => eprintln!("{} Falha ao configurar: {}", "✗".red(), e),
                    }
                }
                "false" | "0" | "no" => {
                    match config.set_default_shell(false) {
                        Ok(_) => println!("{} Aensh removido como shell padrão.", "✓".green()),
                        Err(e) => eprintln!("{} Falha ao configurar: {}", "✗".red(), e),
                    }
                }
                other => {
                    eprintln!("{} Valor inválido '{}'. Use 'true' ou 'false'.", "Erro:".red(), other);
                }
            }
            true
        }
        _ => false
    }
}

fn main() {
    // Handle command line arguments
    if handle_args() {
        return;
    }
    
    show_banner();
    
    let plugin_manager = PluginManager::new();
    let mut readline = ReadLine::new();
    
    loop {
        let prompt = build_prompt();
        
        let input = match readline.read_line(&prompt) {
            Ok(Some(line)) => line,
            Ok(None) => {
                // EOF (Ctrl+D)
                println!("{}", "Goodbye!".bright_green());
                break;
            }
            Err(e) => {
                eprintln!("{} Erro de leitura: {}", "✗".red(), e);
                continue;
            }
        };

        let trimmed = input.trim();
        if trimmed.is_empty() {
            continue;
        }

        // Parse and execute command chain
        let chain = match parse_command_chain(trimmed, &plugin_manager) {
            Ok(chain) => chain,
            Err(err) => {
                err.print();
                continue;
            }
        };

        if let Err(err) = execute_chain(chain, &plugin_manager) {
            err.print();
        }
    }
}
