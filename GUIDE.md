# Rarog development guide

**Status board only: every phase, step and sub-step with a checkbox.**
Nothing else belongs here. Rationale and design live in `PLAN.md`; durable
evidence in `EXPERIMENTS.md`; repeatable procedures in `PROCESS.md`; finished
history in `TRACKER.md`. `GUIDE.md` and `PLAN.md` change in the same commit,
and `python tools/diag/check_guide.py` must pass.

## Current checkpoint

| Item | Value |
|---|---|
| Released baseline | **2.3.2** at `f931722` on `master` |
| Accepted head | RAR-E12 + 4.9a.7, **6,901,489 nodes / EBF 2.458**. Includes the 4.9a.4 mate drive, which is bench-INVISIBLE |
| Integration branch | `dev`; the hce-v3 refit `d1d95ab` is accepted |
| Frozen oracle | `hybrid` at `75d0d43`; never merge it into Rarog |
| Measured search deficit | **355.26 +/- 27.03 Elo** equal nodes; **250.77 +/- 13.12** equal time; speed worth **104.5 Elo** |
| Accepted Phase-4 gains | ProbCut **+15.56 +/- 10.02**; root LMR relief **+2.33 +/- 1.85**; HCE refit **+22.04 +/- 7.51**; TB-corrected labels **+6.73 +/- 3.82**; hce-v3 refit **+11.81 +/- 5.33** |
| Active experiment | none |
| Instrument state | **4.10 repaired; v2 baselines/floors recorded.** Budget transfer and remaining corrections: 4.11.7–4.11.10 |
| Current step | **4.11.8 — datagen label audit**; 4.11.7 held for scheduled compute |
| Next release | Conditional 2.4.0 at 4.20; NNUE follows either way |

## Next and held work

**Next: 4.11.8 — audit datagen labels; zero games.** Then 4.11.9 and 4.11.10
where independent. 4.11.7 has not moved or been completed.

| Open hold | Resume when | Must be resolved before |
|---|---|---|
| **4.11.7** Budget transfer | Maintainer schedules the 60k/200k/600k run | 4.11 closes and 4.11b starts |
| **4.12.21** Future 7-man evidence gap | Independent truth/verification becomes available, or a justified exclusion is recorded | 4.12.23 closes |

Follow the earliest **unblocked** leaf. Keep held items unticked in their
original place; check their return conditions at every handoff. PLAN owns
dependencies and detail. The agent maintains this overview and reports the
next step; the maintainer does not need to run a queue script.

## Phase 4 — bounded pre-NNUE search and HCE

- [x] **4.0** Evidence, baseline and oracle freeze — RAR-M12
- [x] **4.1** Instrumented oracle — `hybrid-diag` `de568b3`
- [x] **4.2** Differential observation harness — RAR-S55
- [x] **4.2a** Harness and instrument integrity sweep — Basilisk-derived
    - [x] **4.2a.1** `sprt.ps1` options-free repair and `-NoAdjudication` wire proof — `cb5ed2a`
    - [x] **4.2a.2** Exit-status sweep: no unguarded native call found; already protected
    - [x] **4.2a.3** Anomaly guard rate-limited so it discriminates instead of voiding every gate — `334c084`
    - [x] **4.2a.4** `sprt.ps1` refuses options its mode cannot honor — `3fb9f57`
- [x] **4.2b** Time-forfeit diagnosis at test concurrency — RAR-M14; fixes belong to 4.17
    - [x] **4.2b.1** Games end at 97-99% of clock; ~2% aggregate slack, ~100ms per game
- [x] **4.3** Mechanism map and order freeze
- [x] **4.4** Matched ablation plus fixed-node correction
- [x] **4.5** LMR contract study — no interior gain; RAR-S70 root gain retained
- [x] **4.6** Shallow-selectivity/rewrite continuation — no accepted gain
    - [x] **4.6.1** Quiet SEE screen: gap closure **+3.38 +/- 27.08**, stopped null
    - [x] **4.6.2** SearchCore: **-9.76 +/- 17.70** over 712 games, reverted
    - [x] **4.6.3** Broad selectivity SPSA and another rewrite declined
