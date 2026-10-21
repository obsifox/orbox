# Performance & Size Budget

This document defines the budget framework and the discipline for filling
it. **It deliberately contains no invented numbers.** A budget entry is
written only after it has been measured on a real run (QEMU with a stated
accelerator, or bare metal) and the measurement method is recorded next to
the value.

## Rules

1. No budget number may be written without a corresponding measurement
   (command, QEMU flags, host machine, date).
2. Budgets are checked in CI once a measurement exists for them; a
   regression beyond the owner-ratified threshold fails the milestone
   report, not silently.
3. Targets (aspirational) are marked `TBD` until the first real
   measurement of that category exists; then a target is proposed to the
   owner as a decision.

## Budget categories

| Category | Unit | Current value | Target | Measured how |
|----------|------|---------------|--------|--------------|
| Boot latency: power-on → `ORBOX-BOOT: ok` marker | ms | TBD — first measurement lands with M1 | TBD | QEMU serial timestamp, `-d` trace + host timer |
| Kernel image size (release) | KiB | TBD — kernel does not exist until M2 | TBD | `cargo xtask build --release` artifact size |
| Boot loader image size | KiB | TBD — `orbox-boot` lands in M1 | TBD | artifact size at M1 acceptance |
| Syscall round-trip latency (M3+) | cycles | TBD | TBD | user↔kernel ping test on QEMU (TCG numbers labeled as TCG-only) |
| Win32 subsystem call overhead (M4+) | µs | TBD | TBD | compat layer benchmark |
| Memory footprint at shell (M7+) | MiB | TBD | TBD | QEMU guest memory probe |

## Why this discipline

An OS project drifts into self-deception faster than most: "it boots" is
meaningless without the boot log; "it's fast" is meaningless without the
timer. Every TBD above is a commitment to measure, not an aspiration to
narrate. The M0 container had no QEMU (no root), so no boot-related
number could honestly be produced — and none is written here.
