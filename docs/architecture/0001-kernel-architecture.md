# ADR 0001 — Kernel Architecture: NT-Style Hybrid

- **Status:** Accepted (owner decision O-02/D-08, 2026-10-20)
- **Context:** M0 decision round; determines everything downstream
- **Deciders:** owner (ratified), agent (analysis)

## Context

Orbox must run unmodified Windows applications. Three classic kernel
architectures were evaluated:

1. **Monolithic** — everything in ring 0. Fast IPC; but every driver and
   service is privileged, and Windows' own model (session manager,
   service processes, user-mode subsystems) does not map cleanly onto it.
2. **Microkernel** — kernel minimal; drivers/services in user mode.
   Clean isolation; but IPC cost and complexity are high, and the Win32
   surface requires many moving parts to be correct early.
3. **Hybrid (NT-style)** — modular kernel containing the executive
   primitives in ring 0, with subsystems and services running in user
   mode as first-class components.

## Decision

Orbox adopts the **NT-style hybrid**: a modular kernel with user-mode
subsystems and services as first-class citizens from day one.

- The kernel provides scheduling, memory management, IPC, handles, and
  the capability object model (ADR 0003).
- The NT-style environment and the Win32 compatibility subsystem are
  user-mode services, not kernel code.
- Drivers are **kernel-mode initially** (pragmatic for storage/input in
  the early milestones), but the driver interface is **versioned and
  migration-ready** so drivers can move to user mode later without an
  interface redesign.

## Consequences

- The Win32 subsystem being user-mode means a subsystem crash is
  recoverable — it must be designed as a restartable service.
- Versioned driver ABI from the start avoids the classic "drivers frozen
  forever" trap.
- Kernel must expose clean user-mode service bootstrap (the NT
  equivalent of smss/session manager paths) earlier than a monolithic
  project would.
- Performance-critical paths (scheduling, memory) stay in-kernel, so
  syscall/IPC budgets (docs/budget.md) remain reachable.
