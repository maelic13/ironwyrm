# Rarog development guide

**Current state and ordered steps only.** Design and rationale live in
`PLAN.md`; durable results live in `EXPERIMENTS.md`.

## Current checkpoint

| Item | Value |
|---|---|
| Released baseline | **2.3.2** at `f931722` on `master` |
| Accepted search head | RAR-S70, **6,977,070 nodes / EBF 2.466** |
| Integration branch | `dev`; failed SearchCore rewrite reverted by `c5e451d`, measurement tooling upgraded by `d2c7788`/`e4f10ca` |
| Frozen oracle | `hybrid` at `75d0d43`; never merge it into Rarog |
| Measured search deficit | **355.26 +/- 27.03 Elo at equal nodes**; **250.77 +/- 13.12** at equal time; speed contribution **104.5 Elo** |
| Accepted Phase-4 gains | ProbCut filter **+15.56 +/- 10.02 Elo**; root LMR relief **+2.33 +/- 1.85 Elo** |
| Active experiment | None |
| Current step | **4.7.1 — audit the existing HCE self-play archives and three-way corpus contract** |
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

- [ ] **4.7** Qualify HCE data, labels, fitting instruments and current-source maturity
    - [ ] **4.7.1 NEXT** Audit the existing self-play archives; freeze leak-free train/validation/test hashes and label/coverage report
    - [ ] **4.7.2** Add exact all-slot instrument coverage plus end-to-end fit/bake/rebuild smoke; no group remains frozen because it was fitted before
    - [ ] **4.7.3** Finish residual/covariance and Stockfish contract map on current source
- [ ] **4.8** Refit and gate the complete existing HCE surface: all identifiable linear weights plus every activated nonlinear/capped surface
- [ ] **4.9** At most two residual-selected structural HCE clusters; skip if the complete refit leaves no supported gap
- [ ] **4.10** Post-structure whole-HCE consolidation; satisfied by 4.8 if no structure changes

**Post-HCE search closeout**

- [ ] **4.11** Re-measure qsearch/TT/eval authority and playing-depth branching on the accepted HCE; build one candidate only if a unique defect remains
- [ ] **4.12** Optional targeted post-HCE search SPSA; skip without a displaced interacting optimum
- [ ] **4.13** Remove unowned alternatives and record the clean search checkpoint
- [ ] **4.14** Final HCE/search checkpoint, attribution and maturity closure

**Release**

- [ ] **4.15** STC/LTC/4T, NPS, portability, ISA and release gate

Current HCE maturity analysis:
`analysis/hce_maturity_2026-08-25.md`. The older
`analysis/hce_analysis.md` is historical; its four concrete activation defects
are already fixed and are not current work.

## What you run next

Step 4.7 starts with the data and instrument contracts, not a new fit. The two
existing self-play archives are inputs only; do not overwrite the old
`train.csv`/`holdout.csv` or open a new frozen test during tool development.

```powershell
python tools\texel\extract_parallel.py `
    tools\texel\data\selfplay-p1025a-zero-n8000-s1-g20000.pgn `
    tools\texel\data\selfplay-p1025a-zero-n8000-s20001-g580000.pgn `
    --out-dir tools\texel\data\hce-v2 --jobs 14 --audit-only
```

The audit must report zero parse errors/replays and meet every three-way phase
quota before publication. Then implement the 4.7.2 coverage command: every
`EvalParams` slot must be claimed by a linear fitter, nonlinear/finite-difference
fitter, algebraic gauge, invariant or measured unidentifiable disposition.
Its end-to-end smoke must prove that vectors are baked rather than discarded.

After that tooling exists, publish the new dataset once and qualify it:

```powershell
python tools\texel\extract_parallel.py `
    tools\texel\data\selfplay-p1025a-zero-n8000-s1-g20000.pgn `
    tools\texel\data\selfplay-p1025a-zero-n8000-s20001-g580000.pgn `
    --out-dir tools\texel\data\hce-v2 --jobs 14

cargo run --release -p texel-tuner -- --verify tools\texel\data\hce-v2\validation.csv
cargo run --release -p texel-tuner -- --feature-support tools\texel\data\hce-v2\train.csv
```

No fit or strength game is authorized until 4.7.1–4.7.3 are complete. Search
authority measurement moves to 4.11 because the HCE refit can change qsearch,
score-scale and pruning populations.

## Documentation ownership

| File | Purpose |
|---|---|
| `GUIDE.md` | Current status, ordered steps and commands |
| `PLAN.md` | Rationale, gates and full roadmap |
| `EXPERIMENTS.md` | Durable evidence, failures, retry triggers and recipes |
| `analysis/hce_maturity_2026-08-25.md` | Current HCE/Stockfish maturity comparison and fitting policy |
| `analysis/manta_tooling_audit_2026-08-25.md` | Manta tool dispositions, imported measurements and limits |
| `analysis/basilisk_audit_2026-08-30.md` | Basilisk method/results audit and Rarog-specific consequences |
| `analysis/ablation_results.md` | Search-deficit measurements and corrected interpretation |
| `PROCESS.md` | Recurring build, Texel, SPSA and release procedures |
| `TRACKER.md` | History only; never a source of the next step |

`GUIDE.md` and `PLAN.md` must change in the same commit. Source, defaults and
reproducible artifacts outrank prose whenever documents disagree.
