# Rarog development plan

Updated 2026-08-18. This is the current roadmap. Detailed historical evidence
lives in `EXPERIMENTS.md`; the operational tracker lives in `GUIDE.md`.

## 1. Current state

| Item | State |
|---|---|
| Released baseline | **2.3.2** at `f931722` on `master` |
| Search fingerprint | `bench 13` = **6,922,439 nodes**, geomean EBF **2.451**, 1T (4.7c accepted, RAR-S57/S58; 2.3.2 was 6,519,711 / 2.449) |
| Integration branch | `dev`, reset to `master` and carrying this plan |
| Frozen oracle | `hybrid` at `75d0d43` — Stockfish `9587eeeb` driving the exact 2.3.2 HCE |
| Active game jobs | None. The stopped no-adjudication hybrid tournament already settled the architectural decision |
| Current phase | **Phase 4 — reference-accelerated search and HCE development**; 4.0–4.4 closed. **RENUMBERED 2026-08-21** on the ablation decomposition: search track 4.5–4.9 (selectivity, which holds 240 of the 250.8 Elo deficit), HCE track 4.10–4.14, release 4.15 |
| Next release | **2.4.0 at 4.15** if the work transfers; a larger cumulative gain may justify a higher minor version. Baseline NNUE then targets **2.5.0** |
| Reference posture | Search maturity is audited against local Stockfish `5062aee5`; HCE maturity against `9587eeeb`, its last pure-HCE master. Both are read for **ideas** only. No Stockfish code or tuned constant enters Rarog |
| HCE status | Frozen through 4.10. The evaluator has mature infrastructure but incomplete feature contracts. Structural work, local Texel refits and narrowly justified SPSA reopen at 4.11 |

**Phase 4 changed scope on 2026-08-12.** The original Phase 4 was a pre-NNUE
search-and-tuning programme that closed with 2.3.2 and a cancelled SPSA. It is
released and must not be reopened. The number is now reused for a different
programme, described in §4, created by the search-oracle experiments RAR-O01
and RAR-O02: Rarog's largest measurable deficit is **search coordination**,
with a second large deficit in **HCE feature coverage**, and both can be
attacked with a public engine as an idea source instead of rediscovered
blindly.

The old partial rating observation — an unfinished development binary roughly
+11 pool Elo over 2.3.1 and 39 below Basilisk 1.9.3 — remains diagnostic only
(RAR-M08). It was never a release gate or a forecast. The target-engine ladder
moves to the 4.15 release gate and, afterward, the post-NNUE frontier gate at
7.4.

## 2. Development process, rules and gates

### The rhythm

The model implements, verifies locally and prepares every artifact. The
maintainer runs the long game jobs and reports results. One item is open at a
time; each candidate gates against the then-current accepted head, never
against a stale baseline and never against another unresolved candidate.

Commit after each finished and verified step. Keep tooling changes and engine
changes in separate commits.

### Durable development rules

1. A roadmap target is not evidence. Rebase the roadmap when experiments
   refute the expected accumulation.
2. Estimate expected value before an expensive tune or cluster: mechanism
   prior, plausible Elo, compute cost, gradient quality and confirmation cost.
3. SPSA optimizes constants around an architecture; it does not create the
   missing mechanism required for a 50–100 Elo jump.
4. A technically correct schedule and resume path are necessary but do not
   justify launching. Sunk preparation work is not a reason to spend more.
5. Categorical gates precede continuous tuning. Do not let a binary switch
   receive another coordinate's SPSA gradient. A pinned A/B knob is an
   unmeasured assumption, not a saved coordinate.
6. Node count, NPS and diagnostics explain a strength result; they do not
   replace paired games. A smaller tree can be weaker.
7. Final-PGO binaries decide material strength. Tune binaries and non-PGO
   probes can size or debug a mechanism only.
8. Do not add features from names or sibling engines. Require a local
   population, a unique signal, an interaction model and an acceptance gate.
   Reading a reference changes what you may *try*; it does not change what you
   must *measure*.
9. Correctness repairs may be retained without an Elo claim when the invariant
   is explicit and covered; record the exception rather than calling it free.
10. Release numbers describe shipped evidence, not the plan that once existed.
11. Individually plausible mechanisms do not compose. The closed Phase 4
    proved this locally: exhaustive measurement of its final bundle showed the
    accumulation did not survive. Gate dependency-complete clusters, not
    accumulated patches.
12. Interacting constants are fitted around current activations. A standalone
    repair to a mechanism the surrounding surface was tuned around usually
    loses. Repair it inside the cluster that owns it and refit jointly.

### Measurement gates

| Change class | Minimum evidence |
|---|---|
| Correctness | Independent invariant/regression test; strength gate when behavior changes materially, unless an explicit correctness exception is recorded |
| Behavior-neutral refactor | Exact benchmark fingerprint plus tests; measure pooled NPS for hot-path work |
| Search/eval strength | Clean revision-matched PGO A/B, paired UHO, registered SPRT and stop rule |
| Tune | Registered surface/schedule/estimator, completed final theta, fresh PGO bake, SPRT; LTC/4T when transfer is plausible |
| Platform optimization | Target-native interleaved A/B, identical-binary calibration and executable ISA verification |
| Release | Prior-release comparison, platform matrix, UCI/bench/ISA smoke tests, docs/version consistency |

Default gain gates use `3+0.03`, one thread, Hash 64 and paired
`tools/books/UHO_Lichess_4852_v1.epd`. Both verification passes matter: run
**debug and release**, because `--release` alone once missed a timeout bug
that then failed a hosted CI run.

`fastchess -use-affinity` with concurrency 14 is mandatory for 1T gates on this
host; unpinned Zen 3 runs carry a hidden per-run offset of roughly ±10 nElo.
Validate any harness change on a null pair — the same executable on both arms
— before trusting a verdict. `-use-affinity` pins one core per game and so
starves any run with `Threads>1`: **drop it for multi-thread runs and
re-calibrate the null pair under the multi-thread configuration**.

### Sizing a game budget

RAR-M10 fits this harness's LLR drift as
`drift/game ≈ 8.3e-6 × (Elo1 − Elo0) × (true_nElo − midpoint)`, so a `[3,10]`
nElo SPRT resolves in roughly:

| True effect (nElo) | Games to accept |
|---:|---:|
| 10 (exactly on H1) | ~14,500 |
| 12 | ~9,200 |
| 15 | ~6,000 |
| 20 | ~3,700 |
| 30 | ~2,200 |
| 40 | ~1,500 |
| 60 | ~950 |

Use this **prospectively only**, to choose a cap before games are seen. It is
never a reason to extend a run in progress. The practical consequence for
Phase 4 is important: a coherent cluster drawn from a 196-Elo population has a
much larger prior than 10 nElo, so it should resolve in a few thousand games.
Do not reflexively budget 16,000 games for a candidate with a 25 nElo prior,
and do not widen a bound after the fact to rescue a weak result.

Speed still converts: roughly **2 Elo per 1% NPS at `3+0.03`**, about three
times the older planning figure. Any NPS measurement must be validated on a
self pair first, pool several PGO builds per arm — two PGO builds of identical
source differ by about 0.36% — and run on an otherwise idle machine.

### Clean-code policy

Clippy at zero warnings across the workspace with all features. Documented
`unsafe` and documented `allow`s only. No new sentinel or positional-argument
patterns.

### Documentation audiences

| File | Audience / purpose |
|---|---|
| `README.md` | Users: install, CPU choice, UCI and build basics |
| `CHANGELOG.md` | Users: visible release deltas and measured claims |
| `RELEASE_NOTES_*.md` | Copy-ready GitHub release text |
| `PLAN.md` | Maintainers: current state, ownership and ordered roadmap |
| `GUIDE.md` | Maintainers/agents: tracker, commands and operating rules |
| `EXPERIMENTS.md` | Durable evidence, failures, retry triggers and artifacts |
| `analysis/phase4_counter_spec.md` | The 4.1/4.2 shared counter contract: names, definitions, sites and tiers |
| `analysis/phase4_mechanism_map.md` | The 4.3 map: per mechanism, the problem, Rarog's answer, the verdict and its owner |
| `tools/spsa_configs/README.md` | Tuning-specific mechanics and lessons |

`PLAN.md` and `GUIDE.md` are the maintainer-facing pair. The user-facing files
carry no project history, roadmap, Elo methodology, internal symbol names,
phase numbers or notes addressed to the maintainer. When facts disagree,
source, defaults and reproducible artifacts outrank prose; fix the prose in
the same change.

## 3. Released work through 2.3.2

### Phases 0–3 — closed

| Line | What it bought |
|---|---|
| 2.0.x | First stable baseline: board, movegen, UCI, search, TT, HCE, testing |
| 2.1.0 | Harness rebuild, search repairs and robustness |
| 2.2.0 | The evaluation programme; large staged self-play gain, smaller real gain |
| 2.3.0 | Correctness programme, search wave, reproducible builds, CI, shipped PGO |
| 2.3.1 | Windows ARM64 PGO patch |

