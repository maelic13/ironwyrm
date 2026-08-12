# Rarog development plan

Updated 2026-08-12. This is the current roadmap. Detailed historical evidence
lives in `EXPERIMENTS.md`; the operational tracker lives in `GUIDE.md`.

## 1. Current state

| Item | State |
|---|---|
| Released baseline | **2.3.2** at `f931722` on `master` |
| Search fingerprint | `bench 13` = **6,519,711 nodes**, geomean EBF **2.449**, 1T |
| Integration branch | `dev`, reset to `master` and carrying this plan |
| Frozen oracle | `hybrid` at `75d0d43` — Stockfish `9587eeeb` driving the exact 2.3.2 HCE |
| Active game jobs | None. The stopped no-adjudication hybrid tournament already settled the architectural decision |
| Current phase | **Phase 4 — reference-accelerated search and HCE development**; 4.0 closed, 4.1 open |
| Next release | **2.4.0 at 4.19** if the work transfers; a larger cumulative gain may justify a higher minor version. Baseline NNUE then targets **2.5.0** |
| Reference posture | Stockfish `9587eeeb` is read for **ideas**. No Stockfish code enters Rarog. Rarog is not a derivative work and does not aim at behavioral similarity |
| HCE status | Frozen through 4.10. Structural, reference-led HCE work reopens at 4.11 under its own gates. No broad constant refit at any point in this phase |

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
moves to the 4.19 release gate and, afterward, the post-NNUE frontier gate at
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
| 4.5 Cluster A | Continuation and capture correction weights; `CorrSkipWhenTtRefined` | Supersede or remove inside the ordering/history cluster |
| 4.6 Cluster B | TT provenance switches; typed evidence consumers | Supersede or remove inside the eval/TT/qsearch cluster |
| 4.7 Cluster C | NMP and IIR provenance switches; `SelectivityProspectiveDepth` | Supersede or remove inside the selectivity cluster |
| 4.8 Cluster D | Singular provenance switches; `SingularTtDepthMargin` | Supersede or remove inside the extensions cluster |
| 4.9 Cluster E | Aspiration shape; root-confidence aspiration and time inputs | Supersede or remove inside the root/clock cluster |
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

### Where the Elo is expected to come from

Priors, for budget sizing and stop-rule design only. They are **not additive**
and no cluster is entitled to its prior.

| Step | Cluster | Prior (STC nElo) | Reasoning |
|---|---|---:|---|
| 4.7 | C — main selectivity | 25–60 | **The only cluster with a positive local result already in hand.** RAR-S54: a blind, untuned, uniform 15% de-selectivity shift beat the fitted values at +4.06 ± 3.71 over 14,196 games while spending +23.2% nodes. RAR-S53 reaches the same place independently. A structural rework with its own refit should beat a blind scalar |
| 4.5 | A — ordering, histories, LMR | 15–45 | Largest coordinated contract and feeds every downstream consumer, but RAR-S52 puts the first-move cutoff rate at 87.65%, only marginally under the ~90% healthy band. The prior therefore rests on the history, reduction and re-search contracts — not on raw ordering quality. Rarog's `MovePicker` is an enum inside `search.rs` with no staged guarantee contract |
| 4.6 | B — static eval, TT, qsearch | 5–25 | Raw/pruning/searched evidence separation is already partly present |
| 4.8 | D — extensions and depth authority | 5–25 | Rarog's check extension already gained +30.75, so co-adaptation risk is high and headroom is narrower |
| 4.9 | E — root search and clock | 5–20 | Root confidence and TM are locally tuned, and RAR-S53 priced speed plus time management together at a ~0 point estimate, so this cluster is about root authority rather than about the clock |
| 4.13–4.16 | F–I — HCE structural clusters | 15–50 each | Drawn from the 328-Elo evaluator population, discounted hard for co-adaptation with a search that is not Stockfish's |

The rows are ordered by prior, not by execution order. Execution still follows
the dependency order in the step list, because selectivity consumes the depth
and history evidence that cluster A owns. If 4.2–4.3 contradict that, reorder
here before implementing.

