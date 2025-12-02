# 🔧 Guia de Desenvolvimento do Aensh

Este documento descreve como contribuir e desenvolver novas funcionalidades para o Aensh.

## 📋 Estrutura do Projeto

```
src/
├── main.rs                 # Ponto de entrada principal
├── core/                   # Módulos principais do shell
│   ├── mod.rs             # Re-exportações
│   ├── banner.rs          # Banner de boas-vindas
│   ├── command.rs         # Estrutura Command
│   ├── errors.rs          # Sistema de erros
│   ├── input.rs           # Parser de entrada
│   └── prompt.rs          # Construtor de prompt
└── builtins/              # Comandos built-in
    ├── mod.rs             # Dispatcher de comandos
    ├── shell/             # Comandos de shell
    │   ├── mod.rs
    │   ├── help.rs        # Comando help
    │   └── exit.rs        # Comando exit
    ├── navigation/        # Navegação
    │   ├── mod.rs
    │   ├── cd.rs          # Comando cd
    │   └── pwd.rs         # Comando pwd
    ├── filesystem/        # Sistema de arquivos
    │   ├── mod.rs
    │   ├── ls.rs          # Listar arquivos
    │   ├── cat.rs         # Exibir arquivo
    │   ├── mkdir.rs       # Criar diretório
    │   ├── touch.rs       # Criar arquivo
    │   ├── rm.rs          # Remover arquivo
    │   ├── cp.rs          # Copiar arquivo
    │   └── mv.rs          # Mover arquivo
    └── system/            # Comandos de sistema
        ├── mod.rs
        ├── echo.rs        # Ecoar texto
        ├── clear.rs       # Limpar tela
        ├── info.rs        # Informações
        ├── whoami.rs      # Usuário atual
        └── date.rs        # Data e hora
```

## ➕ Adicionando um Novo Comando

### Passo 1: Criar o arquivo do comando

Crie um novo arquivo na categoria apropriada. Por exemplo, para um comando de sistema chamado `hostname`:

```bash
touch src/builtins/system/hostname.rs
```

### Passo 2: Implementar o comando

```rust
use crate::core::errors::AenshResult;
use colored::*;

pub fn run(_args: &[String]) -> AenshResult<()> {
    let hostname = gethostname::gethostname().to_string_lossy().to_string();
    println!("{}", hostname.bright_cyan().bold());
    Ok(())
}
```

**Regras importantes:**
- Sempre retornar `AenshResult<()>`
- Usar `colored::*` para colorização
- Usar `AenshError` para erros
- Validar argumentos no início da função

### Passo 3: Registrar o módulo

Adicione o módulo ao arquivo `mod.rs` da categoria:

```rust
// src/builtins/system/mod.rs
pub mod hostname;
```

### Passo 4: Adicionar ao dispatcher

Atualize `src/builtins/mod.rs`:

```rust
// Adicione à lista SUPPORTED_COMMANDS
pub const SUPPORTED_COMMANDS: &[(&str, &str)] = &[
    // ... comandos existentes ...
    ("hostname", "Mostra o hostname da máquina"),
];

// Adicione ao match no dispatcher
pub fn dispatch(command: &Command) -> AenshResult<()> {
    match command.name.as_str() {
        // ... casos existentes ...
        "hostname" => system::hostname::run(&command.args),
        // ...
    }
}
```

### Passo 5: Atualizar a ajuda

Atualize `src/builtins/shell/help.rs` se necessário para incluir o novo comando na categoria apropriada.

## 🎨 Convenções de Código

### Colorização

Use as cores do crate `colored`:

```rust
use colored::*;

// Sucesso
println!("{} Operação concluída", "✓".bright_green());

// Erro
eprintln!("{} Algo deu errado", "✗".bright_red());

// Informação
println!("{}", "Informação".bright_cyan());

// Destaque
println!("{}", "Importante".bright_yellow().bold());
```

### Tratamento de Erros

```rust
use crate::core::errors::{AenshError, AenshResult};

// Para erros de validação
if args.is_empty() {
    return Err(AenshError::Validation("uso: comando <arg>".into()));
}

// Para erros de I/O
fs::read_to_string(path)
    .map_err(|e| AenshError::Io(format!("erro ao ler {}: {}", path, e)))?;
```

### Nomes de Funções

- `run(args: &[String]) -> AenshResult<()>` - Função principal do comando
- Funções auxiliares em snake_case
- Constantes em UPPER_CASE

## 🧪 Testando

### Build

```bash
cargo build
```

### Executar

```bash
cargo run
```

### Testar um comando específico

```bash
cargo run
# No shell do Aensh:
seu_comando arg1 arg2
```

### Verificar erros de compilação

```bash
cargo check
```

### Formatar código

```bash
cargo fmt
```

### Lint

```bash
cargo clippy
```

## 📝 Padrões de Mensagens

### Sucesso
```
✓ Operação concluída
```

### Erro
```
✗ Erro: descrição do erro
```

### Informação
```
ℹ Informação importante
```

### Arquivo/Diretório
```
📄 arquivo.txt
📁 diretório/
```

## 🔄 Fluxo de Desenvolvimento

1. **Criar branch** para sua feature
2. **Implementar** o comando seguindo as convenções
3. **Testar** manualmente no shell
4. **Verificar** com `cargo check` e `cargo clippy`
5. **Formatar** com `cargo fmt`
6. **Fazer commit** com mensagem descritiva
7. **Enviar pull request**

## 🚀 Melhorias Futuras

### Curto prazo
- [ ] Suporte a múltiplos argumentos em `ls`
- [ ] Colorização de saída em `cat`
- [ ] Validação de permissões em `rm`

### Médio prazo
- [ ] Histórico de comandos
- [ ] Autocompletar com Tab
- [ ] Variáveis de ambiente
- [ ] Aliases de comandos

### Longo prazo
- [ ] Pipes e redirecionamento
- [ ] Scripts shell
- [ ] Wildcards
- [ ] Modo interativo melhorado

## 📚 Recursos Úteis

- [Documentação Rust](https://doc.rust-lang.org/)
- [Crate colored](https://docs.rs/colored/)
- [Crate nix](https://docs.rs/nix/)
- [Guia de Rust](https://doc.rust-lang.org/book/)

## 🤝 Contribuindo

1. Fork o repositório
2. Crie uma branch para sua feature (`git checkout -b feature/AmazingFeature`)
3. Commit suas mudanças (`git commit -m 'Add some AmazingFeature'`)
4. Push para a branch (`git push origin feature/AmazingFeature`)
5. Abra um Pull Request

## 📄 Licença

Este projeto está sob a licença MIT. Veja LICENSE para mais detalhes.
