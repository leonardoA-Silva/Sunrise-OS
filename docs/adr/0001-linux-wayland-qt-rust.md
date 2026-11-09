# ADR 0001 — Base Linux, Wayland, Qt e Rust

**Estado** — Diretriz inicial do mantenedor. A implementação e os testes de integração estão pendentes.

## Contexto

O Sunrise OS precisa entregar um desktop original em etapas. O primeiro marco precisa provar a execução do shell e, separadamente, a viabilidade de um compositor próprio em um ambiente isolado.

## Decisão

Usar Linux e Fedora como base de desenvolvimento, Wayland como protocolo gráfico, Qt 6/QML com C++ quando necessário no shell e Rust com Smithay como caminho preferencial para o compositor. Desenvolver o shell sobre um compositor de referência enquanto o compositor próprio é testado em modo aninhado.

## Alternativas e consequências

- Um kernel próprio ampliaria o trabalho de drivers e boot antes de validar o desktop. Linux permite concentrar o primeiro marco em componentes próprios.
- X11 como base primária mudaria o alvo de integração do compositor. O projeto prioriza Wayland e prevê XWayland depois.
- Integrar o compositor próprio desde o primeiro painel acoplaria a validação de duas frentes. A separação permite testar cada uma e expõe cedo lacunas de protocolos, como suporte a layer-shell.
- Qt/QML e Rust exigem duas cadeias de ferramentas. A fronteira entre os processos reduz integrações nativas e permite validar cada build separadamente.

Se evidências inviabilizarem uma decisão, abra nova ADR com estado, contexto, alternativas, evidências, impactos e recomendação. Vincule a decisão anterior e registre aprovação antes de alterar a arquitetura.
