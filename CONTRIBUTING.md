# 🤝 Contribuindo para o Aensh

Obrigado por se interessar em contribuir para o Aensh! Este documento fornece diretrizes e instruções para contribuir.

## 📋 Código de Conduta

Todos os contribuidores devem seguir nosso código de conduta:
- Ser respeitoso com outros contribuidores
- Aceitar críticas construtivas
- Focar no que é melhor para a comunidade
- Mostrar empatia com outros membros

## 🐛 Reportando Bugs

### Antes de reportar
- Verifique se o bug já foi reportado
- Teste com a versão mais recente
- Verifique a documentação

### Como reportar
1. Use um título descritivo
2. Descreva o comportamento esperado vs atual
3. Forneça exemplos específicos para reproduzir
4. Inclua screenshots se relevante
5. Mencione sua versão do Aensh e Rust

### Exemplo de relatório
```
Título: ls não mostra arquivos ocultos

Descrição:
O comando `ls` não mostra arquivos que começam com ponto (.)

Passos para reproduzir:
1. Criar arquivo oculto: touch .hidden
2. Executar: ls
3. Arquivo .hidden não aparece

Comportamento esperado:
Arquivo .hidden deveria ser listado

Versão:
Aensh 0.2.0
Rust 1.70.0
```

## 💡 Sugerindo Melhorias

### Antes de sugerir
- Verifique se a sugestão já existe
- Considere se é útil para a maioria dos usuários

### Como sugerir
1. Use um título claro e descritivo
2. Forneça uma descrição detalhada
3. Liste exemplos de como seria usado
4. Mencione alternativas consideradas

## 🔧 Processo de Desenvolvimento

### 1. Fork e Clone
```bash
# Fork no GitHub
# Clone seu fork
git clone https://github.com/seu-usuario/aensh.git
cd aensh
```

### 2. Crie uma Branch
```bash
# Crie uma branch descritiva
git checkout -b feature/novo-comando
# ou
git checkout -b fix/corrigir-bug
```

### 3. Faça suas Mudanças
- Siga as convenções de código (veja abaixo)
- Escreva commits descritivos
- Teste suas mudanças

### 4. Teste Localmente
```bash
# Build
cargo build

# Teste
cargo run

# Lint
cargo clippy

# Formatar
cargo fmt
```

### 5. Commit e Push
```bash
git add .
git commit -m "Descrição clara da mudança"
git push origin feature/novo-comando
```

### 6. Abra um Pull Request
- Descreva o que foi mudado
- Referencie issues relacionadas
- Aguarde review

## 📝 Convenções de Código

### Rust Style
- Use `cargo fmt` para formatar
- Use `cargo clippy` para lint
- Siga as convenções de nomenclatura Rust

### Nomes
- Funções e variáveis: `snake_case`
- Tipos e traits: `PascalCase`
- Constantes: `UPPER_CASE`

### Comentários
```rust
// Comentário de linha única

/// Documentação de função
/// 
/// # Exemplos
/// ```
/// let result = my_function();
/// ```
pub fn my_function() {
    // ...
}
```

### Tratamento de Erros
```rust
// Sempre retornar AenshResult<T>
pub fn run(args: &[String]) -> AenshResult<()> {
    if args.is_empty() {
        return Err(AenshError::Validation("uso: comando <arg>".into()));
    }
    Ok(())
}
```

### Colorização
```rust
use colored::*;

// Sucesso
println!("{} Concluído", "✓".bright_green());

// Erro
eprintln!("{} Erro", "✗".bright_red());

// Info
println!("{} Informação", "ℹ".bright_blue());
```

## 📚 Adicionando um Novo Comando

Veja `docs/DEVELOPMENT.md` para um guia completo.

Resumo:
1. Crie o arquivo do comando
2. Implemente a função `run()`
3. Registre no módulo
4. Adicione ao dispatcher
5. Atualize a ajuda

## 🧪 Testando

### Teste Manual
```bash
cargo run
# No shell:
seu_comando arg1 arg2
```

### Teste de Compilação
```bash
cargo check
```

### Lint
```bash
cargo clippy
```

### Formatar
```bash
cargo fmt
```

## 📖 Documentação

### Atualizando Documentação
- Mantenha README.md atualizado
- Atualize docs/ conforme necessário
- Adicione exemplos quando possível

### Escrevendo Documentação
- Seja claro e conciso
- Use exemplos práticos
- Inclua links quando relevante

## 🎯 Áreas para Contribuir

### Fácil
- [ ] Melhorar documentação
- [ ] Corrigir typos
- [ ] Adicionar exemplos

### Médio
- [ ] Novos comandos simples
- [ ] Melhorias em comandos existentes
- [ ] Melhorias de UI

### Difícil
- [ ] Pipes e redirecionamento
- [ ] Histórico de comandos
- [ ] Autocompletar

## 📋 Checklist para Pull Request

- [ ] Código segue as convenções
- [ ] Testes passam (`cargo test`)
- [ ] Sem warnings (`cargo clippy`)
- [ ] Código formatado (`cargo fmt`)
- [ ] Documentação atualizada
- [ ] Commit messages descritivas
- [ ] Referencia issues relacionadas

## 🚀 Processo de Review

1. Pelo menos um mantenedor revisa
2. Mudanças podem ser solicitadas
3. Após aprovação, PR é merged
4. Sua contribuição é creditada

## 📞 Comunicação

- Issues: Para bugs e sugestões
- Discussions: Para perguntas e ideias
- Pull Requests: Para contribuições de código

## 📄 Licença

Ao contribuir, você concorda que suas contribuições serão licenciadas sob a licença MIT.

## 🙏 Agradecimentos

Obrigado por contribuir para tornar o Aensh melhor!

---

**Última atualização:** Dezembro 2024

Dúvidas? Abra uma issue ou entre em contato!
