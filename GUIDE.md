# Rarog development guide

**Current state and ordered steps only.** Design and rationale live in
`PLAN.md`; durable results live in `EXPERIMENTS.md`.

## Current checkpoint

| Item | Value |
|---|---|
| Released baseline | **2.3.2** at `f931722` on `master` |
| Accepted search head | RAR-S70, **6,977,070 nodes / EBF 2.466** |
| Integration branch | `dev`; failed SearchCore rewrite reverted by `c5e451d`; HCE audit/fitting pipeline added by `8d8f507` |
| Frozen oracle | `hybrid` at `75d0d43`; never merge it into Rarog |
| Measured search deficit | **355.26 +/- 27.03 Elo at equal nodes**; **250.77 +/- 13.12** at equal time; speed contribution **104.5 Elo** |
| Accepted Phase-4 gains | ProbCut filter **+15.56 +/- 10.02 Elo**; root LMR relief **+2.33 +/- 1.85 Elo** |
| Active experiment | **RAR-E06 registered; games not started** |
| Current step | **4.8.3 — run the mandatory no-adjudication calibration, then the complete-HCE SPRT** |
| HCE | Open now; no historical family is frozen. Refit every identifiable linear and nonlinear surface before adding features |
| Next release | Conditional 2.4.0 at 4.15; then NNUE |

## Phase 4 status

**Evidence foundation — closed**

- [x] **4.0** Evidence, baseline and oracle freeze — RAR-M12
- [x] **4.1** Instrumented oracle — `hybrid-diag` `de568b3`
- [x] **4.2** Differential observation harness — RAR-S55
- [x] **4.3** Mechanism map and order freeze
- [x] **4.4** Matched ablation plus fixed-node correction
- [x] **4.5** LMR contract study — no accepted interior gain; RAR-S70 root gain retained

**Failed continuation — closed**

- [x] **4.6** Shallow-selectivity/rewrite continuation — no accepted gain
    - [x] **4.6.1** Quiet SEE screen: gap closure **+3.38 +/- 27.08 Elo**, stopped null
    - [x] **4.6.2** SearchCore: **-9.76 +/- 17.70 Elo** over 712 complete games, stopped before boundary and reverted
    - [x] **4.6.3** Broad selectivity SPSA and another rewrite declined; entry evidence absent

**HCE maturity and fitting**

- [x] **4.7** Qualify HCE data, labels, fitting instruments and current-source maturity
    - [x] **4.7.1** Archive provenance/content audit: 600k independent starts; qualified capacity is 2.30M/127,778/127,778, not 3M
    - [x] **4.7.2** Published/hash-frozen `hce-v2`: pure self-play WDL, 600k independent starts from 750k unique openings
    - [x] **4.7.3** Exact 1,218-slot instrument partition and vector/bake/rebuild smoke
    - [x] **4.7.4** Current-source Stockfish maturity map; structural gaps remain post-fit hypotheses
- [ ] **4.8** Refit and gate the complete existing HCE surface
    - [x] **4.8.1** Fresh 150k-game confirmation: 127,778 untouched test positions; exact candidate loss **0.12252203** vs source **0.12330291**, delta **-0.00078088**; all broad cohorts improved
    - [x] **4.8.2** Baked exact fit at **7,226,051 / 2.460**; pooled PGO NPS **-1.19%**; RAR-E06 registered at `[0,3]`, 80k, no adjudication
    - [ ] **4.8.3 NEXT** Pass the registered identical-binary no-adjudication calibration, then run RAR-E06; accept only H1
- [ ] **4.9** At most two residual-selected structural HCE clusters; skip if the complete refit leaves no supported gap
- [ ] **4.10** Post-structure whole-HCE consolidation; satisfied by 4.8 if no structure changes

**Post-HCE search closeout**

- [ ] **4.11** Re-measure qsearch/TT/eval authority and playing-depth branching on the accepted HCE; build one candidate only if a unique defect remains
- [ ] **4.12** Optional targeted post-HCE search SPSA; skip without a displaced interacting optimum
- [ ] **4.13** Remove unowned alternatives and record the clean search checkpoint
- [ ] **4.14** Final HCE/search checkpoint, attribution and maturity closure

Basilisk 5.7.3's check/singular three-ply stack is absent in current Rarog;
PLAN 4.11 nevertheless requires both depth and tactical screens for any future
extension change.

**Release**

- [ ] **4.15** STC/LTC/4T, NPS, portability, ISA and release gate