**RAR-S52–S54 predate the oracle and reach the same conclusion from a
different direction.** At exactly equal nodes and equal speed, Rarog searched
**2.5 plies deeper** than Basilisk 1.9.1 and still lost by 65 Elo: it buys
depth it cannot use by discarding width it needs. That gives this phase a
concrete, falsifiable progress metric that costs nothing to run — see 4.10.

**Programme target: cumulative ≥ +100 Elo STC over 2.3.2.** That is what makes
this phase worth its delay to NNUE. The release bar below is lower, because a
smaller confirmed gain still deserves to ship rather than be discarded; but if
the phase looks like it will land near the release bar rather than near the
target, the 4.10 expected-value review is where it closes.

### Two tracks

**Search track (4.0–4.10).** Priority. HCE terms and weights are frozen
throughout. Single-thread search is the owner; time management and SMP are
transfer checks, not places to hide a weak 1T result.

**HCE track (4.11–4.18).** Entered only after 4.10 freezes one search head, so
evaluation is measured against a settled search. Scope is **structural
coverage**: problems Rarog does not address at all, or addresses in a
materially weaker form, each fix carrying its own local refit.

Explicitly out of scope for the whole phase: another broad Texel or SPSA
constant fit over the existing feature set, Stockfish-label weight
distillation (RAR-E03 rejected it at −17.11 despite 4.9% lower holdout loss),
copying a term list, mixed search/eval tuning, and any NNUE integration work.

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

- **4.1 Instrumented oracle.** On `hybrid-diag`, add the 4.2 counter set to
  the Stockfish side, matched name for name. This is what makes the phase
  evidence-led rather than guess-led: without a counter-for-counter
  comparison, cluster selection is intuition. The instrumented build is a
  diagnostic artifact only; it never plays a rating game and never replaces
  the frozen `75d0d43` tournament binary.

- **4.2 Differential observation harness.** Define a versioned fixed suite
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

- **4.3 Mechanism map and order freeze.** Read `search.cpp`, `movepick.cpp`
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

- **4.4 Search-consumed board state.** Several mechanisms in the 4.5–4.9 list
  are only affordable with cheap per-ply state that Rarog recomputes on
  demand: `CheckInfo`, pins and blockers, check squares, `plies_from_null`,
  repetition distance. Land only the parts a 4.5–4.9 design actually consumes,
  as a cached per-ply structure. Gate on an exact benchmark fingerprint where
  the change is behavior-neutral, and on pooled-PGO NPS where it is a layout
  change. This step deliberately does **not** build the evaluator-facing
  dirty-piece delta contract; that stays owned by 5.1, which consumes the same
  per-ply structure. Do not let 4.4 grow into the NNUE runway.

- **4.5 Cluster A — move ordering, histories, LMR.** Implement in dependency
  order, as one coherent cluster:
    - **4.5.1 Move-picker contract:** TT move, good captures, killers and
      counters, quiets, deferred bad captures; legality and duplicate
      guarantees. Rarog's picker is an enum inside `search.rs` with no staged
      guarantee contract, so this substep is structural, not cosmetic.
    - **4.5.2 Evidence ownership:** main, capture, continuation, low-ply and
      pawn history indexing, normalization, aging and cutoff attribution.
    - **4.5.3 Reduction and re-search contract:** LMR population, improving/PV/
      cut-node adjustments, history feedback, the accepted zero-reduction floor
      and full-depth verification.
    - **4.5.4 Integrated gate and ablation:** final-PGO SPRT of the coherent
      cluster, then ablate surprising contributors. Reject and revert the whole
      cluster if no supported subset clears its registered gate.

  This cluster owns the retained continuation and capture correction weights
  and `CorrSkipWhenTtRefined`. Rule 12 applies directly: do not repair a member
  of this cluster standalone.

- **4.6 Cluster B — static eval, TT and quiescence.** Separate raw evaluation,
  pruning evaluation and searched bounds; align TT capabilities and entry
  semantics, qsearch stand-pat, capture ordering and check handling, and
  correction attribution. Preserve Rarog's proven draw and mate-distance
  semantics — they are correctness assets, and nothing here targets them. Owns
  the TT provenance switches and typed evidence consumers.

