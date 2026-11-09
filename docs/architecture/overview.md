# Arquitetura de referência

A plataforma inicial é PC x86-64 com UEFI. A base de desenvolvimento é Fedora mínima ou ambiente Fedora reproduzível. Linux e seus drivers são dependências externas. A estrutura abaixo é lógica. Cada diretório entra quando houver trabalho concreto para ele.

| Caminho | Responsabilidade |
| --- | --- |
| `compositor/sunrise-compositor/` | Superfícies Wayland, entrada, foco, geometria, renderização, monitores e protocolos |
| `shell/` | Desktop, painel, dock, launcher, notificações e widgets em Qt 6/QML. No Sprint 01, o shell usa um compositor de referência |
| `apps/` | Aplicativos como arquivos e configurações. Usam portais e APIs Linux com o menor privilégio necessário |
| `services/` | Sessão, preferências, energia e indexação. Processos separados preferem contratos D-Bus versionados |
| `compatibility/` | Sunrise Bridge, com pesquisa, perfis por aplicativo e resultados de testes. Mantém sua lógica isolada do compositor e do kernel |
| `distribution/` | Imagens, instalador e atualizações após a base do desktop |
| `design/` | Ativos originais, tokens e especificações da futura identidade Sunrise Glass |
| `scripts/`, `tests/`, `.github/workflows/` | Ferramentas locais, testes entre componentes e verificações de CI |

A interface do shell apresenta estados e dispara ações. Regras de descoberta e lançamento pertencem à integração nativa do componente responsável. O compositor fica restrito às responsabilidades Wayland da tabela. Serviços maduros como systemd, Mesa/DRM/KMS, PipeWire, NetworkManager, udev e portais entram conforme necessidade. Clientes Wayland nativos vêm primeiro. XWayland e Flatpak entram em etapas posteriores.

O Bridge investiga Wine para aplicativos Windows e usa Proton como referência para jogos. A compatibilidade ampla com Adobe é requisito de longo prazo, com viabilidade e prazo em aberto. A matriz de evidências faz parte da [política de clean-room](../clean-room/policy.md).
