# Changelog

Todas as mudanças notáveis neste projeto serão documentadas neste arquivo.

## [0.2.0] - 2024-12-02

### ✨ Adicionado
- **Novos comandos de sistema:**
  - `echo` - Exibe texto na tela
  - `whoami` - Mostra o usuário atual
  - `date` - Mostra a data e hora atual
  - `info` - Informações melhoradas do Aensh

- **Novos comandos de filesystem:**
  - `cat` - Exibe conteúdo de arquivos
  - `mkdir` - Cria diretórios
  - `touch` - Cria arquivos vazios
  - `rm` - Remove arquivos/diretórios
  - `cp` - Copia arquivos/diretórios
  - `mv` - Move/renomeia arquivos

- **Melhorias de interface:**
  - Ícones visuais em `ls` (📁 para diretórios, 📄 para arquivos)
  - Tamanho de arquivo exibido em `ls`
  - Prompt melhorado com símbolo `❯` em vez de `$`
  - Mensagens de sucesso com ✓
  - Mensagens de erro com ✗

- **Reorganização de código:**
  - Removido prefixo "a" de todos os comandos
  - Nova estrutura hierárquica em `src/builtins/`
  - Separação clara entre categorias de comandos
  - Módulo `core/` para funcionalidades principais

- **Documentação:**
  - README expandido com exemplos e arquitetura
  - Guia de desenvolvimento (DEVELOPMENT.md)
  - Changelog (este arquivo)

### 🔧 Alterado
- Renomeado `ago` → `cd`
- Renomeado `apwd` → `pwd`
- Renomeado `alist` → `ls`
- Renomeado `ashow` → `cat`
- Renomeado `aclear` → `clear`
- Renomeado `ahelp` → `help`
- Renomeado `aexit` → `exit`
- Renomeado `ainfo` → `info`
- Versão atualizada para 0.2.0

### 🎨 Melhorado
- Interface mais moderna e intuitiva
- Melhor organização de código
- Mensagens de erro mais descritivas
- Saída colorida mais consistente

### 🔒 Segurança
- Validação de entrada mantida
- Bloqueio de sequências perigosas
- Memory-safe através do Rust

## [0.1.0] - 2024-12-01

### ✨ Adicionado
- Implementação inicial do shell
- Comandos básicos com prefixo "a":
  - `ahelp` - Ajuda
  - `aexit` - Sair
  - `aclear` - Limpar tela
  - `ago` - Mudar diretório
  - `apwd` - Diretório atual
  - `alist` - Listar arquivos
  - `ashow` - Exibir arquivo
  - `ainfo` - Informações

- Funcionalidades principais:
  - Prompt colorido
  - Tratamento de sinais (SIGINT, SIGTERM)
  - Validação de entrada
  - Sistema de erros

- Dependências:
  - colored para colorização
  - nix para chamadas de sistema
  - gethostname para hostname
  - libc para bindings C

---

## Planejado para futuras versões

### v0.3.0
- [ ] Histórico de comandos
- [ ] Autocompletar com Tab
- [ ] Suporte a wildcards (*, ?, [])

### v0.4.0
- [ ] Pipes (|)
- [ ] Redirecionamento (>, >>, <)
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

**Nota:** As versões futuras podem ser ajustadas conforme o desenvolvimento progride.
