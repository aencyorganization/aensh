# 🚀 Aensh - A Modern Shell in Rust

Um shell moderno e funcional implementado em Rust, construído do zero para demonstrar conceitos de programação de sistemas com foco em segurança, performance e usabilidade.

## ✨ Características

### 📚 Comandos de Shell
- `help` - Mostra a lista de comandos disponíveis
- `exit` / `quit` - Encerra o shell

### 🗂️ Navegação
- `cd <diretório>` - Altera o diretório atual
- `pwd` - Mostra o diretório atual

### 📁 Sistema de Arquivos
- `ls [diretório]` - Lista arquivos e diretórios com ícones e tamanhos
- `cat <arquivo>` - Exibe o conteúdo de arquivos
- `mkdir <diretório>` - Cria um novo diretório
- `touch <arquivo>` - Cria um arquivo vazio
- `rm <arquivo/diretório>` - Remove arquivos ou diretórios
- `cp <origem> <destino>` - Copia arquivos ou diretórios
- `mv <origem> <destino>` - Move ou renomeia arquivos

### ⚙️ Sistema
- `echo <texto>` - Exibe texto na tela
- `clear` - Limpa a tela
- `info` - Mostra informações do Aensh
- `whoami` - Mostra o usuário atual
- `date` - Mostra a data e hora atual

## 🏗️ Arquitetura

A estrutura do projeto foi reorganizada para melhor hierarquia e manutenibilidade:

```
src/
├── main.rs              # Loop principal do shell
├── core/                # Módulos principais
│   ├── banner.rs        # Banner de boas-vindas
│   ├── command.rs       # Estrutura de comando
│   ├── errors.rs        # Sistema de erros
│   ├── input.rs         # Parser de entrada
│   └── prompt.rs        # Construtor de prompt
└── builtins/            # Comandos built-in
    ├── shell/           # Comandos de shell (help, exit)
    ├── navigation/      # Navegação (cd, pwd)
    ├── filesystem/      # Sistema de arquivos (ls, cat, mkdir, etc)
    └── system/          # Sistema (echo, clear, info, etc)
```

### Tecnologias Utilizadas

- **Rust 1.70+** - Linguagem de programação
- **colored** - Colorização de saída
- **nix** - Chamadas de sistema POSIX
- **gethostname** - Obtenção de hostname
- **libc** - Bindings C

## 🚀 Construção e Execução

### Pré-requisitos
- Rust 1.70 ou superior
- Cargo

### Build

```bash
# Build em modo debug
cargo build

# Build em modo release (otimizado)
cargo build --release
```

### Execução

```bash
# Executar diretamente
./target/debug/aensh

# Ou com cargo
cargo run

# Ou com release
./target/release/aensh
```

### Instalação

```bash
# Usar o script de instalação
chmod +x install.sh
./install.sh
```

## 📖 Exemplos de Uso

```bash
# Navegar entre diretórios
gabriel machine ~ ❯ cd /tmp
gabriel machine /tmp ❯ pwd
/tmp

# Listar arquivos
gabriel machine /tmp ❯ ls
📁 dir1/
📄 file.txt (1.2KB)

# Criar e manipular arquivos
gabriel machine /tmp ❯ touch novo.txt
✓ Arquivo novo.txt criado
gabriel machine /tmp ❯ echo "Olá, Aensh!"
Olá, Aensh!

# Ver informações
gabriel machine /tmp ❯ info
══════════════════════════════════════════════════
  Aensh - A Modern Shell in Rust
══════════════════════════════════════════════════
Versão: 0.2.0
Usuário: gabriel
Máquina: machine
Linguagem: Rust 🦀
══════════════════════════════════════════════════

# Sair
gabriel machine /tmp ❯ exit
Até logo! 👋
```

## 🎨 Interface

O Aensh apresenta uma interface moderna e intuitiva com:

- **Prompt colorido** - Mostra usuário, máquina e diretório atual
- **Ícones visuais** - Emojis para melhor visualização
- **Mensagens de erro claras** - Feedback útil quando algo dá errado
- **Cores temáticas** - Diferentes cores para diferentes tipos de saída

## 🔒 Segurança

O Aensh implementa várias medidas de segurança:

- **Validação de entrada** - Bloqueia sequências perigosas (`&&`, `||`, `;`, `$()`)
- **Sem execução de código arbitrário** - Comandos são validados antes da execução
- **Memory-safe** - Rust garante segurança de memória em tempo de compilação
- **Tratamento de sinais** - Captura SIGINT e SIGTERM corretamente

## 📚 Próximas Melhorias

- [ ] Histórico de comandos
- [ ] Autocompletar com Tab
- [ ] Pipes e redirecionamento
- [ ] Variáveis de ambiente
- [ ] Aliases de comandos
- [ ] Scripts shell
- [ ] Suporte a wildcards
- [ ] Modo interativo melhorado

## 🤝 Contribuindo

Contribuições são bem-vindas! Sinta-se à vontade para:

1. Reportar bugs
2. Sugerir novas funcionalidades
3. Melhorar a documentação
4. Enviar pull requests

## 📄 Licença

Este projeto é de código aberto e está disponível sob a licença MIT.

## 🦀 Por que Rust?

- **Segurança de memória** - Sem segfaults ou vazamentos de memória
- **Performance** - Compilado para código nativo com otimizações
- **Tooling moderno** - Cargo, rustfmt, clippy
- **Type safety** - Erros detectados em tempo de compilação
- **Concorrência segura** - Primitivas de concorrência thread-safe
- **Comunidade ativa** - Ecossistema rico de bibliotecas

## 📞 Suporte

Para dúvidas ou problemas, abra uma issue no repositório.

---

**Versão:** 0.2.0  
**Última atualização:** Dezembro 2024  
**Desenvolvido com ❤️ em Rust**
