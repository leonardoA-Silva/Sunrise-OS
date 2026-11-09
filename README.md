# Sunrise OS

Sunrise OS é um projeto de sistema operacional desktop de código aberto baseado em Linux. O objetivo é criar uma experiência gráfica original, modular e eficiente para PCs x86-64 com UEFI. Fedora é a base inicial de desenvolvimento.

## Arquitetura planejada

- **Rust** é a linguagem preferencial para a lógica própria, incluindo o launcher, os serviços de sessão e o compositor Wayland com Smithay.
- **Qt 6 e QML** cuidam da interface do desktop, incluindo painel, dock e launcher. C++ fica restrito às integrações que exigirem seu uso.
- **Sunrise Bridge** é o subsistema planejado para investigar a execução de aplicativos Windows com Wine. A compatibilidade com aplicativos Adobe é uma meta de longo prazo que depende de testes por aplicativo, versão e função.

## Estado do projeto

O projeto está na fase de documentação e preparação. O primeiro marco é testar painel, dock e launcher sobre um compositor Wayland de referência. O launcher deverá abrir três aplicativos reais. Em paralelo, uma prova isolada do compositor Smithay deverá executar pelo menos um cliente Wayland em uma sessão aninhada. Os testes serão realizados em ambiente de desenvolvimento isolado.

## Documentação

- [Arquitetura e componentes](docs/architecture/overview.md)
- [Escopo e critérios de aceite do Sprint 01](docs/roadmap/sprint-01.md)
- [Padrões de código](docs/development/coding-standards.md)
- [Regras para agentes](AGENTS.md)
- [Índice da documentação](docs/README.md)

## Licença

O projeto usa [Apache-2.0](LICENSE) para o código original. Dependências e componentes de terceiros seguem suas próprias licenças. Consulte a [política de clean-room e licenças](docs/clean-room/policy.md).
