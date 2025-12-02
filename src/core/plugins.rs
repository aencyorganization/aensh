use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::PathBuf;
use std::process::{Command, Stdio};
use serde::{Deserialize, Serialize};
use colored::*;

use crate::core::errors::{AenshError, AenshResult};

/// List of shell commands that are blocked (native shell commands)
const BLOCKED_COMMANDS: &[&str] = &[
    // Navigation/basic shell
    "cd", "pwd", "ls", "dir",
    // File operations
    "cat", "cp", "mv", "rm", "mkdir", "rmdir", "touch",
    // Search
    "grep", "find", "locate", "which", "whereis",
    // Text processing
    "sed", "awk", "cut", "sort", "uniq", "wc", "head", "tail",
    // System
    "ps", "top", "htop", "kill", "killall", "bg", "fg", "jobs",
    // Shell builtins
    "echo", "printf", "read", "export", "unset", "alias", "unalias",
    "source", ".", "exec", "eval", "set", "shift",
    // Dangerous
    "sudo", "su", "chmod", "chown", "chgrp",
    // Network
    "curl", "wget", "ssh", "scp", "rsync", "ping", "netstat", "ifconfig", "ip",
    // Package managers
    "apt", "apt-get", "yum", "dnf", "pacman", "brew", "snap", "flatpak",
    // Shells
    "bash", "sh", "zsh", "fish", "dash", "csh", "tcsh", "ksh",
];

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Plugin {
    pub name: String,
    pub command: String,
    pub description: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PluginManager {
    plugins: HashMap<String, Plugin>,
}

impl PluginManager {
    pub fn new() -> Self {
        let mut manager = Self::load().unwrap_or_default();
        manager.load_from_plugins_dir();
        manager
    }

    fn plugins_path() -> PathBuf {
        dirs::config_dir()
            .unwrap_or_else(|| dirs::home_dir().unwrap_or_else(|| PathBuf::from(".")))
            .join("aensh")
            .join("plugins.json")
    }

    fn plugins_dir() -> PathBuf {
        dirs::config_dir()
            .unwrap_or_else(|| dirs::home_dir().unwrap_or_else(|| PathBuf::from(".")))
            .join("aensh")
            .join("plugins")
    }

    fn load() -> Option<Self> {
        let path = Self::plugins_path();
        let mut file = File::open(&path).ok()?;
        let mut contents = String::new();
        file.read_to_string(&mut contents).ok()?;
        serde_json::from_str(&contents).ok()
    }

    fn load_from_plugins_dir(&mut self) {
        let plugins_dir = Self::plugins_dir();
        
        if !plugins_dir.exists() {
            let _ = fs::create_dir_all(&plugins_dir);
            return;
        }

        if let Ok(entries) = fs::read_dir(&plugins_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() {
                    if let Some(name) = path.file_stem().and_then(|s| s.to_str()) {
                        // Check if it's executable
                        #[cfg(unix)]
                        {
                            use std::os::unix::fs::PermissionsExt;
                            if let Ok(metadata) = fs::metadata(&path) {
                                if metadata.permissions().mode() & 0o111 != 0 {
                                    let plugin = Plugin {
                                        name: name.to_string(),
                                        command: path.to_string_lossy().to_string(),
                                        description: format!("Plugin: {}", name),
                                    };
                                    self.plugins.insert(name.to_string(), plugin);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn save(&self) -> std::io::Result<()> {
        let path = Self::plugins_path();
        
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        
        let mut file = File::create(&path)?;
        let json = serde_json::to_string_pretty(self)?;
        file.write_all(json.as_bytes())?;
        
        Ok(())
    }

    pub fn is_blocked(name: &str) -> bool {
        BLOCKED_COMMANDS.contains(&name)
    }

    pub fn register(&mut self, name: String, command: String, description: String) -> AenshResult<()> {
        if Self::is_blocked(&name) {
            return Err(AenshError::Validation(format!(
                "comando '{}' é bloqueado (comando nativo do shell)",
                name
            )));
        }

        let plugin = Plugin {
            name: name.clone(),
            command,
            description,
        };

        self.plugins.insert(name, plugin);
        self.save().map_err(|e| AenshError::Io(e.to_string()))?;
        
        Ok(())
    }

    pub fn unregister(&mut self, name: &str) -> AenshResult<()> {
        if self.plugins.remove(name).is_none() {
            return Err(AenshError::InvalidCommand(name.to_string()));
        }
        
        self.save().map_err(|e| AenshError::Io(e.to_string()))?;
        Ok(())
    }

    pub fn get(&self, name: &str) -> Option<&Plugin> {
        self.plugins.get(name)
    }

    pub fn list(&self) -> Vec<&Plugin> {
        self.plugins.values().collect()
    }

    pub fn execute(&self, name: &str, args: &[String]) -> AenshResult<()> {
        let plugin = self.get(name).ok_or_else(|| {
            AenshError::InvalidCommand(name.to_string())
        })?;

        let output = Command::new(&plugin.command)
            .args(args)
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .output()
            .map_err(|e| AenshError::Io(format!("Falha ao executar plugin: {}", e)))?;

        if !output.status.success() {
            if let Some(code) = output.status.code() {
                eprintln!("{} Plugin '{}' saiu com código {}", "⚠".yellow(), name, code);
            }
        }

        Ok(())
    }

    pub fn execute_with_pipe(&self, name: &str, args: &[String], input: Option<Vec<u8>>) -> AenshResult<Vec<u8>> {
        let plugin = self.get(name).ok_or_else(|| {
            AenshError::InvalidCommand(name.to_string())
        })?;

        let mut child = Command::new(&plugin.command)
            .args(args)
            .stdin(if input.is_some() { Stdio::piped() } else { Stdio::inherit() })
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .spawn()
            .map_err(|e| AenshError::Io(format!("Falha ao executar plugin: {}", e)))?;

        if let Some(input_data) = input {
            if let Some(mut stdin) = child.stdin.take() {
                let _ = stdin.write_all(&input_data);
            }
        }

        let output = child.wait_with_output()
            .map_err(|e| AenshError::Io(format!("Falha ao aguardar plugin: {}", e)))?;

        Ok(output.stdout)
    }
}
