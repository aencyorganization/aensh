# Guia de Desenvolvimento do Aensh

## Setup

1. Requisitos: Rust 1.74+, Cargo, Linux (o shell usa `nix`).
2. Clone o repositório e rode `cargo build` para garantir dependências.
3. Use `cargo fmt` antes de abrir PR.

## Estrutura de código

- Cada módulo tem responsabilidade única (banner, prompt, input, etc.).
- Comandos ficam em `src/commands/` com um arquivo por comando.
- Erros centralizados em `errors.rs` – nunca imprima manualmente em `stderr` fora dele.

## Adicionando comandos

1. Crie `src/commands/<nome>.rs` com:
   ```rust
   use crate::errors::AenshResult;

   pub fn run(args: &[String]) -> AenshResult<()> {
       // lógica
       Ok(())
   }
   ```
2. Declare o módulo em `src/commands/mod.rs` e atualize `SUPPORTED_COMMANDS`.
3. A atualização automática de validação ocorrerá porque `input::parse_input` usa `is_supported`.
4. Documente o comando em `docs/commands.md`.

## Boas práticas

- **Erros**: retorne `AenshError` com contexto e mantenha PT-BR.
- **Cores**: use `colored` com parcimônia; consulte `prompt-and-ui.md`.
- **Mensagens**: mencione `ahelp` quando útil.
- **Segurança**: nunca invoque `std::process::Command`. O shell deve permanecer 100% interno.

## Testes manuais

1. `cargo run`
2. Exercite todos os comandos (help, go, pwd, list, show, clear, info, exit).
3. Teste entradas inválidas: `&&`, `||`, `;`, comandos sem `a`, crases.

## Roadmap técnico

- Criar suite de testes de integração com `assert_cmd`.
- Adicionar suporte a históricos e `readline` customizado.
- Implementar camada de configuração (arquivo TOML).

## Processo de contribuição

1. Abra issue descrevendo problema ou feature.
2. Faça fork/branch, siga este guia, escreva docs correspondentes.
3. Abra PR explicando mudanças, atualize `/docs`. Use screenshots ou gravações se mexer no visual.
