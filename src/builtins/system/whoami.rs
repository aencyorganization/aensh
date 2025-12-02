use crate::core::errors::AenshResult;
use colored::*;
use std::env;

pub fn run(_args: &[String]) -> AenshResult<()> {
    let user = env::var("USER").unwrap_or_else(|_| "unknown".to_string());
    println!("{}", user.bright_green().bold());
    Ok(())
}
