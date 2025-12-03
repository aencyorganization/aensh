use crate::core::command::Command;
use crate::core::errors::{AenshError, AenshResult};
use crate::builtins;
use crate::core::plugins::PluginManager;
use std::io::Write;

#[derive(Debug, Clone)]
pub enum PipelineSegment {
    Builtin(Command),
    Plugin(Command),
    External(Command),
}

#[derive(Debug)]
pub struct Pipeline {
    pub segments: Vec<PipelineSegment>,
}

impl Pipeline {
    pub fn new() -> Self {
        Self { segments: Vec::new() }
    }

    pub fn add(&mut self, segment: PipelineSegment) {
        self.segments.push(segment);
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum ChainOperator {
    And, // &&
    Pipe, // |
}

#[derive(Debug)]
pub struct CommandChain {
    pub commands: Vec<(Pipeline, Option<ChainOperator>)>,
}

impl CommandChain {
    pub fn new() -> Self {
        Self { commands: Vec::new() }
    }
}

pub fn parse_command_chain(input: &str, plugin_manager: &PluginManager) -> AenshResult<CommandChain> {
    let mut chain = CommandChain::new();
    
    // Split by && first
    let and_parts: Vec<&str> = input.split("&&").collect();
    
    for (i, and_part) in and_parts.iter().enumerate() {
        let trimmed = and_part.trim();
        if trimmed.is_empty() {
            continue;
        }
        
        // Parse pipeline (split by |)
        let pipeline = parse_pipeline(trimmed, plugin_manager)?;
        
        let operator = if i < and_parts.len() - 1 {
            Some(ChainOperator::And)
        } else {
            None
        };
        
        chain.commands.push((pipeline, operator));
    }
    
    if chain.commands.is_empty() {
        return Err(AenshError::EmptyInput);
    }
    
    Ok(chain)
}

fn parse_pipeline(input: &str, plugin_manager: &PluginManager) -> AenshResult<Pipeline> {
    let mut pipeline = Pipeline::new();
    
    // Split by | (but not ||)
    let pipe_parts: Vec<&str> = split_by_pipe(input);
    
    for part in pipe_parts {
        let trimmed = part.trim();
        if trimmed.is_empty() {
            continue;
        }
        
        let segment = parse_single_command(trimmed, plugin_manager)?;
        pipeline.add(segment);
    }
    
    if pipeline.segments.is_empty() {
        return Err(AenshError::EmptyInput);
    }
    
    Ok(pipeline)
}

fn split_by_pipe(input: &str) -> Vec<&str> {
    let mut parts = Vec::new();
    let mut start = 0;
    let chars: Vec<char> = input.chars().collect();
    let mut i = 0;
    
    while i < chars.len() {
        if chars[i] == '|' {
            // Check it's not ||
            if i + 1 < chars.len() && chars[i + 1] == '|' {
                i += 2;
                continue;
            }
            parts.push(&input[start..i]);
            start = i + 1;
        }
        i += 1;
    }
    
    parts.push(&input[start..]);
    parts
}

fn parse_single_command(input: &str, plugin_manager: &PluginManager) -> AenshResult<PipelineSegment> {
    // Validate no dangerous sequences
    if input.contains('`') {
        return Err(AenshError::Validation("uso de crases não é permitido".into()));
    }
    
    if input.contains("$(") {
        return Err(AenshError::Validation("substituição de comando não é permitida".into()));
    }
    
    let parts: Vec<String> = input
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();
    
    if parts.is_empty() {
        return Err(AenshError::EmptyInput);
    }
    
    let name = parts[0].clone();
    let args = parts[1..].to_vec();
    let command = Command::new(name.clone(), args);
    
    // Check if it's a builtin
    if builtins::is_supported(&name) {
        return Ok(PipelineSegment::Builtin(command));
    }
    
    // Check if it's a plugin
    if plugin_manager.get(&name).is_some() {
        return Ok(PipelineSegment::Plugin(command));
    }
    
    // Check if it's a blocked command
    if PluginManager::is_blocked(&name) {
        return Err(AenshError::Validation(format!(
            "'{}' é um comando nativo do shell e não pode ser executado",
            name
        )));
    }
    
    // If not builtin, plugin, or blocked - execute as external command
    Ok(PipelineSegment::External(command))
}

pub fn execute_chain(chain: CommandChain, plugin_manager: &PluginManager) -> AenshResult<()> {
    for (pipeline, operator) in chain.commands {
        let result = execute_pipeline(pipeline, plugin_manager);
        
        match (&result, &operator) {
            (Err(_), Some(ChainOperator::And)) => {
                // Stop on error for &&
                return result;
            }
            (Err(e), None) => {
                return Err(e.clone());
            }
            _ => {}
        }
    }
    
    Ok(())
}

fn execute_pipeline(pipeline: Pipeline, plugin_manager: &PluginManager) -> AenshResult<()> {
    if pipeline.segments.len() == 1 {
        // Single command, no piping
        return execute_segment(&pipeline.segments[0], None, plugin_manager, true);
    }
    
    // Multiple commands with piping
    let mut prev_output: Option<Vec<u8>> = None;
    
    for (i, segment) in pipeline.segments.iter().enumerate() {
        let is_last = i == pipeline.segments.len() - 1;
        
        match segment {
            PipelineSegment::Builtin(cmd) => {
                // For builtins in a pipeline, we need to capture output
                let output = execute_builtin_with_capture(cmd, prev_output.clone())?;
                
                if is_last {
                    // Print final output
                    print!("{}", String::from_utf8_lossy(&output));
                    std::io::stdout().flush().ok();
                } else {
                    prev_output = Some(output);
                }
            }
            PipelineSegment::Plugin(cmd) => {
                let output = plugin_manager.execute_with_pipe(&cmd.name, &cmd.args, prev_output.clone())?;
                
                if is_last {
                    print!("{}", String::from_utf8_lossy(&output));
                    std::io::stdout().flush().ok();
                } else {
                    prev_output = Some(output);
                }
            }
            PipelineSegment::External(cmd) => {
                if let Some(input) = prev_output.take() {
                    // Has input from previous command
                    if is_last {
                        // Last command - output to stdout
                        crate::core::external::execute_external_piped_final(&cmd.name, &cmd.args, &input)?;
                    } else {
                        // Middle command - capture output
                        let output = crate::core::external::execute_external_with_input(&cmd.name, &cmd.args, &input)?;
                        prev_output = Some(output);
                    }
                } else {
                    // First command in pipeline
                    let output = crate::core::external::execute_external_with_capture(&cmd.name, &cmd.args)?;
                    if is_last {
                        print!("{}", String::from_utf8_lossy(&output));
                        std::io::stdout().flush().ok();
                    } else {
                        prev_output = Some(output);
                    }
                }
            }
        }
    }
    
    Ok(())
}

fn execute_segment(
    segment: &PipelineSegment,
    _input: Option<Vec<u8>>,
    plugin_manager: &PluginManager,
    _is_last: bool,
) -> AenshResult<()> {
    match segment {
        PipelineSegment::Builtin(cmd) => {
            builtins::dispatch(cmd)
        }
        PipelineSegment::Plugin(cmd) => {
            plugin_manager.execute(&cmd.name, &cmd.args)
        }
        PipelineSegment::External(cmd) => {
            crate::core::external::execute_external(&cmd.name, &cmd.args)
        }
    }
}

fn execute_builtin_with_capture(cmd: &Command, _input: Option<Vec<u8>>) -> AenshResult<Vec<u8>> {
    // For builtins that support piping, capture their output
    let mut output = Vec::new();
    
    match cmd.name.as_str() {
        "echo" => {
            let text = cmd.args.join(" ");
            output.extend_from_slice(text.as_bytes());
            output.push(b'\n');
        }
        "pwd" => {
            if let Ok(cwd) = std::env::current_dir() {
                output.extend_from_slice(cwd.to_string_lossy().as_bytes());
                output.push(b'\n');
            }
        }
        "cat" => {
            for arg in &cmd.args {
                if let Ok(content) = std::fs::read(arg) {
                    output.extend_from_slice(&content);
                }
            }
        }
        "ls" => {
            // Use system ls for piping since our builtin doesn't support capture
            let dir = cmd.args.first().map(|s| s.as_str()).unwrap_or(".");
            if let Ok(entries) = std::fs::read_dir(dir) {
                for entry in entries.flatten() {
                    if let Some(name) = entry.file_name().to_str() {
                        output.extend_from_slice(name.as_bytes());
                        output.push(b'\n');
                    }
                }
            }
        }
        "find" => {
            // Simple find implementation for piping
            let pattern = cmd.args.first().map(|s| s.as_str()).unwrap_or("*");
            fn find_files(dir: &std::path::Path, pattern: &str, output: &mut Vec<u8>) {
                if let Ok(entries) = std::fs::read_dir(dir) {
                    for entry in entries.flatten() {
                        let path = entry.path();
                        let name = entry.file_name().to_string_lossy().to_string();
                        if name.contains(pattern) || pattern == "*" {
                            output.extend_from_slice(path.to_string_lossy().as_bytes());
                            output.push(b'\n');
                        }
                        if path.is_dir() {
                            find_files(&path, pattern, output);
                        }
                    }
                }
            }
            find_files(std::path::Path::new("."), pattern, &mut output);
        }
        "whoami" => {
            if let Ok(user) = std::env::var("USER") {
                output.extend_from_slice(user.as_bytes());
                output.push(b'\n');
            }
        }
        "date" => {
            use std::process::Command as StdCommand;
            if let Ok(out) = StdCommand::new("date").output() {
                output.extend_from_slice(&out.stdout);
            }
        }
        _ => {
            // For other builtins, just execute normally (no capture)
            builtins::dispatch(cmd)?;
        }
    }
    
    Ok(output)
}

impl Clone for AenshError {
    fn clone(&self) -> Self {
        match self {
            AenshError::EmptyInput => AenshError::EmptyInput,
            AenshError::Validation(s) => AenshError::Validation(s.clone()),
            AenshError::InvalidCommand(s) => AenshError::InvalidCommand(s.clone()),
            AenshError::Io(s) => AenshError::Io(s.clone()),
        }
    }
}
