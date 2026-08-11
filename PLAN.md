# Rarog development plan

Updated 2026-08-11. This is the current roadmap; detailed historical evidence
lives in `EXPERIMENTS.md`.

## 1. Current state

| Item | State |
|---|---|
| Branch | `development` |
| Release preparation | **2.3.2** |
| Released baseline | `v2.3.1` at `a5fd288` |
| Search fingerprint | `bench 13` = **6,519,711 nodes**, EBF **2.449** |
| Active game jobs | None; do not resume the mate-clamp run or launch Phase-4 SPSA |
| Current phase | Phase 4 closed by consolidation; release 2.3.2, then begin Phase 5 |
| Next material-strength program | NNUE runway and baseline NNUE |

The planned 2.4.0 pre-NNUE release was withdrawn. The work produced valuable
infrastructure, measurements, accepted smaller gains and correctness/platform
repairs, but it did not establish the large cumulative strength step a minor
version should represent. Version 2.3.2 accurately describes this release.

The old partial rating observation—an unfinished development binary roughly
+11 pool Elo over 2.3.1 and 39 below Basilisk 1.9.3—remains diagnostic only. It
was neither a release gate nor a forecast. The target-engine ladder moves to the
post-NNUE frontier gate, where a large gain is plausible enough to measure.

## 2. Phase 4 disposition

Phase 4 is complete. It must not be reopened through the canceled SPSA command.

### Accepted and shipping in 2.3.2

| Work | Evidence / policy |
|---|---|
| Broad selectivity fit | Accepted at +15.33 ± 7.34 nElo |
| Zero-reduction LMR floor | Accepted at +9.13 ± 5.45 nElo |
| Anchored Texel refresh | Accepted at +11.56 ± 5.19 Elo; HCE now frozen |
| NMP mate-score clamp | Permanent correctness repair; no strength claim |
| Typed TT evidence and provenance | Retained infrastructure, behavior-neutral at defaults |
| Root abort/fallback coverage | Retained correctness infrastructure |
| AArch64 TT prefetch | Accepted at +1.42% median NPS on M4, 12/12 paired wins |
| Executable ISA contract | Retained; caught accidental baseline POPCNT and enforces ARM prefetch |

The three strength results were sequential and used different estimators. They
are not additive and are not a promise of a particular release Elo gain.

### Removed in 2.3.2 cleanup

These alternatives had no remaining evidence owner. Their accepted defaults
are hardwired, so removal changes neither current strength nor the benchmark:

| Removed option | Preserved behavior |
|---|---|
| `CorrGuardCapture` | Train correction on capture-attributed residuals; graded weighting remains |
| `EvalPruneTtMinDepth` | Main-search TT eval refinement accepts depth 0 |
| `FutilityImprovingDir` | Reverse-futility margin uses the not-improving direction |
| `HistNoAging` | Age histories between searches |
| `LmrReduceLateEvasions` | Do not reduce evasions through LMR |
| `ProbCutStoreDepthAdj` | Store ProbCut evidence at `depth - 3` |
| `QsRefineMinDepth` | Qsearch TT refinement accepts depth 0 |
| `RootConfGapScale`, `RootConfWeightGap` | Keep root gap diagnostic-only; exclude it from confidence |
| `SingularMaxExtension` | Permit double extension through `SingularDoubleMargin` |

The Phase-4 SPSA config, fixed-option config, exclusion registry and registered
10,000-iteration launch were also removed. No games had been played.

### Inert but deliberately retained

“Inert” means the shipped default reproduces the accepted engine. It does not
mean “forgotten.” Every retained alternative has one later owner and must be
accepted or removed there.

| Owner | Retained work | Required disposition |
|---|---|---|
| Phase 7.3 post-NNUE search fit | Aspiration shape; NMP/TT-PV/IIR/singular provenance switches; `SingularTtDepthMargin`; `SelectivityProspectiveDepth`; `CorrSkipWhenTtRefined`; capture and continuation correction weights; root-confidence aspiration/time inputs | Re-measure after NNUE scale freezes; gate categorical architecture first, tune justified continuous consumers, then remove losers |
| Phase 8.0 multi-thread scaling | `RootConfPoolInstability`, `SmpIterationSkip` | Test only on representative 4T/8T+ topology; accept or remove |
| Phase 8.1 runtime ISA dispatch | Universal baseline dispatcher | Consider only as a complete Stockfish-style dispatch architecture, not a dead per-tier startup guard |

