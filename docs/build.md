# Build guide

How to get a machine from "nothing installed" to "Orbox builds and boots
under QEMU". Windows host setup is required before Milestone 1 (the Linux
audit container cannot install QEMU — no root).

## 1. Rust toolchain

Orbox pins the toolchain via `rust-toolchain.toml` (1.99.1 + rustfmt +
clippy + rust-src + x86_64-unknown-uefi). Any recent rustup automatically
downloads the pinned components on first build.

**Windows:** install rustup from <https://rustup.rs> (default options).
**Linux:** `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

If components are missing: `rustup component add rustfmt clippy rust-src`
and `rustup target add x86_64-unknown-uefi`.

## 2. QEMU + OVMF firmware

Boot testing (M1 onward) needs QEMU ≥ 8 with UEFI firmware.

**Windows:**
1. Download the QEMU installer from <https://qemu.org/download/#windows>
   and install to `C:\Program Files\qemu`.
2. Verify the OVMF firmware file exists:
   `C:\Program Files\qemu\share\edk2-x86_64-code.fd`
3. Add `C:\Program Files\qemu` to `PATH` so `cargo xtask info` finds
   `qemu-system-x86_64`.

**Linux (Debian/Ubuntu):** `sudo apt install qemu-system-x86 ovmf`
**Linux (Fedora):** `sudo dnf install qemu-kvm edk2-ovmf`

## 3. Acceleration

| Host | Backend | Notes |
|------|---------|-------|
| Windows 10/11 Pro | WHPX (`-accel whpx`) | Hyper-V platform must be enabled |
| Linux | KVM (`-accel kvm`) | needs `/dev/kvm` access |
| Any | TCG (default fallback) | slower, always available |

## 4. xtask commands

| Command | Effect | Exit codes |
|---------|--------|------------|
| `cargo xtask info` | host environment audit | 0 |
| `cargo xtask build [--release] [--target T]` | build workspace | 0/1/2 |
| `cargo xtask test` | run all tests | 0/1 |
| `cargo xtask fmt [--check]` | format (or verify) sources | 0/1/2 |
| `cargo xtask lint` | fmt-check + clippy `-D warnings` | 0/1 |
| `cargo xtask clean` | remove `target/` (delayed-clean on Windows if files are locked) | 0/1 |
| `cargo xtask run` | QEMU boot — **stub until M1** | 3 |
| `cargo xtask test-boot` | headless boot test — **stub until M1** | 3 |

Convention: `0` success · `1` failure · `2` usage error · `3` not
implemented. CI and scripts should branch on these codes only.

## 5. Windows delayed clean

On Windows, `cargo xtask clean` may fail with locked files (editor or
debugger holding `target/`). Instead of dying, xtask moves the tree to
`.orbox-trash/target-<timestamp>/` and schedules a detached
`rmdir /s /q`. The next `clean` clears stale trash automatically. Add
`.orbox-trash/` to antivirus exclusions if scans slow the removal.

## 6. Troubleshooting

- **`rustc not found` after install** — reopen the shell so `PATH`
  updates, or run `rustup show` to confirm the active toolchain.
- **Pinned toolchain re-downloads every time** — `rustup show` once per
  machine; the pin is intentional (identical behavior everywhere).
- **`qemu-system-x86_64: not found` in `info`** — QEMU not on `PATH`
  (Windows: add `C:\Program Files\qemu`).
- **OVMF `MISS` in `info`** — the firmware file listed in §2 is absent;
  reinstall QEMU or point xtask at the file once M1 lands.
