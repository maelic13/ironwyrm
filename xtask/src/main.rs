use std::env;
use std::ffi::OsStr;
use std::fs;
use std::io::{self, BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::sync::mpsc;
use std::time::{Duration, Instant};

const RUSTFLAGS_SEPARATOR: &str = "\x1f";
const PGO_TRAINING_TIMEOUT: Duration = Duration::from_secs(20 * 60);

type Result<T> = std::result::Result<T, String>;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Arch {
    Base,
    Avx2,
    Pext,
    Arm64,
}

#[derive(Debug)]
struct Config {
    arch: Arch,
    /// Tune codegen for the exact host CPU instead of the arch's portable
    /// baseline. ORTHOGONAL to `arch`: it swaps `-C target-cpu=<baseline>` for
    /// `-C target-cpu=native` and changes nothing else, so `--arch base
    /// --native` is a valid, correct build for a CPU with no BMI2 or AVX2.
    ///
    /// Before 2.3.0 this was an *arch* (`--arch native`) that hardcoded the
    /// PEXT code path, which meant a non-BMI2 x86_64 host could not get a
    /// native build at all — asking for one produced a binary emitting
    /// `_pext_u64` against a `target-cpu` that does not enable the feature,
    /// i.e. an illegal instruction at runtime.
    native: bool,
    target: String,
    pgo: bool,
    bench_depth: u16,
}

fn main() {
    if let Err(err) = run() {
        eprintln!("error: {err}");
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let config = parse_args()?;
    ensure_arch_target_pair(config.arch, &config.target)?;
    ensure_rust_target(&config.target)?;

    if config.pgo {
        build_with_pgo(&config)
    } else {
        let target_dir = target_dir("release", config.arch, config.native, &config.target);
        cargo_build(&config.target, config.arch, config.native, &target_dir, &[])?;
        copy_dist_binary(
            &binary_path(&target_dir, &config.target),
            config.arch,
            config.native,
            &config.target,
            false,
        )
    }
}

fn parse_args() -> Result<Config> {
    let mut args = env::args().skip(1);
    let command = args.next().unwrap_or_else(|| "build".to_string());
    if command == "help" || command == "--help" || command == "-h" {
        print_usage();
        std::process::exit(0);
    }
    if command != "build" {
        return Err(format!(
            "unknown command `{command}`; expected `build`. Run `cargo xtask help`."
        ));
    }

    let mut arch: Option<Arch> = None;
    let mut target: Option<String> = None;
    let mut pgo = false;
    let mut native = false;
    let mut bench_depth = 13u16;

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--arch" | "-a" => {
                let value = args
                    .next()
                    .ok_or_else(|| "`--arch` requires a value".to_string())?;
                if value.eq_ignore_ascii_case("native") {
                    // Legacy spelling: `--arch native` meant "PEXT + host
                    // tuning". Kept working, but it is now expressed as two
                    // independent choices.
                    eprintln!("note: `--arch native` is deprecated; use `--arch pext --native`");
                    arch = Some(Arch::Pext);
                    native = true;
                } else {
                    arch = Some(parse_arch(&value)?);
                }
            }
            "--target" | "-t" => {
                target = Some(
                    args.next()
                        .ok_or_else(|| "`--target` requires a value".to_string())?,
                );
            }
            "--pgo" => pgo = true,
            "--native" => native = true,
            "--bench-depth" => {
                let value = args
                    .next()
                    .ok_or_else(|| "`--bench-depth` requires a value".to_string())?;
                bench_depth = value
                    .parse::<u16>()
                    .map_err(|_| format!("invalid bench depth `{value}`"))?;
            }
            "--help" | "-h" => {
                print_usage();
                std::process::exit(0);
            }
            other => return Err(format!("unknown argument `{other}`")),
        }
    }

    let arch = arch.unwrap_or_else(default_arch);
    let target = target.unwrap_or_else(|| default_target(arch));
    ensure_native_is_buildable(arch, native, &target)?;
    Ok(Config {
        arch,
        native,
        target,
        pgo,
        bench_depth,
    })
}

