# Rarog development plan

This is the forward roadmap. It says what will be done, in what order, why,
and what decides each step. It does not record history: completed work lives
in [HISTORY.md](HISTORY.md), measured evidence in
[EXPERIMENTS.md](EXPERIMENTS.md), procedures in [PROCESS.md](PROCESS.md), and
the day-to-day status board with checkboxes in [GUIDE.md](GUIDE.md). The
pre-rewrite roadmap is archived verbatim at
[docs/archive/PLAN-phase4-2026-09-09.md](docs/archive/PLAN-phase4-2026-09-09.md);
every historical `4.x` reference in the ledger and analyses points there.

Rewritten 2026-09-09 as a battle plan with one objective and a measured
starting point. Phases are lettered (A–G) so that no new identifier collides
with any retired number cited in the ledger, the analyses or source comments.

## 1. Objective and gates

**Objective.** Make Rarog the strongest engine we can build, in two stages:

1. **Classical stage.** With the hand-crafted evaluation, beat the strongest
   HCE-era engines in the maintainer's own pool: **Critter 1.6a, Houdini 3,
   Rybka 4 and Fritz 16**.
2. **NNUE stage.** Train networks on Rarog's own data only, then reach the
   **CCRL top 100**, and later the top 50.

**The classical target gate (E.2).** Colosseum rating tournament, the
maintainer's fixed pool with Houdini 3 added, `3+0.03`, UHO book, no
adjudication, at least 400 games per pair, measured **both at 1 thread and at
4 threads**. The gate is met when Rarog's head-to-head score against each of
the four named engines is at or above 50% at 1T and at 4T, with the 95%
interval of the pooled four-engine score excluding a loss. Ratings inside that
pool are relative; CCRL numbers do not transfer to this control (see
`analysis/endgame_occurrence_tournament_2026-09-05.md`).

**The NNUE target gate (F.10).** A CCRL 40/15 or Blitz list rating inside the
top 100, established by CCRL's own testing after a public release.

### Where we start (measured, 2026-09-04 to 2026-09-09)

| Fact | Value | Source |
|---|---|---|
| Head-to-head at `3+0.03`, 400 games each, Rarog 2.4.0-dev | Houdini 1.5a **−216**, Critter 1.6a **−197**, Fritz 16 **−161**, Rybka 4 **−109**; Shredder 12 +180, HIARCS 14 +76; Basilisk 1.9.3 −26 | Colosseum rating tournament, RAR-M45 registers the refresh |
| Search deficit with Rarog's own evaluation | **−250.8 ± 13.1 Elo** at equal time, −355 at equal nodes, against Stockfish's search | `analysis/ablation_results.md`, RAR-S70-era head |
| Where the search deficit lives | LMR plus shallow-depth pruning explain **272 ± 18** of it, near-additively; everything else about 30 | matched ablation, mask 160 |
| Evaluation deficit with the same search | Stockfish's classical HCE beats Rarog's HCE by **about 329 Elo** | RAR-O02 |
| Speed | 3.22 MNPS at bench 13, PGO, 1T; Basilisk 3.71; board work 24% of time, evaluation 29%, search loop 23% | RAR-M36, RAR-M44 |
| Conversion | 57 draws and 12 losses after holding a piece-up advantage for 12+ plies, in 2,400 games against the six HCE-era engines; Basilisk 40 and 12 | replay of the same tournament, A.6 makes this a tracked instrument |
| Fingerprint | `bench 13` **7,601,220 / EBF 2.474** at `c80df74`, accepted by RAR-E15 | GUIDE checkpoint |

Both halves of the engine have room of the same order. The search half is
attacked first because it is the larger measured single item, because a
stronger search produces better self-play labels for every later evaluation
refit, and because the selectivity stack is where the interaction problem is
worst: LMR, move-loop pruning, histories and correction feed each other, and
the evidence says increments to the current co-adapted optimum measure zero
while an unfitted wholesale rewrite lost. The plan therefore rebuilds the
search as a **coherent architecture adopted as a unit, seeded from a donor,
fitted locally, gated in clusters**, with the evaluation frozen until the
search checkpoint. The evaluation programme then does the same for the
evaluation families, with the search frozen, and a joint re-fit closes the
classical stage.

### Elo budget, stated so it can be wrong

| Programme | Measured deficit | Planned recovery | Basis |
|---|---:|---:|---|
| B search | 251 equal-time | 120–200 | selectivity explains 272; a Reckless-shaped stack fitted locally |
| C evaluation | 329 same-search | 100–160 | six families, whole-surface refits, endgame conversion |
| D clock, SMP, robustness | unmeasured | 15–40 | Reckless-shaped node-fraction TM; 4T quality |
| Speed inside B and C | — | 10–30 | per-node cost of the new search and evaluation modules |

If those bands are right the classical head lands within reach of Rybka 4 and
Fritz 16 and near Critter; Houdini 3 may only fall in the NNUE stage. Each
programme's checkpoint re-measures its deficit meter so a miss is seen as a
miss and the budget above is corrected rather than defended.

## 2. Operating rules

`AGENTS.md` is authoritative for measurement, verification, documents and
gating. The rules below decide order and acceptance in this roadmap.

1. **Donor architecture, own implementation.** Reckless is the primary donor
   for search, threading, time management and NNUE; Stockfish 11 is the donor
   for the classical evaluation and Stockfish 19 for NNUE and SMP details where
   Reckless is silent. Architecture, mechanisms, population choices and
   constants may all be taken. Code is written by us in our own structure;
   line-for-line transcription is used only where an algorithm has one natural
   form or where a different form provably loses throughput. Similarity to a
   donor is never an acceptance criterion; games are.
2. **Constants are seeds.** A ported constant sits on the donor's score scale
   and node population. It is converted through the measured scale ratio (B.0),
   seeded, fitted by SPSA over the cluster's live coordinates, and only then
   gated. Search and evaluation coordinates never share a tune.
3. **Clusters, not features.** The unit of implementation and of strength
   acceptance is one dependency-complete, co-adapted cluster: every mechanism
   that consumes or produces a shared signal moves together. A feature
   implemented so it can be reported exists for nothing. Internal sub-steps
   are compiled and diagnosed separately but are not expected to win standalone
   and do not get their own gates.
4. **Compatibility over completeness.** Before adding a mechanism, audit the
   ones it will feed or be fed by. Re-implement an existing feature when the
   donor's form of it composes better with its neighbours; keep ours when the
   evidence says ours is better. Ordering-to-pruning feedback, evaluation-to-
   margin coupling, TT masking and history gravity are the interactions that
   have bitten this project; name them in every cluster handoff.
5. **Each cluster: audit, register, implement, prove, explain, fit, gate,
   record.** Registration in `EXPERIMENTS.md` before any games: hypothesis,
   baseline SHA, bracket, cap, stop rule and the frozen prediction. Bounds
   default to `[0,3]` nElo; a large prior uses `[0,10]` or `[3,10]` and says
   why; a removal or unknown-sign repair uses a loss-permitting or symmetric
   bracket. Never change a gate after seeing games.
6. **Two rejected clusters in one programme stop it** and force a new evidence
   audit before a third is built.
