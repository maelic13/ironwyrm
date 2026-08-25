# Rarog development plan

Updated 2026-08-25. This is the current roadmap. Historical evidence belongs
in `EXPERIMENTS.md`; current status and commands belong in `GUIDE.md`.

## 1. Current state

| Item | State |
|---|---|
| Released baseline | **2.3.2** at `f931722` on `master` |
| Accepted search head | RAR-S70 on `dev`; `bench 13` = **6,977,070 nodes / EBF 2.466**, 1T |
| Integration state | The failed SearchCore rewrite is reverted by `c5e451d`; accepted behavior is RAR-S70 plus diagnostics |
| Frozen search/HCE oracle | `hybrid` at `75d0d43`, Stockfish `9587eeeb` driving the exact Rarog 2.3.2 HCE |
| Measured search deficit | **355.26 +/- 27.03 Elo at equal nodes** and **250.77 +/- 13.12 Elo at equal time**; Rarog's speed is worth a measured **104.5 Elo** |
| Accepted Phase-4 gains | ProbCut move filtering **+15.56 +/- 10.02 Elo**; root LMR relief **+2.33 +/- 1.85 Elo** |
| Active game job | None |
| Current step | **4.7.1 — qsearch/TT authority observation and fixed-node baseline** |
| HCE state | Frozen through 4.8. Broad coverage and infrastructure are mature; current conditional semantics, residual calibration and a post-structure whole-HCE fit are not |
| Next release | Conditional **2.4.0** after 4.15; baseline NNUE then targets **2.5.0** |

Phase 4 remains a bounded pre-NNUE programme because the hybrid established
large search and HCE populations worth investigating. It is not a commitment
to keep working until the oracle is matched. Each dependency-complete cluster
must earn its own continuation.

## 2. Operating and evidence rules

`AGENTS.md` is authoritative for measurement, verification, documents and
gating. The following rules determine this roadmap's order.

1. Similarity to Stockfish is never an objective or acceptance criterion.
   Reference code identifies problems, dependencies and useful tests; Rarog
   implements its own answer and games decide.
2. Cross-engine ablation measures **marginal value inside each co-adapted
   engine**. It may rank questions, but it is not portable headroom and family
   losses must not be summed.
3. A strength candidate starts from the latest accepted head, is registered in
   `EXPERIMENTS.md` before games, and ends accepted or reverted before another
   candidate opens.
4. The normal unit is one smallest dependency-complete, locally fitted
   cluster. Node count, EBF, tactical suites, fit loss and oracle agreement
   explain or refute a candidate; only a clean final-PGO SPRT accepts it.
5. Default gain bounds are `[0,3]` nElo. Use wider, symmetric or
   simplification bounds only when the prospective prior and RAR-M10 sizing
   justify them. Never change the gate after seeing games.
6. Search SPSA is conditional on live coordinates, interaction and local
   curvature. HCE Texel fitting owns traced linear coefficients. HCE SPSA owns
   only activated nonlinear/global residue that the linear trace cannot fit.
7. Search and HCE coordinates never share a tune. After an HCE changes,
   cp-valued search consumers are audited and, if justified, fitted separately.
8. Cross-evaluator cohorts and HCE-changing A/Bs default to no adjudication.
9. Two fully implemented clusters in the same track without an accepted gain
   stop implementation and force a new evidence audit.
10. Engine, tooling and documentation changes remain separate commits.

### Minimum gates

| Change | Minimum evidence |
|---|---|
| Correctness | Independent invariant/regression test; strength gate if playing behavior changes materially |
| Behavior-neutral hot path | Exact fingerprint, debug/release tests, pooled NPS |
| Search/HCE strength | Revision-matched clean PGO A/B, paired UHO, registered SPRT and stop rule |
| Texel fit | Fixed train/validation/untouched splits, trace reconstruction, activation/covariance, baked PGO and SPRT |
| SPSA | Registered live surface and schedule, completed theta, fresh PGO bake and SPRT |
| Release | Prior-release STC/LTC, 4T direction, NPS, platform/ISA matrix and user-facing docs |

### Independence boundary

Both engines are GPL, but Phase 4 deliberately builds an independent Rarog
design. Problems, dependencies, populations and known failure modes may cross
from a reference. Source, tuned constants, tables, identifiers and structural
transcription may not. The frozen `hybrid` branches are diagnostic artifacts;
they are never merged or shipped.

