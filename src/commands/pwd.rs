use crate::errors::AenshResult;
use colored::*;
use std::env;

pub fn run(_args: &[String]) -> AenshResult<()> {
    let path = env::current_dir()?;
    println!("{}", path.display().to_string().bright_cyan());
    Ok(())
}
