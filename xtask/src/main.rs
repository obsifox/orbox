//! xtask — Orbox build automation harness.
//!
//! Zero third-party dependencies: this crate uses `std` only, so the build
//! bootstrap stays auditable and reproducible on a clean machine.
//!
//! Exit codes (project-wide convention):
//!   0 = success
//!   1 = failure (a command ran and reported an error)
//!   2 = usage error (unknown command or malformed arguments)
//!   3 = not implemented (planned command, intentionally a stub)
//!
//! SPDX-License-Identifier: Apache-2.0

use std::env;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};

pub const EXIT_SUCCESS: u8 = 0;
pub const EXIT_FAILURE: u8 = 1;
pub const EXIT_USAGE: u8 = 2;
pub const EXIT_NOT_IMPLEMENTED: u8 = 3;

const USAGE: &str = "\
orbox xtask — build automation for the Orbox operating system

USAGE:
    cargo xtask <COMMAND> [OPTIONS]

COMMANDS:
    build       Build the workspace (pass --release for release mode)
    test        Run all workspace tests
    fmt         Format all sources (pass --check to verify only)
    lint        Run `fmt --check` and clippy with -D warnings
    clean       Remove build artifacts (delayed clean on Windows)
    info        Print a host environment audit report
    run         Run the OS image in QEMU             [not implemented yet]
    test-boot   Boot-test the UEFI image in QEMU     [not implemented yet]
    help        Print this help

OPTIONS:
    --release   Use the release profile (build)
    --check     Verify formatting without writing (fmt)
    --target T  Cross-compile for target triple T (build)

EXIT CODES:
    0 success   1 failure   2 usage error   3 not implemented
";

/// The command selected on the command line.
#[derive(Debug, Clone, PartialEq, Eq)]
enum Cmd {
    Build {
        release: bool,
        target: Option<String>,
    },
    Test,
    Fmt {
        check: bool,
    },
    Lint,
    Clean,
    Info,
    Run,
    TestBoot,
    Help,
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().skip(1).collect();
    match parse_args(&args) {
        Ok(cmd) => ExitCode::from(dispatch(cmd)),
        Err(msg) => {
            eprintln!("error: {msg}");
            eprintln!();
            eprint!("{USAGE}");
            ExitCode::from(EXIT_USAGE)
        }
    }
}

/// Parses raw command-line arguments (without the program name).
fn parse_args(args: &[String]) -> Result<Cmd, String> {
    let Some(first) = args.first() else {
        return Ok(Cmd::Help);
    };
    let rest = &args[1..];
    match first.as_str() {
        "build" => parse_build(rest),
        "test" => no_options(rest, "test").map(|()| Cmd::Test),
        "fmt" => parse_fmt(rest),
        "lint" => no_options(rest, "lint").map(|()| Cmd::Lint),
        "clean" => no_options(rest, "clean").map(|()| Cmd::Clean),
        "info" => no_options(rest, "info").map(|()| Cmd::Info),
        "run" => no_options(rest, "run").map(|()| Cmd::Run),
        "test-boot" => no_options(rest, "test-boot").map(|()| Cmd::TestBoot),
        "help" | "-h" | "--help" => Ok(Cmd::Help),
        other => Err(format!(
            "unknown command: {other} (run `cargo xtask help` for usage)"
        )),
    }
}

fn parse_build(rest: &[String]) -> Result<Cmd, String> {
    let mut release = false;
    let mut target: Option<String> = None;
    let mut i = 0;
    while i < rest.len() {
        match rest[i].as_str() {
            "--release" => release = true,
            "--target" => {
                i += 1;
                target = Some(
                    rest.get(i)
                        .ok_or("--target requires a triple, e.g. --target x86_64-unknown-uefi")?
                        .clone(),
                );
            }
            other => return Err(format!("unknown option for build: {other}")),
        }
        i += 1;
    }
    Ok(Cmd::Build { release, target })
}

fn parse_fmt(rest: &[String]) -> Result<Cmd, String> {
    let mut check = false;
    for flag in rest {
        match flag.as_str() {
            "--check" => check = true,
            other => return Err(format!("unknown option for fmt: {other}")),
        }
    }
    Ok(Cmd::Fmt { check })
}

