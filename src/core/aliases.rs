use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;
use colored::*;

use crate::core::errors::{AenshError, AenshResult};

/// Gerenciador de aliases do Aensh
#[derive(Debug, Default)]
pub struct AliasManager {
    aliases: HashMap<String, String>,
}

impl AliasManager {
    pub fn new() -> Self {
        let mut manager = Self::default();
        manager.load();
        manager
    }

    /// Caminho do arquivo .aenshrc
    pub fn rc_path() -> PathBuf {
        dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(".aenshrc")
    }

    /// Carrega aliases do .aenshrc
    pub fn load(&mut self) {
        let path = Self::rc_path();
        
        if !path.exists() {
            // Create default .aenshrc
            self.create_default_rc();
            return;
        }

        if let Ok(file) = File::open(&path) {
            let reader = BufReader::new(file);
            
            for line in reader.lines().flatten() {
                let trimmed = line.trim();
                
                // Skip comments and empty lines
                if trimmed.is_empty() || trimmed.starts_with('#') {
                    continue;
                }

                // Parse alias: alias name='command'
                if trimmed.starts_with("alias ") {
                    if let Some(alias_def) = trimmed.strip_prefix("alias ") {
                        self.parse_alias(alias_def);
                    }
                }
            }
        }
    }

    /// Parse uma definição de alias
    fn parse_alias(&mut self, def: &str) {
        // Format: name='command' or name="command"
        if let Some(eq_pos) = def.find('=') {
            let name = def[..eq_pos].trim().to_string();
            let mut value = def[eq_pos + 1..].trim().to_string();
            
            // Remove quotes
            if (value.starts_with('\'') && value.ends_with('\'')) ||
               (value.starts_with('"') && value.ends_with('"')) {
                value = value[1..value.len()-1].to_string();
            }
            
            if !name.is_empty() && !value.is_empty() {
                self.aliases.insert(name, value);
            }
        }
    }

    /// Cria .aenshrc padrão
    fn create_default_rc(&self) {
        let path = Self::rc_path();
        
        let default_content = r#"# Aensh Configuration File
# ~/.aenshrc
#
# Este arquivo é carregado quando o Aensh inicia.
# Use para definir aliases personalizados.
#
# Sintaxe de alias:
#   alias nome='comando'
#   alias nome="comando com argumentos"
#
# Exemplos:
#   alias ll='ls -la'
#   alias la='ls -a'
#   alias ..='cd ..'
#   alias ...='cd ../..'
#   alias gs='git status'
#   alias gc='git commit'
#   alias gp='git push'

# Aliases padrão
alias ll='ls -la'
alias la='ls -a'
alias l='ls'
alias ..='cd ..'
alias ...='cd ../..'
alias cls='clear'

# Git aliases
alias gs='git status'
alias ga='git add'
alias gc='git commit'
alias gp='git push'
alias gl='git log --oneline -10'
alias gd='git diff'

# Navegação rápida
alias home='cd ~'
alias docs='cd ~/Documents'
alias dl='cd ~/Downloads'

"#;

        if let Ok(mut file) = File::create(&path) {
            let _ = file.write_all(default_content.as_bytes());
        }
    }

    /// Salva aliases no .aenshrc
    pub fn save(&self) -> AenshResult<()> {
        let path = Self::rc_path();
        
        // Read existing file to preserve comments
        let mut lines: Vec<String> = Vec::new();
        let mut existing_aliases: Vec<String> = Vec::new();
        
        if path.exists() {
            if let Ok(content) = fs::read_to_string(&path) {
                for line in content.lines() {
                    let trimmed = line.trim();
                    if trimmed.starts_with("alias ") {
                        // Extract alias name
                        if let Some(name) = trimmed.strip_prefix("alias ").and_then(|s| s.split('=').next()) {
                            existing_aliases.push(name.trim().to_string());
                        }
                    }
                    lines.push(line.to_string());
                }
            }
        }

        // Add new aliases that don't exist
        for (name, value) in &self.aliases {
            if !existing_aliases.contains(name) {
                lines.push(format!("alias {}='{}'", name, value));
            }
        }

        // Update existing aliases
        for line in &mut lines {
            if line.trim().starts_with("alias ") {
                if let Some(name) = line.trim().strip_prefix("alias ").and_then(|s| s.split('=').next()) {
                    let name = name.trim();
                    if let Some(value) = self.aliases.get(name) {
                        *line = format!("alias {}='{}'", name, value);
                    }
                }
            }
        }

        let mut file = File::create(&path)
            .map_err(|e| AenshError::Io(format!("Falha ao salvar .aenshrc: {}", e)))?;
        
        for line in lines {
            writeln!(file, "{}", line)
                .map_err(|e| AenshError::Io(format!("Falha ao escrever .aenshrc: {}", e)))?;
        }

        Ok(())
    }

    /// Verifica se um comando é um alias
    #[allow(dead_code)]
    pub fn is_alias(&self, name: &str) -> bool {
        self.aliases.contains_key(name)
    }

    /// Obtém o comando real de um alias
    pub fn get(&self, name: &str) -> Option<&String> {
        self.aliases.get(name)
    }

    /// Expande um alias para o comando real
    pub fn expand(&self, input: &str) -> String {
        let parts: Vec<&str> = input.split_whitespace().collect();
        
        if parts.is_empty() {
            return input.to_string();
        }

        let first = parts[0];
        
        if let Some(expanded) = self.aliases.get(first) {
            // Replace alias with expanded command
            if parts.len() > 1 {
                format!("{} {}", expanded, parts[1..].join(" "))
            } else {
                expanded.clone()
            }
        } else {
            input.to_string()
        }
    }

    /// Adiciona um novo alias
    pub fn add(&mut self, name: String, command: String) -> AenshResult<()> {
        self.aliases.insert(name, command);
        self.save()
    }

    /// Remove um alias
    pub fn remove(&mut self, name: &str) -> AenshResult<()> {
        if self.aliases.remove(name).is_none() {
            return Err(AenshError::InvalidCommand(format!("Alias '{}' não encontrado", name)));
        }
        self.save()
    }

    /// Lista todos os aliases
    pub fn list(&self) {
        if self.aliases.is_empty() {
            println!("{} Nenhum alias definido", "ℹ".blue());
            println!("  Edite {} para adicionar aliases", "~/.aenshrc".bright_cyan());
            return;
        }

        println!("{}", "Aliases:".yellow().bold());
        
        let mut sorted: Vec<_> = self.aliases.iter().collect();
        sorted.sort_by(|a, b| a.0.cmp(b.0));
        
        for (name, command) in sorted {
            println!("  {} {} {}", 
                name.bright_green(),
                "→".bright_black(),
                command.bright_white()
            );
        }
    }

    /// Recarrega aliases do arquivo
    pub fn reload(&mut self) {
        self.aliases.clear();
        self.load();
        println!("{} Aliases recarregados de ~/.aenshrc", "✓".green());
    }
}
