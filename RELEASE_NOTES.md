# 📢 Release Notes - Aensh v0.2.0

**Data de Lançamento:** Dezembro 2, 2024

## 🎉 Principais Mudanças

### ✨ Novos Comandos (12 novos)

#### Sistema de Arquivos
- **`find`** - Busca arquivos em diretórios recursivamente
- **`grep`** - Busca padrões em arquivos com destaque
- **`tree`** - Mostra estrutura de diretórios em árvore visual

#### Sistema
- **`stat`** - Mostra informações detalhadas de arquivos/diretórios
- **`whoami`** - Mostra o usuário atual
- **`date`** - Mostra data e hora atual
- **`echo`** - Exibe texto na tela
- **`info`** - Informações melhoradas do Aensh

#### Expandidos
- **`ls`** - Agora mostra ícones e tamanhos
- **`cat`** - Suporta múltiplos arquivos
- **`mkdir`** - Suporta múltiplos diretórios
- **`touch`** - Suporta múltiplos arquivos

### 🔄 Refatoração Completa

#### Remoção do Prefixo "a"
- `ahelp` → `help`
- `aexit` → `exit`
- `aclear` → `clear`
- `ago` → `cd`
- `apwd` → `pwd`
- `alist` → `ls`
- `ashow` → `cat`
- `ainfo` → `info`

#### Nova Estrutura de Diretórios
```
src/
├── core/          # Módulos principais
├── builtins/      # Comandos built-in
│   ├── shell/     # Comandos de shell
│   ├── navigation/# Navegação
│   ├── filesystem/# Sistema de arquivos
│   └── system/    # Sistema
```

### 🎨 Melhorias de Interface

- ✅ Prompt melhorado com símbolo `❯`
- ✅ Ícones visuais em `ls` (📁 📄)
- ✅ Tamanho de arquivo em `ls`
- ✅ Mensagens de sucesso com ✓
- ✅ Mensagens de erro com ✗
- ✅ Cores mais consistentes
- ✅ Formatação melhorada

### 📚 Documentação Expandida

Novos arquivos de documentação:
- `docs/QUICK_START.md` - Guia rápido
- `docs/USAGE.md` - Guia de uso completo
- `docs/EXAMPLES.md` - Exemplos práticos
- `docs/FAQ.md` - Perguntas frequentes
- `docs/DEVELOPMENT.md` - Guia de desenvolvimento
- `docs/STRUCTURE.md` - Estrutura do projeto
- `docs/CHANGELOG.md` - Histórico de mudanças
- `CONTRIBUTING.md` - Guia de contribuição
- `LICENSE` - Licença MIT
- `RELEASE_NOTES.md` - Este arquivo

### 🏗️ Melhorias de Arquitetura

- Separação clara de responsabilidades
- Módulos bem organizados
- Código mais manutenível
- Melhor reutilização de código
- Sistema de erros consistente

## 📊 Estatísticas

| Métrica | v0.1.0 | v0.2.0 | Mudança |
|---------|--------|--------|---------|
| Comandos | 8 | 20 | +150% |
| Módulos | 1 | 20+ | +1900% |
| Linhas de código | ~500 | ~2000+ | +300% |
| Documentação | Mínima | Completa | ✅ |

## 🔒 Segurança

- ✅ Memory-safe (Rust)
- ✅ Validação de entrada
- ✅ Bloqueio de sequências perigosas
- ✅ Tratamento de sinais
- ✅ Sem execução de código arbitrário

## 🚀 Performance

- ✅ Compilado para código nativo
- ✅ Tempo de inicialização < 1ms
- ✅ Uso de memória < 5MB
- ✅ Operações rápidas

## 🐛 Correções de Bugs

- Melhor tratamento de erros
- Mensagens de erro mais claras
- Validação de argumentos melhorada

## 📝 Notas de Compatibilidade

### Breaking Changes
- Prefixo "a" removido de todos os comandos
- Estrutura de diretórios reorganizada

### Migração de v0.1.0
Se você estava usando v0.1.0, atualize seus comandos:
```bash
# Antigo → Novo
ahelp   → help
aexit   → exit
aclear  → clear
ago     → cd
apwd    → pwd
alist   → ls
ashow   → cat
ainfo   → info
```

## 🎯 Próximas Versões

### v0.3.0 (Planejado)
- [ ] Histórico de comandos
- [ ] Autocompletar com Tab
- [ ] Suporte a wildcards

### v0.4.0 (Planejado)
- [ ] Pipes (|)
- [ ] Redirecionamento (>, >>)
- [ ] Variáveis de ambiente

### v0.5.0 (Planejado)
- [ ] Aliases de comandos
- [ ] Scripts shell
- [ ] Modo batch

### v1.0.0 (Planejado)
- [ ] Suporte a jobs
- [ ] Modo interativo completo
- [ ] Configuração customizável
- [ ] Temas de cores

## 📥 Como Atualizar

```bash
# Pull das mudanças
git pull origin main

# Build
cargo build --release

# Execute
./target/release/aensh
```

## 🙏 Agradecimentos

Obrigado a todos que contribuíram para esta versão!

## 📞 Suporte

- 📖 Documentação: `docs/`
- ❓ FAQ: `docs/FAQ.md`
- 🐛 Bugs: Abra uma issue
- 💡 Sugestões: Abra uma issue

## 📄 Licença

Aensh está sob a licença MIT. Veja `LICENSE` para mais detalhes.

---

**Versão:** 0.2.0  
**Data:** Dezembro 2, 2024  
**Status:** Estável ✅

Aproveite o novo Aensh! 🚀
