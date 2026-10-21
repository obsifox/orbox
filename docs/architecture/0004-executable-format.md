# ADR 0004 — Executable Format: Dual (Native Orbox + PE)

- **Status:** Accepted (owner decision O-04/D-10, 2026-10-20)
- **Context:** M0 decision round; lands concretely in M3 (native) and M5 (PE)
- **Deciders:** owner (ratified), agent (analysis)

## Context

Orbox needs a native executable format of its own, and — for its reason
to exist — must load real Windows PE32+ images. Choosing only one of the
two would fail a core requirement either way:

- Native-only: no Windows compatibility (defeats the project).
- PE-only: Windows legacy formats become Orbox's permanent native ABI,
  freezing its platform onto 1990s decisions.

## Decision

**Dual format, with a clear hierarchy:**

1. **Native Orbox format** — first-class. Designed for the capability
   model (ADR 0003): explicit imports, typed sections, capability
   declarations in the image, no historical baggage.
2. **PE (PE32+)** — supported strictly for Windows compatibility. The
   PE loader translates PE semantics onto the native process model;
   it is compat-critical code, not the platform's foundation.

**Both loaders require strict validation and fuzzing before any
milestone that ships them is reported complete.** Malformed images are
the oldest exploit class in OS history; the loaders are security
boundaries from day one, and both get the same validation framework.

## Consequences

- Two loader code paths must be maintained; the native loader is the
  reference implementation and lands first (M3).
- PE loader work (M5) includes: strict header/field validation, fuzz
  corpus in CI, and an explicit non-goals list (e.g. 16-bit/MS-DOS
  stubs are parsed only as far as required by spec).
- The compat layer may rely on PE-specific quirks (TLS callbacks,
  exception handling tables); each is tracked as a compatibility test
  case, never as loader-special-case sprawl.
