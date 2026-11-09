# ADR 0001 — Linux, Wayland, Qt, and Rust

**Status** — Accepted direction. Implementation and integration evidence are pending.

## Context

The project needs an independent Linux desktop. The first milestone must validate an original shell and investigate a compositor in parallel. A consistent language boundary helps keep business logic testable and UI code small.

## Decision

Use Linux on x86-64 UEFI and develop primarily in a Fedora VM. Use Wayland for display protocols. Start the Qt 6/QML shell on a reference compositor. Build a separate nested Rust compositor probe with Smithay.

Use Rust by default for first-party behavior. Keep presentation and interaction in QML. Require a documented technical reason and a narrow boundary for handwritten C++. Evaluate [CXX-Qt](https://kdab.github.io/cxx-qt/book/getting-started/5-cmake-integration.html) using a Fedora launcher probe. The probe must expose a Rust-backed app list, handle launch failure visibly, and open three applications with a reproducible build. Record the final bridge choice after the probe.

## Alternatives and consequences

- A custom kernel would introduce boot and driver work before the desktop is testable. Using Linux lets the first increment focus on the shell.
- X11 as the primary display target would change the compositor integration path. Wayland is the native target, with XWayland reserved for later compatibility work.
- Starting the shell on the Sunrise compositor would couple two unproven components. A reference compositor lets each run independently and makes protocol gaps easier to isolate.
- A C++ backend would put core logic in a second systems language. The Rust-first approach adds build integration work at the Qt boundary, which the launcher probe must quantify.

A future ADR should identify the affected decision, evidence, alternatives, migration cost, and acceptance checks before changing this direction.