7. **Every strength A/B runs with adjudication off**, at `3+0.03`, 1T, Hash 64,
   paired UHO, `fastchess -use-affinity`, concurrency 14, unless the
   registration states otherwise and why. Multi-thread gates drop affinity and
   calibrate a null pair first.
8. **State the measurement layer.** Theory truth, move quality, conversion,
   fixed-node tree shape, NPS and game strength are different units with no
   exchange rate. Counters, node counts, EBF, tactical suites and fit loss
   explain or screen; only a registered final-PGO SPRT or the target gate
   accepts.
9. **Freeze the prediction before exposure, append the calibration after.**
   A miss is recorded as sign, magnitude, mechanism, interaction, confidence
   or instrument. `NO_CHANGE`, refuted and too-sparse are successful outcomes.
10. **Deficit meters are re-measured at every programme checkpoint**: the
    equal-time gap against the frozen Stockfish-search oracle (B), the
    same-search gap against Stockfish's classical HCE (C), the conversion
    instrument (A.6), pooled-PGO NPS, and the pool score against the four
    target engines.
11. **Engine, tooling and documentation changes are separate commits.** PLAN
    and GUIDE change together. Completed IDs never change; open IDs may be
    renumbered with a map.
12. **Expensive jobs are the maintainer's**: SPRTs, SPSA, datagen, tournaments,
    PGO campaigns, long profiles. Budget: about three SPRT-sized runs per day.
    The agent prepares, verifies and hands over one runnable command.

### Workflow states and capability classes

`RESEARCH -> READY_FOR_IMPLEMENTATION -> IMPLEMENTED -> LOCAL_QUALIFIED -> GAME_GATE -> CLOSED`

`READY_FOR_IMPLEMENTATION` is the boundary at which the mechanism, semantics,
evidence, interactions, invariants, falsifier and accept/reject rule are
frozen; implementation owns ordinary engineering inside that contract and
returns a false premise to `RESEARCH` instead of rescuing it.

| Class | Required capability | Typical use here |
|---|---|---|
| `R3` | Frontier causal/architecture research | programme investigations (B.0, C.0, F.0), cluster design |
| `R2` | Bounded correctness-sensitive reasoning | audits with a known question, contract definition |
| `I2` | Difficult implementation | cluster implementation in search, evaluation, NNUE runtime |
| `I1` | Well-specified implementation | tooling, refactors with an exact fingerprint, ports from a written handoff |
| `M` | Mechanical documentation/provenance | ledger rows, archives, changelogs |
| `V` | Verification/measurement | qualification runs, gate preparation, re-measurements |

GUIDE maps classes to current model names and thinking modes. Investigation
leaves (`R3`) are expected to **spawn** implementation and measurement
sub-steps under their own step; the sub-steps listed below under an
investigation are the expected shape and are confirmed, split or replaced by
the investigation's handoff.

### Standing contracts

Live invariants that every change must keep. Their derivations are in the
linked analyses; this table is the index.

| Contract | Where it is written down |
|---|---|
| Board, legality, make/unmake, SEE king legality and created pins, 41 external fixtures | `analysis/see_contract_2026-09-06.md`, `analysis/see_repair_2026-09-06.md`, `tests/data/see-*.tsv` |
| History capacity and canonical-move contract | `analysis/history_contracts_2026-09-08.md` |
| Draw, null, repetition and rule-50 policy identities | `analysis/draw_policy_2026-09-08.md` |
| Board footprint assertions (`Board <= 264`, `UnmakeInfo <= 24` bytes) | `src/board/board.rs` const assertions |
| Caller-owned move-list delivery; no 520-byte return copy | `analysis/movelist_delivery_2026-09-09.md` |
| Diagnostic counter units and sampling | `analysis/phase4_counter_spec.md` |
| Measurement layers for endgame work | `analysis/endgame_measurement_layers.md` |
| Texel data contract, splits, instrument coverage | `analysis/texel_fitting_handbook.md`, `analysis/hce_archive_audit_2026-08-31.md` |
| Behaviour-neutral change = exact fingerprint plus targeted checks plus pooled NPS | `AGENTS.md` |
| Cross-engine board benchmark parity | `benches/board.rs`, `analysis/board_comparison_411b19_2026-09-09.md` |

## Phase A — Reset: repository, instruments, baselines, consolidation release

Short and mostly mechanical. It leaves the repository clean, ships the
accepted head as a release before the search programme rewrites the search,
and measures every deficit meter on that released binary, which is the head
the programmes start from. Execution order is the numbering: the release
(A.3) comes before the baselines (A.5) so that the baseline measurements are
release evidence as well and are taken on the exact shipped binary; the
universal-binary investigation (A.4) runs while the release gate's games are
being played and can still make this release if it passes its checks.

- **A.1 Document reset — `M`, CLOSED 2026-09-09.** New PLAN, GUIDE and
  HISTORY; the Phase-4 roadmap and GUIDE archived under `docs/archive/`;
  `check_guide.py` adapted to lettered phases; AGENTS and PROCESS references
  updated.
