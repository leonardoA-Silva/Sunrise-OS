# Sunrise App Menu — Sprint 01 probe

This component is the shell's installed-application menu. Its command-line entry point and optional Qt Quick window share one Rust catalog. GIO resolves desktop entries and performs desktop activation. The menu never interprets an entry as shell source. A successful launch request does not prove that the target application finished starting.

The QML window is an integration probe for CXX-Qt 0.10. It uses Cargo for this experiment. The choice of bridge and the long-term shell build remain open until the clean Fedora build and three-application test pass. See [ADR 0001](../../docs/adr/0001-linux-wayland-qt-rust.md).

## Prepare the guest

Follow the [environment guide](../../docs/development/environment.md) on both PCs. From a Fedora Wayland desktop, verify the tools with `sh scripts/check-dev-environment.sh` at the repository root. If packages are missing, review the transaction before installing. These packages supply the compiler, GIO development libraries, Qt Quick, and Wayland support inside the guest.

```sh
sudo dnf install git cargo rustfmt clippy gcc-c++ cmake pkgconf-pkg-config \
  glib2-devel qt6-qtbase-devel qt6-qtdeclarative-devel qt6-qtwayland wayland-devel
```

If qmake is outside the guest's PATH, set `QMAKE` to the path reported by the Qt package. The build needs Qt 6.4 or later for the QML load failure signal. The manifest and lockfile record the Rust dependency versions. The exact Fedora package set still needs a clean-guest build.

On Fedora, cargo 1.98 (the distro-provided 1.98.1 at the time of writing) drops `-l` directives emitted by dependency build scripts, so Qt linking needs `lld` in addition to the packages above:

```sh
sudo dnf install lld
export RUSTFLAGS="-C link-arg=-fuse-ld=lld"
```

Keep this export for every build, clippy, and run command in this README while it applies; plain GNU ld.bfd fails on the generated link group.

## Run the probe

From this directory in the Linux guest:

```sh
cargo fmt --all -- --check
cargo clippy --locked --all-targets --features qt-probe -- -D warnings
cargo test --locked
cargo run --locked -- list
cargo run --locked -- launch org.example.Missing.desktop
QT_QPA_PLATFORM=wayland cargo run --locked --features qt-probe --bin sunrise-app-menu-qt
```

The missing-ID command is an intentional failure and should exit nonzero. The tests cover an invalid desktop entry and a launch request with an unavailable working directory. For a real launch, replace the example ID with one printed by `list`.

In the QML window, navigate with the keyboard and open a file manager, an editor, and a terminal from the catalog. Record their desktop IDs and confirm each window appears. Record the commit, guest release, renderer, and any failure. Both maintainers repeat this on their own VMs. The probe holds a catalog snapshot for the session. Restart it after installing or removing applications.

## Dependency provenance

| Component | Upstream and scope |
| --- | --- |
| Rust GIO bindings | [gtk-rs-core](https://github.com/gtk-rs/gtk-rs-core), `gio` 0.21.5, MIT. Desktop discovery and activation |
| Qt bridge | [KDAB CXX-Qt](https://github.com/KDAB/cxx-qt/tree/v0.10.0), 0.10.0, MIT OR Apache-2.0. Optional QML probe |
| Qt and GLib | Guest-provided libraries. Record package versions and applicable terms when preparing a distribution |

The GIO API is the basis for [menu visibility](https://docs.gtk.org/gio/method.AppInfo.should_show.html) and [launching](https://docs.gtk.org/gio/method.AppInfo.launch.html). The optional build follows KDAB's [Cargo-only example](https://github.com/KDAB/cxx-qt/tree/v0.10.0/examples/cargo_without_cmake). See the [clean-room policy](../../docs/clean-room/policy.md) before redistribution.