## 3. Accepted foundation through 2.3.2 and RAR-S70

| Work | Evidence / disposition |
|---|---|
| Broad selectivity fit | Accepted at +15.33 +/- 7.34 nElo |
| Zero-reduction LMR floor | Accepted at +9.13 +/- 5.45 nElo |
| Anchored HCE refresh | Accepted at +11.56 +/- 5.19 Elo, RAR-E05 |
| Typed TT evidence and provenance | Retained infrastructure; behavior-neutral at accepted defaults |
| Root abort/fallback and correctness coverage | Retained infrastructure |
| AArch64 TT prefetch | Accepted at +1.42% median NPS on M4 |
| Phase-4 ProbCut move filter | Accepted at +15.56 +/- 10.02 Elo, RAR-S57/S58 |
| Root-only LMR relief | Accepted at +2.33 +/- 1.85 Elo, RAR-S70 |

Retained default-off switches are not evidence. Each must be consumed by its
named step or removed:

| Owner | Retained surface |
|---|---|
| 4.7 | TT provenance consumers and raw/pruning/searched evaluation separation |
| 4.8 | Unconsumed continuation/capture correction and history alternatives |
| 4.8 or removal | NMP/IIR/singular provenance alternatives; extensions remain a measured null |
| 4.13 | `SelectivityProspectiveDepth` and cp-valued margins whose populations move under the fitted HCE |
| 8.0 | `RootConfPoolInstability`, `SmpIterationSkip` and high-thread ownership |

## 4. Phase 4 — strongest bounded pre-NNUE search and HCE

### Objective and measured interpretation

The clean no-adjudication RAR-O02 hybrid gave two aggregate observations:

| Contrast | Result | Meaning |
|---|---:|---|
| Stockfish search minus Rarog search, Rarog HCE held | about **+196.5 Elo** | Mature search can use Rarog's HCE much better; not an individual mechanism forecast |
| Stockfish HCE minus Rarog HCE, Stockfish search held | about **+328.6 Elo** | HCE remains a major population; not a sum of portable family gains |

The later matched search ablation initially appeared to assign 116 Elo to LMR
and 124.6 Elo to shallow pruning. Four LMR candidates then measured flat even
though Rarog's LMR base formula matched the reference within 2% and Rarog
ordered better. The corrected conclusion is the phase's central constraint:
the ablation differences measured each mechanism's marginal value inside a
different engine, not Rarog implementation headroom.

Fixed-node measurement subsequently corrected the residual too. Rarog is
**355.26 Elo behind at equal nodes**, its speed closes **104.5 Elo**, and after
the mask-160 comparison the non-LMR/non-shallow residual is about **83 Elo**,
not the obsolete 30. Current counters place qsearch and TT in that population:
Rarog runs about 1.60x the oracle's qsearch per node, hits the TT more and
converts less. This is why 4.7 is next.

### Completed steps

| Step | Status and durable conclusion |
|---|---|
| **4.0** Evidence, baseline and oracle freeze | Closed by RAR-M12 |
| **4.1** Instrumented oracle | Closed on `hybrid-diag` `de568b3` |
| **4.2** Differential observation harness | Closed by RAR-S55; all counter units and invariants must remain explicit |
| **4.3** Mechanism map and order freeze | Closed; reference divergences are questions, never target values |
| **4.4** Matched-ablation instrument and fixed-node correction | Closed; every mask bit proved live; marginal-value interpretation corrected |
| **4.5** LMR contract study | Closed with no accepted interior gain after four candidates; RAR-S70 root relief remains accepted |
| **4.6** Shallow-selectivity/rewrite continuation | **Closed with no accepted gain**; details below |

#### 4.6 closed disposition

- **4.6.1 Quiet SEE prune:** `QuietSeePruneDepth=6`, coefficient 25,
  completed 652 paired-score games against the oracle at
  **-247.39 +/- 23.69 Elo**. Against `G(0) = -250.77 +/- 13.12`, estimated gap
  closure is only **+3.38 +/- 27.08 Elo**. This is a stopped diagnostic null,
  not an SPRT boundary; the candidate stays off.
