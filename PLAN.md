# Rarog development plan

Updated 2026-08-30. This is the current roadmap. Historical evidence belongs
in `EXPERIMENTS.md`; current status and commands belong in `GUIDE.md`.

## 1. Current state

| Item | State |
|---|---|
| Released baseline | **2.3.2** at `f931722` on `master` |
| Accepted search head | RAR-S70 on `dev`; `bench 13` = **6,977,070 nodes / EBF 2.466**, 1T |
| Integration state | The failed SearchCore rewrite is reverted by `c5e451d`; `d2c7788` upgrades measurement/fitting tools and `e4f10ca` adds robust per-position/per-iteration branching evidence without changing accepted behavior |
| Frozen search/HCE oracle | `hybrid` at `75d0d43`, Stockfish `9587eeeb` driving the exact Rarog 2.3.2 HCE |
| Measured search deficit | **355.26 +/- 27.03 Elo at equal nodes** and **250.77 +/- 13.12 Elo at equal time**; Rarog's speed is worth a measured **104.5 Elo** |
| Accepted Phase-4 gains | ProbCut move filtering **+15.56 +/- 10.02 Elo**; root LMR relief **+2.33 +/- 1.85 Elo** |
| Active game job | None |
| Current step | **4.7.1 — audit existing HCE self-play archives and the three-way corpus contract** |
| HCE state | Open now. No historical family or parameter group is presumed fitted; all real coordinates must be re-audited and refitted where identifiable with the correct linear/nonlinear instrument |
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
| Texel fit | Verified label domain; hash-complete whole-start train/validation/frozen-test splits; exact all-slot instrument coverage and bake smoke; reconstruction, covariance/identifiability and semantic bounds; settled trajectory, baked PGO and SPRT |
| SPSA | Registered live surface and immutable horizon, bounded sensitivity pilot when needed, completed full-surface theta, fresh PGO bake and SPRT |
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
| 4.11 | TT provenance consumers and raw/pruning/searched evaluation separation |
| 4.13 | Unconsumed continuation/capture correction and history alternatives |
| 4.13 or removal | NMP/IIR/singular provenance alternatives; extensions remain a measured null |
| 4.12 | `SelectivityProspectiveDepth` and cp-valued margins whose populations move under the fitted HCE |
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
converts less. Those remain high-value search questions, but Basilisk showed
that an HCE refit can materially move qsearch share while leaving other search
counters stable. Since Rarog's HCE population is larger and its complete
parameter surface has not been requalified, HCE qualification/refit is now
4.7–4.10 and the search-authority decision follows on the accepted HCE at 4.11.

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
  evidence after the fitted HCE, not another Stockfish-shaped port.

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

No HCE parameter family is frozen by historical acceptance. Material, PSTs,
mobility, threats, imbalance, sparse terms, king-danger inputs, scaling and all
other existing surfaces re-enter the audit. A coordinate may be fixed only for
an algebraic null direction, an invariant/padding slot or measured lack of
identifiability on the new corpus, with the reason recorded. “Already tuned” is
not a reason. Algebraic gauge fixing (for example material versus a constant
shift of every PST square) removes duplicate coordinates without removing the
represented chess degree of freedom.

| Family | Current maturity question |
|---|---|
| Score foundation | Phase/tempo/rule-50/lazy ordering and sign-preserving winnability |
| Material/PST/imbalance | Material-conditioned residuals versus compensating correlated terms |
| Pawns/passers | Blocker, file, exchange, race and conversion conditionality |
| Activity/space/threats | Pin-aware legal activity, usable space, safe pawn pushes and exact relations |
| King safety | Shelter/storm dimensionality, castling destination, pinned defenders, weak/flank inputs |
| Scaling/endgames | Exact material conversion, OCB scope, Syzygy-backed won/drawn/cursed separation |
| Calibration/data | Need a current frozen train/validation/frozen-test corpus, activation/covariance and full/lazy/search residuals |

Stockfish comparison may enumerate and test these contracts. Reciprocal family
ablation is optional coarse sensitivity evidence only; it cannot rank build
order by itself or assign recoverable Elo.

Manta strengthens the process, not the expected-value estimate. MAN-E19's
coherent coverage-plus-constrained-fit bundle won +35.91 +/- 11.19 Elo while
costing 36.2% evaluator throughput; MAN-E18/E20 show that a lower static loss
cannot rescue a semantically wrong or weak feature, and MAN-E21 shows that a
plausible faster mechanism can still lose games. Therefore categorical
semantics and instrument contracts precede their fits, while a complete
existing-surface refit precedes new structure. Static/NPS filters may reject
but never promote, and the whole fitted cluster pays its own game and
search-NPS gate.

Basilisk supplies a new ordering prior, not portable values. Its recent HCE
programme added sixteen terms and lost 77.92 Elo, then gained about 12 Elo by
removing those terms and refitting two old surfaces that had been incorrectly
excluded: nonlinear king safety (+2.64) and 768 PST weights (+9.52). A fit
reported as 348/1,190 parameters had hidden the omission. Its larger holdout
improvement lost while the 14-times-smaller one won. Therefore Rarog audits and
fits the complete existing representational surface before adding features,
and never ranks candidates by validation delta.

