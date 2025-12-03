use std::env;
use colored::*;
use crate::core::errors::AenshResult;

pub fn run(args: &[String]) -> AenshResult<()> {
    if args.is_empty() {
        eprintln!("{} Uso: export NOME=valor", "Erro:".red());
        return Ok(());
    }

    let input = args.join(" ");
    
    if let Some(eq_pos) = input.find('=') {
        let name = input[..eq_pos].trim();
        let value = input[eq_pos + 1..].trim();
        
        // Remove quotes if present
        let value = value.trim_matches(|c| c == '"' || c == '\'');
        
        env::set_var(name, value);
        println!("{} {}={}", "✓".green(), name.bright_cyan(), value);
    } else {
        // Just show the variable
        let name = &args[0];
        if let Ok(value) = env::var(name) {
            println!("{}={}", name.bright_cyan(), value);
        } else {
            eprintln!("{} Variável '{}' não definida", "✗".red(), name);
        }
    }

    Ok(())
}
