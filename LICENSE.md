# Licensing

Orbox uses dual licensing by component (owner decision O-03, ADR 0002).

## Component map

| Component | License | Why |
|-----------|---------|-----|
| `boot` (orbox-boot and future boot stages) | MPL-2.0 | OS core: file-level copyleft keeps every improvement to these files open |
| kernel | MPL-2.0 | OS core |
| drivers | MPL-2.0 | OS core |
| runtime | Apache-2.0 | ecosystem-friendly for user-facing layers |
| compat (Win32 subsystem, NT environment emulation) | Apache-2.0 | ecosystem-friendly |
| crates (shared libraries) | Apache-2.0 | reusable without copyleft burden |
| tools (xtask, host utilities) | Apache-2.0 | build tooling |
| documentation | Apache-2.0 | maximal reuse |

SPDX expression for mixed-scope files (if any ever arise): `MPL-2.0 OR
Apache-2.0`. Every source file carries an `SPDX-License-Identifier` header.

## Canonical texts

- `licenses/MPL-2.0.txt` — Mozilla Public License 2.0 (downloaded,
  unmodified, from mozilla.org during the original M0 audit)
- `licenses/Apache-2.0.txt` — Apache License 2.0 (downloaded, unmodified,
  from apache.org during the original M0 audit)

## Third-party code policy

No third-party code or dependencies are currently vendored (`xtask` is
std-only by policy). Every future dependency must be listed in
[THIRD_PARTY.md](THIRD_PARTY.md) with its license and source, and must be
compatible with the component's license above. Copying any Microsoft
code, headers, or assets is prohibited by the clean-room protocol.
