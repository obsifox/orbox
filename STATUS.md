# Status

**Last updated:** 2026-10-21 (reconstruction build)
**Current milestone:** M0 complete → M1 (UEFI boot prototype) ready to start

## ⚠ Reconstruction notice — read first

The original M0 working copy was lost on 2026-10-21 when the container
environment was reset between sessions (filesystem wiped, toolchain
removed). The repository you are reading was **rebuilt from the session
record** of the original work on 2026-10-20. See
[RECOVERY.md](RECOVERY.md) for exactly what was reconstructed, what was
lost, and what was re-verified.

## Environment audit

### Linux container (original audit, 2026-10-20 — real results)

| Component | Result |
|-----------|--------|
| rustc / cargo | installed via rustup → **1.99.1** |
| rustfmt / clippy | **1.10.0 / 0.1.99** |
| rust-src | installed |
| x86_64-unknown-uefi target | installed |
| QEMU / OVMF | **NOT installable** (no root in container) — known limitation |
| git | 2.47.3 |
| Network | reachable (HTTP 200) |

QEMU + OVMF **must** be installed on the owner's Windows host before M1
boot testing; `C:\Program Files\qemu\share\edk2-x86_64-code.fd` is the
expected firmware location (guide: [docs/build.md](docs/build.md)).

### Reconstruction container (2026-10-21)

- Sandbox reset wiped the original tree and the Rust toolchain.
- Rust 1.99.1 re-installed via rustup (rustfmt 1.10.0, clippy 0.1.99,
  rust-src, x86_64-unknown-uefi) — matching the original audit exactly.
- The rebuilt tree was **actually verified**: 12/12 tests pass, lint
  green, build green (dev + release), stubs exit 3, `info` audit runs.
  Five real defects were found and fixed during this verification —
  full list in [RECOVERY.md](RECOVERY.md).
- QEMU/OVMF remain absent here (no root) — unchanged limitation.

## M0 deliverables

- Build harness `xtask` (std-only): `build` `test` `fmt` `lint` `clean`
  `info` implemented; `run` `test-boot` stubs (exit 3 by design)
- Documentation: README, ROADMAP, build guide, budget framework
- Decision register: ADR 0001–0006, all owner-ratified
- Dual licensing applied: MPL-2.0 (boot/kernel/drivers) +
  Apache-2.0 (runtime/compat/crates/tools/xtask/docs)

## Open items

1. **Owner action:** install QEMU + OVMF on the Windows host, run
   `cargo xtask info`, return the output (M1 prerequisite).
2. **Owner action:** re-supply the original `AGENT_SPEC.md` (owner-authored;
   lost in the reset — see RECOVERY.md).
3. **M1 kickoff:** `orbox-boot` UEFI loader; acceptance marker
   `ORBOX-BOOT: ok` on serial from `cargo xtask test-boot`.
