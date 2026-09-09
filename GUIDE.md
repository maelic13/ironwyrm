# Rarog development guide

## How to work with the engine agent

1. Ask **"what measured defect are we fixing?"** before asking which feature
   to add. Keep unresolved chess or architecture reasoning in `RESEARCH`.
2. Use the cheapest useful falsifier before expensive coding or games. Search
   prior negative results and do not retry one unless its recorded trigger
   fired.
3. Promote to `READY_FOR_IMPLEMENTATION` only when the mechanism, semantics,
   local evidence, interactions, falsifier and accept/reject rule are frozen.
4. Implementation acts like a colleague on ordinary code structure, builds,
   tests and cheap qualification. It does not redesign or broaden the
   experiment; a false premise returns the leaf to `RESEARCH`.
5. The agent prepares and verifies long tournaments, SPRTs, SPSA, datagen,
   PGO and profiling jobs; the maintainer starts them (about three
   SPRT-sized runs per day).
6. Freeze the prediction before exposure; judge the postmortem against it.
7. A clean negative result is progress. Clusters, not features; compatibility
   over completeness; donor architecture, own implementation.

| State | Boundary / control |
|---|---|
| `RESEARCH` | Evidence, alternatives, interactions, prediction, falsifier and stop rule are being established. |
| `READY_FOR_IMPLEMENTATION` | Research decision frozen; implementation may make ordinary local engineering choices. |
| `IMPLEMENTED` | Intended semantics exist; no qualification claim. |
| `LOCAL_QUALIFIED` | Cheap correctness/performance checks passed; expensive gate prepared. |
| `GAME_GATE` | Registered playing gate running or resolved under maintainer control. |
| `CLOSED` | Accepted, rejected, no-change or deferred disposition and calibration recorded. |

### Current model mapping

PLAN records only stable capability classes. Edit this table when model
generations change; do not rewrite the roadmap. These are maintainer
judgments, not measured rankings. An investigation leaf (`R3`) spawns the
implementation and measurement sub-steps under its step.

| Class | Capability | GPT | Claude |
|---|---|---|---|
| `R3` | Frontier causal/architecture research | GPT-6 Astra — Extra High | Claude Fable 5.1 — High |
| `R2` | Bounded correctness-sensitive reasoning | GPT-5.6 Sol — High | Claude Opus 5 — High |
| `I2` | Difficult implementation | GPT-5.6 Sol — High | Claude Opus 5 — High |
| `I1` | Well-specified implementation | GPT-5.6 Terra — Medium | Claude Sonnet 5 — Medium |
| `M` | Mechanical/docs/provenance | GPT-5.6 Terra — Medium | Claude Sonnet 5 — Medium |
| `V` | Verification/measurement | GPT-5.6 Sol — High | Claude Sonnet 5 — High |

### Reusable research prompt

> Investigate `<PLAN leaf>` as research, not implementation. Read PLAN,
> EXPERIMENTS, HISTORY's number map, linked analysis and relevant source;
> measured evidence outranks roadmap assumptions. Read the donor (Reckless
> first, Stockfish second) for mechanism, population and interaction, never
> for transcription. State the precise question, leading and competing
> hypotheses, shared signals and interactions, and whether search,
> evaluation, tooling or instrument effects could explain it. Design the
> cheapest discriminating test first; freeze its prediction, confidence,
> falsifiers and stop rule before exposure. Spawn the implementation and
> measurement sub-steps the handoff needs under the investigation's step,
> with a class each. Finish `READY_FOR_IMPLEMENTATION`, `MORE_RESEARCH` or
> `NO_CHANGE`, with the evidence for that verdict.

### Reusable implementation prompt

> Implement `<PLAN leaf>` from its registered handoff. Treat the research
> decision, semantics, invariants and experiment design as fixed. Write the
> donor's mechanism in Rarog's own structure; do not transcribe. Use normal
> engineering judgment for code, focused builds/debugging/tests and cheap
> qualification. Do not broaden the mechanism, tune unrelated behaviour or
> continue other roadmap work. If a research premise is false, preserve useful
> instrumentation, document the contradiction and return the leaf to
> `RESEARCH`. Prepare but do not start maintainer-owned expensive jobs. Report
> changes, interactions, validation, remaining gate and false assumptions;
> update PLAN, GUIDE and EXPERIMENTS under their ownership rules.

## Status board

Every phase, step and sub-step is a checkbox here. Rationale and design live
in `PLAN.md`; durable evidence in `EXPERIMENTS.md`; procedures in
`PROCESS.md`; finished work in `HISTORY.md`. `GUIDE.md` and `PLAN.md` change
together, and `python tools/diag/check_guide.py` must pass.

## Current checkpoint

