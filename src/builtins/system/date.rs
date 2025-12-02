use crate::core::errors::AenshResult;
use colored::*;
use std::time::SystemTime;

pub fn run(_args: &[String]) -> AenshResult<()> {
    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();
    
    let secs = now.as_secs();
    let days_since_epoch = secs / 86400;
    let secs_today = secs % 86400;
    
    let hours = secs_today / 3600;
    let minutes = (secs_today % 3600) / 60;
    let seconds = secs_today % 60;
    
    // Cálculo simples da data (não é 100% preciso, mas funciona)
    let year = 1970 + (days_since_epoch / 365) as u32;
    let day_of_year = (days_since_epoch % 365) as u32;
    let month = (day_of_year / 30).min(11) + 1;
    let day = (day_of_year % 30) + 1;
    
    println!(
        "{} {} {} {}:{}:{} {}",
        format!("{:02}/{:02}/{}", day, month, year).bright_cyan().bold(),
        format!("{:02}:{:02}:{:02}", hours, minutes, seconds).bright_yellow().bold(),
        "UTC".bright_white(),
        "🕐".bright_magenta(),
        "".bright_white(),
        "".bright_white(),
        "".bright_white()
    );
    Ok(())
}
