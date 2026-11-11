# Sunrise OS

An open-source desktop operating system project for creators and developers who want to make their computer their own.

Sunrise aims to replace Windows as its maintainers' everyday computer: a desktop for programming, managing files, creating content, and playing games. Its longer-term direction includes agents inside applications, helping users adapt tools and customize the system's appearance and behavior.

The current foundation is **Linux, Rust, Wayland, and Qt/QML**, targeting **x86-64 UEFI PCs**. These are engineering choices, not guarantees that every planned application or workflow will work.

> **Early development — not ready for daily use.** This repository contains an application-menu prototype and the project's technical foundation, not an installable Sunrise distribution. Fedora testing has demonstrated the Rust backend and QML startup; the complete desktop and application compatibility remain unproven. Implementation is paused while the product vision is refined.

[Project goals](#project-goals) · [Current status](#current-status) · [Architecture](#architecture) · [Get started](#get-started) · [Roadmap](#roadmap) · [Contributing](#contributing)

## Project goals

- Build a usable desktop before adding advanced automation: working sessions, windows, files, applications, settings, and recoverable updates.
- Keep first-party logic in Rust and separate interface code from process launch, system services, and privileged operations.
- Work independently of Windows, including without a Windows installation, Windows VM, remote Windows host, or proprietary Windows system components.
- Measure resource use and responsiveness on recorded hardware rather than describe untested builds as lightweight.
- Develop an original visual identity informed by Frutiger Aero and glass effects, with readable text and usable controls. Sunrise Glass is a working direction; the final design is still under discussion.
- Explore user-requested, contextual agents that use the user's own online providers. Broad customization must preserve a protected recovery base and a last working system version.

Agents, compatibility layers, and visual effects are future work. They are not prerequisites for the first usable desktop, and their detailed architecture has not been approved.

## Current status

The following summarizes recorded development results as of **November 10, 2026**. It describes specific tests on the first Fedora VM, not general hardware support or a complete Sunrise session.

| Area | Recorded result | Still to demonstrate |
| --- | --- | --- |
| Development environment | Fedora 45 x86-64 VM with UEFI and a GNOME Wayland session | Independent validation on the second maintainer's VM |
| Rust application catalog | Native build and two tests passed; discovered a file manager, text editor, and terminal; rejected an unknown desktop ID | Complete end-to-end launch and failure behavior through the UI |
| Qt/QML integration | CXX-Qt probe built with `lld`; QML root creation returned a non-null object and the process survived a 12-second smoke test | Keyboard interaction and opening all three applications from the window; visual evidence remains incomplete |
| Code checks | Format, Clippy, and default-feature tests passed in the recorded Fedora session | Repeat checks for future implementation changes; verify hosted CI results separately |
| Panel, dock, and compositor | Responsibilities and acceptance criteria documented | Functional panel/dock and a separate nested Smithay test with a real Wayland client |
| Stability and distribution | Milestone gates documented | A 30-minute stability run, complete desktop session, installation, updates, and recovery |

The Linux CI workflow is present, but a successful hosted run is not claimed here. The existing probe uses a session catalog snapshot: restart it after installing or removing applications.

## Architecture

During Sprint 01, the shell runs on an existing Wayland compositor. Smithay experiments run **separately, nested inside the development session**. The shell moves to a Sunrise compositor only after the required protocols and behaviors are demonstrated.

```text
Current development session
  Linux / Fedora + reference Wayland compositor
    Qt Quick / QML application-menu window
      CXX-Qt integration probe
        Rust catalog and launch backend
          GIO / XDG desktop entries

Separate experiment
  Reference desktop session
    Nested Smithay compositor
      Native Wayland test client
```

| Component | Responsibility | Direction |
| --- | --- | --- |
| Desktop shell | Desktop presentation, panel, dock, application menu, notifications, and widgets | Qt 6/QML for layout and interaction; Rust for first-party logic |
| Application-menu backend | Discover and validate installed applications, honor desktop-entry semantics, request launches, and report errors | Rust with GIO; never interpret desktop entries as shell source |
| Sunrise compositor | Wayland surfaces, input, focus, window placement, rendering, and monitors | Rust with Smithay; nested proof first |
| Desktop services | Session lifecycle, settings, power, and indexing | Separate responsibilities; versioned D-Bus interfaces where appropriate |
| Native applications | File management and other first-party tools | Linux APIs and portals with the least privilege needed |
| Sunrise Bridge | Isolated application profiles, runtime selection, and compatibility diagnostics | Planned research subsystem; optional for native desktop operation |
| Distribution | Images, installation, updates, and recovery | Later work, gated by desktop validation |

CXX-Qt is an integration candidate, not a settled long-term bridge. Handwritten C++ requires a documented Qt or native API need and a minimal interface. See the [architecture overview](docs/architecture/overview.md), [platform ADR](docs/adr/0001-linux-wayland-qt-rust.md), and [coding standards](docs/development/coding-standards.md).

**Naming:** the application menu in `shell/launcher/` opens installed programs. The planned **Sunrise Launcher** is a separate official-app store and update interface; the store is outside Sprint 01.

## Get started

Use a Fedora VM for primary validation. Keep builds on the guest filesystem and perform graphical checks from its Wayland desktop session. An existing, authorized WSL2/WSLg setup can support limited checks, but does not replace Fedora or nested-compositor evidence.

1. Follow the [development environment guide](docs/development/environment.md) to prepare and record the guest.
2. From the repository root **inside the Linux guest**, run the read-only diagnostic:

   ```sh
   sh scripts/check-dev-environment.sh
   ```

3. Follow the [application-menu probe guide](shell/launcher/README.md) for dependencies, the Fedora `lld` requirement, backend checks, and the optional QML window.
4. Record the tested revision, guest release, renderer, commands, and observed results using the [delivery workflow](docs/development/agent-workflow.md).

The diagnostic does not install packages. Review package transactions and take a guest snapshot before administrative changes. Do not change host disks, boot configuration, drivers, or security settings to try this prototype.

## Roadmap

### First technical milestone: Sprint 01

Sprint 01 establishes a reproducible developer session and demonstrates:

- A panel with the real system time and access to the application menu.
- A dock with discovered icons and working launch actions.
- A QML menu that lists eligible desktop entries, opens three real applications through Rust, and shows launch failures.
- A separate nested Smithay compositor displaying a real Wayland client.
- Applicable format, lint, build, and test checks, a dependency/license review, and a 30-minute crash-free session.

The [Sprint 01 plan](docs/roadmap/sprint-01.md) defines the technical acceptance criteria. Partial results keep their remaining criteria open.

### First usable system

The broader product goal is a VM that can boot into Sunrise, connect to the internet, browse and download files, manage files with a trash facility, run and manage applications, adjust basic settings, and restart without losing files or configuration. It also needs a tested recovery path for failed updates.

This is larger than Sprint 01. It does not require physical installation, agents, Adobe applications, Roblox Studio, or games for its first acceptance. Those remain longer-term objectives.

### Later milestones

The [project roadmap](docs/roadmap/overview.md) records the dependency gates for an integrated desktop, the Sunrise compositor, services, distribution, and application compatibility. Native apps such as Sunrise Files, a calculator, and a text editor are planned, not demonstrated releases. Agent integration and broad customization remain product directions without a committed delivery date.

## Application compatibility

Sunrise must build, boot, update, and run independently of Windows. A Windows VM or remote Windows session is not a compatibility solution for this project.

Sunrise Bridge is the planned boundary for independently implemented compatibility. Wine is a research candidate; Proton is a reference for games. Neither implies that a particular application, plugin, or anticheat works.

Future workflows under investigation include:

- Content creation with Photoshop 2025, Premiere Pro 2025, and After Effects 2025, including the plugins required for real projects.
- Game development with Roblox Studio.
- Selected contemporary games, with limitations documented per title and anticheat.

**No Linux compatibility claim is made for these applications.** Adobe compatibility is a mandatory future milestone, but supported versions, plugins, formats, GPU paths, and workflows require explicit acceptance criteria and reproducible tests. Launching an application is not proof that editing, saving, rendering, or exporting works.

Tests must use legitimate installers, accounts, and licenses. Publish results by application, version, environment, and workflow, separating authentication, codec, GPU, plugin, and stability blockers. See the [compatibility requirements](docs/architecture/compatibility.md) and [clean-room policy](docs/clean-room/policy.md).

## Repository guide

| Location | What to find |
| --- | --- |
| [`shell/launcher/`](shell/launcher/README.md) | Rust catalog, command-line entry point, and optional Qt/QML probe |
| [`docs/README.md`](docs/README.md) | Documentation index |
| [`docs/architecture/`](docs/architecture/overview.md) | Component ownership and compatibility boundaries |
| [`docs/adr/`](docs/adr/0001-linux-wayland-qt-rust.md) | Recorded architecture decisions |
| [`docs/development/`](docs/development/environment.md) | Environment, coding standards, and delivery workflow |
| [`docs/roadmap/`](docs/roadmap/overview.md) | Milestones and acceptance gates |
| [`scripts/check-dev-environment.sh`](scripts/check-dev-environment.sh) | Read-only environment diagnostic |
| [`.github/workflows/linux-checks.yml`](.github/workflows/linux-checks.yml) | Linux checks for pull requests and pushes to `main` |

Other component paths in the architecture document describe ownership boundaries; they do not imply that implementations already exist.

## Contributing

Start with a small, independently verifiable change tied to an issue and an acceptance check. Follow the [contribution guide](CONTRIBUTING.md), [agent instructions](AGENTS.md), and [coding standards](docs/development/coding-standards.md).

- Preserve the separation between Rust logic, QML presentation, services, and compatibility work.
- Record cross-component or build-system decisions in an ADR before implementation.
- Run the applicable checks in the documented environment; distinguish passed, failed, blocked, partial, and unexecuted checks.
- Include dependency provenance and licensing evidence. Reconstruct behavior independently rather than copying proprietary code, assets, or branding.
- Keep Sprint 01 independent of paid or mandatory remote services. Do not expand its scope with a production ISO, installer, full compositor, or unsupported compatibility promises.

For a bug report, include the revision, guest/hardware details, reproduction steps, expected and observed behavior, and a small relevant log excerpt. Remove credentials and private data before sharing logs.

## License

First-party code uses [Apache-2.0](LICENSE). Third-party libraries, runtimes, and assets retain their own licenses; the project license does not grant redistribution rights for them. See the [third-party inventory](docs/reference/third-party.md) and [clean-room policy](docs/clean-room/policy.md).
