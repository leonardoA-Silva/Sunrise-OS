# Contributing to Sunrise OS

Thanks for your interest. Project decisions and technical contracts live in [AGENTS.md](AGENTS.md) and the [documentation index](docs/README.md). This file covers how to propose and land changes.

## Scope of an increment

- Work one independently verifiable change at a time, following the [agent workflow](docs/development/agent-workflow.md). State the goal, paths, risks, and checks before large changes.
- Keep milestone work aligned with [Sprint 01](docs/roadmap/sprint-01.md). Out-of-scope items (custom kernel, dual boot, production ISO, installer, Adobe claims, telemetry) stay out until the roadmap gates pass.
- Record architecture decisions that cross component boundaries in an ADR before implementation.

## Submitting changes

1. Open a GitHub issue describing the problem, evidence, and acceptance check.
2. Fork or branch from `main` with a descriptive name, such as `codex/feat-sprint-01-app-menu`.
3. Keep first-party logic in Rust, presentation in QML, and handwritten C++ at documented integration boundaries ([coding standards](docs/development/coding-standards.md)).
4. Run the applicable checks for every crate you touch, for example:
   ```sh
   cargo fmt --all -- --check
   cargo clippy --locked --all-targets -- -D warnings
   cargo test --locked
   ```
5. Verify dependency and licensing changes against the [clean-room policy](docs/clean-room/policy.md) and the [third-party inventory](docs/reference/third-party.md).

## Review and licenses

- Pull requests to `main` require review from a maintainer other than the author.
- New dependencies or assets need proof of upstream source, exact version, license, and redistribution terms recorded in the inventory before they are merged.
- Validate Fedora behavior in a virtual machine per the [environment guide](docs/development/environment.md). A Windows-side check does not substitute for guest evidence.

## Reporting results

Use the status table (`PASS`, `FAIL`, `BLOCKED`, `NOT RUN`, `PARTIAL`) and the increment report format from the [agent workflow](docs/development/agent-workflow.md). Keep every claim tied to observed evidence.
