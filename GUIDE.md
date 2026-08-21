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

## Phase 4 — the ordered steps

Renumbered 2026-08-21 on the ablation decomposition
(`analysis/ablation_results.md`). `PLAN.md` holds what each step actually
involves; this is the order and the price.

| step | work | measured target |
|---|---|---:|
| **4.5** | LMR rework — replace the reduction contract, land `LmrMinReducedDepth`, refit the root term | **116.0 ± 17.9 Elo** |
| **4.6** | shallow-depth pruning rework — move-count, history, futility, the two SEE prunes | **124.6 ± 17.7 Elo** |
| 4.7 | history and ordering, as INPUTS to 4.5/4.6 | inside the above |
| 4.8 | ONE seeded selectivity SPSA, after the contracts land | refit only |
| 4.9 | integrate, re-measure `G(0)`, freeze the search head | — |
| 4.10 | reciprocal HCE oracle + eval ablation instrument | — |
| 4.11 | decompose the **+328.6 Elo** eval gap by term family, BEFORE building | to be measured |
| 4.12 | rework the eval families 4.11 ranks, in that order | to be measured |
| 4.13 | Texel consolidation, after the representations freeze | — |
| 4.14 | HCE checkpoint and ablation | — |
| **4.15** | transfer, portability, SMP, release gate → **2.4.0** | — |

**Parked by measurement, not by judgement:** extensions (11.8 ± 18.8, a 1.2
sigma null), TT and quiescence, root and clock, and the whole answer-harness
line. Matched ablation bounds all of them together at ~30 Elo. PLAN's "Parked
by measurement" table keeps each one with the evidence that parked it; any of
them revives the moment a measurement puts Elo behind it.

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
