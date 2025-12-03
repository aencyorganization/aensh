use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum PreviousShell {
    Bash,
    Zsh,
    Fish,
}

impl PreviousShell {
    pub fn rc_file(&self) -> &'static str {
        match self {
            PreviousShell::Bash => ".bashrc",
            PreviousShell::Zsh => ".zshrc",
            PreviousShell::Fish => ".config/fish/config.fish",
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            PreviousShell::Bash => "Bash",
            PreviousShell::Zsh => "Zsh",
            PreviousShell::Fish => "Fish",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "bash" | "1" => Some(PreviousShell::Bash),
            "zsh" | "2" => Some(PreviousShell::Zsh),
            "fish" | "3" => Some(PreviousShell::Fish),
            _ => None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub default_shell: bool,
    pub previous_shell: Option<PreviousShell>,
    pub setup_completed: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            default_shell: false,
            previous_shell: None,
            setup_completed: false,
        }
    }
}

impl Config {
    pub fn load() -> Self {
        let path = Self::config_path();
        
        if let Ok(mut file) = File::open(&path) {
            let mut contents = String::new();
            if file.read_to_string(&mut contents).is_ok() {
                if let Ok(config) = serde_json::from_str(&contents) {
                    return config;
                }
            }
        }
        
        Self::default()
    }

    pub fn save(&self) -> std::io::Result<()> {
        let path = Self::config_path();
        
        // Create config directory if it doesn't exist
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        
        let mut file = File::create(&path)?;
        let json = serde_json::to_string_pretty(self)?;
        file.write_all(json.as_bytes())?;
        
        Ok(())
    }

    pub fn config_path() -> PathBuf {
        dirs::config_dir()
            .unwrap_or_else(|| dirs::home_dir().unwrap_or_else(|| PathBuf::from(".")))
            .join("aensh")
            .join("config.json")
    }

    #[allow(dead_code)]
    pub fn config_dir() -> PathBuf {
        dirs::config_dir()
            .unwrap_or_else(|| dirs::home_dir().unwrap_or_else(|| PathBuf::from(".")))
            .join("aensh")
    }

    pub fn needs_setup(&self) -> bool {
        !self.setup_completed || self.previous_shell.is_none()
    }

    pub fn set_previous_shell(&mut self, shell: PreviousShell) -> std::io::Result<()> {
        self.previous_shell = Some(shell);
        self.setup_completed = true;
        self.save()
    }

    pub fn set_default_shell(&mut self, enabled: bool) -> std::io::Result<()> {
        self.default_shell = enabled;
        self.save()?;
        
        // Update shell rc file based on previous shell
        if let Some(shell) = self.previous_shell {
            Self::update_shell_rc(shell, enabled)?;
        }
        
        Ok(())
    }

    fn update_shell_rc(shell: PreviousShell, enabled: bool) -> std::io::Result<()> {
        let home = dirs::home_dir().ok_or_else(|| {
            std::io::Error::new(std::io::ErrorKind::NotFound, "Home directory not found")
        })?;

        let rc_file = home.join(shell.rc_file());

        // Create parent directories if needed (for Fish)
        if let Some(parent) = rc_file.parent() {
            fs::create_dir_all(parent)?;
        }

        // Create file if it doesn't exist
        if !rc_file.exists() {
            File::create(&rc_file)?;
        }

        let content = fs::read_to_string(&rc_file)?;
        
        // Remove existing Aensh block if present
        let cleaned: String = content
            .lines()
            .filter(|line| {
                !line.contains("Aensh default shell") &&
                !line.contains("AENSH_RUNNING") &&
                !line.contains("# End Aensh") &&
                !line.contains("exec \"$HOME/.local/bin/aensh\"") &&
                !line.contains("exec $HOME/.local/bin/aensh")
            })
            .collect::<Vec<_>>()
            .join("\n");

        let aensh_block = match shell {
            PreviousShell::Fish => {
                "# Aensh default shell\nif test -x $HOME/.local/bin/aensh; and not set -q AENSH_RUNNING\n    set -gx AENSH_RUNNING 1\n    exec $HOME/.local/bin/aensh\nend\n# End Aensh".to_string()
            }
            _ => {
                "# Aensh default shell\nif [ -x \"$HOME/.local/bin/aensh\" ] && [ -z \"$AENSH_RUNNING\" ]; then\n    export AENSH_RUNNING=1\n    exec \"$HOME/.local/bin/aensh\"\nfi\n# End Aensh".to_string()
            }
        };

        let new_content = if enabled {
            format!("{}\n\n{}\n", cleaned.trim(), aensh_block)
        } else {
            format!("{}\n", cleaned.trim())
        };

        fs::write(&rc_file, new_content)?;

        Ok(())
    }
}
