use crate::core::command::Command;
use crate::builtins;
use crate::core::errors::{AenshError, AenshResult};

const FORBIDDEN_SEQUENCES: [&str; 4] = ["&&", "||", ";", "$("];

pub fn parse_input(raw: &str) -> AenshResult<Command> {
    let trimmed = raw.trim();

    if trimmed.is_empty() {
        return Err(AenshError::EmptyInput);
    }

    for seq in FORBIDDEN_SEQUENCES {
        if trimmed.contains(seq) {
            return Err(AenshError::Validation(format!(
                "sequência '{}' não é permitida",
                seq
            )));
        }
    }

    if trimmed.contains('`') {
        return Err(AenshError::Validation("uso de crases não é permitido".into()));
    }

    let parts: Vec<String> = trimmed
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();

    if parts.is_empty() {
        return Err(AenshError::EmptyInput);
    }

    let name = parts[0].clone();
    if !builtins::is_supported(&name) {
        return Err(AenshError::InvalidCommand(name));
    }

    Ok(Command::new(name, parts[1..].to_vec()))
}