- [x] **4.7** Qualify HCE data, labels, instruments and current-source maturity
    - [x] **4.7.1** Archive audit: 600k independent starts; capacity 2.30M/127,778/127,778
    - [x] **4.7.2** Hash-frozen `hce-v2`: pure self-play WDL from 750k unique openings
    - [x] **4.7.3** Exact 1,218-slot instrument partition and vector/bake/rebuild smoke
    - [x] **4.7.4** Current-source Stockfish maturity map
- [x] **4.8** Refit and gate the complete existing HCE surface
    - [x] **4.8.1** Fresh 150k-game confirmation: test loss **0.12252203** vs **0.12330291**
    - [x] **4.8.2** Baked at **7,226,051 / 2.460**, NPS **-1.19%**; RAR-E06 registered
    - [x] **4.8.3** RAR-E06 **ACCEPTED**: H1 at 3,914 games, **+22.04 +/- 7.51 Elo**, +32.05 nElo
- [x] **4.8a** Post-refit redundancy removal — **closed, no gate owed**; RAR-E07
    - [x] **4.8a.1** Inventory: 5 slots zeroed, 90 of 132 sparse slots structurally unreachable
    - [x] **4.8a.2** No removal cluster exists; 3 inert 1-slot terms handed to 4.18.2
- [x] **4.9** Structural HCE clusters — **closed, none opened**; RAR-E09
    - [x] **4.9.1** Residual audit — RAR-E09: no structural residual; a label defect instead
    - [x] **4.9.2** Closed: no cohort residual licenses a cluster; retry trigger recorded
- [x] **4.9a** Endgame truth foundation — done; five results SUPERSEDED, owners named
    - [x] **4.9a.1** Truth corpus — result was invalid, REPLACED by the 4.11.1 baseline
    - [x] **4.9a.2** Endgame-start cohort book — 788 verified positions, 21 families
    - [x] **4.9a.3** Regression contract — floors REPLACED by 4.11.2; the 64 theory vetoes always stood
    - [x] **4.9a.4** SUPERSEDED -> 4.11.9 — gain stands (KBN-K **19.4% -> 96.9%**); isolation reaches 6 families
    - [x] **4.9a.5** RAR-E08 **ACCEPTED** +6.73 +/- 3.82 Elo — TB-corrected labels win
    - [x] **4.9a.6** `hce-v3-tb` fitted and gated — RAR-E12 **+11.81 +/- 5.33 Elo**
    - [x] **4.9a.7** SUPERSEDED -> 4.12.2 — KRPKR drawn overclaim **37.1% -> 25.8%** stands; conversion half does not
    - [x] **4.9a.8** SUPERSEDED -> 4.12.6 — KRPKB drawn-cohort null stands; conversion half does not
- [x] **4.10** Instrument integrity and tooling upgrade — 12 leaves, complete
    - [x] **4.10.1** Tablebase-truth termination; material shed becomes a diagnostic
    - [x] **4.10.2** Cohort fingerprint; refuse any comparison across position sets
    - [x] **4.10.3** Sharded workers; parallel output byte-identical to serial
    - [x] **4.10.4** Prove every guard FAILS on a known-bad input; thin-sample refusal
    - [x] **4.10.5** Measurement-layer contract and per-report layer/budget/set fields
    - [x] **4.10.6** Node budget as a run condition; 60k/200k/600k bracket runner
    - [x] **4.10.7** Held-out split, McNemar paired z, runner-up slot, spent-cohort rule
    - [x] **4.10.8** `datagen_label_audit.py` — corpus labels against tablebase truth
    - [x] **4.10.9** Gate runner refuses wrong revision, dirty tree or mismatched bench
    - [x] **4.10.10** `check_guide.py` enforces SUPERSEDED owners and compares step sets
    - [x] **4.10.11** Compile-time bound on the shipped mop-up constants, every build type
    - [x] **4.10.12** Feature-matrix build audit; `--all-features` is not the shipped config
