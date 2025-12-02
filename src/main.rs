mod core;
mod builtins;

use std::io::{self, Write};
use colored::*;
use nix::sys::signal::{self, SigHandler, Signal};

use core::{show_banner, parse_input, build_prompt};
use builtins::dispatch;

fn setup_signal_handlers() {
    unsafe {
        signal::signal(Signal::SIGINT, SigHandler::Handler(handle_sigint)).expect("Failed to setup SIGINT handler");
        signal::signal(Signal::SIGTERM, SigHandler::Handler(handle_sigterm)).expect("Failed to setup SIGTERM handler");
    }
}

extern "C" fn handle_sigint(_signal: libc::c_int) {
    println!("\n{} Use 'exit' para sair.", "Interrompido".yellow());
    // Re-setup handler to catch future Ctrl+C
    setup_signal_handlers();
}

extern "C" fn handle_sigterm(_signal: libc::c_int) {
    println!("\n{} Encerrando...", "Aensh".red());
    std::process::exit(0);
}

fn main() {
    show_banner();
    
    setup_signal_handlers();
    
    loop {
        let prompt = build_prompt();
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(0) => {
                println!("\n{}", "Goodbye!".bright_green());
                break;
            }
            Ok(_) => (),
            Err(e) => {
                eprintln!("{} Error reading input: {}", "Error".red(), e);
                continue;
            }
        }

        let command = match parse_input(&input) {
            Ok(cmd) => cmd,
            Err(err) => {
                err.print();
                continue;
            }
        };

        if let Err(err) = dispatch(&command) {
            err.print();
        }
    }
}
