use colored::*;

pub const AENSH_FIGLET: &str = r#"
    _                  _     
   / \   ___ _ __  ___| |__  
  / _ \ / _ \ '_ \/ __| '_ \ 
 / ___ \  __/ | | \__ \ | | |
/_/   \_\___|_| |_|___/_| |_|
"#;

pub fn show_banner() {
    println!("{}", AENSH_FIGLET.bright_magenta().bold());
    println!(
        "{}\n",
        "Bem-vindo ao Aensh v0.2.0 - use 'help' para começar".bright_white()
    );
}