fn print_usage() {
    println!(
        "Usage:
  cargo xtask build [--arch base|x86-64|avx2|pext|arm64] [--native] [--target <triple>] [--pgo] [--bench-depth <n>]

`--arch` picks the ISA contract: which source path compiles (PEXT vs portable
magic bitboards) and which CPU features are required.
`--native` is INDEPENDENT of it: it swaps the portable `target-cpu` baseline
for this exact host CPU. LOCAL ONLY - such a binary is not guaranteed to run
anywhere else, and is marked `-native` in its filename.

Examples:
  cargo xtask build                              # portable x86-64
  cargo xtask build --arch avx2
  cargo xtask build --arch pext --pgo            # the shipped pext asset
  cargo xtask build --arch pext --native --pgo   # fastest build for this box
  cargo xtask build --arch base --native         # native on a pre-BMI2 CPU
  cargo xtask build --arch arm64 --target aarch64-apple-darwin"
    );
}

fn parse_arch(value: &str) -> Result<Arch> {
    match value.to_ascii_lowercase().as_str() {
        "base" | "x86-64" | "x86_64" | "x64" => Ok(Arch::Base),
        "avx2" => Ok(Arch::Avx2),
        "pext" | "bmi2" => Ok(Arch::Pext),
        "arm64" | "aarch64" => Ok(Arch::Arm64),
        _ => Err(format!(
            "unknown arch `{value}`; expected base, avx2, pext, or arm64"
        )),
    }
}

/// Refuse a combination that cannot produce a working binary on this machine.
///
/// `--arch pext` compiles `_pext_u64` and demands BMI2. Asking for it *and*
/// `--native` says "build this for the machine I am on", so if that machine has
/// no BMI2 the result would be an illegal-instruction binary. Only checked when
/// building for the host, since cross-builds legitimately target other CPUs.
fn ensure_native_is_buildable(arch: Arch, native: bool, target: &str) -> Result<()> {
    if !native || arch != Arch::Pext {
        return Ok(());
    }
    let host = host_triple().unwrap_or_default();
    if target != host {
        return Ok(());
    }
    #[cfg(target_arch = "x86_64")]
    if !std::arch::is_x86_feature_detected!("bmi2") {
        return Err(
            "`--arch pext --native` needs BMI2, which this CPU does not report.              Use `--arch avx2 --native` or `--arch base --native` for a native              build here, or drop `--native` to cross-build a portable pext asset."
                .to_string(),
        );
    }
    Ok(())
}

fn default_arch() -> Arch {
    let host = host_triple().unwrap_or_default();
    if host.starts_with("aarch64-") {
        Arch::Arm64
    } else {
        Arch::Base
    }
}

fn default_target(arch: Arch) -> String {
    let host = host_triple().unwrap_or_default();
    let os = if host.contains("windows") {
        "windows"
    } else if host.contains("apple-darwin") {
        "macos"
    } else {
        "linux"
    };

    match (arch, os) {
        (Arch::Arm64, "windows") => "aarch64-pc-windows-msvc",
        (Arch::Arm64, "macos") => "aarch64-apple-darwin",
        (Arch::Arm64, _) => "aarch64-unknown-linux-gnu",
        (_, "windows") => "x86_64-pc-windows-msvc",
        (_, "macos") => "x86_64-apple-darwin",
        (_, _) => "x86_64-unknown-linux-gnu",
    }
    .to_string()
}

fn ensure_arch_target_pair(arch: Arch, target: &str) -> Result<()> {
    match arch {
        Arch::Base | Arch::Avx2 | Arch::Pext if !target.starts_with("x86_64-") => Err(format!(
            "`--arch {}` requires an x86_64 target, got `{target}`",
            arch_arg_name(arch)
        )),
        Arch::Arm64 if !target.starts_with("aarch64-") => Err(format!(
            "`--arch arm64` requires an aarch64 target, got `{target}`"
        )),
        _ => Ok(()),
    }
}

fn arch_arg_name(arch: Arch) -> &'static str {
    match arch {
        Arch::Base => "base",
        Arch::Avx2 => "avx2",
        Arch::Pext => "pext",
        Arch::Arm64 => "arm64",
    }
}

