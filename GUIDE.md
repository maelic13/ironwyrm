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
| Current step | **4.8a — post-refit redundancy removal** |
| Next release | Conditional 2.4.0 at 4.15; NNUE follows either way |

## What you run next

RAR-E06 is accepted and `dev` carries it. Nothing is running. The next step is
4.8a: list the coefficients the complete fit drove to zero or left without
support, remove them as one cluster, and gate at `[-1.75, 0.25]`.

Confirm the accepted head before building anything on it:

```powershell
cargo build --release
'bench 13' | .\target\release\rarog.exe
```

It must print **7,226,051** nodes and **EBF 2.460**. A different number means
different code, not a different machine.

## Phase 4 — bounded pre-NNUE search and HCE

- [x] **4.0** Evidence, baseline and oracle freeze — RAR-M12
- [x] **4.1** Instrumented oracle — `hybrid-diag` `de568b3`
- [x] **4.2** Differential observation harness — RAR-S55
- [ ] **4.2a** Harness and instrument integrity sweep — Basilisk-derived
    - [x] **4.2a.1** `sprt.ps1` options-free repair and `-NoAdjudication` wire proof — `cb5ed2a`
    - [ ] **4.2a.2** Unchecked native exit status sweep across `tools/*.ps1`
    - [x] **4.2a.3** Anomaly guard rate-limited so it discriminates instead of voiding every gate — `334c084`
    - [ ] **4.2a.4** Every option a script accepts must be honored or refuse to launch
- [ ] **4.2b** Time-forfeit margin at test concurrency — RAR-M14
    - [x] **4.2b.1** Diagnosis: games end at 97-99% of clock; ~2% aggregate slack
    - [ ] **4.2b.2** Null-pair sweep of `Move Overhead` against forfeit rate
    - [ ] **4.2b.3** Gate any time-management default change; it alters play
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
- [ ] **4.8a NEXT** Post-refit redundancy removal — Basilisk BAS-E25 method
    - [ ] **4.8a.1** List coefficients the complete fit drove to zero or left unsupported
    - [ ] **4.8a.2** Remove as one cluster and gate at loss-permitting `[-1.75, 0.25]`
- [ ] **4.9** At most two residual-selected structural HCE clusters
- [ ] **4.9a** Endgame conversion and reference-function closure
    - [ ] **4.9a.1** Syzygy/no-adjudication truth corpus and reproducible baseline
    - [ ] **4.9a.2** Contextual regression contract: hard theory, aggregate floors
    - [ ] **4.9a.3** Search-visible gradient audit vs pruning margins — KBNK/KXK first
    - [ ] **4.9a.4** KNNK value/draw classification — present, unaudited
    - [ ] **4.9a.5** KNNKP value and conversion boundary — absent
    - [ ] **4.9a.6** KXK value, with KQK/KRK/KBBK conversion floors — present, 94/86/76%
    - [ ] **4.9a.7** KBNK value, corner/king/minor gradients — present, converts **15%**
    - [ ] **4.9a.8** KPK exact bitbase, value and rule-50 handling — present, unaudited
    - [ ] **4.9a.9** KRKP value — partial, conservative scale only
    - [ ] **4.9a.10** KRKB value — absent
    - [ ] **4.9a.11** KRKN value — absent
    - [ ] **4.9a.12** KQKP fortress-aware value — partial, rook/bishop pawn only
    - [ ] **4.9a.13** KQKR value — absent
    - [ ] **4.9a.14** KBPsK scale, wrong-bishop rook pawn — partial, wrong-corner subset
    - [ ] **4.9a.15** KQKRPs scale — absent
    - [ ] **4.9a.16** KRPKR scale — absent
    - [ ] **4.9a.17** KRPKB scale — absent
    - [ ] **4.9a.18** KRPPKRP scale — absent
    - [ ] **4.9a.19** KPsK scale — absent
    - [ ] **4.9a.20** KBPKB scale — absent
    - [ ] **4.9a.21** KBPPKB scale — absent
    - [ ] **4.9a.22** KBPKN scale — absent
    - [ ] **4.9a.23** KPKP scale — absent
    - [ ] **4.9a.24** Dependency-complete family refits and no-adjudication gates
    - [ ] **4.9a.25** Conversion, theory, STC/LTC and endgame-cohort closure
- [ ] **4.10** Post-structure whole-HCE consolidation; satisfied by 4.8 if nothing changed
- [ ] **4.11** Re-measure qsearch/TT/eval authority and branching on the accepted HCE
    - [ ] **4.11.1** Observation, baseline and live-wire proof; write the analysis
    - [ ] **4.11.2** One candidate and gate, only if 4.11.1 isolates a unique defect
- [ ] **4.12** Optional post-HCE search SPSA; skip without a displaced optimum
- [ ] **4.13** Search cleanup and clean checkpoint
    - [ ] **4.13.1** Dead and unreachable mechanism inventory — Basilisk-derived
    - [ ] **4.13.2** Remove unconsumed 4.6 and retained default-off alternatives
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
