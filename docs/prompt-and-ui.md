# Prompt, Banner e UI

## Banner Figlet

O Aensh inicia sempre exibindo o logotipo abaixo:

```
    _                  _     
   / \   ___ _ __  ___| |__  
  / _ \ / _ \ '_ \/ __| '_ \ 
 / ___ \  __/ | | \__ \ | | |
/_/   \_\___|_| |_|___/_| |_|
```

- O banner é renderizado em **magenta brilhante** com `bold`.
- Logo após, uma mensagem introdutória branca lembra o comando `ahelp`.

## Prompt

Formato geral:
```
<user> <host> <path> $ 
```

| Segmento | Cor/Estilo | Detalhes |
|----------|------------|----------|
| Usuário  | Verde brilhante + negrito | `USER` da sessão |
| Hostname | Magenta brilhante + negrito | Obtido via `gethostname` |
| Caminho  | Ciano brilhante + negrito | Diretório atual absoluto |
| Sufixo   | Amarelo brilhante + negrito | Sempre `$`, com um espaço final |

### Regras visuais

1. **Separador único**: componentes separados por um único espaço.
2. **Sem prompt alternativo**: não há `:` nem `@`, para reforçar o estilo próprio.
3. **Contraste consistente**: evite alterar as cores base; se necessário, atualize esta doc e `prompt.rs` juntos.
4. **Espaço final obrigatório** para permitir digitação alinhada.

## Saídas coloridas

- Diretórios listados: azul brilhante com `/` ao final.
- Arquivos: branco brilhante.
- Texto informativo: branco/ciano.
- Avisos: amarelo.
- Erros: vermelho intenso com prefixo `Erro:`.

## Limpeza de tela

`aclear` envia `\x1B[2J\x1B[1;1H` e força `flush`. Não existe fallback para comandos externos (`clear`).

## Guidelines de UX

1. **Português claro** – todas as mensagens, incluindo erros, devem ser PT-BR.
2. **Sem gírias técnicas obscuras** – prefira frases que orientem o usuário.
3. **Feedback imediato** – a cada erro, explique o porquê e sugira `ahelp` se fizer sentido.
4. **Consistência** – qualquer mudança de cor ou texto deve ser replicada nos módulos relevantes e documentada aqui.
