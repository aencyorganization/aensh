# 🚀 Quick Start - Aensh

Comece a usar o Aensh em 5 minutos!

## 1️⃣ Instalação

### Pré-requisitos
- Rust 1.70+
- Cargo

### Build

```bash
# Clone ou navegue até o diretório
cd aensh

# Build
cargo build --release

# Execute
./target/release/aensh
```

## 2️⃣ Primeiros Passos

```bash
# Ver ajuda
help

# Ver informações
info

# Ver usuário
whoami

# Ver data/hora
date
```

## 3️⃣ Navegação

```bash
# Ver diretório atual
pwd

# Mudar para home
cd ~

# Mudar para /tmp
cd /tmp

# Voltar para home
cd ~
```

## 4️⃣ Trabalhar com Arquivos

```bash
# Listar arquivos
ls

# Criar arquivo
touch meu_arquivo.txt

# Criar diretório
mkdir meu_diretorio

# Exibir arquivo
cat meu_arquivo.txt

# Copiar arquivo
cp meu_arquivo.txt copia.txt

# Mover arquivo
mv copia.txt meu_diretorio/

# Remover arquivo
rm meu_arquivo.txt

# Remover diretório
rm meu_diretorio
```

## 5️⃣ Buscar e Explorar

```bash
# Listar com estrutura em árvore
tree .

# Buscar arquivo
find . arquivo.txt

# Buscar padrão em arquivo
grep "padrão" arquivo.txt

# Ver informações de arquivo
stat arquivo.txt
```

## 6️⃣ Sistema

```bash
# Exibir texto
echo "Olá, Aensh!"

# Limpar tela
clear

# Sair
exit
```

## 📚 Comandos Disponíveis

### Shell
- `help` - Mostra ajuda
- `exit` / `quit` - Sair

### Navegação
- `cd` - Mudar diretório
- `pwd` - Diretório atual

### Arquivos
- `ls` - Listar
- `cat` - Exibir
- `mkdir` - Criar diretório
- `touch` - Criar arquivo
- `rm` - Remover
- `cp` - Copiar
- `mv` - Mover
- `find` - Buscar
- `grep` - Buscar padrão
- `tree` - Estrutura

### Sistema
- `echo` - Exibir texto
- `clear` - Limpar tela
- `info` - Informações
- `whoami` - Usuário
- `date` - Data/hora
- `stat` - Info de arquivo

## 🎯 Exemplo Prático

```bash
# 1. Criar projeto
mkdir meu_projeto
cd meu_projeto

# 2. Criar estrutura
mkdir src docs
touch README.md

# 3. Ver estrutura
tree .

# 4. Criar arquivo
touch src/main.rs

# 5. Listar
ls

# 6. Ver informações
stat README.md

# 7. Sair
exit
```

## 💡 Dicas

- Use `help` para ver todos os comandos
- Use `cd ~` para ir para home
- Use `clear` para limpar a tela
- Use `tree .` para ver estrutura
- Use `exit` para sair

## 📖 Documentação Completa

- `docs/USAGE.md` - Guia de uso completo
- `docs/EXAMPLES.md` - Exemplos práticos
- `docs/FAQ.md` - Perguntas frequentes
- `docs/DEVELOPMENT.md` - Desenvolvimento

## 🆘 Precisa de Ajuda?

```bash
# Ver ajuda
help

# Ver informações
info

# Ver exemplos
# Consulte docs/EXAMPLES.md
```

## ⚡ Próximas Funcionalidades

- Histórico de comandos
- Autocompletar
- Pipes e redirecionamento
- Variáveis de ambiente
- Aliases

---

**Pronto para começar?** Execute `./target/release/aensh` e divirta-se! 🎉
