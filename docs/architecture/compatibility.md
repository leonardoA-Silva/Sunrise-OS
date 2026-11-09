# Application compatibility requirements

## Independence from Windows

Windows can inform compatibility semantics through lawful public documentation and observed behavior. Sunrise OS must build, boot, update, and operate on Linux without a Windows installation, VM, dual-boot setup, proprietary Windows system DLLs, or Windows services. A Windows machine used for comparative research is outside the Sunrise build and runtime path.

Sunrise Bridge is the planned isolated subsystem for application profiles, runtime selection, and diagnostics. Wine is a research candidate for independently implemented compatibility. Proton is a reference for games. The choice of runtime and application-specific configuration depends on reproducible evidence and license review.

## Mandatory future Adobe milestone

Broad compatibility with Adobe desktop applications is a mandatory future project requirement. An exact supported-app list, target versions, required workflows, and release acceptance threshold must be agreed before claiming the milestone. The requirement covers real work, including opening, editing, saving, and exporting supported files, plus GPU or plugin workflows where specified.

Tests use legitimate installers, accounts, and licensing. Record authentication, DRM, API, codec, and graphics blockers as separate findings. Keep compatibility claims tied to the tested scope and record unresolved feasibility risks.

## Evidence by application and version

| Dimension | Evidence |
| --- | --- |
| Environment | Sunrise revision, runtime version and configuration, hardware, GPU driver, display backend |
| Installation and identity | Installer version and lawful acquisition, installation result, sign-in and activation behavior |
| Core workflows | Launch and UI, file open, edit, save, and reopen, with input format and expected result |
| Extended workflows | Plugin loading, GPU acceleration, rendering, codecs, and export where applicable |
| Stability | Repeated workflow, observed failures, diagnostics, and reproduction steps |

Record PASS, FAIL, BLOCKED, or NOT RUN by workflow. Separate partial success from overall compatibility. Apply the [clean-room policy](../clean-room/policy.md) throughout research and testing.
