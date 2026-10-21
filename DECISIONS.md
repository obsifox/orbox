# Decision Register

Decisions D-01…D-06 were ratified earlier inside the original
`AGENT_SPEC.md` (owner-authored; that document was lost in the 2026-10-21
environment reset and must be re-supplied by the owner — see
RECOVERY.md). Decisions D-07…D-12 below correspond to the open decisions
O-01…O-06 that the owner resolved during Milestone 0 on 2026-10-20; each
is formalized in an ADR under `docs/architecture/`.

## M0 decision round (owner ratification, 2026-10-20)

| ID | Question | Owner decision | ADR |
|----|----------|----------------|-----|
| D-07 (was O-01) | Security model | **Capability-based**, not NT tokens/ACLs; Win32 security APIs (OpenProcessToken, ACLs…) are emulated by the compat layer | [0003](docs/architecture/0003-security-model.md) |
| D-08 (was O-02) | Kernel architecture | **NT-style hybrid**: modular kernel + user-mode subsystems/services as first-class citizens from day one; drivers kernel-mode initially, with versioned, migration-ready interfaces | [0001](docs/architecture/0001-kernel-architecture.md) |
| D-09 (was O-03) | Licensing | **MPL-2.0** (boot/kernel/drivers) + **Apache-2.0** (runtime/compat/crates/tools/xtask/docs) | [0002](docs/architecture/0002-licensing.md) |
| D-10 (was O-04) | Executable format | **Dual format**: native Orbox format + PE (Windows compatibility only); both loaders strictly validated and fuzzed | [0004](docs/architecture/0004-executable-format.md) |
| D-11 (was O-05) | 32-bit / ARM | **Deferred**; WOW64 path stays open; revisit after Tier-3 evaluation | [0005](docs/architecture/0005-arch-tiering-32bit-arm.md) |
| D-12 (was O-06) | Implementation language | **Rust everywhere**; assembly only where strictly required; C requires a dedicated ADR per case | [0006](docs/architecture/0006-language-policy.md) |

## Rule

No open decision may be silently absorbed into implementation. Any new
architectural question becomes an ADR draft; the owner ratifies or
rejects it before the affected code lands.
