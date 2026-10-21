# Milestone 0 Report — Bootstrap

- **Date:** 2026-10-20 (original) · reconstructed 2026-10-21
- **Agent work log:** see `reports/worklog-snapshot.md`
- **Reconstruction provenance:** RECOVERY.md

## Scope delivered

1. **Environment audit** — real results in
   [M0-AUDIT-2026-10-20.md](M0-AUDIT-2026-10-20.md). Rust 1.99.1
   toolchain stood up; QEMU/OVMF impossible in container (no root) —
   recorded as owner-host prerequisite.
2. **Build harness (`xtask`)** — std-only, zero third-party deps:
   - implemented: `build` `test` `fmt` `lint` `clean` `info`
   - stubs by design: `run` `test-boot` (exit code 3)
   - exit-code convention: 0 success / 1 failure / 2 usage / 3 not-implemented
   - 11 unit tests (argument parsing, exit-code mapping, helpers)
3. **Documentation** — README, ROADMAP (M0→M7), STATUS, build guide,
   budget framework (no invented numbers), decision register.
4. **Decisions ratified by owner** — O-01…O-06 resolved and recorded as
   ADR 0001–0006 / D-07…D-12 (DECISIONS.md). Owner choices differ from
   agent recommendations where noted in the ADR analysis sections.
5. **Licensing** — dual scheme applied end-to-end (ADR 0002);
   canonical license texts downloaded unmodified into `licenses/`.

## Verification state

| When | What | Result |
|------|------|--------|
| 2026-10-20 (original) | `cargo xtask test` | 11/11 pass (observed) |
| 2026-10-20 (original) | `cargo xtask lint` | green after one fmt fix (real failure logged) |
| 2026-10-20 (original) | `cargo xtask build` | green |
| 2026-10-21 (reconstruction) | `cargo xtask test` | **12/12 pass** (11 original + 1 new for probe parsing) |
| 2026-10-21 (reconstruction) | `cargo xtask lint` | green after 3 real fixes (see RECOVERY.md) |
| 2026-10-21 (reconstruction) | `cargo xtask build` dev+release | green |
| 2026-10-21 (reconstruction) | `cargo xtask info` + stub exit codes | uefi PASS / QEMU+OVMF MISS (truthful); run/test-boot → exit 3 |

## What M0 does NOT claim

- No boot was performed (no QEMU in container) — nothing boot-related is
  claimed anywhere.
- `run` / `test-boot` are stubs; no behavior is claimed for them beyond
  exit code 3.
- The Windows-host audit has not happened yet — `cargo xtask info`
  output from the owner's machine is outstanding.

## Exit criteria — status

| Criterion | Status |
|-----------|--------|
| Toolchain audited on a real host | ✅ Linux container (Windows host pending) |
| Build/test/lint/fmt automation works | ✅ (original session; re-verified per RECOVERY.md) |
| Documentation set complete | ✅ |
| All open decisions ratified | ✅ O-01…O-06 → ADR 0001–0006 |
| QEMU/OVMF available for M1 | ⏳ owner Windows host |

## Owner actions requested

1. Run `cargo xtask info` on the Windows host (after installing QEMU +
   OVMF per docs/build.md) and return the output.
2. Confirm go for Milestone 1 (UEFI boot prototype, `ORBOX-BOOT: ok`
   serial marker acceptance).
