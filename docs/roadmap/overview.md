# Project roadmap

Milestones describe dependency order and acceptance gates. They do not imply current functionality or fixed delivery dates.

| Milestone | Exit evidence |
| --- | --- |
| Developer foundation | [Sprint 01](sprint-01.md) demonstrates the reference shell, Rust-backed launcher, and separate nested compositor probe |
| Integrated desktop session | A stable guest session provides window controls, panel, dock, launcher, and basic preferences with native Wayland clients |
| Sunrise compositor | Real clients demonstrate input, focus, move and resize, monitor handling, rendering, and diagnosed fallback paths |
| Desktop services | Files, settings, notifications, power, network, audio, and portals work in the session. Validate accessibility and later XWayland and Flatpak integration |
| Distribution | Verify installation and recovery in a VM and supported hardware. Test secure updates, rollback, and failure recovery |
| Windows application compatibility | Run selected apps through isolated profiles on Linux and publish results by version and workflow. Verify independence from Windows components |
| Adobe application compatibility | Meet the mandatory future [Adobe compatibility requirements](../architecture/compatibility.md) using agreed app versions, workflows, and acceptance criteria |

## Performance and polish

Measure startup, idle memory and CPU, frame times, and GPU behavior on recorded hardware before tuning. Initial investigation targets are 60 FPS at 60 Hz, roughly 16.7 ms per frame, below 1.5 GB idle RAM, and around 3% idle use of one CPU core or less under a defined methodology. These are provisional targets. Evaluate Sunrise Glass effects alongside stability and accessibility.
