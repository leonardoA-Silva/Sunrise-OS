# Clean-room and licensing policy

## Research and implementation

- Use original requirements and lawful public documentation. Keep proprietary code, disassembly, extracted assets, fonts, icons, logos, and copied interfaces out of the repository and implementation inputs.
- Record the public source, relevant version, observed behavior, and intended test. Separate research observations from independent implementation work. If reverse engineering is lawful and permitted for the particular case, document the basis and boundaries before relying on it. Refer unresolved questions for appropriate review.
- Use Windows behavior as a compatibility reference. Keep Windows installations, system files, and proprietary Windows services outside the build and runtime design.
- Use legitimately obtained Adobe installers and licenses for authorized tests. Redistribution requires permission. Respect authentication, DRM, and licensing controls and log blockers rather than bypassing them.

## Dependency intake

For each imported dependency or asset, record the upstream source, exact version, license, notices, intended use, and applicable redistribution obligations. Preserve third-party terms. Maintain a dependency inventory and third-party license notices as the project starts integrating dependencies. Include `NOTICE` and SPDX identifiers where applicable.

The repository's [Apache-2.0 license](../../LICENSE) covers first-party code. The Linux kernel and other third-party components keep their own licenses. Review the actual mode of integration, especially when GPLv2-only code is involved.

For per-application research and evidence, use the [compatibility requirements](../architecture/compatibility.md).