- **4.6.2 SearchCore rewrite:** Steps 13 and 16 were rebuilt together. It
  changed the fingerprint from 6,977,070 / 2.466 to 3,479,169 / 2.343 and
  solved 182/300 WAC positions against 167/300 on fewer nodes, but at the
  stopped paired sample it scored **-9.76 +/- 17.70 Elo** over 712 complete
  games, LOS 13.76%. It never reached the registered `[-5,5]` boundary. The
  wholesale rewrite was reverted by `c5e451d` because its structural and
  constant effects were inseparable and its best zero-game signals did not
  predict play.
- **4.6.3 Decision:** no selectivity SPSA and no second broad search rewrite.
  The planned SPSA entry condition was not met: neither an accepted replacement
  contract nor a shrinking matched gap exists. Re-entry requires new local
  evidence after 4.7 or the fitted HCE, not another Stockfish-shaped port.

### 4.7 Qsearch, TT and evaluation authority — NEXT

This is one bounded search cluster, not a reopening of every parked TT option.
Its question is whether Rarog loses useful work because raw HCE, pruning
evaluation, qsearch stand-pat and searched TT bounds have insufficiently
explicit producer/consumer authority.

#### 4.7.1 Observation and baseline

1. Rebuild exact RAR-S70 immediately before measurement with the intended
   feature set and reproduce 6,977,070 / 2.466.
2. Re-run the versioned differential suite at
   `RAROG_DIAG_SAMPLE_STRIDE=1`, plus fixed-node and fixed-time `G(0)`, using
   the repository parsers only.
3. Extend counters only where the current contract cannot answer:
    - main versus qsearch TT probe, hit, usable cutoff and store;
    - entry writer/provenance, depth/bound/window rejection reason and consumer;
    - qsearch entry reason, stand-pat outcome, moves generated/searched/pruned,
      checks/evasions/promotions and SEE/delta exemptions;
    - raw HCE, corrected/pruning value, stand-pat and searched score authority;
    - all denominators stated as per main node, per qnode or per move.
4. Prove every new wire live with an absurd-value or forced-position probe,
   then prove diagnostics-off identity.
5. Write `analysis/phase4_qsearch_tt_authority.md` with the same-unit baseline,
   dependency map and explicit candidate/no-candidate decision.

#### 4.7.2 Candidate, only if 4.7.1 isolates one

The default design prior is a Rarog-native authority bundle: preserve exact raw
evaluation; keep a separate pruning value; allow TT refinement only from
compatible ordinary searched evidence; make qsearch stand-pat, searched moves
and stored bounds retain their provenance. Manta MAN-S19's +13.02 nElo is a
process-level corroboration, not a formula or expected value.

Do not retry the rejected flat minimum-depth knobs. Implement the smallest
dependency-complete producer/consumer change that the counters support, prove
switch-off identity and populate focused tests. If no unique signal exists,
4.7 closes without code.

#### 4.7.3 Fit and gate

Categorical authority is frozen before tuning. Search SPSA is used only if at
least two live continuous coordinates interact and a zero-game local sweep
shows curvature; otherwise reasoned local seeds go directly to the gate.
Register the PGO candidate against exact RAR-S70, normally `[0,3]` nElo with a
RAR-M10-sized overnight cap. Accept or revert before 4.8.

### 4.8 Search checkpoint and freeze

1. Remove 4.6 scaffolding and every retained search alternative with no
   measured owner. Preserve diagnostic infrastructure with a Phase-5/7 owner.
2. Re-run debug/release tests, all-feature/all-target clippy, exact benchmark,
   pooled-PGO NPS, fixed-time `G(0)` and the fixed-node deficit.
3. Record source hash, binary hash, fingerprint, NPS and the separate strength
   verdict for 4.7.
4. Freeze one 1T search head for the HCE programme. No second pre-HCE search
   cluster opens unless 4.7 exposes a separate, high-population defect and the
   maintainer prospectively amends this plan.

### HCE maturity conclusion

The current-code comparison is
`analysis/hce_maturity_2026-08-25.md`. The old 2026-07 audit is historical:
its four concrete activation defects (`attacked2`, enemy rook/passers,
unstoppable passers and phalanxes) were fixed by `d5a6054` and are tested.