- **A.2 Repository and branch cleanup.**
    - **A.2.1 Tracked-file cleanup — DONE 2026-09-09.** Removed, all last
      present at `6fa6731`: the 4.11.7 study runners (`archive_4117.py`,
      `run_4117_registered.py`, `summarize_4117.py`; RAR-M21's outputs are
      archived locally), `run_board_search_profile_411b7.ps1` (one remote
      measurement; the reusable ETW capture and summarizer stay),
      `profile_probe.py` and `profile_attrib.ps1` (legacy duplicate-work
      probes, superseded by the ETW profile), `nps_ab.ps1` (superseded by
      `nps_multibuild.ps1`), `perft_compare.py` (superseded by the
      cross-engine board benchmark), `tools/texel/reference/basilisk_tuner.cpp`
      (a copy of Basilisk's tuner; the Rust port is the tool), `import_beast.py`
      (the rejected Stockfish-label path, RAR-E03), and `holdout.py` with its
      test (imported by nothing). Kept for named owners: the SMP probe scripts
      (D.2), the answer harness and search-quality readouts (B.0 decides), the
      SPSA configs (A.2.3 decides).
    - **A.2.2 Branch and tag disposition — DONE 2026-09-09.** The oracle
      package (`rarog-stockfish-hce-hybrid.exe` `da78a145…`, `rarog_hce.dll`
      `e43b602b…`, licences) is archived at
      `D:/chess/engines/oracle-rarog-hybrid-75d0d43/`. Tags `oracle/hybrid`
      (75d0d43), `oracle/hybrid-diag` (2682f64), `oracle/hybrid-ablate`
      (984f478), `arm/p410-jitter-1t` (e7965b9), `arm/p410-lmr-relief`
      (5dbeb52), `arm/p410-margin-relief` (e950f03) and `arm/p46-root-relief`
      (2a64941) were created and pushed; the seven branches were deleted
      locally and on `origin`, and three stale worktrees (a temporary
      `hybrid-ablate` checkout, `target/411b7-probe-work`,
      `D:/code/rarog-411b8-baseline`) were removed. Only `master` and `dev`
      remain. Every ledger SHA cited on those branches resolves through a tag.
      Observation for the maintainer: the local lightweight tags `v1.3.0`
      through `v2.3.0` point at different objects than the annotated tags on
      `origin` (`git push --tags` rejected twelve); `origin` is authoritative
      and `git fetch --force --tags` would realign them.
    - **A.2.3 Feature and option inventory — DONE 2026-09-09.** All four
      Cargo features stay (`diag` and `tune` as instruments, `texel` as the
      fitting path, `ablate` until B.9); all nine UCI options are consumed.
      Of the 99 `SearchParams` entries, **42 are inert at default** (zero
      guards, zero additive terms, or weights behind an off switch) and are
      removed in B.1; **55 are live seeds** the B.2 cluster replaces with
      donor-shaped successors; `lazy_margin` belongs to C.1 and
      `ablation_mask` leaves with the feature. RAR-S65 to S69 are superseded by
      B.2 and recorded so at B.1; typed TT provenance and the SPSA configs are
      B.0/B.1 decisions. Evidence: `analysis/feature_inventory_2026-09-09.md`.
- **A.3 Consolidation release — `V`/`M`.** Ship the accepted head before the
  search programme changes it. The 2026-09-04 pool already has the head's
  predecessor at **+43.7 Elo head-to-head over 2.3.2** (400 games) and +29 in
  pooled pool score; ProbCut, root LMR relief, two HCE refits, TB-corrected
  labels and the board cluster are all gated individually. The version follows
  the release rule in section 4: 2.4.0 if the registered STC gate's point
  estimate is at least +40 with the lower bound above +25, else 2.3.3.
    - **A.3.1 Toolchain bump, behaviour-neutral — DONE 2026-09-09 (`ca8988a`),
      RAR-P18.** `rust-toolchain.toml` moved from 1.97.1 to 1.98.1
      (`48a229cea`), with no experiment in flight and before RAR-E16's binaries
      exist. Every done criterion met: fingerprint 7,601,220 / EBF 2.474 exact
      on the `x86-64`, `avx2` and `pext` plain builds and on three `pext` PGO
      builds; `cargo test -p rarog` debug and release green, tooling crates
      green, fmt and clippy clean; `verify-isa` clean on all four assets;
      pooled-PGO NPS −0.53% (95% CI −1.99% .. +0.08%), inside ±1%, with a
      same-source null pair at −0.42% (−1.30% .. +0.33%) showing the
      instrument cannot separate the compiler from per-build profile luck.
      **Two obligations remain open and belong to other owners:** RAR-P08's
      `rust-lld` Windows ARM64 PGO workaround is unverified on 1.98.1 and needs
      the ARM64 compatibility host (RAR-P14 is the 1.97.1 precedent), and the
      CI matrix has not yet run on the new pin. Both are held in GUIDE and are
      required before A.3.3 publishes assets.
    - **A.3.2 Release gate — RAR-E16, `V`.** Registered before games: the
      release candidate (A.3.1 head, PGO pext) against the 2.3.2 release binary,
      `3+0.03`, 1T, Hash 64, paired UHO, no adjudication, `[3,10]` nElo, cap
      16,000 games, plus `10+0.1` and 4T `10+0.1` direction checks of 400 games
      each with zero forfeits. Prediction frozen in the row. H1 with the point
      estimate at or above +40 and the lower bound above +25 licenses 2.4.0;
      any other H1 licenses 2.3.3; H0 stops the release and is itself a finding
      against the accepted-gains ledger. While these games run, the agent works
      A.4. **Prepared 2026-09-09, games not started.** Both arms are built with
      verified manifests; RAR-E16 carries their paths, hashes and fingerprints.
      The registered
      baseline artifact was wrong and was replaced before any game: the file the
      row named benches the development fingerprint, not 2.3.2's, and its
      `--native` flavour is never released and is refused by the harness's
      flavour guard. The replacement is built from tag `v2.3.2` with the release
      recipe and reproduces RAR-M12's recorded 6,519,711 / EBF 2.449. Bounds,
      cap, clock, book, adjudication and prediction are unchanged.
    - **A.3.3 Release — `M`.** Version strings, README, CHANGELOG from the
      accepted ledger rows since 2.3.2, fmt, suites, clippy, feature builds,
      fingerprint, PGO assets with ISA verification, CI matrix on the release
      commit, tag and publish on maintainer instruction. `master` fast-forwards
      to the release commit. Ships the universal x86-64 binary **only if A.4.5
      adopted it**; otherwise the per-tier assets as before, and A.4's verdict
      names its later owner. The release does not wait beyond A.4's stop rule.
- **A.4 Universal x86-64 binary — `R2` investigation with `I2`/`V`
  sub-steps.** Goal: one Windows and one Linux x86-64 executable that selects
  the best code path at startup, so users stop choosing `base`/`avx2`/`pext`.
  Stockfish 19 does this by compiling the whole engine once per tier into
  separate C++ namespaces (`Stockfish_<arch>::main`), collecting each tier's
  static initializers into its own section, linking every copy into one file,
  and dispatching from CPUID in `src/universal/entry_x86.cpp`; its asm
  sections are rewritten by scripts and each tier is PGO-trained. Rust has no
  namespace wrapper and cargo applies target features per build, not per
  crate, so the mechanism must be found, not assumed. Adoption criteria, fixed
  now: single file per OS; exact `bench 13` identity per forced tier against
  the dedicated build; fixed-node whole-search NPS per tier within **1%** of
  the dedicated PGO binary of that tier; startup under 50 ms; no writes to
  disk at runtime; baseline fallback on a CPU without BMI2/AVX2 and on an OS
  with vector state disabled; a forced-tier override for tests; PGO per tier
  or a measured statement of what PGO is lost. Stop rule: if A.4.1 finds no
  design meeting the single-file and 1% criteria, or A.4.2's prototype cannot
  be made to link cleanly within two working sessions, the release ships
  per-tier assets and adoption moves to G.2 with the blocker recorded.
    - **A.4.1 Design — `R2`.** Compare, with a two-crate link prototype for
      the symbol question before anything else: (a) a fat binary of N engine
      copies, built by `xtask` as N `staticlib`s with distinct `-C metadata`
      and `-C target-feature`, each exporting one `extern "C"` tier entry, and
      a baseline-compiled dispatcher that links all N (falsifier: duplicate
      Rust std or allocator symbols across the copies fail the link on MSVC and
      GNU ld; test it first); (b) function-level multiversioning of the hot
      kernels only (slider lookups PEXT versus magic, and later NNUE), one
      baseline codegen elsewhere (falsifier: the measured loss of global
      `target-cpu` codegen; the 4.11b lesson says measure in search, not on
      the bench); (c) a launcher executable that detects the CPU and hands
      its stdio to a sibling tier executable from the same archive (one name
      for GUIs, not one file; falsifier: the single-file criterion); (d) a
      single-file launcher with embedded tier images written to a per-user
      cache (falsifier: the no-runtime-writes criterion). Deliver
      `analysis/universal_binary_2026-xx.md` with the chosen design, the
      dispatch table (BMI2 fast versus slow PEXT CPUs, AVX2, OS XSAVE state),
      and frozen A.4.2–A.4.4 handoffs.
    - **A.4.2 Prototype — `I2`.** Implement the chosen design as an `xtask`
      build target (`--arch universal`) in isolation; production defaults and
      per-tier assets unchanged; ISA verification adapted to per-tier code
      regions; a forced-tier override and a selected-tier report on startup.
    - **A.4.3 Compatibility and identity — `V`.** Every forced tier on this
      host and the automatic choice; baseline-only and vector-disabled cases
      via the override or ISA-constrained execution, labelled as gaps where no
      hardware exists; exact bench identity per tier; UCI, stop and thread
      lifecycle; debug and release suites; perft and the board fixtures.
    - **A.4.4 Performance and size — `V`.** Fixed-node NPS per tier against
      the dedicated PGO binaries, pooled and alternated; startup time; binary
      size; memory. The 1% criterion decides.
    - **A.4.5 Decision — `R2`.** Adopt for A.3.3, or defer to G.2 with the
      blocker and the measured shortfall. If adopted, the released universal
      asset is re-validated against the gated pext binary by bench identity
      per tier and a 400-game null pair before it ships; the SPRT is not
      repeated.
- **A.5 Baselines on the release binary — `V`.** All maintainer-run,
  registered first. The Rarog arm is the A.3.3 release binary (its pext tier
  if universal), or the A.3.1 head if the release was not cut.
    - **A.5.1 Reference pool refresh (RAR-M45).** Colosseum rating tournament
      with Houdini 3 added to the existing 14-engine pool, `3+0.03`, 1T, 400
      games per pair. Produces the head-to-head table the E.2 gate is measured
      against and the first Houdini 3 number. Because 2.3.2 stays in the pool,
      the run is also release evidence at the pool level.
    - **A.5.2 Four-thread pool (RAR-M46).** The same pool at 4T for the four
      target engines and Basilisk only, 400 games per pair, no affinity, after
      a 4T null pair. Establishes the SMP starting point for D.2.
    - **A.5.3 Oracle deficit meter (RAR-O03).** Paired equal-time run of the
      release binary against the `hybrid` oracle, `3+0.03`, 3,000 games, no
      adjudication. The oracle is the frozen Stockfish `9587eeeb` search
      driving `rarog_hce.dll`; rebuild that DLL from the release head's
      evaluation (`hybrid/build.ps1` on the `oracle/hybrid` tag) so the meter
      holds the evaluation constant and measures search only. RAR-S70's 250.8
      is the prior. Prediction registered in the row.
    - **A.5.4 Speed baseline.** Pooled-PGO NPS of the release head with the
      RAR-M41 protocol; three builds; archived hashes. The number B and C
      measure against.
- **A.6 Conversion instrument — `I1`.** Make the replay used on 2026-09-09
  (`tools/results/conversion-replay-20260909/replay.py`) a tracked tool:
  `tools/diag/conversion_audit.py` reads a Colosseum tournament by id,
  replays every game of a named engine against a named opponent set, and
  reports draws and losses after a persistent material advantage (12 plies,
  at least a minor piece, lone-minor exclusions by material signature), by
  termination and by phase, plus saves from persistent deficits. Baseline:
  Rarog 57/12, Basilisk 40/12 on tournament `41768fe9`. The tool is re-run at
  every programme checkpoint; it is a diagnostic layer, never an acceptance
  layer.
- **A.7 Codebase consolidation analysis — `R2`.** Inventory the crate
  (24,103 lines; `search.rs` 6,275 with a 1,700-line `negamax`, `eval.rs`
  3,756, `diag.rs` 1,298, `params.rs` 987, `evidence.rs` 707) against the
  module layout the programmes will produce, and decide what is refactored
  now, what is replaced by B and C, and what is deleted. Rule: do not refactor
  what a programme is about to replace. Expected output: the B.1 and C.1
  restructure handoffs, a dead-code list for A.2.3, and the target layout
  below. No behaviour change in this leaf.

Target module layout after B and C (the investigations may adjust it):

```
src/board/…                board, movegen, see, zobrist        (as today)
src/search/mod.rs          iterative deepening, root, aspiration
src/search/node.rs         negamax<NodeType>, qsearch
src/search/stack.rs        per-ply StackEntry, PlyArray
src/search/movepick.rs     staged picker and scoring
src/search/history.rs      quiet/noisy/pawn/continuation histories
src/search/correction.rs   correction histories and eval correction
src/search/params.rs       search_params! (tunable surface)
src/search/threads.rs      lazy SMP, shared context, voting
src/search/time.rs         soft/hard limits, node-fraction multiplier
src/tt.rs                  transposition table
src/eval/mod.rs            evaluate, phase, scale, tempo
src/eval/{material,pawns,pieces,king,threats,passers,space,initiative,endgame}.rs
src/eval/trace.rs          EvalTrace and fitting instrument
src/uci/…                  protocol, options, engine loop
src/diag.rs                counters (feature `diag`)
```

## Phase B — Search programme

**Goal:** recover the measured 251-Elo equal-time search deficit, with the
evaluation frozen at the accepted `hce-v3` head, by rebuilding the search as
a Reckless-shaped architecture implemented in Rarog's own code, seeded,
fitted and gated cluster by cluster.

**Why this shape.** The matched ablation says the deficit is selectivity:
LMR and shallow-depth pruning explain 272 ± 18 Elo, the remaining mechanisms
about 30. Rarog already carries an LMR table with adjustments, LMP, futility,
SEE pruning, NMP, ProbCut, singular extensions, IIR, correction histories and
a staged picker, and orders moves better than the oracle by its own counters;
what differs is the population each mechanism admits and how the signals that
gate them are produced and combined. Previous attempts changed one mechanism
at a time against a co-adapted optimum (zero), or rewrote the core without
fitting it (lost). The donor's search is a working co-adapted point; adopting
its shape as a unit and fitting our constants is the cheapest way to move to a
different optimum.

**What "Reckless-shaped" means, concretely** (source of record: Reckless at
`31d9cd6`, `src/search.rs`, `history.rs`, `movepick.rs`, `transposition.rs`,
`time.rs`, `thread.rs`; read it, do not copy it):

- Node types as compile-time constants (`Root`, `PV`, `NonPV`), a per-ply
  stack entry holding the static eval, TT move, TT-pv flag, move count, the
  applied reduction, laterality, and pointers to the continuation-history and
  continuation-correction sub-tables selected by the move just made.
- One TT with 3-entry 32-byte clusters, 8-byte entries carrying a 16-bit key,
  move, score, raw static eval, depth, bound, tt-pv and a 5-bit age; static
  eval stored on every probe miss; a probe-aligned "estimated score" that
  tightens the static eval with the TT bound.
- Histories: quiet history indexed by side and by whether the from and to
  squares are threatened; noisy history by piece, to-square, captured type and
  to-square threat; pawn-structure history by pawn key bucket; continuation
  history at plies 1, 2, 4, 6 keyed by in-check and capture; bonus/malus
  formulas linear in depth with caps, gravity updates, and the moved-late malus
  scaled by index.
- Correction histories: pawn key, non-pawn key per colour, continuation
  correction at plies 2 and 4, all bucketed by the fifty-move clock; applied to
  the raw eval together with material-scaled optimism and rule-50 damping.
- Move picker with stages hash, generate-noisy, good-noisy by SEE threshold
  derived from the move's own score, quiet, bad-noisy; quiet scores from the
  four histories plus threat escape, checking-square, en-prise and offense
  terms.
- Selectivity: razoring on the estimated score, RFP with improvement and
  correction terms, NMP with adaptive reduction and verification, ProbCut with
  reduced-depth verification, singular extensions with double/triple margins,
  multi-cut and negative extensions, low-depth singular extension, hindsight
  reductions from the parent's eval delta; in the move loop LMP, futility with
  history and correction terms, bad-noisy futility, history pruning, SEE
  pruning with depth-quadratic thresholds; LMR in 1024ths with terms for
  log-depth, improvement, correction, TT bound, quiet history, PV window
  width, laterality, tt-pv, cut-node, check, cutoff count, singular margin, the
  parent's reduction and a per-node jitter; deeper/shallower re-search
  decisions from the reduced score; PVS on PV nodes.
- Quiescence: TT probe and cutoff, corrected stand-pat, interpolated
  fail-high scores, LMP at three moves when not in check, SEE pruning by a
  margin from alpha, TT write on every exit.
- Root: aspiration delta from eval and PV stability, optimism from the best
  score average, root move node accounting, forgotten-mate and aborted-loss
  guards, multi-PV structure.
- Time: soft and hard bounds from the clock and increment with a fullmove
  curve; a soft-limit multiplier from the best move's node fraction, score
  trend, PV and eval stability and best-move changes; hard check every 2,048
  nodes; a soft-stop vote across threads.
- Threads: lazy SMP with per-thread data, shared TT, shared correction
  histories, a shared best-stats word, and no depth skipping.

**Scale conversion (B.0).** Reckless's centipawn-like scale multiplies the
network output by material; Rarog's HCE is on its own refit scale. Every
cp-valued seed is converted by a measured ratio: the ratio of mean absolute
static evaluation over the same 40 bench positions plus 10,000 datagen
positions, Rarog HCE against Reckless's network. The ratio is recorded with
the B.0 handoff and used for every seed; it is a starting point that SPSA
moves, not a result.

**Gating shape for B.** Each cluster: (1) fingerprint changes, so no
neutrality claim; (2) fixed-node diagnostics against the oracle at sample
stride 1 (depth at 300k nodes, EBF, qsearch share, cutoff composition, LMR
re-search rate, NMP conversion), registered as explanation only; (3) a 2,000
game paired diagnostic run at seeds converted but unfitted, to detect a broken
port early (stop rule: worse than −40 Elo means a defect, not a tuning need);
(4) SPSA over the cluster's live coordinates, registered surface and horizon;
(5) the SPRT, `[0,10]` for the selectivity core and `[0,3]` or `[0,5]` for the
smaller clusters, sized from RAR-M10 at the expected value; (6) the ledger
row with calibration. Reject returns the cluster to `RESEARCH` with its
diagnostics; two rejections stop B.

- **B.0 Investigation: current search against the donor, cluster boundaries,
  seeds and instruments — `R3`.** Produce `analysis/search_programme_2026-xx.md`:
  the mechanism-by-mechanism map of Rarog's `negamax`/`quiescence`/picker/
  histories/TT/TM/SMP against Reckless (and Stockfish 19 where Reckless is
  silent), with each difference classified as adopt / keep ours with evidence
  / drop; the exact cluster contents below confirmed or changed; the scale
  ratio; the list of Rarog mechanisms with local evidence that must survive
  (no in-check extension: +30.75 Elo for removing it; root LMR relief; ProbCut
  move filter; typed TT provenance only if a consumer is named); the SPSA
  surface per cluster; the oracle-differential counter set re-mapped to the
  new mechanism names; the AblationMask disposition. Ends with frozen handoffs
  for B.1–B.3 and predictions for B.2. **No engine implementation.**
- **B.1 Search restructure, behaviour-neutral — `I1`.** Split `search.rs`
  into the target modules; introduce the `NodeType` constants, the
  `StackEntry`, `PlyArray` and shared-context types; move params into
  `search/params.rs`; remove parameters classified dead in A.2.3; keep every
  mechanism exactly as it is. Done criteria: exact fingerprint
  7,601,220 / EBF 2.474 on magic and PEXT, debug and release suites, clippy,
  pooled-PGO NPS inside ±0.5% of A.5.4. This is the scaffold the clusters land
  on; it earns no strength credit.
- **B.2 Cluster 1 — the selectivity core — `I2`, then `V`.** One cluster:
  TT entry format with stored raw eval and tt-pv and the estimated-score rule;
  the correction histories and corrected-eval formula (including optimism
  and rule-50 damping in the search, replacing the evaluator's damping if B.0
  says so); the quiet/noisy/pawn/continuation histories with their update
  rules; the move picker stages and scoring; the move-loop pruning (LMP,
  futility, bad-noisy futility, history pruning, SEE pruning); the LMR formula
  and re-search rules; RFP and razoring on the estimated score; hindsight
  reductions; cutoff counting. NMP, ProbCut, singular and extensions keep
  Rarog's current forms in this cluster so that B.3 can measure them
  separately. Sub-steps:
    - **B.2.1** Implement to the B.0 handoff; unit tests for every table's
      bounds and gravity; picker exhaustiveness tests; TT store/probe tests
      including age and replacement; deterministic unwind tests.
    - **B.2.2** Diagnostics: oracle differential at stride 1, depth at 300k
      nodes, EBF, tactical suite at fixed depth and equal nodes, 2,000-game
      unfitted paired run. Registered as explanation.
    - **B.2.3** SPSA over the registered live coordinates (expected 40–70),
      `tools/spsa.ps1`, immutable horizon, staged stop. Maintainer-run.
    - **B.2.4** Gate: registered SPRT `[0,10]` against the B.1 head, cap
      sized from RAR-M10; then ledger row and calibration. Accepted head
      becomes the base for B.3.
- **B.3 Cluster 2 — proof searches and extensions — `I2`, then `V`.** NMP
  with adaptive reduction and verification, ProbCut with reduced-depth
  verification and the TT-served shortcut, singular extensions with
  double/triple margins, multi-cut, negative extension, low-depth singular
  extension, IIR versus hindsight-depth policy. Rarog's own evidence rules:
  no in-check extension unless a new measurement says otherwise. Same
  sub-step shape as B.2 (implement, diagnose, SPSA if curvature justifies,
  SPRT `[0,5]`).
- **B.4 Cluster 3 — quiescence — `I2`, then `V`.** Reckless-shaped qsearch:
  TT cutoff, corrected stand-pat, fail-high interpolation, LMP at three
  moves, SEE pruning by margin, TT write on exit, check evasions only when
  in check. Target: Rarog's qsearch share (62% larger than the oracle's per
  interior node) without losing tactical suite results at equal nodes. SPRT
  `[0,3]`.
