use crate::core::errors::AenshResult;
use std::io::{self, Write};

pub fn run(_args: &[String]) -> AenshResult<()> {
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush()?;
    Ok(())
}