fn no_options(rest: &[String], cmd: &str) -> Result<(), String> {
    if rest.is_empty() {
        Ok(())
    } else {
        Err(format!("command '{cmd}' takes no options"))
    }
}

fn dispatch(cmd: Cmd) -> u8 {
    match cmd {
        Cmd::Help => {
            print!("{USAGE}");
            EXIT_SUCCESS
        }
        Cmd::Build { release, target } => run_build(release, target.as_deref()),
        Cmd::Test => cargo(&["test", "--workspace"]),
        Cmd::Fmt { check } => run_fmt(check),
        Cmd::Lint => run_lint(),
        Cmd::Clean => run_clean(),
        Cmd::Info => run_info(),
        Cmd::Run => stub_not_implemented("run"),
        Cmd::TestBoot => stub_not_implemented("test-boot"),
    }
}

fn run_build(release: bool, target: Option<&str>) -> u8 {
    let mut args: Vec<String> = vec!["build".into(), "--workspace".into(), "--all-targets".into()];
    if release {
        args.push("--release".into());
    }
    if let Some(t) = target {
        args.push("--target".into());
        args.push(t.to_string());
    }
    let refs: Vec<&str> = args.iter().map(String::as_str).collect();
    cargo(&refs)
}

fn run_fmt(check: bool) -> u8 {
    if check {
        cargo(&["fmt", "--all", "--", "--check"])
    } else {
        cargo(&["fmt", "--all"])
    }
}

fn run_lint() -> u8 {
    // Lint = format gate + clippy gate. Formatting problems are reported
    // first because they are the cheapest to fix.
    if cargo(&["fmt", "--all", "--", "--check"]) != EXIT_SUCCESS {
        eprintln!();
        eprintln!("lint: formatting check failed — run `cargo xtask fmt` and retry");
        return EXIT_FAILURE;
    }
    cargo(&[
        "clippy",
        "--workspace",
        "--all-targets",
        "--",
        "-D",
        "warnings",
    ])
}

fn run_clean() -> u8 {
    let root = workspace_root();
    let trash = root.join(".orbox-trash");
    // Best-effort: clear stale trash left behind by earlier delayed cleans.
    let _ = std::fs::remove_dir_all(&trash);

    let target = root.join("target");
    if !target.exists() {
        println!("clean: nothing to remove (no target directory)");
        return EXIT_SUCCESS;
    }
    match std::fs::remove_dir_all(&target) {
        Ok(()) => {
            println!("clean: removed {}", target.display());
            EXIT_SUCCESS
        }
        Err(err) => {
            // Windows keeps files locked while editors/debuggers hold them.
            // Move the tree aside now, delete it from a detached process.
            let stamp = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_millis())
                .unwrap_or(0);
            let moved = trash.join(format!("target-{stamp}"));
            match std::fs::create_dir_all(&trash).and_then(|()| std::fs::rename(&target, &moved)) {
                Ok(()) => {
                    println!("clean: target locked ({err}); moved to {}", moved.display());
                    println!("clean: scheduling delayed removal…");
                    schedule_detached_delete(&moved);
                    EXIT_SUCCESS
                }
                Err(err2) => {
                    eprintln!("error: clean failed ({err}; rename: {err2})");
                    EXIT_FAILURE
                }
            }
        }
    }
}

/// Detached best-effort delete of a locked directory tree (Windows uses
/// `cmd /C rmdir`; other platforms never call this in practice).
fn schedule_detached_delete(path: &Path) {
    #[cfg(windows)]
    {
        let _ = Command::new("cmd")
            .args(["/C", "rmdir", "/s", "/q", &path.to_string_lossy()])
            .spawn();
    }
    #[cfg(not(windows))]
    {
        let _ = path;
    }
}

