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
| Accepted head | RAR-E06, **7,226,051 nodes / EBF 2.460** (was RAR-S70 6,977,070 / 2.466) |
| Integration branch | `dev`; the complete HCE refit `5188eca` is accepted |
| Frozen oracle | `hybrid` at `75d0d43`; never merge it into Rarog |
| Measured search deficit | **355.26 +/- 27.03 Elo** equal nodes; **250.77 +/- 13.12** equal time; speed worth **104.5 Elo** |
| Accepted Phase-4 gains | ProbCut **+15.56 +/- 10.02**; root LMR relief **+2.33 +/- 1.85**; complete HCE refit **+22.04 +/- 7.51** |
| Active experiment | none; RAR-E06 accepted 2026-09-01 |
| Current step | **4.9 / 4.9a — structural HCE residuals and endgame closure** |
| Next release | Conditional 2.4.0 at 4.15; NNUE follows either way |

## What you run next

Work Phase 4 in the order the list is written. Two items sit before the
endgame families and both are zero-game analysis:

- **4.9.1** post-fit residual/cohort analysis, which is 4.9's missing entry
  evidence and may close 4.9 outright.
- **4.9a.2** the no-adjudication endgame-start cohort, without which the
  families that never arise from UHO openings cannot be measured at all.

Then 4.9a.5 (KRPKR) leads the families: 52% conversion at 10.04% of games.

Rebuild the truth corpus for any candidate with the same seed and compare
per-position, never by differencing aggregates:

```powershell
python tools/diag/endgame_truth.py --engine <candidate.exe> --syzygy D:\chess	ablebases\syzygy3456 --positions 100 --nodes 60000 --max-plies 100 --per-position --output tools
esults\<run>\endgame-truth.json
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
- [x] **4.2b** Time-forfeit diagnosis at test concurrency — RAR-M14; fixes belong to 4.12a
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
    - [x] **4.8a.2** No removal cluster exists; 3 inert 1-slot terms handed to 4.13.2
- [ ] **4.9** At most two residual-selected structural HCE clusters
    - [ ] **4.9.1** Produce the entry evidence: post-fit residual/cohort analysis
    - [ ] **4.9.2** Open at most two clusters, or close 4.9 with the reason recorded
- [ ] **4.9a** Endgame conversion and reference-function closure — worked in order
    - [x] **4.9a.1** Syzygy truth corpus and per-move grading — `endgame_truth.py`
    - [ ] **4.9a.2** No-adjudication endgame-start cohort games
    - [ ] **4.9a.3** Regression contract: hard theory vetoes, aggregate floors
    - [ ] **4.9a.4** Gradient magnitude vs pruning margins — KBNK/KXK, passer approach
    - [ ] **4.9a.5** KRPKR scale [ref 13] — absent; conv **52%**, 10.04% of games
    - [ ] **4.9a.6** KXK value [ref 3] — present; KQK/KRK/KBBK 94/91/86%, 37.34%
    - [ ] **4.9a.7** KRPKB scale [ref 14] — absent; conv 56%, 1.23%
    - [ ] **4.9a.8** KPsK scale [ref 16] — absent; 4.19%
    - [ ] **4.9a.9** KPK value [ref 5] — present bitbase; conv 95%, 2.84%
    - [ ] **4.9a.10** KRKP value [ref 6] — partial; conv 93%, 2.40%
    - [ ] **4.9a.11** KBPsK scale [ref 11] — partial wrong-corner subset; 1.92%
    - [ ] **4.9a.12** KPKP scale [ref 20] — absent; 1.23%
    - [ ] **4.9a.13** KQKP value [ref 9] — partial fortress; conv 96%, 1.17%
    - [ ] **4.9a.14** KBPKB scale [ref 17] — absent; conv 81%, 0.89%
    - [ ] **4.9a.15** KBPPKB scale [ref 18] — absent; conv 79%, 0.66%
    - [ ] **4.9a.16** KRKN value [ref 8] — absent; conv 83%, win-preserving 0.973
    - [ ] **4.9a.17** KRKB value [ref 7] — absent; conv 94%, win-preserving 0.963
    - [ ] **4.9a.18** KBNK value [ref 4] — present; conv **7%**, 0.28%; see 4.9a.4
    - [ ] **4.9a.19** KBPKN scale [ref 19] — absent; conv 79%, 0.28%
    - [ ] **4.9a.20** KNNKP value [ref 2] — absent; discards the win on **19%** of moves
    - [ ] **4.9a.21** KNNK value [ref 1] — present; drawn in 100/100, drawn-subset cohort
    - [ ] **4.9a.22** KQKR value [ref 10] — absent; conv 83%; never occurred in games
    - [ ] **4.9a.23** KQKRPs scale [ref 12] — absent; never occurred in games
    - [ ] **4.9a.24** KRPPKRP scale [ref 15] — absent; never occurred in games
    - [ ] **4.9a.25** Dependency-complete family refits and gates, tiered by occurrence
    - [ ] **4.9a.26** Conversion, theory, STC/LTC and endgame-cohort closure
- [ ] **4.10** Iterated no-adjudication refit cycles — at least one is owed
    - [ ] **4.10.1** Opening supply: reuse is safe; fresh openings optional, not a blocker
    - [ ] **4.10.2** Composition screen against the `datagen-v1` archive (sizing)
    - [ ] **4.10.3** RAR-E08: self-play vs TB-corrected labels; decides the contract
    - [ ] **4.10.4** Regenerate on the winning contract; never edit `hce-v2`
    - [ ] **4.10.5** Cycle 1: full 4.8 schedule, own frozen test, registered gate
    - [ ] **4.10.6** Repeat while a cycle accepts; stop at the first that does not
    - [ ] **4.10.7** Record the cycle table and close
- [ ] **4.11** Re-measure qsearch/TT/eval authority and branching on the accepted HCE
    - [ ] **4.11.1** Observation, baseline and live-wire proof; write the analysis
    - [ ] **4.11.2** One candidate and gate, only if 4.11.1 isolates a unique defect
- [ ] **4.12** Optional post-HCE search SPSA; skip without a displaced optimum
- [ ] **4.12a** Time management: review, repair and gate — owns all TM work
    - [ ] **4.12a.1** Revalidate accepted clock behavior on the accepted HCE — RAR-R01/R02
    - [ ] **4.12a.2** `Move Overhead` vs forfeit rate on a null pair; size it first — RAR-M14
    - [ ] **4.12a.3** `RootConfTime`'s six identifiable consumers: tune or remove — RAR-S47
    - [ ] **4.12a.4** Root-instability TM from a completed snapshot only — RAR-X06, RAR-R05
    - [ ] **4.12a.5** Registered gate; zero forfeits is a precondition, not the verdict
- [ ] **4.13** Search cleanup and clean checkpoint
    - [ ] **4.13.1** Dead and unreachable mechanism inventory — Basilisk-derived
    - [ ] **4.13.2** Remove unconsumed 4.6 alternatives, plus the 3 terms RAR-E07 left inert
    - [ ] **4.13.3** Re-verify tests, clippy, fingerprint, NPS and deficits
- [ ] **4.14** Final HCE/search checkpoint, attribution and maturity closure
- [ ] **4.15** STC/LTC/4T, NPS, portability, ISA and release gate

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
