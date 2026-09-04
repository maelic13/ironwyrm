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
| Instrument state | **The endgame truth harness is defective and under repair.** Every pawn-family conversion number is superseded; bare-king families are provably unaffected |
| Current step | **4.10.10 — checker enforces SUPERSEDED owners and step sets** |
| Next release | Conditional 2.4.0 at 4.20; NNUE follows either way |

## What you run next

**4.10.10 — teach `check_guide.py` the `SUPERSEDED -> <leaf>` marker**: it may
sit only on a ticked leaf, must name a leaf that exists, and that leaf must be
unticked. Also compare the GUIDE and PLAN step SETS, not just membership — a
seven-sub-step PLAN item was listed with five in GUIDE and nothing caught it.
Tooling; no engine change.

**Deployment is 153,466 nodes/move median at 3+0.03** and the endgame screen is
60,000, below its p25 — so every fixed-node endgame verdict so far is
PROVISIONAL. Evidence: `analysis/node_budget_2026-09-04.md`.

The standard cohort is frozen by content: seed `6200600`, 19 families, 100
positions each, overall digest `fe486604...`. Both of 4.11.1's arms must report
it, and 4.11.1 can now run `--workers N`.

Phase 4's open work was reordered on 2026-09-04 — **instruments (4.10), then
re-measurement (4.11), then development (4.12 onward)**. PLAN section 13 maps
the old numbers to the new ones. Nothing in 4.12 may be ordered or gated on a
number the old instrument produced.

**Run this board strictly top to bottom: the first unticked box is the next
leaf, always.** A completed step whose RESULT was invalidated stays ticked and
carries `SUPERSEDED -> <leaf>` naming the open leaf that repairs it — an open
box that only a later leaf can discharge would make the board unrunnable, which
is why the five 4.9a entries are marked rather than un-ticked.

Do not read the queue off the board by eye — it is 146 lines. Generate it:

```bash
python tools/diag/check_guide.py --next 10
```

```bash
python -m unittest discover -s tools/diag -p "test_*.py" && python tools/diag/check_guide.py
```

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
    - [x] **4.9a.1** SUPERSEDED -> 4.11.1 — truth corpus; the material abort invalidated every pawn family
    - [x] **4.9a.2** Endgame-start cohort book — 788 verified positions, 21 families
    - [x] **4.9a.3** SUPERSEDED -> 4.11.2 — floors half only; the 64 frozen theory vetoes stand
    - [x] **4.9a.4** SUPERSEDED -> 4.11.9 — gain stands (KBN-K **19.4% -> 96.9%**); isolation reaches 6 families
    - [x] **4.9a.5** RAR-E08 **ACCEPTED** +6.73 +/- 3.82 Elo — TB-corrected labels win
    - [x] **4.9a.6** `hce-v3-tb` fitted and gated — RAR-E12 **+11.81 +/- 5.33 Elo**
    - [x] **4.9a.7** SUPERSEDED -> 4.12.2 — KRPKR drawn overclaim **37.1% -> 25.8%** stands; conversion half does not
    - [x] **4.9a.8** SUPERSEDED -> 4.12.3 — KRPKB drawn-cohort null stands; conversion half does not
- [ ] **4.10** Instrument integrity and tooling upgrade — tooling commits, no engine change
    - [x] **4.10.1** Tablebase-truth termination; material shed becomes a diagnostic
    - [x] **4.10.2** Cohort fingerprint; refuse any comparison across position sets
    - [x] **4.10.3** Sharded workers; parallel output byte-identical to serial
    - [x] **4.10.4** Prove every guard FAILS on a known-bad input; thin-sample refusal
    - [x] **4.10.5** Measurement-layer contract and per-report layer/budget/set fields
    - [x] **4.10.6** Node budget as a run condition; 60k/200k/600k bracket runner
    - [x] **4.10.7** Held-out split, McNemar paired z, runner-up slot, spent-cohort rule
    - [x] **4.10.8** `datagen_label_audit.py` — corpus labels against tablebase truth
    - [x] **4.10.9** Gate runner refuses wrong revision, dirty tree or mismatched bench
    - [ ] **4.10.10** `check_guide.py` enforces SUPERSEDED owners and compares step sets
    - [ ] **4.10.11** Compile-time bound on the shipped mop-up constants, every build type
    - [ ] **4.10.12** Feature-matrix build audit; `--all-features` is not the shipped config
