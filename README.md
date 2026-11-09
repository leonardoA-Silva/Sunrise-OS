 Sunrise OS

Sunrise OS is an open-source desktop operating system project built on Linux for x86-64 UEFI PCs. It aims to deliver an original desktop with a modular shell, a native Wayland compositor, and independently implemented compatibility for selected Windows applications.

**Project status** — Documentation and engineering planning are in place. The first milestone is a runnable developer session. The repository currently has no buildable desktop, installer, or verified application compatibility.

## How the system is designed

- **Linux and Wayland** provide the platform. Fedora in a virtual machine is the primary development and validation environment.
- **Rust** is the default for first-party logic, including the compositor, launcher backend, and desktop services. Smithay is the preferred compositor toolkit.
- **Qt 6 and QML** provide the shell UI. QML owns presentation and interaction. Handwritten C++ requires a documented integration need.
- **Sunrise Bridge** is the planned, isolated compatibility subsystem. It draws on publicly documented Windows behavior while Sunrise builds, boots, and runs independently of Windows.
- **Sunrise Glass** is the planned visual identity. The first milestone prioritizes functioning controls, keyboard interaction, and reliable startup.

## First milestone

Sprint 01 targets a real panel, dock, and launcher on a reference Wayland compositor. The launcher must discover and open three real applications through its Rust backend. A separate Smithay experiment must run nested and display a real Wayland client. The sprint records a clean build, a 30-minute stability run, and any environment blockers.

Broad compatibility with Adobe applications is a mandatory future requirement. Compatibility will be measured by app, version, and workflow. The target application list and validated versions remain open. See the [compatibility requirements](docs/architecture/compatibility.md).

## Start here

| Goal | Document |
| --- | --- |
| Set up a development environment | [Fedora VM guide](docs/development/environment.md) |
| Understand component ownership | [Architecture overview](docs/architecture/overview.md) |
| Find acceptance criteria | [Sprint 01](docs/roadmap/sprint-01.md) |
| See later milestones | [Project roadmap](docs/roadmap/overview.md) |
| Contribute or work with an agent | [Agent instructions](AGENTS.md) and [coding standards](docs/development/coding-standards.md) |

See the [documentation index](docs/README.md) for all project references.

## License

First-party code uses [Apache-2.0](LICENSE). Third-party components retain their own licenses. See the [clean-room policy](docs/clean-room/policy.md) for provenance and review requirements.
