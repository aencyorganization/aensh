# 🚀 Aensh - A Modern Shell in Rust

Um shell moderno e funcional implementado em Rust, construído do zero com foco em segurança, performance e usabilidade.

## 📦 Instalação

### Instalação Rápida (Recomendado)

```bash
curl -sSL https://raw.githubusercontent.com/aencyorganization/aensh/main/install.sh | bash
```

O instalador irá automaticamente:
- Instalar Rust/Cargo se necessário
- Compilar o Aensh
- Adicionar ao PATH
- Criar diretórios de configuração

### Instalação Manual

```bash
# Clone o repositório
git clone https://github.com/gabriel/aensh.git
cd aensh

# Compile
cargo build --release

# Instale
cp target/release/aensh ~/.local/bin/
```

## 🚀 Uso

```bash
# Iniciar o shell
aensh

# Ver ajuda
aensh --help

# Ver versão
aensh --version

# Definir como shell padrão ao abrir terminal
aensh --default true

# Remover como shell padrão
aensh --default false
```

## ✨ Características

### 🎯 Navegação Avançada
- **Setas ↑/↓** - Navegar no histórico de comandos
- **Setas ←/→** - Mover cursor na linha
- **Home/End** - Ir para início/fim da linha
- **Ctrl+A/E** - Início/fim da linha
- **Ctrl+U** - Limpar linha
- **Ctrl+W** - Apagar palavra anterior

### 🔗 Piping e Encadeamento
```bash
# Encadear comandos com &&
echo "Hello" && echo "World"

# Piping de comandos
cat arquivo.txt | grep "texto"
```

### 🔌 Sistema de Plugins
Adicione comandos personalizados sem modificar o shell:

```bash
# Listar plugins
plugin list

# Adicionar plugin
plugin add myplugin /path/to/script "Descrição do plugin"

# Remover plugin
plugin remove myplugin
```

Ou coloque scripts executáveis em `~/.config/aensh/plugins/`

### 📚 Comandos Built-in

| Categoria | Comandos |
|-----------|----------|
| **Shell** | `help`, `exit`, `quit`, `plugin` |
| **Navegação** | `cd`, `pwd` |
| **Arquivos** | `ls`, `cat`, `mkdir`, `touch`, `rm`, `cp`, `mv`, `find`, `grep`, `tree` |
| **Sistema** | `echo`, `clear`, `info`, `whoami`, `date`, `stat` |

## 🔒 Segurança

O Aensh bloqueia comandos nativos do shell para evitar conflitos:
- Comandos como `bash`, `sh`, `sudo`, `apt`, etc. são bloqueados
- Use plugins para adicionar funcionalidades extras de forma segura
- Validação de entrada contra injeção de comandos

## 📁 Estrutura de Arquivos

```
~/.config/aensh/
├── config.json      # Configurações
└── plugins/         # Plugins executáveis

~/.aensh_history     # Histórico de comandos
```

## 🛠️ Desenvolvimento

```bash
# Build debug
cargo build

# Build release
cargo build --release

# Executar
cargo run
```

## 📄 Licença

MIT License - Veja o arquivo LICENSE para detalhes.

---

**Versão:** 0.2.0 | **Desenvolvido com ❤️ em Rust 🦀**