- [ ] **4.11** Re-measurement and re-derivation — corrections recorded in place
    - [ ] **4.11.1** Re-run both truth arms, binaries pinned by SHA-256, per-position on
    - [ ] **4.11.2** Re-derive the floors; restate 4.12.21's KBN-K target on a real artifact
    - [ ] **4.11.3** Re-derive the attained reference results — replaces RAR-E11
    - [ ] **4.11.4** Re-rank 4.12 on corrected conversion, drawn-share bias and occurrence
    - [ ] **4.11.5** Budget transfer: repeat decisive verdicts at 60k/200k/600k
    - [ ] **4.11.6** Occurrence census with endgame roots excluded; report both numbers
    - [ ] **4.11.7** Drawn-share bias census per material class across the corpus
    - [ ] **4.11.8** Datagen label audit on `hce-v2` and `hce-v3-tb`
    - [ ] **4.11.9** Mate-drive blast radius over the dispatcher's promotion closure
    - [ ] **4.11.10** Restate RAR-E08, RAR-E11 and RAR-E12's conversion claims as superseded
- [ ] **4.12** Endgame reference functions — order registered by 4.11.4
    - [ ] **4.12.1** Adopt the order; confirm recognizer-vs-scale classification per family
    - [ ] **4.12.2** KRPKR [ref 13] scale — reopened conversion half
    - [ ] **4.12.3** KRPKB [ref 14] scale — reopened conversion half
    - [ ] **4.12.4** KPsK [ref 16] scale — absent
    - [ ] **4.12.5** KPK [ref 5] verdict — present bitbase
    - [ ] **4.12.6** KRKP [ref 6] verdict — partial
    - [ ] **4.12.7** KBPsK [ref 11] scale — partial wrong-corner subset
    - [ ] **4.12.8** KPKP [ref 20] scale — absent
    - [ ] **4.12.9** KQKP [ref 9] verdict — partial fortress; owns RAR-E08's KQ-KP debt
    - [ ] **4.12.10** KBPKB [ref 17] scale — absent
    - [ ] **4.12.11** KBPPKB [ref 18] scale — absent
    - [ ] **4.12.12** KRKN [ref 8] verdict — absent
    - [ ] **4.12.13** KRKB [ref 7] verdict — absent
    - [ ] **4.12.14** KBPKN [ref 19] scale — absent; carries a mate-drive debt from 4.11.9
    - [ ] **4.12.15** KNNKP [ref 2] verdict — absent
    - [ ] **4.12.16** KNNK [ref 1] verdict — present; drawn-subset cohort only
    - [ ] **4.12.17** KQKR [ref 10] verdict — absent
    - [ ] **4.12.18** KQKRPs [ref 12] scale — absent; 2nd in the search tree and verifiable
    - [ ] **4.12.19** KRPPKRP [ref 15] scale — 7 men, UNVERIFIABLE; record as a gap
    - [ ] **4.12.20** KXK [ref 3] verdict — present; mechanism at 4.9a.4
    - [ ] **4.12.21** KBNK [ref 4] verdict — present; owns RAR-E12's dtz debt
    - [ ] **4.12.22** Dependency-complete family refits and gates, tiered by occurrence
    - [ ] **4.12.23** Conversion, theory, STC/LTC and endgame-cohort closure
- [ ] **4.13** Datagen label truth and corpus contract
    - [ ] **4.13.1** Quantify contradicted rows by family and datagen budget
    - [ ] **4.13.2** Relabel and whole-game adjudication as separate registered arms
    - [ ] **4.13.3** Record that more datagen nodes is the weak fix; do not buy it
    - [ ] **4.13.4** Freeze the winning contract under a new corpus name
