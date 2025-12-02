# Tratamento de Erros no Aensh

O Aensh centraliza erros no tipo `AenshError`, definido em `src/errors.rs`. Todos os comandos retornam `AenshResult<T> = Result<T, AenshError>`.

## Tipos de erro

| Variante | Quando acontece | Mensagem padrão |
|----------|-----------------|-----------------|
| `EmptyInput` | Usuário pressiona Enter sem digitar nada | "nenhum comando informado" |
| `Validation(String)` | Entrada contém sequências proibidas (`&&`, `||`, `;`, `$(`, crases) ou formato incorreto | `"comando inválido: ..."` |
| `InvalidCommand(String)` | Comando não está registrado em `SUPPORTED_COMMANDS` | `"'<cmd>' não existe no Aensh. Use 'ahelp'."` |
| `Io(String)` | Falhas de I/O (filesystem, stdout, `chdir`, etc.) | Mensagem propagada com contexto |

## Fluxo

1. `input::parse_input` retorna `AenshError` para entradas inválidas antes mesmo de gerar `Command`.
2. Cada comando em `src/commands/` retorna `AenshResult<()>`. Qualquer `std::io::Error` ou `nix::Error` vira `AenshError::Io` automaticamente (`From` implementado).
3. `main` captura esses erros e chama `err.print()`, que escreve `"Erro:"` em vermelho intenso + descrição amigável.

## Boas práticas ao adicionar erros

- Sempre forneça contexto em português: `format!("não consigo abrir {}: {}", caminho, erro)`.
- Prefira `AenshError::Validation` para entradas malformadas e `AenshError::Io` para falhas externas.
- Não exponha mensagens brutas do sistema sem explicação.
- Se criar uma nova categoria de erro, atualize esta doc e implemente `describe` para manter o padrão.

## Depuração

- Mensagens aparecem imediatamente após a linha digitada.
- Para investigar, rode `cargo run` e adicione `dbg!` temporários no comando afetado.
- Caso o erro venha de `parse_input`, habilite logs extras verificando o conteúdo de `raw` antes dos bloqueios.
