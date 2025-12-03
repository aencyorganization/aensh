use std::env;
use std::path::Path;
use colored::*;
use crate::core::errors::AenshResult;
use crate::builtins;

pub fn run(args: &[String]) -> AenshResult<()> {
    if args.is_empty() {
        eprintln!("{} Uso: which <comando>", "Erro:".red());
        return Ok(());
    }

    for cmd in args {
        // Check if it's a builtin
        if builtins::is_supported(cmd) {
            println!("{}: {} builtin", cmd.bright_cyan(), "aensh".bright_green());
            continue;
        }

        // Search in PATH
        if let Ok(path_var) = env::var("PATH") {
            let mut found = false;
            
            for dir in path_var.split(':') {
                let full_path = Path::new(dir).join(cmd);
                if full_path.exists() && full_path.is_file() {
                    println!("{}", full_path.display().to_string().bright_green());
                    found = true;
                    break;
                }
            }

            if !found {
                eprintln!("{} '{}' não encontrado", "✗".red(), cmd);
            }
        }
    }

    Ok(())
}
