use crate::errors::AenshResult;
use colored::*;

pub fn run(_args: &[String]) -> AenshResult<()> {
    println!("{}", "Até logo!".bright_green());
    std::process::exit(0);
}
