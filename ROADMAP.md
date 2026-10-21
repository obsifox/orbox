# Roadmap

Milestones are delivered one at a time. Each milestone ends with a report
to the owner and an explicit go/no-go decision before the next one starts
(per the project protocol). Success criteria are verifiable by machine
wherever possible — a milestone is not "done" because its author says so.

## M0 — Bootstrap ✅ (complete)

Repo, protocol, toolchain audit, build harness.

- [x] Environment audit on the Linux container (real results recorded)
- [x] `xtask` harness: `build` `test` `fmt` `lint` `clean` `info`
      implemented; `run` `test-boot` as intentional stubs (exit 3)
- [x] Documentation set + decision register (ADR 0001–0006)
- [x] Dual licensing applied (MPL-2.0 / Apache-2.0)
- [x] Windows-host verification pending: owner runs `cargo xtask info`
      and returns the output (QEMU + OVMF must be installed there)

## M1 — UEFI boot prototype (next)

First silicon contact: code runs on real (virtual) hardware.

- Deliverable: `orbox-boot` — UEFI boot loader; FAT ESP image; GOP
  framebuffer + serial diagnostics
- Acceptance: `cargo xtask test-boot` boots headless QEMU/OVMF and the
  serial log contains the marker `ORBOX-BOOT: ok`
- Unblocks: `run` and `test-boot` xtask commands (replace stubs)

## M2 — Kernel core

- Orbox native kernel: scheduling, physical/virtual memory management,
  interrupt handling; capability-based object model foundation (ADR 0003)
- Acceptance: kernel boots from `orbox-boot`, runs user-mode processes,
  demonstrates preemption + paging on QEMU

## M3 — NT-style executive

- System-call interface, process/thread model, handles
- Native Orbox executable format loader (ADR 0004, native side)
- Acceptance: native processes perform syscalls through the executive

## M4 — Win32 subsystem foundations

- Win32 compat subsystem as user-mode service (ADR 0001: user-mode
  subsystems are first-class citizens from day one)
- Basic process/heap/console/file Win32 surface
- Acceptance: a small native Win32 test binary (built from scratch, not
  Microsoft code) launches and terminates cleanly

## M5 — PE loader + first real Windows app

- PE32+ loader with strict validation + fuzzing (ADR 0004)
- Acceptance: a selected open-source Windows x64 executable runs
  unmodified under Orbox

## M6 — Drivers, storage, input

- Kernel-mode driver interface with versioned ABI (ADR 0001)
- AHCI/NVMe storage, filesystem (FAT32 first), keyboard/mouse
- Acceptance: persistent storage round-trip from a Win32 app

## M7 — Installer, SDK, release engineering

- Bootable installer image, SDK packaging, CI on Linux + Windows hosts
- Acceptance: third party can build and boot Orbox from the release

## Deferred (tracked, not forgotten)

- 32-bit x86 and ARM/AArch64 ports — decision deferred by owner (ADR 0005);
  WOW64 compatibility path stays architecturally open; revisit after the
  Tier-3 target evaluation