Rarog already has a broad approximately 1,200-parameter tapered HCE with
trace reconstruction, caches, material/PST, mobility, threats, nonlinear king
danger, imbalance, passers, specialized endgames, lazy evaluation, rule-50
damping and correction history. It is not yet mature under the Phase-4 bar
because current conditional semantics and residual calibration are incomplete,
and no whole-HCE fit has been run after the representations this programme may
change.

| Family | Current maturity question |
|---|---|
| Score foundation | Phase/tempo/rule-50/lazy ordering and sign-preserving winnability |
| Material/PST/imbalance | Material-conditioned residuals versus compensating correlated terms |
| Pawns/passers | Blocker, file, exchange, race and conversion conditionality |
| Activity/space/threats | Pin-aware legal activity, usable space, safe pawn pushes and exact relations |
| King safety | Shelter/storm dimensionality, castling destination, pinned defenders, weak/flank inputs |
| Scaling/endgames | Exact material conversion, OCB scope, Syzygy-backed won/drawn/cursed separation |
| Calibration/data | Current frozen train/validation/untouched corpus, activation/covariance and full/lazy/search residuals |

Stockfish comparison may enumerate and test these contracts. Reciprocal family
ablation is optional coarse sensitivity evidence only; it cannot rank build
order by itself or assign recoverable Elo.

### 4.9 HCE evidence base and current-source maturity map

1. Pull Phase 5.0's measurement corpus forward: by-game
   train/validation/untouched-test splits, exact material/phase/king/passer
   cohorts, paired counterfactuals, Syzygy WDL/DTZ and external deep labels.
   Record source, teacher, settings, split IDs and hashes.
2. For current Rarog emit raw, lazy, corrected, qsearch and depth-N values;
   full residuals by cohort; activation and covariance; and exact `EvalTrace`
   reconstruction.
3. Classify every maturity family as equivalent, intentionally different with
   evidence, candidate, or rejected. There may be no stale/fixed defect or
   unclassified first draft in the map.
4. If reciprocal Stockfish family ablation is retained, prove every bit live,
   keep arms in a readable band and use it only as one prior alongside local
   residual, activation, covariance and NPS.
5. Register no structural candidate until the evidence selects a local signal.

### 4.10 Structural HCE upgrades

Select at most two dependency-complete structural clusters. Selection requires
all of: a missing or misspecified local signal, meaningful activation, residual
concentration, manageable covariance and acceptable projected NPS cost.
King-safety conditionality, material-specific winnability/endgames and
passer/threat conditionality are current hypotheses, not a fixed order.

For each cluster:

1. freeze categorical semantics and add directional/counterfactual tests;
2. make every changed feature reconstruct exactly through `EvalTrace`;
3. run a local anchored Texel fit over moved and materially covariant weights;
4. check frozen validation/cohorts, pooled evaluator and search NPS;
5. bake final PGO and run the registered no-adjudication SPRT;
6. accept or revert before selecting the next cluster.

Two fully fitted cluster failures stop 4.10 and force a 4.9 re-audit.

### 4.11 Whole-HCE Texel consolidation — REQUIRED

After all accepted representations freeze, run one anchored whole-HCE fit over
the active identifiable vector. Pin exact free/fixed groups, data/split hashes,
regularization, initialization, seed and checkpoint rule before training.
Validation chooses the completed vector; the untouched test is reported once
and never used to select weights.

Bake the completed vector into clean PGO and run one no-adjudication SPRT
against the pre-consolidation accepted HCE. Lower loss cannot accept it:
RAR-E03 lost 17.11 Elo despite lower holdout loss and RAR-E04 was flat despite
better on-policy validation. A second data/fit cycle is allowed only if
registered prospectively and both the first held-out report and game verdict
support a concrete changed-data hypothesis. Stop on the first failed cycle.

### 4.12 Optional HCE nonlinear SPSA — CONDITIONAL

This step may close as skipped. Open it only when the frozen HCE contains
important activated nonlinear/global parameters that `EvalTrace` cannot fit,
their local response is not flat or monotone, and a small joint surface has a
credible game-objective interaction. Freeze all linear Texel weights, register
coordinates/schedule/rails/cap and require a completed theta, fresh PGO bake
and SPRT. No full-surface HCE SPSA and no checkpoint selection after the fact.