- [ ] **4.11** Re-measurement and re-derivation — corrections recorded in place
    - [x] **4.11.1** Re-run both truth arms — head **0.9300**, reference **0.9920**
    - [x] **4.11.2** Floors re-derived — **0.9300** over n=1371, 18 families, cohort-stamped
    - [x] **4.11.3** Attained reference results frozen — **1361/1372**, hard residue **8**
    - [x] **4.11.4** Drawn-share census — **KR-KN 1.000, KR-KB 0.996** at +346/+307 cp
    - [x] **4.11.5** Occurrence split by root — **3 of 40 roots give 56%** of the census
    - [x] **4.11.6** 4.12 re-ranked and REGISTERED; leaves renumbered to match
    - [ ] **4.11.7** HELD — budget transfer; resume on maintainer-scheduled compute
    - [ ] **4.11.8** Datagen label audit on `hce-v2` and `hce-v3-tb`
    - [ ] **4.11.9** Mate-drive blast radius over the dispatcher's promotion closure
    - [ ] **4.11.10** Restate RAR-E08, RAR-E11 and RAR-E12's conversion claims as superseded
    - [x] **4.11.11** Panic reported on stdout, where the harness keeps it
    - [x] **4.11.12** Occurrence re-measured over 36,400 rated games; 4.12 re-ranked to **v2**
- [ ] **4.11b** Board correctness and HCE throughput
    - [x] **4.11b.1** Freeze the board audit and three-engine comparison
    - [ ] **4.11b.2** Strengthen benchmark coverage and correctness oracles
    - [ ] **4.11b.3** Repair move parsing and counter boundaries
    - [ ] **4.11b.4** Define SEE contracts and independent fixtures
    - [ ] **4.11b.5** Repair the SEE king-exchange defect
    - [ ] **4.11b.6** Add neutral value injection; restore comparable SEE timing
    - [ ] **4.11b.7** Profile board work in HCE search
    - [ ] **4.11b.8** Optimize legal generation and move-list delivery
    - [ ] **4.11b.9** Measure fused piece relocation
    - [ ] **4.11b.10** Share useful pin/check information
    - [ ] **4.11b.11** Optimize the corrected SEE kernel
    - [ ] **4.11b.12** Decide whether king-square caching pays
    - [ ] **4.11b.13** Define history capacity and mutation contracts
    - [ ] **4.11b.14** Decide whether a larger representation change pays
    - [ ] **4.11b.15** Review draw/null/repetition policy boundaries
    - [ ] **4.11b.16** Qualify integrated correctness and throughput
    - [ ] **4.11b.17** Register and qualify the playing cluster
    - [ ] **4.11b.18** Refresh affected endgame evidence and close
