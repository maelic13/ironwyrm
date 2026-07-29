use std::sync::Arc;
use std::thread;

use rarog::engine::Engine;
use rarog::engine_command::{EngineCommandQueue, EngineControl};
use rarog::infra::capitalize_first_letter;
use rarog::uci_protocol::UciProtocol;

// 9.0b: Rarog supports 64-bit targets only (user decision 2026-07-19 — the
// shipped arches are x86-64/x86-64-v3, and 64-bit is what makes the
// `u64 → usize` hash-indexing conversions in the TT/eval caches lossless; see
// `infra::index`). Fail at COMPILE time on anything else instead of shipping
// silently-wrong index math.
#[cfg(not(target_pointer_width = "64"))]
compile_error!("Rarog supports only 64-bit targets (u64 hash -> usize indexing relies on it).");

const ENGINE_THREAD_STACK_SIZE: usize = 16 * 1024 * 1024;

fn main() {
    request_fine_grained_scheduling();
    if !pext_build_is_supported() {
        eprintln!(
            "{} PEXT build requires a CPU with BMI2/PEXT support. Use the AVX2 build on this machine.",
            capitalize_first_letter(env!("CARGO_PKG_NAME"))
        );
        std::process::exit(1);
    }

    println!(
        "{} {} by {}",
        capitalize_first_letter(env!("CARGO_PKG_NAME")),
        env!("CARGO_PKG_VERSION"),
        env!("CARGO_PKG_AUTHORS").replace(':', ", ")
    );

    let commands = EngineCommandQueue::default();
    let control = Arc::new(EngineControl::default());
    let engine_commands = commands.clone();
    let engine_control = Arc::clone(&control);
    let engine_thread = thread::Builder::new()
        .name("rarog-engine".to_string())
        .stack_size(ENGINE_THREAD_STACK_SIZE)
        // Construct the Engine (which owns the large inline-array Searcher)
        // *inside* this 16 MB thread, not on the caller's stack. In debug builds
        // the default 1 MB Windows main-thread stack overflows while building the
        // Searcher (no copy elision); doing it here keeps the big frames on the
        // large stack the search already runs on. Zero search impact.
        .spawn(move || {
            let mut engine = Engine::new(engine_commands, engine_control);
            engine.start();
        })
        .expect("Engine thread failed to start.");

    UciProtocol::new(commands, control).uci_loop();
    engine_thread.join().expect("Engine thread failed.");
}

/// Ask Windows for 1 ms scheduling granularity.
///
/// Written during the 8.13(e) time-forfeit hunt, and kept with an honest
/// scope note: this did NOT fix the forfeits (measured — the ~35 ms stalls
/// are scheduler starvation under multi-thread contention, addressed by the
/// SMP time reserve in `time_manager.rs`). What it does buy: the 1 ms
/// `thread::sleep` in the ponder/infinite wait loop actually sleeps ~1 ms
/// instead of a 15.6 ms tick, so `ponderhit`/`stop` are picked up promptly,
/// and short timed waits across the engine stop being tick-quantised.
///
/// `timeBeginPeriod(1)` lasts for the lifetime of the process and Windows
/// reverts it at exit, so no paired `timeEndPeriod` is needed.
#[cfg(windows)]
fn request_fine_grained_scheduling() {
    #[link(name = "winmm")]
    unsafe extern "system" {
        fn timeBeginPeriod(u_period: u32) -> u32;
    }
    // SAFETY: plain FFI call with no pointers or state; the only effect is a
    // process-scoped timer-resolution request the OS reverts at exit.
    unsafe {
        timeBeginPeriod(1);
    }
}

#[cfg(not(windows))]
fn request_fine_grained_scheduling() {}

#[cfg(all(rarog_pext, target_arch = "x86_64"))]
fn pext_build_is_supported() -> bool {
    std::is_x86_feature_detected!("bmi2")
}

#[cfg(not(all(rarog_pext, target_arch = "x86_64")))]
fn pext_build_is_supported() -> bool {
    true
}