## Phase 5 status

- [ ] **5.0** Endgame knowledge and conversion maturity before NNUE
    - [ ] **5.0.1** Syzygy/no-adjudication truth corpus and reproducible conversion baseline
    - [ ] **5.0.2** Contextual regression contract: hard theory/correctness, aggregate conversion floors
    - [ ] **5.0.3** Audit and close all 20 final-HCE Stockfish value/scale functions; current meaningful coverage is about 7/20
    - [ ] **5.0.4** Search-visible gradient magnitude audit, starting with KBNK/KXK and passer king approach
    - [ ] **5.0.5** Refit and gate dependency-complete endgame families
    - [ ] **5.0.6** Conversion, theory, STC/LTC and endgame-cohort closure
- [ ] **5.1** NNUE corpus handoff
- [ ] **5.2** Per-ply state and dirty pieces
- [ ] **5.3** Accumulator scaffolding
- [ ] **5.4** Trainer preflight
- [ ] **5.5** Runway gate
- [ ] **5.6** Optional threat-map hooks

Current HCE maturity analysis:
`analysis/hce_maturity_2026-08-25.md`; archive measurements:
`analysis/hce_archive_audit_2026-08-31.md`. The older
`analysis/hce_analysis.md` is historical; its four concrete activation defects
are already fixed and are not current work.

## What you run next

Run this single PowerShell block from `D:\code\rarog`. It first performs the
registered 30,000-game identical-binary no-adjudication calibration. Only a
calibration whose full 95% nElo interval lies inside ±5 proceeds to the
candidate's `[0,3]` nElo, 80,000-game gate. Only H1 accepts; H0 or the cap
rejects the vector.

```powershell
& {
    $ErrorActionPreference = "Stop"
    & .\tools\sprt.ps1 `
        -EngineA .\tools\test_engines\rarog-hce-refit-candidate-pext-pgo.exe `
        -EngineB .\tools\test_engines\rarog-hce-refit-candidate-pext-pgo.exe `
        -NameA HCERefitNullA -NameB HCERefitNullB `
        -Mode calibrate -Games 30000 -CalibrationTolerance 5 `
        -TC "3+0.03" -Threads 1 -Hash 64 -Concurrency 14 -TimeMargin 20 `
        -Book .\tools\books\UHO_Lichess_4852_v1.epd `
        -Seed 174839201 -NoAdjudication

    & .\tools\sprt.ps1 `
        -EngineA .\tools\test_engines\rarog-hce-refit-candidate-pext-pgo.exe `
        -EngineB .\tools\test_engines\rarog-hce-refit-base-pext-pgo.exe `
        -NameA HCERefit -NameB HCEBase `
        -Elo0 0 -Elo1 3 -Alpha 0.05 -Beta 0.05 -MaxGames 80000 `
        -TC "3+0.03" -Threads 1 -Hash 64 -Concurrency 14 -TimeMargin 20 `
        -Book .\tools\books\UHO_Lichess_4852_v1.epd `
        -Seed 918274631 -NoAdjudication
}
```

Search-authority measurement remains at 4.11 because an accepted HCE can
change qsearch, score scale and pruning populations.

## Documentation ownership

| File | Purpose |
|---|---|
| `GUIDE.md` | Current status, ordered steps and commands |
| `PLAN.md` | Rationale, gates and full roadmap |
| `EXPERIMENTS.md` | Durable evidence, failures, retry triggers and recipes |
| `analysis/hce_maturity_2026-08-25.md` | Current HCE/Stockfish maturity comparison and fitting policy |
| `analysis/hce_archive_audit_2026-08-31.md` | Exact archive provenance, content, capacity and quota audit |
| `analysis/manta_tooling_audit_2026-08-25.md` | Manta tool dispositions, imported measurements and limits |
| `analysis/basilisk_audit_2026-08-30.md` | Basilisk method/results audit and Rarog-specific consequences |
| `analysis/endgame_conversion_audit_2026-09-01.md` | Rarog/Basilisk conversion, 20-function inventory, defects and test policy |
| `analysis/ablation_results.md` | Search-deficit measurements and corrected interpretation |
| `PROCESS.md` | Recurring build, Texel, SPSA and release procedures |
| `TRACKER.md` | History only; never a source of the next step |

`GUIDE.md` and `PLAN.md` must change in the same commit. Source, defaults and
reproducible artifacts outrank prose whenever documents disagree.