### The closed Phase-4 line — accepted and shipping in 2.3.2

| Work | Evidence / policy |
|---|---|
| Broad selectivity fit | Accepted at +15.33 ± 7.34 nElo |
| Zero-reduction LMR floor | Accepted at +9.13 ± 5.45 nElo |
| Anchored Texel refresh | Accepted at +11.56 ± 5.19 Elo (RAR-E05) |
| NMP mate-score clamp | Permanent correctness repair; no strength claim |
| Typed TT evidence and provenance | Retained infrastructure, behavior-neutral at defaults |
| Root abort/fallback coverage | Retained correctness infrastructure |
| AArch64 TT prefetch | Accepted at +1.42% median NPS on M4, 12/12 paired wins |
| Executable ISA contract | Retained; caught accidental baseline POPCNT and enforces ARM prefetch |

The three strength results were sequential and used different estimators. They
are not additive and were never a promise of a particular release Elo gain.
Version 2.3.2 accurately describes what shipped.

**The closed line's item numbers 4.0–4.10 are retired.** They are not
referenced by this plan and must not be reused; `EXPERIMENTS.md` rows that
formerly pointed at them now point at their real future owner. The cancelled
Phase-4 SPSA config, fixed-option config, exclusion registry and registered
10,000-iteration launch were removed before any games were played. Do not run
`./tools/spsa.ps1 -ConfigGroup phase4 -LaunchOnly`; that surface no longer
exists.

### Removed in the 2.3.2 cleanup

These alternatives had no remaining evidence owner. Their accepted defaults
are hardwired, so removal changed neither strength nor the benchmark:

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

### Inert but deliberately retained — with owners

“Inert” means the shipped default reproduces the accepted engine. It does not
mean “forgotten.” Every retained alternative has exactly one later owner and
must be accepted or removed there.

| Owner | Retained work | Required disposition |
|---|---|---|
| 4.7 selectivity inputs | Continuation and capture correction weights; `CorrSkipWhenTtRefined` | Supersede or remove inside the history cluster |
| PARKED (was Cluster B) | TT provenance switches; typed evidence consumers | No owner: the eval/TT/qsearch cluster is parked inside the ~30 Elo residual. Remove at 4.9 unless a measurement revives it |
| 4.6 shallow-depth pruning | NMP and IIR provenance switches; `SelectivityProspectiveDepth` | `SelectivityProspectiveDepth` is settled in 4.6.2. The NMP and IIR switches are parked with their mechanisms |
| PARKED (was Cluster D) | Singular provenance switches; `SingularTtDepthMargin` | Extensions measured at a 1.2 sigma null. Remove at 4.9 unless a measurement revives them |
| PARKED (was Cluster E) | Aspiration shape; root-confidence aspiration and time inputs | Inside the ~30 Elo residual. Remove at 4.9 unless a measurement revives them |
| 7.3 post-NNUE fit | Anything no Phase-4 cluster actually reached | Re-measure after NNUE scale freezes; gate architecture first, remove losers |
| 8.0 multi-thread scaling | `RootConfPoolInstability`, `SmpIterationSkip` | Test only on representative 4T/8T+ topology; accept or remove |
| 8.1 runtime ISA dispatch | Universal baseline dispatcher | Consider only as a complete Stockfish-style dispatch architecture, not a dead per-tier startup guard |

A Phase-4 cluster may supersede an inert item only when its coherent
reference-led work reaches that exact consumer. It may not activate a dormant
switch opportunistically: a touched switch must be removed, kept inert with a
named owner, or separately gated. The post-NNUE owner remains intentional for
ordinary retuning, because NNUE changes score scale, correction residuals,
pruning margins, node cost and time allocation.

## 4. Phase 4 — Reference-accelerated search and HCE development (→ conditional 2.4.0)

### Objective

Build Rarog's own strongest search, and then its own strongest HCE, **faster
than blind discovery would allow**, by reading a strong public engine for
ideas. The deliverable is Rarog's design. It may end up unlike Stockfish's,
and where Rarog's answer is better, Rarog keeps its answer.

What the reference actually buys is the expensive part of engine development:
knowing **which problems are worth solving and in what order**. Rarog has
spent whole cycles discovering that a plausible mechanism does not pay. The
`9587eeeb` revision is a working existence proof that a particular set of
problems matters and can be solved together, so reading it replaces a long
sequence of blind, individually-gated guesses. That is the acceleration, and it
is the only thing being taken.

What it does **not** buy is a target to imitate. Rarog is an independent Rust
engine and stays one: its board, move generation, evaluator, transposition
table, UCI, build system, testing methodology and product identity are its
own. Similarity to Stockfish is not a goal, not a metric and not evidence
anywhere in this phase. Success is measured only as Rarog Elo against Rarog's
own accepted head.

The frozen `hybrid` branch is an executable **diagnostic instrument** — it
sizes targets and explains counters. It is never a thing to become.

This is a bounded exception to the previous "NNUE next" ordering. It exists
because the evidence changed, not because the roadmap wanted more pre-NNUE
work.

### Why this runs before NNUE

Search work is **evaluator-agnostic**. Every accepted search mechanism
survives NNUE intact, so this is the opposite of the constant-fitting that
rule 3 forbids — it is not work spent on a surface NNUE will replace. HCE work
pays forward twice: directly as strength now, and as a better teacher for
Phase 6.1 data generation.

The cost is real and stated plainly: this phase delays NNUE by its own
duration, and it can fail at any cluster. The stop rules below exist to make
that failure cheap.

### Reference evidence

`hybrid` at `75d0d43` builds Stockfish `9587eeeb` — the last pure-HCE master
commit before NNUE merged — with its board, search, move ordering, pruning,
TT, time management and UCI, calling the unchanged Rarog 2.3.2 HCE through a
checked Rust DLL ABI. The same executable with `Use Rarog HCE=false` is the
exact-revision Stockfish-HCE control, which removes compiler, revision and UCI
setup as confounders.

RAR-O02, the cleaner **no-adjudication** run (1,238 games, 982 ended in
natural checkmate, `3+0.03`, 1T, paired UHO, no forfeits):

| Contrast | What it isolates | Result |
|---|---|---|
| Hybrid − Rarog 2.3.2 | Stockfish search vs Rarog search, HCE held at Rarog's | **≈ +196.5** Elo |
| Stockfish-HCE − Hybrid | Stockfish HCE vs Rarog HCE, search held at Stockfish's | **≈ +328.6** Elo |
| Hybrid − Basilisk 1.9.3 | the same search advantage against the sibling engine | ≈ +196.5 Elo |
| Basilisk 1.9.3 − Rarog 2.3.2 | whole-engine sibling contrast | ≈ +30.4 Elo |

The hybrid achieved this at **1.5M NPS against Rarog's 2.4M**, so the search
result is not a throughput artifact and is if anything understated. It also
means Rarog can afford to spend some NPS on a better-coordinated search — but
must measure both Elo and NPS for every cluster.

Read these with discipline:

1. They are ordinary logistic point estimates from a deliberately stopped run,
   **not** paired-pentanomial SPRT results. They size a target and order the
   work. They are never a release claim and are never added to anything.
2. RAR-O01, the same experiment with evaluator-dependent adjudication,
   reported +270.9 where RAR-O02 without it reported +196.5. **Cross-evaluator
   cohorts must run with adjudication off**; the confounder is worth about 75
   Elo here.
3. The direction and order of magnitude are what transfer. No individual
   Stockfish mechanism has been credited with any Elo by this experiment.
4. The 196.5 figure is a *population*, not a forecast. What fraction of it
   survives reimplementation into a different engine is unknown, and measuring
   that fraction is exactly what this phase does.

### Where the Elo is — MEASURED, 2026-08-21

This section used to hold priors. It now holds measurements. The whole deficit
was decomposed in about four hours with matched cross-engine ablation
(`analysis/ablation_results.md`), and the priors it replaces were wrong in both
directions — they gave main selectivity 25–60 nElo when it is worth ~240 Elo,
and gave extensions 5–25 when it is worth zero.

**The deficit against the oracle is `G(0) = 250.77 ± 13.12 Elo`**, a direct
adjudicated 2,000-game measurement on the current head. It supersedes RAR-O02's
~196, which came from ~205 games per pair with adjudication off. Netting out
Rarog's 1.80x NPS advantage (~51 Elo) the true search-quality gap is **~302
Elo**.

| mechanism | mask | deficit explained | status |
|---|---:|---:|---|
| shallow-depth pruning | 32 | **124.6 ± 17.7** | → 4.6 |
| late move reductions | 128 | **116.0 ± 17.9** | → 4.5 |
| both together | 160 | **272.2 ± 17.9** | near-additive: interaction +31.6 ± 30.9 |
| extensions | 64 | 11.8 ± 18.8 | **null at 1.2 sigma — parked** |
| everything else | — | **~30 total** | residual after speed adjustment — parked |

