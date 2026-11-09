# Coding standards

## Component layout

| Area | Convention |
| --- | --- |
| Rust | Use `Cargo.toml`, `src/`, and crate-local `tests/` for integration tests. Introduce a workspace when crates need a coordinated build |
| Qt and QML | Use CMake targets per component, `qml/` for QML, `src/` for handwritten native integration, and `tests/` for component tests. Keep bridge inputs and build ownership explicit |
| Shared checks | Use root `tests/` for cross-component scenarios and `scripts/` for reusable environment and build tasks |
| Naming | Use Rust `snake_case` for functions and modules, `PascalCase` for Rust types and QML components, and `lowerCamelCase` for QML properties and Qt-facing methods |

Follow established local conventions and configuration. Use English names and repository documentation. Extract shared abstractions when concrete reuse justifies them.

## Implementation contracts

- Use Rust for domain logic, validation, filesystem work, and process control. Keep QML focused on bindings, visual state, input, and accessibility. Document any handwritten C++ exception and keep it at the native boundary.
- Model recoverable Rust failures with `Result`. Add context at the layer that knows what operation failed. Keep secrets and sensitive path or profile contents out of logs.
- Treat desktop entries, filenames, D-Bus payloads, and imported profiles as external input. Use the [Desktop Entry Specification](https://specifications.freedesktop.org/desktop-entry/latest/) for discovery, eligibility, argument handling, and activation. Pass arguments structurally and avoid interpreting external strings with a shell.
- Give interactive UI controls accessible names, keyboard focus, and an observable success or failure state. Keep expensive work off the UI thread. Document the ownership and threading rules of any bridge.
- Keep established callers working. Cover changes to trust boundaries and regressions with a focused check. Record the relevant dependency version and license when adding a dependency.

## Verification conventions

Use repository commands once defined. Typical Rust checks are `cargo fmt --all -- --check`, `cargo clippy --all-targets`, and `cargo test` from the applicable crate or workspace. Use the configured build and test targets for CMake and QML. Document the exact setup, commands, and tool versions once verified. Compare performance only against a reproducible baseline.
