<p align="center">
  <img src="https://raw.githubusercontent.com/aencyorganization/aensh/main/assets/logo.png" alt="Aensh Logo" width="200"/>
</p>

<h1 align="center">🦀 Aensh</h1>

<p align="center">
  <strong>Um shell moderno, seguro e extensível escrito em Rust</strong>
</p>

<p align="center">
  <a href="https://github.com/aencyorganization/aensh/releases">
    <img src="https://img.shields.io/badge/version-0.1.0-blue.svg" alt="Version"/>
  </a>
  <a href="LICENSE">
    <img src="https://img.shields.io/badge/license-MIT-green.svg" alt="License"/>
  </a>
  <a href="https://www.rust-lang.org/">
    <img src="https://img.shields.io/badge/rust-1.70+-orange.svg" alt="Rust"/>
  </a>
  <a href="https://github.com/aencyorganization/aensh/actions">
    <img src="https://img.shields.io/badge/build-passing-brightgreen.svg" alt="Build"/>
  </a>
</p>

<p align="center">
  <a href="#-instalação">Instalação</a> •
  <a href="#-uso">Uso</a> •
  <a href="#-comandos">Comandos</a> •
  <a href="#-recursos">Recursos</a> •
  <a href="#-configuração">Configuração</a> •
  <a href="#-desenvolvimento">Desenvolvimento</a>
</p>

---

## 📖 Índice