**Two mechanisms hold 109% of the measured deficit, and they are separate
failures rather than one seen twice.** With both removed from both engines,
Rarog is +21.4 ± 12.2 ahead on the board — about 30 behind once its speed
advantage is netted out. Move ordering, TT, quiescence, extensions, null move,
futility, aspiration and time management are collectively ~30 Elo.

**HCE is the larger prize and is not yet decomposed.** Swapping Rarog's HCE for
the reference's under one search is worth **+328.6 Elo** (RAR-O02). That number
has never been broken down by term family, and 4.10–4.11 do to evaluation
exactly what was just done to search.

**RAR-S52–S54 predate the oracle and corroborate this from another direction.**
At equal nodes and equal speed, Rarog searched **2.5 plies deeper** than
Basilisk 1.9.1 and still lost by 65 Elo: it buys depth it cannot use by
discarding width it needs. That is the same finding as "Rarog's LMR is worth a
third of the reference's" — the reductions produce depth, but not depth that
holds up.

**Programme target: cumulative ≥ +100 Elo STC over 2.3.2**, unchanged as the
bar that justifies this phase's delay to NNUE — but it is no longer a hopeful
number. The search track alone now has ~240 Elo of measured, localised headroom
against the oracle, and the HCE track has a ~328 Elo population behind it.
Capturing even a third of the search half clears the target. What the phase
cannot claim in advance is transfer: a mechanism worth 116 Elo to the oracle is
not automatically worth 116 to Rarog, and only registered gates decide.

### Two tracks

**Search track (4.0–4.10).** Priority. HCE terms and weights are frozen
throughout. The end state is a mature single-thread search architecture that
also transfers at 4T; NUMA and high-thread scaling remain owned by Phase 8.

**HCE track (4.10–4.14).** Entered only after 4.9 freezes one search head, so
evaluation is measured against a settled search. Scope is **mature structural
coverage plus calibration**: every material contract in the last Stockfish HCE
must be classified, and every accepted representation carries its own local
Texel refit before the final whole-HCE convergence cycles.

Explicitly out of scope for the whole phase: undirected full-surface SPSA,
Stockfish-label weight distillation (RAR-E03 rejected it at −17.11 despite
4.9% lower holdout loss), copied terms or tuned constants, mixed search/eval
tuning in one candidate, and any NNUE integration work.

### Maturity audit — 2026-08-18

The reference roles are deliberately different. Local Stockfish `5062aee5` is
the current search reference. It is pure NNUE: there is no standalone HCE left
in its `evaluate.cpp`. HCE therefore uses `9587eeeb`, the last pure-HCE master
already frozen by the hybrid experiment. The oracle remains `9587eeeb`; the
newer search tree is an audit source, not a replacement tournament binary.

**Search verdict: not yet mature.** Rarog already has iterative deepening,
PVS, aspiration, typed TT evidence, NMP and verification, ProbCut, singular
extension, LMR, quiescence, Syzygy, a real clock and Lazy SMP. Its diagnostics,
abort handling and correctness coverage are production-quality. The remaining
gap is coordination, not a missing headline algorithm:

| Maturity contract | Audit disposition |
|---|---|
| Per-ply authority | Missing a coherent Rarog-owned context for previous reduction, statistical score, cutoff count and previous-PV following |
| History semantics | Main, capture, pawn, low-ply, continuation and correction histories exist; check/capture context, update attribution and ageing policy need an integrated owner |
| Move selection and LMR | Both exist, but stage guarantees, evidence flow, reductions and re-search authority are not one explicit contract |
| TT, raw eval and qsearch | Present, but raw/pruning/searched evidence, stand-pat, corrected eval and bound propagation need a full producer/consumer audit |
| Selectivity | Broad coverage exists; 4.7 is active. A second pass is required only after later history/depth contracts move, without altering 4.7 in progress |
| Extensions | Check, singular, double and negative extensions exist; TT-move reliability, multi-cut feedback and higher extension authority are incomplete |
| Root and clock | Strong Rarog-specific confidence model exists; completed-root, retry, extra-iteration and fallback authority still need one contract |

Current Stockfish constants, history seeds, qsearch blends, depth formulae and
time multipliers are **candidates, not maturity requirements**. Each underlying
problem must be measured on Rarog, and an intentionally different accepted
answer closes the contract. MultiPV and Skill Level are product features;
NUMA and high-thread history placement remain Phase 8 work.

**HCE verdict: not yet mature.** The parameter/trace macro, Texel
reconstruction, tune/load surface, caches, symmetry tests, KPK bitbase,
quadratic imbalance and several endgame handlers are mature infrastructure.
The evaluator itself has materially weaker coverage than the last Stockfish
HCE in the following families:

| Maturity contract | Verified gap |
|---|---|
| Score foundation | Lazy evaluation, rule-50/tempo ordering, score grain, space gating and winnability/complexity need explicit local dispositions |
| Pawns and passers | Weak-unopposed/lever semantics, blocked-passer support, edge-file effects, path safety and king-distance progression are partial or absent |
| Activity and threats | Mobility lacks pin-aware areas and selected x-rays; reachable outposts, queen/king-ring pressure and several threat relations are absent or inert |
| King safety | Shelter/storm is too low-dimensional, castling-destination shelter and pinned-defender danger are absent, and flank/safe-check inputs are incomplete or zero |
| Scaling and endgames | The dispatcher is sound but materially narrower, especially rook-and-pawn, bishop-and-pawn, KQKR and generic winnability scales |
| Calibration | Many structurally present terms are zero or were fitted around weaker representations; local and final whole-HCE refits are required |

Generic ideas absent from `9587eeeb` — chain length, triple-pawn penalties and
connected-passer bonuses, for example — are not smuggled into the programme as
reference requirements. They need separate local evidence.

**End-of-Phase-4 maturity bar.** A contract is complete only when it is
classified as equivalent, intentionally different with evidence, accepted
through its registered game gate, or rejected after a dependency-complete
implementation. There may be no unclassified or first-draft item in the search
or HCE maps. Every accepted HCE representation must be traceable, activation-
and covariance-audited, locally fitted, PGO/NPS checked and SPRT-confirmed.
This is an implementation-maturity target, not a promise to match Stockfish's
strength, constants, source shape or exact feature list.

### The independence boundary

Both engines are GPLv3, so copying would be *legally* permissible. This
boundary is therefore a **product and engineering decision, not a licence
constraint**, and it is deliberately stricter than the licence requires.

| May cross into Rarog | May not cross |
|---|---|
| The problem a mechanism solves, and why it matters | Source code, in any language, in any amount |
| That a problem exists at all — the thing that is expensive to discover | Line-by-line or structure-for-structure transcription |
| Which mechanisms interact, and the order they must be built in | Tuned constants and margins |
| Which populations are worth instrumenting and measuring | Identifier names, file layout or type shapes copied for their own sake |
| Known pitfalls and failure modes, so Rarog does not rediscover them | Behavioral equivalence as a goal or as an acceptance criterion |

Consequences, stated plainly so no later step can drift:

- **Read, understand, close the file, then design.** The implementation step
  starts from Rarog's own code and Rarog's own diagnostic evidence. If a
  change cannot be justified without pointing at the reference, it is not
  understood well enough to ship.
- **No Stockfish code is copied, so Rarog is not a derivative work of it.**
  `README.md` already carries the correct posture — an independent engine that
  benefits from the community's published ideas, with thanks to Stockfish for
  the inspiration. That acknowledgement is accurate and sufficient; do not
  restyle it as an attribution of derived code. If some future step genuinely
  needs actual upstream code, that is a separate maintainer decision with full
  GPL attribution, and it is explicitly **not** what this phase authorizes.
- The `hybrid` branch does vendor upstream source, correctly attributed with
  its `AUTHORS` and `Copying.txt`. It is a diagnostic artifact: never merged,
  never shipped, never a source to copy from.
- **Similarity is never a reason to accept anything.** A candidate whose trace
  looks more Stockfish-like and loses games is rejected, and a candidate that
  looks nothing like it and wins is accepted. Games decide, exactly as before.
- **A counter that diverges from the oracle is a question, not a defect.** 4.2
  uses divergence to choose where to look next. It never sets a target value,
  and closing a gap in a counter is not an outcome.
- Rarog solving a problem differently, or deciding a problem does not apply to
  Rarog, is a first-class result. Record it with its reason and move on.
- Existing accepted Rarog mechanisms are not sacred, but replacing one
  requires a registered game gate against the currently accepted head.

### Branch and checkpoint model

| Object | Rule |
|---|---|
| `master` / `f931722` | Immutable Rarog 2.3.2 baseline until a release gate passes |
| `hybrid` / `75d0d43` | Frozen oracle and reproduction source. No retrospective edits, and its tournament binary never changes |
| `hybrid-diag` | Separate branch for the instrumented oracle build (4.1). Never merged, never shipped |
| `dev` | Ordered integration branch holding only accepted behavior plus diagnostics |
| Step candidate | Starts from the latest accepted integration SHA; never from another unresolved candidate |

