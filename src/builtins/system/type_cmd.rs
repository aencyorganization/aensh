use std::env;
use std::path::Path;
use colored::*;
use crate::core::errors::AenshResult;
use crate::builtins;
use crate::core::plugins::PluginManager;

pub fn run(args: &[String]) -> AenshResult<()> {
    if args.is_empty() {
        eprintln!("{} Uso: type <comando>", "Erro:".red());
        return Ok(());
    }

    let plugin_manager = PluginManager::new();

    for cmd in args {
        // Check if it's a builtin
        if builtins::is_supported(cmd) {
            println!("{} is a {} builtin command", cmd.bright_cyan(), "aensh".bright_green());
            continue;
        }

        // Check if it's a plugin
        if plugin_manager.get(cmd).is_some() {
            println!("{} is an {} plugin", cmd.bright_cyan(), "aensh".bright_yellow());
            continue;
        }

        // Search in PATH
        if let Ok(path_var) = env::var("PATH") {
            let mut found = false;
            
            for dir in path_var.split(':') {
                let full_path = Path::new(dir).join(cmd);
                if full_path.exists() && full_path.is_file() {
                    println!("{} is {}", cmd.bright_cyan(), full_path.display().to_string().bright_green());
                    found = true;
                    break;
                }
            }

            if !found {
                eprintln!("{}: not found", cmd.bright_red());
            }
        }
    }

    Ok(())
}
