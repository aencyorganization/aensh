# Visão Geral do Aensh

```
    _                  _     
   / \   ___ _ __  ___| |__  
  / _ \ / _ \ '_ \/ __| '_ \ 
 / ___ \  __/ | | \__ \ | | |
/_/   \_\___|_| |_|___/_| |_|
```

O **Aensh** é um shell 100% escrito em Rust com foco em:

- **Identidade própria** – todos os comandos começam com `a` e são implementados internamente.
- **Segurança** – sanitização agressiva impede encadeamentos, substituições e comandos perigosos.
- **Experiência visual** – cores consistentes, banner Figlet, mensagens amigáveis em PT-BR.
- **Modularidade** – cada comando, componente de prompt e manipulador de erros fica em um arquivo dedicado.

## Pipeline simplificado

1. Banner Figlet e mensagem de boas-vindas.
2. Configuração dos tratadores SIGINT/SIGTERM.
3. Leitura + sanitização da entrada em `input::parse_input`.
4. Conversão para `Command` (nome + argumentos).
5. Despacho em `commands::dispatch`, que executa apenas comandos internos.
6. Retorno de `AenshResult`, impresso por `errors::AenshError::print`.

## Filosofia

| Princípio      | Descrição |
|----------------|-----------|
| Segurança      | Sem execuções externas, sem subshells, sem comandos do sistema. |
| Clareza        | Mensagens em português, instruções claras e cores consistentes. |
| Extensibilidade| Novos comandos entram adicionando um módulo em `src/commands`. |
| UX amigável    | Prompt legível, separador simples, feedback imediato. |

Para detalhes adicionais, consulte os demais arquivos dentro de `/docs`.
