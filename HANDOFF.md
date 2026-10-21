# HANDOFF — Continuing Orbox on Your Own Machine (with an AI agent)

This package is a self-contained handoff: unzip it, verify it, and hand
the folder to an AI coding agent (e.g. Z code) to continue from
Milestone 1.

## 1. Unzip and verify

```bat
unzip orbox-m0.zip -d orbox
cd orbox
```

Expected layout: `Cargo.toml`, `rust-toolchain.toml`, `xtask/`, `docs/`,
`reports/`, `licenses/`, and the `RECOVERY.md` notice.

## 2. Install prerequisites (once)

1. Rust via rustup — <https://rustup.rs> (the pinned 1.99.1 toolchain
   downloads automatically on first build).
2. QEMU for Windows + OVMF — see `docs/build.md` §2. Verify
   `C:\Program Files\qemu\share\edk2-x86_64-code.fd` exists.

## 3. Verify the tree yourself (do not skip)

```bat
cargo xtask info     <- send this output to the agent
cargo xtask test
cargo xtask lint
```

All three must be green before M1 starts. If anything fails, that is
data for the agent, not a blocker — paste the failure verbatim.

## 4. Re-supply the authoritative spec

The original `AGENT_SPEC.md` (owner-authored) was lost in the
environment reset. Place it at the repo root. It is authoritative over
everything else (see RECOVERY.md).

## 5. First prompt for your agent

Paste this as the agent's kickoff message:

```
You are continuing the Orbox project. Read, in order:
RECOVERY.md, STATUS.md, ROADMAP.md, DECISIONS.md, docs/build.md,
reports/M0-REPORT.md. The working protocol: real-environment audits
only; never fabricate test results; log failures; end each milestone
with a report and wait for my go/no-go.

First: run `cargo xtask info` and reconcile the output with
reports/M0-AUDIT-2026-10-20.md. Then start Milestone 1 per ROADMAP:
implement orbox-boot (UEFI loader, FAT ESP image, GOP + serial
diagnostics) and replace the run/test-boot stubs in xtask. Acceptance:
`cargo xtask test-boot` boots headless QEMU/OVMF and the serial log
contains the marker ORBOX-BOOT: ok. Report and stop for my decision.
```

## 6. Milestone protocol (unchanged)

- One milestone at a time; report → owner decision → next milestone.
- No invented numbers anywhere (docs/budget.md discipline).
- Any new architectural question becomes an ADR for owner ratification.