### Rust owners

Named now so every step has a concrete surface:

| Contract area | Owner |
|---|---|
| Search driver, `Searcher`, the `MovePicker` enum, LMR/pruning/extensions | `src/search.rs` (5,544 lines) |
| Scored move lists, bad-capture staging | `src/move_ordering.rs` |
| Transposition table, entry layout, provenance | `src/tt.rs` |
| Typed result evidence | `src/evidence.rs` |
| Diagnostic counters | `src/diag.rs` |
| Tunable constants and UCI tune surface | `src/params.rs`, `src/search_options.rs` |
| Board, `CheckInfo`, `gives_check_with`, SEE, repetition | `src/board/board.rs` |
| Move generation | `src/board/movegen.rs` |
| Evaluator | `src/eval.rs` |
| Clock | `src/time_manager.rs` |
| Threads, root publication, vote merge | `src/search_threads.rs` |

### Search track — ordered work

- **4.0 Evidence, baseline and oracle freeze — CLOSED 2026-08-12 (RAR-M12).** Record RAR-O01/RAR-O02, the
  baseline and oracle SHAs, binary SHA-256 hashes, benchmark fingerprint,
  tournament protocol and the exact independence boundary. Preserve the
  Stage-1 hybrid package so the observation can be reproduced. Reproduce 2.3.2
  clean from `master`: `cargo fmt --check`, workspace tests in debug and
  release, all-feature clippy, `bench 13` = 6,519,711 / 2.449, PGO build and
  ISA verify. Register the Phase-4 compute budget and stop rules before any
  code moves.

- **4.1 Instrumented oracle — CLOSED 2026-08-12 (`hybrid-diag` `de568b3`).** On `hybrid-diag`, add the 4.2 counter set to
  the Stockfish side, matched name for name. This is what makes the phase
  evidence-led rather than guess-led: without a counter-for-counter
  comparison, cluster selection is intuition. The instrumented build is a
  diagnostic artifact only; it never plays a rating game and never replaces
  the frozen `75d0d43` tournament binary.

- **4.2 Differential observation harness — CLOSED 2026-08-12 (RAR-S55).** Define a versioned fixed suite
  spanning UHO openings, quiet middlegames, tactics, checks, zugzwangs and
  endgames. At fixed depth and fixed nodes, one thread, emit deterministic
  counters for:
    - **TT producer/consumer kind** — which mechanism wrote an entry and which
      read it, kept distinct rather than pooled;
    - **prune recall and overlap** — not node savings. Rule 6 says a smaller
      tree can be worse, so measure best-move recall, contradiction, and which
      prunes fire redundantly on the same node;
    - **correction attribution** — which correction context claimed a node and
      whether the correction changed the decision;
    - **history update attribution** — main, capture, continuation, low-ply and
      pawn;
    - nodes and qnodes, move-picker source, fail-high move index, LMR
      population and re-searches, NMP, ProbCut, futility, razoring, extensions,
      aspiration retries and completed-root ownership.

  Diagnostics **off** must reproduce `bench 13` = 6,519,711 exactly.
  Diagnostics **on** must preserve best move and node counts with bounded
  overhead. Run the identical suite against the 4.1 oracle: **the counters that
  diverge most select the work.**

  **Shadow-evidence discipline.** A concern this step surfaces but does not own
  — stand-pat provenance, ProbCut, NMP/IIR/singular cooperation, checking-move
  LMR, root confidence — is recorded as shadow evidence, not acted on here. Its
  first owner is the cluster that reaches it (4.5–4.9); anything no cluster
  reaches falls through to 7.3. Recording it is mandatory; acting on it here is
  not permitted.

- **4.3 Mechanism map and order freeze — CLOSED 2026-08-12 (`analysis/phase4_mechanism_map.md`).** Read `search.cpp`, `movepick.cpp`
  and their per-ply state as a catalogue of **problems and one working set of
  answers**, and write down, per mechanism: the problem it solves, whether
  Rarog's own 4.2 evidence shows that problem is present here, what Rarog does
  about it today, and which other mechanisms it must move with. Classify each
  as **Rarog already solves this**, **problem present and Rarog's answer is
  weaker**, **problem does not apply to Rarog** (with reason), or **unknown,
  needs measurement**. The map's output is a Rarog work list, not a diff
  against Stockfish, and it must be usable without the reference open. Use the
  4.2 populations to choose and document the first cluster. If the evidence
  contradicts the provisional order below, **edit this plan before
  implementing** — never after seeing games.

- **4.4 Search-consumed board state — CLOSED 2026-08-12, nothing required.** Several mechanisms in the 4.5–4.9 list
  are only affordable with cheap per-ply state that Rarog recomputes on
  demand: `CheckInfo`, pins and blockers, check squares, `plies_from_null`,
  repetition distance. Land only the parts a 4.5–4.9 design actually consumes,
  as a cached per-ply structure. Gate on an exact benchmark fingerprint where
  the change is behavior-neutral, and on pooled-PGO NPS where it is a layout
  change. This step deliberately does **not** build the evaluator-facing
  dirty-piece delta contract; that stays owned by 5.1, which consumes the same
  per-ply structure. Do not let 4.4 grow into the NNUE runway.

- **4.5 Selectivity rework I — LMR. CLOSED 2026-08-21, NO GAIN.** Four
  candidates measured flat; see `analysis/ablation_results.md`. Rarog's LMR is
  contract-equivalent to the reference's — the base formulas agree to 2% — and
  it orders BETTER at every rank bucket, so it has 30% fewer late moves for a
  reduction to act on. **The 116 was marginal value inside each engine, not
  headroom**, and that inference was the most expensive error of the phase.
  Original scope kept below for the record.

- **4.5-ORIGINAL (superseded) — LATE MOVE REDUCTIONS.**
  The largest single measured component of the deficit.
  `analysis/ablation_results.md`: removing LMR from both engines closes
  **116.0 ± 17.9 Elo** of the gap, and Rarog's LMR is worth **33%** of what the
  oracle's is (−62.89 against −188.39 in self-play). Two independent methods
  agreeing to 0.8 sigma.
    - **4.5.1 Replace the reduction contract, do not tune it.** Rarog's
      `lmr_reduction_units` takes twelve loose arguments and could not see the
      root at all until RAR-S70. The reference's Step 16 is a formula plus seven
      annotated adjustments: history-based ~30 Elo, on-PV ~10, cut-node ~10,
      stat-score ~10, tt-capture ~5, opponent move count ~5, singularly-extended
      ttMove ~3. Implement the CONTRACT in Rust from the principle; the 0-for-5
      record on constant shifts says the constants were never the problem.
    - **4.5.2 Land `LmrMinReducedDepth`, already built and default-off.** The
      4.8.1 audit found **46.7%** of Rarog's applied reductions reduce the move
      to depth 0, i.e. answered by quiescence — a prune wearing a reduction's
      name, counted in no pruning family. At floor 1: `lmr_qs_clamped`
      686,179 → 0, qnodes −29.8%, nodes −18.4%. It belongs in this cluster, not
      in a gate of its own.
    - **4.5.3 Re-fit the root term against the new contract.** `LmrRootRelief`
      sits at 1536 from RAR-S70, fitted against the OLD interior contract. It
      has to move when the contract underneath it does.
    - **4.5.4 Use matched ablation as the progress meter, not agreement.** After
      each substep, `G(128)` should shrink toward `G(0)`. That is a direct
      readout of how much of the 116 has been captured, at ~20 minutes a
      measurement, and it needs no SPRT to be informative.

- **4.6 Selectivity rework II — SHALLOW-DEPTH PRUNING.** ⚠ **124.6 is marginal
  value, not a target.** Live item is 4.6.4, the quiet SEE prune — the one
  Step-13 gap the counters cannot explain as a population effect (`see_prune`
  0.20x, and `see_ge` short-circuits every non-capture, so it could not
  evaluate a quiet move at all). Every other gap here is measured BEFORE it is
  built.
  Marginally the larger of the two, and **separate**: the joint ablation
  measured interaction at only +31.6 ± 30.9, so these are two failures rather
  than one seen twice. Removing it from both engines closes
  **124.6 ± 17.7 Elo**.
    - **4.6.1 Rework the Step 13 families as one contract:** move-count pruning,
      countermove/history pruning (~20 Elo annotated), parent-node futility
      (~5), and the two SEE-based prunes (~20 and ~25). Rarog has all of them
      and captures a fraction of their value.
    - **4.6.2 Settle the prospective-depth question inside this cluster.**
      `SelectivityProspectiveDepth` exists because the pruning consumers and the
      reduction disagreed about the depth a move would actually be searched at.
      That is a selectivity contract, so it is fitted here and nowhere else.
    - **4.6.3 Audit the pruning shadow counters against the live block.** The
      `prune_shadow_*` family shadows the real decision, and an ablation anchor
      landed on the shadow instead of the live site during this work. Either
      make the shadow provably track the live block, or delete it.

