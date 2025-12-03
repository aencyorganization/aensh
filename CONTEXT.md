# 📋 Aensh - Contexto Completo do Projeto

Este documento contém todas as informações necessárias para entender, desenvolver e manter o Aensh.

---

## 📌 Informações Gerais

| Campo | Valor |
|-------|-------|
| **Nome** | Aensh (Aency Shell) |
| **Versão** | 0.1.0 |
| **Linguagem** | Rust |
| **Edição Rust** | 2021 |
| **MSRV** | 1.70+ |
| **Licença** | MIT |
| **Autor** | Aency Organization |
| **GitHub** | aencyorganization |
| **Repositório** | https://github.com/aencyorganization/aensh |

---

## 📁 Estrutura do Projeto

```
aensh/
├── Cargo.toml                 # Dependências e metadata do projeto
├── Cargo.lock                 # Lock de versões exatas
├── README.md                  # Documentação principal (1500+ linhas)
├── CONTEXT.md                 # Este arquivo - contexto do projeto
├── LICENSE                    # Licença MIT
├── install.sh                 # Script de instalação via curl
│
├── src/
│   ├── main.rs                # Entry point, CLI args, main loop
│   │
│   ├── core/                  # Módulos principais do shell
│   │   ├── mod.rs             # Exports do módulo core
│   │   ├── aliases.rs         # Sistema de aliases (~/.aenshrc)
│   │   ├── banner.rs          # Banner ASCII de inicialização
│   │   ├── command.rs         # Struct Command (nome + args)
│   │   ├── config.rs          # Configuração persistente (config.json)
│   │   ├── errors.rs          # AenshError, AenshResult
│   │   ├── external.rs        # Execução de comandos externos
│   │   ├── history.rs         # Histórico de comandos persistente
│   │   ├── pipeline.rs        # Parser e executor de pipelines
│   │   ├── plugins.rs         # Sistema de plugins
│   │   ├── prompt.rs          # Geração do prompt colorido
│   │   ├── readline.rs        # Input com navegação por setas
│   │   └── setup.rs           # Setup inicial interativo
│   │
│   └── builtins/              # Comandos built-in
│       ├── mod.rs             # Dispatcher principal + SUPPORTED_COMMANDS
│       │
│       ├── shell/             # Comandos de shell
│       │   ├── mod.rs
│       │   ├── alias.rs       # Gerenciamento de aliases
│       │   ├── help.rs        # Ajuda
│       │   ├── exit.rs        # Sair do shell
│       │   └── plugin.rs      # Gerenciamento de plugins
│       │
│       ├── navigation/        # Comandos de navegação
│       │   ├── mod.rs
│       │   ├── cd.rs          # Mudar diretório
│       │   └── pwd.rs         # Mostrar diretório atual
│       │
│       ├── filesystem/        # Comandos de filesystem
│       │   ├── mod.rs
│       │   ├── ls.rs          # Listar arquivos
│       │   ├── cat.rs         # Mostrar conteúdo
│       │   ├── mkdir.rs       # Criar diretório
│       │   ├── touch.rs       # Criar arquivo
│       │   ├── rm.rs          # Remover
│       │   ├── cp.rs          # Copiar
│       │   ├── mv.rs          # Mover
│       │   ├── find.rs        # Buscar arquivos
│       │   ├── grep.rs        # Buscar texto
│       │   ├── tree.rs        # Árvore de diretórios
│       │   ├── head.rs        # Primeiras linhas
│       │   ├── tail.rs        # Últimas linhas
│       │   └── wc.rs          # Contar linhas/palavras
│       │
│       └── system/            # Comandos de sistema
│           ├── mod.rs
│           ├── echo.rs        # Imprimir texto
│           ├── clear.rs       # Limpar tela
│           ├── info.rs        # Info do Aensh
│           ├── whoami.rs      # Usuário atual
│           ├── date.rs        # Data/hora
│           ├── stat.rs        # Info de arquivo
│           ├── env.rs         # Variáveis de ambiente
│           ├── export.rs      # Definir variável
│           ├── unset.rs       # Remover variável
│           ├── history.rs     # Histórico
│           ├── which.rs       # Caminho de comando
│           └── type_cmd.rs    # Tipo de comando
│
├── docs/                      # Documentação adicional
│   ├── QUICK_START.md
│   ├── USAGE.md
│   ├── EXTERNAL_COMMANDS.md
│   └── ...
│
└── target/                    # Build output (gitignored)
    ├── debug/
    └── release/
```