The post-NNUE owner is intentional: NNUE changes score scale, correction
residuals, pruning margins, node cost and time allocation. Tuning these against
HCE immediately before replacing HCE would fit the wrong objective.

## 3. Durable development rules

1. A roadmap target is not evidence. Rebase the roadmap when experiments refute
   the expected accumulation.
2. Estimate expected value before an expensive tune: mechanism prior, plausible
   Elo, compute cost, gradient quality and confirmation cost.
3. SPSA optimizes constants around an architecture; it does not create the
   missing mechanism required for a 50–100 Elo jump.
4. A technically correct schedule and resume path are necessary but do not
   justify launching. Sunk preparation work is not a reason to spend more.
5. Categorical gates precede continuous tuning. Do not let a binary switch
   receive another coordinate's SPSA gradient.
6. Node count, NPS and diagnostics explain a strength result; they do not
   replace paired games. A smaller tree can be weaker.
7. Final-PGO binaries decide material strength. Tune binaries and non-PGO probes
   can size or debug a mechanism only.
8. Do not add features from names or sibling engines. Require a local population,
   unique signal, an interaction model and an acceptance gate.
9. Correctness repairs may be retained without an Elo claim when the invariant
   is explicit and covered; record the exception rather than calling it free.
10. Release numbers describe shipped evidence, not the plan that once existed.

## 4. Measurement gates

| Change class | Minimum evidence |
|---|---|
| Correctness | Independent invariant/regression test; strength gate when behavior changes materially, unless an explicit correctness exception is recorded |
| Behavior-neutral refactor | Exact benchmark fingerprint plus tests; measure pooled NPS for hot-path work |
| Search/eval strength | Clean revision-matched PGO A/B, paired UHO, registered SPRT and stop rule |
| Tune | Registered surface/schedule/estimator, completed final theta, fresh PGO bake, SPRT; LTC/4T when transfer is plausible |
| Platform optimization | Target-native interleaved A/B, identical-binary calibration and executable ISA verification |
| Release | Prior-release comparison, platform matrix, UCI/bench/ISA smoke tests, docs/version consistency |

Default gain gates use `3+0.03`, one thread, Hash 64 and paired
`UHO_Lichess_4852_v1.epd`. Use `[3,10]` nElo only when the prior can plausibly
clear it; do not spend 16,000 games to measure an expected 2–5 nElo idea. The
harness calibration predicts roughly 14,500 games for a true candidate on the
10 nElo H1 boundary. Recalibrate after changing TC, book, adjudication or model.

## 5. Phase 5 — NNUE runway

- [ ] **5.0 Frozen measurement corpus.** Freeze quiet, tactical, endgame,
      rule-50, phase-balanced and search-disagreement cohorts. Record teacher
      SHA/settings, labels, hashes and untouched split IDs.
- [ ] **5.1 Per-ply state and dirty pieces.** Define exact reversible state and
      dirty-piece semantics for quiets, captures, EP, promotions, castling and
      null; randomized make/unmake compares board, keys, attacks and state.
- [ ] **5.2 Accumulator scaffolding.** Add per-thread/per-ply ownership, refresh
      markers and debug full-recompute seams while HCE search stays fingerprint-
      identical. No inference yet.
- [ ] **5.3 Trainer preflight.** Pin trainer/Bullet/toolchain/GPU; verify
      conversion, shuffle, deterministic splits/manifests, reference vectors
      and resume semantics. Malformed or lossy input fails loudly.
- [ ] **5.4 Runway gate.** Exact benchmark, fmt/tests, randomized unwind,
      reproducible pilot corpus and trainer conformance. Create an integration
      branch only after this passes.

## 6. Phase 6 — Baseline NNUE (target: justified 2.4.0)

- [ ] **6.0 Trainer hardening:** strict CLI, train/validation/untouched-test
      splits, checkpoint selection, hashes, seeds and exact references.
- [ ] **6.1 Controlled data:** generate 30–60M unique teacher positions; test
      search score/WDL blend, node budget, natural finishes and disagreement
      mining.
