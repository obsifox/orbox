# ADR 0003 — Security Model: Capability-Based

- **Status:** Accepted (owner decision O-01/D-07, 2026-10-20)
- **Context:** M0 decision round; shapes kernel object model (M2+)
- **Deciders:** owner (ratified), agent (analysis)

## Context

Windows security rests on tokens, SIDs, and discretionary ACLs checked
by the security reference monitor. Orbox must run Windows software, but
is not obligated to re-implement Windows' internal security model as its
**native** model.

Options considered:

1. **Replicate NT tokens/ACLs natively** — maximum compatibility inside
   the kernel; but inherits a model widely regarded as complex, and
   makes the kernel pay for legacy design.
2. **Capability-based native model** — every object (process, thread,
   file, section, port) is reached only through unforgeable capabilities
   with fine-grained rights; the kernel checks capability possession.
3. DAC-on-VFS-only hybrid — rejected early: too little structure for a
   system whose whole point is a new NT-style environment.

## Decision

Orbox natively implements a **capability-based** security model:

- Kernel handles are capabilities: (object, rights set) pairs, obtained
  only from a parent/creator or a capability broker.
- No ambient authority: a process can do only what its capabilities
  allow.
- **Win32 security APIs (`OpenProcessToken`, `GetTokenInformation`,
  ACL functions, etc.) are emulated by the compat layer**, mapping them
  onto capabilities — applications observe plausible NT semantics
  without the kernel implementing NT tokens.

## Consequences

- The kernel object model (M2) must be capability-first from its first
  commit — retrofitting capabilities is the classic failed path.
- The compat layer carries a mapping complexity cost; its emulation of
  token/ACL semantics needs a documented model + security review before
  M4, because it becomes a security boundary by proxy.
- Sandboxing falls out almost for free (withhold capabilities), which
  benefits the fuzzing/validation strategy for loaders (ADR 0004).
