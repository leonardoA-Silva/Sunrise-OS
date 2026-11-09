# Sprint 01 — Functional foundation

The initial estimate is 10 working days. Use evidence to determine completion. This milestone targets a reproducible developer session, a minimal working shell, and a separate Smithay probe.

| Step | Acceptance evidence |
| --- | --- |
| S01-01 — Diagnose the environment | Record root, branch, Git status, tooling versions, graphics/session capabilities, and missing prerequisites using the [environment guide](../development/environment.md) |
| S01-02 — Prepare the repository | Confirm README, license, contribution rules, third-party inventory, and ignore rules. Preserve existing documentation and changes |
| S01-03 — Build and CI | Reproduce a clean build from the documented Fedora setup. Provide safe build and prerequisite commands, CMake/Qt configuration, Cargo where needed, and actual Linux CI checks |
| Reference shell | Demonstrate a panel with live system time and a launcher action. Demonstrate a dock with discovered icons and working launches. Record layer-shell support and test any required fallback |
| Rust-backed launcher | In QML, display eligible desktop entries and launch three real applications from Rust. Demonstrate invalid-entry handling and a visible launch failure. Claim running state only where observed |
| Nested Smithay probe | Run separately and display at least one real Wayland client. Record the backend, rendering, input, protocols, and gaps relevant to shell integration |
| Stability and audit | Observe a 30-minute session without a crash. Record format, lint, build, and test results and review dependency licenses and ignored files |

Outside this sprint are a custom kernel, dual boot, production ISO, installer, store, proprietary drivers, Adobe execution, final Sunrise Glass effects, a full compositor, and remote telemetry.

Record each result using the [agent workflow](../development/agent-workflow.md). If nested rendering is blocked, record the tested backend, environment, and logs. Keep the acceptance criterion open until it is demonstrated.