- [ ] **4.12** Endgame reference functions — order registered by 4.11.6, re-derived at 4.11.12
    - [ ] **4.12.1** Adopt the order; confirm recognizer-vs-scale classification per family
    - [ ] **4.12.2** KRPKR [ref 13] scale — 30.7% overclaim remains after 4.9a.7
    - [ ] **4.12.3** KXK [ref 3] verdict — largest occurrence in the set (37.8%); mechanism at 4.9a.4
    - [ ] **4.12.4** KRKN [ref 8] scale — **100%** overclaim at +346; Rarog reaches it 1.6x the pool rate
    - [ ] **4.12.5** KRKB [ref 7] scale — **99.6%** overclaim at +307; same 1.6x over-representation
    - [ ] **4.12.6** KRPKB [ref 14] scale — **100%** overclaim at +328 after 4.9a.8's rook pawns
    - [ ] **4.12.7** KBPKB [ref 17] scale — 60.9% overclaim at +142
    - [ ] **4.12.8** KRKP [ref 6] scale — 26.4% overclaim at +72
    - [ ] **4.12.9** KBPKN [ref 19] scale — 50.7% overclaim; mate-drive debt from 4.11.9
    - [ ] **4.12.10** KQKR [ref 10] verdict — deficit 23 — **0.63% of Rarog's games, not zero**
    - [ ] **4.12.11** KPK [ref 5] scale — 4.6% overclaim; present bitbase
    - [ ] **4.12.12** KPKP [ref 20] scale — 3.8% overclaim, nearly clean
    - [ ] **4.12.13** KQKP [ref 9] verdict — owns RAR-E08's KQ-KP debt
    - [ ] **4.12.14** KBNK [ref 4] verdict — owns RAR-E12's dtz debt, target 0.7260
    - [ ] **4.12.15** KNNKP [ref 2] scale — 57.7% overclaim; holds 7 of the 8 hard residue
    - [ ] **4.12.16** KNNK [ref 1] scale — **no defect measured** — close it
    - [ ] **4.12.17** KPsK [ref 16] ? — **MEASURE FIRST** — 4.52% of Rarog's games, never measured
    - [ ] **4.12.18** KBPsK [ref 11] ? — **MEASURE FIRST** — 2.59% of Rarog's games, never measured
    - [ ] **4.12.19** KBPPKB [ref 18] ? — **MEASURE FIRST** — 0.50%, never measured
    - [ ] **4.12.20** KQKRPs [ref 12] ? — **MEASURE FIRST** — 0.42% of games and 4.41% of the TREE
    - [ ] **4.12.21** KRPPKRP [ref 15] ? — **5.40% of games and UNVERIFIABLE** — 7 men; record the gap
    - [ ] **4.12.22** Dependency-complete family refits and gates, tiered by occurrence
    - [ ] **4.12.23** Conversion, theory, STC/LTC and endgame-cohort closure
- [ ] **4.13** Datagen label truth and corpus contract
    - [ ] **4.13.1** Audit data/label pipeline; quantify contradictions
    - [ ] **4.13.2** Relabel and whole-game adjudication as separate registered arms
    - [ ] **4.13.3** Record that more datagen nodes is the weak fix; do not buy it
    - [ ] **4.13.4** Freeze the winning contract under a new corpus name
- [ ] **4.13a** ANALYSIS — HCE correctness, feature interactions and throughput
- [ ] **4.14** Iterated whole-surface refit cycles — at least one is owed
    - [ ] **4.14.1** Initialization control: neutral start vs accepted start, offline
    - [ ] **4.14.2** Opening supply: reuse is clean; fresh starts are a nice-to-have
    - [ ] **4.14.3** Composition screen against the `datagen-v1` archive (sizing)
    - [ ] **4.14.4** Regenerate and hash-freeze under a new corpus name
    - [ ] **4.14.5** Cycle 1: full 4.8 schedule, own frozen test, registered gate
    - [ ] **4.14.6** Repeat while a cycle accepts; stop at the first that does not
    - [ ] **4.14.7** Record cycles, refresh the residual audit and close
- [ ] **4.15** Search composition, throughput and score authority
    - [ ] **4.15.1** ANALYSIS — search/ordering/pruning interactions and throughput
    - [ ] **4.15.2** One candidate and gate, only if 4.15.1 isolates a unique defect
    - [ ] **4.15.3** Audit the SEE scale after final HCE; zero games
    - [ ] **4.15.4** Fit justified SEE values through the existing interface; gate
    - [ ] **4.15.5** Revalidate normalized SEE timing after fitting
- [ ] **4.15a** ANALYSIS — TT, caches, hashing and memory layout
- [ ] **4.15b** ANALYSIS — threads, engine/UCI lifecycle and tablebases
- [ ] **4.15c** ANALYSIS — diagnostics, harnesses and build/ISA delivery
- [ ] **4.16** Optional post-HCE search SPSA; skip without a displaced optimum
- [ ] **4.17** Time management: review, repair and gate — owns all TM work
    - [ ] **4.17.1** ANALYSIS — clock/search/root interactions; revalidate behavior
    - [ ] **4.17.2** `Move Overhead` vs forfeit rate on a null pair; size it first — RAR-M14
    - [ ] **4.17.3** `RootConfTime`'s six identifiable consumers: tune or remove — RAR-S47
    - [ ] **4.17.4** Root-instability TM from a completed snapshot only — RAR-X06, RAR-R05
    - [ ] **4.17.5** Registered gate; zero forfeits is a precondition, not the verdict