- **4.7 Selectivity inputs — HISTORY AND ORDERING. Inside the selectivity budget.**
  Absorbs the old 4.5.3 and 4.9b. This is NOT a separate lever: histories are
  what 4.5 and 4.6 CONSUME. The reference's single largest LMR adjustment is
  history-based (~30 Elo) and its largest shallow-pruning term after move count
  is countermove history (~20). A history rework is worth doing only to the
  extent it improves the two clusters above, and is measured that way.
    - Semantics: bonus/malus symmetry, aging, continuation-table
      dimensionality, and which tables each consumer reads.
    - ⚠ Ordering quality is NOT separately measurable by matched ablation.
      Ordering was intact on both sides at mask 160, so its direct effect sits
      inside the ~30 Elo residual — but it also determines how SAFE the pruning
      is, and that part is inside the 240 Elo. Do not read the residual as
      "ordering does not matter".

- **4.8 ONE seeded selectivity SPSA, after the architecture is in.**
  PLAN rule 4 and RAR-S13 both apply: RAR-S13 ran an LMR-family SPSA before the
  architecture was settled and lost **7.78 ± 8.00**, because the tuner found a
  sibling-local optimum that beat its own siblings and then lost to the head.
    - **Seed from Rarog's own measurements, NOT from the reference's
      constants.** An earlier draft of this line said to read the constants out
      of `search.cpp`. That was wrong twice over: the independence boundary in
      `PROCESS.md` forbids tuned constants and margins from crossing, and the
      reference's thresholds are expressed in ITS history units, which are not
      Rarog's — so they would not transfer even if copying them were allowed.
      What crosses is the MECHANISM. Seed each 4.5.1 term from a zero-game
      sweep of its own parameter on the bench and the suite, and let SPSA
      refine from there.
    - **One run, one coordinate set: the selectivity surface only.** No HCE
      coordinates, no undirected full-surface tune, and no second run "to be
      sure".
    - **Entry condition:** 4.5 and 4.6 have landed their contracts, and `G(128)`
      and `G(32)` have both measurably shrunk. A flat or monotone zero-game
      sweep is evidence AGAINST spending the budget, per rule 4.

- **4.9 Search residual, integration and freeze.**
  Everything else in search is **~30 Elo collectively, measured** — see the
  parked table below for what that covers. This step does not chase it.
    - Integrate 4.5–4.8, re-verify the bench fingerprint, run the correctness
      suite in debug and release, and confirm no NPS regression: Rarog's
      **2.95 Mnps against the oracle's 1.64** is a real asset and must survive.
    - Re-measure `G(0)`. That single 20-minute run prices the whole search track
      against the 250.8 baseline, and it is the number the release claim rests
      on.
    - Freeze the search head as the immutable HCE baseline. Record source hash,
      binary hash, bench fingerprint and NPS.

### HCE track — ordered work, and it is the LARGER prize

Under the SAME search, swapping Rarog's HCE for the reference's is worth
**+328.6 Elo** (RAR-O02, adjudication off; +304.8 with adjudication in RAR-O01).
That is bigger than the entire search deficit. It rests on ~205 games per pair,
so treat it as "large" rather than as 328.6 exactly — and re-measure it properly
in 4.10, which now costs 20 minutes.

**The method changes, and this is the lesson the search track paid for.** The
old track went straight into Clusters F–I without ever measuring which
evaluation family carries the Elo. The search track made exactly that mistake
for months — five gates, ~39,500 games, 0 for 5, every one aimed at what is now
measured to be a ~30 Elo bucket — until matched ablation found the answer in
four hours. **Decompose first, build second.**

- **4.10 Reciprocal HCE oracle and matched-ablation harness.** Mirror of the
  search instrument, which is built and validated. Both evaluators run under ONE
  search — the frozen 4.9 head, or the oracle's — with an `EvalAblationMask`
  exposing one bit per term family on each side. Every bit proved live before
  use by a mechanical check that the term actually moves scores: a dead guard
  reads as a null, and that cost two false results in this phase. Re-measure the
  RAR-O02 gap under the frozen search. Register the budget and stop rules before
  touching evaluation code.

- **4.11 HCE decomposition — measure which families carry the 328.**
  Matched cross-engine at equal mask, exactly as `G(mask)` was used for search:
  material/PST, mobility, king safety, pawns and passers, threats, imbalance,
  scaling and winnability. One ~20-minute run per family. The output is the
  eval equivalent of "LMR 116, shallow pruning 124.6, extensions zero".
    - ⚠ **Ablate one family at a time until the readable band is known.** The
      mask-163 search run collapsed both arms to 4–8%, where the Elo scale
      amplifies 4.4x and the measurement means nothing. Keep the ablated arm
      inside roughly 20–80%.
    - ⚠ **A lower aggregate Texel loss cannot accept a candidate.** RAR-E03
      already disproved that proxy for this HCE, and the 4.6 answer harness was
      a second instance of the same mistake in the same phase.

- **4.12 Rework the families 4.11 ranks, in that order, as coherent clusters.**
  Content preserved from the old 4.13–4.16; the ORDER is now an output of 4.11
  rather than an assumption, and a family 4.11 prices near zero is not built.
    - **Score foundation, winnability, endgame dispatch** — material/PST
      ownership, phase interpolation, tempo, score grain and POV, rule-50
      damping, and the specialized dispatcher (queen-vs-rook,
      rook-and-pawn-vs-rook, rook-vs-pawn, bishop/pawn, KPKP). Preserve proven
      draw and mate semantics; tablebase invariants precede any refit.
    - **Pawns, passers and pawn-dependent scaling** — pawn-cache inputs and
      lifetime, weak-unopposed and lever semantics, doubled/backward/opposed
      classification, blocked-passer support, edge files, path safety,
      progression, king distances, and the scaling consumers that read them.
      Terms planted at zero are activation- and covariance-audited, not assumed
      disproven.
    - **Piece activity, threats and space** — pin-aware mobility areas,
      bishop/rook x-rays, reachable and bad outposts, bishop-pawn severity,
      trapped-rook geometry, king-ring and queen pressure,
      weak/restricted/hanging threats, material-gated space. Hot attack
      generation owes a pooled-PGO NPS gate.
    - **King safety and nonlinear imbalance** — replace the low-dimensional
      shelter/storm approximation with rank/file-sensitive, blocked/unblocked,
      castling-destination-aware inputs; rework attack units, safe and unsafe
      checks, weak squares, pinned defenders, flank camp/pressure, pawnless
      flanks and the mobility/score feedback. A coupled Rarog model, not
      independent bonus copying. Existing zero weak-ring, flank,
      missing-shelter, storm and shelter-storm inputs are unidentified, not
      disproven.

- **4.13 Texel consolidation of the upgraded HCE.** Only after the
  representations in 4.12 have frozen — fitting a representation that is about
  to change is the same error as tuning a reduction contract before replacing
  it. Anchored whole-HCE fit over activated, identifiable weights, with fixed
  train/validation/untouched splits. Bake and SPRT each completed cycle; repeat
  only while both held-out fit AND games improve; stop on the first failed or
  no-gain cycle rather than selecting a lucky checkpoint.
    - Then, and only then, audit the cp-valued SEARCH margins whose populations
      moved under the new evaluation. Any search-margin SPSA is a separate,
      narrow compatibility candidate with frozen HCE. **Never mix HCE and search
      coordinates in one tune.**

- **4.14 HCE checkpoint and ablation.** Compare the accepted HCE head with the
  4.9 baseline, revision-matched final-PGO, adjudication off. Ablate surprising
  contributors. Close every contract classification from 4.10. Record the exact
  search-versus-HCE attribution — with both tracks measured by the same
  instrument, that attribution is a number rather than an argument.

### Transfer, cleanup and release

- **4.15 Transfer, portability, SMP and release gate.** Retained in full: the
  new evidence says nothing against it, and SMP is a product requirement rather
  than a strength lever.
    - Compare the final head with 2.3.2 directly. Confirm direction at LTC
      `10+0.1` and at 4T, benchmark and pooled NPS, the platform and ISA matrix,
      UCI conformance and the correctness suite.
    - **NPS is a first-class release criterion now, not an afterthought.**
      Rarog's 1.80x speed over the oracle is worth roughly 51 Elo and is doing
      real work in every measurement above.
    - Remove diagnostic scaffolding with no future owner. The `ablate` feature
      and the `hybrid-ablate` branch **stay** — they are the instrument that
      produced this plan, and they are needed again in every later phase.
    - Harness caveat: drop `-use-affinity` for the 4T cells and re-calibrate the
      null pair under that configuration.
    - Final no-adjudication target cohort including Basilisk 1.9.3 and the
      oracle as the diagnostic reference point.

### Parked by measurement — preserved, not deleted

These were real ideas and several are well argued. They are parked because
matched ablation bounds **all of them together** at ~30 Elo, not because any one
was examined and found wrong. Any of them becomes live again the moment a
measurement puts Elo behind it.

