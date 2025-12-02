use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;

pub struct History {
    entries: Vec<String>,
    position: usize,
    max_size: usize,
    file_path: PathBuf,
}

impl History {
    pub fn new() -> Self {
        let file_path = Self::get_history_path();
        let entries = Self::load_from_file(&file_path);
        let len = entries.len();
        
        Self {
            entries,
            position: len,
            max_size: 1000,
            file_path,
        }
    }

    fn get_history_path() -> PathBuf {
        dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(".aensh_history")
    }

    fn load_from_file(path: &PathBuf) -> Vec<String> {
        if let Ok(file) = File::open(path) {
            BufReader::new(file)
                .lines()
                .filter_map(|l| l.ok())
                .filter(|l| !l.is_empty())
                .collect()
        } else {
            Vec::new()
        }
    }

    pub fn add(&mut self, command: &str) {
        let trimmed = command.trim();
        if trimmed.is_empty() {
            return;
        }

        // Don't add duplicates consecutively
        if self.entries.last().map(|s| s.as_str()) != Some(trimmed) {
            self.entries.push(trimmed.to_string());
            
            // Trim if exceeds max size
            if self.entries.len() > self.max_size {
                self.entries.remove(0);
            }
            
            self.save_to_file();
        }
        
        self.position = self.entries.len();
    }

    fn save_to_file(&self) {
        if let Ok(mut file) = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&self.file_path)
        {
            for entry in &self.entries {
                let _ = writeln!(file, "{}", entry);
            }
        }
    }

    pub fn previous(&mut self) -> Option<&str> {
        if self.entries.is_empty() {
            return None;
        }
        
        if self.position > 0 {
            self.position -= 1;
        }
        
        self.entries.get(self.position).map(|s| s.as_str())
    }

    pub fn next(&mut self) -> Option<&str> {
        if self.entries.is_empty() {
            return None;
        }
        
        if self.position < self.entries.len() {
            self.position += 1;
        }
        
        if self.position >= self.entries.len() {
            None // Return None to clear the line (new command)
        } else {
            self.entries.get(self.position).map(|s| s.as_str())
        }
    }

    pub fn reset_position(&mut self) {
        self.position = self.entries.len();
    }
}

impl Default for History {
    fn default() -> Self {
        Self::new()
    }
}
