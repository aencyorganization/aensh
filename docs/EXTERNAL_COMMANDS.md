# 🔌 Comandos Externos no Aensh

## Como Funciona

O Aensh permite executar **qualquer comando do shell padrão** que não esteja bloqueado. O sistema funciona em 3 camadas:

### 1️⃣ **Comandos Built-in (Aensh)**
Comandos nativos do Aensh, implementados em Rust:
- `help`, `exit`, `quit`, `plugin`
- `cd`, `pwd`
- `ls`, `cat`, `mkdir`, `touch`, `rm`, `cp`, `mv`, `find`, `grep`, `tree`
- `echo`, `clear`, `info`, `whoami`, `date`, `stat`

### 2️⃣ **Plugins**
Comandos personalizados registrados pelo usuário:
```bash
plugin add myscript /path/to/script "Descrição"
```

### 3️⃣ **Comandos Externos**
Qualquer comando do shell padrão que não esteja bloqueado é executado normalmente:
```bash
aensh> python script.py
aensh> node app.js
aensh> curl https://example.com
aensh> docker ps
```

## 🚫 Comandos Bloqueados

Estes comandos **NÃO podem ser executados** no Aensh (para evitar conflitos):

### Navegação/Shell
- `cd` - Use o built-in do Aensh
- `pwd` - Use o built-in do Aensh
- `ls` - Use o built-in do Aensh
- `dir` - Use `ls`

### Shells
- `bash`, `sh`, `zsh`, `fish`, `dash`, `csh`, `tcsh`, `ksh`

### Gerenciamento de Processos
- `ps`, `top`, `htop`, `kill`, `killall`, `bg`, `fg`, `jobs`

### Gerenciamento de Permissões
- `sudo`, `su`, `chmod`, `chown`, `chgrp`

### Gerenciadores de Pacotes
- `apt`, `apt-get`, `yum`, `dnf`, `pacman`, `brew`, `snap`, `flatpak`

### Rede
- `ssh`, `scp`, `rsync`, `ping`, `netstat`, `ifconfig`, `ip`

### Outros Comandos Nativos
- `grep`, `find`, `sed`, `awk`, `cut`, `sort`, `uniq`, `wc`, `head`, `tail`
- `cat`, `cp`, `mv`, `rm`, `mkdir`, `rmdir`, `touch`
- `echo`, `printf`, `read`, `export`, `unset`, `alias`, `unalias`
- `source`, `.`, `exec`, `eval`, `set`, `shift`

## ✅ Comandos Permitidos (Exemplos)

```bash
# Desenvolvimento
aensh> python script.py
aensh> node app.js
aensh> ruby script.rb
aensh> go run main.go
aensh> cargo build

# Ferramentas
aensh> curl https://api.example.com
aensh> wget https://example.com/file.zip
aensh> git clone https://github.com/user/repo
aensh> docker ps
aensh> docker run ubuntu

# Compiladores
aensh> gcc main.c -o main
aensh> javac Main.java
aensh> rustc main.rs

# Editores
aensh> vim file.txt
aensh> nano file.txt
aensh> code .

# Utilitários
aensh> zip archive.zip file.txt
aensh> tar -czf archive.tar.gz folder/
aensh> ffmpeg -i video.mp4 audio.mp3
aensh> imagemagick convert image.png image.jpg
```

## 🔗 Piping com Comandos Externos

Você pode fazer piping entre comandos built-in, plugins e externos:

```bash
# Builtin | Externo
aensh> echo "Hello World" | wc -w
2

# Externo | Builtin
aensh> cat /etc/passwd | grep root

# Externo | Externo
aensh> curl https://api.example.com | jq .

# Builtin | Externo | Builtin
aensh> echo "data" | python process.py | cat
```

## 🔀 Encadeamento com &&

Encadeie comandos que devem executar sequencialmente:

```bash
# Builtin && Externo
aensh> cd /tmp && python script.py

# Externo && Builtin
aensh> cargo build && echo "Build completo!"

# Externo && Externo
aensh> git pull && npm install && npm start
```

## 📋 Verificar Disponibilidade de Comando

Para saber se um comando está bloqueado:

```bash
# Tente executar
aensh> comando_teste
✗ Erro: 'comando_teste' é um comando nativo do shell e não pode ser executado

# Se receber este erro, o comando está bloqueado
# Caso contrário, será executado normalmente
```

## 🎯 Casos de Uso

### Desenvolvimento Web
```bash
aensh> npm install
aensh> npm start
aensh> curl http://localhost:3000
```

### Data Science
```bash
aensh> python train.py
aensh> jupyter notebook
aensh> pip install pandas
```

### DevOps
```bash
aensh> docker build -t myapp .
aensh> docker run -p 8080:8080 myapp
aensh> kubectl apply -f deployment.yaml
```

### Processamento de Arquivos
```bash
aensh> ffmpeg -i video.mp4 -c:v libx264 output.mp4
aensh> imagemagick convert image.png image.jpg
aensh> pandoc document.md -o document.pdf
```

## 🔐 Por Que Alguns Comandos São Bloqueados?

1. **Evitar Conflitos** - Comandos como `cd` têm implementação especial no Aensh
2. **Segurança** - `sudo` e `su` poderiam contornar restrições
3. **Consistência** - Gerenciadores de pacotes variam entre sistemas
4. **Clareza** - Deixa claro que você está usando o Aensh, não bash/zsh

## 💡 Dica: Usar Aliases

Se quiser usar um comando bloqueado, crie um plugin:

```bash
# Criar script em ~/.config/aensh/plugins/mypwd
#!/bin/bash
pwd

# Tornar executável
chmod +x ~/.config/aensh/plugins/mypwd

# Usar no Aensh
aensh> mypwd
```

## 📚 Mais Informações

- Ver lista de built-ins: `help`
- Gerenciar plugins: `plugin help`
- Ver comandos bloqueados: Consulte `src/core/plugins.rs`

---

**Versão:** 0.1.0 | **Última atualização:** Dezembro 2024