- [Sobre](#-sobre)
- [Características](#-características)
- [Instalação](#-instalação)
  - [Instalação Rápida](#instalação-rápida)
  - [Instalação Manual](#instalação-manual)
  - [Compilação do Código Fonte](#compilação-do-código-fonte)
  - [Verificação da Instalação](#verificação-da-instalação)
- [Uso](#-uso)
  - [Iniciando o Aensh](#iniciando-o-aensh)
  - [Opções de Linha de Comando](#opções-de-linha-de-comando)
  - [Setup Inicial](#setup-inicial)
  - [Definir como Shell Padrão](#definir-como-shell-padrão)
- [Comandos](#-comandos)
  - [Comandos de Shell](#comandos-de-shell)
  - [Comandos de Navegação](#comandos-de-navegação)
  - [Comandos de Filesystem](#comandos-de-filesystem)
  - [Comandos de Sistema](#comandos-de-sistema)
- [Recursos](#-recursos)
  - [Navegação com Setas](#navegação-com-setas)
  - [Histórico de Comandos](#histórico-de-comandos)
  - [Piping e Encadeamento](#piping-e-encadeamento)
  - [Aliases](#aliases)
  - [Comandos Externos](#comandos-externos)
  - [Plugins](#plugins)
- [Configuração](#-configuração)
  - [Arquivos de Configuração](#arquivos-de-configuração)
  - [.aenshrc](#aenshrc)
  - [config.json](#configjson)
- [Arquitetura](#-arquitetura)
  - [Estrutura do Projeto](#estrutura-do-projeto)
  - [Módulos Principais](#módulos-principais)
  - [Fluxo de Execução](#fluxo-de-execução)
- [Desenvolvimento](#-desenvolvimento)
  - [Pré-requisitos](#pré-requisitos)
  - [Compilação](#compilação)
  - [Testes](#testes)
  - [Contribuindo](#contribuindo)
- [FAQ](#-faq)
- [Troubleshooting](#-troubleshooting)
- [Roadmap](#-roadmap)
- [Licença](#-licença)
- [Créditos](#-créditos)

---

## 🎯 Sobre

**Aensh** (Aency Shell) é um shell de linha de comando moderno escrito em Rust, projetado para ser:

- **🔒 Seguro**: Memory-safe por design, sem buffer overflows ou use-after-free
- **⚡ Rápido**: Compilado para código nativo, sem overhead de interpretação
- **🎨 Bonito**: Interface colorida e moderna com prompts customizáveis
- **🔧 Extensível**: Sistema de aliases e plugins para customização
- **📚 Educativo**: Código limpo e bem documentado para aprendizado

O Aensh foi criado como uma alternativa moderna aos shells tradicionais (bash, zsh, fish), combinando o melhor de cada um com as garantias de segurança do Rust.

---

## ✨ Características

### Core Features

| Feature | Descrição |
|---------|-----------|
| **Navegação Inteligente** | Use setas ↑↓ para histórico e ←→ para mover o cursor |
| **Piping Completo** | Encadeie comandos com `\|` e `&&` |
| **Aliases** | Defina atalhos para comandos frequentes |
| **Histórico Persistente** | Histórico salvo entre sessões |
| **Comandos Externos** | Execute qualquer comando do sistema |
| **Setup Interativo** | Configuração guiada na primeira execução |
| **Prompt Customizável** | Mostra usuário, host e diretório atual |

### Comandos Built-in

O Aensh inclui **30+ comandos built-in** implementados em Rust puro:

```
Shell:      help, exit, quit, alias, reload, source, plugin
Navegação:  cd, pwd
Filesystem: ls, cat, mkdir, touch, rm, cp, mv, find, grep, tree, head, tail, wc
Sistema:    echo, clear, info, whoami, date, stat, env, export, unset, history, which, type
```

### Compatibilidade

| Sistema | Status |
|---------|--------|
| Linux (Ubuntu, Debian, Fedora, Arch) | ✅ Suportado |
| macOS | ✅ Suportado |
| WSL (Windows Subsystem for Linux) | ✅ Suportado |
| Windows (nativo) | ❌ Não suportado |

---

## 📦 Instalação

### Instalação Rápida

A forma mais fácil de instalar o Aensh é usando o script de instalação:

```bash
curl -sSL https://raw.githubusercontent.com/aencyorganization/aensh/main/install.sh | bash
```

Este script irá:
1. Verificar se Rust está instalado (instala se necessário)
2. Clonar o repositório
3. Compilar o Aensh em modo release
4. Instalar em `~/.local/bin/`
5. Adicionar ao PATH
6. Criar diretórios de configuração

### Instalação Manual

Se preferir instalar manualmente:

```bash
# 1. Clone o repositório
git clone https://github.com/aencyorganization/aensh.git
cd aensh

# 2. Compile em modo release
cargo build --release

# 3. Copie o binário para um diretório no PATH
cp target/release/aensh ~/.local/bin/

# 4. Certifique-se que ~/.local/bin está no PATH
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

### Compilação do Código Fonte

#### Pré-requisitos

- **Rust 1.70+** - Instale via [rustup](https://rustup.rs/)
- **Cargo** - Vem junto com Rust
- **Git** - Para clonar o repositório

#### Passos

```bash
# Instalar Rust (se necessário)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Verificar instalação
rustc --version
cargo --version

# Clonar e compilar
git clone https://github.com/aencyorganization/aensh.git
cd aensh
cargo build --release

# O binário estará em target/release/aensh
./target/release/aensh --version
```

### Verificação da Instalação

```bash
# Verificar versão
aensh --version

# Verificar informações do sistema
aensh --info

# Iniciar o shell
aensh
```

---

## 🚀 Uso

### Iniciando o Aensh

```bash
# Iniciar normalmente
aensh

# Executar um comando e sair
aensh -c "ls -la"

# Ver ajuda
aensh --help
```

### Opções de Linha de Comando

| Opção | Descrição |
|-------|-----------|
| `-h`, `--help` | Mostra mensagem de ajuda |
| `-v`, `--version` | Mostra versão do Aensh |
| `--setup` | Re-executa o setup inicial |
| `--info` | Mostra informações do sistema |
| `--config` | Mostra configuração atual |
| `--default true` | Define Aensh como shell padrão |
| `--default false` | Remove Aensh como shell padrão |
| `-c <cmd>` | Executa comando e sai |

### Setup Inicial

Na primeira execução, o Aensh apresenta um setup interativo:

```
╔══════════════════════════════════════════════════════════════╗
║           🦀 Bem-vindo ao Aensh! 🦀                           ║
║                                                              ║
║   Vamos configurar seu shell em alguns passos rápidos.       ║
╚══════════════════════════════════════════════════════════════╝

Passo 1: Qual shell você estava usando antes?

  [1] Bash
  [2] Zsh
  [3] Fish

Escolha (1-3): 
```

O setup pergunta:
1. **Shell anterior**: Bash, Zsh ou Fish
2. **Shell padrão**: Se deseja que o Aensh inicie automaticamente

### Definir como Shell Padrão

Para fazer o Aensh iniciar automaticamente quando você abre o terminal:

```bash
# Ativar
aensh --default true

# Desativar
aensh --default false
```

Isso adiciona/remove um script no arquivo RC do seu shell anterior (`.bashrc`, `.zshrc` ou `config.fish`).

---

## 📚 Comandos

### Comandos de Shell

#### `help`
Mostra a lista de comandos disponíveis.

```bash
aensh> help
```

#### `exit` / `quit`
Encerra o shell.

```bash
aensh> exit
# ou
aensh> quit
```

#### `alias`
Gerencia aliases de comandos.

```bash
# Listar todos os aliases
aensh> alias

# Ver um alias específico
aensh> alias ll

# Definir um alias
aensh> alias ll='ls -la'

# Adicionar alias
aensh> alias add gs git status

# Remover alias
aensh> alias remove ll

# Recarregar aliases do arquivo
aensh> alias reload
```

#### `reload` / `source`
Recarrega o arquivo de configuração `~/.aenshrc`.

```bash
aensh> reload
# ou
aensh> source
```

#### `plugin`
Gerencia plugins externos.

```bash
# Listar plugins
aensh> plugin list

# Adicionar plugin
aensh> plugin add nome /caminho/script "Descrição"

# Remover plugin
aensh> plugin remove nome

# Ajuda
aensh> plugin help
```

---

### Comandos de Navegação

#### `cd`
Altera o diretório atual.

```bash
# Ir para um diretório
aensh> cd /home/user/Documents

# Ir para home
aensh> cd ~
aensh> cd

# Voltar um nível
aensh> cd ..

# Voltar dois níveis
aensh> cd ../..
```

#### `pwd`
Mostra o diretório atual.

```bash
aensh> pwd
/home/user/Documents
```

---

### Comandos de Filesystem

#### `ls`
Lista arquivos e diretórios.

```bash
# Listar diretório atual
aensh> ls

# Listar diretório específico
aensh> ls /home/user

# Listar com detalhes (via alias)
aensh> ll
```

#### `cat`
Exibe o conteúdo de arquivos.

```bash
# Ver conteúdo de um arquivo
aensh> cat arquivo.txt

# Ver múltiplos arquivos
aensh> cat arquivo1.txt arquivo2.txt
```

#### `mkdir`
Cria um novo diretório.

```bash
# Criar diretório
aensh> mkdir novo_diretorio

# Criar diretórios aninhados
aensh> mkdir -p pasta/subpasta/outra
```

#### `touch`
Cria um arquivo vazio ou atualiza timestamp.

```bash
# Criar arquivo
aensh> touch novo_arquivo.txt

# Criar múltiplos arquivos
aensh> touch arquivo1.txt arquivo2.txt
```

#### `rm`
Remove arquivos ou diretórios.

```bash
# Remover arquivo
aensh> rm arquivo.txt

# Remover diretório (recursivo)
aensh> rm -r diretorio/

# Forçar remoção
aensh> rm -f arquivo.txt
```

#### `cp`
Copia arquivos ou diretórios.

```bash
# Copiar arquivo
aensh> cp origem.txt destino.txt

# Copiar diretório
aensh> cp -r pasta/ nova_pasta/
```

#### `mv`
Move ou renomeia arquivos.

```bash
# Mover arquivo
aensh> mv arquivo.txt /outro/lugar/

# Renomear arquivo
aensh> mv antigo.txt novo.txt
```

#### `find`
Busca arquivos em diretórios.

```bash
# Buscar por nome
aensh> find *.txt

# Buscar em diretório específico
aensh> find /home/user documento
```

#### `grep`
Busca padrões em arquivos.

```bash
# Buscar texto em arquivo
aensh> grep "padrão" arquivo.txt

# Buscar em múltiplos arquivos
aensh> grep "texto" *.txt
```

#### `tree`
Mostra estrutura de diretórios em árvore.

```bash
# Árvore do diretório atual
aensh> tree

# Árvore de diretório específico
aensh> tree /home/user/projeto
```

#### `head`
Mostra as primeiras linhas de um arquivo.

```bash
# Primeiras 10 linhas (padrão)
aensh> head arquivo.txt

# Primeiras N linhas
aensh> head -n 20 arquivo.txt
```

#### `tail`
Mostra as últimas linhas de um arquivo.

```bash
# Últimas 10 linhas (padrão)
aensh> tail arquivo.txt

# Últimas N linhas
aensh> tail -n 20 arquivo.txt
```

#### `wc`
Conta linhas, palavras e caracteres.

```bash
# Contagem completa
aensh> wc arquivo.txt

# Apenas linhas
aensh> wc -l arquivo.txt

# Apenas palavras
aensh> wc -w arquivo.txt

# Apenas caracteres
aensh> wc -c arquivo.txt
```

---

### Comandos de Sistema

#### `echo`
Exibe texto na tela.

```bash
# Texto simples
aensh> echo Hello World

# Com variáveis de ambiente
aensh> echo $HOME
```

#### `clear`
Limpa a tela do terminal.

```bash
aensh> clear
```

#### `info`
Mostra informações do Aensh.

```bash
aensh> info
```

#### `whoami`
Mostra o usuário atual.

```bash
aensh> whoami
gabriel
```

#### `date`
Mostra a data e hora atual.

```bash
aensh> date
Tue Dec  3 14:30:00 -03 2024
```

#### `stat`
Mostra informações de arquivo/diretório.

```bash
aensh> stat arquivo.txt
```

#### `env`
Mostra variáveis de ambiente.

```bash
# Listar todas
aensh> env

# Ver variável específica
aensh> env HOME
```

#### `export`
Define variável de ambiente.

```bash
# Definir variável
aensh> export MINHA_VAR=valor

# Verificar
aensh> env MINHA_VAR
```

#### `unset`
Remove variável de ambiente.

```bash
aensh> unset MINHA_VAR
```

#### `history`
Mostra histórico de comandos.

```bash
# Ver todo histórico
aensh> history

# Ver últimos N comandos
aensh> history 20
```

#### `which`
Mostra caminho de um comando.

```bash
aensh> which python
/usr/bin/python

aensh> which ls
ls: aensh builtin
```

#### `type`
Mostra tipo de um comando.

```bash
aensh> type ls
ls is a aensh builtin command

aensh> type python
python is /usr/bin/python
```

---

## 🔧 Recursos

### Navegação com Setas

O Aensh suporta navegação completa na linha de comando:

| Tecla | Ação |
|-------|------|
| `↑` | Comando anterior no histórico |
| `↓` | Próximo comando no histórico |
| `←` | Move cursor para esquerda |
| `→` | Move cursor para direita |
| `Home` | Vai para início da linha |
| `End` | Vai para fim da linha |
| `Ctrl+A` | Vai para início da linha |
| `Ctrl+E` | Vai para fim da linha |
| `Ctrl+U` | Limpa a linha |
| `Ctrl+W` | Apaga palavra anterior |
| `Ctrl+C` | Cancela comando atual |
| `Ctrl+D` | Sai do shell (EOF) |
| `Backspace` | Apaga caractere anterior |
| `Delete` | Apaga caractere atual |

### Histórico de Comandos

O histórico é salvo automaticamente em `~/.aensh_history`:

```bash
# Ver histórico
aensh> history

# Ver últimos 20 comandos
aensh> history 20
```

Características:
- Persistente entre sessões
- Máximo de 1000 entradas
- Não duplica comandos consecutivos
- Navegável com setas ↑↓

### Piping e Encadeamento

#### Piping (`|`)

Redireciona a saída de um comando para a entrada de outro:

```bash
# Listar e filtrar
aensh> ls | grep txt

# Contar arquivos
aensh> ls | wc -l

# Encadear múltiplos
aensh> cat arquivo.txt | grep erro | wc -l
```

#### Encadeamento (`&&`)

Executa comandos em sequência (para se um falhar):

```bash
# Criar e entrar em diretório
aensh> mkdir projeto && cd projeto

# Compilar e executar
aensh> cargo build && cargo run

# Múltiplos comandos
aensh> git add . && git commit -m "update" && git push
```

### Aliases

Aliases são atalhos para comandos frequentes.

#### Definindo Aliases

```bash
# Via comando
aensh> alias ll='ls -la'
aensh> alias gs='git status'

# Via arquivo ~/.aenshrc
alias ll='ls -la'
alias la='ls -a'
alias ..='cd ..'
```

#### Arquivo ~/.aenshrc

O arquivo `~/.aenshrc` é carregado automaticamente:

```bash
# ~/.aenshrc

# Navegação
alias ll='ls -la'
alias la='ls -a'
alias l='ls'
alias ..='cd ..'
alias ...='cd ../..'

# Git
alias gs='git status'
alias ga='git add'
alias gc='git commit'
alias gp='git push'
alias gl='git log --oneline -10'

# Atalhos
alias cls='clear'
alias home='cd ~'
```

### Comandos Externos

O Aensh pode executar qualquer comando do sistema que não seja um built-in:

```bash
# Ferramentas de desenvolvimento
aensh> python script.py
aensh> node app.js
aensh> cargo build
aensh> npm install

# Utilitários
aensh> curl https://api.example.com
aensh> wget https://example.com/file.zip
aensh> git clone https://github.com/user/repo

# Editores
aensh> vim arquivo.txt
aensh> nano arquivo.txt
aensh> code .

# Programas interativos
aensh> htop
aensh> cmatrix
aensh> vim
```

### Plugins

Plugins são scripts executáveis que estendem o Aensh.

#### Criando um Plugin

1. Crie um script em `~/.config/aensh/plugins/`:

```bash
#!/bin/bash
# ~/.config/aensh/plugins/hello

echo "Hello from plugin!"
echo "Arguments: $@"
```

2. Torne executável:

```bash
chmod +x ~/.config/aensh/plugins/hello
```

3. Use no Aensh:

```bash
aensh> hello world
Hello from plugin!
Arguments: world
```

#### Gerenciando Plugins

```bash
# Listar plugins
aensh> plugin list

# Adicionar plugin manualmente
aensh> plugin add myscript /path/to/script "Descrição"

# Remover plugin
aensh> plugin remove myscript
```

---

## ⚙️ Configuração

### Arquivos de Configuração

| Arquivo | Descrição |
|---------|-----------|
| `~/.aenshrc` | Aliases e configurações do usuário |
| `~/.config/aensh/config.json` | Configuração do Aensh |
| `~/.config/aensh/plugins/` | Diretório de plugins |
| `~/.config/aensh/plugins.json` | Registro de plugins |
| `~/.aensh_history` | Histórico de comandos |

### .aenshrc

Arquivo de configuração do usuário, carregado na inicialização:

```bash
# ~/.aenshrc
# Aensh Configuration File

# Aliases de navegação
alias ll='ls -la'
alias la='ls -a'
alias l='ls'
alias ..='cd ..'
alias ...='cd ../..'
alias cls='clear'

# Aliases de Git
alias gs='git status'
alias ga='git add'
alias gc='git commit'
alias gp='git push'
alias gl='git log --oneline -10'
alias gd='git diff'

# Aliases de navegação rápida
alias home='cd ~'
alias docs='cd ~/Documents'
alias dl='cd ~/Downloads'
alias proj='cd ~/Projects'

# Aliases de desenvolvimento
alias py='python3'
alias pip='pip3'
alias nr='npm run'
alias dc='docker-compose'
```

### config.json

Configuração interna do Aensh:

```json
{
  "default_shell": true,
  "previous_shell": "Fish",
  "setup_completed": true
}
```

| Campo | Tipo | Descrição |
|-------|------|-----------|
| `default_shell` | boolean | Se Aensh é o shell padrão |
| `previous_shell` | string | Shell anterior (Bash/Zsh/Fish) |
| `setup_completed` | boolean | Se o setup foi completado |

---

## 🏗️ Arquitetura

### Estrutura do Projeto

```
aensh/
├── Cargo.toml              # Dependências e metadata
├── Cargo.lock              # Lock de versões
├── README.md               # Este arquivo
├── CONTEXT.md              # Contexto do projeto
├── LICENSE                 # Licença MIT
├── install.sh              # Script de instalação
│
├── src/
│   ├── main.rs             # Entry point, CLI, main loop
│   │
│   ├── core/               # Módulos principais
│   │   ├── mod.rs          # Exports
│   │   ├── aliases.rs      # Sistema de aliases
│   │   ├── banner.rs       # Banner de inicialização
│   │   ├── command.rs      # Struct Command
│   │   ├── config.rs       # Configuração
│   │   ├── errors.rs       # Tipos de erro
│   │   ├── external.rs     # Comandos externos
│   │   ├── history.rs      # Histórico
│   │   ├── pipeline.rs     # Parser e executor
│   │   ├── plugins.rs      # Sistema de plugins
│   │   ├── prompt.rs       # Geração do prompt
│   │   ├── readline.rs     # Input com setas
│   │   └── setup.rs        # Setup inicial
│   │
│   └── builtins/           # Comandos built-in
│       ├── mod.rs          # Dispatcher
│       ├── filesystem/     # ls, cat, mkdir, etc
│       ├── navigation/     # cd, pwd
│       ├── shell/          # help, exit, alias, plugin
│       └── system/         # echo, clear, env, etc
│
├── docs/                   # Documentação
│   ├── QUICK_START.md
│   ├── USAGE.md
│   └── ...
│
└── target/                 # Build output
    ├── debug/
    └── release/
```

### Módulos Principais

#### `core/`

| Módulo | Responsabilidade |
|--------|------------------|
| `aliases.rs` | Gerenciamento de aliases (~/.aenshrc) |
| `banner.rs` | Banner ASCII de inicialização |
| `command.rs` | Estrutura Command (nome + args) |
| `config.rs` | Configuração persistente |
| `errors.rs` | Tipos de erro (AenshError, AenshResult) |
| `external.rs` | Execução de comandos externos |
| `history.rs` | Histórico persistente |
| `pipeline.rs` | Parser de comandos e pipelines |
| `plugins.rs` | Sistema de plugins |
| `prompt.rs` | Geração do prompt colorido |
| `readline.rs` | Input com navegação por setas |
| `setup.rs` | Setup inicial interativo |

#### `builtins/`

| Categoria | Comandos |
|-----------|----------|
| `shell/` | help, exit, alias, plugin |
| `navigation/` | cd, pwd |
| `filesystem/` | ls, cat, mkdir, touch, rm, cp, mv, find, grep, tree, head, tail, wc |
| `system/` | echo, clear, info, whoami, date, stat, env, export, unset, history, which, type |

### Fluxo de Execução

```
main()
  │
  ├─► handle_args()           # Processa --help, --version, etc
  │
  ├─► check_and_run_setup()   # Setup inicial se necessário
  │
  ├─► show_banner()           # Exibe banner ASCII
  │
  └─► loop {
        │
        ├─► build_prompt()        # Gera prompt colorido
        │
        ├─► readline.read_line()  # Lê input com setas
        │
        ├─► alias_manager.expand() # Expande aliases
        │
        ├─► parse_command_chain() # Parse: cmd1 && cmd2 | cmd3
        │
        └─► execute_chain()       # Executa pipeline
      }
```

### Pipeline

O parser suporta:

- **Comandos simples**: `ls -la`
- **Piping**: `ls | grep txt`
- **Encadeamento**: `mkdir dir && cd dir`
- **Combinação**: `cat file | grep pattern && echo done`

```rust
enum PipelineSegment {
    Builtin(Command),   // Comando built-in do Aensh
    Plugin(Command),    // Plugin registrado
    External(Command),  // Comando do sistema
}

struct Pipeline {
    segments: Vec<PipelineSegment>,  // Separados por |
}

struct CommandChain {
    pipelines: Vec<Pipeline>,  // Separados por &&
}
```

---

## 🛠️ Desenvolvimento

### Pré-requisitos

- **Rust 1.70+**: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- **Cargo**: Vem com Rust
- **Git**: Para controle de versão

### Compilação

```bash
# Clone o repositório
git clone https://github.com/aencyorganization/aensh.git
cd aensh

# Build debug (rápido, com símbolos de debug)
cargo build

# Build release (otimizado)
cargo build --release

# Executar diretamente
cargo run

# Executar com argumentos
cargo run -- --help
```

### Testes

```bash
# Verificar compilação
cargo check

# Lint com Clippy
cargo clippy

# Formatar código
cargo fmt

# Testar build release
cargo build --release
./target/release/aensh --version
```

### Contribuindo

1. **Fork** o repositório
2. **Clone** seu fork: `git clone https://github.com/seu-usuario/aensh.git`
3. **Crie uma branch**: `git checkout -b feature/nova-feature`
4. **Faça suas mudanças**
5. **Commit**: `git commit -m 'Add nova feature'`
6. **Push**: `git push origin feature/nova-feature`
7. **Abra um Pull Request**

#### Convenções de Código

```rust
// Funções e variáveis: snake_case
fn minha_funcao() {}
let minha_variavel = 42;

// Tipos e traits: PascalCase
struct MeuStruct {}
trait MeuTrait {}

// Constantes: UPPER_CASE
const MINHA_CONSTANTE: i32 = 42;

// Erros: sempre retornar AenshResult<T>
pub fn run(args: &[String]) -> AenshResult<()> {
    Ok(())
}

// Cores: usar colored::*
use colored::*;
println!("{} OK", "✓".green());
eprintln!("{} Erro", "✗".red());
```

#### Adicionando um Novo Comando

1. Crie o arquivo em `src/builtins/<categoria>/novo_comando.rs`
2. Implemente a função `run(args: &[String]) -> AenshResult<()>`
3. Adicione ao `mod.rs` da categoria
4. Adicione ao `SUPPORTED_COMMANDS` em `src/builtins/mod.rs`
5. Adicione ao `dispatch()` em `src/builtins/mod.rs`
6. Adicione à lista de bloqueados em `src/core/plugins.rs`

---

## ❓ FAQ

### O Aensh substitui meu shell atual?

Não automaticamente. Você pode usar o Aensh como um programa normal ou configurá-lo para iniciar automaticamente com `aensh --default true`.

### Posso usar comandos do meu shell anterior?

Sim! O Aensh executa qualquer comando do sistema que não seja um built-in. Comandos como `curl`, `git`, `python`, `docker`, etc. funcionam normalmente.

### Como volto para meu shell anterior?

1. Digite `exit` ou pressione `Ctrl+D` para sair do Aensh
2. Para desativar a inicialização automática: `aensh --default false`

### Onde ficam minhas configurações?

- Aliases: `~/.aenshrc`
- Configuração: `~/.config/aensh/config.json`
- Histórico: `~/.aensh_history`
- Plugins: `~/.config/aensh/plugins/`

### O Aensh é compatível com scripts bash?

Não diretamente. O Aensh tem sua própria sintaxe. Para scripts bash, use `bash script.sh`.

### Como reporto um bug?

Abra uma issue em: https://github.com/aencyorganization/aensh/issues

---

## 🔧 Troubleshooting

### Erro: "rustc/cargo not found"

Instale Rust:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

### Erro: "permission denied"

Adicione permissão de execução:
```bash
chmod +x target/release/aensh
```

### Erro: "command not found" após instalar

Adicione ao PATH:
```bash
export PATH="$HOME/.local/bin:$PATH"
```

### Build falha

Limpe e reconstrua:
```bash
cargo clean
cargo build --release
```

### Comando externo não funciona

Verifique se o comando existe:
```bash
aensh> which comando
aensh> type comando
```

### Aliases não carregam

Recarregue o arquivo:
```bash
aensh> reload
```

---

## 🗺️ Roadmap

### v0.1.0 (Atual) ✅
- [x] Shell funcional básico
- [x] Comandos built-in (30+)
- [x] Navegação com setas
- [x] Histórico persistente
- [x] Piping e encadeamento
- [x] Sistema de aliases
- [x] Setup inicial interativo
- [x] Comandos externos

### v0.2.0 (Próximo)
- [ ] Autocompletar com Tab
- [ ] Wildcards (*, ?, [])
- [ ] Busca no histórico (Ctrl+R)
- [ ] Mais comandos built-in

### v0.3.0
- [ ] Variáveis de ambiente persistentes
- [ ] Redirecionamento (>, >>)
- [ ] Entrada padrão (<)
- [ ] Subshells

### v1.0.0
- [ ] Scripts Aensh
- [ ] Funções
- [ ] Jobs (bg, fg)
- [ ] Temas customizáveis
- [ ] Configuração avançada

---

## 📄 Licença

Este projeto está licenciado sob a **MIT License**.

```
MIT License

Copyright (c) 2024 Aency Organization

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```

---

## 🙏 Créditos

### Autor

**Aency Organization** - [@aencyorganization](https://github.com/aencyorganization)

### Dependências

| Crate | Versão | Descrição |
|-------|--------|-----------|
| `nix` | 0.27 | Bindings Unix |
| `libc` | 0.2 | Bindings C |
| `colored` | 2.1 | Cores no terminal |
| `gethostname` | 0.4 | Nome do host |
| `crossterm` | 0.27 | Terminal cross-platform |
| `dirs` | 5.0 | Diretórios do sistema |
| `serde` | 1.0 | Serialização |
| `serde_json` | 1.0 | JSON |

### Inspiração

- **Bash** - O shell clássico
- **Zsh** - Recursos avançados
- **Fish** - Interface amigável
- **Nushell** - Shell moderno em Rust

---

<p align="center">
  <strong>Feito com ❤️ em Rust 🦀</strong>
</p>

<p align="center">
  <a href="https://github.com/aencyorganization/aensh">GitHub</a> •
  <a href="https://github.com/aencyorganization/aensh/issues">Issues</a> •
  <a href="https://github.com/aencyorganization/aensh/pulls">Pull Requests</a>
</p>
