# 📖 Guia de Uso do Aensh

Um guia completo para usar todos os comandos disponíveis no Aensh.

## 🚀 Iniciando o Aensh

```bash
./target/debug/aensh
# ou
cargo run
```

Você verá o banner de boas-vindas e o prompt:

```
    _                  _     
   / \   ___ _ __  ___| |__  
  / _ \ / _ \ '_ \/ __| '_ \ 
 / ___ \  __/ | | \__ \ | | |
/_/   \_\___|_| |_|___/_| |_|

Bem-vindo ao Aensh v0.1.0 - use 'help' para começar

gabriel machine ~ ❯ 
```

## 🧩 Modos de Uso como Shell

### 1. Uso normal (programa comum)

```bash
aensh           # roda o shell e não mexe no seu shell padrão
```

### 2. Shell padrão via RC (`--default`)

Adiciona um bloco no RC do seu shell anterior (`.bashrc`, `.zshrc`, `config.fish`) para iniciar o Aensh automaticamente:

```bash
aensh --default true   # ativa
aensh --default false  # desativa
```

Esse modo **não tenta** alterar o shell de login do sistema.

### 3. Integração forte (`--system-default`)

Quando você quiser que o Aensh tente se tornar também o shell de login via `chsh`, use a opção `--system-default` junto com a variável de ambiente `AENSH_ENABLE_CHSH`:

```bash
# Ativar integração forte (se possível)
AENSH_ENABLE_CHSH=1 aensh --system-default true

# Remover integração forte e tentar restaurar o shell anterior
AENSH_ENABLE_CHSH=1 aensh --system-default false
```

Regras de segurança:
- Se `AENSH_ENABLE_CHSH` **não** estiver definida, `--system-default` se comporta como `--default`.
- Antes de chamar `chsh`, o Aensh verifica se o caminho alvo existe e está em `/etc/shells`.
- Se não houver nenhum shell válido para restaurar, o Aensh apenas remove o bloco de RC e não força nenhum ajuste de sistema.

## 📚 Comandos de Shell

### `help`
Mostra a lista de todos os comandos disponíveis.

```bash
gabriel machine ~ ❯ help

Aensh Comandos

📚 Comandos de Shell:
  help - Mostra a lista de comandos disponíveis
  exit - Encerra o shell
  quit - Encerra o shell (alias para exit)

🗂️  Navegação:
  cd - Altera o diretório atual
  pwd - Mostra o diretório atual

📁 Sistema de Arquivos:
  ls - Lista arquivos e diretórios
  cat - Exibe o conteúdo de arquivos
  mkdir - Cria um novo diretório
  touch - Cria um arquivo vazio ou atualiza timestamp
  rm - Remove arquivos ou diretórios
  cp - Copia arquivos ou diretórios
  mv - Move ou renomeia arquivos

⚙️  Sistema:
  echo - Exibe texto na tela
  clear - Limpa a tela
  info - Mostra informações do Aensh
  whoami - Mostra o usuário atual
  date - Mostra a data e hora atual

Use 'help' para ver esta mensagem novamente.
```

### `exit` / `quit`
Encerra o shell.

```bash
gabriel machine ~ ❯ exit
Até logo! 👋
```

## 🗂️ Navegação

### `cd <diretório>`
Altera o diretório atual. Use `~` para ir para o diretório home.

```bash
# Ir para /tmp
gabriel machine ~ ❯ cd /tmp
gabriel machine /tmp ❯ 

# Ir para home
gabriel machine /tmp ❯ cd ~
gabriel machine ~ ❯ 

# Ir para diretório relativo
gabriel machine ~ ❯ cd Documents
gabriel machine ~/Documents ❯ 
```

### `pwd`
Mostra o caminho completo do diretório atual.

```bash
gabriel machine ~ ❯ pwd
/home/gabriel
```

## 📁 Sistema de Arquivos

### `ls [diretório]`
Lista arquivos e diretórios com ícones e tamanhos.

```bash
gabriel machine ~ ❯ ls
📁 Documents/
📁 Downloads/
📄 README.md (2.5KB)
📄 script.sh (1.2KB)

# Listar outro diretório
gabriel machine ~ ❯ ls /tmp
📁 cache/
📄 temp.txt (0.5KB)
```

### `cat <arquivo>`
Exibe o conteúdo de um ou mais arquivos.

```bash
gabriel machine ~ ❯ cat README.md
# Aensh - A Modern Shell in Rust
...

# Exibir múltiplos arquivos
gabriel machine ~ ❯ cat file1.txt file2.txt
Conteúdo do file1.txt
Conteúdo do file2.txt
```

### `mkdir <diretório>`
Cria um novo diretório.

```bash
gabriel machine ~ ❯ mkdir meu_projeto
✓ Diretório meu_projeto criado

gabriel machine ~ ❯ mkdir dir1 dir2 dir3
✓ Diretório dir1 criado
✓ Diretório dir2 criado
✓ Diretório dir3 criado
```

### `touch <arquivo>`
Cria um arquivo vazio ou atualiza seu timestamp.

```bash
gabriel machine ~ ❯ touch novo.txt
✓ Arquivo novo.txt criado

gabriel machine ~ ❯ touch file1.txt file2.txt
✓ Arquivo file1.txt criado
✓ Arquivo file2.txt criado
```

