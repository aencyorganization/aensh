use crate::core::errors::AenshResult;
use colored::*;
use std::env;
use std::process::Command;
use gethostname::gethostname;

fn uname(arg: &str) -> String {
    Command::new("uname")
        .arg(arg)
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| "desconhecido".to_string())
}

pub fn run(_args: &[String]) -> AenshResult<()> {
    let user = env::var("USER").unwrap_or_else(|_| "user".into());
    let host = gethostname::gethostname().to_string_lossy().to_string();
    let os = uname("-s");
    let kernel = uname("-r");

    let logo = [
        "            .-/+oossssoo+/-.",
        "        `:+ssssssssssssssss+:`",
        "      -+sssssssssssssssssssss+-",
        "    .osssssssssssssssssssssssso.",
        "   :ssssssssssssssssssssssssssss:",
        "  /ssssssssssssssssssssssssssssss/",
        " `osssssssssssssssssssssssssssssso`",
        " :ssssssssssssssssssssssssssssssss:",
        " :ssssssssssssssssssssssssssssssss:",
        " :ssssssssssssssssssssssssssssssss:",
        " `osssssssssssssssssssssssssssssso`",
        "  /ssssssssssssssssssssssssssssss/",
        "   :ssssssssssssssssssssssssssss:",
        "    .osssssssssssssssssssssssso.",
        "      -+sssssssssssssssssssss+-",
        "        `:+ssssssssssssssss+:`",
        "            .-/+oossssoo+/-.",
    ];

    let info = vec![
        ("user", format!("{}@{}", user.bright_cyan(), host.bright_magenta())),
        ("os", os.bright_green().to_string()),
        ("kernel", kernel.bright_green().to_string()),
        ("shell", "aensh".bright_yellow().bold().to_string()),
        ("version", "0.2.0".bright_yellow().to_string()),
        ("rust", "🦀".bright_red().to_string()),
    ];

    println!();

    let max_lines = logo.len().max(info.len());

    for i in 0..max_lines {
        let left = logo.get(i).unwrap_or(&"");
        let left_colored = left.bright_cyan();

        if let Some((key, value)) = info.get(i) {
            println!("{}   {:>10}  {}", left_colored, key.bright_white().bold(), value);
        } else {
            println!("{}", left_colored);
        }
    }

    println!();
    Ok(())
}
