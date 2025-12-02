# 🗺️ Roadmap - Aensh

Visão de longo prazo e planejamento futuro do Aensh.

## 📊 Versões Planejadas

### ✅ v0.2.0 - Refatoração e Expansão (Atual)
**Status:** Lançado ✓

**Mudanças:**
- ✅ Remoção do prefixo "a"
- ✅ Reorganização de diretórios
- ✅ 12 novos comandos
- ✅ Documentação completa
- ✅ Melhorias de UI

**Comandos:** 20

---

### 🔄 v0.3.0 - Interatividade (Q1 2025)
**Status:** Planejado

**Objetivos:**
- [ ] Histórico de comandos
- [ ] Autocompletar com Tab
- [ ] Suporte a wildcards (*, ?, [])
- [ ] Melhorias de performance

**Novos Comandos:**
- [ ] `history` - Ver histórico
- [ ] `clear-history` - Limpar histórico

**Melhorias:**
- [ ] Busca no histórico (Ctrl+R)
- [ ] Navegação com setas
- [ ] Edição de linha

**Estimativa:** 4-6 semanas

---

### 🔗 v0.4.0 - Pipes e Redirecionamento (Q2 2025)
**Status:** Planejado

**Objetivos:**
- [ ] Pipes (|)
- [ ] Redirecionamento (>, >>)
- [ ] Entrada padrão (<)
- [ ] Variáveis de ambiente

**Novos Comandos:**
- [ ] `export` - Definir variáveis
- [ ] `unset` - Remover variáveis
- [ ] `env` - Listar variáveis

**Exemplos:**
```bash
# Pipes
ls | grep .txt

# Redirecionamento
echo "texto" > arquivo.txt
cat arquivo.txt >> outro.txt

# Variáveis
export VAR=valor
echo $VAR
```

**Estimativa:** 6-8 semanas

---

### 🎯 v0.5.0 - Scripting (Q3 2025)
**Status:** Planejado

**Objetivos:**
- [ ] Aliases de comandos
- [ ] Scripts shell
- [ ] Modo batch
- [ ] Funções

**Novos Comandos:**
- [ ] `alias` - Criar alias
- [ ] `unalias` - Remover alias
- [ ] `source` - Executar script

**Exemplos:**
```bash
# Alias
alias ll="ls -la"

# Script
#!/bin/aensh
echo "Olá"
cd /tmp
ls

# Função
function backup() {
    cp -r . backup/
}
```

**Estimativa:** 8-10 semanas

---

### 🚀 v1.0.0 - Produção (Q4 2025)
**Status:** Planejado

**Objetivos:**
- [ ] Suporte a jobs
- [ ] Modo interativo completo
- [ ] Configuração customizável
- [ ] Temas de cores
- [ ] Performance otimizada

**Novos Comandos:**
- [ ] `jobs` - Listar jobs
- [ ] `bg` - Background
- [ ] `fg` - Foreground
- [ ] `config` - Configuração

**Melhorias:**
- [ ] Arquivo de configuração (~/.aensh/config)
- [ ] Temas customizáveis
- [ ] Plugins
- [ ] Performance crítica

**Estimativa:** 12-16 semanas

---

## 🎯 Objetivos por Categoria

### Curto Prazo (v0.3.0)
- ✅ Histórico de comandos
- ✅ Autocompletar
- ✅ Wildcards

### Médio Prazo (v0.4.0 - v0.5.0)
- ✅ Pipes e redirecionamento
- ✅ Variáveis de ambiente
- ✅ Aliases
- ✅ Scripts

### Longo Prazo (v1.0.0+)
- ✅ Jobs
- ✅ Configuração
- ✅ Temas
- ✅ Plugins

## 📈 Crescimento Esperado

```
v0.1.0 (Inicial)
├─ 8 comandos
├─ 1 módulo
└─ Documentação mínima

v0.2.0 (Atual)
├─ 20 comandos (+150%)
├─ 20+ módulos (+1900%)
└─ Documentação completa

v0.3.0 (Próximo)
├─ 22 comandos (+10%)
├─ Histórico
└─ Autocompletar

v0.4.0
├─ 25 comandos (+13%)
├─ Pipes
├─ Redirecionamento
└─ Variáveis

v0.5.0
├─ 28 comandos (+12%)
├─ Aliases
├─ Scripts
└─ Funções

v1.0.0
├─ 35+ comandos (+25%)
├─ Jobs
├─ Configuração
└─ Temas
```

## 🔧 Tecnologia

### Dependências Atuais
- nix 0.27
- libc 0.2
- colored 2.1
- gethostname 0.4

### Dependências Futuras (Planejadas)
- `rustyline` - Histórico e edição (v0.3.0)
- `regex` - Wildcards (v0.3.0)
- `toml` - Configuração (v1.0.0)
- `serde` - Serialização (v1.0.0)

## 🎓 Aprendizado

Cada versão adiciona novos conceitos:

- **v0.2.0:** Organização modular, UI
- **v0.3.0:** Histórico, autocompletar
- **v0.4.0:** Pipes, variáveis
- **v0.5.0:** Scripting, funções
- **v1.0.0:** Jobs, configuração

## 📋 Checklist de Funcionalidades

### Essencial (v0.2.0)
- ✅ Comandos básicos
- ✅ Navegação
- ✅ Arquivos
- ✅ Sistema

### Importante (v0.3.0 - v0.4.0)
- ⏳ Histórico
- ⏳ Autocompletar
- ⏳ Pipes
- ⏳ Redirecionamento

### Desejável (v0.5.0 - v1.0.0)
- ⏳ Aliases
- ⏳ Scripts
- ⏳ Jobs
- ⏳ Configuração

## 🤝 Contribuições Bem-Vindas

Áreas onde contribuições são especialmente bem-vindas:

### Fácil
- [ ] Documentação
- [ ] Exemplos
- [ ] Testes

### Médio
- [ ] Novos comandos
- [ ] Melhorias de UI
- [ ] Performance

### Difícil
- [ ] Histórico
- [ ] Autocompletar
- [ ] Pipes
- [ ] Scripting

## 📞 Feedback

Sua opinião é importante! Abra uma issue para:
- Sugerir funcionalidades
- Reportar bugs
- Fazer perguntas
- Compartilhar ideias

## 🎯 Visão Final

Transformar o Aensh em um shell completo, moderno e fácil de usar, mantendo a segurança, performance e educabilidade como prioridades.

### Metas Principais
1. **Segurança** - Memory-safe, validação de entrada
2. **Performance** - Rápido e eficiente
3. **Usabilidade** - Intuitivo e bem documentado
4. **Extensibilidade** - Fácil adicionar funcionalidades
5. **Educação** - Código limpo e bem estruturado

## 📅 Timeline Estimada

```
2024 Q4 ✅ v0.2.0 - Refatoração
2025 Q1 ⏳ v0.3.0 - Interatividade
2025 Q2 ⏳ v0.4.0 - Pipes
2025 Q3 ⏳ v0.5.0 - Scripting
2025 Q4 ⏳ v1.0.0 - Produção
```

## 🚀 Começar a Contribuir

1. Escolha uma funcionalidade do roadmap
2. Abra uma issue para discutir
3. Faça um fork e crie uma branch
4. Implemente a funcionalidade
5. Envie um Pull Request

Veja [CONTRIBUTING.md](CONTRIBUTING.md) para mais detalhes.

---

**Última atualização:** Dezembro 2024  
**Versão atual:** 0.2.0  
**Próxima versão:** 0.3.0 (Q1 2025)

Obrigado por acompanhar o desenvolvimento do Aensh! 🚀
