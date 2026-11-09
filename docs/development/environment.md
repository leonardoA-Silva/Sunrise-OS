# How to prepare the development environment

## Fedora VM

1. Use an approved hypervisor and create an x86-64 Fedora guest with a Wayland desktop and a virtual disk. Give it enough resources to run a graphical session and compile Rust. Record the guest release, virtual CPU and memory, GPU configuration, and hypervisor version.
2. Keep the working copy on the guest filesystem and run builds as an ordinary user. Take a VM snapshot before changes that require guest administration. Obtain authorization before any host-level changes.
3. Check the environment inside the guest. Run the graphics checks from the desktop session that will launch the shell.

```sh
cat /etc/os-release
uname -m
dnf --version
git status --short --branch
rustc --version
cargo --version
cmake --version
pkg-config --modversion Qt6Core wayland-client
printf 'Session=%s Wayland display=%s\n' "${XDG_SESSION_TYPE:-unknown}" "${WAYLAND_DISPLAY:-unset}"
```

4. Record each prerequisite as present, missing, or unverified. A missing command or metadata file is a diagnostic result. Resolve packages against the actual Fedora release and record exact versions and install steps after verifying them.
5. Run a known Wayland client in the guest before testing Sunrise. Record the rendering path and whether the nested backend works. A set display variable is an environment hint. A rendered client supplies execution evidence.

## WSL2 and WSLg

Use an existing authorized setup for Rust checks and limited Qt/QML application smoke tests. Record distro, WSL version, and graphics configuration. A successful application window under WSLg is evidence for that application test. Record nested compositor behavior and guest GPU capabilities separately in the Fedora VM. [Microsoft WSLg guide](https://learn.microsoft.com/en-us/windows/wsl/tutorials/gui-apps)

## Environment record

| Item | Record |
| --- | --- |
| Checkout | Remote, branch, commit, and working-tree status |
| Tooling | Rust, Cargo, CMake, Qt, and Wayland versions or missing checks |
| Runtime | Guest or WSL distro, session type, renderer, and backend |
| Evidence | Command, outcome, and smallest useful log excerpt |
| Blockers | Impact and next reproducible check |

This guide defines the diagnostic procedure. Project build and launch commands will be added when corresponding targets exist and have been verified.
