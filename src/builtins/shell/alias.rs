use colored::*;
use crate::core::errors::AenshResult;
use crate::core::aliases::AliasManager;

pub fn run(args: &[String], alias_manager: &mut AliasManager) -> AenshResult<()> {
    if args.is_empty() {
        // List all aliases
        alias_manager.list();
        return Ok(());
    }

    match args[0].as_str() {
        "help" | "--help" | "-h" => {
            print_help();
        }
        "reload" => {
            alias_manager.reload();
        }
        "add" => {
            if args.len() < 3 {
                eprintln!("{} Uso: alias add <nome> <comando>", "Erro:".red());
                return Ok(());
            }
            let name = args[1].clone();
            let command = args[2..].join(" ");
            alias_manager.add(name.clone(), command)?;
            println!("{} Alias '{}' adicionado", "✓".green(), name.bright_cyan());
        }
        "remove" | "rm" => {
            if args.len() < 2 {
                eprintln!("{} Uso: alias remove <nome>", "Erro:".red());
                return Ok(());
            }
            let name = &args[1];
            alias_manager.remove(name)?;
            println!("{} Alias '{}' removido", "✓".green(), name.bright_cyan());
        }
        "list" => {
            alias_manager.list();
        }
        _ => {
            // Check if it's a definition: alias name='command'
            let input = args.join(" ");
            if let Some(eq_pos) = input.find('=') {
                let name = input[..eq_pos].trim().to_string();
                let mut value = input[eq_pos + 1..].trim().to_string();
                
                // Remove quotes
                if (value.starts_with('\'') && value.ends_with('\'')) ||
                   (value.starts_with('"') && value.ends_with('"')) {
                    value = value[1..value.len()-1].to_string();
                }
                
                if !name.is_empty() && !value.is_empty() {
                    alias_manager.add(name.clone(), value)?;
                    println!("{} Alias '{}' definido", "✓".green(), name.bright_cyan());
                } else {
                    eprintln!("{} Formato inválido. Use: alias nome='comando'", "Erro:".red());
                }
            } else {
                // Show specific alias
                let name = &args[0];
                if let Some(command) = alias_manager.get(name) {
                    println!("{} {} {}", 
                        name.bright_green(),
                        "→".bright_black(),
                        command.bright_white()
                    );
                } else {
                    eprintln!("{} Alias '{}' não encontrado", "✗".red(), name);
                }
            }
        }
    }

    Ok(())
}

fn print_help() {
    println!("{}", "alias - Gerenciar aliases".bright_cyan().bold());
    println!();
    println!("{}", "USO:".yellow().bold());
    println!("    alias                      Lista todos os aliases");
    println!("    alias <nome>               Mostra um alias específico");
    println!("    alias nome='comando'       Define um alias");
    println!("    alias add <nome> <cmd>     Adiciona um alias");
    println!("    alias remove <nome>        Remove um alias");
    println!("    alias reload               Recarrega ~/.aenshrc");
    println!();
    println!("{}", "EXEMPLOS:".yellow().bold());
    println!("    alias ll='ls -la'");
    println!("    alias add gs git status");
    println!("    alias remove ll");
    println!();
    println!("{}", "ARQUIVO:".yellow().bold());
    println!("    ~/.aenshrc                 Arquivo de configuração");
}
