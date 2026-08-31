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
| Active experiment | None |
| Current step | **4.8.1 — obtain an untouched confirmation set and rerun the repaired exact source-to-rounded HCE report** |
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
    - [ ] **4.8.1 NEXT** First fit's comparator/rounding report was invalid; use fresh unused openings and the repaired exact source-to-persisted-candidate report
    - [ ] **4.8.2** Apply a valid fit on a clean branch, bake final PGO, measure NPS and register the game gate
    - [ ] **4.8.3** Run the registered SPRT; accept or restore the complete vector
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

Current HCE maturity analysis:
`analysis/hce_maturity_2026-08-25.md`; archive measurements:
`analysis/hce_archive_audit_2026-08-31.md`. The older
`analysis/hce_analysis.md` is historical; its four concrete activation defects
are already fixed and are not current work.

## What you run next

Run this from a clean `dev` worktree. It repeats the completed archive audit,
publishes the qualified corpus only if `hce-v2` does not exist, runs the fixed
complete fit, verifies the baked candidate, and restores both source and the
accepted normal-feature binary afterward.

```powershell
pwsh -NoProfile -File tools\texel\fit_complete.ps1
```

All artifacts land in a new ignored `tools/results/hce-fit-<timestamp>/`:
`summary.json`, settings, audit/support/trajectory/test logs, four complete
vectors, candidate patch/binary and hashes. This run gives an **offline fitting
result only**. Do not start games or apply the patch until 4.8.2 review and
prospective registration. Search-authority measurement remains at 4.11 because
an accepted HCE can change qsearch, score scale and pruning populations.

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
| `analysis/ablation_results.md` | Search-deficit measurements and corrected interpretation |
| `PROCESS.md` | Recurring build, Texel, SPSA and release procedures |
| `TRACKER.md` | History only; never a source of the next step |

`GUIDE.md` and `PLAN.md` must change in the same commit. Source, defaults and
reproducible artifacts outrank prose whenever documents disagree.
