# Rarog development workflow guide

This is the operational handover for maintainers and coding agents. The roadmap
is in `PLAN.md`; evidence and failed attempts are in `EXPERIMENTS.md`.

## Current checkpoint

| Item | Value |
|---|---|
| Version being prepared | **2.3.2** |
| Accepted fingerprint | **6,519,711 nodes / EBF 2.449** at `bench 13`, 1T |
| Active experiment | None |
| Current action | Finish local release verification, then hosted CI/release matrix |
| Next development work | PLAN Phase 5.0, frozen NNUE measurement corpus |

Do not run `./tools/spsa.ps1 -ConfigGroup phase4 -LaunchOnly`. The Phase-4
configuration was canceled and removed before any games. Do not resume the
interrupted mate-clamp SPRT; the clamp is retained as a correctness decision.

## 2.3.2 release workflow

From a clean `development` worktree:

```powershell
cargo fmt --check
cargo test --workspace --all-targets
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo build --release
"bench" | ./target/release/rarog.exe
cargo xtask build --arch pext --pgo
cargo xtask verify-isa --arch pext
```

Expected benchmark: `Nodes searched : 6519711`, geomean EBF `2.449`.

Also build with `--features tune`, issue `uci`, and verify two properties:

- none of the ten removed options is advertised;
- later-owned inert options are still advertised at accepted defaults.

The hosted build workflow is the final production check because this machine
cannot create all Linux/macOS/Windows and x86/ARM assets. Each cell must verify
the UCI handshake, benchmark fingerprint and executable ISA contract.

Commit the release-prep changes locally. Do not tag, push or publish unless the
maintainer explicitly asks.

## What was cleaned up

Ten abandoned parameters were removed and their accepted defaults were placed
directly at the call sites. This intentionally reduces the tune surface without
changing play. The root-gap observation remains in diagnostics, but cannot
enter root confidence because null-window rival scores made it degenerate.

Future-owned experiments remain inert:

| Later owner | Examples | Why skipped now |
|---|---|---|
| Post-NNUE search fit | aspiration, TT provenance/NMP/singular alternatives, prospective selectivity, correction weighting, root confidence | NNUE changes score/residual/margin/time-cost distributions; an HCE fit would target a soon-obsolete surface |
| Multi-thread scaling | pooled root instability, helper iteration skipping | They have no useful 1T population and need representative topology |
| Runtime dispatch | universal CPU dispatch | It is product architecture, not a search knob |

At the owning phase, each item must pass and activate or be removed. “Retained”
is not permission for indefinite dormant code.

## SPSA go/no-go procedure

The generic harness is intentionally retained, but it is not the next task.
Before any future tune:

1. Name the strength-bearing mechanism and show local evidence that its
   consumers are misfit.
2. Estimate plausible Elo and opportunity cost before optimizing schedule
   details. Cancel if the plausible gain is inside the gate's dead zone.
3. Gate categorical switches separately and freeze the winner in both arms.
4. Select continuous coordinates from activation and interaction evidence.
   Do not target 24 merely because an old plan said 24.
5. Choose horizon from gradient quality, integer resolution and compute budget.
   5,000 is a prior calibration, not a universal answer.
6. Run `./tools/audit_spsa_coverage.ps1` and register surface, fixed values,
   iterations, games, gain and estimator before launch.
7. Complete the final theta without post-hoc checkpoint selection; bake into a
   fresh clean PGO binary and run paired SPRT, then LTC/4T where appropriate.

The generic stop/resume implementation is sound: state updates are
transactional, logs append, state saves every ten iterations and the schedule
is restored from state. Those properties preserve an approved tune; they do
not make a low-value tune worth running.

## Opening book

SPSA and default SPRT both use
`tools/books/UHO_Lichess_4852_v1.epd`, paired/reversed, at `3+0.03`. That is the
correct default because the optimizer and confirmation gate see aligned opening
and clock distributions. Use a second book or LTC as an additional robustness
check for a mechanism suspected to be condition-sensitive; do not create an
unnecessary tuning/confirmation mismatch.

## CPU compatibility design

There is deliberately no startup CPU guard inside specialized assets. When the
compiler is told that BMI2/AVX2/FMA are mandatory, ordinary feature-detection
macros fold those checks to true and the guard is removed. A working in-process
guard would require baseline-compiled CPUID code to execute before specialized
code, adding a separate dispatch boundary.

The current design is simple and close to the specialized-binary model: users
choose `x86-64`, `avx2`, `pext` or `arm64`, README states exact requirements,
and release tooling disassembles each asset to enforce the promise. If a single
universal binary becomes a product goal, Phase 8.1 may add a Stockfish-style
baseline dispatcher which selects specialized kernels. That is a dispatcher,
not a friendly check bolted into a binary already compiled for the newer ISA.

## Experiment discipline

- Begin from a clean revision and record both binary hashes.
- Register hypothesis, interactions, gate, stop rule and budget before games.
- Keep compilation, profiling and unrelated load off the match host.
- Treat tune/non-PGO results as diagnostics unless the experiment says
  otherwise; final-PGO games decide promotion.
- Do not turn node reduction into Elo. Use diagnostics to explain a game result.
- Record rejected and neutral outcomes in `EXPERIMENTS.md`; never silently
  rewrite them into a later success story.
- A correctness exception must name the invariant, tests and incomplete
  strength evidence honestly.

## NNUE handover

The next code phase is runway, not network training. First freeze the corpus and
contracts, then land reversible state/dirty pieces and accumulator scaffolding
while the HCE benchmark stays exact. Only after the runway gate passes should an
NNUE integration branch and large data generation begin.

Once a baseline NNUE is accepted, reassess score scale and only gross safety
margins. The one broad post-NNUE SPSA belongs after network architecture and
scale are frozen. That tune must decide the retained inert search features and
remove losers, followed by final-PGO STC/LTC/4T confirmation.

## Documentation ownership

| File | Audience / purpose |
|---|---|
| `README.md` | Users: install, CPU choice, UCI and build basics |
| `CHANGELOG.md` | Users: visible release deltas and measured claims |
| `RELEASE_NOTES_2.3.2.md` | Copy-ready GitHub release text |
| `PLAN.md` | Maintainers: current state, ownership and ordered roadmap |
| `GUIDE.md` | Maintainers/agents: commands and operating rules |
| `EXPERIMENTS.md` | Durable evidence, failures, retry triggers and artifacts |
| `tools/spsa_configs/README.md` | Tuning-specific mechanics and lessons |

When facts disagree, source/defaults and reproducible artifacts outrank prose;
fix the prose in the same change.