---

## 📦 Dependências

```toml
[dependencies]
nix = { version = "0.27", features = ["fs", "signal"] }  # Bindings Unix
libc = "0.2"                                              # Bindings C
colored = "2.1"                                           # Cores no terminal
gethostname = "0.4"                                       # Nome do host
crossterm = "0.27"                                        # Terminal cross-platform
dirs = "5.0"                                              # Diretórios do sistema
serde = { version = "1.0", features = ["derive"] }        # Serialização
serde_json = "1.0"                                        # JSON
```

---

## 🎯 Comandos Built-in

### Total: 30 comandos

| Categoria | Comandos | Quantidade |
|-----------|----------|------------|
| **Shell** | help, exit, quit, alias, reload, source, plugin | 7 |
| **Navegação** | cd, pwd | 2 |
| **Filesystem** | ls, cat, mkdir, touch, rm, cp, mv, find, grep, tree, head, tail, wc | 13 |
| **Sistema** | echo, clear, info, whoami, date, stat, env, export, unset, history, which, type | 12 |

### Lista Completa

```
Shell:      help, exit, quit, alias, reload, source, plugin
Navegação:  cd, pwd
Filesystem: ls, cat, mkdir, touch, rm, cp, mv, find, grep, tree, head, tail, wc
Sistema:    echo, clear, info, whoami, date, stat, env, export, unset, history, which, type
```

---

## 🔧 Arquivos de Configuração

### Localizações

| Arquivo | Caminho | Descrição |
|---------|---------|-----------|
| **Aliases** | `~/.aenshrc` | Aliases do usuário |
| **Config** | `~/.config/aensh/config.json` | Configuração do Aensh |
| **Plugins** | `~/.config/aensh/plugins/` | Diretório de plugins |
| **Plugins JSON** | `~/.config/aensh/plugins.json` | Registro de plugins |
| **Histórico** | `~/.aensh_history` | Histórico de comandos |

### ~/.aenshrc (Exemplo)

```bash
# Aensh Configuration File
# ~/.aenshrc

# Navegação
alias ll='ls -la'
alias la='ls -a'
alias l='ls'
alias ..='cd ..'
alias ...='cd ../..'
alias cls='clear'

# Git
alias gs='git status'
alias ga='git add'
alias gc='git commit'
alias gp='git push'
alias gl='git log --oneline -10'
alias gd='git diff'

# Atalhos
alias home='cd ~'
alias docs='cd ~/Documents'
alias dl='cd ~/Downloads'
```

### config.json (Estrutura)

```json
{
  "default_shell": true,
  "previous_shell": "Fish",
  "setup_completed": true
}
```

| Campo | Tipo | Valores | Descrição |
|-------|------|---------|-----------|
| `default_shell` | bool | true/false | Se Aensh inicia automaticamente |
| `previous_shell` | enum | Bash/Zsh/Fish | Shell anterior do usuário |
| `setup_completed` | bool | true/false | Se setup foi completado |

---

## 🖥️ CLI (Linha de Comando)

### Opções

```bash
aensh                    # Inicia o shell (setup na primeira vez)
aensh -h, --help         # Mostra ajuda
aensh -v, --version      # Mostra versão
aensh --setup            # Re-executa setup inicial
aensh --info             # Mostra info do sistema
aensh --config           # Mostra configuração atual
aensh --default true     # Define como shell padrão
aensh --default false    # Remove como shell padrão
aensh -c "comando"       # Executa comando e sai
```

---

## 🏗️ Arquitetura