- [ ] **4.14** Iterated whole-surface refit cycles — at least one is owed
    - [ ] **4.14.1** Initialization control: neutral start vs accepted start, offline
    - [ ] **4.14.2** Opening supply: reuse is clean; fresh starts are a nice-to-have
    - [ ] **4.14.3** Composition screen against the `datagen-v1` archive (sizing)
    - [ ] **4.14.4** Regenerate and hash-freeze under a new corpus name
    - [ ] **4.14.5** Cycle 1: full 4.8 schedule, own frozen test, registered gate
    - [ ] **4.14.6** Repeat while a cycle accepts; stop at the first that does not
    - [ ] **4.14.7** Record the cycle table and close
- [ ] **4.15** Re-measure qsearch/TT/eval authority and branching on the accepted HCE
    - [ ] **4.15.1** Observation, baseline and live-wire proof; write the analysis
    - [ ] **4.15.2** One candidate and gate, only if 4.15.1 isolates a unique defect
- [ ] **4.16** Optional post-HCE search SPSA; skip without a displaced optimum
- [ ] **4.17** Time management: review, repair and gate — owns all TM work
    - [ ] **4.17.1** Revalidate accepted clock behavior on the accepted HCE — RAR-R01/R02
    - [ ] **4.17.2** `Move Overhead` vs forfeit rate on a null pair; size it first — RAR-M14
    - [ ] **4.17.3** `RootConfTime`'s six identifiable consumers: tune or remove — RAR-S47
    - [ ] **4.17.4** Root-instability TM from a completed snapshot only — RAR-X06, RAR-R05
    - [ ] **4.17.5** Registered gate; zero forfeits is a precondition, not the verdict
- [ ] **4.18** Search cleanup and clean checkpoint
    - [ ] **4.18.1** Dead and unreachable mechanism inventory — Basilisk-derived
    - [ ] **4.18.2** Remove unconsumed 4.6 alternatives, plus the 3 terms RAR-E07 left inert
    - [ ] **4.18.3** Re-verify tests, clippy, fingerprint, NPS and deficits
- [ ] **4.19** Final HCE/search checkpoint, attribution and maturity closure
- [ ] **4.20** STC/LTC/4T, NPS, portability, ISA and release gate

## Phase 5 — NNUE runway

- [ ] **5.1** Measurement corpus handoff; freeze 4.7 splits and manifests
- [ ] **5.2** Per-ply state and dirty pieces with randomized make/unmake parity
- [ ] **5.3** Accumulator scaffolding; HCE active and fingerprint identical
- [ ] **5.4** Trainer preflight: pin `D:/code/net_trainer`, Bullet, toolchain, GPU
- [ ] **5.5** Runway gate: fingerprint, debug/release, unwind, pilot corpus
- [ ] **5.6** Threat-map hooks, optional; reserve only to avoid a second rewrite

## Phase 6 — baseline NNUE

- [ ] **6.0** Trainer hardening: strict CLI, deterministic splits, hashes, seeds
- [ ] **6.1** Controlled data: 30-60M unique positions, by-game splits, manifests
- [ ] **6.2** Baseline networks: at least two seeds per width/bucket configuration
- [ ] **6.3** Scalar integration: `quantised.bin` contract, integer-exact conformance
- [ ] **6.4** Incremental and SIMD parity, bit identity, pooled-PGO NPS gate
- [ ] **6.5** Architecture loop: output buckets, king buckets, then relation inputs
- [ ] **6.6** Gross search-scale safety; broad search fitting waits for 7.3
- [ ] **6.7** Baseline release: beat the pre-NNUE master at STC/LTC and 4T

## Phase 7 — NNUE frontier and final search fit

- [ ] **7.0** Residual and disagreement analysis by phase, material, king, cohort
- [ ] **7.1** Data frontier: scale, deduplicate, mine hard positions, refresh on policy
- [ ] **7.2** Architecture ladder, one axis at a time
- [ ] **7.3** One post-NNUE search fit over demonstrably displaced coordinates
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
