# Sprint 01 — Fundação funcional

Prazo estimado de 10 dias úteis. A documentação de agentes é a preparação inicial. Os critérios técnicos abaixo aguardam execução.

| Etapa | Critério de aceite |
| --- | --- |
| S01-01 — Diagnóstico | Registrar raiz, branch, estado Git, instruções locais e matriz de pré-requisitos, com riscos de GPU e virtualização |
| S01-02 — Fundação | Preparar README, contribuição, licença e inventário inicial no checkout. Preservar documentação existente e adotar regras de arquivos ignorados |
| S01-03 — Ambiente e build | Verificar build limpo com comandos documentados em Fedora ou ambiente reproduzível. Adicionar scripts seguros, configuração Qt/QML com CMake, Cargo conforme necessidade e CI Linux com checks executáveis |
| Shell de referência | Rodar painel com hora real e ação para abrir o launcher. Rodar dock com ícones e cliques que iniciam apps. Documentar suporte a layer-shell e validar o fallback quando aplicável |
| Launcher | Descobrir entradas `.desktop` e abrir pelo menos três apps reais no ambiente de teste. Tratar erro de lançamento e exibir estado de execução somente quando houver evidência |
| Prova Smithay | Executar compositor aninhado e abrir pelo menos um cliente Wayland. Registrar protocolos, entrada, renderização e lacunas para integração |
| Estabilidade e revisão | Executar sessão de 30 minutos sem crash e verificar formato, lint e testes aplicáveis. Auditar dependências, licenças, alterações e arquivos ignorados |

Fora do marco ficam kernel próprio, dual boot, ISO de produção, instalador, loja, drivers exclusivos, Adobe funcional, tema Sunrise Glass final, compositor completo e telemetria remota. Use o [fluxo dos agentes](../development/agent-workflow.md) para classificar e registrar critérios pendentes.
