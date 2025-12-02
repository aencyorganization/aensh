use std::io::{self, Write, stdout};
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    terminal::{disable_raw_mode, enable_raw_mode},
    cursor,
    execute,
};

use super::history::History;

pub struct ReadLine {
    history: History,
}

impl ReadLine {
    pub fn new() -> Self {
        Self {
            history: History::new(),
        }
    }

    pub fn read_line(&mut self, prompt: &str) -> io::Result<Option<String>> {
        print!("{}", prompt);
        stdout().flush()?;

        enable_raw_mode()?;
        let result = self.read_line_raw(prompt);
        disable_raw_mode()?;

        println!(); // Move to next line after input
        
        result
    }

    fn read_line_raw(&mut self, prompt: &str) -> io::Result<Option<String>> {
        let mut buffer = String::new();
        let mut cursor_pos: usize = 0;
        let prompt_len = Self::visible_len(prompt);

        loop {
            if let Event::Key(key_event) = event::read()? {
                match key_event {
                    KeyEvent { code: KeyCode::Enter, .. } => {
                        if !buffer.trim().is_empty() {
                            self.history.add(&buffer);
                        }
                        self.history.reset_position();
                        return Ok(Some(buffer));
                    }
                    
                    KeyEvent { code: KeyCode::Char('c'), modifiers: KeyModifiers::CONTROL, .. } => {
                        print!("^C");
                        stdout().flush()?;
                        return Ok(Some(String::new()));
                    }
                    
                    KeyEvent { code: KeyCode::Char('d'), modifiers: KeyModifiers::CONTROL, .. } => {
                        if buffer.is_empty() {
                            return Ok(None); // EOF
                        }
                    }
                    
                    KeyEvent { code: KeyCode::Backspace, .. } => {
                        if cursor_pos > 0 {
                            buffer.remove(cursor_pos - 1);
                            cursor_pos -= 1;
                            self.redraw_line(prompt, &buffer, cursor_pos, prompt_len)?;
                        }
                    }
                    
                    KeyEvent { code: KeyCode::Delete, .. } => {
                        if cursor_pos < buffer.len() {
                            buffer.remove(cursor_pos);
                            self.redraw_line(prompt, &buffer, cursor_pos, prompt_len)?;
                        }
                    }
                    
                    KeyEvent { code: KeyCode::Left, .. } => {
                        if cursor_pos > 0 {
                            cursor_pos -= 1;
                            execute!(stdout(), cursor::MoveLeft(1))?;
                        }
                    }
                    
                    KeyEvent { code: KeyCode::Right, .. } => {
                        if cursor_pos < buffer.len() {
                            cursor_pos += 1;
                            execute!(stdout(), cursor::MoveRight(1))?;
                        }
                    }
                    
                    KeyEvent { code: KeyCode::Up, .. } => {
                        if let Some(prev) = self.history.previous() {
                            buffer = prev.to_string();
                            cursor_pos = buffer.len();
                            self.redraw_line(prompt, &buffer, cursor_pos, prompt_len)?;
                        }
                    }
                    
                    KeyEvent { code: KeyCode::Down, .. } => {
                        if let Some(next) = self.history.next() {
                            buffer = next.to_string();
                        } else {
                            buffer.clear();
                        }
                        cursor_pos = buffer.len();
                        self.redraw_line(prompt, &buffer, cursor_pos, prompt_len)?;
                    }
                    
                    KeyEvent { code: KeyCode::Home, .. } => {
                        cursor_pos = 0;
                        execute!(stdout(), cursor::MoveToColumn(prompt_len as u16))?;
                    }
                    
                    KeyEvent { code: KeyCode::End, .. } => {
                        cursor_pos = buffer.len();
                        execute!(stdout(), cursor::MoveToColumn((prompt_len + buffer.len()) as u16))?;
                    }
                    
                    KeyEvent { code: KeyCode::Char('a'), modifiers: KeyModifiers::CONTROL, .. } => {
                        cursor_pos = 0;
                        execute!(stdout(), cursor::MoveToColumn(prompt_len as u16))?;
                    }
                    
                    KeyEvent { code: KeyCode::Char('e'), modifiers: KeyModifiers::CONTROL, .. } => {
                        cursor_pos = buffer.len();
                        execute!(stdout(), cursor::MoveToColumn((prompt_len + buffer.len()) as u16))?;
                    }
                    
                    KeyEvent { code: KeyCode::Char('u'), modifiers: KeyModifiers::CONTROL, .. } => {
                        buffer.clear();
                        cursor_pos = 0;
                        self.redraw_line(prompt, &buffer, cursor_pos, prompt_len)?;
                    }
                    
                    KeyEvent { code: KeyCode::Char('w'), modifiers: KeyModifiers::CONTROL, .. } => {
                        // Delete word backwards
                        while cursor_pos > 0 && buffer.chars().nth(cursor_pos - 1) == Some(' ') {
                            buffer.remove(cursor_pos - 1);
                            cursor_pos -= 1;
                        }
                        while cursor_pos > 0 && buffer.chars().nth(cursor_pos - 1) != Some(' ') {
                            buffer.remove(cursor_pos - 1);
                            cursor_pos -= 1;
                        }
                        self.redraw_line(prompt, &buffer, cursor_pos, prompt_len)?;
                    }
                    
                    KeyEvent { code: KeyCode::Char(c), modifiers: KeyModifiers::NONE | KeyModifiers::SHIFT, .. } => {
                        buffer.insert(cursor_pos, c);
                        cursor_pos += 1;
                        
                        if cursor_pos == buffer.len() {
                            print!("{}", c);
                            stdout().flush()?;
                        } else {
                            self.redraw_line(prompt, &buffer, cursor_pos, prompt_len)?;
                        }
                    }
                    
                    _ => {}
                }
            }
        }
    }

    fn redraw_line(&self, _prompt: &str, buffer: &str, cursor_pos: usize, prompt_len: usize) -> io::Result<()> {
        // Move to start of input area
        execute!(stdout(), cursor::MoveToColumn(prompt_len as u16))?;
        // Clear from cursor to end of line
        print!("\x1b[K");
        // Print buffer
        print!("{}", buffer);
        // Move cursor to correct position
        execute!(stdout(), cursor::MoveToColumn((prompt_len + cursor_pos) as u16))?;
        stdout().flush()?;
        Ok(())
    }

    fn visible_len(s: &str) -> usize {
        // Strip ANSI escape codes to get visible length
        let mut len = 0;
        let mut in_escape = false;
        
        for c in s.chars() {
            if c == '\x1b' {
                in_escape = true;
            } else if in_escape {
                if c == 'm' {
                    in_escape = false;
                }
            } else {
                len += 1;
            }
        }
        
        len
    }
}

impl Default for ReadLine {
    fn default() -> Self {
        Self::new()
    }
}