| old item | content | why parked |
|---|---|---|
| 4.6 Cluster B | static-eval separation, TT admission/replacement/bounds, quiescence PV contract, quiet checks in qsearch | inside the ~30 Elo residual at mask 160 |
| 4.8 Cluster D | check, singular, double and negative extensions; IIR; excluded-move | **measured at 11.8 ± 18.8 Elo, a 1.2 sigma null** — the strongest parking evidence in this table |
| 4.9 Cluster E | aspiration retries, completed-root authority, PV and interrupted fallback, stability inputs, extra-iteration policy | inside the ~30 Elo residual; RAR-S70 already took the root's reduction term |
| 4.6.1–4.6.8 | answer-harness sub-steps: mate answers, cohort agreement, premature conviction, score volatility | the instrument was calibrated and **cannot rank candidates** — `analysis/answer_harness_calibration.md` |
| 4.10 second-pass selectivity | broad re-fit after integration | superseded by 4.8, which is narrower, seeded and single-run |
| razoring, IID | ~1 Elo each by the reference's own annotation | **unmeasurable at `3+0.03`** — 10 forfeits per 3,000 games is worth ~1 Elo, so noise equals signal |

**What is NOT parked and must not be lost:** `LmrMinReducedDepth` (built,
default-off, now 4.5.2), the reduction-contract audit inherited from 4.5.4 and
4.8.1 (now 4.5.1), history semantics (now 4.7), and the entire HCE track, which
gains scope rather than losing it.

### Old-to-new item map

| old | new | note |
|---|---|---|
| 4.5 Cluster A | 4.5, 4.7 | LMR half promoted to its own cluster; histories to 4.7 |
| 4.6 Cluster B | parked | ~30 Elo residual |
| 4.6c root-answer cluster | 4.5.3 | root term is a reduction-contract term |
| 4.7 Cluster C | 4.6 | main selectivity, now measured at 124.6 Elo |
| 4.8 Cluster D | parked | measured null |
| 4.8.1 reduction contract | 4.5.1 | promoted; it is the 116 Elo item |
| 4.9 Cluster E | parked | ~30 Elo residual |
| 4.9b history semantics | 4.7 | folded into the selectivity inputs |
| 4.10 integration/second pass | 4.8, 4.9 | SPSA split out and narrowed; integration kept |
| 4.11 HCE baseline | 4.10 | now also builds the eval ablation instrument |
| 4.12 evaluator harness | 4.10, 4.11 | decomposition step added ahead of any rework |
| 4.13–4.16 Clusters F–I | 4.12 | content preserved; order becomes an output of 4.11 |
| 4.17 HCE convergence | 4.13 | Texel consolidation plus the search-margin audit |
| 4.18 HCE checkpoint | 4.14 | unchanged in substance |
| 4.19 transfer/SMP/release | 4.15 | unchanged in substance; NPS promoted to a criterion |

### Release rule

1. **2.4.0** requires a cumulative STC point estimate of at least **+40 Elo**
   over 2.3.2 with the 95% lower bound above **+25 Elo**, plus positive LTC
   and 4T lower bounds, the final platform gates and no unresolved correctness
   regression. Search and HCE contributions must remain separately
   attributable even though the verdict is cumulative.
2. A cumulative result at or above the **+100 Elo** programme target, with a
   lower bound above +75, may justify a higher minor version. That is a
   maintainer decision, not an automatic consequence of the number.
3. Below the 2.4.0 bar, ship 2.3.x by explicit maintainer decision, or close
   the phase without a release.
4. If this phase releases 2.4.0, the baseline NNUE material release target
   becomes **2.5.0**. If it does not, Phase 6 retains the conditional 2.4.0
   target. In either case the next roadmap item after closure is 5.0.

### Cluster discipline and stop rules

These govern 4.5–4.14 and exist because the closed Phase 4 failed by
accumulating individually plausible search mechanisms that did not compose.

**The normal strength unit is one dependency-complete, locally fitted
cluster.** Do not SPRT every feature or internal substep: sparse and interacting
features are commonly worthless before their consumers and surrounding
constants move together. Do not implement the whole phase and rely on one final
tune either: that destroys attribution and allows losing representations to
hide inside a bundle. The required sequence is:

1. land behavior-neutral substrate under correctness, fingerprint and NPS
   gates, with no strength claim;
2. implement all interacting behavior required by the registered cluster;
3. freeze categorical choices, prove activation and fit that cluster's moved
   continuous surface with Texel or targeted SPSA where justified;
4. bake the completed candidate into clean PGO and run the cluster SPRT;
5. accept or revert before the next cluster, then run the later consolidation
   fit and its own SPRT only over already accepted structures.

A categorical subcandidate may receive its own preliminary SPRT only when it
is independently meaningful and does not require the rest of the cluster to
be fitted fairly. Such a probe does not replace the integrated cluster gate.

1. 4.2, 4.3 and 4.12 are observational and owe exact diagnostic-off
   fingerprint parity with their respective frozen baseline.
2. Each cluster starts from the last **accepted** integration head, has a
   pre-registered hypothesis, dependency map, baseline SHA, gate, cap and stop
   rule in `EXPERIMENTS.md` before any games, and ends accepted or reverted
   before the next cluster starts.
3. Implement the smallest dependency-complete cluster. Internal substeps may
   be compiled and diagnosed separately, but they are not expected to win
   standalone and never become the next strength baseline.
4. Fit after the cluster's representation and categorical choices freeze, but
   before its primary strength gate. HCE uses a local Texel family fit; search
   uses targeted SPSA only when activation, interaction and curvature justify
   the cost. Untuned first-draft weights are not a fair cluster test.
5. Counters and tuning loss explain a candidate; they cannot accept it. Only a
   registered final-PGO SPRT accepts. Borderline results are not accumulated as
   hidden debt.
6. Bounds follow the cluster's prior, chosen before games. A `[3,10]` nElo
   SPRT is used only when the cluster plausibly pays at least 10 nElo; the §2
   sizing table gives the cap.
7. Ablate a surprising integrated result before crediting a subcomponent.
8. **After two fully implemented search clusters fail to produce an accepted
   gain, stop and re-audit 4.2–4.3.** After two coherent HCE clusters fail,
   stop implementation and re-audit 4.12 and the remaining order. Track H may
   close early only by explicitly conceding the Phase-4 HCE maturity target;
   it may not silently leave UNKNOWN or first-draft contracts behind.
9. Record both Elo and NPS for every cluster. A richer contract that wins per
   node but loses enough depth is not an accepted implementation of it.
10. HCE-changing A/Bs and every cross-engine cohort default to **no
   adjudication**, because evaluator scale and semantics differ. Adjudication
   may be enabled only after a registered calibration demonstrates equivalent
   behavior for both arms; never reuse the search-only `strength-v2`
   assumption automatically.
11. Every changed HCE feature must reconstruct exactly through `EvalTrace`,
   report activation and covariance, and use train/validation/untouched
   separation. Texel proposes a fitted candidate; only its clean PGO SPRT can
   accept it.
12. SPSA is reserved for accepted architectures whose game-objective
   coordinates are not identifiable from the trace, or for search parameters.
   Freeze categorical choices, complete final theta and never select a
   checkpoint retrospectively.

## 5. Phase 5 — NNUE runway

Measurement and state rework. No direct strength; it unblocks Phase 6. Every
step is bench-identical or explicitly NPS-gated, and board surgery never mixes
with strength patches. 5.0 has no engine footprint and may be pulled forward
into Phase-4 SPRT downtime.

- **5.0 Frozen measurement corpus.** Freeze quiet, tactical, endgame, rule-50,
  phase-balanced and search-disagreement cohorts with deep external teacher
  cp/WDL labels — **not** Rarog-adjudicated, because that datagen loop is
  self-referential — plus Syzygy WDL/DTZ cohorts. Use by-game
  train/validation/ untouched-test separation, exact material-signature,
  phase, king-danger and passer cohort labels, and paired counterfactual
  positions per intended feature. Record teacher SHA and settings, label
  recipe, hashes and untouched split IDs. Per-candidate reports cover residual
  by cohort, full-versus-lazy deltas, raw-versus-corrected HCE, HCE versus
  qsearch and depth-N, and activation counts and covariance. This is
  diagnostic and experiment-selection material only; SPRT remains the verdict.
  It doubles as the Phase-6 stage-gate metric source and the Phase-9 selector.

