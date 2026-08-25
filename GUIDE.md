# Rarog development guide

**Current state and ordered steps only.** Design and rationale live in
`PLAN.md`; durable results live in `EXPERIMENTS.md`.

## Current checkpoint

| Item | Value |
|---|---|
| Released baseline | **2.3.2** at `f931722` on `master` |
| Accepted search head | RAR-S70, **6,977,070 nodes / EBF 2.466** |
| Integration branch | `dev`, with failed SearchCore rewrite reverted by `c5e451d` |
| Frozen oracle | `hybrid` at `75d0d43`; never merge it into Rarog |
| Measured search deficit | **355.26 +/- 27.03 Elo at equal nodes**; **250.77 +/- 13.12** at equal time; speed contribution **104.5 Elo** |
| Accepted Phase-4 gains | ProbCut filter **+15.56 +/- 10.02 Elo**; root LMR relief **+2.33 +/- 1.85 Elo** |
| Active experiment | None |
| Current step | **4.7.1 — qsearch/TT authority observation and baseline** |
| HCE | Frozen through 4.8; maturity evidence/structure 4.9–4.10, required Texel consolidation 4.11, conditional SPSA 4.12–4.13 |
| Next release | Conditional 2.4.0 at 4.15; then NNUE |

## Phase 4 status

**Evidence foundation — closed**

- [x] **4.0** Evidence, baseline and oracle freeze — RAR-M12
- [x] **4.1** Instrumented oracle — `hybrid-diag` `de568b3`
- [x] **4.2** Differential observation harness — RAR-S55
- [x] **4.3** Mechanism map and order freeze
- [x] **4.4** Matched ablation plus fixed-node correction
- [x] **4.5** LMR contract study — no accepted interior gain; RAR-S70 root gain retained

**Failed continuation — closed**

- [x] **4.6** Shallow-selectivity/rewrite continuation — no accepted gain
    - [x] **4.6.1** Quiet SEE screen: gap closure **+3.38 +/- 27.08 Elo**, stopped null
    - [x] **4.6.2** SearchCore: **-9.76 +/- 17.70 Elo** over 712 complete games, stopped before boundary and reverted
    - [x] **4.6.3** Broad selectivity SPSA and another rewrite declined; entry evidence absent

**Search closeout**

- [ ] **4.7** Qsearch, TT and evaluation authority
    - [ ] **4.7.1 NEXT** Rebuild/reproduce RAR-S70; fixed-node/time baseline; same-unit producer/consumer counters; live-wire proof; write `analysis/phase4_qsearch_tt_authority.md`
    - [ ] **4.7.2** Build one dependency-complete authority candidate only if 4.7.1 isolates a unique signal
    - [ ] **4.7.3** Fit only a justified live search surface; register `[0,3]` gate, accept or revert
- [ ] **4.8** Clean retained alternatives, remeasure search, freeze one HCE baseline

**HCE maturity and fitting**

- [ ] **4.9** Freeze residual corpus and finish current-source Stockfish maturity map
- [ ] **4.10** At most two evidence-selected structural HCE clusters, each locally Texel-fitted and gated
- [ ] **4.11** Required anchored whole-HCE Texel consolidation and PGO SPRT
- [ ] **4.12** Optional nonlinear/global HCE SPSA; skip unless activation, interaction and curvature justify it
- [ ] **4.13** Post-HCE search-margin audit; optional targeted search SPSA only if the fitted HCE displaced a live optimum
- [ ] **4.14** Final HCE/search checkpoint, attribution and maturity closure

**Release**

- [ ] **4.15** STC/LTC/4T, NPS, portability, ISA and release gate

Current HCE maturity analysis:
`analysis/hce_maturity_2026-08-25.md`. The older
`analysis/hce_analysis.md` is historical; its four concrete activation defects
are already fixed and are not current work.

## What you run next

Step 4.7.1 starts with a clean exact-feature rebuild. Do not measure the
all-feature/`texel` binary.

```powershell
cargo fmt --check
cargo test --workspace --all-targets
cargo test --workspace --all-targets --release
cargo clippy --workspace --all-targets --all-features -- -D warnings

cargo build --release
@("bench 13", "quit") | .\target\release\rarog.exe
```

Expected benchmark: **6,977,070 nodes / EBF 2.466**. Stop if it differs.

Use the repository aggregators for counters and complete-pair results:

```powershell
python tools/diag/bench_counters.py --exe target/release/rarog.exe --depth 13 --stride 1 --filter q_
python tools/diag/bench_counters.py --exe target/release/rarog.exe --depth 13 --stride 1 --filter tt_
pwsh -File tools/pgn_result.ps1 -Pgn <match.pgn> -Engine <candidate-name>
```

The 4.7.1 differential run requires revision-matched diagnostic Rarog and
oracle assets. Record their hashes before running:

```powershell
python tools/diag/phase4_differential.py --rarog <rarog-diag.exe> --oracle <hybrid-diag.exe> --depth 8
```

No strength game is authorized by this documentation change. Register a 4.7
candidate in `EXPERIMENTS.md` only after the observation artifact names a
unique producer/consumer defect.

## Documentation ownership

| File | Purpose |
|---|---|
| `GUIDE.md` | Current status, ordered steps and commands |
| `PLAN.md` | Rationale, gates and full roadmap |
| `EXPERIMENTS.md` | Durable evidence, failures, retry triggers and recipes |
| `analysis/hce_maturity_2026-08-25.md` | Current HCE/Stockfish maturity comparison and fitting policy |
| `analysis/ablation_results.md` | Search-deficit measurements and corrected interpretation |
| `PROCESS.md` | Recurring build, Texel, SPSA and release procedures |
| `TRACKER.md` | History only; never a source of the next step |

`GUIDE.md` and `PLAN.md` must change in the same commit. Source, defaults and
reproducible artifacts outrank prose whenever documents disagree.