### 4.7 HCE data and instrument qualification — NEXT

#### 4.7.1 Dataset and label contract

1. Qualify the extraction path: stable whole-start hash assignment to
   train/validation/frozen-test, no replay leakage, retained rule-50 clock,
   exact phase quotas, sequential/parallel equivalence, atomic publication and
   hashes of every input, output and setting.
2. Verify the labels mechanically. A self-play-WDL corpus contains exactly
   `0`, `0.5` and `1`; blended/search/teacher targets require a distinct
   registered experiment and manifest. Do not trust a filename or summary.
3. Audit unique starts, phase and exact-material coverage, natural mate and
   decisive-result coverage, draw rate and per-phase usable yield before a fit.
   Rounds may not exceed independent starts. Use diverse Beast starts with
   Rarog's own current search and no adjudication; external-engine outcomes are
   not a default label source.
4. Freeze split hashes once. Validation selects; the frozen test opens once
   after the completed vector is selected.

#### 4.7.2 Complete parameter-to-instrument audit

Enumerate every `EvalParams` slot and assign exactly one disposition:

- linear gradient fit;
- nonlinear re-evaluation/coordinate or finite-difference fit;
- algebraic gauge representative;
- invariant/padding slot; or
- corpus-unidentifiable, with activation/covariance evidence and retry trigger.

The audit must total exactly to `EvalParams::FLAT_SIZE`. Old sparse findings and
old fitted groups are re-measured on the new corpus. PSTs, material and every
historically staged family are included; a group label such as `all` is not
proof of coverage. For capped/bucket-selecting terms—including king-danger
inputs and any winnability/scaling contract—the linear trace is not presumed to
provide a valid gradient merely because it records an activation.

Run an end-to-end smoke before a production fit: deliberately move one value in
every instrument class, require the vector, bake, source and benchmark to move,
check every native exit code, rebuild the tuner from the baked source and
restore the tree. A success banner over an unchanged fingerprint is a failure.

#### 4.7.3 Current-source maturity and residual map

For current Rarog emit raw, lazy, corrected, qsearch and depth-N values; full
residuals by phase/material/king/passer cohorts; feature support, covariance and
identifiability; and exact `EvalTrace` reconstruction. Classify every maturity
family as equivalent, intentionally different with evidence, fit candidate,
structural candidate or rejected. Reciprocal Stockfish ablation remains an
optional coarse prior only. Register no HCE candidate until this record and the
parameter-to-instrument audit are complete.

### 4.8 Refit the complete existing HCE surface

This step tests whether Rarog is mis-calibrated before assuming it is
under-featured.

1. Register initialization, data/split hashes, free/gauge/invariant slots,
   semantic rails, regularization, seed, alternating schedule and checkpoint
   rule. Preserve every representational degree of freedom despite any gauge.
2. Fit the complete identifiable linear surface jointly, including PSTs and
   every previously staged/sparse family now supported. Record the initial
   vector and full train/validation trajectory.
3. Fit every activated nonlinear/capped surface with its correct instrument:
   at minimum king-danger table plus selectors and all scaling/winnability
   parameters the audit marks reachable. Prefer deterministic coordinate or
   finite-difference fitting; use SPSA only for a small interacting residue
   those instruments cannot resolve.
4. Alternate linear and nonlinear passes only under the registered schedule.
   The selected vector must be settled, semantically valid and baked in full.
   Stop on the first failed/no-gain cycle or return to the same attractor.
5. Static loss, signs, cohorts and pooled evaluator/search NPS may refute only.
   Bake clean PGO and run one registered no-adjudication SPRT against the
   pre-refit HCE. Accept or restore before 4.9.

RAR-E03/RAR-E04 and Basilisk establish why the gate is mandatory: label/loss
improvements can be neutral or catastrophically wrong. Conversely, Basilisk's
accepted +9.52 Elo came from only -0.43% holdout loss; small offline movement
is not a reason to skip a properly formed candidate.

### 4.9 Structural HCE upgrades — CONDITIONAL

Open at most two dependency-complete structural clusters, and only for residual
signals the full existing-surface refit could not represent. King-safety
conditionality, material-specific winnability/endgames and passer/threat
conditionality remain hypotheses, not an order.

For each cluster:

1. define categorical semantics and directional/counterfactual tests;
2. reconstruct every changed feature exactly through `EvalTrace` or a named
   nonlinear instrument;
3. locally refit the changed feature and **all materially covariant existing
   parameters**—historical group boundaries do not freeze them;
4. apply prospective semantic/support/loss/NPS filters as refutation only;
5. bake final PGO and run the registered no-adjudication SPRT;
6. accept or revert before selecting the next cluster.

Two fully fitted cluster failures close structural expansion and force a 4.7
re-audit; they do not authorize more feature inventory.

