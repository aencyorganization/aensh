# Documentação do Aensh

> "Cada comando tem o prefixo `a`, porque o Aensh não terceiriza identidade."

Esta pasta é o hub oficial de conhecimento do shell. Os conteúdos foram divididos em arquivos temáticos para facilitar desde consultas rápidas até estudos aprofundados.

## Mapa dos documentos

| Arquivo | Conteúdo |
|---------|----------|
| [overview.md](overview.md) | Visão geral, filosofia e pipeline de alto nível |
| [prompt-and-ui.md](prompt-and-ui.md) | Design do prompt, cores, banner Figlet e diretrizes visuais |
| [commands.md](commands.md) | Referência completa dos comandos nativos, parâmetros e fluxos de erro |
| [architecture.md](architecture.md) | Estrutura dos módulos em `src/`, ciclo de vida do comando e diagramas textuais |
| [errors.md](errors.md) | Tipos de erro (`AenshError`), mensagens exibidas e dicas de depuração |
| [development.md](development.md) | Como contribuir, adicionar comandos, manter estilo e testar |

## Como navegar

1. Leia `overview.md` para entender o propósito e os princípios do shell.
2. Consulte `prompt-and-ui.md` ao ajustar visual ou planejar temas.
3. Use `commands.md` durante o uso diário ou scripting.
4. Estude `architecture.md` + `errors.md` antes de modificar código.
5. Siga `development.md` para manter contribuições coerentes.

## Roadmap (alto nível)

- Histórico e navegação por setas
- Autocompletar
- Configurações externas (tema, prompt, validações)
- Pipes/redirecionamento nativos
- Suite de testes automatizados

Contribuições são bem-vindas. Abra issues ou PRs explicando motivação e impacto.