/// Directory/filename tag for an (arch, native) pair.
///
/// Native and portable builds of the same arch are DIFFERENT binaries, so they
/// must not share a PGO profile directory or an intermediate target dir —
/// otherwise switching flavours silently reuses the other's layout. Cargo
/// fingerprints RUSTFLAGS so it would rebuild rather than emit a stale binary,
/// but keying the paths keeps the artifact tree unambiguous and stops the two
/// flavours thrashing each other's incremental state.
fn flavour_name(arch: Arch, native: bool) -> String {
    if native {
        format!("{}-native", arch_arg_name(arch))
    } else {
        arch_arg_name(arch).to_string()
    }
}

fn asset_arch_name(arch: Arch) -> &'static str {
    match arch {
        Arch::Base => "x86-64",
        Arch::Avx2 => "avx2",
        Arch::Pext => "pext",
        Arch::Arm64 => "arm64",
    }
}

/// Codegen flags for an (arch, native) pair.
///
/// `arch` selects the ISA contract — which source path is compiled
/// (`--cfg rarog_pext`) and which features are required. `native` only swaps
/// the portable `target-cpu` baseline for the host's own CPU. They are
/// independent, so every combination is meaningful: `--arch base --native`
/// tunes for this machine while emitting nothing above the x86-64 baseline,
/// and `--arch pext` stays portable across every BMI2-capable CPU.
///
/// `target-cpu=native` is LOCAL ONLY — the resulting binary is not guaranteed
/// to run on any other machine, so distributed assets never set it.
fn rustflags(arch: Arch, native: bool) -> Vec<String> {
    let cpu = |portable: &str| -> Vec<String> {
        vec![
            "-C".into(),
            if native {
                "target-cpu=native".to_string()
            } else {
                format!("target-cpu={portable}")
            },
        ]
    };
    match arch {
        Arch::Base => cpu("x86-64"),
        Arch::Avx2 => cpu("x86-64-v3"),
        Arch::Pext => {
            let mut flags = vec!["--cfg".into(), "rarog_pext".into()];
            flags.extend(cpu("x86-64-v3"));
            // Required even under `native`: the PEXT source path calls
            // `_pext_u64`, so the feature must be on regardless of how the
            // baseline was chosen.
            flags.extend(["-C".into(), "target-feature=+bmi2".into()]);
            flags
        }
        Arch::Arm64 => cpu("generic"),
    }
}

fn build_with_pgo(config: &Config) -> Result<()> {
    let host = host_triple()?;
    if config.target != host {
        return Err(format!(
            "PGO training must run the instrumented binary locally, so target `{}` must match host `{host}`",
            config.target
        ));
    }

    let llvm_profdata = ensure_llvm_profdata()?;
    let pgo_dir = PathBuf::from("target")
        .join("pgo")
        .join(sanitize(&config.target))
        .join(flavour_name(config.arch, config.native));
    let raw_dir = pgo_dir.join("raw");
    if raw_dir.exists() {
        fs::remove_dir_all(&raw_dir)
            .map_err(|err| format!("failed to remove `{}`: {err}", raw_dir.display()))?;
    }
    fs::create_dir_all(&raw_dir)
        .map_err(|err| format!("failed to create `{}`: {err}", raw_dir.display()))?;

    let mut generate_flags = vec![
        "-C".to_string(),
        format!("profile-generate={}", raw_dir.display()),
    ];
    generate_flags.extend(rustflags(config.arch, config.native));

    let gen_target_dir = target_dir("pgo-gen", config.arch, config.native, &config.target);
    cargo_build(
        &config.target,
        config.arch,
        config.native,
        &gen_target_dir,
        &generate_flags,
    )?;

    let instrumented = binary_path(&gen_target_dir, &config.target);
    run_training_bench(&instrumented, &raw_dir, config.bench_depth)?;

    let profdata = pgo_dir.join("rarog.profdata");
    merge_profiles(&llvm_profdata, &raw_dir, &profdata, &config.target)?;

    let mut use_flags = vec![
        "-C".to_string(),
        format!("profile-use={}", profdata.display()),
    ];
    use_flags.extend(rustflags(config.arch, config.native));

    let use_target_dir = target_dir("pgo-use", config.arch, config.native, &config.target);
    cargo_build(
        &config.target,
        config.arch,
        config.native,
        &use_target_dir,
        &use_flags,
    )?;
    copy_dist_binary(
        &binary_path(&use_target_dir, &config.target),
        config.arch,
        config.native,
        &config.target,
        true,
    )
}