### Fluxo de Execução

```
main()
  │
  ├─► handle_args()              # Processa CLI args
  │     └─► --help, --version, --setup, --info, --config, --default, -c
  │
  ├─► check_and_run_setup()      # Setup inicial se necessário
  │     └─► Pergunta shell anterior e se quer ser padrão
  │
  ├─► show_banner()              # Exibe banner ASCII
  │
  ├─► PluginManager::new()       # Carrega plugins
  │
  ├─► AliasManager::new()        # Carrega aliases de ~/.aenshrc
  │
  ├─► ReadLine::new()            # Inicializa readline
  │
  └─► loop {
        │
        ├─► build_prompt()           # user@host:dir>
        │
        ├─► readline.read_line()     # Lê input com setas
        │
        ├─► alias_manager.expand()   # Expande aliases
        │
        ├─► parse_command_chain()    # Parse: cmd1 && cmd2 | cmd3
        │     │
        │     ├─► split by &&        # CommandChain
        │     ├─► split by |         # Pipeline
        │     └─► parse_single_command()
        │           └─► Builtin / Plugin / External
        │
        └─► execute_chain()          # Executa
              │
              ├─► execute_pipeline()
              │     ├─► execute_segment() (single)
              │     └─► execute com piping (multiple)
              │
              └─► Trata && (para se erro)
      }
```

### Tipos Principais

```rust
// Comando simples
pub struct Command {
    pub name: String,
    pub args: Vec<String>,
}

// Segmento de pipeline
pub enum PipelineSegment {
    Builtin(Command),   // Comando built-in
    Plugin(Command),    // Plugin registrado
    External(Command),  // Comando do sistema
}

// Pipeline (comandos separados por |)
pub struct Pipeline {
    pub segments: Vec<PipelineSegment>,
}

// Chain (pipelines separados por &&)
pub struct CommandChain {
    pub commands: Vec<(Pipeline, Option<ChainOperator>)>,
}

// Erros
pub enum AenshError {
    EmptyInput,
    InvalidCommand(String),
    Validation(String),
    Io(String),
}

pub type AenshResult<T> = Result<T, AenshError>;
```

---

## 📝 Padrões de Código

### Nomenclatura

| Tipo | Convenção | Exemplo |
|------|-----------|---------|
| Funções | snake_case | `fn parse_command()` |
| Variáveis | snake_case | `let file_path = ...` |
| Tipos/Structs | PascalCase | `struct CommandChain` |
| Traits | PascalCase | `trait Executable` |
| Constantes | UPPER_CASE | `const MAX_HISTORY: usize` |
| Módulos | snake_case | `mod pipeline` |
| Arquivos | snake_case | `type_cmd.rs` |

### Estrutura de Comando Built-in

```rust
// src/builtins/<categoria>/novo_comando.rs

use colored::*;
use crate::core::errors::{AenshError, AenshResult};

pub fn run(args: &[String]) -> AenshResult<()> {
    if args.is_empty() {
        eprintln!("{} Uso: comando <args>", "Erro:".red());
        return Ok(());
    }

    // Implementação...

    println!("{} Sucesso!", "✓".green());
    Ok(())
}
```

### Cores (colored crate)

```rust
use colored::*;

// Sucesso
println!("{} OK", "✓".green());
println!("{} Concluído", "✓".bright_green());

// Erro
eprintln!("{} Falhou", "✗".red());
eprintln!("{} {}", "Erro:".red(), mensagem);

// Info
println!("{} Informação", "ℹ".blue());
println!("{} Dica", "ℹ".bright_blue());

// Warning
println!("{} Aviso", "⚠".yellow());

// Destaque
println!("{}", "Título".bright_cyan().bold());
println!("{}", texto.bright_white());
println!("{}", path.bright_cyan());
```

### Tratamento de Erros