- **5.1 Per-ply state and dirty pieces.** Consolidate keys, castling, EP,
  rule-50, `plies_from_null`, checkers and captured piece — today scattered
  across `Board` fields and unmake info — extending the per-ply structure 4.4
  introduced. Define the exact reversible state and dirty-piece semantics for
  quiets, captures, EP, promotions, castling and null. Randomized make/unmake
  compares board, keys, attacks and state against a full refresh every ply.

  Adopt the observer shape read from Reckless: a `BoardObserver` trait with
  three events — `on_piece_change(piece, sq, add)`, `on_piece_move(piece, from,
  to)` and `on_piece_mutate(old, new, sq)` — emitted at the exact mutation
  points, so castling fires rook-remove plus king-move plus rook-add and EP
  fires the `to^8` pawn-remove. `make_move<T: BoardObserver>` is generic, so the
  null observer used by perft, tests and datagen monomorphizes to zero code;
  verify with the perft comparison suite unchanged plus bench bit-identity.
  Two delta channels have different timing needs: a compact pre-make push
  (move, moving piece, captured) into a `MAX_PLY` stack entry feeds accumulators
  and is reconstructable later, while the during-make observer events feed
  threat features that need the board mid-transition and cannot be
  reconstructed post-hoc. Null moves emit nothing and do not push, so the top
  entry stays accurate; `pop()` is `index -= 1`, so undo is free.

- **5.2 Accumulator scaffolding.** Per-thread and per-ply ownership, refresh
  markers and debug full-recompute seams. The accumulator lives with the
  search worker, **not** inside the copyable `Board`. HCE keeps running
  through the evaluator boundary untouched and the search stays
  fingerprint-identical. No inference yet. Reserve the king-bucket
  refresh-cache slot but build it in 6.5, where the trainer defines the bucket
  layout.

- **5.3 Trainer preflight.** Pin the trainer, Bullet, toolchain and GPU;
  verify conversion, shuffle, deterministic splits and manifests, reference
  vectors and resume semantics. Malformed or lossy input must fail loudly.

- **5.4 Runway gate.** Exact benchmark, `cargo fmt`, workspace tests in debug
  and release, randomized unwind, reproducible pilot corpus and trainer
  conformance. Create an integration branch only after this passes.

- **5.5 Threat-map hooks (optional).** Reserve the dirty-threat interface so
  threat inputs can land in Phase 7 without another make/unmake rewrite.

**Boundary rule:** never let the search know how the evaluation works. If a
pruning condition needs evaluator internals explained, it is a boundary
violation.

## 6. Phase 6 — Baseline NNUE (target: justified 2.4.0 or 2.5.0)

A competitive NNUE is necessary, not sufficient, for top-level strength: data
quality and scale, incremental inference speed, search recalibration and
repeated self-play cycles matter as much as layer sizes. Keep the evaluator
call the only search↔eval boundary, so HCE remains a known-good fallback and
search never depends on evaluator internals.

**Training stack: `net_trainer` (`D:/code/net_trainer`)** — the existing
engine-agnostic, Bullet-based pipeline: datagen → extract → convert/shuffle →
GPU train → `quantised.bin`. Rarog's side of the work is implementing the
consumer contract in `docs/nnue_format.md`, not building a trainer. The v1
architecture is **chess768 → (H×2, perspective, SCReLU) → 1×8 material output
buckets** with `QA=255`, `QB=64`, `SCALE=400` and `H=1024` by default;
correctness is gated by integer-exact conformance vectors in `models/test/`,
with reference implementations in `examples/`. The documented upgrade path is
Bullet's progression: v1 (output buckets, no king buckets, accumulators never
refresh) → v2 mirrored king-bucket inputs → v3 multi-layer/pairwise-mul.
Training needs an NVIDIA GPU and CUDA; data tools run anywhere. Do not fork
the format: architecture changes go through `net_trainer` as trainer plus
format doc plus a new conformance net, together.

- **6.0 Trainer hardening:** strict CLI, train/validation/untouched-test
  splits, checkpoint selection, hashes, seeds and exact references.

- **6.1 Controlled data:** generate **30–60M unique** teacher positions,
  roughly 10–20 sampled positions per game. Use the built-in label recipe
  `target = (1−λ)·sigmoid(score/SCALE) + λ·result` and select λ on validation;
  the earlier pure-WDL win for *HCE* does not transfer automatically, since a
  higher-capacity student can use cp signal a linear eval could not. Seed from
  a diverse EPD book. Around the pipeline, Rarog's repo adds by-game and
  trajectory splits, dedup, the frozen 5.0 test set, and a dataset manifest
  recording source engine and net SHA, search budget, book, λ, RNG seed and
  trainer commit. Include on-policy positions, hard loss/fortress/conversion
  cohorts and tablebase-supervised endgames. Do not train primarily on
  positions adjudicated early by the same evaluator. Report once on the frozen
  test; SPRT decides.

- **6.2 Baseline networks:** documented widths and buckets with at least two
  seeds; validation chooses within a run, untouched cohorts are used once.

- **6.3 Scalar integration:** implement the format in Rarog starting from the
  reference Rust example. **The acceptance gate is the conformance vectors,
  integer-exact** — that replaces any custom header or versioning scheme,
  since the file is the raw `quantised.bin` and H is recoverable from file
  size. Embed the net file's hash in engine identification and manifests for
  provenance. Require strict embedded/EvalFile layout validation, malformed
  and truncated net rejection, exact Rust/NumPy/engine full-recompute
  agreement, and a clean HCE fallback.

- **6.4 Incremental and SIMD:** dirty deltas per ply and per thread;
  randomized incremental-versus-full parity across castling, EP, promotion and
  null; integer bound proof; portable, x86 and ARM64 kernels bit-exact and
  target-native PGO-smoked. Hard pooled-PGO NPS gate before games; optimize
  the update paths before adding capacity if the baseline is too slow.

- **6.5 Architecture loop:** vary data, label blend, width and buckets,
  activation, learning rate and duration **one axis at a time** with two
  seeds. Step to king-conditioned inputs (trainer v2, mirrored king buckets)
  as the minimum serious architecture; the engine cost is accumulator refresh
  on king-bucket and mirror changes with cached refresh tables, consuming
  5.2's reserved slot and 5.1's king-moved/bucket-changed flags. Compare
  bucket counts by quantized NPS, frozen-cohort residuals and SPRT against the
  accepted first net. Capacity must match data: grow `--hidden` and step v1→v2
  as the corpus grows, not before. Data-scale comparisons keep architecture
  and recipe fixed; architecture comparisons keep the data snapshot fixed. Do
  not declare the NNUE programme complete without testing king conditioning.

- **6.6 Gross search-scale safety:** adjust only clearly invalid margins or
  clock scale. The broad fit waits until 7.3.

- **6.7 Baseline release:** NNUE beats the accepted pre-NNUE master at STC and
  LTC, transfers at 4T, passes external checks and has zero
  incremental-versus-reference mismatch. Archive the 6.1 manifest and the
  trainer commit alongside each accepted `quantised.bin` so every net stays
  reproducibly trainable. Use **2.4.0 only if Phase 4 did not already use that
  version and this is a material release**; otherwise use 2.5.0 after a
  Phase-4 2.4.0, or remain on the current minor line.

Phase 9 is entered only if a king-conditioned net plus at least one inference
optimization and one meaningful data-scale retry fail to produce a viable net,
or if the programme later stalls well below target despite the Phase-7
frontier work. A weak first bring-up is not evidence that NNUE failed.

## 7. Phase 7 — NNUE frontier and final search fit

- **7.0 Residual and disagreement analysis:** by phase, material, king,
  tactical and endgame cohort, plus calibration, refresh cost and
  teacher-search disagreement.

- **7.1 Data frontier:** scale and deduplicate, natural finishes,
  hard-position mining and controlled label/depth A/Bs against untouched sets.
  Generate fresh on-policy data with each clearly stronger net, mix it with
  stable teacher and tablebase data, retrain, and stop only when both SPRT and
  frozen-cohort improvement flatten. If local compute is the limiter,
  distributed or donated training becomes part of this phase rather than a
  reason to retreat to HCE.

- **7.2 Architecture ladder:** king and perspective buckets, threat and
  material inputs, width and activation, and refresh-friendly variants; two
  seeds, exact conformance, NPS and SPRT. Residual-driven relation inputs —
  exact threat pairs first, then pawn-pair inputs for chains, levers and rams
  — belong here, each family being a full architecture revision (trainer,
  format doc and conformance net) rather than an engine-side patch. Memory
  bandwidth and update frequency are first-class costs. Do not add two
  families at once and do not hand-copy another engine's final shape; the
  trainer's own v3 multi-layer/pairwise-mul path competes for the same slot,
  so pick by measured residuals.

- **7.3 One post-NNUE search fit:** first resolve the retained categorical
  switches listed in §3 that no Phase-4 cluster reached; then register only
  the continuous coordinates whose optimum likely moved. cp margins do **not**
  transfer across evaluators, while structural search mechanisms do. Refit
  correction source weights, run one joint cp-margin fit over the applicable
  RFP/null/futility/ProbCut/LMR/lazy coordinates, and reconsider or remove HCE
  lazy evaluation. Coordinate count and horizon are derived from activation,
  curvature and compute budget — not a remembered target of 24 or 5,000. Bake
  final theta, PGO, SPRT, LTC and 4T, and ablate surprising winners.

