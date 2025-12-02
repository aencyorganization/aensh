use crate::command::Command;
use crate::errors::{AenshError, AenshResult};

pub mod clear;
pub mod exit_cmd;
pub mod go;
pub mod help;
pub mod info;
pub mod list;
pub mod pwd;
pub mod show;

pub const SUPPORTED_COMMANDS: &[(&str, &str)] = &[
    ("ahelp", "Mostra a lista de comandos"),
    ("aexit", "Encerra o shell"),
    ("aclear", "Limpa a tela"),
    ("ago", "Altera o diretório atual"),
    ("apwd", "Mostra o diretório atual"),
    ("alist", "Lista arquivos e diretórios"),
    ("ashow", "Exibe o conteúdo de arquivos"),
    ("ainfo", "Mostra informações do Aensh"),
];

pub fn is_supported(name: &str) -> bool {
    SUPPORTED_COMMANDS.iter().any(|(cmd, _)| cmd == &name)
}

pub fn dispatch(command: &Command) -> AenshResult<()> {
    match command.name.as_str() {
        "aclear" => clear::run(&command.args),
        "aexit" => exit_cmd::run(&command.args),
        "ago" => go::run(&command.args),
        "ahelp" => help::run(&command.args),
        "ainfo" => info::run(&command.args),
        "alist" => list::run(&command.args),
        "apwd" => pwd::run(&command.args),
        "ashow" => show::run(&command.args),
        other => Err(AenshError::InvalidCommand(other.to_string())),
    }
}
