use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Config {
    pub default_shell: bool,
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

    pub fn set_default_shell(&mut self, enabled: bool) -> std::io::Result<()> {
        self.default_shell = enabled;
        self.save()?;
        
        // Update shell rc files
        Self::update_shell_rc(enabled)?;
        
        Ok(())
    }

    fn update_shell_rc(enabled: bool) -> std::io::Result<()> {
        let home = dirs::home_dir().ok_or_else(|| {
            std::io::Error::new(std::io::ErrorKind::NotFound, "Home directory not found")
        })?;

        let rc_files = vec![
            home.join(".bashrc"),
            home.join(".zshrc"),
        ];

        let aensh_marker = "# Aensh default shell";
        let aensh_block = format!(
            "{}\nif [ -x \"$HOME/.local/bin/aensh\" ] && [ -z \"$AENSH_RUNNING\" ]; then\n    export AENSH_RUNNING=1\n    exec \"$HOME/.local/bin/aensh\"\nfi\n# End Aensh",
            aensh_marker
        );

        for rc_file in rc_files {
            if !rc_file.exists() {
                continue;
            }

            let content = fs::read_to_string(&rc_file)?;
            
            // Remove existing Aensh block if present
            let cleaned: String = content
                .lines()
                .filter(|line| {
                    !line.contains("Aensh default shell") &&
                    !line.contains("AENSH_RUNNING") &&
                    !line.contains("# End Aensh") &&
                    !line.contains("exec \"$HOME/.local/bin/aensh\"")
                })
                .collect::<Vec<_>>()
                .join("\n");

            let new_content = if enabled {
                format!("{}\n\n{}\n", cleaned.trim(), aensh_block)
            } else {
                format!("{}\n", cleaned.trim())
            };

            fs::write(&rc_file, new_content)?;
        }

        Ok(())
    }
}