| Item | Value |
|---|---|
| Released baseline | **2.3.2** at `f931722` on `master` |
| Development head | `dev` at `ca8988a`; fingerprint **7,601,220 / EBF 2.474**; accepted by RAR-E15 (+12.12 ± 10.17 Elo); pinned `rustc 1.98.1` since A.3.1 |
| Pool position, `3+0.03` 1T | Houdini 1.5a −216, Critter 1.6a −197, Fritz 16 −161, Rybka 4 −109, Basilisk 1.9.3 −26 (2026-09-04, 2.4.0-dev) |
| Search deficit | **250.8 ± 13.1 Elo** equal time against the frozen oracle; selectivity explains 272 ± 18 |
| Evaluation deficit | **about 329 Elo** against Stockfish's classical HCE with the same search |
| Speed | 3.22 MNPS bench 13, PGO 1T; Basilisk 3.71 |
| Conversion | 57 draws + 12 losses after a persistent piece-up in 2,400 games vs the six HCE-era engines |
| Active experiment | none; RAR-E16 CLOSED (H1, +54.77 ± 17.04 Elo); RAR-R11 registered for A.3.3; RAR-M45, RAR-M46 and RAR-O03 registered for A.5 |
| Current step | **A.3.3 — time-forfeit repair** (agent), then the maintainer runs RAR-R11; release at **A.3.4** as **2.4.0**; then **A.5** baselines on the released binary |
| Next release | **2.4.0** — RAR-E16 read +54.77 ± 17.04, lower bound +37.73, so the rule is met; cut at A.3.4 after A.3.3 and the two 1.98.1 holds; universal binary if A.4 adopts; later 3.0.0 if E.2 is met |

## Next and held work

**Next: A.3.3 time-forfeit repair** (agent diagnoses and fixes; the maintainer
runs the registered RAR-R11 validation). A.3.4 then cuts **2.4.0**, with the
universal asset only if A.4.5 adopted it. A.4 is the agent's parallel work
whenever a maintainer job is occupying the box. The maintainer then runs the A.5 baselines on the released
binary while the agent does A.6 (conversion instrument) and A.7 (consolidation
analysis). **B.0**, the search programme investigation, opens after A.7.

| Open hold / obligation | Resume or resolve when | Must be resolved before |
|---|---|---|
| Windows ARM64 PGO `rust-lld` workaround unverified on the 1.98.1 pin (RAR-P08, RAR-P14) | The ARM64 compatibility host runs `cargo xtask build --arch arm64 --pgo` | A.3.4 publishes assets |
| CI matrix has not run on the 1.98.1 pin (RAR-P15 precedent) | A `workflow_dispatch` of `ci.yml` on the A.3.1 head or later goes green | A.3.4 publishes assets |
| KRPPKRP 7-man truth gap | Independent truth becomes available, or C.5.8 records an explicit exclusion | C.5.8 closes |
| KRP-KB win-preserving 0.9990 → 0.9949 (−2.2 SE, RAR-M42) | Non-blocking; blocking if a later change pushes it past 3 SE | C.5.4 closes (owner) |

Follow the earliest unblocked leaf. Held items stay unticked in place.

## Phase A — Reset: repository, instruments, baselines, consolidation release

Open active leaves show `workflow state / capability class`. Execution order
is the numbering: release first, baselines on the released binary.

- [x] **A.1** Document reset — new PLAN, GUIDE, HISTORY; archives; checker — CLOSED, 2026-09-09
- [x] **A.2** Repository and branch cleanup
    - [x] **A.2.1** Tracked-file cleanup: twelve one-off or superseded files removed, each with its last commit — DONE 2026-09-09
    - [x] **A.2.2** Branch and tag disposition: seven branches tagged and deleted, oracle package archived, stale worktrees removed — DONE 2026-09-09
    - [x] **A.2.3** Feature and option inventory: 42 inert parameters for B.1, 55 seeds for B.2, features kept — DONE 2026-09-09
- [ ] **A.3** Consolidation release before the search programme
    - [x] **A.3.1** Toolchain bump 1.97.1 → 1.98.1, behaviour-neutral: fingerprint, suites, ISA and pooled NPS all clean (RAR-P18) — DONE 2026-09-09
    - [x] **A.3.2** Release gate RAR-E16: **H1 accepted at 742 games, +54.77 ± 17.04 Elo**; 4T check +79.53 ± 21.21, zero forfeits; 2.4.0 licensed — DONE 2026-09-09
    - [ ] **A.3.3** Time-forfeit repair: reconstructed clocks of the seven forfeits, low-time reserve fix, RAR-R11 10,000-game validation — **READY_FOR_IMPLEMENTATION / R2**
    - [ ] **A.3.4** Release 2.4.0 or 2.3.3 per the release rule; universal binary only if A.4.5 adopted it — **RESEARCH / M**
