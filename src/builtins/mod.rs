use crate::core::command::Command;
use crate::core::errors::AenshResult;

pub mod navigation;
pub mod filesystem;
pub mod system;
pub mod shell;

pub const SUPPORTED_COMMANDS: &[(&str, &str)] = &[
    // Shell
    ("help", "Mostra a lista de comandos disponíveis"),
    ("exit", "Encerra o shell"),
    ("quit", "Encerra o shell (alias para exit)"),
    ("plugin", "Gerencia plugins externos"),
    
    // Navigation
    ("cd", "Altera o diretório atual"),
    ("pwd", "Mostra o diretório atual"),
    
    // Filesystem
    ("ls", "Lista arquivos e diretórios"),
    ("cat", "Exibe o conteúdo de arquivos"),
    ("mkdir", "Cria um novo diretório"),
    ("touch", "Cria um arquivo vazio ou atualiza timestamp"),
    ("rm", "Remove arquivos ou diretórios"),
    ("cp", "Copia arquivos ou diretórios"),
    ("mv", "Move ou renomeia arquivos"),
    ("find", "Busca arquivos em diretórios"),
    ("grep", "Busca padrões em arquivos"),
    ("tree", "Mostra estrutura de diretórios em árvore"),
    
    // System
    ("echo", "Exibe texto na tela"),
    ("clear", "Limpa a tela"),
    ("info", "Mostra informações do Aensh"),
    ("whoami", "Mostra o usuário atual"),
    ("date", "Mostra a data e hora atual"),
    ("stat", "Mostra informações de arquivo/diretório"),
];

pub fn is_supported(name: &str) -> bool {
    SUPPORTED_COMMANDS.iter().any(|(cmd, _)| cmd == &name)
}

pub fn dispatch(command: &Command) -> AenshResult<()> {
    match command.name.as_str() {
        // Shell
        "help" => shell::help::run(&command.args),
        "exit" | "quit" => shell::exit::run(&command.args),
        "plugin" => shell::plugin::run(&command.args),
        
        // Navigation
        "cd" => navigation::cd::run(&command.args),
        "pwd" => navigation::pwd::run(&command.args),
        
        // Filesystem
        "ls" => filesystem::ls::run(&command.args),
        "cat" => filesystem::cat::run(&command.args),
        "mkdir" => filesystem::mkdir::run(&command.args),
        "touch" => filesystem::touch::run(&command.args),
        "rm" => filesystem::rm::run(&command.args),
        "cp" => filesystem::cp::run(&command.args),
        "mv" => filesystem::mv::run(&command.args),
        "find" => filesystem::find::run(&command.args),
        "grep" => filesystem::grep::run(&command.args),
        "tree" => filesystem::tree::run(&command.args),
        
        // System
        "echo" => system::echo::run(&command.args),
        "clear" => system::clear::run(&command.args),
        "info" => system::info::run(&command.args),
        "whoami" => system::whoami::run(&command.args),
        "date" => system::date::run(&command.args),
        "stat" => system::stat::run(&command.args),
        
        other => Err(crate::core::errors::AenshError::InvalidCommand(other.to_string())),
    }
}
