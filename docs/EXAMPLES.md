# 📚 Exemplos de Uso do Aensh

Exemplos práticos e úteis de como usar o Aensh.

## 🚀 Iniciando

```bash
# Executar o Aensh
./target/debug/aensh

# Você verá:
    _                  _     
   / \   ___ _ __  ___| |__  
  / _ \ / _ \ '_ \/ __| '_ \ 
 / ___ \  __/ | | \__ \ | | |
/_/   \_\___|_| |_|___/_| |_|

Bem-vindo ao Aensh v0.2.0 - use 'help' para começar

gabriel machine ~ ❯ 
```

## 📁 Trabalhando com Arquivos

### Criar uma estrutura de projeto

```bash
# Criar diretório principal
gabriel machine ~ ❯ mkdir meu_projeto
✓ Diretório meu_projeto criado

# Entrar no diretório
gabriel machine ~ ❯ cd meu_projeto
gabriel machine ~/meu_projeto ❯ 

# Criar subdiretorios
gabriel machine ~/meu_projeto ❯ mkdir src docs tests
✓ Diretório src criado
✓ Diretório docs criado
✓ Diretório tests criado

# Criar arquivos
gabriel machine ~/meu_projeto ❯ touch README.md main.rs
✓ Arquivo README.md criado
✓ Arquivo main.rs criado

# Listar estrutura
gabriel machine ~/meu_projeto ❯ tree .
meu_projeto
├── 📁 docs/
├── 📁 src/
├── 📁 tests/
├── 📄 README.md (0.0KB)
└── 📄 main.rs (0.0KB)
```

### Copiar e organizar arquivos

```bash
# Criar alguns arquivos
gabriel machine ~ ❯ touch file1.txt file2.txt file3.txt
✓ Arquivo file1.txt criado
✓ Arquivo file2.txt criado
✓ Arquivo file3.txt criado

# Criar diretório de backup
gabriel machine ~ ❯ mkdir backup
✓ Diretório backup criado

# Copiar arquivo
gabriel machine ~ ❯ cp file1.txt backup/
✓ file1.txt copiado para backup/

# Mover arquivo
gabriel machine ~ ❯ mv file2.txt backup/
✓ file2.txt movido para backup/

# Listar
gabriel machine ~ ❯ ls
📁 backup/
📄 file3.txt (0.0KB)

gabriel machine ~ ❯ ls backup/
📄 file1.txt (0.0KB)
📄 file2.txt (0.0KB)
```

### Remover arquivos

```bash
# Remover arquivo único
gabriel machine ~ ❯ rm file3.txt
✓ file3.txt removido

# Remover diretório (recursivamente)
gabriel machine ~ ❯ rm backup
✓ backup removido
```

## 📝 Trabalhando com Conteúdo

### Criar e visualizar arquivos

```bash
# Criar arquivo com conteúdo
gabriel machine ~ ❯ echo "Olá, Aensh!" > hello.txt
Olá, Aensh!

# Visualizar conteúdo
gabriel machine ~ ❯ cat hello.txt
Olá, Aensh!

# Criar múltiplos arquivos
gabriel machine ~ ❯ touch file1.txt file2.txt
✓ Arquivo file1.txt criado
✓ Arquivo file2.txt criado

# Visualizar múltiplos arquivos
gabriel machine ~ ❯ cat file1.txt file2.txt
Conteúdo do file1
Conteúdo do file2
```

### Buscar em arquivos

```bash
# Criar arquivo de exemplo
gabriel machine ~ ❯ echo "Linha 1: Aensh é legal" > exemplo.txt
Linha 1: Aensh é legal

# Buscar padrão
gabriel machine ~ ❯ grep "Aensh" exemplo.txt
1 Linha 1: Aensh é legal
```

### Buscar arquivos

```bash
# Buscar todos os arquivos em um diretório
gabriel machine ~ ❯ find . 
.
./src
./docs
./README.md

# Buscar com padrão
gabriel machine ~ ❯ find . .txt
./file1.txt
./file2.txt
```

## 🗂️ Navegação

### Navegar entre diretórios

```bash
# Ir para home
gabriel machine /tmp ❯ cd ~
gabriel machine ~ ❯ 

# Ir para diretório específico
gabriel machine ~ ❯ cd /tmp
gabriel machine /tmp ❯ 

# Ver diretório atual
gabriel machine /tmp ❯ pwd
/tmp
```

