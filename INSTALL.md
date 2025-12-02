# 📦 Guia de Instalação - Aensh

Instruções completas para instalar e usar o Aensh.

## 🔧 Pré-requisitos

### Obrigatórios
- **Rust 1.70+** - Linguagem de programação
- **Cargo** - Gerenciador de pacotes Rust
- **Git** - Controle de versão (opcional)

### Sistemas Suportados
- Linux (Ubuntu, Debian, Fedora, Arch, etc)
- macOS
- WSL (Windows Subsystem for Linux)

### Não Suportado
- ❌ Windows (nativo)
- ❌ Sistemas embarcados

## 📥 Instalação

### Opção 1: Build Local (Recomendado)

#### 1. Clonar o Repositório
```bash
git clone https://github.com/aencyorganization/aensh.git
cd aensh
```

#### 2. Build
```bash
# Build em modo debug (mais rápido)
cargo build

# Ou build em modo release (otimizado)
cargo build --release
```

#### 3. Executar
```bash
# Modo debug
./target/debug/aensh

# Modo release
./target/release/aensh
```

### Opção 2: Usar Script de Instalação

```bash
# Clonar repositório
git clone https://github.com/aencyorganization/aensh.git
cd aensh

# Executar script de instalação
chmod +x install.sh
./install.sh
```

### Opção 3: Instalar Globalmente

```bash
# Build em modo release
cargo build --release

# Copiar para /usr/local/bin
sudo cp target/release/aensh /usr/local/bin/

# Agora pode executar de qualquer lugar
aensh
```

## ✅ Verificar Instalação

```bash
# Verificar se Rust está instalado
rustc --version
cargo --version

# Verificar se Aensh foi compilado
./target/release/aensh --version  # (se implementado)
# ou
./target/release/aensh
# Deve mostrar o banner
```

## 🚀 Primeiros Passos

### 1. Executar o Aensh
```bash
./target/release/aensh
```

### 2. Ver Ajuda
```bash
help
```

### 3. Explorar Comandos
```bash
# Ver informações
info

# Ver usuário
whoami

# Ver data
date

# Listar arquivos
ls

# Sair
exit
```

## 📚 Documentação

Após instalar, consulte:

- **Quick Start:** `docs/QUICK_START.md` (5 min)
- **Guia Completo:** `docs/USAGE.md`
- **Exemplos:** `docs/EXAMPLES.md`
- **FAQ:** `docs/FAQ.md`

## 🔧 Troubleshooting

### Erro: "rustc not found"

**Solução:** Instale Rust
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

### Erro: "cargo not found"

**Solução:** Instale Cargo (vem com Rust)
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Erro: "permission denied"

**Solução:** Adicione permissão de execução
```bash
chmod +x target/release/aensh
```

### Erro: "command not found" (após instalar globalmente)

**Solução:** Atualize o PATH
```bash
export PATH="/usr/local/bin:$PATH"
```

### Build falha

**Solução:** Limpe e reconstrua
```bash
cargo clean
cargo build --release
```

## 🎯 Próximos Passos

1. Leia [QUICK_START.md](docs/QUICK_START.md)
2. Explore os [EXAMPLES.md](docs/EXAMPLES.md)
3. Consulte [USAGE.md](docs/USAGE.md) conforme necessário
4. Considere [contribuir](CONTRIBUTING.md)

## 📊 Requisitos de Sistema

### Mínimo
- 50 MB de espaço em disco (build)
- 5 MB de RAM (execução)
- Processador 64-bit

### Recomendado
- 200 MB de espaço em disco
- 512 MB de RAM
- Conexão com internet (para download de dependências)

## 🔄 Atualizar

```bash
# Ir para o diretório
cd aensh

# Atualizar código
git pull origin main

# Reconstruir
cargo build --release

# Executar
./target/release/aensh
```

## 🗑️ Desinstalar

### Se instalou globalmente
```bash
sudo rm /usr/local/bin/aensh
```

### Se apenas compilou localmente
```bash
# Simplesmente delete o diretório
rm -rf aensh
```

## 📞 Suporte

Se tiver problemas:

1. Consulte [FAQ.md](docs/FAQ.md)
2. Verifique [TROUBLESHOOTING](#-troubleshooting)
3. Abra uma issue no GitHub

## 🎓 Desenvolvimento

Se quer contribuir:

1. Leia [CONTRIBUTING.md](CONTRIBUTING.md)
2. Consulte [DEVELOPMENT.md](docs/DEVELOPMENT.md)
3. Explore [STRUCTURE.md](docs/STRUCTURE.md)

## 📄 Licença

Aensh está sob a licença MIT. Veja [LICENSE](LICENSE).

## 🚀 Começar Agora

```bash
# Clone
git clone https://github.com/aencyorganization/aensh.git
cd aensh

# Build
cargo build --release

# Execute
./target/release/aensh

# Divirta-se! 🎉
```

---

**Versão:** 0.2.0  
**Data:** Dezembro 2024  
**Status:** Pronto para usar ✅

Aproveite o Aensh!
