use std::env;
use colored::*;
use crate::core::errors::AenshResult;

pub fn run(args: &[String]) -> AenshResult<()> {
    if args.is_empty() {
        // List all environment variables
        let mut vars: Vec<_> = env::vars().collect();
        vars.sort_by(|a, b| a.0.cmp(&b.0));
        
        for (key, value) in vars {
            println!("{}={}", key.bright_cyan(), value);
        }
    } else {
        // Show specific variable
        let name = &args[0];
        if let Ok(value) = env::var(name) {
            println!("{}", value);
        } else {
            eprintln!("{} Variável '{}' não definida", "✗".red(), name);
        }
    }

    Ok(())
}