fn cargo_build(
    target: &str,
    arch: Arch,
    native: bool,
    target_dir: &Path,
    override_flags: &[String],
) -> Result<()> {
    let flags = if override_flags.is_empty() {
        rustflags(arch, native)
    } else {
        override_flags.to_vec()
    };

    println_flush(format_args!(
        "Building Rarog {} for {}{}",
        asset_arch_name(arch),
        target,
        if override_flags.is_empty() {
            ""
        } else {
            " with PGO flags"
        }
    ));

    let status = Command::new("cargo")
        .arg("build")
        .arg("--release")
        .arg("--target")
        .arg(target)
        .arg("--target-dir")
        .arg(target_dir)
        .env("CARGO_ENCODED_RUSTFLAGS", flags.join(RUSTFLAGS_SEPARATOR))
        .env_remove("RUSTFLAGS")
        .status()
        .map_err(|err| format!("failed to run cargo build: {err}"))?;

    if status.success() {
        Ok(())
    } else {
        Err(format!("cargo build failed with status {status}"))
    }
}

fn run_training_bench(binary: &Path, raw_dir: &Path, depth: u16) -> Result<()> {
    println_flush(format_args!(
        "Training PGO profile with internal bench depth {depth}"
    ));
    let profile_pattern = raw_dir.join("rarog-%p-%m.profraw");
    let mut child = Command::new(binary)
        .env("LLVM_PROFILE_FILE", &profile_pattern)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .spawn()
        .map_err(|err| format!("failed to run `{}`: {err}", binary.display()))?;

    {
        let stdin = child
            .stdin
            .as_mut()
            .ok_or_else(|| "failed to open engine stdin".to_string())?;
        writeln!(stdin, "bench {depth}").map_err(|err| format!("failed to start bench: {err}"))?;
        stdin
            .flush()
            .map_err(|err| format!("failed to flush stdin: {err}"))?;
    }

    let stdout = child
        .stdout
        .take()
        .ok_or_else(|| "failed to open engine stdout".to_string())?;
    let (line_tx, line_rx) = mpsc::channel();
    std::thread::spawn(move || {
        let reader = BufReader::new(stdout);
        for line in reader.lines() {
            if line_tx.send(line).is_err() {
                break;
            }
        }
    });

    let deadline = Instant::now() + PGO_TRAINING_TIMEOUT;
    let mut saw_summary = false;
    loop {
        let remaining = deadline.saturating_duration_since(Instant::now());
        if remaining.is_zero() {
            kill_child(&mut child);
            return Err(format!(
                "training bench timed out after {} seconds",
                PGO_TRAINING_TIMEOUT.as_secs()
            ));
        }

        match line_rx.recv_timeout(remaining.min(Duration::from_millis(250))) {
            Ok(Ok(line)) => {
                println_flush(format_args!("{line}"));
                // Mirror Basilisk's PGO guard: a corrupt/illegal bench position
                // must never silently train the profile. `bench` emits
                // "failed to parse" for any position `from_fen` rejects (bad pawn
                // count, back-rank pawns, etc.) and then aborts without a summary
                // — fail fast here rather than hanging until the timeout.
                const ILLEGAL_MARKERS: [&str; 3] = [
                    "failed to parse",
                    "more than 8 pawns",
                    "not legal on the first or eighth rank",
                ];
                if ILLEGAL_MARKERS.iter().any(|marker| line.contains(marker)) {
                    kill_child(&mut child);
                    return Err(format!(
                        "PGO training hit an illegal bench position: {line}"
                    ));
                }
                if line.starts_with("Nodes/second") {
                    saw_summary = true;
                    break;
                }
            }
            Ok(Err(err)) => {
                kill_child(&mut child);
                return Err(format!("failed reading engine output: {err}"));
            }
            Err(mpsc::RecvTimeoutError::Timeout) => {
                if let Some(status) = child
                    .try_wait()
                    .map_err(|err| format!("failed checking engine status: {err}"))?
                {
                    if status.success() {
                        break;
                    }
                    return Err(format!("training bench exited with status {status}"));
                }
            }
            Err(mpsc::RecvTimeoutError::Disconnected) => break,
        }
    }

    if let Some(stdin) = child.stdin.as_mut() {
        let _ = writeln!(stdin, "quit");
    }

    let status = wait_child_with_timeout(&mut child, Duration::from_secs(10))?;
    if !status.success() {
        return Err(format!("training bench exited with status {status}"));
    }
    if !saw_summary {
        return Err("training bench did not produce a bench summary".to_string());
    }
    Ok(())
}