### `rm <arquivo/diretório>`
Remove arquivos ou diretórios (recursivamente).

```bash
# Remover arquivo
gabriel machine ~ ❯ rm arquivo.txt
✓ arquivo.txt removido

# Remover diretório
gabriel machine ~ ❯ rm meu_projeto
✓ meu_projeto removido

# Remover múltiplos
gabriel machine ~ ❯ rm file1.txt file2.txt dir1
✓ file1.txt removido
✓ file2.txt removido
✓ dir1 removido
```

### `cp <origem> <destino>`
Copia arquivos ou diretórios.

```bash
# Copiar arquivo
gabriel machine ~ ❯ cp original.txt copia.txt
✓ original.txt copiado para copia.txt

# Copiar diretório
gabriel machine ~ ❯ cp -r projeto projeto_backup
✓ projeto copiado para projeto_backup
```

### `mv <origem> <destino>`
Move ou renomeia arquivos e diretórios.

```bash
# Renomear arquivo
gabriel machine ~ ❯ mv arquivo_antigo.txt arquivo_novo.txt
✓ arquivo_antigo.txt movido para arquivo_novo.txt

# Mover para outro diretório
gabriel machine ~ ❯ mv arquivo.txt /tmp/
✓ arquivo.txt movido para /tmp/
```

## ⚙️ Sistema

### `echo <texto>`
Exibe texto na tela.

```bash
gabriel machine ~ ❯ echo Olá, Aensh!
Olá, Aensh!

gabriel machine ~ ❯ echo "Texto com espaços"
Texto com espaços

gabriel machine ~ ❯ echo
# (linha vazia)
```

### `clear`
Limpa a tela do terminal.

```bash
gabriel machine ~ ❯ clear
# Tela limpa
gabriel machine ~ ❯ 
```

### `info`
Mostra informações sobre o Aensh.

```bash
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

### `whoami`
Mostra o usuário atual.

```bash
gabriel machine ~ ❯ whoami
gabriel
```

### `date`
Mostra a data e hora atual.

```bash
gabriel machine ~ ❯ date
02/12/2024 18:35:42 UTC 🕐
```

## 🎯 Exemplos Práticos

### Criar uma estrutura de projeto

```bash
gabriel machine ~ ❯ mkdir meu_projeto
✓ Diretório meu_projeto criado

gabriel machine ~ ❯ cd meu_projeto
gabriel machine ~/meu_projeto ❯ 

gabriel machine ~/meu_projeto ❯ mkdir src docs
✓ Diretório src criado
✓ Diretório docs criado

gabriel machine ~/meu_projeto ❯ touch README.md
✓ Arquivo README.md criado

gabriel machine ~/meu_projeto ❯ ls
📁 docs/
📁 src/
📄 README.md (0.0KB)
```

### Copiar e organizar arquivos

```bash
gabriel machine ~ ❯ touch file1.txt file2.txt file3.txt
✓ Arquivo file1.txt criado
✓ Arquivo file2.txt criado
✓ Arquivo file3.txt criado

gabriel machine ~ ❯ mkdir backup
✓ Diretório backup criado

gabriel machine ~ ❯ cp file1.txt backup/
✓ file1.txt copiado para backup/

gabriel machine ~ ❯ mv file2.txt backup/
✓ file2.txt movido para backup/
```

### Visualizar e editar arquivos

```bash
gabriel machine ~ ❯ echo "Conteúdo do arquivo" > arquivo.txt
Conteúdo do arquivo

gabriel machine ~ ❯ cat arquivo.txt
Conteúdo do arquivo
```

## ⚠️ Mensagens de Erro Comuns

### Comando não encontrado
```
✗ Erro: 'comando_invalido' não existe. Use 'help' para ver os comandos disponíveis.
```
**Solução:** Use `help` para ver os comandos disponíveis.

### Arquivo não encontrado
```
✗ Erro: não consigo abrir arquivo.txt: Arquivo ou diretório não encontrado
```
**Solução:** Verifique se o arquivo existe com `ls`.

### Permissão negada
```
✗ Erro: não consigo mudar para /root: Permissão negada
```
**Solução:** Você não tem permissão para acessar este diretório.

### Diretório não vazio
```
✗ Erro: não consigo remover diretório: Diretório não vazio
```
**Solução:** Use `rm` para remover o diretório e seu conteúdo.

## 💡 Dicas e Truques

1. **Use `~` para home:** `cd ~` leva você para o diretório home
2. **Múltiplos argumentos:** Muitos comandos aceitam múltiplos argumentos
3. **Veja a ajuda:** Use `help` sempre que tiver dúvidas
4. **Limpe a tela:** Use `clear` para melhor visualização
5. **Navegue com `cd`:** Use `cd ..` para subir um nível (não suportado ainda)

## 🔒 Segurança

O Aensh bloqueia automaticamente:
- Sequências perigosas: `&&`, `||`, `;`, `$()`
- Crases: `` ` ``

Isso garante que você não execute comandos perigosos acidentalmente.

## 📞 Precisa de Ajuda?

- Use `help` para ver todos os comandos
- Consulte a documentação em `docs/`
- Abra uma issue no repositório

---

**Última atualização:** Dezembro 2024
