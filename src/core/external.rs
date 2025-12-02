use std::process::{Command, Stdio};
use crate::core::errors::{AenshError, AenshResult};

/// Executa um comando externo no shell padrão do sistema
pub fn execute_external(cmd: &str, args: &[String]) -> AenshResult<()> {
    let mut command = Command::new(cmd);
    command.args(args)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit());

    let status = command.status()
        .map_err(|e| AenshError::Io(format!("Falha ao executar '{}': {}", cmd, e)))?;

    if !status.success() {
        if let Some(code) = status.code() {
            if code != 0 {
                // Comando falhou, mas não é um erro do Aensh
                return Ok(());
            }
        }
    }

    Ok(())
}

/// Executa comando externo com captura de saída (para piping)
pub fn execute_external_with_capture(cmd: &str, args: &[String]) -> AenshResult<Vec<u8>> {
    let mut command = Command::new(cmd);
    command.args(args)
        .stdin(Stdio::inherit())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit());

    let output = command.output()
        .map_err(|e| AenshError::Io(format!("Falha ao executar '{}': {}", cmd, e)))?;

    Ok(output.stdout)
}
