# RECOVERY — Environment Reset & Reconstruction Notice

**Read this before trusting anything in this repository.**

## What happened

| Event | Date |
|-------|------|
| Original M0 work performed | 2026-10-20 |
| Original git history: `e7e5c7e` (docs) → `0f56e7c` (build system) → `4bc0857` (decisions + LICENSE) | 2026-10-20 |
| **Container environment reset** — filesystem wiped, `orbox/` tree, `worklog.md`, and the Rust toolchain destroyed | 2026-10-21 |
| Reconstruction of this tree from the session record | 2026-10-21 |

## What this repository is

A **rebuild from the conversation/session record**, not the original
bytes. The session record preserved: the full design of every file, the
decision set (O-01…O-06 with the owner's exact choices), the audit
results, the test inventory (11 tests), and the milestone protocol. The
reconstruction reproduces that content faithfully; it is not guaranteed
to be byte-identical to the originals.

## What was lost and is NOT in this tree

1. **`AGENT_SPEC.md`** — the owner-authored authoritative spec. The
   agent never held its full verbatim text as a recoverable artifact.
   **The owner must re-supply it** into the repo root.
2. **Exact original file wording** — prose was rewritten from the
   record; meaning is preserved, phrasing may differ.
3. **Original git objects** — commit hashes `e7e5c7e`/`0f56e7c`/
   `4bc0857` are recorded here for reference only; this history is new.
4. **`worklog.md`** — a best-effort snapshot lives at
   `reports/worklog-snapshot.md`, reconstructed from the record.

## What was re-verified during reconstruction

The container was reset to "no Rust installed". The following were
actually re-executed during reconstruction:

| Check | Result |
|-------|--------|
| rustup install (1.99.1 minimal + rustfmt 1.10.0 + clippy 0.1.99 + rust-src) | ✅ done |
| `x86_64-unknown-uefi` target | ✅ installed |
| `cargo xtask test` (rebuilt tree) | ✅ **12/12 pass** (the 11 original tests + 1 added for probe-output parsing) |
| `cargo xtask lint` (rebuilt tree) | ✅ green — after 3 real fixes below |
| `cargo xtask build` (dev + release) | ✅ green |
| `cargo -q xtask info` | ✅ correctly reports uefi PASS; QEMU/OVMF MISS (truthful — they are not in this container) |
| stub exit codes | ✅ `run` → 3, `test-boot` → 3 (as designed) |

### Real failures found & fixed during reconstruction (protocol: logged, not hidden)

1. Missing `.cargo/config.toml` `xtask` alias — `cargo xtask` did not
   route to the xtask crate until the alias file was added.
2. Formatting diff — caught by the lint fmt gate; fixed via
   `cargo xtask fmt` (same class of failure as the original session).
3. `Option<&str>::as_deref()` no-op — caught by clippy `-D warnings`.
4. **Functional bug:** `probe()` returned only the first line of a
   command's stdout, so `uefi_target_installed()` could never see the
   uefi line of `rustup target list --installed`. Found by actually
   running `info`; fixed with `probe_full()` + a new unit test (12th).
5. `manual_contains` clippy lint — fixed per suggestion.

## Standing protocol (unchanged)

- Real-environment audits only; no fabricated results.
- Failures are logged, not hidden.
- Each milestone ends with a report to the owner and a go/no-go wait.
- The owner's `AGENT_SPEC.md` is authoritative when re-supplied; any
  conflict between it and this reconstruction is resolved in its favor.
