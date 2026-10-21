# ADR 0002 — Licensing: MPL-2.0 + Apache-2.0 Dual Scheme

- **Status:** Accepted (owner decision O-03/D-09, 2026-10-20)
- **Context:** M0 decision round
- **Deciders:** owner (ratified), agent (analysis)

## Context

An OS with a compatibility goal attracts contributions at two very
different layers: the OS core (boot/kernel/drivers — where users care
that improvements stay open) and the ecosystem layers (runtime, compat
subsystem, shared crates, tools, docs — where permissive terms maximize
adoption and reuse).

## Decision

Component-split dual licensing:

- **MPL-2.0** — `boot`, kernel, drivers. File-level copyleft: changes to
  licensed files must be shared, but linking/combining with proprietary
  code at larger scale remains possible — appropriate for an OS core.
- **Apache-2.0** — runtime, compat, crates, tools, xtask, documentation.
  Permissive with an explicit patent grant — appropriate for ecosystem
  layers.

Canonical texts live unmodified in `licenses/` (MPL-2.0.txt,
Apache-2.0.txt); every source file carries an
`SPDX-License-Identifier` header. The full map: [LICENSE.md](../../LICENSE.md).

## Consequences

- SPDX headers must be added to every new file at review time.
- Third-party dependencies must match their component's license
  (THIRD_PARTY.md register gates this).
- The split is an owner-ratified decision; moving a component between
  licenses requires a new ADR.