### 4.13 Post-HCE search compatibility

The fitted HCE changes centipawn scale, residuals and pruning confidence.
Re-measure activation and local response for cp-valued RFP, null, futility,
ProbCut, qsearch, correction and LMR consumers. Keep categorical search policy
frozen. Run one targeted search-margin SPSA only if several live coordinates
show a displaced interacting optimum; otherwise make only a registered narrow
repair or close the step. Never tune HCE and search coordinates together.

### 4.14 Final HCE/search checkpoint

Compare the accepted fitted HCE head with the 4.8 frozen-search baseline using
revision-matched final-PGO binaries and no adjudication. Record HCE-attributed
Elo, NPS, fixed-node search behavior, STC and LTC direction. Ablate surprising
contributors. Close every maturity classification and record why 4.12/4.13
ran or were skipped.

The HCE is mature for this release only when:

- the current-source family map contains no unknown or first-draft row;
- every accepted representation reconstructs through `EvalTrace` and has
  activation/covariance plus a game verdict;
- the required whole-HCE Texel consolidation has a clean verdict;
- optional SPSA is completed and gated or explicitly skipped for lack of a
  justified surface;
- the fitted HCE remains a tested fallback and suitable datagen baseline.

### 4.15 Transfer, portability, SMP and release gate

1. Compare final head directly with 2.3.2 at STC, LTC `10+0.1` and 4T.
2. Record pooled-PGO NPS, benchmark, UCI, correctness, platform and ISA matrix.
3. Drop `-use-affinity` for multi-thread cells and calibrate a null pair under
   that topology.
4. Run a final no-adjudication target cohort including Basilisk and the oracle
   as diagnostic reference points.
5. Remove diagnostic scaffolding without a future owner; retain the ablation
   instrument and frozen oracle branches.

#### Release rule

- 2.4.0 requires cumulative STC point estimate at least **+40 Elo** over 2.3.2,
  95% lower bound above **+25 Elo**, positive LTC and 4T lower bounds, and all
  release gates.
- A cumulative result at or above +100 Elo with lower bound above +75 may
  justify a higher minor version by maintainer decision.
- Below the bar, ship 2.3.x only by explicit decision or close Phase 4 without
  a release. NNUE follows either way.

## 5. Phase 5 — NNUE runway

Phase 5 creates no intended playing-strength change. Work that 4.9 already
completed is reused and extended, not rebuilt.

- **5.0 Measurement corpus handoff.** Freeze the accepted 4.9 corpus and
  manifests as the NNUE residual/stage-gate source. Add only NNUE-specific
  labels or scale; preserve untouched splits.
- **5.1 Per-ply state and dirty pieces.** Define exact deltas for quiets,
  captures, EP, promotion, castling and null. Randomized make/unmake compares
  board, keys, attacks and state against full refresh every ply.
- **5.2 Accumulator scaffolding.** Per-thread/per-ply ownership, refresh
  markers, debug full-recompute seams and reserved king-bucket refresh cache.
  HCE remains active and search stays fingerprint-identical.
- **5.3 Trainer preflight.** Pin `D:/code/net_trainer`, Bullet, toolchain and
  GPU; verify conversion, shuffle, splits, manifests, reference vectors and
  resume semantics.
- **5.4 Runway gate.** Exact benchmark, debug/release tests, randomized unwind,
  reproducible pilot corpus and trainer conformance.
- **5.5 Threat-map hooks, optional.** Reserve only if Phase-7 relation inputs
  would otherwise require another make/unmake rewrite.

Boundary rule: search consumes an evaluator score and evidence class, never
evaluator internals.

## 6. Phase 6 — baseline NNUE

- **6.0 Trainer hardening.** Strict CLI, deterministic splits, hashes, seeds,
  checkpoint selection and exact references.
- **6.1 Controlled data.** Generate 30–60M unique positions with by-game
  splits, deduplication, external and tablebase cohorts, manifest provenance
  and a validated score/result blend.
- **6.2 Baseline networks.** At least two seeds for documented widths and
  buckets; validation selects, untouched cohorts report once.
- **6.3 Scalar integration.** Implement the documented `quantised.bin`
  contract with integer-exact engine/NumPy/reference conformance and clean HCE
  fallback.
