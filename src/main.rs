mod core;
mod builtins;

use std::env;
use colored::*;

use core::{
    show_banner, build_prompt, 
    parse_command_chain, execute_chain,
    Config, PluginManager, ReadLine,
    check_and_run_setup, AliasManager
};

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn print_help() {
    println!("{}", "Aensh - A Modern Shell in Rust".bright_cyan().bold());
    println!("Versão: {}", VERSION);
    println!();
    println!("{}", "USO:".yellow().bold());
    println!("    aensh [OPÇÕES] [COMANDO]");
    println!();
    println!("{}", "OPÇÕES:".yellow().bold());
    println!("    {}, {}       Mostra esta mensagem de ajuda", "-h".green(), "--help".green());
    println!("    {}, {}    Mostra a versão", "-v".green(), "--version".green());
    println!("    {}            Re-executa o setup inicial", "--setup".green());
    println!("    {}           Mostra informações do sistema", "--info".green());
    println!("    {}         Mostra configuração atual", "--config".green());
    println!("    {}  Define Aensh como shell padrão", "--default true".green());
    println!("    {} Remove Aensh como shell padrão", "--default false".green());
    println!("    {}      Executa um comando e sai", "-c <cmd>".green());
    println!();
    println!("{}", "EXEMPLOS:".yellow().bold());
    println!("    aensh                    # Inicia o shell");
    println!("    aensh --setup            # Re-executa o setup");
    println!("    aensh --default true     # Define como shell padrão");
    println!("    aensh -c \"ls -la\"        # Executa comando e sai");
    println!("    aensh --info             # Mostra info do sistema");
    println!();
    println!("{}", "RECURSOS:".yellow().bold());
    println!("    • Navegação com setas (histórico ↑↓ e cursor ←→)");
    println!("    • Piping de comandos (cmd1 | cmd2)");
    println!("    • Encadeamento com && (cmd1 && cmd2)");
    println!("    • Comandos externos do sistema (curl, git, python, etc)");
    println!("    • Sistema de plugins personalizados");
    println!();
    println!("{}", "DIRETÓRIOS:".yellow().bold());
    println!("    Plugins: ~/.config/aensh/plugins/");
    println!("    Config:  ~/.config/aensh/config.json");
    println!("    History: ~/.aensh_history");
    println!();
    println!("{}", "MAIS INFO:".yellow().bold());
    println!("    https://github.com/aencyorganization/aensh");
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
            
            // Check if setup was completed
            if config.needs_setup() {
                eprintln!("{} Execute o Aensh primeiro para completar o setup inicial.", "Erro:".red());
                return true;
            }
            
            match args[2].as_str() {
                "true" | "1" | "yes" => {
                    match config.set_default_shell(true) {
                        Ok(_) => {
                            let shell_name = config.previous_shell.map(|s| s.name()).unwrap_or("shell");
                            println!("{} Aensh definido como shell padrão!", "✓".green());
                            println!("  Script adicionado ao seu {}", shell_name.bright_cyan());
                        }
                        Err(e) => eprintln!("{} Falha ao configurar: {}", "✗".red(), e),
                    }
                }
                "false" | "0" | "no" => {
                    match config.set_default_shell(false) {
                        Ok(_) => {
                            let shell_name = config.previous_shell.map(|s| s.name()).unwrap_or("shell");
                            println!("{} Aensh removido como shell padrão.", "✓".green());
                            println!("  Script removido do seu {}", shell_name.bright_cyan());
                        }
                        Err(e) => eprintln!("{} Falha ao configurar: {}", "✗".red(), e),
                    }
                }
                other => {
                    eprintln!("{} Valor inválido '{}'. Use 'true' ou 'false'.", "Erro:".red(), other);
                }
            }
            true
        }
        "--setup" => {
            // Force re-run setup
            match core::setup::run_setup() {
                Ok(_) => println!("{} Setup concluído!", "✓".green()),
                Err(e) => eprintln!("{} Erro no setup: {}", "✗".red(), e),
            }
            true
        }
        "--info" => {
            print_info();
            true
        }
        "--config" => {
            print_config();
            true
        }
        "-c" => {
            if args.len() < 3 {
                eprintln!("{} -c requer um comando", "Erro:".red());
                return true;
            }
            
            let cmd = args[2..].join(" ");
            execute_single_command(&cmd);
            true
        }
        _ => false
    }
}

fn print_info() {
    println!("{}", "Aensh - System Info".bright_cyan().bold());
    println!();
    println!("{}: {}", "Versão".yellow(), VERSION);
    println!("{}: {}", "OS".yellow(), std::env::consts::OS);
    println!("{}: {}", "Arch".yellow(), std::env::consts::ARCH);
    println!("{}: {}", "Home".yellow(), dirs::home_dir().map(|p| p.display().to_string()).unwrap_or_else(|| "N/A".to_string()));
    println!("{}: {}", "Config".yellow(), core::Config::config_path().display());
    
    let config = Config::load();
    println!("{}: {}", "Shell anterior".yellow(), config.previous_shell.map(|s| s.name()).unwrap_or("Não configurado"));
    println!("{}: {}", "Shell padrão".yellow(), if config.default_shell { "Sim" } else { "Não" });
    
    // Count plugins
    let plugin_manager = PluginManager::new();
    println!("{}: {}", "Plugins".yellow(), plugin_manager.list().len());
}

fn print_config() {
    let config = Config::load();
    
    println!("{}", "Aensh - Configuração".bright_cyan().bold());
    println!();
    
    if config.needs_setup() {
        println!("{} Setup não foi completado. Execute: aensh --setup", "⚠".yellow());
        return;
    }
    
    println!("{}", serde_json::to_string_pretty(&config).unwrap_or_else(|_| "Erro ao serializar".to_string()));
}

fn execute_single_command(cmd: &str) {
    let plugin_manager = PluginManager::new();
    
    let chain = match parse_command_chain(cmd, &plugin_manager) {
        Ok(chain) => chain,
        Err(err) => {
            err.print();
            return;
        }
    };

    if let Err(err) = execute_chain(chain, &plugin_manager) {
        err.print();
    }
}

fn main() {
    // Handle command line arguments
    if handle_args() {
        return;
    }
    
    // Check if setup is needed
    let _config = match check_and_run_setup() {
        Ok(config) => config,
        Err(e) => {
            eprintln!("{} Erro no setup: {}", "✗".red(), e);
            return;
        }
    };
    
    show_banner();
    
    let plugin_manager = PluginManager::new();
    let mut alias_manager = AliasManager::new();
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

        // Expand aliases
        let expanded = alias_manager.expand(trimmed);
        
        // Check for special alias command
        if trimmed.starts_with("alias") || trimmed == "reload" || trimmed == "source" {
            let parts: Vec<String> = trimmed.split_whitespace().map(|s| s.to_string()).collect();
            if !parts.is_empty() {
                let cmd = core::command::Command::new(parts[0].clone(), parts[1..].to_vec());
                if let Err(err) = builtins::dispatch_with_alias(&cmd, &mut alias_manager) {
                    err.print();
                }
                continue;
            }
        }

        // Parse and execute command chain
        let chain = match parse_command_chain(&expanded, &plugin_manager) {
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
