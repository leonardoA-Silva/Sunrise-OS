# Padrões de código

## Organização

| Área | Convenção inicial |
| --- | --- |
| Rust | Cargo. `Cargo.toml` por crate, módulos em `src/` e testes de integração em `tests/` da crate. Use workspace quando houver crates que precisem de build coordenado |
| C++ e QML | CMake com alvos por componente. Mantenha C++ em `src/`, QML em `qml/` e testes em `tests/` dentro do componente |
| Testes entre componentes | Use `tests/` na raiz para cenários que atravessam componentes |
| Scripts | Use `scripts/` para verificações e builds reutilizados. Declare ambiente, argumentos e pré-requisitos. Use códigos de saída coerentes |

Em código existente, siga a organização local e os arquivos de configuração. Em código novo, use `snake_case` para módulos e funções Rust, `PascalCase` para tipos Rust e componentes QML, e `lowerCamelCase` para propriedades QML e métodos de integração Qt. Use identificadores técnicos em inglês. Mantenha documentação de contribuição em português.

## Implementação

- Coloque lógica no componente responsável segundo a [arquitetura](../architecture/overview.md). Reuse padrões existentes. Extraia uma abstração compartilhada quando houver pelo menos dois usos concretos.
- Use tipos explícitos nas fronteiras. Em Rust, propague falhas recuperáveis com `Result`. Em QML, mantenha apresentação, estado visual e interação. Exponha a lógica nativa por uma interface pequena e testável.
- Valide dados externos. Use API compatível com a semântica das entradas `.desktop` e preserve os argumentos sem interpretar texto em um shell. Trate entradas inválidas, apps indisponíveis e falhas ao lançar.
- Preserve acessibilidade por teclado e nomes acessíveis para controles. Registre erros com contexto suficiente para diagnóstico, sem dados sensíveis.
- Use configurações de formatação versionadas quando existirem. Para Rust, a referência inicial é `cargo fmt --all -- --check`, `cargo clippy --all-targets` e `cargo test` no manifesto ou workspace aplicável. Para C++/QML, use os alvos e ferramentas configurados no componente. Registre versões verificadas antes de fixar mínimos.
- Adicione verificação focada quando a mudança introduzir lógica relevante ou corrigir regressão sem cobertura. Documente build reproduzível, pré-requisitos e passos de execução quando houver código executável. CI Linux deve executar verificações reais. Evite declarar ganho de desempenho sem linha de base comparável.