- [ ] **4.18** Search cleanup and clean checkpoint
    - [ ] **4.18.1** Audit coverage/interaction closure; dead-path inventory
    - [ ] **4.18.2** Remove unconsumed 4.6 alternatives, plus the 3 terms RAR-E07 left inert
    - [ ] **4.18.3** Re-verify tests, clippy, fingerprint, NPS and deficits
- [ ] **4.19** Final HCE/search checkpoint, attribution and maturity closure
- [ ] **4.20** STC/LTC/4T, NPS, portability, ISA and release gate

## Phase 5 — NNUE runway

- [ ] **5.1** Measurement corpus handoff; freeze 4.7 splits and manifests
- [ ] **5.2** Per-ply state and dirty pieces
    - [ ] **5.2.1** Freeze board baselines, including Reckless; start the cost ledger
    - [ ] **5.2.2** Define factual move deltas and callback timing
    - [ ] **5.2.3** Add the update interface and ownership
    - [ ] **5.2.4** Verify every transition independently
    - [ ] **5.2.5** Qualify HCE behavior and measure move-event costs
- [ ] **5.3** Accumulator scaffolding
    - [ ] **5.3.1** Allocate evaluator-owned per-thread/per-ply state
    - [ ] **5.3.2** Define validity, refresh and null semantics
    - [ ] **5.3.3** Verify scaffolding against full refresh
    - [ ] **5.3.4** Gate HCE behavior and measure scaffold costs
- [ ] **5.4** Trainer preflight: pin `D:/code/net_trainer`, Bullet, toolchain, GPU
- [ ] **5.5** Runway gate: fingerprint, debug/release, unwind, pilot corpus
- [ ] **5.6** Threat-map hooks, optional; reserve only to avoid a second rewrite

## Phase 6 — baseline NNUE

- [ ] **6.0** Trainer hardening: strict CLI, deterministic splits, hashes, seeds
- [ ] **6.1** Controlled data: 30-60M unique positions, by-game splits, manifests
- [ ] **6.2** Baseline networks: at least two seeds per width/bucket configuration
- [ ] **6.3** Scalar integration: `quantised.bin` contract, integer-exact conformance
- [ ] **6.4** Incremental and SIMD parity, performance and cost attribution
    - [ ] **6.4.1** Prove actual-network incremental/full-refresh parity
    - [ ] **6.4.2** Qualify SIMD, integer bounds and supported targets
    - [ ] **6.4.3** Measure board/update/inference costs; gate whole-search NPS
- [ ] **6.5** Architecture loop: output buckets, king buckets, then relation inputs
- [ ] **6.6** Gross search-scale safety; broad search fitting waits for 7.3
- [ ] **6.7** Baseline release: beat the pre-NNUE master at STC/LTC and 4T

## Phase 7 — NNUE frontier and final search fit

- [ ] **7.0** Residual and disagreement analysis by phase, material, king, cohort
- [ ] **7.1** Data frontier: scale, deduplicate, mine hard positions, refresh on policy
- [ ] **7.2** Architecture ladder, one axis at a time
- [ ] **7.3** Re-audit NNUE/search interactions; fit justified displaced constants
- [ ] **7.4** Frontier gate against 2.3.2, the Phase-4 head and target engines

## Phase 8 — scaling, platforms and product completeness

- [ ] **8.0** High-thread and NUMA: price depth diversity at 4/8/16T
- [ ] **8.1** Runtime dispatch, TT/net placement and large pages
- [ ] **8.2** Product/platform: demand-led Chess960, distributed testing
- [ ] **8.3** Scaling release: topology, clock, net, ISA and user-doc gate

## Phase 9 — optional post-NNUE classical fallback

Enter only if serious NNUE work fails and the maintainer abandons NNUE.

- [ ] **9.1** King-safety semantic rework
- [ ] **9.2** Material-specific winnability and scaling
- [ ] **9.3** Passer and pawn conditionality
- [ ] **9.4** Threat and usable-activity conditionality
- [ ] **9.5** Material/phase specialization, last classical step only