/// Prints a host environment audit: toolchain, components, cross target,
/// QEMU and OVMF firmware. Informational — always exits 0 unless probes
/// themselves fail hard.
fn run_info() -> u8 {
    println!("== Orbox environment audit ==");
    println!();
    println!(
        "[host]    os={} arch={} family={}",
        env::consts::OS,
        env::consts::ARCH,
        env::consts::FAMILY
    );
    println!();

    let mut critical_missing: Vec<&str> = Vec::new();

    let rustc = probe("rustc", &["--version"]);
    report("rustc", rustc.as_deref(), "rustup (https://rustup.rs)");
    if rustc.is_none() {
        critical_missing.push("rustc");
    }

    let cargo = probe_cargo(&["--version"]);
    report("cargo", cargo.as_deref(), "rustup (https://rustup.rs)");
    if cargo.is_none() {
        critical_missing.push("cargo");
    }

    let fmt = probe("rustfmt", &["--version"]);
    report("rustfmt", fmt.as_deref(), "rustup component add rustfmt");

    let clippy = probe_cargo(&["clippy", "--version"]);
    report("clippy", clippy.as_deref(), "rustup component add clippy");

    let src_ok = rust_src_present();
    report(
        "rust-src",
        if src_ok { Some("present") } else { None },
        "rustup component add rust-src",
    );

    let uefi = uefi_target_installed();
    report(
        "target x86_64-unknown-uefi",
        if uefi { Some("installed") } else { None },
        "rustup target add x86_64-unknown-uefi",
    );
    if !uefi {
        critical_missing.push("x86_64-unknown-uefi target");
    }

    let qemu = probe("qemu-system-x86_64", &["--version"]);
    report(
        "qemu-system-x86_64",
        qemu.as_deref(),
        "QEMU >= 8.x from qemu.org",
    );
    if qemu.is_none() {
        critical_missing.push("QEMU");
    }

    let ovmf = ovmf_firmware_path();
    report(
        "OVMF firmware",
        ovmf,
        "edk2-x86_64-code.fd (ships with QEMU)",
    );
    if ovmf.is_none() {
        critical_missing.push("OVMF");
    }

    println!();
    if critical_missing.is_empty() {
        println!("summary: all required tools present — boot testing available");
    } else {
        println!("summary: missing: {}", critical_missing.join(", "));
        println!("         `run`/`test-boot` will not work until these are installed.");
    }
    EXIT_SUCCESS
}

fn probe(command: &str, args: &[&str]) -> Option<String> {
    probe_full(command, args)
        .as_deref()
        .map(first_line)
        .map(str::to_string)
}

/// Runs a command and returns its FULL stdout on success (None otherwise).
fn probe_full(command: &str, args: &[&str]) -> Option<String> {
    let output = Command::new(command).args(args).output().ok()?;
    output
        .status
        .success()
        .then(|| String::from_utf8_lossy(&output.stdout).to_string())
}

fn probe_cargo(args: &[&str]) -> Option<String> {
    let cargo = env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());
    probe(&cargo, args)
}

fn rust_src_present() -> bool {
    let Some(sysroot) = probe("rustc", &["--print", "sysroot"]) else {
        return false;
    };
    Path::new(sysroot.trim())
        .join("lib/rustlib/src/rust")
        .is_dir()
}

fn uefi_target_installed() -> bool {
    let Some(out) = probe_full("rustup", &["target", "list", "--installed"]) else {
        // rustup missing does not prove the target is missing; report false
        // with the install hint so `info` stays actionable.
        return false;
    };
    installed_targets(&out).contains(&"x86_64-unknown-uefi")
}

/// Parses `rustup target list --installed` output into a list of triples.
fn installed_targets(output: &str) -> Vec<&str> {
    output
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .collect()
}

fn ovmf_firmware_path() -> Option<&'static str> {
    const CANDIDATES: &[&str] = &[
        "/usr/share/OVMF/OVMF_CODE_4M.fd",
        "/usr/share/OVMF/OVMF_CODE.fd",
        "/usr/share/ovmf/OVMF.fd",
        "/usr/share/qemu/edk2-x86_64-code.fd",
        "C:\\Program Files\\qemu\\share\\edk2-x86_64-code.fd",
    ];
    CANDIDATES.iter().copied().find(|p| Path::new(p).is_file())
}