## ⚙️ Informações do Sistema

### Ver informações

```bash
# Usuário atual
gabriel machine ~ ❯ whoami
gabriel

# Data e hora
gabriel machine ~ ❯ date
02/12/2024 18:35:42 UTC 🕐

# Informações do Aensh
gabriel machine ~ ❯ info

══════════════════════════════════════════════════
  Aensh - A Modern Shell in Rust
══════════════════════════════════════════════════
Versão: 0.2.0
Usuário: gabriel
Máquina: machine
Linguagem: Rust 🦀
══════════════════════════════════════════════════
```

### Informações de arquivo

```bash
# Ver informações de arquivo
gabriel machine ~ ❯ stat README.md

📊 Informações de: README.md

Tipo: Arquivo 📄
Tamanho: 2.50 KB
Permissões: rw-r--r--
Inode: 12345678
Links: 1
```

## 🎯 Fluxo de Trabalho Completo

### Projeto de exemplo

```bash
# 1. Criar estrutura
gabriel machine ~ ❯ mkdir projeto_rust
✓ Diretório projeto_rust criado

gabriel machine ~ ❯ cd projeto_rust
gabriel machine ~/projeto_rust ❯ 

# 2. Criar diretórios
gabriel machine ~/projeto_rust ❯ mkdir src docs
✓ Diretório src criado
✓ Diretório docs criado

# 3. Criar arquivos
gabriel machine ~/projeto_rust ❯ touch README.md Cargo.toml
✓ Arquivo README.md criado
✓ Arquivo Cargo.toml criado

gabriel machine ~/projeto_rust ❯ touch src/main.rs src/lib.rs
✓ Arquivo src/main.rs criado
✓ Arquivo src/lib.rs criado

# 4. Ver estrutura
gabriel machine ~/projeto_rust ❯ tree .
projeto_rust
├── 📁 docs/
├── 📁 src/
│   ├── 📄 lib.rs (0.0KB)
│   └── 📄 main.rs (0.0KB)
├── 📄 Cargo.toml (0.0KB)
└── 📄 README.md (0.0KB)

# 5. Criar backup
gabriel machine ~/projeto_rust ❯ mkdir backup
✓ Diretório backup criado

gabriel machine ~/projeto_rust ❯ cp README.md backup/
✓ README.md copiado para backup/

# 6. Ver informações
gabriel machine ~/projeto_rust ❯ stat src/main.rs

📊 Informações de: src/main.rs

Tipo: Arquivo 📄
Tamanho: 0.00 B
Permissões: rw-r--r--
Inode: 12345679
Links: 1

# 7. Limpar tela
gabriel machine ~/projeto_rust ❯ clear

# 8. Sair
gabriel machine ~/projeto_rust ❯ exit
Até logo! 👋
```

## 💡 Dicas Úteis

### Combinações úteis

```bash
# Ver ajuda
gabriel machine ~ ❯ help

# Listar com detalhes
gabriel machine ~ ❯ ls

# Ver estrutura de diretório
gabriel machine ~ ❯ tree .

# Buscar arquivo
gabriel machine ~ ❯ find . arquivo.txt

# Buscar padrão em arquivo
gabriel machine ~ ❯ grep "padrão" arquivo.txt

# Ver informações de arquivo
gabriel machine ~ ❯ stat arquivo.txt
```

### Atalhos úteis

- `cd ~` - Ir para home
- `pwd` - Ver diretório atual
- `clear` - Limpar tela
- `exit` - Sair do Aensh

## 🚨 Tratamento de Erros

### Erros comuns e soluções

```bash
# Comando não encontrado
gabriel machine ~ ❯ comando_invalido
✗ Erro: 'comando_invalido' não existe. Use 'help' para ver os comandos disponíveis.

# Arquivo não encontrado
gabriel machine ~ ❯ cat arquivo_inexistente.txt
✗ Erro: não consigo abrir arquivo_inexistente.txt: Arquivo ou diretório não encontrado

# Diretório não encontrado
gabriel machine ~ ❯ cd /diretorio/inexistente
✗ Erro: não consigo mudar para /diretorio/inexistente: Arquivo ou diretório não encontrado

# Permissão negada
gabriel machine ~ ❯ cd /root
✗ Erro: não consigo mudar para /root: Permissão negada
```

---

**Última atualização:** Dezembro 2024

Para mais informações, consulte `docs/USAGE.md` ou use `help` no Aensh.
