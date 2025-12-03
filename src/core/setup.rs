use std::io::{self, Write, stdout};
use colored::*;
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent},
    terminal::{disable_raw_mode, enable_raw_mode},
};

use super::config::{Config, PreviousShell};

pub fn run_setup() -> io::Result<Config> {
    let mut config = Config::load();
    
    println!();
    println!("{}", "╔══════════════════════════════════════════════════════════════╗".bright_cyan());
    println!("{}", "║           🦀 Bem-vindo ao Aensh! 🦀                           ║".bright_cyan());
    println!("{}", "║                                                              ║".bright_cyan());
    println!("{}", "║   Vamos configurar seu shell em alguns passos rápidos.       ║".bright_cyan());
    println!("{}", "╚══════════════════════════════════════════════════════════════╝".bright_cyan());
    println!();
    
    // Step 1: Select previous shell
    println!("{}", "Passo 1: Qual shell você estava usando antes?".yellow().bold());
    println!();
    println!("  {} Bash", "[1]".bright_green());
    println!("  {} Zsh", "[2]".bright_green());
    println!("  {} Fish", "[3]".bright_green());
    println!();
    print!("{} ", "Escolha (1-3):".bright_white());
    stdout().flush()?;
    
    let shell = loop {
        let choice = read_single_key()?;
        
        if let Some(shell) = PreviousShell::from_str(&choice) {
            println!("{}", choice);
            break shell;
        } else if choice == "q" || choice == "\x1b" {
            println!();
            println!("{}", "Setup cancelado.".yellow());
            return Ok(config);
        }
    };
    
    println!();
    println!("{} Shell anterior definido como: {}", "✓".green(), shell.name().bright_cyan());
    
    config.set_previous_shell(shell)?;
    
    // Step 2: Set as default shell?
    println!();
    println!("{}", "Passo 2: Deseja definir o Aensh como shell padrão?".yellow().bold());
    println!();
    println!("  Isso fará com que o Aensh inicie automaticamente");
    println!("  quando você abrir o terminal.");
    println!();
    println!("  {} Sim", "[S]".bright_green());
    println!("  {} Não", "[N]".bright_green());
    println!();
    print!("{} ", "Escolha (S/N):".bright_white());
    stdout().flush()?;
    
    let set_default = loop {
        let choice = read_single_key()?.to_lowercase();
        
        match choice.as_str() {
            "s" | "y" | "1" => {
                println!("Sim");
                break true;
            }
            "n" | "0" | "2" => {
                println!("Não");
                break false;
            }
            "q" | "\x1b" => {
                println!();
                println!("{}", "Setup cancelado.".yellow());
                return Ok(config);
            }
            _ => continue,
        }
    };
    
    if set_default {
        config.set_default_shell(true)?;
        println!();
        println!("{} Aensh definido como shell padrão!", "✓".green());
        println!("  O script foi adicionado ao seu {}", shell.rc_file().bright_cyan());
    } else {
        println!();
        println!("{} Você pode ativar isso depois com:", "ℹ".blue());
        println!("  {}", "aensh --default true".bright_white());
    }
    
    // Done!
    println!();
    println!("{}", "╔══════════════════════════════════════════════════════════════╗".bright_green());
    println!("{}", "║           ✅ Setup Concluído!                                 ║".bright_green());
    println!("{}", "╚══════════════════════════════════════════════════════════════╝".bright_green());
    println!();
    println!("{}", "Dicas:".yellow().bold());
    println!("  • Digite {} para ver comandos disponíveis", "help".bright_cyan());
    println!("  • Use {} para navegar no histórico", "↑/↓".bright_cyan());
    println!("  • Use {} para mover o cursor", "←/→".bright_cyan());
    println!("  • Comandos do sistema como {}, {} funcionam normalmente", "curl".bright_cyan(), "git".bright_cyan());
    println!();
    
    Ok(config)
}

fn read_single_key() -> io::Result<String> {
    enable_raw_mode()?;
    
    let result = loop {
        if let Event::Key(KeyEvent { code, .. }) = event::read()? {
            match code {
                KeyCode::Char(c) => break Ok(c.to_string()),
                KeyCode::Enter => break Ok("\n".to_string()),
                KeyCode::Esc => break Ok("\x1b".to_string()),
                _ => continue,
            }
        }
    };
    
    disable_raw_mode()?;
    result
}

/// Check if setup is needed and run it
pub fn check_and_run_setup() -> io::Result<Config> {
    let config = Config::load();
    
    if config.needs_setup() {
        run_setup()
    } else {
        Ok(config)
    }
}