- [ ] **A.4** Universal x86-64 binary: one file per OS selecting its code path at startup
    - [ ] **A.4.1** Design: symbol-isolation link prototype first; fat binary vs kernel multiversioning vs launcher; dispatch table — **READY_FOR_IMPLEMENTATION / R2**
    - [ ] **A.4.2** Prototype as `xtask --arch universal`, isolated; forced-tier override; per-region ISA check — **RESEARCH / I2**
    - [ ] **A.4.3** Compatibility and identity: every tier forced and automatic, bench identity per tier, lifecycle, suites — **RESEARCH / V**
    - [ ] **A.4.4** Performance and size: fixed-node NPS per tier within 1% of the dedicated PGO binary; startup; size — **RESEARCH / V**
    - [ ] **A.4.5** Decision: adopt for A.3.4 with a null pair against the gated pext binary, or defer to G.2 — **RESEARCH / R2**
- [ ] **A.5** Baselines on the release binary
    - [ ] **A.5.1** Reference pool refresh with Houdini 3, 1T, 400 games per pair — RAR-M45 — **READY_FOR_IMPLEMENTATION / V**
    - [ ] **A.5.2** Four-thread pool against the four targets and Basilisk — RAR-M46 — **READY_FOR_IMPLEMENTATION / V**
    - [ ] **A.5.3** Oracle deficit meter G(0), evaluation held constant, 3,000 paired games — RAR-O03 — **READY_FOR_IMPLEMENTATION / V**
    - [ ] **A.5.4** Pooled-PGO NPS baseline, three builds, archived hashes — **READY_FOR_IMPLEMENTATION / V**
- [ ] **A.6** Conversion instrument `tools/diag/conversion_audit.py`; baseline 57/12 vs 40/12 — **READY_FOR_IMPLEMENTATION / I1**
- [ ] **A.7** Codebase consolidation analysis: target layout, B.1 and C.1 handoffs, dead-code list — **RESEARCH / R2**

## Phase B — Search programme (evaluation frozen)

- [ ] **B.0** Investigation: current search vs donor, cluster boundaries, scale ratio, seeds, instruments — **RESEARCH / R3**
- [ ] **B.1** Search restructure, behaviour-neutral: modules, `NodeType`, `StackEntry`; exact fingerprint — **RESEARCH / I1**
- [ ] **B.2** Cluster 1 — selectivity core: TT eval storage, correction, histories, picker, move-loop pruning, LMR — **RESEARCH / I2**
    - [ ] **B.2.1** Implement to the B.0 handoff with table, picker, TT and unwind tests — **RESEARCH / I2**
    - [ ] **B.2.2** Diagnostics: oracle differential, depth at 300k, EBF, tactical suite, 2,000-game unfitted run — **RESEARCH / V**
    - [ ] **B.2.3** SPSA over the registered live coordinates — **RESEARCH / V**
    - [ ] **B.2.4** Gate: SPRT `[0,10]` against the B.1 head; ledger row and calibration — **RESEARCH / V**
- [ ] **B.3** Cluster 2 — NMP, ProbCut, singular/multi-cut/negative/LDSE extensions, IIR policy; SPRT `[0,5]` — **RESEARCH / I2**
- [ ] **B.4** Cluster 3 — quiescence: TT, corrected stand-pat, LMP, SEE margin; SPRT `[0,3]` — **RESEARCH / I2**
- [ ] **B.5** Cluster 4 — root, aspiration, iterative deepening, PV/multi-PV; SPRT `[0,3]` — **RESEARCH / I2**
- [ ] **B.6** Joint search SPSA, only if curvature justifies it — **RESEARCH / V**
- [ ] **B.7** Search speed pass on the new modules; pooled-PGO floor +0.5% per change — **RESEARCH / I1**
- [ ] **B.8** Cleanup: dead parameters, old picker, unconsumed provenance, ownerless diagnostics — **RESEARCH / I1**
- [ ] **B.9** Checkpoint: G(0), depth/EBF, NPS, conversion, pool gauntlet; remove `ablate`; freeze the search head — **RESEARCH / V**

## Phase C — Evaluation programme (search frozen)