- **B.5 Cluster 4 — root, aspiration, iterative deepening — `I2`, then `V`.**
  Aspiration delta from eval and PV stability, optimism, root move node
  accounting, forgotten-mate and aborted-loss guards, PV table, multi-PV.
  SPRT `[0,3]`. Root-only LMR relief keeps its accepted place unless B.2's
  formula subsumes it, which B.0 decides.
- **B.6 Search SPSA — `V`.** One joint SPSA over the coordinates the four
  clusters left live, only if B.0's curvature evidence and the cluster
  results justify it. Registered surface; PGO bake; SPRT `[0,3]`.
- **B.7 Search speed pass — `I1`, then `V`.** Behaviour-neutral throughput
  work on the new modules (allocation, layout, prefetch, inlining measured in
  search not on the bench), pooled-PGO NPS with a +0.5% floor per change,
  exact fingerprint. The 4.11b lesson stands: a bench-column win is a screen,
  not a result.
- **B.8 Cleanup — `I1`.** Remove dead parameters, unconsumed switches, the
  old `MovePicker`, evidence/provenance plumbing without a named consumer,
  and any diagnostic without an owner. Exact fingerprint; no game gate.
- **B.9 Checkpoint — `V`.** Re-measure the deficit meters: RAR-O-series
  equal-time G(0) against the oracle, fixed-node depth and EBF, pooled-PGO
  NPS, conversion instrument, and a pool gauntlet against the four target
  engines and Basilisk at 1T (400 games each). Record attributed Elo per
  accepted cluster from the SPRTs, and the checkpoint against the budget
  table. Remove the `ablate` feature afterwards; archive the oracle branches
  as tags (A.2.2) if not already done. **Freeze the search head for C.**