fn wait_child_with_timeout(
    child: &mut Child,
    timeout: Duration,
) -> Result<std::process::ExitStatus> {
    let deadline = Instant::now() + timeout;
    loop {
        if let Some(status) = child
            .try_wait()
            .map_err(|err| format!("failed checking engine status: {err}"))?
        {
            return Ok(status);
        }
        if Instant::now() >= deadline {
            kill_child(child);
            return child
                .wait()
                .map_err(|err| format!("failed waiting for killed engine: {err}"));
        }
        std::thread::sleep(Duration::from_millis(25));
    }
}

fn kill_child(child: &mut Child) {
    let _ = child.kill();
    let _ = child.wait();
}

fn merge_profiles(
    llvm_profdata: &Path,
    raw_dir: &Path,
    profdata: &Path,
    target: &str,
) -> Result<()> {
    let mut inputs = Vec::new();
    for entry in fs::read_dir(raw_dir)
        .map_err(|err| format!("failed to read `{}`: {err}", raw_dir.display()))?
    {
        let path = entry
            .map_err(|err| format!("failed to read profile entry: {err}"))?
            .path();
        if path.extension() == Some(OsStr::new("profraw")) {
            inputs.push(path);
        }
    }
    if inputs.is_empty() {
        return Err(format!(
            "no .profraw files found in `{}`",
            raw_dir.display()
        ));
    }

    println_flush(format_args!("Merging {} profile file(s)", inputs.len()));
    let status = Command::new(llvm_profdata)
        .arg("merge")
        .arg("-output")
        .arg(profdata)
        .args(&inputs)
        .status()
        .map_err(|err| format!("failed to run `{}`: {err}", llvm_profdata.display()))?;

    if status.success() {
        return Ok(());
    }
    let mut message = format!("llvm-profdata merge failed with status {status}");
    message.push_str(&pgo_merge_hint(target));
    Err(message)
}

/// Known-broken PGO targets get an actionable hint instead of a bare exit code.
///
/// On `aarch64-pc-windows-msvc` the instrumented binary builds and runs
/// correctly — the training bench produces the right node count — but the
/// profiling runtime writes a `.profraw` whose symbol-name table is empty, so
/// `llvm-profdata merge` rejects every input with "malformed instrumentation
/// profile data: symbol name is empty" and then "no profile can be merged".
/// Nothing in this repo can work around it. Observed with rustc 1.97.1 /
/// LLVM 22.1.6; re-test on every toolchain bump and drop this hint once the
/// merge succeeds.
fn pgo_merge_hint(target: &str) -> String {
    if target.starts_with("aarch64-pc-windows") {
        format!(
            "\n\nnote: PGO is a known toolchain limitation on `{target}`. The \
             profiling runtime emits .profraw files with an empty symbol-name \
             table, so no profile can ever be merged. Build this target \
             without `--pgo`; the binary is correct, only slower."
        )
    } else {
        String::new()
    }
}

fn ensure_rust_target(target: &str) -> Result<()> {
    if find_on_path("rustup").is_none() {
        eprintln!(
            "rustup not found; if `{target}` is not installed, run `rustup target add {target}`."
        );
        return Ok(());
    }

    let status = Command::new("rustup")
        .arg("target")
        .arg("add")
        .arg(target)
        .status()
        .map_err(|err| format!("failed to run rustup target add: {err}"))?;
    if status.success() {
        Ok(())
    } else {
        Err(format!(
            "failed to install target `{target}`; run `rustup target add {target}` manually"
        ))
    }
}