fn report(name: &str, found: Option<&str>, hint: &str) {
    match found {
        Some(detail) => println!("[tool]    {name:<26} PASS   {detail}"),
        None => println!("[tool]    {name:<26} MISS   install hint: {hint}"),
    }
}

fn first_line(text: &str) -> &str {
    text.lines().next().unwrap_or("").trim()
}

fn stub_not_implemented(name: &str) -> u8 {
    eprintln!("xtask: '{name}' is not implemented yet.");
    eprintln!("       It lands in Milestone 1 together with the UEFI boot prototype.");
    eprintln!("       Exit code {EXIT_NOT_IMPLEMENTED} (not implemented).");
    EXIT_NOT_IMPLEMENTED
}

fn cargo(args: &[&str]) -> u8 {
    let cargo = env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());
    println!("+ {cargo} {}", args.join(" "));
    match Command::new(cargo).args(args).status() {
        Ok(status) if status.success() => EXIT_SUCCESS,
        Ok(_) => EXIT_FAILURE,
        Err(err) => {
            eprintln!("error: failed to spawn cargo: {err}");
            EXIT_FAILURE
        }
    }
}

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .map(Path::to_path_buf)
        .expect("xtask must live inside the workspace")
}

// ---------------------------------------------------------------------------
// Tests — argument parsing and pure helpers only; no cargo invocations.
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn args(list: &[&str]) -> Vec<String> {
        list.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn no_args_shows_help() {
        assert_eq!(parse_args(&args(&[])), Ok(Cmd::Help));
    }

    #[test]
    fn help_flags_map_to_help() {
        for flag in ["help", "-h", "--help"] {
            assert_eq!(parse_args(&args(&[flag])), Ok(Cmd::Help), "flag: {flag}");
        }
    }

    #[test]
    fn parses_build_default() {
        assert_eq!(
            parse_args(&args(&["build"])),
            Ok(Cmd::Build {
                release: false,
                target: None
            })
        );
    }

    #[test]
    fn parses_build_release_and_target() {
        assert_eq!(
            parse_args(&args(&[
                "build",
                "--release",
                "--target",
                "x86_64-unknown-uefi"
            ])),
            Ok(Cmd::Build {
                release: true,
                target: Some("x86_64-unknown-uefi".to_string())
            })
        );
    }

    #[test]
    fn build_target_requires_a_value() {
        assert!(parse_args(&args(&["build", "--target"])).is_err());
    }

    #[test]
    fn unknown_command_is_usage_error() {
        assert!(parse_args(&args(&["deploy"])).is_err());
    }

    #[test]
    fn unknown_option_is_usage_error() {
        assert!(parse_args(&args(&["test", "--release"])).is_err());
    }

    #[test]
    fn parses_fmt_check_vs_write() {
        assert_eq!(parse_args(&args(&["fmt"])), Ok(Cmd::Fmt { check: false }));
        assert_eq!(
            parse_args(&args(&["fmt", "--check"])),
            Ok(Cmd::Fmt { check: true })
        );
    }

    #[test]
    fn simple_commands_reject_options() {
        assert!(parse_args(&args(&["lint", "--force"])).is_err());
        assert!(parse_args(&args(&["info", "-v"])).is_err());
        assert!(parse_args(&args(&["clean", "extra"])).is_err());
    }

    #[test]
    fn stub_commands_parse_cleanly() {
        assert_eq!(parse_args(&args(&["run"])), Ok(Cmd::Run));
        assert_eq!(parse_args(&args(&["test-boot"])), Ok(Cmd::TestBoot));
    }

    #[test]
    fn first_line_trims_and_stops() {
        assert_eq!(
            first_line("rustc 1.99.1 (hash)\nmore\n"),
            "rustc 1.99.1 (hash)"
        );
        assert_eq!(first_line("   \n  x"), "");
    }

    #[test]
    fn installed_targets_parses_all_lines() {
        let out = "x86_64-unknown-linux-gnu\nx86_64-unknown-uefi\n\n";
        let targets = installed_targets(out);
        assert_eq!(targets.len(), 2);
        assert!(targets.contains(&"x86_64-unknown-uefi"));
        assert!(installed_targets("").is_empty());
    }
}
