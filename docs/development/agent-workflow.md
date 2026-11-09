# How to deliver an increment

1. Inspect the branch, working tree, local instructions, affected callers, and the last verified behavior. State the goal, paths, risks, and checks. Use the [environment guide](environment.md) for development setup.
2. Implement one independently verifiable change. Preserve existing work and error handling. Review dependencies, licensing, accessibility, and permission changes. If a design decision crosses a component boundary, record its evidence and tradeoffs in an ADR.
3. Run the relevant checks from the component's normal entry point. For manual tests, record the OS, session type, tool versions, input, reproduction steps, and observed behavior. For graphic tests, record guest GPU or software rendering and distinguish a reference shell session from a nested compositor test.
4. Review the diff, then report each criterion using the status table. For a failure or blocker, capture the smallest useful log excerpt, impact, and retry condition. Record a local follow-up or open a remote issue when authorized.
5. When publishing is authorized, use English semantic subjects such as `docs: expand the Fedora development guide`. Stage only the intended change, run `git diff --cached --check`, and verify the published revision.

| Status | Meaning |
| --- | --- |
| PASS | Evidence confirms the criterion |
| FAIL | The check ran and the observed result contradicted the criterion |
| BLOCKED | A missing prerequisite prevented the check |
| NOT RUN | The check was outside the increment or remains pending |
| PARTIAL | The delivery includes both completed and outstanding criteria |

```text
SUNRISE OS — INCREMENT REPORT
Milestone or task:
Status:
Implemented:
Files changed:
How to reproduce:
Checks and observed results:
Known limitations:
Risks and licensing:
Next increment:
```