- **4.7 Cluster C — main selectivity.** In observed dependency order, rework
  razoring, reverse futility, null-move verification, ProbCut, move-count and
  history pruning, and quiet/capture futility. Use prospective searched depth
  consistently. Gate categorical architecture before any narrow constant fit;
  do not launch a broad SPSA. The 2.3.2 broad selectivity fit (+15.33 nElo)
  was a constant fit around the current architecture and does not pre-empt
  this. Owns the NMP and IIR provenance switches and
  `SelectivityProspectiveDepth`. RAR-S54 licenses a structural rework here
  with its own refit; it does **not** license shipping the uniform 15% scalar
  that produced the evidence.

- **4.8 Cluster D — extensions and depth authority.** Rework check, singular,
  double and negative extension and IIR semantics against TT provenance and
  LMR. Preserve mate and abort correctness, including the accepted NMP
  unproven-mate clamp. Gate the integrated contract, never the individual
  extensions — RAR-X02 showed check-extension removal cost Basilisk −10.17
  while Rarog's extension had gained +30.75, which is co-adaptation, not a
  portable verdict. Owns the singular provenance switches and
  `SingularTtDepthMargin`.

- **4.9 Cluster E — root search and clock handoff.** Rework aspiration
  retries, completed-root authority, PV and fallback ownership, and stability
  inputs. **Total time allocation must not move until the root evidence is
  coherent**; then gate any real-clock change separately. Owns the aspiration
  shape and the root-confidence aspiration and time inputs.

- **4.10 Search-only cumulative checkpoint, freeze and EV review.** Build the
  accepted search head and 2.3.2 through the same pinned final-PGO path. Run a
  direct fixed-protocol comparison at 1T STC, explain it with the frozen 4.2
  diagnostics, close every open search candidate and freeze the resulting
  head. Then perform a new expected-value review before entering 4.11:
    - if the search track banked a material gain and the remaining HCE
      programme still has a plausible path to the phase target, continue;
    - if it banked little and the HCE population is the only remaining
      argument, decide explicitly whether the HCE track is worth its cost
      rather than continuing by momentum;
    - if neither track can plausibly reach the release bar, close the phase and
      resume Phase 5 from `master`.

  The frozen head may equal 2.3.2 if no search cluster won. That is an accepted
  outcome and is recorded as such.

  Also re-run the RAR-S53 fixed-node depth readout here, because it is free and
  falsifiable: at `-Nodes 250000`, mean depth should have **fallen** toward ~14
  while Elo **rose**. An accepted head that still carries the +2.5-ply lead over
  Basilisk 1.9.1 has not fixed the over-selectivity, whatever its gate said, and
  that contradiction must be explained before the head is frozen.

### HCE track — ordered work

Track H learns from the reference evaluator the same way track S learns from
its search: map behavioral contracts and interacting consumers, implement them
in native Rust, measure them on Rarog, and reject what does not transfer.

- **4.11 HCE baseline and reciprocal-oracle freeze.** Make the 4.10 search
  head the immutable HCE baseline; record source and binary hashes, benchmark
  and NPS, and a no-adjudication reproduction slice of Stockfish-HCE versus
  Rarog-HCE under the frozen oracle search. Register the HCE budget and stop
  rules before changing evaluation code.

- **4.12 Differential evaluator harness and contract map.** On a versioned,
  legal corpus containing quiet, tactical, king-attack, pawn-structure,
  endgame, rule-50 and search-disagreement positions, record Stockfish and
  Rarog raw white-POV and side-to-move scores, phase, term breakdown and cost.
  Map `evaluate.cpp`, the pawn/material/endgame helpers and their search
  consumers onto owners in `src/eval.rs`. Measure scale, tempo, tapering,
  volatility, sign and bound disagreements, and search-conditioned activation.
  Diagnostics off must reproduce the 4.11 baseline fingerprint exactly. A
  lower aggregate teacher-fit loss cannot accept a candidate; RAR-E03 already
  disproved that proxy for this HCE.

- **4.13 Cluster F — score foundation and endgame dispatch.** Rework material
  and PST ownership, phase interpolation, tempo, score grain and units,
  evaluation POV, rule-50 scaling and specialized-endgame dispatch. Gate
  structural choices before narrow constants; preserve proven draw and mate
  semantics.

