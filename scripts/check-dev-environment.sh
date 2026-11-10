#!/bin/sh
# Read-only Sprint 01 diagnostics. Run from the Linux guest desktop session.
set -eu

if [ "$(uname -s)" != Linux ]; then
    printf '%s\n' 'BLOCKED: run this diagnostic from the Linux guest.' >&2
    exit 1
fi

missing=0
check() {
    if "$@"; then
        return
    fi
    printf 'MISSING OR FAILED: %s\n' "$*" >&2
    missing=1
}

check cat /etc/os-release
check uname -m
check dnf --version
check git status --short --branch
check rustc --version
check cargo --version
check cmake --version
check pkg-config --modversion gio-2.0 Qt6Core Qt6Qml wayland-client

if [ -n "${QMAKE:-}" ]; then
    check "$QMAKE" -query QT_VERSION
else
    check qmake6 -query QT_VERSION
fi

printf 'Session: %s\nWayland display: %s\n' "${XDG_SESSION_TYPE:-unknown}" "${WAYLAND_DISPLAY:-unset}"
if [ -z "${WAYLAND_DISPLAY:-}" ]; then
    printf '%s\n' 'BLOCKED: no Wayland display advertised by this session.' >&2
    missing=1
fi

printf '%s\n' 'Renderer and nested compositor require a separate graphical test.'
exit "$missing"
