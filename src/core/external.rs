use std::process::{Command, Stdio, Child};
use std::io::Write;
use crate::core::errors::{AenshError, AenshResult};
use nix::sys::signal::{self, SigHandler, Signal};

/// Executa um comando externo de forma interativa (para comandos como cmatrix, vim, etc)
pub fn execute_external(cmd: &str, args: &[String]) -> AenshResult<()> {
    let mut command = Command::new(cmd);
    command
        .args(args)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit());

    // Enquanto o comando externo roda, o shell deve ignorar SIGINT (Ctrl+C)
    // para que apenas o processo filho seja encerrado.
    let old_handler = unsafe {
        signal::signal(Signal::SIGINT, SigHandler::SigIgn)
            .map_err(|e| AenshError::Io(format!("Falha ao configurar sinal: {}", e)))?
    };

    let status_result = command.status();

    // Restaura o handler original de SIGINT, independentemente do resultado
    let _ = unsafe { signal::signal(Signal::SIGINT, old_handler) };

    let status = status_result
        .map_err(|e| AenshError::Io(format!("Falha ao executar '{}': {}", cmd, e)))?;

    // Ignore non-zero exit codes (normal for Ctrl+C, etc)
    if !status.success() {
        if let Some(code) = status.code() {
            // 130 = Ctrl+C, 143 = SIGTERM - these are normal
            if code == 130 || code == 143 {
                return Ok(());
            }
        }
    }

    Ok(())
}

/// Executa comando externo com captura de saída (para piping)
pub fn execute_external_with_capture(cmd: &str, args: &[String]) -> AenshResult<Vec<u8>> {
    let output = Command::new(cmd)
        .args(args)
        .stdin(Stdio::inherit())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .output()
        .map_err(|e| AenshError::Io(format!("Falha ao executar '{}': {}", cmd, e)))?;

    Ok(output.stdout)
}

/// Executa comando externo com input de pipe
pub fn execute_external_with_input(cmd: &str, args: &[String], input: &[u8]) -> AenshResult<Vec<u8>> {
    let mut child = Command::new(cmd)
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .spawn()
        .map_err(|e| AenshError::Io(format!("Falha ao executar '{}': {}", cmd, e)))?;

    // Write input to stdin
    if let Some(mut stdin) = child.stdin.take() {
        stdin.write_all(input).ok();
    }

    let output = child.wait_with_output()
        .map_err(|e| AenshError::Io(format!("Falha ao aguardar '{}': {}", cmd, e)))?;

    Ok(output.stdout)
}

/// Executa comando externo com input de pipe e output para stdout
pub fn execute_external_piped_final(cmd: &str, args: &[String], input: &[u8]) -> AenshResult<()> {
    let mut child = Command::new(cmd)
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .map_err(|e| AenshError::Io(format!("Falha ao executar '{}': {}", cmd, e)))?;

    // Write input to stdin
    if let Some(mut stdin) = child.stdin.take() {
        stdin.write_all(input).ok();
    }

    child.wait()
        .map_err(|e| AenshError::Io(format!("Falha ao aguardar '{}': {}", cmd, e)))?;

    Ok(())
}

/// Spawna um comando externo para pipeline (retorna Child para controle)
#[allow(dead_code)]
pub fn spawn_external_for_pipe(cmd: &str, args: &[String], input: Option<Stdio>) -> AenshResult<Child> {
    let stdin = input.unwrap_or(Stdio::inherit());
    
    Command::new(cmd)
        .args(args)
        .stdin(stdin)
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .spawn()
        .map_err(|e| AenshError::Io(format!("Falha ao executar '{}': {}", cmd, e)))
}
