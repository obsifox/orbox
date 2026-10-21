# ADR 0005 — Architecture Tiering: 32-bit x86 and ARM Deferred

- **Status:** Accepted (owner decision O-05/D-11, 2026-10-20)
- **Context:** M0 decision round
- **Deciders:** owner (ratified), agent (analysis)

## Context

Should Orbox target 32-bit x86 (for legacy Win32 binaries) or ARM/AArch64
(including Windows-on-ARM emulation scenarios) from the start? Every
additional early target multiplies: boot paths, ABI details, driver
validation matrix, and CI time — at the stage where zero lines of
kernel code exist.

## Decision

- **x86_64 is the sole Tier-1 target** through M7.
- **32-bit x86 and ARM are deferred**, not rejected. The architecture
  keeps the **WOW64 path open**: the process/executive model must not
  hardcode pointer widths or ABI assumptions that would make a future
  32-bit-on-64-bit compatibility layer impossible.
- A **Tier-3 evaluation** (experimental target bring-up) is scheduled
  after M7; the deferral is revisited there as an owner decision, not
  silently extended.

## Consequences

- Code review checks for width/ABI assumptions (`usize` misuse where a
  fixed-width integer is semantically required) from M2 onward — cheap
  now, expensive to retrofit.
- Early PE loader work (M5) targets PE32+ only; PE32 (32-bit) support
  rides on the same deferral decision.
- If the Tier-3 evaluation reopens 32-bit or ARM, it happens as a new
  ADR with a bring-up plan, not as drift.
