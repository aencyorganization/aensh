use colored::*;
use gethostname::gethostname;
use std::env;
use std::path::Path;

pub fn build_prompt() -> String {
    let current_dir = env::current_dir().unwrap_or_else(|_| Path::new("/").to_path_buf());
    let user = env::var("USER").unwrap_or_else(|_| "user".to_string());
    let hostname = gethostname().to_string_lossy().to_string();

    format!(
        "{} {} {} {} ",
        user.bright_green().bold(),
        hostname.bright_magenta().bold(),
        current_dir.display().to_string().bright_cyan().bold(),
        "❯".bright_yellow().bold()
    )
}
