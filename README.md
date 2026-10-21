# Orbox

A clean-room, open-source operating system that natively runs Windows
applications: x86_64 code executes directly on the CPU — no emulation, no
JIT translation — behind an NT-style kernel environment and a Win32
subsystem.

> **Status:** Milestone 0 (bootstrap) complete. Next: Milestone 1 — UEFI
> boot prototype. See [STATUS.md](STATUS.md) and [ROADMAP.md](ROADMAP.md).

## What Orbox is

Orbox targets a specific, honest goal: the ability to run unmodified
Windows executables as first-class citizens of a native OS. Windows
programs speak two languages: the machine (x86_64 instructions) and the
system (NT/Win32 API surface). Orbox speaks both. Machine code runs
untranslated; the system-call surface is re-implemented by an NT-style
executive and a Win32 compatibility subsystem, so applications cannot
distinguish the host.

What Orbox is **not**: an emulator, a VM wrapper around existing Windows
components, or a Linux distribution with a compatibility shim. No code,
headers, or assets from Microsoft are used; everything is written from
scratch against publicly documented behavior, verified by black-box
testing.

## Quick start

Requirements: Rust 1.99.1 (pinned via `rust-toolchain.toml`), QEMU 8+,
OVMF firmware. Full setup guide: [docs/build.md](docs/build.md).

```sh
# Environment audit — verify your host is ready
cargo xtask info

# Build, test, format, lint (all exit 0 on success)
cargo xtask build
cargo xtask test
cargo xtask fmt
cargo xtask lint

# Not implemented yet — land in Milestone 1 (exit code 3)
cargo xtask run
cargo xtask test-boot
```

Exit-code convention (project-wide): `0` success · `1` failure ·
`2` usage error · `3` not implemented.

## Repository layout

```
Cargo.toml            workspace manifest
rust-toolchain.toml   pinned toolchain (1.99.1)
xtask/                build automation (std-only, zero dependencies)
docs/
  build.md            toolchain + QEMU/OVMF setup, acceleration notes
  budget.md           performance/size budget framework (no invented numbers)
  architecture/       architecture decision records (ADR 0001–0006)
licenses/             canonical license texts (MPL-2.0, Apache-2.0)
reports/              milestone reports and environment audits
```

## Licensing

Dual licensing by component (ADR 0002):

| Scope | License | Rationale |
|-------|---------|-----------|
| `boot`, kernel, drivers | **MPL-2.0** | file-level copyleft keeps the OS core open |
| runtime, compat, crates, tools, xtask, docs | **Apache-2.0** | permissive for the application/tooling ecosystem |

See [LICENSE.md](LICENSE.md) for the component map and
[THIRD_PARTY.md](THIRD_PARTY.md) for the (currently empty) dependency
register.