- **6.4 Incremental and SIMD.** Randomized incremental/full parity, integer
  bounds, portable/x86/ARM bit identity and pooled-PGO NPS gate.
- **6.5 Architecture loop.** Controlled data-versus-capacity experiments,
  progressing from output buckets to mirrored king buckets and then justified
  relation/multilayer inputs.
- **6.6 Gross search-scale safety.** Repair only clearly invalid scale/margins;
  broad search fitting waits for 7.3.
- **6.7 Baseline release.** Beat the accepted pre-NNUE master at STC/LTC,
  transfer at 4T, pass platform gates and archive every accepted net with its
  reproducible training manifest.

## 7. Phase 7 — NNUE frontier and final search fit

- **7.0** Residual and disagreement analysis by phase, material, king,
  tactical/endgame cohort, calibration and refresh cost.
- **7.1** Data frontier: scale, deduplicate, mine hard positions and refresh
  on-policy data only when a clearly stronger net changes the policy.
- **7.2** Architecture ladder: king/material/threat/pawn relation inputs,
  width/activation and refresh-friendly variants, one axis at a time.
- **7.3** One post-NNUE search fit over demonstrably displaced live
  coordinates, followed by PGO, SPRT, LTC and 4T.
- **7.4** Frontier gate against 2.3.2, the Phase-4 head and target engines.

## 8. Phase 8 — scaling, platforms and product completeness

- **8.0 High-thread and NUMA.** Price the measured depth-diversity deficit at
  4/8/16T; test helper depth/ordering/TT ownership and retained SMP switches.
- **8.1 Runtime dispatch and memory.** Universal dispatch, TT/net placement and
  large pages only as complete architectures with target-native evidence.
- **8.2 Product/platform.** Demand-led Chess960 and platform work; consider
  OpenBench-style distributed testing when typical gains reach 1–3 Elo.
- **8.3 Scaling release.** Full topology, clock, net, ISA and user-doc gate.

## 9. Optional post-NNUE classical fallback

Enter only if a serious king-conditioned NNUE, inference optimization and
data-scale retry fail and the maintainer explicitly abandons NNUE. Reuse the
4.9 residual corpus. Any family accepted in 4.10 is closed here.

1. King-safety semantic rework.
2. Material-specific winnability and scaling.
3. Passer/pawn conditionality.
4. Threat and usable-activity conditionality.
5. Material/phase specialization only as a last classical step.

Every fallback item is structure plus fit plus one gate, not additive term
accretion.

## 10. Release checklist

- [ ] Version, README, CHANGELOG and release notes agree.
- [ ] `cargo fmt --check` passes.
- [ ] Workspace/all-target tests pass in debug and release.
- [ ] All-feature/all-target clippy passes with zero warnings.
- [ ] Feature builds and tune-option inventory are correct.
- [ ] Benchmark fingerprint is recorded and every move explained.
- [ ] Local PGO asset passes UCI, benchmark and ISA verification.
- [ ] Prior-release STC/LTC and 4T direction pass the release rule.
- [ ] Hosted platform/CI matrix passes on the release commit.
- [ ] Commit locally; tag, push and publish only on maintainer instruction.

## 11. Reference tools and commands

| Tool / path | Purpose |
|---|---|
| `tools/sprt.ps1` | Paired pentanomial GSPRT; default 1T `3+0.03`, Hash 64, UHO |
| `tools/diag/phase4_differential.py` | Same-unit Phase-4 suite aggregation |
| `tools/diag/bench_counters.py` | Sum all per-position bench counter dumps |
| `tools/pgn_result.ps1` | Reconstruct complete-pair PGN results |
| `tools/spsa.ps1` | Registered targeted SPSA only |
| `cargo xtask build --arch <arch> --pgo` | Production PGO asset |
| `cargo xtask verify-isa --arch <arch>` | Executable ISA contract |
| `hybrid/build.ps1` | Frozen diagnostic oracle package, hybrid branch only |
| `D:/code/net_trainer` | Phase-6 NNUE data/training stack |

```powershell
cargo fmt --check
cargo test --workspace --all-targets
cargo test --workspace --all-targets --release
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo build --release
@("bench 13", "quit") | .\target\release\rarog.exe
```