- [ ] **6.2 Baseline networks:** documented widths/buckets with at least two
      seeds; validation chooses within a run, untouched cohorts are used once.
- [ ] **6.3 Scalar integration:** strict embedded/EvalFile layout validation and
      exact Rust/NumPy/engine full-recompute agreement.
- [ ] **6.4 Incremental and SIMD:** dirty deltas per ply/thread; randomized
      incremental/full parity; integer bound proof; portable, x86 and ARM64
      kernels bit-exact and target-native PGO-smoked.
- [ ] **6.5 Architecture loop:** vary data, label blend, width/buckets,
      activation, learning rate and duration one axis at a time with two seeds.
- [ ] **6.6 Gross search-scale safety:** adjust only clearly invalid margins or
      clock scale. The broad fit waits until Phase 7.3.
- [ ] **6.7 Baseline release:** NNUE beats 2.3.2 at STC/LTC, transfers at 4T,
      passes external checks and has zero incremental/reference mismatch. Use
      **2.4.0 only if this is a material release**; otherwise remain on 2.3.x.

## 7. Phase 7 — NNUE frontier and final search fit

- [ ] **7.0 Residual/disagreement analysis:** phase, material, king, tactical,
      endgame, calibration, refresh cost and teacher-search disagreement.
- [ ] **7.1 Data frontier:** scale/deduplicate, natural finishes, hard-position
      mining and controlled label/depth A/Bs with untouched sets.
- [ ] **7.2 Architecture ladder:** king/perspective buckets, threat/material
      inputs, width/activation and refresh-friendly variants; two seeds, exact
      conformance, NPS and SPRT.
- [ ] **7.3 One post-NNUE search fit:** first resolve retained categorical
      switches; then register only continuous coordinates whose optimum likely
      moved. Coordinate count and horizon are derived from activation,
      curvature and compute budget—not a remembered target of 24 or 5,000.
      Bake final theta, PGO, SPRT, LTC/4T and ablate surprising winners.
- [ ] **7.4 Frontier gate:** direct 2.3.2/baseline-NNUE comparison and calibrated
      matches against contemporary target engines. This is where the Basilisk
      gap is re-measured and where a large-version claim is earned.

## 8. Phase 8 — Scaling, platforms and product completeness

- [ ] **8.0 High-thread/NUMA:** price the known depth-diversity deficit at
      4T/8T/16T; test retained pool instability/iteration skipping, first-touch,
      TT/accumulator sharing and false sharing. Remove losing switches.
- [ ] **8.1 Runtime dispatch and memory:** consider a baseline universal binary
      which selects specialized kernels, plus TT/network placement and large
      pages. Do not add a specialized-binary startup CPU guard.
- [ ] **8.2 Product/platform:** demand-led Chess960 or platform work; preserve
      scalar/SIMD and clock parity.
- [ ] **8.3 Scaling release:** full topology, clock, net, ISA and user-doc gate.

## 9. Optional HCE fallback

Enter only if serious NNUE contract, data and architecture retries fail and the
maintainer explicitly abandons NNUE. Diagnose the failure first, choose a small
residual-led HCE program, run one fitted/gated wave, and preserve all NNUE
artifacts for a later return.

## 10. 2.3.2 release checklist

- [x] Version set to 2.3.2; user README/changelog/release notes updated.
- [x] Canceled SPSA launch surface removed; future-owned mechanisms documented.
- [x] Abandoned options removed with accepted defaults hardwired.
- [x] `cargo fmt --check`, tests, clippy and feature builds pass.
- [x] Post-cleanup benchmark equals 6,519,711 / 2.449.
- [x] Tune UCI exposes no removed option and retained inert options remain.
- [x] Local PEXT PGO asset passes UCI, benchmark and ISA checks.
- [ ] Hosted release/CI matrix passes on the release commit.
- [ ] Commit locally. Tag/push/release only on maintainer instruction.

## 11. Common commands

```powershell
cargo fmt --check
cargo test --workspace --all-targets
cargo clippy --workspace --all-targets --all-features -- -D warnings
"bench" | ./target/release/rarog.exe
cargo xtask build --arch pext --pgo
cargo xtask verify-isa --arch pext
./tools/audit_spsa_coverage.ps1
```
