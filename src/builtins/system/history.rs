use colored::*;
use crate::core::errors::AenshResult;
use crate::core::history::History;

pub fn run(args: &[String]) -> AenshResult<()> {
    let history = History::new();
    let entries = history.entries();
    
    if entries.is_empty() {
        println!("{} Histórico vazio", "ℹ".blue());
        return Ok(());
    }

    let limit = if !args.is_empty() {
        args[0].parse().unwrap_or(entries.len())
    } else {
        entries.len()
    };

    let start = if entries.len() > limit {
        entries.len() - limit
    } else {
        0
    };

    for (i, entry) in entries.iter().enumerate().skip(start) {
        println!("{:>5}  {}", 
            (i + 1).to_string().bright_black(),
            entry
        );
    }

    Ok(())
}
