# Rarog development guide

**Current state and the ordered steps. Nothing else.** Detail belongs
elsewhere and is linked below — if you are adding more than a few lines here,
it probably belongs in `PLAN.md`, `TRACKER.md` or `PROCESS.md` instead.

## Current checkpoint

| Item | Value |
|---|---|
| Released baseline | **2.3.2** at `f931722` on `master` |
| Accepted fingerprint | **6,977,070 nodes / EBF 2.466** (RAR-S70) |
| Integration branch | `dev` |
| Frozen oracle | `hybrid` at `75d0d43`; never merge it into Rarog |
| Ablation instrument | `hybrid-ablate` (oracle) + `--features ablate` (Rarog); one shared bitmask |
| Measured deficit | **`G(0)` = 250.77 ± 13.12 Elo** vs the oracle; ~302 once Rarog's 1.80x NPS is netted out |
| Active experiment | None |
| Current step | **PLAN 4.5 — rework LMR.** 116 Elo measured, the largest single item |
| Evaluation | Frozen through 4.9; the HCE decomposition starts at 4.10 |
| Next releases | Conditional 2.4.0 at 4.15; then NNUE 2.5.0 |

## Phase 4 — status

Renumbered 2026-08-21 on the ablation decomposition
(`analysis/ablation_results.md`). `PLAN.md` holds what each step involves.
**Tick a box only when the step is finished AND verified**, and update this
file and `PLAN.md` in the same commit.

**Observation and instruments — closed**

- [x] **4.0** Evidence, baseline and oracle freeze — RAR-M12
- [x] **4.1** Instrumented oracle — `hybrid-diag` `de568b3`
- [x] **4.2** Differential observation harness — RAR-S55
- [x] **4.3** Mechanism map and order freeze
- [x] **4.4** Search-consumed board state — nothing required
- [x] **4.4a** Matched-ablation instrument — `hybrid-ablate` + `--features ablate`, every bit proved live
- [x] **4.4b** Deficit decomposed — `G(0)` = 250.77 ± 13.12; LMR 116.0, shallow pruning 124.6, extensions null

**Search track — selectivity holds 240 of the 250.8 Elo**

- [ ] **4.5 LMR rework — target 116.0 ± 17.9 Elo** ← CURRENT
    - [x] 4.5.1 reduction is a contract (`ReductionInputs`) + 4 missing terms, all default-off
    - [x] 4.5.2 `LmrMinReducedDepth` built, default-off — 46.7% of reductions land in qsearch
    - [x] 4.5.3 root term measured and accepted — RAR-S70, +2.33 ± 1.85; **refit needed once 4.5.1 lands**
    - [ ] 4.5.4 re-measure `G(128)` as the progress meter
- [ ] **4.6 Shallow-depth pruning rework — target 124.6 ± 17.7 Elo**
    - [ ] 4.6.1 move-count, history, parent futility, the two SEE prunes, as one contract
    - [ ] 4.6.2 settle `SelectivityProspectiveDepth` here
    - [ ] 4.6.3 make `prune_shadow_*` track the live block, or delete it
- [ ] **4.7** History and ordering, as inputs to 4.5/4.6
- [ ] **4.8** ONE seeded selectivity SPSA, after the contracts land
- [ ] **4.9** Integrate, re-measure `G(0)`, freeze the search head

**HCE track — the larger prize, +328.6 Elo, not yet decomposed**

- [ ] **4.10** Reciprocal HCE oracle + eval ablation instrument
- [ ] **4.11** Decompose the +328.6 Elo by term family — BEFORE building anything
- [ ] **4.12** Rework the families 4.11 ranks, in that order
- [ ] **4.13** Texel consolidation, after the representations freeze
- [ ] **4.14** HCE checkpoint and ablation

**Release**

- [ ] **4.15** Transfer, portability, SMP, release gate → **2.4.0**

**Accepted in Phase 4 so far:** 4.7c ProbCut move filter **+15.56 ± 10.02**
(RAR-S57/S58) and RAR-S70 root relief **+2.33 ± 1.85**. Programme target is
≥ +100 Elo cumulative over 2.3.2.

**Parked by measurement, not by judgement:** extensions (11.8 ± 18.8, a 1.2
sigma null), TT and quiescence, root and clock, and the whole answer-harness
line. Matched ablation bounds all of them together at ~30 Elo. PLAN's "Parked
by measurement" table keeps each with the evidence that parked it; any revives
the moment a measurement puts Elo behind it.

**After Phase 4:** Phase 5 NNUE runway → Phase 6 baseline NNUE (2.5.0) →
Phases 7–9. See `PLAN.md`.

## What you run now

Rebuild before measuring, with the exact feature set. Never measure a binary
you did not just build — see `AGENTS.md`.

```bash
cargo xtask build --arch pext --pgo
```

Ablation, either engine, one shared bitmask (0 razoring, 1 futility-child,
2 nullmove, 3 probcut, 4 iir, 5 shallow-pruning, 6 extensions, 7 lmr):

```bash
pwsh -File tools/sprt.ps1 -EngineA tools/test_engines/ablate/rarog-ablate-pext-pgo.exe -EngineB tools/test_engines/ablate/rarog-stockfish-hce-hybrid.exe -NameA Rarog -NameB Oracle -OptionsA AblationMask=0 -OptionsB AblationMask=0 -Mode fixed -Games 2000
```

A registered strength gate, `[0,3]` nElo, sized from RAR-M10 before
registering:

```bash
pwsh -File tools/sprt.ps1 -EngineA <candidate> -EngineB <base> -NameA New -NameB Head -Elo0 0 -Elo1 3 -MaxGames 80000
```

Bench counters, summed over all 40 per-position dumps:

```bash
python tools/diag/bench_counters.py --exe target/release/rarog.exe --depth 13 --filter lmr
```

The hosted release workflow remains the final production check: one machine
cannot create all Linux/macOS/Windows and x86/ARM assets.

## Documentation ownership

| File | Audience / purpose |
|---|---|
| `GUIDE.md` | **This file. Current state and ordered steps only** |
| `PLAN.md` | Maintainers: what each step involves, ownership, the full roadmap |
| `TRACKER.md` | The detailed per-step checklist, past and forward. Retired numbering |
| `PROCESS.md` | Recurring procedures: step lifecycle, Texel, SPSA go/no-go, toolchain, book |
| `AGENTS.md` | The rules that stop wrong results. Imported by `CLAUDE.md` |
| `EXPERIMENTS.md` | Durable evidence, failures and retry triggers |
| `README.md` | Users: install, CPU choice, UCI and build basics |
| `CHANGELOG.md` | Users: visible release deltas and measured claims |
| `analysis/ablation_results.md` | The deficit decomposition that ordered Phase 4 |
| `analysis/phase4_counter_spec.md` | Shared 4.1/4.2 counter contract |
| `tools/spsa_configs/README.md` | Tuning-specific mechanics and lessons |

**`GUIDE.md` and `PLAN.md` are updated in the SAME commit.** GUIDE is the
overview of PLAN; a GUIDE that disagrees with PLAN is worse than no GUIDE.

When facts disagree, source, defaults and reproducible artifacts outrank prose;
fix the prose in the same change.
