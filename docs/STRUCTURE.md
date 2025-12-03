# 🏗️ Estrutura do Projeto Aensh

## 📂 Estrutura de Diretórios

```
aensh/
├── src/                          # Código-fonte Rust
│   ├── main.rs                   # Ponto de entrada principal
│   ├── core/                     # Módulos principais do shell
│   │   ├── mod.rs               # Re-exportações
│   │   ├── banner.rs            # Banner de boas-vindas
│   │   ├── command.rs           # Estrutura Command
│   │   ├── errors.rs            # Sistema de erros
│   │   ├── input.rs             # Parser de entrada
│   │   └── prompt.rs            # Construtor de prompt
│   └── builtins/                # Comandos built-in
│       ├── mod.rs               # Dispatcher principal
│       ├── shell/               # Comandos de shell
│       │   ├── mod.rs
│       │   ├── help.rs          # Comando help
│       │   └── exit.rs          # Comando exit
│       ├── navigation/          # Navegação
│       │   ├── mod.rs
│       │   ├── cd.rs            # Comando cd
│       │   └── pwd.rs           # Comando pwd
│       ├── filesystem/          # Sistema de arquivos
│       │   ├── mod.rs
│       │   ├── ls.rs            # Listar arquivos
│       │   ├── cat.rs           # Exibir arquivo
│       │   ├── mkdir.rs         # Criar diretório
│       │   ├── touch.rs         # Criar arquivo
│       │   ├── rm.rs            # Remover arquivo
│       │   ├── cp.rs            # Copiar arquivo
│       │   ├── mv.rs            # Mover arquivo
│       │   ├── find.rs          # Buscar arquivos
│       │   ├── grep.rs          # Buscar padrões
│       │   └── tree.rs          # Estrutura em árvore
│       └── system/              # Comandos de sistema
│           ├── mod.rs
│           ├── echo.rs          # Ecoar texto
│           ├── clear.rs         # Limpar tela
│           ├── info.rs          # Informações
│           ├── whoami.rs        # Usuário atual
│           ├── date.rs          # Data e hora
│           └── stat.rs          # Informações de arquivo
├── docs/                         # Documentação
│   ├── README.md                # Documentação principal
│   ├── DEVELOPMENT.md           # Guia de desenvolvimento
│   ├── USAGE.md                 # Guia de uso
│   ├── EXAMPLES.md              # Exemplos práticos
│   ├── FAQ.md                   # Perguntas frequentes
│   ├── CHANGELOG.md             # Histórico de mudanças
│   └── STRUCTURE.md             # Este arquivo
├── target/                       # Artefatos de build
│   ├── debug/                   # Build debug
│   └── release/                 # Build release
├── Cargo.toml                    # Configuração do projeto
├── Cargo.lock                    # Lock de dependências
├── README.md                     # Documentação principal
├── CONTRIBUTING.md              # Guia de contribuição
├── LICENSE                       # Licença MIT
├── .gitignore                    # Arquivo gitignore
└── install.sh                    # Script de instalação
```

## 📊 Hierarquia de Módulos

```
aensh (crate raiz)
├── core
│   ├── banner
│   ├── command
│   ├── errors
│   ├── input
│   └── prompt
└── builtins
    ├── shell
    │   ├── help
    │   └── exit
    ├── navigation
    │   ├── cd
    │   └── pwd
    ├── filesystem
    │   ├── ls
    │   ├── cat
    │   ├── mkdir
    │   ├── touch
    │   ├── rm
    │   ├── cp
    │   ├── mv
    │   ├── find
    │   ├── grep
    │   └── tree
    └── system
        ├── echo
        ├── clear
        ├── info
        ├── whoami
        ├── date
        └── stat
```

## 🔄 Fluxo de Execução

```
main.rs
  ↓
setup_signal_handlers()
  ↓
show_banner() [core::banner]
  ↓
LOOP:
  ├─ build_prompt() [core::prompt]
  ├─ read_input()
  ├─ parse_input() [core::input]
  │   └─ is_supported() [builtins]
  ├─ dispatch() [builtins]
  │   └─ Executa comando específico
  └─ Volta ao LOOP
```

## 📦 Dependências

```toml
[dependencies]
nix = "0.27"           # Chamadas de sistema POSIX
libc = "0.2"           # Bindings C
colored = "2.1"        # Colorização de saída
gethostname = "0.4"    # Obtenção de hostname
```

## 🎯 Categorias de Comandos

### Shell (2 comandos)
- `help` - Mostra ajuda
- `exit` / `quit` - Sair

### Navigation (2 comandos)
- `cd` - Mudar diretório
- `pwd` - Diretório atual

### Filesystem (10 comandos)
- `ls` - Listar arquivos
- `cat` - Exibir arquivo
- `mkdir` - Criar diretório
- `touch` - Criar arquivo
- `rm` - Remover arquivo
- `cp` - Copiar arquivo
- `mv` - Mover arquivo
- `find` - Buscar arquivos
- `grep` - Buscar padrões
- `tree` - Estrutura em árvore

### System (6 comandos)
- `echo` - Ecoar texto
- `clear` - Limpar tela
- `info` - Informações
- `whoami` - Usuário atual
- `date` - Data e hora
- `stat` - Informações de arquivo

**Total: 20 comandos**

## 📈 Estatísticas

| Métrica | Valor |
|---------|-------|
| Linhas de código | ~2000+ |
| Módulos | 20+ |
| Comandos | 20 |
| Dependências | 4 |
| Versão | 0.2.0 |

## 🔐 Segurança

- ✅ Memory-safe (Rust)
- ✅ Validação de entrada
- ✅ Bloqueio de sequências perigosas
- ✅ Tratamento de sinais
- ✅ Sem execução de código arbitrário

## 🎨 Interface

- ✅ Prompt colorido
- ✅ Ícones visuais (emojis)
- ✅ Mensagens de erro claras
- ✅ Feedback visual

## 📚 Documentação

| Arquivo | Conteúdo |
|---------|----------|
| README.md | Visão geral e features |
| DEVELOPMENT.md | Guia de desenvolvimento |
| USAGE.md | Guia de uso completo |
| EXAMPLES.md | Exemplos práticos |
| FAQ.md | Perguntas frequentes |
| CHANGELOG.md | Histórico de mudanças |
| CONTRIBUTING.md | Guia de contribuição |
| STRUCTURE.md | Este arquivo |

## 🚀 Próximas Melhorias

### v0.3.0
- [ ] Histórico de comandos
- [ ] Autocompletar com Tab
- [ ] Suporte a wildcards

### v0.4.0
- [ ] Pipes (|)
- [ ] Redirecionamento (>, >>)
- [ ] Variáveis de ambiente

### v0.5.0
- [ ] Aliases de comandos
- [ ] Scripts shell
- [ ] Modo batch

### v0.1.0
- [ ] Suporte a jobs
- [ ] Modo interativo completo
- [ ] Configuração customizável
- [ ] Temas de cores

---

**Última atualização:** Dezembro 2024

Para mais informações, consulte os arquivos de documentação em `docs/`.