fn ensure_llvm_profdata() -> Result<PathBuf> {
    if let Some(path) = find_llvm_profdata() {
        return Ok(path);
    }

    if find_on_path("rustup").is_none() {
        return Err(
            "llvm-profdata was not found. Install it with `rustup component add llvm-tools-preview` or add LLVM's bin directory to PATH."
                .to_string(),
        );
    }

    println_flush(format_args!(
        "Installing llvm-tools-preview for PGO support"
    ));
    let status = Command::new("rustup")
        .arg("component")
        .arg("add")
        .arg("llvm-tools-preview")
        .status()
        .map_err(|err| format!("failed to run rustup component add: {err}"))?;
    if !status.success() {
        return Err(
            "failed to install llvm-tools-preview; run `rustup component add llvm-tools-preview` manually"
                .to_string(),
        );
    }

    find_llvm_profdata().ok_or_else(|| {
        "llvm-profdata was still not found after installing llvm-tools-preview".to_string()
    })
}

fn find_llvm_profdata() -> Option<PathBuf> {
    find_on_path("llvm-profdata").or_else(find_rustup_llvm_profdata)
}

fn find_rustup_llvm_profdata() -> Option<PathBuf> {
    let sysroot = command_output("rustc", &["--print", "sysroot"]).ok()?;
    let host = host_triple().ok()?;
    let candidate = PathBuf::from(sysroot)
        .join("lib")
        .join("rustlib")
        .join(host)
        .join("bin")
        .join(tool_name("llvm-profdata"));
    candidate.exists().then_some(candidate)
}

fn find_on_path(program: &str) -> Option<PathBuf> {
    let paths = env::var_os("PATH")?;
    for dir in env::split_paths(&paths) {
        let candidate = dir.join(program);
        if candidate.is_file() {
            return Some(candidate);
        }
        if cfg!(windows) && !program.ends_with(".exe") {
            let candidate = dir.join(format!("{program}.exe"));
            if candidate.is_file() {
                return Some(candidate);
            }
        }
    }
    None
}

fn tool_name(name: &str) -> String {
    if cfg!(windows) {
        format!("{name}.exe")
    } else {
        name.to_string()
    }
}

fn host_triple() -> Result<String> {
    let output = command_output("rustc", &["-vV"])?;
    output
        .lines()
        .find_map(|line| line.strip_prefix("host: ").map(str::to_string))
        .ok_or_else(|| "failed to parse rustc host triple".to_string())
}

fn command_output(program: &str, args: &[&str]) -> Result<String> {
    let output = Command::new(program)
        .args(args)
        .output()
        .map_err(|err| format!("failed to run `{program}`: {err}"))?;
    if !output.status.success() {
        return Err(format!("`{program}` failed with status {}", output.status));
    }
    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

fn target_dir(kind: &str, arch: Arch, native: bool, target: &str) -> PathBuf {
    PathBuf::from("target").join("xtask").join(format!(
        "{kind}-{}-{}",
        sanitize(target),
        flavour_name(arch, native)
    ))
}

fn binary_path(target_dir: &Path, target: &str) -> PathBuf {
    target_dir
        .join(target)
        .join("release")
        .join(format!("rarog{}", exe_suffix(target)))
}

fn copy_dist_binary(
    binary: &Path,
    arch: Arch,
    native: bool,
    target: &str,
    pgo: bool,
) -> Result<()> {
    if !binary.exists() {
        return Err(format!(
            "expected binary `{}` does not exist",
            binary.display()
        ));
    }
    let dist = PathBuf::from("target").join("dist");
    fs::create_dir_all(&dist)
        .map_err(|err| format!("failed to create `{}`: {err}", dist.display()))?;
    let asset = dist.join(asset_name(arch, native, target, pgo)?);
    fs::copy(binary, &asset).map_err(|err| {
        format!(
            "failed to copy `{}` to `{}`: {err}",
            binary.display(),
            asset.display()
        )
    })?;
    println_flush(format_args!("Built {}", asset.display()));
    Ok(())
}

fn println_flush(args: std::fmt::Arguments<'_>) {
    println!("{args}");
    io::stdout().flush().expect("stdout flush failed");
}

fn asset_name(arch: Arch, native: bool, target: &str, pgo: bool) -> Result<String> {
    let pgo_suffix = if pgo { "-pgo" } else { "" };
    Ok(format!(
        "rarog-v{}-{}-{}{}{}{}",
        package_version()?,
        os_name(target),
        asset_arch_name(arch),
        // A native build is host-specific and must never be mistaken for a
        // distributable asset, so it is marked in the filename.
        if native { "-native" } else { "" },
        pgo_suffix,
        exe_suffix(target)
    ))
}

fn package_version() -> Result<String> {
    let manifest = fs::read_to_string("Cargo.toml")
        .map_err(|err| format!("failed to read Cargo.toml: {err}"))?;
    for line in manifest.lines() {
        let line = line.trim();
        if let Some(version) = line.strip_prefix("version = ") {
            return Ok(version.trim_matches('"').to_string());
        }
    }
    Err("failed to find package version in Cargo.toml".to_string())
}

fn os_name(target: &str) -> &'static str {
    if target.contains("windows") {
        "windows"
    } else if target.contains("apple-darwin") {
        "macos"
    } else {
        "linux"
    }
}

