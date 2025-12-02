use colored::*;
use std::fmt;

pub type AenshResult<T> = Result<T, AenshError>;

#[derive(Debug)]
pub enum AenshError {
    EmptyInput,
    Validation(String),
    InvalidCommand(String),
    Io(String),
}

impl AenshError {
    pub fn describe(&self) -> String {
        match self {
            AenshError::EmptyInput => "nenhum comando informado".to_string(),
            AenshError::Validation(msg) => format!("comando inválido: {}", msg),
            AenshError::InvalidCommand(cmd) => format!("'{}' não existe no Aensh. Use 'ahelp'.", cmd),
            AenshError::Io(msg) => msg.to_string(),
        }
    }

    pub fn print(&self) {
        eprintln!("{} {}", "Erro:".bright_red().bold(), self.describe());
    }
}

impl fmt::Display for AenshError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.describe())
    }
}

impl std::error::Error for AenshError {}

impl From<std::io::Error> for AenshError {
    fn from(value: std::io::Error) -> Self {
        AenshError::Io(value.to_string())
    }
}

impl From<nix::Error> for AenshError {
    fn from(value: nix::Error) -> Self {
        AenshError::Io(value.to_string())
    }
}