- [ ] **C.0** Investigation: family map, residuals, donor conditioning, shared inputs, cluster order, refit protocol — **R3**
- [ ] **C.1** Evaluation restructure, behaviour-neutral: modules, one attack-map producer; exact fingerprint — **I1**
- [ ] **C.2** Datagen and label contract for the programme; corpus frozen under a new name — **V**
- [ ] **C.3** King safety cluster: danger units, safe/unsafe checks, weak ring, flank, shelter/storm; refit; gate — **I2**
- [ ] **C.4** Threats and mobility cluster: mobility area, weak enemies, hanging, restricted, pawn push, queen threats; refit; gate — **I2**
- [ ] **C.5** Endgame handling and winnability cluster — **R3**
    - [ ] **C.5.1** Classification and deciding instrument per family — **R2**
    - [ ] **C.5.2** Generic winnability and scaling: pawn count, opposite bishops, rule-50 scale, complexity — **I2**
    - [ ] **C.5.3** Conversion cluster: KXK, KBNK, KQKR; rule-50 damping interaction measured — **I2**
    - [ ] **C.5.4** Rook versus minor cluster: KRKN, KRKB, KRPKB — **I2**
    - [ ] **C.5.5** Rook and pawn cluster: KRPKR, KRKP, KPK, KPKP audit — **R2**
    - [ ] **C.5.6** Measure-first families: KPsK, KBPsK, KBPPKB, KQKRPs — **R2**
    - [ ] **C.5.7** Theory sweep: KBPKB, KBPKN, KNNKP, KNNK, KQKP from one dispatcher — **I1**
    - [ ] **C.5.8** Endgame gate: endgame-start cohort SPRT plus STC SPRT; floors; conversion; 7-man exclusion — **V**
- [ ] **C.6** Pawns and passers cluster; refit; gate — **I2**
- [ ] **C.7** Material, imbalance, phase and pieces cluster; refit; gate — **I2**
- [ ] **C.8** Refit cycles: regenerate, refit, gate; stop at the first non-accepting cycle — **V**
- [ ] **C.9** HCE SPSA of nonlinear residue, or a written skip — **V**
- [ ] **C.10** Search cp-margin re-fit after the new evaluation; SPRT `[0,3]` — **V**
- [ ] **C.11** Checkpoint: same-search deficit, conversion, NPS, pool gauntlet; freeze the classical evaluation — **V**

## Phase D — Clock, threads, robustness

- [ ] **D.1** Time management: audit, soft/hard bounds with node-fraction multiplier, forfeit margin; SPRT `[0,3]` — **R2**
- [ ] **D.2** Lazy SMP quality at 4T/8T: diversity, shared TT and correction, soft-stop voting; 4T SPRT `[0,5]` — **R2**
- [ ] **D.3** Engine lifecycle and protocol robustness; zero crashes over pool tournaments — **R2**
- [ ] **D.4** Tablebase policy: probing depth/limits, WDL/DTZ in conversion, recogniser interaction — **R2**

## Phase E — Classical checkpoint and release

- [ ] **E.1** Attribution checkpoint: STC, `10+0.1`, 4T against 2.3.2 and the B.9/C.11 heads; maturity checklist — **V**
- [ ] **E.2** Target gate: ≥50% against Critter 1.6a, Houdini 3, Rybka 4 and Fritz 16 at 1T and 4T — **V**
- [ ] **E.3** Release 3.0.0 (gate met) or 2.4.0: changelog, suites, PGO assets, ISA, CI, tag on instruction — **M**
- [ ] **E.4** Universal binary adoption, only if A.4 deferred it — **I1**

## Phase F — NNUE (own data only)

- [ ] **F.0** Investigation: board events, accumulator ownership, trainer choice, data format, first architecture — **R3**
- [ ] **F.1** Board events and accumulator scaffolding, behaviour-neutral for HCE; cost ledger — **I2**
- [ ] **F.2** Data generation at scale: 30–60M unique positions, splits, manifests, hashes — **V**
- [ ] **F.3** Trainer hardening and baseline nets, two seeds per configuration — **I2**
- [ ] **F.4** Scalar integration: `quantised.bin` contract, integer-exact conformance, HCE fallback — **I2**
- [ ] **F.5** Incremental and SIMD: same-net parity on every move type, tiers, pooled NPS attribution — **I2**
- [ ] **F.6** Search re-fit for the network — **V**
- [ ] **F.7** Architecture ladder: output buckets, king buckets, relation/threat inputs; one axis at a time — **R3**
- [ ] **F.8** Data frontier: on-policy refresh, deduplication, hard-position mining — **V**
- [ ] **F.9** NNUE release: beat the classical release at STC, LTC and 4T; platform matrix — **M**
- [ ] **F.10** CCRL top-100 gate — **V**

## Phase G — Scaling, platforms and the top 50

- [ ] **G.1** High-thread and NUMA: 8/16/32T, TT and net placement, large pages, affinity policy — **R2**
- [ ] **G.2** Platform and product: universal dispatch, Chess960 on demand, distributed testing — **I1**
- [ ] **G.3** Frontier: larger nets, data scaling, LTC search fit; CCRL top-50 gate — **R3**
