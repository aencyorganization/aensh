# ❓ Perguntas Frequentes (FAQ)

## Geral

### O que é Aensh?
Aensh é um shell moderno implementado em Rust, construído do zero para demonstrar conceitos de programação de sistemas com foco em segurança, performance e usabilidade.

### Por que Rust?
Rust oferece segurança de memória, performance nativa, tooling moderno e type safety, tornando-o ideal para programação de sistemas.

### Aensh é um shell completo como Bash?
Não, Aensh é um shell educacional com funcionalidades essenciais. Ele não suporta pipes, redirecionamento ou scripts ainda, mas essas são melhorias planejadas.

### Posso usar Aensh como meu shell principal?
Atualmente não é recomendado, pois faltam funcionalidades essenciais como pipes e redirecionamento. Use para aprender ou experimentar.

## Instalação e Execução

### Como instalo o Aensh?
```bash
# Clone o repositório
git clone https://github.com/gabriel/aensh.git
cd aensh

# Build
cargo build --release

# Execute
./target/release/aensh

# Ou use o script de instalação
chmod +x install.sh
./install.sh
```

### Quais são os requisitos?
- Rust 1.70 ou superior
- Cargo
- Um sistema Unix-like (Linux, macOS, etc)

### Como saio do Aensh?
Use `exit` ou `quit`:
```bash
gabriel machine ~ ❯ exit
Até logo! 👋
```

## Comandos

### Como vejo todos os comandos disponíveis?
Use `help`:
```bash
gabriel machine ~ ❯ help
```

### Por que meu comando não funciona?
Verifique:
1. Se o comando existe com `help`
2. Se a sintaxe está correta
3. Se os argumentos são válidos

### Posso usar pipes (|)?
Não, pipes ainda não são suportados. Essa é uma melhoria planejada para v0.4.0.

### Posso usar redirecionamento (>, >>)?
Não, redirecionamento também não é suportado ainda.

### Como uso variáveis de ambiente?
Variáveis de ambiente ainda não são suportadas. Essa é uma melhoria planejada para v0.4.0.

### Posso criar aliases?
Não, aliases ainda não são suportados. Planejado para v0.5.0.

## Funcionalidades

### Como listo arquivos com mais detalhes?
Use `ls` para listar com ícones e tamanhos:
```bash
gabriel machine ~ ❯ ls
📁 Documents/
📄 README.md (2.5KB)
```

### Como busco um arquivo?
Use `find`:
```bash
gabriel machine ~ ❯ find . arquivo.txt
```

### Como busco um padrão em um arquivo?
Use `grep`:
```bash
gabriel machine ~ ❯ grep "padrão" arquivo.txt
```

### Como vejo a estrutura de diretórios?
Use `tree`:
```bash
gabriel machine ~ ❯ tree .
```

### Como vejo informações de um arquivo?
Use `stat`:
```bash
gabriel machine ~ ❯ stat arquivo.txt
```

## Segurança

### Por que algumas sequências são bloqueadas?
O Aensh bloqueia sequências perigosas (`&&`, `||`, `;`, `$()`) para evitar execução de código arbitrário.

### É seguro usar Aensh?
Sim, Aensh é escrito em Rust, que garante segurança de memória em tempo de compilação. Além disso, há validação de entrada.

### Posso executar scripts?
Não, scripts shell ainda não são suportados. Planejado para v0.5.0.

## Desenvolvimento

### Como contribuo?
1. Fork o repositório
2. Crie uma branch para sua feature
3. Commit suas mudanças
4. Envie um Pull Request

Veja `docs/DEVELOPMENT.md` para mais detalhes.

### Como adiciono um novo comando?
Veja `docs/DEVELOPMENT.md` para um guia passo a passo.

### Quais são as próximas funcionalidades?
Veja `docs/CHANGELOG.md` para o roadmap.

## Troubleshooting

### O Aensh não compila
Verifique se você tem Rust 1.70+ instalado:
```bash
rustc --version
```

Se não, instale com:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Recebo erro "comando não encontrado"
Verifique se o comando existe com `help`.

### Recebo erro "permissão negada"
Você não tem permissão para acessar esse arquivo/diretório. Verifique as permissões.

### Aensh trava
Pressione Ctrl+C. Se não funcionar, abra uma issue.

### Tenho outro problema
Abra uma issue no repositório com:
- Descrição do problema
- Passos para reproduzir
- Saída de erro
- Versão do Aensh (`info`)

## Performance

### Aensh é rápido?
Sim, Aensh é compilado para código nativo e é geralmente mais rápido que shells em Python ou Ruby.

### Quanto de memória Aensh usa?
Aensh usa muito pouca memória, geralmente menos de 5MB.

## Compatibilidade

### Aensh funciona no Windows?
Não, Aensh é específico para Unix-like systems (Linux, macOS, etc).

### Aensh funciona no macOS?
Sim, Aensh funciona em macOS.

### Aensh funciona em WSL?
Sim, Aensh funciona em WSL (Windows Subsystem for Linux).

## Licença

### Qual é a licença do Aensh?
Aensh está sob a licença MIT. Veja LICENSE para mais detalhes.

### Posso usar Aensh comercialmente?
Sim, a licença MIT permite uso comercial.

## Contato

### Como reporto um bug?
Abra uma issue no repositório com detalhes do problema.

### Como sugiro uma funcionalidade?
Abra uma issue no repositório com sua sugestão.

### Posso entrar em contato com o desenvolvedor?
Você pode abrir uma issue ou enviar um email (se disponível no repositório).

---

**Última atualização:** Dezembro 2024

Não encontrou sua pergunta? Abra uma issue no repositório!