```rust
// Sempre retornar AenshResult<T>
pub fn run(args: &[String]) -> AenshResult<()> {
    // Para erros de IO
    let content = fs::read_to_string(path)
        .map_err(|e| AenshError::Io(format!("Falha ao ler: {}", e)))?;

    // Para erros de validação
    if args.is_empty() {
        return Err(AenshError::Validation("Argumento obrigatório".into()));
    }

    // Para comandos não encontrados
    return Err(AenshError::InvalidCommand(name.to_string()));

    Ok(())
}
```

---

## 🔌 Sistema de Plugins

### O que são Plugins?

Plugins são **scripts executáveis** que estendem o Aensh com novos comandos. Eles ficam em `~/.config/aensh/plugins/` e são carregados automaticamente.

### Criando um Plugin

1. Crie um script:
```bash
#!/bin/bash
# ~/.config/aensh/plugins/hello
echo "Hello from plugin!"
echo "Args: $@"
```

2. Torne executável:
```bash
chmod +x ~/.config/aensh/plugins/hello
```

3. Use:
```bash
aensh> hello world
Hello from plugin!
Args: world
```

### Gerenciando Plugins

```bash
aensh> plugin list              # Listar
aensh> plugin add nome /path "Desc"  # Adicionar
aensh> plugin remove nome       # Remover
aensh> plugin help              # Ajuda
```

---

## 🔄 Piping e Encadeamento

### Piping (`|`)

Passa stdout de um comando para stdin do próximo:

```bash
aensh> ls | grep txt           # Filtra arquivos .txt
aensh> cat file | wc -l        # Conta linhas
aensh> echo hello | cat        # Passa texto
```

### Encadeamento (`&&`)

Executa comandos em sequência (para se um falhar):

```bash
aensh> mkdir dir && cd dir     # Cria e entra
aensh> cargo build && cargo run # Compila e executa
```

### Combinação

```bash
aensh> ls | grep txt && echo "Encontrado"
aensh> cat file | grep pattern | wc -l && echo "Contado"
```

---

## ⌨️ Atalhos de Teclado

| Tecla | Ação |
|-------|------|
| `↑` | Comando anterior |
| `↓` | Próximo comando |
| `←` | Cursor esquerda |
| `→` | Cursor direita |
| `Home` | Início da linha |
| `End` | Fim da linha |
| `Ctrl+A` | Início da linha |
| `Ctrl+E` | Fim da linha |
| `Ctrl+U` | Limpar linha |
| `Ctrl+W` | Apagar palavra |
| `Ctrl+C` | Cancelar |
| `Ctrl+D` | Sair (EOF) |
| `Backspace` | Apagar anterior |
| `Delete` | Apagar atual |

---

## 🛠️ Desenvolvimento

### Build

```bash
cargo build              # Debug
cargo build --release    # Release
cargo run                # Executar
cargo run -- --help      # Com args
```

### Qualidade

```bash
cargo check              # Verificar compilação
cargo clippy             # Lint
cargo fmt                # Formatar
```

### Adicionar Novo Comando

1. Criar arquivo em `src/builtins/<categoria>/novo.rs`
2. Implementar `pub fn run(args: &[String]) -> AenshResult<()>`
3. Adicionar `pub mod novo;` em `src/builtins/<categoria>/mod.rs`
4. Adicionar em `SUPPORTED_COMMANDS` em `src/builtins/mod.rs`
5. Adicionar case no `dispatch()` em `src/builtins/mod.rs`
6. Adicionar em `BLOCKED_COMMANDS` em `src/core/plugins.rs`

---

## 📊 Métricas

| Métrica | Valor |
|---------|-------|
| Linhas de código | ~3000 |
| Arquivos Rust | ~35 |
| Comandos built-in | 30 |
| Dependências | 8 |
| Tamanho binário (release) | ~2MB |

---

## 🔗 Links

- **Repositório**: https://github.com/aencyorganization/aensh
- **Issues**: https://github.com/aencyorganization/aensh/issues
- **Pull Requests**: https://github.com/aencyorganization/aensh/pulls

---

**Última atualização**: Dezembro 2024  
**Versão**: 0.1.0  
**Autor**: aencyorganization