- **4.14 Cluster G — pawns, passers and endgame scaling.** Rework pawn-cache
  inputs and lifetime, structural pawn classes, passed-pawn progression and
  king/pawn distances together with the endgame scaling consumers that read
  them. Gate the coherent cluster including NPS; do not retain a prettier
  static fit that loses games.

- **4.15 Cluster H — piece activity, threats and space.** Rework mobility-area
  semantics, outposts, files, weak and restricted pieces, hanging threats and
  space as one interacting group. Use held-out term ablations to detect
  duplicate signals, then run the registered final-PGO game gate.

- **4.16 Cluster I — king safety and imbalance.** Rework shelter and storm,
  attack units, safe checks, weak squares and blockers, and nonlinear material
  imbalance, together with the activity and threat inputs they consume. Treat
  this as a coupled model, not independent bonus copying; gate it as one
  coherent candidate. Rarog's own king-safety inputs for weak ring, flank,
  missing shelter, storm and shelter-storm currently fit to zero — zeroed
  inputs are unidentified, not disproven, so this cluster is where the
  representation is repaired before any weight is trusted.

- **4.17 Search compatibility, cost and narrow calibration.** Measure lazy
  evaluation and cache behavior, parent–child score stability, pruning-bound
  populations, NPS and endgame pathologies on the frozen search. Any search
  margin change is a separately registered compatibility gate. Tune only a
  small set of activated, identifiable constants whose structural owner has
  already passed. No broad HCE SPSA and no mixed search/eval tune.

- **4.18 Cumulative HCE checkpoint and ablation.** Compare the accepted HCE
  head directly with the 4.10 baseline using revision-matched final-PGO builds
  and adjudication off. Ablate surprising contributors, remove rejected or
  dormant HCE alternatives with no later owner, and record the exact
  search-versus-HCE attribution.

### Transfer, cleanup and release

- **4.19 Transfer, portability, SMP and release gate.** Compare the final
  accepted head directly with 2.3.2. Confirm direction at LTC `10+0.1` and at
  4T, benchmark and pooled NPS, the platform and ISA matrix, UCI conformance
  and the correctness suite. Remove diagnostic scaffolding that has no future
  owner and resolve obsolete dormant switches. Run a final no-adjudication
  target cohort including Basilisk 1.9.3 and the 4.1 oracle as the diagnostic
  reference point. Harness caveat: drop `-use-affinity` for the 4T cells and
  re-calibrate the null pair under that configuration.

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

These govern 4.5–4.18 and exist because the closed Phase 4 failed by
accumulating individually plausible search mechanisms that did not compose.

1. 4.2, 4.3 and 4.12 are observational and owe exact diagnostic-off
   fingerprint parity with their respective frozen baseline.
2. Each cluster starts from the last **accepted** integration head, has a
   pre-registered hypothesis, dependency map, baseline SHA, gate, cap and stop
   rule in `EXPERIMENTS.md` before any games, and ends accepted or reverted
   before the next cluster starts.
3. Implement the smallest dependency-complete change. Substeps may be compiled
   and diagnosed separately, but an incomplete cluster never becomes the next
   strength baseline.
4. Counters explain a candidate; they cannot accept it. Only a registered
   final-PGO SPRT accepts. Borderline results are not accumulated as hidden
   debt.
5. Bounds follow the cluster's prior, chosen before games. A `[3,10]` nElo
   SPRT is used only when the cluster plausibly pays at least 10 nElo; the §2
   sizing table gives the cap.
6. Ablate a surprising integrated result before crediting a subcomponent.
7. **After two fully implemented search clusters fail to produce an accepted
   gain, stop and re-audit 4.2–4.3.** Track H has its own stop: after two
   coherent HCE clusters fail, close it and proceed to 4.19 or Phase 5.
8. Record both Elo and NPS for every cluster. A richer contract that wins per
   node but loses enough depth is not an accepted implementation of it.
9. HCE-changing A/Bs and every cross-engine cohort default to **no
   adjudication**, because evaluator scale and semantics differ. Adjudication
   may be enabled only after a registered calibration demonstrates equivalent
   behavior for both arms; never reuse the search-only `strength-v1`
   assumption automatically.

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
  only where cheap, joint danger-input fits. Closed if 4.16 landed it.
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