### Active workflow register

One row per open leaf in the active phases (A and B). The checker requires
the state and class here to match GUIDE's suffix. Later phases carry only a
class until they open.

| Leaf | Workflow state | Class | Current decision |
|---|---|---|---|
| A.3.2 | LOCAL_QUALIFIED | V | Arms built and manifest-verified, baseline corrected before games; maintainer runs the STC SPRT, then the LTC and 4T direction checks |
| A.3.3 | RESEARCH | M | Waits for the A.3.2 verdict and the A.4.5 decision; version per the release rule |
| A.4.1 | READY_FOR_IMPLEMENTATION | R2 | Symbol-isolation link prototype first; then the design document and handoffs |
| A.4.2 | RESEARCH | I2 | Waits for A.4.1 |
| A.4.3 | RESEARCH | V | Waits for A.4.2 |
| A.4.4 | RESEARCH | V | Waits for A.4.3 |
| A.4.5 | RESEARCH | R2 | Adopt for A.3.3 or defer to G.2 |
| A.5.1 | READY_FOR_IMPLEMENTATION | V | RAR-M45 registered; maintainer-run pool with Houdini 3 at 1T, on the release binary |
| A.5.2 | READY_FOR_IMPLEMENTATION | V | RAR-M46 registered; 4T pool after a 4T null pair |
| A.5.3 | READY_FOR_IMPLEMENTATION | V | RAR-O03 registered; equal-time G(0) with the evaluation held constant |
| A.5.4 | READY_FOR_IMPLEMENTATION | V | Pooled-PGO NPS of the release head, three builds, hashes archived |
| A.6 | READY_FOR_IMPLEMENTATION | I1 | Track the 2026-09-09 replay as a tool with tests; baseline recorded |
| A.7 | RESEARCH | R2 | Inventory and target layout; outputs the B.1 and C.1 handoffs and the dead-code list |
| B.0 | RESEARCH | R3 | Opens after A.7; ends with frozen handoffs for B.1–B.3 and the scale ratio |
| B.1 | RESEARCH | I1 | Waits for the B.0 handoff; exact fingerprint required |
| B.2.1 | RESEARCH | I2 | Waits for B.1 |
| B.2.2 | RESEARCH | V | Waits for B.2.1 |
| B.2.3 | RESEARCH | V | Waits for B.2.2; maintainer-run SPSA |
| B.2.4 | RESEARCH | V | Waits for B.2.3; SPRT `[0,10]` registered before games |
| B.3 | RESEARCH | I2 | Waits for the accepted B.2 head |
| B.4 | RESEARCH | I2 | Waits for B.3 |
| B.5 | RESEARCH | I2 | Waits for B.4 |
| B.6 | RESEARCH | V | Conditional on curvature evidence |
| B.7 | RESEARCH | I1 | After B.6 or its skip |
| B.8 | RESEARCH | I1 | After B.7 |
| B.9 | RESEARCH | V | Closes the programme; freezes the search head |