fn exe_suffix(target: &str) -> &'static str {
    if target.contains("windows") {
        ".exe"
    } else {
        ""
    }
}

fn sanitize(value: &str) -> String {
    value
        .chars()
        .map(|ch| if ch.is_ascii_alphanumeric() { ch } else { '-' })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn flags(arch: Arch, native: bool) -> String {
        rustflags(arch, native).join(" ")
    }

    /// The property that motivated the 2.3.0 rework: `--arch` and `--native`
    /// are INDEPENDENT. Before it, `native` was an arch that hardcoded the PEXT
    /// path, so a pre-BMI2 x86_64 host could not get a native build at all —
    /// it got `_pext_u64` against a `target-cpu` that did not enable BMI2.
    #[test]
    fn native_is_orthogonal_to_arch() {
        // Non-PEXT archs must NEVER pull in the PEXT path or BMI2, native or not.
        for arch in [Arch::Base, Arch::Avx2, Arch::Arm64] {
            for native in [false, true] {
                let f = flags(arch, native);
                assert!(
                    !f.contains("rarog_pext"),
                    "{arch:?} native={native} must not enable the PEXT source path: {f}"
                );
                assert!(
                    !f.contains("bmi2"),
                    "{arch:?} native={native} must not require BMI2: {f}"
                );
            }
        }
        // ...and every arch must honour `--native` by targeting the host CPU.
        for arch in [Arch::Base, Arch::Avx2, Arch::Pext, Arch::Arm64] {
            assert!(
                flags(arch, true).contains("target-cpu=native"),
                "{arch:?} --native must target the host CPU"
            );
            assert!(
                !flags(arch, false).contains("target-cpu=native"),
                "{arch:?} without --native must stay portable"
            );
        }
    }

    /// Exact portable baselines — these are the contract the shipped assets are
    /// built against, so a change here changes what users run.
    #[test]
    fn portable_baselines_are_pinned() {
        assert_eq!(flags(Arch::Base, false), "-C target-cpu=x86-64");
        assert_eq!(flags(Arch::Avx2, false), "-C target-cpu=x86-64-v3");
        assert_eq!(
            flags(Arch::Pext, false),
            "--cfg rarog_pext -C target-cpu=x86-64-v3 -C target-feature=+bmi2"
        );
        assert_eq!(flags(Arch::Arm64, false), "-C target-cpu=generic");
    }

    /// PEXT compiles `_pext_u64`, so BMI2 must be required even under `--native`
    /// — `target-cpu=native` alone would not guarantee the feature is on.
    #[test]
    fn pext_requires_bmi2_even_when_native() {
        let f = flags(Arch::Pext, true);
        assert!(f.contains("--cfg rarog_pext"), "{f}");
        assert!(f.contains("target-feature=+bmi2"), "{f}");
        assert!(f.contains("target-cpu=native"), "{f}");
    }

    /// A bare `llvm-profdata` exit code is unactionable on the one target
    /// where PGO cannot work at all, so that target — and only that target —
    /// must name the limitation in the error.
    #[test]
    fn pgo_merge_hint_fires_only_for_windows_arm64() {
        let hint = pgo_merge_hint("aarch64-pc-windows-msvc");
        assert!(
            hint.contains("known toolchain limitation") && hint.contains("--pgo"),
            "windows-arm64 merge failure must explain itself: {hint}"
        );
        for target in [
            "aarch64-apple-darwin",
            "aarch64-unknown-linux-gnu",
            "x86_64-pc-windows-msvc",
            "x86_64-unknown-linux-gnu",
        ] {
            assert!(
                pgo_merge_hint(target).is_empty(),
                "{target} has working PGO and must not claim otherwise"
            );
        }
    }

    /// PGO is a third independent axis: it PREPENDS its own flag and must leave
    /// the arch/native flags untouched, in both phases.
    #[test]
    fn pgo_flags_compose_with_every_flavour() {
        for arch in [Arch::Base, Arch::Avx2, Arch::Pext, Arch::Arm64] {
            for native in [false, true] {
                for phase in ["profile-generate=/tmp/raw", "profile-use=/tmp/x.profdata"] {
                    let mut composed = vec!["-C".to_string(), phase.to_string()];
                    composed.extend(rustflags(arch, native));
                    let joined = composed.join(" ");
                    assert!(joined.contains(phase), "{joined}");
                    for flag in rustflags(arch, native) {
                        assert!(
                            joined.contains(&flag),
                            "PGO composition dropped `{flag}` for {arch:?} native={native}"
                        );
                    }
                }
            }
        }
    }

    /// A host-tuned binary must never be mistaken for a distributable asset,
    /// and native/portable builds must not share PGO or intermediate dirs.
    #[test]
    fn native_builds_are_tagged_everywhere() {
        let portable = asset_name(Arch::Pext, false, "x86_64-pc-windows-msvc", true).unwrap();
        let native = asset_name(Arch::Pext, true, "x86_64-pc-windows-msvc", true).unwrap();
        assert!(!portable.contains("native"), "{portable}");
        assert!(native.contains("-native"), "{native}");
        assert!(portable.ends_with("-pgo.exe") && native.ends_with("-pgo.exe"));
        assert_ne!(portable, native);

        assert_eq!(flavour_name(Arch::Pext, false), "pext");
        assert_eq!(flavour_name(Arch::Pext, true), "pext-native");
        assert_ne!(
            target_dir("pgo-gen", Arch::Pext, false, "x86_64-pc-windows-msvc"),
            target_dir("pgo-gen", Arch::Pext, true, "x86_64-pc-windows-msvc"),
            "native and portable PGO builds must not share an intermediate dir"
        );
    }

    /// `native` is no longer an arch; it is a flag. `parse_args` still accepts
    /// the old spelling, but `parse_arch` itself must not.
    #[test]
    fn native_is_not_an_arch() {
        assert!(parse_arch("native").is_err());
        assert_eq!(parse_arch("pext").unwrap(), Arch::Pext);
        assert_eq!(parse_arch("bmi2").unwrap(), Arch::Pext);
        assert_eq!(parse_arch("x86-64").unwrap(), Arch::Base);
        assert_eq!(parse_arch("aarch64").unwrap(), Arch::Arm64);
    }

    #[test]
    fn arch_and_target_must_agree() {
        assert!(ensure_arch_target_pair(Arch::Pext, "x86_64-pc-windows-msvc").is_ok());
        assert!(ensure_arch_target_pair(Arch::Pext, "aarch64-apple-darwin").is_err());
        assert!(ensure_arch_target_pair(Arch::Arm64, "aarch64-apple-darwin").is_ok());
        assert!(ensure_arch_target_pair(Arch::Arm64, "x86_64-unknown-linux-gnu").is_err());
    }

    /// The BMI2 pre-flight only applies to a native PEXT build FOR THIS HOST;
    /// every other combination must pass regardless of what CPU runs the tests.
    #[test]
    fn bmi2_guard_only_fires_for_native_pext_on_host() {
        assert!(ensure_native_is_buildable(Arch::Pext, false, "x86_64-pc-windows-msvc").is_ok());
        assert!(ensure_native_is_buildable(Arch::Base, true, "x86_64-pc-windows-msvc").is_ok());
        assert!(ensure_native_is_buildable(Arch::Avx2, true, "x86_64-pc-windows-msvc").is_ok());
        // Cross-build: the host's features say nothing about the target's.
        assert!(ensure_native_is_buildable(Arch::Pext, true, "some-other-triple").is_ok());
    }
}
