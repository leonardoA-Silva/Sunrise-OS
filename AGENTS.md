# Sunrise OS agent instructions

These rules apply across the repository. Read any more specific instructions in the directories you change. Use English for repository documentation and semantic commit messages.

## Start each task

1. Confirm the repository root, branch, and working-tree status. Inspect existing changes and read the [documentation index](docs/README.md) and the relevant component docs.
2. Report the confirmed state, the intended increment, affected paths, and acceptance checks. Ask before proceeding when a missing decision changes architecture, compatibility scope, or authorization. State reversible assumptions and continue when the task is clear.
3. Use the [Sprint 01 plan](docs/roadmap/sprint-01.md) for milestone work. Keep the increment small, preserve unrelated changes, and extend an existing pattern before introducing a new one.

## Enforce the architecture

- Write first-party logic in Rust. Keep layout, visual state, and interaction in QML. For handwritten C++, document the Qt or native API that requires it, the Rust approach considered, and the smallest interface that meets the need. Record cross-component or build-system changes in an ADR.
- Keep application discovery, validation, and process launch in the launcher backend. Validate external data and use structured invocation that honors desktop entry semantics. Keep privileged work outside UI code.
- Keep shell, compositor, services, and compatibility in their documented boundaries. Treat CXX-Qt as a candidate until the launcher probe passes on Fedora. Follow the [architecture](docs/architecture/overview.md) and [coding standards](docs/development/coding-standards.md).
- Ensure all Sunrise build, boot, desktop, and compatibility paths can operate without Windows, Windows system files, or a Windows runtime. Base compatibility work on lawful public behavior and independent implementations. Treat Adobe compatibility as a mandatory future milestone and tie every support claim to evidence.

## Respect operational limits

- Use a Fedora VM for primary validation. Use an existing, authorized WSL2/WSLg setup for limited checks. Run compositor experiments nested inside a development session.
- Obtain explicit authorization before formatting disks, changing partitions or UEFI, installing a bootloader, replacing host drivers, disabling protections, or configuring dual boot. Explain the effects of system package installation or elevation and respect the environment's permissions.
- Preserve user files and Git history. Obtain explicit authorization for commits, pushes, publishing, destructive Git actions, or delegation to auxiliary agents. Inspect external scripts before running them. Keep credentials out of code and logs. Keep Sprint 01 independent of paid or mandatory remote services.
- Apply the [clean-room policy](docs/clean-room/policy.md) before incorporating external code, assets, dependencies, or research.

## Verify and report

Use the [delivery workflow](docs/development/agent-workflow.md). Run applicable format, lint, build, test, and manual checks. Report the actual environment, commands, outcomes, and blockers. Label prototypes clearly and give each unexecuted check its real status. Record limitations without weakening acceptance criteria after testing.
