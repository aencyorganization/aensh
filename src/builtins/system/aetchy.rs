use crate::core::errors::AenshResult;
use colored::*;
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::time::Duration;
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

fn read_uptime() -> Option<Duration> {
    let contents = fs::read_to_string("/proc/uptime").ok()?;
    let first = contents.split_whitespace().next()?;
    let secs: f64 = first.parse().ok()?;
    Some(Duration::from_secs_f64(secs))
}

fn format_duration(d: Duration) -> String {
    let total_secs = d.as_secs();
    let days = total_secs / 86_400;
    let hours = (total_secs % 86_400) / 3_600;
    let mins = (total_secs % 3_600) / 60;

    if days > 0 {
        format!("{}d {}h {}m", days, hours, mins)
    } else if hours > 0 {
        format!("{}h {}m", hours, mins)
    } else {
        format!("{}m", mins)
    }
}

fn current_dir() -> String {
    std::env::current_dir()
        .unwrap_or_else(|_| PathBuf::from("?"))
        .display()
        .to_string()
}

pub fn run(_args: &[String]) -> AenshResult<()> {
    let user = env::var("USER").unwrap_or_else(|_| "user".into());
    let host = gethostname().to_string_lossy().to_string();
    let os = uname("-s");
    let kernel = uname("-r");
    let arch = uname("-m");

    let shell = env::var("SHELL").unwrap_or_else(|_| "aensh".into());
    let term = env::var("TERM").unwrap_or_else(|_| "desconhecido".into());
    let home = env::var("HOME").unwrap_or_else(|_| "?".into());
    let cwd = current_dir();

    let cpu_count = std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1);

    let uptime_str = read_uptime()
        .map(format_duration)
        .unwrap_or_else(|| "desconhecido".into());

    println!();
    println!(
        "{} {}",
        "aetchy".bright_magenta().bold(),
        "— snapshot do seu sistema".bright_white()
    );
    println!("{}", "─".repeat(50).bright_black());

    println!(
        "{:>10}  {}",
        "user".bright_white().bold(),
        format!("{}@{}", user.bright_cyan(), host.bright_magenta())
    );
    println!(
        "{:>10}  {}",
        "os".bright_white().bold(),
        format!("{} {}", os.bright_green(), arch.bright_green())
    );
    println!(
        "{:>10}  {}",
        "kernel".bright_white().bold(),
        kernel.bright_green()
    );
    println!(
        "{:>10}  {}",
        "shell".bright_white().bold(),
        shell.bright_yellow()
    );
    println!(
        "{:>10}  {}",
        "version".bright_white().bold(),
        "0.1.0".bright_yellow()
    );
    println!(
        "{:>10}  {}",
        "cpus".bright_white().bold(),
        format!("{}", cpu_count).bright_cyan()
    );
    println!(
        "{:>10}  {}",
        "uptime".bright_white().bold(),
        uptime_str.bright_cyan()
    );
    println!(
        "{:>10}  {}",
        "home".bright_white().bold(),
        home.bright_blue()
    );
    println!(
        "{:>10}  {}",
        "cwd".bright_white().bold(),
        cwd.bright_blue()
    );
    println!(
        "{:>10}  {}",
        "term".bright_white().bold(),
        term.bright_magenta()
    );

    println!("{}", "─".repeat(50).bright_black());
    println!();

    Ok(())
}