## Phase C — Evaluation programme

**Goal:** recover the measured 329-Elo same-search evaluation deficit, with
the search frozen at the B.9 head, by re-implementing the evaluation families
in Stockfish 11's classical shape where its conditioning is stronger, keeping
ours where the evidence says ours is better, refitting the whole surface
after every family cluster, and giving endgame handling its own bounded
cluster.

**Why Stockfish 11 here.** Reckless has no hand-crafted evaluation. Stockfish
11 is the last classical Stockfish and the reference the maturity record
already compares against (`analysis/hce_maturity_2026-08-25.md`). Its
families and their conditioning are the donor; its constants are seeds on a
different scale and ride the next Texel refit.

**Cluster shape for C.** Each family cluster: (1) the family's current terms
traced and their activation and residual measured on the fitting corpus by
cohort (king danger by attacker count, passers by rank and blocker, threats
by piece pair, endgames by material signature); (2) the donor's form of the
family read for its conditioning and populations; (3) a design that states
which terms are replaced, which stay, and which neighbouring families share
inputs (attack maps, mobility areas, pawn structure) so that the shared inputs
are computed once; (4) implementation with `EvalTrace` coverage for every new
slot and the reconstruction test; (5) a whole-surface Texel refit with the
existing toolchain, frozen test reported once; (6) a PGO bake and SPRT `[0,3]`
(`[3,10]` when the family's residual is large); (7) ledger row. Fit loss is a
screen and a falsifier, never acceptance (RAR-E03 lost 17 Elo with better
loss).

- **C.0 Investigation: family map, residuals and cluster order — `R3`.**
  Produce `analysis/eval_programme_2026-xx.md`: the six-family map from the
  maturity record refreshed on the B.9 head, per-family residual and
  activation evidence, the donor comparison of conditioning, the shared-input
  plan, the cluster order by expected value, the datagen and refit protocol
  for the programme (corpus name, size, splits, label policy from the label
  audit), and frozen handoffs for C.1 and the first family cluster. **No
  engine implementation.**
- **C.1 Evaluation restructure, behaviour-neutral — `I1`.** Split `eval.rs`
  into the target modules; one attack-map and mobility-area producer consumed
  by pieces, king, threats and space; `EvalTrace` unchanged in meaning. Exact
  fingerprint, suites, pooled NPS inside ±0.5%.
- **C.2 Datagen and label contract for the programme — `V`.** Generate the
  programme's corpus with the B.9 search under the adjudication-off datagen
  profile; audit labels against tablebase truth (existing tool); freeze
  splits and manifests under a new corpus name. Records the label-contradiction
  rate and the corpus hash. Maintainer-run generation.
- **C.3 King safety cluster — `I2`, then `V`.** King danger in the donor's
  shape: attacker units and weights, safe and unsafe checks by piece type,
  weak squares in the king ring, king-flank attacks and defence, shelter and
  storm by file with the castling-destination alternative, queen-absent
  reduction, and the nonlinear danger-to-score map. Rarog's existing nonlinear
  danger table is the seed for the map. Refit, gate.
- **C.4 Threats and mobility cluster — `I2`, then `V`.** Mobility with a
  mobility area that excludes own king, queen, blocked pawns and pawn-attacked
  squares; threats: minor and rook attacks on weak enemies, hanging pieces,
  restricted squares, threat by pawn push, king threats, slider and knight
  attacks on the queen, weak queen protection. Shared attack maps from C.1.
  Refit, gate.
- **C.5 Endgame handling and winnability cluster — `R3` investigation with
  `I2`/`V` sub-steps.** The rescoped endgame section. Its goal is measured
  conversion and correct draw recognition where games actually go, not
  coverage of a function list.
    - **C.5.1 Classification and instruments — `R2`.** Adopt the registered
      family order (`tools/diag/endgame_ranking_v2.json`), confirm each family's
      kind (verdict, scale, conversion) against the code, and name the deciding
      instrument per family: theory truth (`endgame_truth.py`), drawn-cohort
      overclaim (`endgame_drawn.py`), conversion (`endgame_conversion.py`),
      floors, and the A.4 conversion audit at the game level.
    - **C.5.2 Generic winnability and scaling — `I2`.** The donor's scale
      factor logic in our form: pawn-count scaling for the stronger side,
      opposite-bishop scaling by non-pawn material and passers, rule-50
      scaling in the scale rather than only the global damping, and an
      initiative/complexity term conditioned on pawn count, king distance and
      both-flank pawns. This is what decides KRPPKRP (5.4% of games, no local
      7-man truth), KPsK (4.5%) and KBPsK (2.6%) generically. Refit, gate with
      an endgame-start cohort and STC.
    - **C.5.3 Conversion cluster: KXK, KBNK, KQKR — `I2`.** Mate drives and
      verdict families with the largest occurrence (KXK 37.8% of the set) and
      the largest measured conversion deficit (KQKR 23/13/3 at 60k/200k/600k
      nodes). Rule-50 damping interaction measured here, sign not assumed.
      Deciding instrument: conversion at bracketed budgets plus theory vetoes.
    - **C.5.4 Rook versus minor cluster: KRKN, KRKB, KRPKB — `I2`.** Three
      families with 100% or 99.6% drawn-cohort overclaim at +300 and the same
      over-representation in Rarog's games; one scaling mechanism, one gate.
    - **C.5.5 Rook and pawn cluster: KRPKR, KRKP, KPK, KPKP — `R2`.** Audit
      the existing scalers and the KPK bitbase integration; repair the 30.7%
      KRPKR overclaim if the drawn cohort supports it; close KPK/KPKP
      `NO_CHANGE` if their 4–5% overclaims do not select a mechanism.
    - **C.5.6 Measure-first families: KPsK, KBPsK, KBPPKB, KQKRPs — `R2`.**
      Measure coverage after C.5.2, decide whether any specific recogniser is
      still justified, otherwise close them as served generically.
    - **C.5.7 Theory sweep: KBPKB, KBPKN, KNNKP, KNNK, KQKP — `I1`.** Sub-1%
      families implemented or confirmed from one dispatcher with Syzygy tests
      and promotion-closure tests, no per-family research cards; `NO_CHANGE`
      where the evidence is clean (KNNK already measured clean).
    - **C.5.8 Endgame gate and closure — `V`.** One endgame-start cohort SPRT
      plus one STC SPRT for the whole C.5 cluster after its refit; floors and
      theory vetoes re-run; conversion instrument re-measured; KRPPKRP's 7-man
      hold recorded as an explicit exclusion unless independent truth appears.
- **C.6 Pawns and passers cluster — `I2`, then `V`.** Passed pawns with king
  proximity, blocker ownership and type, path safety and attack, unstoppable
  and unblocked conditions, rook behind; pawn structure conditionality
  (doubled, isolated, backward, connected by rank and phalanx, weak lever).
  Refit, gate.
- **C.7 Material, imbalance, phase and pieces cluster — `I2`, then `V`.**
  Imbalance in the donor's quadratic form seeded from current material terms,
  phase interpolation review, bishop pair and bishop-pawn colour terms,
  outposts, minor behind pawn, rook on open and semi-open files, trapped
  rook, weak queen, king protector distances. Refit, gate.
- **C.8 Refit cycles — `V`.** After the family clusters: regenerate data with
  the accepted head, refit the whole surface, gate; repeat while a cycle
  accepts, stop at the first that does not. Initialization control (neutral
  start against accepted start) in the first cycle.
- **C.9 HCE SPSA of nonlinear residue — `V`.** Only the activated nonlinear or
  global terms the linear trace cannot fit; skipped with a written reason if
  the surface is flat.
- **C.10 Search re-fit after the new evaluation — `V`.** The search's
  cp-valued margins were fitted on the B-era scale. One joint SPSA over the
  registered cp coordinates, PGO, SPRT `[0,3]`.
- **C.11 Checkpoint — `V`.** Same-search deficit against Stockfish's classical
  HCE re-measured (a fresh hybrid build at the C head is required; the oracle
  package recipe is on the tagged `hybrid` branch), conversion instrument,
  pooled NPS, pool gauntlet at 1T. **Freeze the classical evaluation.**

## Phase D — Clock, threads, robustness

- **D.1 Time management — `R2` investigation, `I2`/`V` sub-steps.** Audit the
  current clock (budget, overhead, `smp_reserve`, root confidence consumers)
  against the Reckless shape; implement the soft/hard bound model with the
  node-fraction and stability multiplier if B.5 has not already; forfeit
  margin sized on a null pair; one registered SPRT `[0,3]` at STC and a
  direction check at `10+0.1`.
- **D.2 Lazy SMP quality — `R2` investigation, `I2`/`V` sub-steps.** 4T and
  8T scaling against 1T at equal wall time; helper diversity, TT sharing,
  shared correction histories, soft-stop voting, thread-safe counters. Gate:
  4T SPRT `[0,5]` against the 1T-accepted head at 4T, no affinity, null pair
  first. High-thread and NUMA remain G.1.
- **D.3 Engine lifecycle and protocol robustness — `R2`, then `I1`.** UCI
  parsing and dispatch, stop/ponder/infinite semantics, new-game resets,
  malformed input, panic reporting, Syzygy probe policy and thread safety.
  Deterministic tests; zero crashes over the pool tournaments.
- **D.4 Tablebase policy — `R2`.** Root and interior probing depth and limits,
  WDL/DTZ use in conversion, interaction with the C.5 recognisers. Endgame-start
  cohort and conversion instrument decide.

## Phase E — Classical checkpoint and release

- **E.1 Attribution checkpoint — `V`.** Final head against 2.3.2 and against
  the B.9 and C.11 heads at STC, `10+0.1` and 4T; attributed Elo per programme
  from the accepted SPRTs; deficit meters; NPS; the maturity checklist
  (family map without unknown rows, every slot with a fitting instrument,
  every accepted representation reconstructing through `EvalTrace`).
- **E.2 Target gate — `V`.** The pool measurement defined in section 1, at 1T
  and 4T. Met, or not met with the measured shortfall per engine recorded.
- **E.3 Release — `M`/`V`.** Version, changelog, release notes, fmt, debug and
  release suites, clippy, feature builds, fingerprint, PGO assets, ISA
  verification, CI matrix, tag and publish on maintainer instruction. Version
  is 3.0.0 if E.2 is met, else 2.4.0.
- **E.4 Universal binary adoption — `I1`, only if A.4 deferred it.** Ship the
  A.4 design at this release if its blocker was resolved; otherwise G.2.

## Phase F — NNUE

**Rules.** Own data only, generated by Rarog's classical head and later by its
NNUE heads. Reckless is the runtime and trainer-pipeline donor; the
architecture ladder is ours. The classical evaluation stays in the tree as the
datagen baseline and the fallback until F.9 replaces it in releases.

- **F.0 Investigation: runtime, data pipeline and first architecture — `R3`.**
  Board event interface and accumulator ownership (dirty pieces, per-thread
  per-ply accumulators, king buckets, refresh cache); trainer choice
  (`D:/code/net_trainer` against Bullet) with feature ordering, quantisation
  and export contracts; data format, deduplication and split policy; the first
  architecture (768×N perspective network with output buckets); the cost
  ledger inherited from the board audit. Frozen handoffs for F.1–F.4.
- **F.1 Board events and accumulator scaffolding — `I2`.** Behaviour-neutral
  for the HCE: factual move deltas, evaluator-owned stacks, validity and
  refresh semantics, randomized unwind tests, exact fingerprint, pooled NPS
  cost recorded.
- **F.2 Data generation at scale — `V`.** 30–60M unique positions from the
  classical head under the adjudication-off profile, by-game splits,
  manifests, tablebase and hard-position cohorts; hashes frozen. Maintainer-run.
- **F.3 Trainer hardening and baseline nets — `I2`, then `V`.** Deterministic
  pipeline, two seeds per configuration, validation selects, frozen test
  reports once.
- **F.4 Scalar integration — `I2`.** `quantised.bin` contract, integer-exact
  conformance against the trainer's reference evaluation, clean HCE fallback.
- **F.5 Incremental and SIMD — `I2`, then `V`.** Same-net incremental parity
  on every move type, SIMD tiers (AVX2, PEXT builds, ARM NEON), scalar
  reference retained, pooled-PGO NPS attribution.
- **F.6 Search re-fit for the network — `V`.** Score scale, correction
  histories, margins, qsearch and SEE thresholds re-fitted on the new
  evaluator (C.10's protocol).
- **F.7 Architecture ladder — `R3` with `I2`/`V` sub-steps.** Output buckets,
  king buckets with mirroring, then relation and threat inputs as in
  Reckless, one axis at a time; each net gated against the previous.
- **F.8 Data frontier — `V`.** On-policy refresh with the strongest net,
  deduplication, hard-position mining; repeat while a cycle accepts.
- **F.9 NNUE release — `M`/`V`.** Beat the classical release at STC, LTC
  and 4T; platform matrix; publish.
- **F.10 CCRL top-100 gate — `V`.** Submit; the list decides. Shortfall
  measured against the pool and fed back into F.7/F.8.

## Phase G — Scaling, platforms and the top 50

- **G.1 High-thread and NUMA — `R2`, then `I2`.** 8/16/32T scaling, TT and
  net placement, large pages, thread affinity policy.
- **G.2 Platform and product — `I1`.** Universal dispatch if E.4 deferred it,
  Chess960 on demand, distributed testing when typical gains reach 1–3 Elo.
- **G.3 Frontier — `R3`.** Larger nets, data scaling, search fit at LTC; the
  top-50 gate is the CCRL list again.

## 3. Measurement protocols

| Meter | Protocol | Owner |
|---|---|---|
| Pool score | Colosseum, fixed pool with Houdini 3, `3+0.03`, UHO, no adjudication, 400 games per pair, 1T and 4T | A.5, B.9, C.11, E.2 |
| Search deficit G(0) | `tools/sprt.ps1` paired, head against the frozen `hybrid` oracle, 3,000 games, equal time, no adjudication | A.5.3, B.9 |
| Evaluation deficit | Hybrid at the current head against Stockfish-HCE hybrid, same search, 2,400 games | C.11 |
| Cluster acceptance | Registered final-PGO SPRT, brackets per rule 5, `[0,10]` for B.2 | every cluster |
| Neutral change | Exact fingerprint on magic and PEXT, debug and release suites, `nps_multibuild.ps1` pooled PGO | B.1, B.7, C.1, F.1 |
| Conversion | `tools/diag/conversion_audit.py` on the latest pool tournament | every checkpoint |
| Fixed-node shape | oracle differential at stride 1, depth at 300k nodes, EBF, tactical suite at fixed depth and equal nodes | B clusters |
| Endgame layers | theory truth, drawn overclaim, conversion at 60k/200k/600k, floors | C.5 |

Sizing every SPRT: `tools/spsa_convergence_model.py` and RAR-M10's drift
model at the expected value, before registration. Bracket, cap, book, clock
and adjudication never change after games are seen.

## 4. Release rules

- A release ships only from a head whose every accepted cluster has a ledger
  row and whose deficit meters are recorded at the checkpoint before it.
- 3.0.0 requires the E.2 gate met; 2.4.0 requires at least +40 Elo at STC over
  2.3.2 with the lower bound above +25, positive LTC and 4T lower bounds.
- NNUE releases require a win over the last classical release at STC, LTC and
  4T, and a clean platform matrix.
- Tag, push and publish only on maintainer instruction.

## 5. Documentation ownership

| File | Purpose |
|---|---|
| `GUIDE.md` | Operator guide, model mapping, prompts, the full checkbox board, checkpoint and next action |
| `PLAN.md` | This roadmap: objective, rules, phases, protocols |
| `EXPERIMENTS.md` | Frozen predictions, results, calibration, retry triggers, recipes |
| `PROCESS.md` | Research/handoff template and recurring build, fit, gate and release procedures |
| `HISTORY.md` | Completed work, retired numbering and the number map; never a source of the next step |
| `analysis/` | Per-leaf analyses and measurement records; raw artifacts stay local and ignored |
| `docs/archive/` | Verbatim archived roadmaps |

## 6. Number map

Every identifier from the archived roadmap is retired. Where a retired open
leaf continues here, this is the mapping; everything else is history.

| Retired | Continues as | Note |
|---|---|---|
| 4.12 (20 endgame functions) | C.5 (8 leaves) | rescoped from function coverage to conversion and generic scaling |
| 4.13, 4.14 (labels, refit cycles) | C.2, C.8 | inside the evaluation programme |
| 4.13a (HCE audit) | C.0 | |
| 4.15, 4.15a–c, 4.16, 4.18 (search audits, SPSA, cleanup) | B.0–B.9 | replaced by the search programme |
| 4.17 (time management) | D.1 | |
| 4.19, 4.20 (checkpoint, release) | E.1–E.3 | |
| 4.21 (universal binary) | A.4 | investigation and prototype before the consolidation release; E.4/G.2 own deferred adoption |
| Phase 5, 6, 7 (NNUE runway, baseline, frontier) | F | |
| Phase 8 (scaling) | G | |
| Phase 9 (classical fallback) | dropped | the classical evaluation stays as datagen baseline and fallback by construction |
