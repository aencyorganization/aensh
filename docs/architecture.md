# Arquitetura do Aensh

## Visão por módulos (`src/`)

```
src/
├── banner.rs      # Banner Figlet e mensagem inicial
├── command.rs     # Estrutura `Command` (nome + args)
├── commands/      # Implementações dos comandos nativos
│   ├── clear.rs   # aclear
│   ├── exit_cmd.rs# aexit
│   ├── go.rs      # ago
│   ├── help.rs    # ahelp
│   ├── info.rs    # ainfo
│   ├── list.rs    # alist
│   ├── pwd.rs     # apwd
│   └── show.rs    # ashow
├── errors.rs      # `AenshError` e `AenshResult`
├── input.rs       # Sanitização e parsing de linhas
├── main.rs        # Loop principal, sinais e dispatcher
└── prompt.rs      # Construção do prompt colorido
```

## Ciclo de vida de um comando

1. `main.rs` lê a linha e passa para `input::parse_input`.
2. `parse_input`:
   - Remove espaços extras.
   - Bloqueia sequências perigosas (`&&`, `||`, `;`, `$(`, crases).
   - Valida se o comando está em `commands::SUPPORTED_COMMANDS`.
   - Retorna `Command { name, args }` via `AenshResult`.
3. `commands::dispatch` roteia para o módulo correto e executa `run`.
4. Cada `run` devolve `AenshResult<()>`. Erros são propagados e impressos por `main` usando `AenshError::print`.

## Tratamento de sinais

- `setup_signal_handlers` registra `SIGINT` e `SIGTERM` via `nix::sys::signal`.
- `SIGINT` apenas informa “Use 'aexit' para sair” e reinicia o handler.
- `SIGTERM` imprime "Encerrando..." e finaliza o processo.

## Dependências principais

| Crate | Uso |
|-------|-----|
| `colored` | Cores no prompt, banner e mensagens |
| `nix` | `chdir` e tratadores de sinal |
| `gethostname` | Nome da máquina |
| `libc` | Assinaturas de handlers |

## Extensibilidade

- Adicionar um comando novo exige apenas um módulo em `commands/` + atualização de `SUPPORTED_COMMANDS`.
- Novos tipos de erro devem ser adicionados em `errors.rs` para manter mensagens centralizadas.
- Componentes visuais (banner/prompt) ficam em arquivos isolados para facilitar customização sem tocar no loop principal.

## Fluxo textual

```
stdin -> parse_input -> Command -> dispatch -> comando/run -> stdout
                                     │
                                     └─> erro -> AenshError::print
```

Essa separação garante que nenhuma parte do shell execute chamadas externas ou dependa do shell do sistema.
