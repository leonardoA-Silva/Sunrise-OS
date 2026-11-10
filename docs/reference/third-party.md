# Third-party dependency inventory

Record provenance, exact version, license, intended use, and redistribution obligations for every imported dependency, asset, or service per the [clean-room policy](../clean-room/policy.md). Direct dependencies only; transitive crates keep their licenses in the Cargo lockfile and gem or package registries.

## Rust crates (`shell/launcher/`)

| Dependency | Version (usage) | Upstream | License | Intended use | Redistribution notes |
| --- | --- | --- | --- | --- | --- |
| `gio` | 0.21.5 | [gtk-rs-core](https://github.com/gtk-rs/gtk-rs-core) | MIT | Desktop entry discovery, eligibility (`AppInfo.should_show`), and activation (`AppInfo.launch`) | Preserve MIT notice in redistributed binaries |
| `cxx` | `1` (optional under `qt-probe`) | [dtolnay/cxx](https://github.com/dtolnay/cxx) | MIT, Apache-2.0 | FFI bridge for the CXX-Qt probe | Dual-licensed; keep both notices |
| `cxx-qt`, `cxx-qt-lib` | 0.10 | [KDAB/cxx-qt](https://github.com/KDAB/cxx-qt) | MIT, Apache-2.0 | Expose Rust catalog to the QML probe | Bridge choice still under evaluation ([ADR 0001](../adr/0001-linux-wayland-qt-rust.md)) |
| `cxx-qt-build` | 0.10 (build) | KDAB/cxx-qt | MIT, Apache-2.0 | Cargo-only QML probe build | Same notices |

## Guest-provided libraries

| Component | Source | License | Notes |
| --- | --- | --- | --- |
| Qt 6 (Base, Declarative, Quick, Wayland) | Fedora packages (not vendored) | LGPL-3.0-or-later | First-party code links dynamically; record exact package versions when preparing distribution |
| GLib / GIO C libraries | Fedora packages | LGPL-2.1-or-later | Reached through the `gio` Rust bindings |

## External references

| Material | Use |
| --- | --- |
| [Desktop Entry Specification](https://specifications.freedesktop.org/desktop-entry/latest/) | Public specification; drives discovery and launch semantics |
| [Microsoft WSLg guide](https://learn.microsoft.com/en-us/windows/wsl/tutorials/gui-apps) | Public documentation for limited WSLg smoke tests |
| [Smithay Smallvil example](https://github.com/Smithay/smithay) | Reference probe run from a separate checkout, not copied into Sunrise code |

## Maintenance rule

Update this inventory in the same increment that adds, replaces, or upgrades a dependency. `cargo update` version bumps require re-reading upstream licenses and a fresh entry or confirmation that existing rows remain accurate.
