use crate::core::errors::AenshResult;
use colored::*;

pub fn run(_args: &[String]) -> AenshResult<()> {
    println!("{}", "Até logo! 👋".bright_green().bold());
    std::process::exit(0);
}
