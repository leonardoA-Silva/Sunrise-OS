# Como entregar um incremento

1. Identifique a raiz, leia as regras do caminho afetado e inspecione os arquivos e as mudanças locais. Se houver dúvida sobre qual checkout deve receber uma edição, esclareça o destino antes de escrever.
2. Confirme o ambiente para os comandos da tarefa. Para build gráfico, registre SO, Fedora ou alternativa Linux, gerenciador de pacotes, Rust/Cargo, CMake, Qt, Wayland, ferramentas de build, virtualização e capacidades gráficas disponíveis. Registre versões, lacunas e impacto. Em Windows, use VM Fedora ou WSL2/WSLg já autorizado e adequado. A abertura de uma janela em WSLg e a validação do compositor aninhado são verificações distintas.
3. Delimite arquivos, riscos e critérios. Implemente uma unidade funcional. Inspecione o diff, dependências, licenças, tratamento de falhas e acessibilidade aplicável.
4. Execute os checks de formato, lint, build e testes aplicáveis. Para testes manuais, registre ambiente, versão, comando ou procedimento, resultado e trecho relevante de log. Informe limitações de GPU ou virtualização. Use capturas e métricas somente como evidência do que foi observado.
5. Classifique cada critério. Use `PASS` para critério comprovado, `FAIL` para execução que contrariou o critério, `BLOCKED` para pré-requisito ausente que impediu execução e `NOT RUN` para verificação pendente ou fora do incremento. Use `PARTIAL` no resumo quando a entrega tiver critérios cumpridos e outros pendentes. Registre o bloqueio ou falha em um item técnico local. Abra issue remota quando houver autorização.
6. Entregue o relatório abaixo. Distingua código funcional, protótipos, falhas e verificações pendentes. Avance ao próximo marco quando os critérios mínimos do marco atual estiverem comprovados.

```text
SUNRISE OS — RELATÓRIO DE INCREMENTO
Marco/Tarefa:
Status: PASS / PARTIAL / FAIL / BLOCKED / NOT RUN
Implementado:
Arquivos alterados:
Como executar:
Testes executados e resultados:
Limitações conhecidas:
Riscos/licenças:
Próximo incremento recomendado:
```
