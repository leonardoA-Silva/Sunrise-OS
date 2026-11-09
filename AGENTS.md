# Instruções para agentes do Sunrise OS

Sunrise OS é um sistema desktop original, de código aberto, baseado em Linux. A arquitetura começa com Fedora, Wayland, Qt 6/QML e uma prova isolada do compositor em Rust com Smithay. O estado inicial desta pasta é documental.

## Antes de editar

1. Confirme a raiz do projeto. Em um checkout Git, verifique branch, `git status --short` e alterações existentes. Leia o README, quando existir, e os `AGENTS.md` do caminho afetado.
2. Leia o [índice da documentação](docs/README.md) e consulte as referências do componente. Preserve alterações e arquivos do mantenedor.
3. Informe objetivo, estado confirmado, arquivos previstos e critérios de aceite. Faça perguntas quando faltar uma decisão que mude o desenho ou uma autorização necessária. Declare suposições reversíveis e avance.
4. Faça um incremento funcional por vez. Siga o [padrão de código](docs/development/coding-standards.md), reutilize a estrutura existente e preserve contratos. Registre mudanças arquiteturais em ADR antes de trocar a direção do projeto.

## Limites

- Use agentes auxiliares somente com autorização explícita do mantenedor.
- Exija autorização explícita para commits, push, publicação e operações Git destrutivas. Preserve histórico e mudanças locais.
- Exija autorização explícita para formatar discos, alterar partições ou UEFI, instalar bootloader, trocar drivers do host, desativar proteções ou preparar dual boot. Explique impacto e necessidade antes de instalar dependências de sistema ou usar elevação. Obedeça às permissões do ambiente.
- Prefira testes isolados em VM ou sessão Wayland aninhada. Inspecione scripts externos antes de rodar. Mantenha segredos fora do código, dos arquivos rastreados e dos logs.
- Aplique a [política de clean-room](docs/clean-room/policy.md) antes de incorporar código, dependências, ativos ou informações de pesquisa. Mantenha o Sprint 01 executável sem serviços pagos ou infraestrutura remota obrigatória.

## Entrega

Use o [fluxo de trabalho](docs/development/agent-workflow.md) para validar e relatar. Identifique protótipos, indique verificações pendentes e associe cada resultado à evidência observada. Preserve os critérios do [Sprint 01](docs/roadmap/sprint-01.md).
