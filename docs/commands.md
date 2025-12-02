# Referência de Comandos do Aensh

Todos os comandos são internos e começam com `a`. Nenhum executável externo é invocado pelo shell.

| Comando | Parâmetros | Descrição | Erros principais |
|---------|------------|-----------|------------------|
| `ahelp` | — | Lista todos os comandos suportados com breve descrição. | — |
| `aexit` | — | Encerra imediatamente o shell com mensagem "Até logo!". | — |
| `aclear` | — | Limpa a tela enviando `ESC[2J ESC[1;1H`. | Falha ao dar `flush` no stdout. |
| `ago <dir>` | `<dir>` opcional (`~` por default) | Muda o diretório atual usando `nix::unistd::chdir`. | Diretório inexistente ou permissão negada. |
| `apwd` | — | Imprime o diretório atual colorido em ciano. | Falha ao ler diretório atual. |
| `alist [dir]` | Caminho opcional (default `.`) | Lista arquivos com ordenação alfabética. Diretórios aparecem em azul com `/`. | Caminho inexistente, erro de metadata, leitura de entrada. |
| `ashow <arquivo>` | Caminho obrigatório | Lê arquivo completo e imprime em branco. | Arquivo inexistente, permissão negada, leitura inválida. |
| `ainfo` | — | Mostra usuário, hostname e versão do Aensh. | Falha ao ler variáveis/env/hostname. |

## Convenções gerais

- Todos os argumentos são separados por espaço único (não há parsing de aspas).
- `~` é resolvido apenas em `ago`. Demais comandos tratam o texto literalmente.
- Mensagens de erro sempre são prefixadas por `Erro:` e explicam a causa em PT-BR.
- Use `ahelp` sempre que esquecer a sintaxe.

## Exemplos

```
ago ~/projetos/aensh
apwd
alist src
ashow docs/overview.md
ainfo
```

## Adicionando novos comandos

1. Crie um arquivo em `src/commands/<nome>.rs` expondo `pub fn run(args: &[String]) -> AenshResult<()>`.
2. Registre o módulo em `src/commands/mod.rs` e atualize `SUPPORTED_COMMANDS`.
3. Opcional: documente o comando adicionando uma linha no quadro acima e descrevendo parâmetros.
