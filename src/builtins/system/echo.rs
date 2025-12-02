use crate::core::errors::AenshResult;
use colored::*;

pub fn run(args: &[String]) -> AenshResult<()> {
    if args.is_empty() {
        println!();
    } else {
        println!("{}", args.join(" ").bright_white());
    }
    Ok(())
}