- **7.4 Frontier gate:** direct comparison of 2.3.2, the Phase-4 head and the
  baseline NNUE, plus calibrated matches against contemporary target engines.
  This is where the Basilisk gap is re-measured and where a large-version
  claim is earned.

## 8. Phase 8 — Scaling, platforms and product completeness

- **8.0 High-thread and NUMA:** price the known depth-diversity deficit at 4T,
  8T and 16T; test the retained pool-instability and iteration-skipping
  switches, first-touch placement, TT and accumulator sharing, and false
  sharing. Keep the score/depth-weighted vote merge — it already outperforms
  deepest-thread selection. Measure helper TT write policy rather than
  intuiting it, and price helper root-move diversity and whole-tree ordering
  jitter at 2/4/8 threads. Remove losing switches.

- **8.1 Runtime dispatch and memory:** consider a baseline universal binary
  that selects specialized kernels, plus TT and network placement and large
  pages. Do not add a specialized-binary startup CPU guard. There is
  deliberately no startup CPU guard inside specialized assets: when the
  compiler is told that BMI2/AVX2/FMA are mandatory, ordinary
  feature-detection macros fold to true and the guard is removed. A working
  in-process guard would need baseline-compiled CPUID code executing before
  specialized code, which is a dispatch boundary, not a friendly check bolted
  into a binary already compiled for the newer ISA.

- **8.2 Product and platform:** demand-led Chess960 castling metadata and FRC
  regression coverage, or other platform work; preserve scalar/SIMD and clock
  parity. Also parked here: large-page and NUMA-aware TT, the shared-TT atomic
  packing revisit, AVX-512/VNNI kernels, the full match-manifest schema,
  stratified micro-bench workloads, and OpenBench-style distributed testing
  once typical accepted patches are +1–3 Elo.

- **8.3 Scaling release:** full topology, clock, net, ISA and user-doc gate.

**Parked — rule-50-bucketed TT search key.** Probe, store and prefetch with
`hash ^ RULE50_KEY[halfmove / 8]` while repetition keeps the raw key. Parked
because its prerequisites were SPRT-rejected, both draw-adjacent reworks lost
7–12 Elo, and the historical harness's early draw adjudication meant test
games almost never reached high clocks, so the benefit was invisible at the
gates while the de-tuning risk was not. **Re-entry triggers:** LTC-era primary
testing, an adjudication-policy change, or the 7.3 post-NNUE re-tune. Note
that Phase 4 runs its cross-evaluator cohorts with adjudication off, which
weakens the original objection — re-check it at 7.3.

## 9. Phase 9 — Optional post-NNUE classical fallback

Distinct from the bounded HCE track in Phase 4. Enter only if serious NNUE
contract, data and architecture retries fail and the maintainer explicitly
abandons NNUE. Everything here is representational work that a
king-conditioned, threat-aware NNUE subsumes, which is why it sits behind a
demonstrated NNUE attempt rather than before it.

Diagnose the failure first, use Phase-4 track-H evidence and 5.0 cohort
residuals to choose the order, run one fitted and gated wave, and preserve all
NNUE artifacts for a later return. Selection discipline: zero or sign-flipped
fitted weights trigger activation and covariance analysis first, never a
direct chess conclusion. Every item is a structure-plus-refit bundle with one
gate. **Any item that Phase 4's HCE track already landed is closed here, not
retried.**

- **9.0 King-safety semantic rework** — the largest remaining classical
  family. Activation instrumentation by queen presence and phase, legal versus
  geometric safe checks, blocked/unblocked/lever-supported storms,
  current-versus-reachable castling shelter, defender overload and pin inputs
  only where cheap, joint danger-input fits. Closed if 4.12's king-safety
  cluster landed it.
- **9.1 Winnability and material-specific scaling** — replace the sign-only
  initiative term, which can only push a nonzero endgame score away from zero.
  Residual tables by exact material signature first, Syzygy WDL/DTZ as direct
  evidence, sign-preserving non-amplifying scalers only, drawn/won/cursed
  cohorts validated separately.
- **9.2 Passer and pawn conditionality** — blocker ownership and type,
  rear-line openness for both rook-behind terms, connected-passer semantics,
  candidate-passer exchange conditioning, and a short-horizon race diagnostic
  instead of more static path terms.
- **9.3 Threat conditionality** — SEE-safe pawn pushes rather than the current
  "no enemy pawn attacks the push square" test, restricted mobility counted
  per affected piece rather than board-global, and pin/overload relations only
  where cheaply available. NPS-check first; threat recomputation is hot-path.
  Do not hand-write a threat net one scalar at a time.
- **9.4 Broad positional repairs** — queen infiltration on the full enemy
  attack map, bad-bishop blocked and central-pawn conditioning, space
  usability (all three space weights fit to zero, so the representation is the
  problem, not the scale), and conditioned rook-on-seventh.
- **9.5 Material and phase specialization** — bucketed coefficients,
  king-bucketed PSTs, queen-presence gates. Worst time-to-Elo on the list,
  because it hand-builds what the NNUE output buckets already provide. Only if
  NNUE is abandoned outright.
- **9.6 Lazy-margin conditioning** — only if dual-eval data shows a material
  sign-flip cohort; margin by non-pawn material and king danger.
- **9.7 OCB material-scope refinement** — the opposite-coloured-bishop scaler
  currently fires with queens, rooks and knights present. Add a small material
  hierarchy — strong scaling for pure OCB, milder with extra minors, near none
  with majors — with non-amplification, sign, pure-OCB, plus-minor and
  plus-major tests. Cheap and high-confidence, so it is the natural first item
  if this phase is ever entered.

## 10. Release checklist

Run for every release, in order:

- [ ] Version set; README, CHANGELOG and release notes updated for users only.
- [ ] `cargo fmt --check` clean.
- [ ] `cargo test --workspace --all-targets` in **debug and release**.
- [ ] `cargo clippy --workspace --all-targets --all-features -- -D warnings`.
- [ ] Feature builds pass; `--features tune` advertises the expected option
      set and no removed option.
- [ ] Benchmark fingerprint recorded, and explained if it moved.
- [ ] Local PGO asset passes UCI, benchmark and ISA verification.
- [ ] Prior-release comparison at STC and LTC, plus 4T direction.
- [ ] Hosted release and CI matrix pass on the release commit.
- [ ] Commit locally. Tag, push and publish only on maintainer instruction.

### 2.3.2 — released

- [x] Version set to 2.3.2; user docs updated.
- [x] Cancelled SPSA launch surface removed; future-owned mechanisms
      documented.
- [x] Abandoned options removed with accepted defaults hardwired.
- [x] `cargo fmt --check`, tests, clippy and feature builds pass.
- [x] Post-cleanup benchmark equals 6,519,711 / 2.449.
- [x] Tune UCI exposes no removed option and retained inert options remain.
- [x] Local PEXT PGO asset passes UCI, benchmark and ISA checks.
- [x] Hosted release/CI matrix passed on the release commit.

## 11. Reference and common commands

| Tool / path | Purpose |
|---|---|
| `tools/sprt.ps1 -EngineA <exe> -EngineB <exe> -NameA -NameB -Elo1 3 [-TC "10+0.1"]` | SPRT via fastchess; default `3+0.03`, hash 64, 1T, concurrency 14, explicit affinity, UHO book |
| `tools/spsa.ps1 -ConfigGroup <g> -EngineSuffix <s> [-Iterations N] [-Resume]` | weather-factory SPSA; groups in `tools/spsa_configs/` |
| `tools/audit_spsa_coverage.ps1` | Verify a tune surface against the registered option set |
| `tools/build_test.ps1 -Suffix <s> [-Tune\|-Native]` | Test binaries into `tools/test_engines/` |
| `cargo xtask build --arch pext\|avx2\|arm64\|native --pgo` | Release and deploy builds; PGO trains on `bench` |
| `cargo xtask verify-isa --arch <a>` | Disassemble the asset and enforce its ISA promise |
| `tools/books/UHO_Lichess_4852_v1.epd` | SPRT/SPSA/gauntlet opening book; unbalanced human openings, played from both colours per pair |
| `wac [depth]` (engine command, like `bench`) | WAC-300 tactical suite; deterministic solved count |
| `hybrid/build.ps1` | Rebuild the Stage-1 oracle package (`hybrid` branch only) |
| `D:/code/net_trainer` | Phase-6 NNUE training stack: datagen → extract → convert/shuffle → train → `quantised.bin` |
| `D:/code/net_trainer/docs/nnue_format.md` + `models/test/` | Net consumer contract plus integer-exact conformance vectors (the 6.3 acceptance gate) |
| `D:/code/basilisk` | Sibling C++ engine; its Phase 5 runs the same reference-accelerated programme independently |

```powershell
cargo fmt --check
cargo test --workspace --all-targets
cargo test --workspace --all-targets --release
cargo clippy --workspace --all-targets --all-features -- -D warnings
"bench" | ./target/release/rarog.exe
cargo xtask build --arch pext --pgo
cargo xtask verify-isa --arch pext
```
