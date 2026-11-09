# Reference architecture

The initial target is x86-64 UEFI. Linux, drivers, and Fedora development images are external dependencies. The following paths are logical ownership boundaries. A directory is added when it has an implementation or an artifact to maintain.

| Path | Ownership |
| --- | --- |
| `compositor/sunrise-compositor/` | Wayland surfaces, focus, input, window placement, rendering, monitors, and protocol state in Rust |
| `shell/` | Qt/QML presentation for desktop, panel, dock, launcher, notifications, and widgets. First-party application logic lives in Rust |
| `services/` | Session lifecycle, settings, power, and indexing. Separate processes prefer versioned D-Bus interfaces |
| `apps/` | Native applications such as file management and settings. They use Linux APIs and portals with least privilege |
| `compatibility/` | Sunrise Bridge research, application profiles, and reproducible test results |
| `distribution/` | Images, installation, recovery, and update mechanisms after desktop validation |
| `design/` | Original assets, design tokens, and Sunrise Glass specifications |
| `scripts/`, `tests/`, `.github/workflows/` | Reusable tooling, cross-component checks, and Linux CI |

## Runtime boundaries

During Sprint 01, the shell runs on a reference Wayland compositor. The nested Smithay prototype has a separate runtime and acceptance test. The shell eventually moves to the Sunrise compositor once protocol and behavior gaps are resolved.

The launcher UI requests discovery or launch through a small backend interface. Rust handles desktop entry semantics, structured invocation, and errors. The UI presents known results and keeps an unknown running state distinct from a confirmed one.

Systemd, Mesa/DRM/KMS, PipeWire, NetworkManager, udev, and XDG Desktop Portals enter as their capabilities are needed. Native Wayland clients come first. XWayland and Flatpak integration follow desktop stabilization.

Sunrise Bridge remains optional for native desktop operation. It owns per-app configuration and diagnostics. Its compatibility paths run on Linux without a Windows installation or Windows system components. See the [compatibility requirements](compatibility.md) and [platform decision](../adr/0001-linux-wayland-qt-rust.md).