### 4.10 Post-structure whole-HCE consolidation

If 4.9 accepts any representation, rerun the complete 4.8 linear/nonlinear
instrument schedule over the new model, retain the trajectory, open the frozen
test once and gate the baked vector against the pre-consolidation accepted HCE.
If 4.9 accepts no representation, close 4.10 as already satisfied by 4.8.

A second data cycle requires a prospective changed-data hypothesis supported by
the first fit and game verdict. More games, labels or epochs are not a default
response to a failed fit.

### 4.11 Post-HCE qsearch, TT and evaluation authority

HCE fitting can change score scale, qsearch share and pruning populations.
Basilisk's +12-Elo HCE refit moved qsearch share from 30.8% to 35.1% while most
ordering/LMR statistics held; which metrics move is engine-specific. Therefore
the old RAR-S70 counters are priors, not a candidate basis.

#### 4.11.1 Observation and baseline

1. Compare the accepted HCE head with exact RAR-S70 at fixed nodes/time, then
   re-run the revision-matched oracle differential at sample stride 1.
2. Profile cumulative and per-iteration nodes over a full-suite shallow/mid
   segment and a fixed representative deep segment that reaches playing depth.
   Report aggregate and per-position median/min/max. Do not infer a target from
   one endpoint, cumulative shallow ratios, absolute cross-engine node counts
   or outlier-sensitive mean depth.
3. Measure main/qsearch TT probe, hit, cutoff and store authority; qsearch entry,
   stand-pat, generated/searched/pruned move reasons; raw/corrected/pruning/
   stand-pat/searched score ownership; and explicit same-unit denominators.
4. Prove each wire and UCI option live with an absurd value. Parameter sweeps
   use a real `go nodes`/`go depth` path; `bench` is valid only after proving it
   consumes that option.
5. Write `analysis/phase4_qsearch_tt_authority.md` with the dependency map and
   an explicit candidate/no-candidate decision.

#### 4.11.2 Candidate and gate, only if 4.11.1 isolates one

The design prior is a Rarog-native authority bundle: preserve exact raw HCE;
keep a separate pruning value; refine only from compatible searched evidence;
and retain qsearch stand-pat/search/store provenance. Manta MAN-S19's +13.02
nElo corroborates the question, not a formula or expected value. Basilisk's
recent contract inventory likewise shows why internal coherence and actual
consumer semantics outrank feature parity or reference constants.

Implement the smallest dependency-complete change, prove switch-off identity,
fit only a justified continuous residue and run the registered `[0,3]` PGO
SPRT. If no unique signal exists, close without code.

### 4.12 Optional post-HCE search SPSA

Open only if several live cp-valued RFP, null, futility, ProbCut, qsearch,
correction or LMR coordinates show a displaced interacting optimum. First run
a registered bounded sensitivity pilot, then audit the entire active
interacting surface. Pilot theta is neither candidate nor seed; the full tune
starts from accepted defaults and preserves its registered horizon under any
staged `StopAfter`. Never mix HCE and search coordinates.

### 4.13 Search cleanup and checkpoint

Remove every unconsumed 4.6/retained alternative without a future owner.
Re-run debug/release tests, all-feature/all-target clippy, exact benchmark,
pooled-PGO NPS, fixed-time/fixed-node deficits and the accepted 4.11/4.12 game
verdicts. Preserve only diagnostics with a named Phase-5/7 owner.

### 4.14 Final HCE/search checkpoint

Compare final head with exact RAR-S70 using revision-matched final-PGO binaries
and no adjudication. Record separately attributed HCE and post-HCE-search Elo,
NPS, fixed-node behavior, STC and LTC direction. Ablate surprising integrated
contributors and close every maturity classification.

The HCE is mature for this release only when:

- the current-source family map contains no unknown or first-draft row;
- every accepted representation reconstructs through `EvalTrace` and has
  activation/covariance plus a game verdict;
- every real parameter slot has a named, verified fitting instrument or a
  written invariant/gauge/unidentifiable disposition;
- the complete existing HCE refit and any post-structure consolidation have
  clean game verdicts;
- optional HCE/search SPSA is completed and gated or explicitly skipped;
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

Phase 5 creates no intended playing-strength change. Work that 4.7 already
completed is reused and extended, not rebuilt.

- **5.0 Measurement corpus handoff.** Freeze the accepted 4.7 corpus and
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
4.7 residual corpus. Any family accepted in 4.9 is closed here.

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
| `tools/branching_profile.ps1` | Hash-bound per-position and per-iteration depth/branching shape with robust aggregates; refutation evidence only |
| `tools/pgn_result.ps1` | Reconstruct complete-pair PGN results |
| `tools/build_test.ps1` | Hash-bound build manifests and exact benchmark qualification |
| `tools/spsa.ps1` | Registered targeted SPSA with immutable horizon and staged stop |
| `tools/texel/extract.py`, `extract_parallel.py` | Leak-resistant three-way phase-balanced extraction |
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
'bench 13' | .\target\release\rarog.exe
```
