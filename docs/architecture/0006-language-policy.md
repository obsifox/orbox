# ADR 0006 — Language Policy: Rust Everywhere

- **Status:** Accepted (owner decision O-06/D-12, 2026-10-20)
- **Context:** M0 decision round
- **Deciders:** owner (ratified), agent (analysis)

## Context

An OS is exactly the domain where memory-safety bugs become security
bugs: loaders, syscalls, and drivers parse hostile input in ring 0 or
adjacent to it. The implementation-language choice also fixes the
toolchain story, hiring/contribution surface, and the audit story.

## Decision

**Rust for all components** — boot, kernel, drivers, runtime, compat
subsystem, crates, tools, and xtask.

- **Assembly only where strictly required**: early boot entry, context
  switch, special-register access. Every asm block carries a comment
  explaining why it must exist.
- **C is not used**; introducing a C component (e.g. for a prebuilt
  driver blob or firmware interaction) requires a dedicated ADR
  justifying it per case.
- The build bootstrap (`xtask`) is additionally **std-only, zero
  third-party dependencies**, so a fresh machine audits what it runs
  before trusting it.

## Consequences

- `no_std` + custom-alloc discipline for kernel-side crates; `alloc`
  available once the kernel heap exists (M2).
- The pinned toolchain (`rust-toolchain.toml`, 1.99.1) is part of the
  contract; toolchain bumps are owner-visible decisions, never silent.
- `x86_64-unknown-uefi` (Tier 2) is the first bare-metal target; its
  quirks (no OS, UEFI boot services lifetime) are handled in M1's
  `orbox-boot` and become the template for later bare-metal crates.
- Unsafe Rust is allowed where the domain requires it ( MMIO, boot
  hand-off) under the review rule: every `unsafe` block states its
  invariant in a comment.
