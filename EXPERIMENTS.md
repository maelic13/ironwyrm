# Rarog experiment ledger

This is the indexed maintainer record of measured experiments and the lessons
that may inform later work. It is not a roadmap: [`PLAN.md`](PLAN.md) owns what
will be done and in what order. [`CHANGELOG.md`](CHANGELOG.md) remains the
user-facing release record.

Every lesson below is conditional. A result describes one engine state, test
protocol, time control, compiler and machine population; it does not establish
a universal chess-programming rule. An experiment from Basilisk is only a
prior for Rarog and never bypasses Rarog's own gates.

**Numbering note.** Rows cite three retired numbering schemes: the legacy
Lynx/Rarog phases (`8.2(a)`, `9.0a`), the closed pre-2.3.2 Phase-4 line, and
the 2026-08-11 to 2026-09-09 Phase-4 roadmap (`4.5`, `4.11b.19`, `4.12.7`,
Phases 5-9). Resolve them through `HISTORY.md`; the archived roadmap is
`docs/archive/PLAN-phase4-2026-09-09.md` and PLAN section 6 maps its open
leaves onto the current lettered roadmap (`A.2.1`). Rows from RAR-M45 on cite
the current roadmap.

## Contents

- [1. How to use this ledger](#1-how-to-use-this-ledger)
  - [Result and evidence vocabulary](#result-and-evidence-vocabulary)
  - [Recording contract](#recording-contract)
  - [Prediction freeze and calibration](#prediction-freeze-and-calibration)
- [2. Measurement, harness and tuning](#2-measurement-harness-and-tuning)
- [3. Search and selectivity](#3-search-and-selectivity)
  - [Search-accuracy decomposition](#search-accuracy-decomposition)
  - [Search-oracle observations](#search-oracle-observations)
  - [Accepted or retained](#accepted-or-retained)
  - [Rejected, neutral or deferred](#rejected-neutral-or-deferred)
- [4. Root search, time management and SMP](#4-root-search-time-management-and-smp)
- [5. Evaluation and data experiments](#5-evaluation-and-data-experiments)
- [6. Throughput, build and platforms](#6-throughput-build-and-platforms)
- [7. Correctness and protocol lessons](#7-correctness-and-protocol-lessons)
- [8. Cross-engine evidence imported from Basilisk](#8-cross-engine-evidence-imported-from-basilisk)
- [9. Open retry map](#9-open-retry-map)
- [10. Template for a new experiment](#10-template-for-a-new-experiment)

## 1. How to use this ledger

Search the contents by subsystem before proposing a mechanism, tune or retry.
Use the stable IDs in commit messages and `PLAN.md` when a prior result changes
a future decision. Do not copy the tables into `PLAN.md`.

### Result and evidence vocabulary

| Term | Meaning in this document |
|---|---|
| **Accepted** | Passed the registered gate and entered an accepted baseline. |
| **Retained** | Kept for correctness, infrastructure or structural value; any Elo figure may be unresolved. |
| **Rejected** | Failed its registered gate or had a clear adverse measurement and was reverted. |
| **Neutral/inconclusive** | Evidence did not distinguish a useful effect at the tested resolution. |
| **Observation** | Diagnostic evidence, not an acceptance verdict. |
| **No-change** | Research closed because the measured premise/opportunity did not justify an engine change. |
| **Deferred** | Not decided under present prerequisites; owner and objective resume condition are recorded. |
| **Imported prior** | Evidence from Basilisk; useful for ordering or designing a Rarog test, never for accepting it. |

Unless a row says otherwise, historical strength tests used paired games at
fast time control. Results before the pinned-harness repair of 2026-07-21 may
carry scheduler-placement bias. Fast-TC deltas are non-additive and may
compress or reverse at longer TC.

### Recording contract

Register before exposure, then update the same entry when accepting, reverting
or closing. Record the research question; baseline/candidate SHAs and any
dirty-diff identity; binary/compiler/PGO identity; hypothesis, competing
explanations and interacting consumers; cheapest prior falsifier; frozen
prediction and confidence; falsification and stop rules; full conditions;
diagnostics separately from the verdict; result/disposition; calibration and
postmortem; conditional lesson; objective retry trigger; and artifacts.

The prediction must state expected diagnostic movement, expected Elo sign or
range only when defensible, probability the candidate is positive/useful, and
the most likely failure mode. It is frozen once any deciding result is exposed.
Correct only a genuine clerical error, mark that correction explicitly, and
never fabricate a prediction for a historical entry that lacked one.

Use cautious language: “under these conditions this suggests …”, not “feature
X is good/bad”. If conditions or artifacts are unknown, say so.

### Prediction freeze and calibration

Keep **what was believed before exposure** separate from the postmortem. A good
retrospective explanation is not evidence that the result was predicted. After
a surprise ask which part of the original causal model was wrong, not why the
outcome now seems obvious.

At each phase checkpoint, review new prospectively registered experiments by
category (search/selectivity, HCE, endgame, TT/cache, SMP/time,
board/performance, tooling, data/tuning or NNUE): Was the sign right? Was the
magnitude systematically optimistic? Did high confidence predict reliability?
Which mechanism or interaction was missed? Did the instrument fail? Was any
rejected idea retried without its trigger? Record only repeated calibration
lessons; do not build a score or rewrite the frozen entries.

## 2. Measurement, harness and tuning

| ID | Experiment and conditions | Result / disposition | Conditional lesson and retry trigger | Source |
|---|---|---|---|---|
| RAR-M01 | Early fixed-`movetime` gates were compared with the deployed clock path at `3+0.03`. | Fixed movetime manufactured false negatives; SPRT/SPSA moved to a unified clock TC. | A test TC must exercise the same time-management semantics as deployment. Fixed movetime remains useful only for deterministic diagnostics. | legacy plan at `757e9a3^` |
| RAR-M02 | Historical unpinned fastchess runs were audited under explicit physical-core placement on the Ryzen 9 5950X. | Real affinity/topology defects were found; the original +9.34 ± 8.20 null did not itself prove a fixed +9 Elo offset. | On this Windows host, small unpinned results may be biased. Re-audit a borderline old verdict only if it affects a current decision. | legacy plan at `757e9a3^` |
| RAR-M03 | Identical-binary null testing after harness changes. | The old symmetric `[-3,+3]` setup had zero expected LLR drift at equality; current policy is fixed-N 30k at 1T, requiring the full 95% nElo CI inside ±5. | Equivalence needs a calibration design, not an ordinary gain SPRT. Repeat after runner, scheduler or topology changes that can create arm asymmetry. Merely disabling both draw and resign adjudication symmetrically does not change placement or side assignment and does not consume another 30k null. | `PLAN.md` §2 |
| RAR-M04 | Opening-book migration to paired UHO games. | Retained because it increased decisive-game signal and aligned SPRT, SPSA and gauntlet conditions. | Raw Elo from old and UHO protocols is not directly comparable without a bridge test. | legacy plan at `757e9a3^` |
| RAR-M05 | SPSA schedule audit: iteration/game units, PowerShell `$A`/`$a` collision and integer perturbation resolution. | Several schedule defects were repaired; old runs annealed faster than intended. Accepted bakes remain accepted because independent SPRTs passed. | A plausible SPSA trajectory is not proof of a correct schedule. Assert every emitted derived constant and inspect coordinate observability over the full horizon. | legacy plan at `757e9a3^` |
| RAR-M06 | Resignation threshold replay against 69,350 historical games. | `400/3` one-sided was too aggressive for Rarog's scale; `600/3` one-sided became `strength-v1`. | Adjudication scores are engine-scale dependent. Recalibrate after a material score-scale change such as NNUE integration. | legacy plan at `757e9a3^`; `PLAN.md` §2 |
| RAR-M07 | Staged self-play gains were checked in an external engine cohort. | The 2.2 cycle's roughly +316 staged result transferred as about +240 over 2.1.0. The 2.3 boundary measured +76/+78 at 1T and +194 at 4T over 2.2.0. | Self-play gives direction under these conditions, not an additive external forecast. Phase boundaries need direct prior-release and target-engine checks. | `CHANGELOG.md` 2.2.0, 2.3.0; legacy plan |
| RAR-M08 | The 36,400-game 2026-08-05 rating tournament used a 2.4-dev binary with interim values from an unfinished aspiration SPSA. | **Closed observation:** the last supplied checkpoint was 8,626 games, +11 pool Elo over 2.3.1 and 39 below Basilisk 1.9.3. It was not completed or accepted as a gate. | Under these conditions a mixed unfinished binary located a possible gap but established neither component value nor a new baseline. Do not extrapolate the partial pool ratings. | `PLAN.md` §1, §3 |
| RAR-M11 | Phase-4 consolidated SPSA go/no-go review. Proposed surface: 30 coordinates, three fixed architecture switches, 10,000 iterations × 32 games (320,000 games), about 79 hours, complete final theta. Setup and resume mechanics were validated; no games started. | **Canceled before launch and removed in 2.3.2.** The architecture bundle had failed to accumulate, its best node-saving pair had no material Elo prior, and the three proposed fixed switches were unaccepted. A valid schedule could at most refine a low-prior HCE surface immediately before NNUE. | Perform the expected-value review before investing in schedule optimization. SPSA is a local constant optimizer, not the missing mechanism for a 50–100 Elo target; clean setup, sunk preparation and a larger coordinate count do not create that prior. The next broad fit is post-NNUE and must select its coordinates/horizon from new activation and curvature evidence. | `PLAN.md` §2–3; `tools/spsa_convergence_model.py`; `tools/spsa_configs/README.md` |
| RAR-M10 | Calibrating this harness's LLR drift so a game budget can be derived rather than guessed. Fitted on three completed `[0,3]` nElo gates at `3+0.03`, 1T, paired UHO, concurrency 14 with affinity: RAR-S31 (+5.24 nElo, LLR 2.96 in 31,822 games), RAR-S29 (−4.95, −2.95 in 18,436) and RAR-S27 (−2.33, −2.19 in 23,044). | **Retained method tool.** `drift per game ≈ 8.3e-6 × (Elo1 − Elo0) × (true_nElo − midpoint)` predicts all three observed drifts within 1% (9.30e-5 vs 9.30e-5; −1.61e-4 vs −1.60e-4; −9.54e-5 vs −9.50e-5). Applied to the `[3,10]` default it gives ~9,200 games for a true 12 nElo, **~14,500 for a candidate exactly on H1 (10 nElo)**, ~33,700 for 8 nElo, and ~14,500 to H0 for one exactly on H0. That is why the default cap moved from 12,000 to **16,000**: at 12,000 the effective bar was about 12 nElo rather than the stated 10. | Under this harness a cap and its bounds must be checked against each other, or the gate silently becomes stricter than it reads. Use the fit to size a budget **prospectively** only; it is a design tool and never a justification for extending a run whose games have been seen. Recalibrate after any change to adjudication, book, TC or the pentanomial model, since `k` absorbs all of them. Three points spanning +5.2 to −5.0 nElo at one bound pair is a narrow basis — treat predictions outside roughly ±6 nElo, or under different bounds, as extrapolation. | `tools/results/sprt_*`; `tools/sprt.ps1`; `PLAN.md` §2 |
| RAR-M09 | Phase-4.1 normal versus diagnostic release builds on the Ryzen 9 5950X, non-PGO, `bench 13` plus four depth-10 positions. | **Retained infrastructure:** both builds matched 6,502,902 nodes / EBF 2.449; the four probes matched nodes, scores, PVs and best/ponder moves. Paired best-of-three NPS was 2,585,646 versus 2,301,097 (11.0% diagnostic cost including legacy exact atomics). | Under these conditions, the sampled observers did not alter the tree and their offline cost was bounded. This does not prove equivalence on every ISA/thread count or make counter movement an Elo proxy; repeat the gate after diagnostic control-flow changes. | `src/diag.rs`; `tools/diag_search_quality.ps1`; Plan 4.1 |
| RAR-M12 | **Phase-4 step 4.0 — baseline and oracle freeze**, 2026-08-12 on the Ryzen 9 5950X. Reproduced the 2.3.2 baseline from `dev` at `5294e2c`, first confirming its code tree is byte-identical to `master` `f931722` (the diff is documentation only, so the build is the released revision). Toolchain `rustc 1.97.1 (8bab26f4f 2026-07-14)`, matching the `rust-toolchain.toml` pin. | **Closed; baseline accepted.** `cargo fmt --check` clean; all-feature workspace clippy clean at `-D warnings`. Tests **258 passed / 0 failed** in debug and **259 / 0** in release, 23 suites each — the one-test difference is deliberate and documented, `random_position_garbage_never_crashes_or_hangs` being `#[cfg(not(debug_assertions))]` because a debug engine spends ~5 s per process start and the property under test belongs to the shipped binary. Release `bench 13` = **6,519,711 nodes / EBF 2.449**, median 149,097, top-position share 6.8% (440,767), 2,939,454 nps. `--features tune` advertises **101** options: all ten options removed in 2.3.2 are absent, and the sampled later-owned inert options (`CorrSkipWhenTtRefined`, `SelectivityProspectiveDepth`, `SingularTtDepthMargin`, `RootConfPoolInstability`, `SmpIterationSkip`) are present. PGO PEXT asset `rarog-v2.3.2-windows-pext-pgo.exe` reproduces the identical fingerprint, SHA-256 `389E234ECCB725D81BEBB4030D4AF17ED181D130F0803E9948B5437E05046E28`; `verify-isa --arch pext` holds (pext 303, avx 6247, zero sse3/ssse3/sse4.1/sse4.2, tzcnt 344 permitted as `rep bsf`). Oracle `hybrid` at `75d0d43` re-verified: both frozen binaries hash byte-exact to the values recorded under the search-oracle section. | This is the revision every Phase-4 candidate gates against; do not re-derive it per cluster. Two conditions are recorded rather than assumed: the profile-dependent test count is a documented `cfg`, not drift, so a future 258/259 reading needs no investigation, while any *other* asymmetry does; and the doc-only equality between `dev` and `master` is what licenses building the baseline from the integration branch — it must be re-checked, not assumed, the moment Phase-4 code lands. **Risk discharged 2026-08-12:** `hybrid` and `spsa_impr` were pushed; `origin/hybrid` matches `75d0d43` exactly, so the oracle no longer lives on one machine. | `PLAN.md` §4 step 4.0; `GUIDE.md` tracker; `tests/fuzz_lite.rs`; `target/dist/rarog-v2.3.2-windows-pext-pgo.exe` |
| RAR-M13 | **Adjudication unified on 600/3 two-sided, 2026-08-18**, by maintainer decision on consistency rather than on new measurement. `strength-v1` (600/3 one-sided) becomes `strength-v2` (600/3 two-sided); `datagen-v1` is unchanged and now carries identical values. Draw rule untouched at `movenumber=40 movecount=8 score=10`. Implemented in `tools/harness_common.ps1` and read from there by `sprt.ps1`, `spsa.ps1`, `setup_tools.ps1` and `datagen.ps1`, so the two profiles cannot drift apart again; the weather-factory patch marker moves V2 -> V3 and its guards were re-pointed. | **Instrument change, no games.** Two-sided requires both engines to agree before a game is called, so this is the conservative direction: fewer adjudications, more games played out, marginally more wall time. RAR-M06's replay over 69,350 games is what makes it cheap — one-sided and two-sided 600/3 differed on **0.20%** of triggers (71 of 35,486) and on **no** final chess result. | A verdict instrument should be one rule, not two justified rules, once the measured difference between them is 0.20% and never changes a result. The 0.20% is also the exact size of the discontinuity between ledger rows: **strength results recorded before 2026-08-18 were adjudicated one-sided.** Do not re-derive a pre-2026-08-18 Elo from post-change games without noting it. Recalibrate the whole rule after any material score-scale change, as RAR-M06 already requires. | `tools/harness_common.ps1`; RAR-M06 |
| RAR-M14 | **Time-forfeit floor at concurrency 14, Threads 1, `3+0.03`, measured from RAR-E06's 3,915-game PGN.** Every game ends having spent **97-99% of its whole clock** (base 3s plus 0.03s per move); the five longest games, 367-494 plies, sit at 97.5-98.7% and do not forfeit. The three that did forfeit were 90/98/121 plies -- **shorter** than the 131-ply median -- and one of them flagged while its own reported move times summed to only **94.5%** of budget. | **Observation.** The forfeits are not clock mismanagement and not long-game exhaustion. They are the gap between engine-reported thinking time and harness-measured wall time, against an aggregate slack of about 2% of a ~4.9s budget -- roughly 100ms for a whole game. A single descheduling event of that size, with all 14 physical cores running engines and fastchess contending for the same silicon, is a forfeit. Rate 0.077%, consistent with 0.135% and 0.172% in two identical-binary null pairs. | `Move Overhead` defaults to 10ms and `time_manager.rs` reserves `2*overhead` only below ~520ms of clock; the `smp_reserve` of 30ms is gated on `threads > 1`, so a single-threaded engine under a saturated runner gets no equivalent protection. Its comment records `0/3,460 at Threads=1`, which no longer holds at this concurrency. Retry trigger: measure forfeit rate against `Move Overhead` on a null pair before changing any default -- a TM change alters playing behavior and needs its own gate. Diagnosis owner **4.2b**; every repair owner **4.12a**. | `tools/results/sprt_HCERefit_vs_HCEBase_20260901_072106.pgn`; `src/time_manager.rs` |
| RAR-M15 | **How often the 20 reference endgames actually occur, and what adjudication does to them.** Replayed all 3,915 RAR-E06 games (no adjudication) and classified every position at <=6 pieces; then re-simulated the games under `strength-v2` (draw |cp|<=10 for 8 moves from move 40; resign |cp|>=600 for 3 moves, two-sided) from the PGN's own eval comments. | **Observation, decisive for 4.9a's gate design.** **52.7%** of games reach a <=6-piece position and 60.9% reach <=7, so endgames are not rare in aggregate -- but the per-family spread is three orders of magnitude. KXK **37.34%**, KRPKR **10.04%**, KPsK 4.19%, KPK 2.84%, KRKP 2.40%, KBPsK 1.92%; then KQKP 1.17%, KRPKB/KPKP 1.23%, KBPKB 0.89%, KBPPKB 0.66%, KRKN 0.61%, KRKB 0.51%; then **KBNK 0.28%** (11 games), KBPKN 0.28%, KNNKP 0.05%, KNNK 0.03%; and **KQKR, KQKRPs and KRPPKRP occurred exactly ZERO times**. Under simulated `strength-v2`, endgames reached fall from 52.7% to **24.9%** -- adjudication destroys **52.7% of all endgames before they are reached**. | **One gating policy cannot cover this range, and a whole-match SPRT is structurally incapable for the tail.** A change confined to a class occurring in 0.28% of games cannot produce a detectable whole-match Elo at any budget this project has; three of the twenty never occur at all, so an endgame-start cohort for them must be CONSTRUCTED, not sampled. Conversely KXK and KRPKR are common enough that a normal no-adjudication STC SPRT can see them, so the pessimism is not uniform either. The adjudication figure is why rule 8 is not merely a preference for endgame work: adjudication halves the sample of the thing being measured. Simulation caveat: the rule is applied to the PGN's eval comments rather than reproducing fastchess's internal bookkeeping exactly, so treat 24.9% as approximate -- the halving is far larger than any plausible error in that approximation. | `tools/results/sprt_HCERefit_vs_HCEBase_20260901_072106.pgn`; 4.9a |
| RAR-M16 | **What adjudication actually buys, measured from stored logs.** Games/minute taken from every stored gate log over 400 games with a recorded total time, split by the manifest's adjudication line; all 1T `3+0.03`, concurrency 14. | **Observation.** Adjudicated runs cluster tightly at **~97.5 games/min** (median over 27 runs, range 94.5-112.9 with the fastest being 2,000-game runs whose fixed startup cost is amortised differently). The one no-adjudication run, RAR-E06, measured **88.4 games/min**: about **9-10% slower**. On RAR-E06's own numbers that is 44.3 minutes instead of ~40, and an 8-hour gate becomes ~8h50m. | **The price of playing games out is far lower than the cost of what adjudication removes, so the default should be no-adjudication and the burden should fall on keeping it, not dropping it.** Adjudication is not *unfair* -- it is symmetric between two arms of the same engine and RAR-M06 found one-sided and two-sided differed on 0.20% of triggers and no final result -- but it is **lossy**, and lossy in exactly the direction this project is now working. It removes conversion and defensive-holding skill from the measurement: RAR-M15 measured it destroying **52.7% of all endgames before they are reached**, and RAR-O01 vs RAR-O02 priced the cross-evaluator adjudication confounder at **74 Elo**. A candidate that is better precisely at converting won endgames is the candidate adjudication is blindest to. Caveat: n=1 for the no-adjudication throughput figure, so treat 9-10% as one measurement rather than a fitted constant. Flipping the harness default is a maintainer decision and introduces a before/after discontinuity of unknown size, unlike RAR-M13's measured 0.20%. | `tools/results/sprt_*.{log,manifest.txt}`; RAR-M06; RAR-M13; RAR-M15; RAR-O01/O02 |
| RAR-M17 | **Adjudication dropped as the harness default, 2026-09-01, by maintainer decision on RAR-M16.** `sprt.ps1` and `gauntlet.ps1` now run with no draw or resign adjudication unless `-Adjudicate` is passed; `-NoAdjudication` is retained and still truthful, so every recipe already recorded here reproduces verbatim. Extended the same day to **every** instrument: `datagen.ps1` moved to a new `datagen-v2` profile with no adjudication, and `setup_tools.ps1` strips both the resign and the draw line from weather-factory's `cutechess.py` (`RAROG_ADJUDICATION_PATCH_V4`), with `spsa.ps1` refusing to start a tune without it while still exempting a resume. `datagen-v1` is retained unedited so `hce-v2`'s manifests keep meaning what they said. | **Instrument change, no games.** Verified in both directions rather than by reading the flag: a bare invocation produced **0 adjudicated terminations in 30 games** and a manifest reading `adjudication: none`, while `-Adjudicate` produced **30 of 30** and the `strength-v2` label. Passing both switches is refused. | **Results before and after 2026-09-01 used different instruments and the size of the discontinuity is UNKNOWN.** This is unlike RAR-M13, whose one-sided-to-two-sided change was backed by a 69,350-game replay showing 0.20% of triggers and no changed result; nothing equivalent has been replayed for adjudication-versus-none, and RAR-O01 vs RAR-O02 suggests it can be large when evaluators differ. Do not difference a pre-change Elo against a post-change one without saying so. Retry trigger: the whole default is worth revisiting once 4.9a closes -- adjudication's loss scales with how badly the engine converts, so it costs a 99% converter far less than a 52% one. | `tools/sprt.ps1`; `tools/gauntlet.ps1`; RAR-M13; RAR-M15; RAR-M16; RAR-O01/O02 |
| RAR-M18 | **`datagen-v3`: Syzygy tablebase truth for labels, 2026-09-01.** Removing eval adjudication (`datagen-v2`) does not make labels truthful; it makes them reflect what the datagen engine can convert at 8,000 nodes. 4.9a.1 measured conversion at 60,000 nodes -- KBN-K **7%**, KRP-KR **52%**, KBB-K 86% -- and 8,000 is worse, so a theoretically won endgame is played out, drawn on the fifty-move rule, and recorded as a draw, mislabelling every position sampled from it. `datagen-v3` adds `-tb -tbpieces 6 -tbadjudicate BOTH` and deliberately keeps the fifty-move rule, so a cursed win is labelled the draw it really is. | **Instrument addition, verified.** A 40-game probe ended **20 of 40 games on tablebase truth** (9 White wins, 3 Black wins, 8 draws), the rest by rules, with the manifest recording `datagen-v3`. | **Truth and realized skill are different measurements and want opposite instruments.** Datagen asks what a position is WORTH, so tablebase adjudication is strictly better than either alternative. A strength gate asks what this engine can actually CONVERT, so the same flag would credit both arms for an endgame only one of them wins -- never use it there. **More nodes is not a substitute and that is measured:** Basilisk's same fit read -2.85 +/- 3.11 on 8k-node outcomes and **+1.00 +/- 2.11, stopped unresolved** on 25k-node outcomes with LTC +0.29 +/- 5.46. About 3x the datagen compute bought a result indistinguishable from zero; reading that +1.00 as an improvement is the RAR-S61 point-estimate error. | `tools/harness_common.ps1`; `tools/datagen.ps1`; RAR-M15; RAR-M17; RAR-S61; `analysis/basilisk_audit_2026-08-30.md` |
| RAR-M19 | **Audit of the SEE / move-ordering piece-value scale, 2026-09-05.** Prompted by the `cross-engine-board-v1` benchmark's threshold-SEE column not being comparable with Basilisk or Manta. Zero games; the evidence is the source, `git log -S`, and the two peer implementations. | **`piece_value()` has not moved since the initial commit while the evaluator was refit four times underneath it.** Three vectors sit on consecutive lines of `src/eval.rs`: `MG_VAL` = 88/394/418/537/1131 and `EG_VAL` = 123/239/290/486/930, both Texel-fitted inside the 1,218-slot surface, and `PIECE_VALUES` = 100/320/330/500/900/`MATE_SCORE`, traced by `git log -S` to `d3f58a2` "Version 1.0.0" (2026-05-22) and never tuned since. RAR-E05, RAR-E06, RAR-E08 and RAR-E12 each moved the evaluator's material and left it alone. Measured blast radius in `src/search.rs`: **10 executable `see_ge` / `see_ge_quiet_aware` sites plus 7 direct `piece_value` uses** in MVV-LVA scores, promotion ordering bonuses and the qsearch delta margin `stand_pat + piece_value(Queen) + 200 < alpha` -- a margin sized on a 900-cp queen while the evaluator's queen is 1131 mg. | **Operating rule 7 already required this audit and it was never run:** after an HCE changes, cp-valued search consumers are audited and, if justified, fitted separately. The peers show the two coherent designs and Rarog has neither -- **Manta** parameterises SEE (`see.PieceValues` as a comptime argument), injects the contract's 100/300/300/500/900/20000 into its benchmark and passes its own fitted `mg_val` = 84/323/364/514/1085 in production; **Basilisk** hardcodes a dedicated `SEE_VALUES` table in `board.cpp` that already equals the contract, so its bench needs no injection. Rarog reuses a legacy evaluation constant and can do neither. **This is not a regression**: every accepted SPRT was played with these values, so current strength already includes them; the open question is whether the coupling costs Elo. Owner **4.15.3** (zero-game audit), **4.15.4** (give the values an owner and a tunable surface, gate it), **4.15.5** (restore the benchmark column), with the vector joining **4.16**'s SPSA surface if 4.15.4 exposes it. | `src/eval.rs:30-33`; `benches/board.rs` header; `D:/code/manta/src/eval/hce.zig:125`; `D:/code/basilisk/src/board.cpp:1604`; PLAN 4.15.3-4.15.5 |

| RAR-M20 | **Board audit and native three-engine comparison, 2026-09-05.** Rarog ca03a46; Basilisk d734766; Reckless 91b56c2 plus the complete benchmark-only adapter. Native optimized non-PGO builds, Ryzen 9 5950X, affinity mask 4; three cyclic rounds, 150ms warmup + 11x150ms per workload. | **Basilisk faster in all five comparable workloads.** Median M ops/s Rarog/Basilisk/Reckless: legal moves **447.131/642.646/339.844**; captures **98.204/120.138/61.597**; generation+make/unmake **42.521/55.031/23.494**; perft **273.741/382.726/177.944**; two-ply simulation **351.809/513.537/246.626**. Native SEE **46.676/58.814/39.722** is NOT comparable: value vectors/contracts differ. Confirmed SEE king-exchange defect (-400/true instead of -300/false at zero), Unicode move parser panic, fullmove debug overflow/release wrap; no Rarog fix or games in this audit. | Owner **4.11b**, correctness then HCE profile and bounded optimization. Every Basilisk round beat every Rarog round, which beat every Reckless round, in the five comparable columns; active desktop load 6.25–9.17%, substantial scatter in some cells, no small-gain or Elo inference. Reckless uses **NullBoardObserver**, so this excludes NNUE arithmetic and does not isolate NNUE-related board cost. Preserve raw data for **5.2.1**, measure move-event/scaffold costs at **5.2.5/5.3.4**, actual-network update/inference at **6.4.3**. Keep 4.15 production fitting separate from 4.11b.6's neutral value injection. | `analysis/board_audit_2026-09-05.md`; `analysis/board_benchmark_recipe_2026-09-05.md` embeds exact builds, complete adapter patch, source/binary hashes, runner and nine raw outputs; machine-readable `analysis/artifacts/board-audit-20260905/manifest.json`. Archived binaries remain at `D:/chess/results/board-audit-20260905/binaries/`; recipes and results do not depend on a candidate branch. |

**RAR-M21 — 4.11.7 budget transfer, registered 2026-09-05; COMPLETE 2026-09-06.**
Baseline `6e8044a`, exact production features (empty), bench 13
6,901,489 / EBF 2.458. Frozen Stockfish 18 reference; full corrected 19-family
cohort, 100 positions/family, seed 6200600, 60k/200k/600k nodes/move,
100-ply cap, Hash 16, Threads 1, engine TB disabled, 30 workers.
Hypothesis: the existing per-family conversion deficits persist at deployment-
representative budgets. This is a diagnostic, not an SPRT or an acceptance
gate; no engine change is proposed. Stop on failed commands, changed cohort,
or failure to reproduce either historical 60k family report; investigate
before advancing. Complete all registered budgets otherwise. Protocol and
decisive cases: `analysis/endgame_budget_transfer_2026-09-05.md`.
Exact commands, binary/harness hashes and exit statuses:
`tools/results/budget-transfer-20260905/manifest.json`, preserved with all raw
reports in `analysis/artifacts/budget-transfer-20260905.zip`;
reproduction driver: `python tools/diag/run_4117_registered.py` (build/input
prerequisites and protected output paths are documented in the analysis).
**Result:** both fresh 60k reports reproduce exactly; all six commands succeed.
Rarog converts **1276/1336/1346 of 1372** at 60k/200k/600k, Stockfish
**1361/1363/1362**: net deficit **85/27/16**. KBN-K and KQ-KP reach full
conversion at both higher budgets; a persistent conversion-defect claim from
their 60k result is not supported. KQ-KR remains behind **23/13/3**;
KNN-KP **9/6/8**, non-monotone. KRP-KR, KRP-KB, KBP-KN and KP-KP also
retain deficits across the bracket. Paired Rarog gains/losses are **70/10**
then **19/9**, so aggregate improvement does not imply per-position dominance.
**Disposition:** close 4.11.7; preserve v2's frozen 60k ranking, attach this
budget qualification to 4.11b.18/4.12.1 and the family owners. Static-draw
overclaims and historical matched-arm refit/mate-drive debts are not cancelled.
No engine implementation or strength gate. Debug/release tests, fmt, Clippy,
156 tooling tests and report/byte-level archive validation passed.

**RAR-M22 — 4.11.8 datagen label audit, COMPLETE 2026-09-06.** Hash-verified
the two 8,000-node `hce-v2` PGN segments (600,000 games total) and the
8,000-node `hce-v3` source PGN of `hce-v3-tb` (602,619 games), then audited
each game's first 3–6-man Syzygy clean win against its final PGN result. Cursed
wins excluded; only the first clean win per game counted. **Result:** hce-v2
has 26,316 not won / 134,948 clean wins (**19.50%**, **4.39% of all games**);
hce-v3 source 54,186 / 266,490 (**20.33%**, **8.99% of all games**). Both
budgets are 8,000, so no budget comparison is available. This is game-level
raw-label evidence, not a remaining row-error count: `hce-v3-tb` already
Syzygy-corrected 125,643 ≤6-man CSV rows and still leaves >6-man rows unchanged.
**Disposition:** the raw-game labels are materially biased toward draws;
4.13.1 owns row-level lineage and separate post-hoc/whole-game contracts. No
refit, engine change or strength claim. Full analysis and byte-preserved output:
`analysis/datagen_label_audit_2026-09-06.md`,
`analysis/artifacts/datagen-label-audit-20260906.zip`.

**RAR-M23 — 4.11.9 mate-drive promotion closure, COMPLETE 2026-09-06.**
Recompared the hash-pinned pre-4.9a.4 and accepted-mate-drive reports on their
identical 19-family, 100-position-per-family, 60,000-node, seed-6200600 cohort;
the derivation asserts equal schema, Syzygy path, budget, seed, family set,
FEN, index and Syzygy truth before comparing `mated` conversion. Six families
change: direct KBB-K +22 and KBN-K +76; promotion-reached KPP-K net 0, KBP-K
net +2, KBP-KB net **-1**, KBP-KN net **-1**. **Disposition:** the two
negative-net families are causal debt owned by 4.12.7/4.12.9. Their paired
gains do not cancel a family loss; nonnegative-net KPP-K/KBP-K remain closure
guards. The reports use the pre-4.10 material-shed instrument, so this is not
a current conversion floor or a rollback case. No engine change, game or
strength claim. Full matrix and byte-preserved inputs/derivation:
`analysis/mate_drive_promotion_closure_2026-09-06.md`,
`analysis/artifacts/mate-drive-promotion-closure-20260906.zip`.

**RAR-M24 — 4.11.10 corrected conversion claims, COMPLETE 2026-09-06.**
Reran the preserved RAR-E08 baseline/head and RAR-E08/E12-candidate binaries
through the repaired v2 truth runner, requiring matching schema, conditions,
cohort, FEN/index and Syzygy truth before every difference. **RAR-E08's v1
aggregate 83.24% -> 83.45% is superseded:** the corrected pair is
**1255/1372 = 0.9147 -> 1254/1372 = 0.9140**. Its four-family 400-position
result is retained under v2, including **KQ-KP 390/396 -> 375/396, -3.79 pp**;
that is a real historical causal debt for 4.12.13. RAR-M21 qualifies it: the
current head's KQ-KP 60k shortfall closes at 200k and 600k. **RAR-E12's v1
0.8345 -> 0.8477 is superseded:** corrected aggregate conversion is
**1254/1372 = 0.9140 -> 1278/1372 = 0.9315** (+24). KQ-KP DTZ progress rises,
but its conversion falls 96/98 -> 94/98, so the former “debt repaid” statement
was overbroad. RAR-E11 stays superseded in full: corrected reference is
1361/1372 = 0.9920, current head 1276/1372 = 0.9300, and reference is worse in
no family. No engine change or strength claim; both accepted Elo verdicts
remain valid. Full derivation and byte-preserved reports:
`analysis/conversion_claims_correction_2026-09-06.md`,
`analysis/artifacts/conversion-claims-correction-20260906.zip`.

**RAR-M25 — 4.11b.2 board-v2 instrument and correctness corpus, COMPLETE
2026-09-06.** Added a versioned ten-position profile generated and checked by
`python-chess` 1.11.2. It mechanically requires real single/double checks and
evasions, legal/pinned-illegal EP, quiet/capture underpromotions, all four
castles, and sparse long-history material. Rarog now checks exact canonical
FEN, sorted legal/capture UCI identities, perft/divides, keys, occupancy,
pieces and full restoration through normal/hinted/staged/null/clone/unwind
paths. A negative-control test corrupts a legal move, perft count and board
state and requires the preflight to reject each. Coordinate-ray coverage
passes every relevant slider occupancy on magic and PEXT, debug and release.
The independent `rarog-board-v2` benchmark is not the frozen cross-engine-v1
benchmark: it reports separately precomputed generation, mutation and SEE
workloads, raw samples, a portable `black_box` output checksum, and a warmed
allocation guard that observed zero allocations. The archived magic run has
the compiler, host, flags, input hashes and raw samples at
`analysis/artifacts/board-v2-20260906/`. **No engine change, cross-engine
comparison, NPS conclusion or strength claim.**

**RAR-M26 — 4.11b.3 parser and fullmove boundaries, COMPLETE 2026-09-06.**
The previous `Move::from_uci("aé1")` passed its four-byte length check and
panicked when slicing through UTF-8. It now refuses every non-ASCII token
before indexing; an actual release UCI `position startpos moves aé1` exits 1
with the existing `CRITICAL ERROR` diagnostic, rather than aborting. Fullmove
is intentionally bounded to `u16`: FEN `0` remains compatible and normalizes
to 1, `65535` is accepted and `65536` is rejected. At the maximum, real and
null black moves saturate, white moves retain the counter, and both undo paths
restore the original FEN/state. The new tests failed first on the debug
overflow and UTF-8 panic, then passed in debug and release; full suites,
fmt and all-feature/all-target clippy pass. A rebuilt default-feature
production `bench 13` is exactly **6,901,489 / EBF 2.458**. This is a
correctness repair for malformed and extreme input, not an NPS or strength
claim.

**RAR-M19 ownership update, 2026-09-05:** its historical result above is
unchanged. Behavior-neutral value injection and initial normalized SEE
benchmark restoration now belong to **4.11b.6**. Post-final-HCE scale audit
and fitted production policy remain **4.15.3–4.15.4**; **4.15.5** revalidates
the normalized benchmark after fitting. Exposing a benchmark input does not
itself authorize production tuning or SPSA.

**RAR-M27 — 4.11b.4 SEE contracts and independent fixtures, COMPLETE
2026-09-06.** Baseline `6d1a670`, engine `a170f8c`; no production changes.
Ten threshold calls (two diag-only) and one full-SEE call inventoried with
their ordering/pruning/LMR/history consumers. Eighteen python-chess legal
same-square capture-tree fixtures, independently hand-scored, expose three
debts: king-after-pawn -300 vs current -400/true, newly created pin +100 vs
-230/false, recapture promotion -800 vs 0/true (booleans at threshold zero).
All three named pending acceptance tests fail in debug and release; repair
owner 4.11b.5. Ordinary quiet/quiet-promotion immediate-gain policies remain
explicit, with quiet-aware handling only for ordinary quiets; quiet promotions
have no current production SEE caller. Scoped Rust checks pass 8 tests per
profile, with the 3 debt tests intentionally ignored; five Python oracle
checks, fmt and all-feature/all-target clippy pass. No games or speed claim.
Exact FENs, arithmetic, caller contracts, raw observations and reproduction:
`analysis/see_contract_2026-09-06.md`, `tests/data/see-contract-v1.tsv`,
`analysis/artifacts/see-contract-20260906/`.
**Defect status SUPERSEDED by RAR-M28:** the three failures above are historical
baseline observations; all three acceptance tests are now active and passing.

**RAR-M28 — 4.11b.5 SEE legality and promotion repair, COMPLETE 2026-09-07.**
Engine/test `fce0b44`, entry `e954e38`. Current-occupancy king safety replaces
stale pin masks; king captures terminate legally, recapture promotions include
the promotion gain and promoted victim value, and threshold comparisons preserve
equality. Values remain 100/320/330/500/900/20000; quiet/promotion shortcut
policies remain explicit. The three repaired full/threshold-zero results are
**-300/false, +100/true, -800/false**. Forty-one independent legal-tree fixtures
and 1,802 legal-capture parity checks pass. Full suites pass **268 debug / 269
release**, zero failures/ignores; six Python tests, fmt and Clippy pass.
Exact-feature production bench is **7,601,220 / EBF 2.474**, +10.14% nodes
against 6,901,489 / 2.458. Development fingerprint updated; **no strength or
comparative NPS claim**, no games. Playing qualification remains 4.11b.17.
The first process-argument bench invocation was a no-op, detected from its
missing summary; it is explicitly invalidated and replaced by a hash-verified
UCI-driver run. Reproduction, source/binary identity, raw logs and exact engine
diff against the entry source: `analysis/see_repair_2026-09-06.md` and
`analysis/artifacts/see-repair-20260906/`.

**RAR-M29 — 4.11b.6 neutral SEE injection and normalized timing, COMPLETE
2026-09-07.** Engine/test `46f1af2`, entry `2c59911`. `SeeValues` owns the
board scale; production remains 100/320/330/500/900/20000 with no runtime
engine option. Explicit production and normalized 100/300/300/500/900/20000
injection each pass all 41 independent fixtures. Complete suites pass **270
debug / 271 release**, zero failed/ignored; seven oracle tests, fmt and Clippy
pass. Exact production `bench 13` remains **7,601,220 / EBF 2.474**.

The benchmark's deliberately absurd rook=1 input flips its independent probe
false -> true, proving the wire. All three adapters report identical normalized
values and ten move/verdict answers. Three cyclic rounds give threshold-SEE
medians Rarog/Basilisk/Reckless **44.923/58.335/40.823 M captures/s**: Basilisk
is +29.86% over Rarog and Rarog +10.04% over Reckless by median. Rarog's round
span is **12.20%** and it loses the third round to Reckless, so magnitudes are
directional; all timed host-busy checks pass. RAR-M20's native SEE row is
**SUPERSEDED for ranking**, not deleted. Its vectors differed and Rarog's
kernel changed at 4.11b.5, so it is not an injection-overhead baseline.

**RAR-M19/RAR-M27/RAR-M28 correction:** board SEE's king sentinel was already
20,000, not `MATE_SCORE`/32,000; those records conflated it with eval's separate
piece-value vector. Kings are never legal SEE victims, so no exchange result
changes. No value fit, games, NPS or Elo claim. Evidence:
`analysis/see_value_injection_2026-09-07.md` and
`analysis/artifacts/see-normalized-20260907/`. Production fitting remains
4.15.3–4.15.4. At this closure point, 4.11b.7 was the next leaf.

**RAR-M30 — 4.11b.7 full-search board profile, COMPLETE 2026-09-07.** Source
`02420dc`, 20 frozen roots in five cohorts, 600,000 nodes, three counter and
five ETW repeats. Production SHA-256 `3c81ef95...bf1d904dfd0`; diagnostic
`aaeda618...25d42e1`; all **60/60** instrumentation-off searches match depth,
seldepth, reported nodes, score type/value and best move; PV and ponder move
were not compared. All 151,142 engine samples resolve from the archived PE/PDB.
Weighted process shares are generation/legality **6.751%**,
make/unmake **7.143%**, check queries **5.177%**, SEE **5.304%**; relocation
helpers are an overlapping **2.998%**, king lookup **0.544%**. Over 30,604,224
diagnostic nodes, checked makes are 89.02% of real makes, threshold SEE is
92.77% of SEE calls, and 25,718,154 history pushes cause **zero growth**.
No games, NPS acceptance, or strength claim. At closure, 4.11b.8 was next; evidence,
full hashes, time budget and reproduction:
`analysis/board_search_profile_2026-09-07.md` and
`analysis/artifacts/board-search-profile-20260907/summary.json`.

**RAR-M31 — 4.11b.8 pin discovery measurement, recorded 2026-09-07;
research disposition CLOSED: candidate withdrawn in `c44608a`.**
Baseline `407de51`, engine `2ea279f`; replace four x-ray slider lookups by two
empty-board lookups and test all occupied squares between king and aligned
enemy slider. Keep a sole friendly blocker as pinned. Local board-v2 median
gains over three alternating rounds: legal **+8.54%**, capture **+11.43%**,
staged **+7.41%**. Twelve alternating full-search pairs on each backend measure
generic **+0.57%** (bootstrap 95% interval −0.51% to +1.00%) and PEXT **+1.45%**
(−1.57% to +4.26%); **neither establishes a whole-search gain**. PEXT host load
varied more. Non-PGO, 1T, Hash 16 MiB, frozen 20 roots, 600k node limit.
All four production fingerprints are **7,601,220 / EBF 2.474**; 480 paired
root answers match including PV and ponder. Independent pin-ray oracle,
debug/release suites, PEXT board tests, fmt and Clippy pass. Retained for local
generation gains under the original execution contract; no games or strength
acceptance. The later `b592b40` research card requires a prospective practical
whole-search floor, which this run did not register. It does not qualify the
leaf under that new contract. **Original retention decision SUPERSEDED:**
restore the prior x-ray algorithm and retain its independent oracle. Decline
another standalone campaign before shared-geometry research; this is a research
prioritization decision, not a statistical finding of no gain or a post-hoc
floor. Later cache/search changes were not measured in the timing study.
Restoration against `b90232b`: 274 debug / 275 release tests, fmt and Clippy
pass; fresh no-feature before/after builds reproduce 7,601,220 / EBF 2.474;
20 roots match standard harness identity fields (not full PV/ponder). No new
performance or Elo claim. 4.11b.10 owns any justified, prospectively registered retry.
Playing gate remains 4.11b.17.
Recipe, hashes and raw observations: `analysis/movegen_2026-09-07.md` and
`analysis/artifacts/movegen-20260907/`.

**RAR-M32 — 4.11b.9 fused ordinary relocation, 2026-09-07; VOID, SUPERSEDED BY
RAR-M33. Its `NO_CHANGE` disposition is WITHDRAWN.** The run was taken while a
Manta SPRT held the host at **50.2–53.4% CPU busy** per arm against 3.7–5.8% for
the comparable 4.11b.8 run; re-measured idle, the same baseline code runs at
3,071,903 nps versus this run's 2,182,590 nps. The full-search timing conclusion
is withdrawn; the deterministic findings below (fingerprint parity, 240 paired
root answers, emitted-code comparison) stand. Original record follows.
**RAR-M32 original text —** Baseline `af83abf` on
`dev`; qualification frozen in `86e39f8` **before** any timing. Candidate fuses
ordinary `QUIET` make/unmake relocation into one from/to mask and one paired
key across mailbox, piece/colour occupancy, `all_occ` and the pawn/minor/
non-pawn keys; captures, double pushes, en passant, promotion, castling and
null moves keep their existing paths. Semantics were exact: both no-feature
builds reproduce **7,601,220 / EBF 2.474** (asserted in-runner before timing),
and **240 paired root answers** (12 pairs x 20 roots) match on name, repeat,
depth, seldepth, reported nodes, score type/value, best move, **full PV and
ponder**. Executables are distinct — baseline `fde1ed0e...bf59a4` (the
registered hash), candidate `0da54ca9...cfd9dcf3`, board arms `72e8be2c...`
and `2166a33e...`; frozen suite `0c8cefdf...6b153e3`. Isolated `make/unmake
only` gained **+16.28% / +15.21% / +15.27%** over three alternating board-v2
rounds, meeting the registered local condition; unchanged noise-control columns
moved by mixed sign and smaller magnitude. Twelve alternating full-search pairs
(600,000 nodes, one discarded warm-up per arm, seed 4119, all pairs retained)
measure a **+1.016%** median, bootstrap 95% **-0.450% to +3.609%**, 10/12
candidate-faster, max host CPU busy 53.43%. **The interval includes zero, so
the frozen retention rule rejects.** Emitted-code screen: `make_move_inner`
**468 -> 568** instructions (+21.4%), whole-crate 87,294 -> 87,994 (+0.80%),
symbol count unchanged — refuting "LLVM already fuses this" while supporting
"larger code repays part of the saving". RAR-M30's 7.143% make/unmake share
projects the primitive gain to +0.96% whole-search versus the measured +1.02%,
so the mechanism behaved as predicted and the miss is **instrument power**, not
mechanism: twelve pairs cannot resolve a ~1% effect. This is insufficient
evidence of deployable value, **not** proof of zero benefit, regression or
defect. No games, NPS acceptance or Elo claim. `src/` restored byte-identical
to `af83abf`; the targeted per-piece-class relocation test is retained in
`8a73cfd`. Closure on the restored tree: fmt exit 0, **275 debug / 276 release**
tests pass, Clippy `--all-features --all-targets` zero warnings. Retry is not
authorized standalone; it belongs to **4.11b.16** under a pooled-PGO build with
a precision calculation and whole-search floor registered before the run.
Playing gate remains 4.11b.17. At this closure point, 4.11b.10 is next.
Recipe, hashes and raw observations: `analysis/relocation_2026-09-07.md` and
`tools/results/relocation-411b9/` (ignored, local).

**RAR-M33 — 4.11b.9 fused ordinary relocation re-measured on a verified-idle
host, COMPLETE 2026-09-07; ACCEPTED and integrated in `5c439da`.** Baseline
`1d720af` on `dev`; contract frozen in `tools/results/relocation-411b9-v2/
registration.md` and reproduced in the analysis document **before** the
candidate was compiled. Candidate re-implemented from the PLAN handoff because
RAR-M32 saved no patch; scope is **`flags == QUIET` only**, adding
`Board::move_piece` to update both mailbox endpoints, the piece and colour
occupancies, `all_occ` and the applicable pawn/minor/non-pawn keys with one
from/to mask and one paired key, with captures, double pushes, en passant,
promotions, castling and null moves untouched and the position hash still
caller-owned. Prospective prediction: full-search median **+0.7% to +1.2%**
from RAR-M30's 7.143% make/unmake weight and the +15.27% primitive gain
(projection +0.96%); isolated gain +14% to +17%; bootstrap half-width 0.33% to
0.46%. Instrument **32 alternating pairs at 1,200,000 nodes**, one discarded
warm-up per arm, seed 4119, all pairs retained, non-PGO, 1T, Hash 16 MiB,
frozen 20-root suite. The runner now **asserts** host idleness instead of
annotating it, and the gate was proven live by aborting under a deliberately
absurd `0.0` threshold; the gate was recalibrated 10% -> 15% before any timing
against a measured 5.52% ambient on a 32-thread host, changing an
instrument-validity precondition only and leaving the acceptance rule frozen.
Results: both builds fingerprint **7,601,220 / EBF 2.474**; **640/640** paired
root answers match on depth, seldepth, reported nodes, score type/value, best
move, **full PV and ponder**; host busy min 5.41 / mean 6.67 / **max 11.80%**;
isolated `make/unmake only` **+16.33 / +17.30 / +19.32%** across three
alternating rounds; full-search median **+0.876%** (3,071,903 -> 3,098,821 nps),
95% bootstrap **[+0.050%, +2.055%]**, 23/32 candidate-faster. **The interval
excludes zero, so the frozen rule retains the candidate.** Emitted code
`make_move_inner` **468 -> 542** instructions; RAR-M32's archived candidate was
568, so this is the same mechanism with leaner codegen, reported as
corroboration rather than exact reproduction. Artifacts distinct: baseline
`62ac2599...`, candidate `d364f2ad...`, board arms `afd11222...` and
`e20b865a...`; the baseline `.s` hashes identically to RAR-M32's
(`75ed0249...`), proving the baseline source state reproduced exactly.
Calibration: magnitude HIT (+0.876% against a predicted +0.7–1.2%), isolated
gain HIT but slightly under-predicted (one round at +19.32%), **interval width
MISS** — actual half-width **1.003%** against a predicted 0.33–0.46%, because
the projection scaled a bootstrapped median's variance by naive `sqrt(n)`.
A post-hoc 200-seed sweep excludes zero in **198/200** (lower bound -0.017% to
+0.101%), characterising robustness without altering the frozen verdict; the
evidence supports "the gain is above approximately zero", not a bankable floor
of 0.9%. Behaviour-neutral, so no game gate is owed and **no Elo is claimed**;
cluster playing qualification remains 4.11b.17. Debug 275 / release 276 tests,
fmt and Clippy `--all-features --all-targets` clean; a fresh no-feature build of
the committed source reproduces 7,601,220 / EBF 2.474. At this closure point,
4.11b.10 is next. Recipe, hashes and raw observations:
`analysis/relocation_2026-09-07.md` and `tools/results/relocation-411b9-v2/`
(ignored, local; `candidate.patch` archived there).

**RAR-M34 — 4.11b.10 shared pin/check information, COMPLETE 2026-09-08;
research disposition `NO_CHANGE`.** Source `33c373c` on `dev`. Closed on
**structure, not cost**: the three producers share no work. `compute_pinned`
queries from our king against their sliders, `check_info` from their king
against our sliders — different square, different piece sets, only `all_occ`
in common — so no cache of either can serve the other in any node.
`see_recapturer` queries `attackers_to_color(king, after, !side)` against the
**evolving** exchange occupancy, so reusing a real-position pin or attack mask
is the stale `see_pins` defect repaired at 4.11b.5: a correctness boundary, not
a tradeoff. The cross-ply candidate (parent `check_info` versus child
`compute_pinned` at the same king square after the side flip) fails on changed
occupancy and on a different predicate — either-colour sole blocker versus
sole friendly blocker behind an x-ray. The one real sharing opportunity, one
pinned set per node across capture and quiet stages, was already delivered by
the 10.3 speed pass and is measurably active: **422,246** staged quiet
generations cost **zero** extra `compute_pinned` calls. Exact counters
(`bench 13`, `RAROG_DIAG_SAMPLE_STRIDE=1`, diag build `6b6c3e18...`, nodes
**7,601,220** unchanged so the instrument does not perturb the search):
`board_see_threshold_calls` 7,547,296 (**0.993/node**),
`board_gives_check_fast_calls` 25,540,503, `board_check_info_calls` 2,245,089
(0.295/node), `board_compute_pinned_calls` 2,079,992 (0.274/node),
`board_calculate_checkers_calls` 1,155,770, `board_see_full_calls` 445,100.
Units were reconciled before differencing: generator calls 2,243,478 minus
`compute_pinned` 2,079,992 leaves **163,486**, exactly the `generate_captures`
early-out that increments its counter and returns before computing pins. No
ETW re-profile was requested — it needs an elevated prompt, is a maintainer
job, and cannot make un-shareable work shareable; the post-`5c439da` share
update is derived arithmetically and moves every unchanged region by at most
**0.06 percentage points**, explicitly not a measurement. No implementation,
no games, no timing claim and no Elo claim. Owed to 4.11b.11: a fresh profile
where size decides, and an SEE re-baseline, since the 4.11b.9 board benchmark
showed `threshold SEE only` down 1.77/0.98/1.68% in all three rounds —
consistent-signed, most likely code layout after `make_move_inner` grew
468 -> 542 instructions. Reopen only for a genuinely new consumer of pin or
check geometry, never on donor-engine similarity. At this closure point,
4.11b.11 is next. Evidence: `analysis/pin_check_sharing_2026-09-08.md` and
`tools/results/pinshare-411b10/` (ignored, local).

**RAR-M35 -- 4.11b.11 incremental SEE attacker maintenance, COMPLETE 2026-09-08;
`NO_CHANGE`, production path withdrawn.** Baseline `8d7da2c` on `dev`; contract
frozen in `tools/results/see-kernel-411b11/registration.md` before any timing,
with correctness gates run first and no throughput number observed at freeze.
The candidate carried an all-colour attacker set (`attackers_to(target, occ) &
occ`) built once per exchange, selected via `attackers & color_occ(side)` --
exactly reproducing `attackers_to_color`, since each colour-specific term of
`attackers_to` is a subset of that colour's occupancy -- and extended by
`see_expose`, which adds only the ray vacating the source can open (diagonal for
pawn/bishop/queen, orthogonal for rook/queen; a knight attacking the target is
never aligned with it, and a king recapture ends the exchange first). All
4.11b.5 semantics were preserved, including the per-candidate selected-king
legality test, the `& !Bitboard::from(target)` exclusion, promotion/new-victim
values, threshold parity, and retention of an illegal candidate in the carried
set. **Correctness was exact and verified beyond the fingerprint**: `bench 13`
7,601,220 / EBF 2.474; all 41 external fixtures (`see_contract` 8/8),
`see_pins` 6/6, debug 275/275, release 276/276, fmt and Clippy `--all-features
--all-targets` clean; plus a `debug_assert_eq!` comparing the carried set with a
fresh `attackers_to_color` on EVERY SEE call, **proven live** by deliberately
dropping queen orthogonal exposure, which made
`threshold_parity_on_deterministic_legal_walks` panic immediately. Rejection is
on throughput. The registered two-stage design required stage 1 -- three
alternating `board_v2` rounds -- to improve `threshold SEE only` in all three
rounds with a median of at least +5%. Measured **-2.92% / -10.42% / -0.69%**,
median -2.92%, zero rounds up, host ambient 5.38%. **Stage 2 was therefore never
run**, saving the entire expensive full-search arm. Round 1 was disturbed on
unrelated columns (`make/unmake only` -4.79%), so the honest effect is rounds 0
and 2: a **1-3% regression**; no round was discarded. Calibration: the
registration named this exact failure mode before exposure -- short exchanges
gain nothing because the initial `attackers_to` builds both colours where the
old first step built one, and `see_ge_impl` exits early on much of its 7.55M
threshold calls -- so direction was a HIT while the predicted +5% to +20% upside
band was a MISS. **A leaf premise is corrected**: the two `attackers_to_color`
calls per exchange step are not duplicates. The second is the mandatory
per-candidate king-legality test at a different square under a different
occupancy, which a carried target-attacker set cannot serve; future SEE work
must target that test, not the attacker set. `src/` was restored byte-identical
to `8d7da2c` with the fingerprint re-verified after withdrawal. No games, no
Elo, no timing claim retained. A fresh ETW profile is still owed and is now more
valuable, since it can attribute SEE's 5.3% between recapturer rebuild and
legality test; it needs an elevated prompt and is a maintainer job. At this
closure point, 4.11b.12 is next. Evidence:
`analysis/see_kernel_2026-09-08.md` and `tools/results/see-kernel-411b11/`
(ignored, local).

**RAR-M36 — full-search board profile refreshed at head, COMPLETE 2026-09-08.**
Source `2d621ff`; production `a3cca8dc...`, PDB `c61e93e3...`, 162,846 process
samples, five cohorts, 600,000 nodes, 5 repeats. **Recipe recovered.** RAR-M30's
per-sample attribution was a side effect of xperf failing to discover the PDB;
`952711f` fixed that discovery and silently switched the report to per-function
aggregation, where board work inlined into `negamax`/`evaluate` is charged to
those functions and `summarize_board_search_etw.py` — reading a fixed column
that had been correct for the per-address table — resolved `limit`, the byte one
past each function's end, while reporting "100% of engine samples resolved". The
working recipe is to deny xperf symbols on purpose: empty `_NT_SYMBOL_PATH`,
empty `_NT_SYMCACHE_PATH`, and no `rarog.pdb` beside the executable; an empty
symbol path alone is insufficient because xperf reuses its symcache and dbghelp
finds an adjacent PDB first. The PDB must then be restored beside the executable
for llvm-symbolizer, which resolves by the embedded name rather than `--pdb`.
Both directions are now detected from the DATA (`base == limit, size == 0` is
per-address and accepted; non-zero size is per-function and refused with the
regeneration recipe). Refreshed shares against RAR-M30: generation/legality
**6.556%** (6.751%), make/unmake **6.677%** (7.143%), SEE **5.239%** (5.304%),
check queries **5.179%** (5.177%); mechanisms piece relocation **2.752%**
(2.998%), gives_check **1.654%**, check_info **1.026%** (0.912%), compute_pinned
**0.979%** (1.003%), king square lookup **0.502%** (0.544%). **The instrument
validates independently**: RAR-M33's +0.876% whole-search from an ~18% local
make/unmake gain requires that region to be ~6.3%, and this profile reads 6.677%
against RAR-M30's 7.143% — the drop is 4.11b.9, measured by a second instrument
that knew nothing about it, while check_queries reproduces to within 0.002pp. A
**stale mechanism marker** was found and fixed: `piece_relocation_helpers` keyed
only on `::remove_piece`/`::add_piece` and under-read at 1.419% after 4.11b.9
fused the QUIET path into `Board::move_piece`; with `::move_piece` added it
reads 2.752%. The symbolized per-function view additionally shows
`see_recapturer` at **4.35%** exclusive against `see_ge_impl` at **0.87%** —
independent measured support for RAR-M35's conclusion that SEE cost sits in the
per-candidate king-legality test, not the attacker set. It cannot split the two
`attackers_to_color` calls inside `see_recapturer`; that needs a counter or an
`#[inline(never)]` probe. No engine change, no games, no Elo claim. Evidence:
`analysis/board_search_profile_2026-09-08.md`.

**RAR-M37 — 4.11b.12 king-square caching, COMPLETE 2026-09-08; research
disposition `NO_CHANGE`, no prototype built.** Source `edfb35b` on `dev`. The
leaf's conditional trigger — material cost remaining after shared-geometry work
— is not met: RAR-M36 reads king-square lookup at **0.502%** (RAR-M30 0.544%),
and 4.11b.10/4.11b.11 both closed without touching the board, so the small move
is only 4.11b.9 re-weighting shares. **The register asked for a predeclared
practical floor and none was registered**; the measurement is already exposed
twice, so declaring one now would be choosing a number to fit a result, and the
decision deliberately does not use one. It rests on instrument capability: the
2x-local whole-search ceiling is **0.25%**, against RAR-M33's measured bootstrap
half-width of **1.003%** and RAR-M35's projected 0.6-0.7%. The best possible
version is two to four times smaller than the uncertainty of the gate that must
accept it, so it cannot reach `LOCAL_QUALIFIED` however written. The realistic
gain is smaller than the ceiling — `king_sq` is one bitboard load plus a
`tzcnt`, a cache swaps the `tzcnt` for a field load and removes no memory
access, and 0.502% is an overlapping share already counted inside the
generation, check-query and SEE regions. Cost side: maintenance through castling
but not promotion or null moves, restoration on undo for both colours, copying
on worker cloning, and new fields in independent consistency reconstruction —
the class of derived state whose staleness went undetected in 4.11b.5. Retry
trigger: king lookup above **2%** in a profile AND a caller that invokes it in a
loop rather than once per node; donor similarity is explicitly not a trigger. No
engine change, no prototype, no games, no Elo claim. At this closure point,
4.11b.13 is next. Evidence: `analysis/king_square_cache_2026-09-08.md`.

**RAR-M38 — 4.11b.13 history capacity and mutation contracts, COMPLETE
2026-09-08; tightened and integrated in `f70ac19`.** Baseline `745976b` on
`dev`. Behaviour-neutral, **no speed claim in either direction**, per the
register's own condition that zero observed growth events cannot support one.
`Board::reserve_history` (`pub(crate)`) reserves further make/unmake pairs, and
`search_impl` reserves **`MAX_PLY`** on the root once before any hot path or
helper exists; `Board::clone` preserves capacity, so every worker's
`root.clone()` inherits the reservation and no thread reallocates while
searching. The history stays a `Vec`: a clamped fixed array would silently drop
repetition evidence in a long game. **The gap was real but invisible to the
instrument that looked for it** — peak depth is `game_plies + search_depth`, so
an ordinary 64-move game hits `len == capacity` at 128 and reallocates on the
next search's first push, yet RAR-M30 measured zero growth across 25,718,154
pushes because `bench` builds every position from FEN and leaves game history
empty. `is_legal` audited: `Move::from_uci` always yields `QUIET`, so only
`legal_move`'s canonical move carries real flags and a caller playing its own
input corrupts make/unmake; `is_legal` has **no production callers** (three test
assertions that never play the move) and all five search sites bind what
`legal_move` returns, so the property the leaf asked to preserve holds. It is
now documented and pinned by a test asserting raw flags are `QUIET` while
canonical flags differ, across a double push, a capture and a castle. Five
contract tests cover headroom, no reallocation across a 128-ply walk, clone
inheritance plus the clone walking without reallocating, exact hash/history
restoration on a 64-ply unwind, and canonicalization. **The clone test was
proven live**: reverting `Clone` to `Vec::with_capacity(self.history.len())`
made it fail, and only it failed. Verification: fresh no-feature build
reproduces **7,601,220 / EBF 2.474**; debug **280** / release **281** tests,
fmt and Clippy `--all-features --all-targets` clean. No public surface widened —
`reserve_history` is `pub(crate)` and the tests live in a `#[cfg(test)]` module
inside `board.rs` so the private field stays private. No games, no Elo. At this
closure point, 4.11b.14 is next. Evidence:
`analysis/history_contracts_2026-09-08.md`.

**RAR-M39 — 4.11b.14 larger board representation, COMPLETE 2026-09-08; research
disposition `NO_CHANGE`, no comparison registered and no implementation
opened.** Source `0d69c5f` on `dev`. The leaf's gate — open an implementation
only if the preceding profile still identifies substantial representation cost —
is **not met**: RAR-M36 puts no board region above **6.7%** (make/unmake 6.677%,
generation and legality 6.556%, SEE 5.239%, check queries 5.179%), and those are
the costs of doing the work rather than of the representation, which a different
layout trades rather than deletes. This session's two direct experiments concur:
4.11b.9 won +0.876%, 4.11b.11 lost 1-3% on its own benchmark. **Six type boards
plus colours, rejected on a measured trade**: `Board` is **264 bytes**, the
variant saves **48** (Rarog already keeps `occupancy[2]` and `all_occ`), both
sit far inside L1 with neither near a meaningful boundary, and against that
every `pieces(color, piece)` gains a load plus an AND across **208 call sites**,
**102 in `eval.rs`**, the profile's largest region at 29.49% exclusive.
**Per-ply state copying, quantitatively worse**: `UnmakeInfo` is **24 bytes** so
a 128-ply stack costs **3 KiB** and stays in L1, whereas copying whole board
state in the shape of Reckless's `InternalState` costs **128 x 264 = 33 KiB**
and leaves it — elevenfold, to avoid inverse work 4.11b.9 reduced to a single
fused mask. **Selectively checked generation, already amortized**: RAR-M34
measured 422,246 staged quiet generations served at zero extra `compute_pinned`,
and `board_gives_check_fast_calls` 25,540,503 against
`board_gives_check_full_calls` 49,385 is about **517:1**, so deferring legality
would trade a shared per-node cost for a per-move cost on a population pruning
discards unexamined. The only change made is a compile-time guard pinning the
two footprints the decision rests on, as upper bounds (`Board <= 264`,
`UnmakeInfo <= 24`) since padding may differ between supported targets and only
growth invalidates the argument; each message names this leaf so a breaking
field addition fails the build. **Proven live** by adding a `[u64; 4]` field,
which failed the build with the intended message. Behaviour unchanged at
**7,601,220 / EBF 2.474**; debug **280** / release **281**, fmt and Clippy
`--all-features --all-targets` clean. Retry trigger: a single board region above
**12%** AND a named mechanism that removes work rather than relocating it; donor
similarity is explicitly not one. No games, no Elo, no NNUE interaction — full
NNUE stacks remain Phase 5. At this closure point, 4.11b.15 is next. Evidence:
`analysis/representation_2026-09-08.md`.

**RAR-M40 — 4.11b.15 draw-state policy boundary, COMPLETE 2026-09-08; research
disposition `NO_CHANGE` on all four policies.** Source `95db376` on `dev`;
engine source untouched, so `bench 13` holds at **7,601,220 / EBF 2.474**. The
only change is `tests/draw_semantics.rs` (`df94b7d`). **What RAR-S18
establishes and what it does not**: arm A (null-clock + cross-null fence +
root-aware) **−7.21 ± 6.03**, arm B (same without root-aware) **−11.91 ± 7.67**;
both exclude zero so both bundles were harmful, but **neither isolates a single
part**, and although B is worse than A by 4.70 the intervals overlap heavily
(`[−13.24,−1.18]` vs `[−19.58,−4.24]`) so that ordering is unsupported. No
disposition leans on these as evidence about one component. **(1) Rule-50 clock
— KEEP, no retry trigger**: mate on the 100th-clock move outranks the draw, four
tests cover it. **(2) Null-move boundaries — KEEP**: the cross-null question
resolves structurally rather than on Elo, since `is_repetition` compares the
full hash including side to move so crossing a null yields only false NEGATIVES,
and `can_declare_draw` is reached only from `game_result` and the root tablebase
gate where history holds no nulls; the rejected fence guarded a search scoring
imprecision, not a legality defect. Retry needs a measured case where a
cross-null match changes a **root** best move. **(3) Pre-root versus in-search
repetition — KEEP**, and partial root-awareness already exists: `search.rs:2218`
guards `ply > 0`, so the root is never scored a draw in search and the rejected
change was a further one; retry needs a demonstrated **won game** lost to the
aggressive twofold, not node counts. **(4) Repetition versus TT and evaluation
keys — KEEP, audited clean, now pinned**: repetition uses the position hash
only; the TT key is `board.hash` with the clock applied on READ as a mate-score
correction in `tt::score_from_tt`; the eval cache stores a `halfmove_clock`
compared for equality (`eval.rs:1254`) as entry validity, not in a hash. Two new
tests pin this — one asserting positions differing only in the clock share a
hash, one recording that the scan bound is a **cost** choice because an
irreversible move permanently changes the hash. **Proving the identity test live
took three sabotage attempts**: via `check_consistency` (never called by these
tests) and via `from_fen` before the clock is parsed (still zero, a no-op),
before mixing it in after parsing failed the test and only it — a sabotage that
does not visibly change the thing under test proves nothing. Debug **282** /
release **283**, fmt and Clippy `--all-features --all-targets` clean. No games,
no Elo, no playing change proposed and no bundle rescued. At this closure point,
4.11b.16 is next. Evidence: `analysis/draw_policy_2026-09-08.md`.

**RAR-M41 — 4.11b.16 integrated board cluster qualification, COMPLETE
2026-09-08; QUALIFIED, speed claim banked, no Elo claimed.** Registration frozen
in `120b8d9` before the run; arms `1d720af` against head `1be34ac`, which is
exactly the fused relocation, history reservation and footprint assertions. Both
arms **behaviour-identical**: all six PGO binaries reproduce `bench 13` at
**7,601,220 / EBF 2.474**, so trees match and fixed-node NPS is a clean
throughput comparison. Section entry was deliberately not the baseline, because
the 4.11b.5 SEE repair changed the fingerprint and measuring across it would
confound throughput with a correctness fix owned by 4.11b.17. **Correctness
matrix, all passing**: debug **282** / release **283**; `see_contract` 8/8 with
all 41 external fixtures; `see_pins` 6/6; `draw_semantics` 8/8; randomized
`board_differential` and `fuzz_lite`; **72** tests under the PEXT slider backend
(`--cfg rarog_pext -C target-cpu=native`); fmt and Clippy `--all-features
--all-targets` clean; feature-off default. The one difference from section entry
is the SEE repair, and six independent fingerprint checks confirm every later
board change preserved it. The supported-target cross build was **not runnable
on this host** — `aarch64-pc-windows-msvc` is not installed and the vendored
fathom C code needs a cross `cl.exe` — and is stated as owed to the ARM64
compatibility host rather than claimed. **Instrument**: six PGO binaries via
`cargo xtask build --arch pext --native --pgo`, three per arm, rotating so no
build carries an arm; PGO build variance **verified, not assumed**, two builds
of identical source differing by hash. A **null pair** of same-revision builds
measured **+0.222%, 95% [-0.130%, +0.630%]** — containing zero, so the
instrument is unbiased, and that upper bound became the effective floor over the
0.5% practical floor derived from the ~2 Elo per 1% NPS constant. **Main
comparison**: 96 alternating pairs at 2,000,000 nodes, seed 4119, one discarded
warm-up per binary, 1T, Hash 16 MiB, rustc 1.97.1 PEXT `target-cpu=native`, no
affinity pinning; **+1.421%, 95% [+0.953%, +1.764%]**, median 3,606,933 ->
3,658,176 nps, **91/96** pairs faster, max host busy **9.11%** against a 15%
gate. Every registered condition passed and the frozen rule banks the claim.
**Calibration**: projected ~0.5% half-width from RAR-M33's measured 1.003%, got
**0.405%** — deriving the projection from a measured width rather than a
variance model corrected RAR-M33's miss; the estimate exceeds RAR-M33's non-PGO
+0.876% but is consistent with it since that interval contains 1.421%, with PGO
amplification and the two extra commits as unestablished candidate reasons; and
the registration's pre-stated risk that variance might exceed the effect did not
materialise. **Claimed**: +1.421% [+0.953%, +1.764%] whole-search NPS under
production pooled-PGO settings on a verified-idle host with identical behaviour.
**Not claimed**: any Elo — the constant would suggest ~+2.8 Elo but that is an
inference, not a measurement, and no games were played. At this closure point,
4.11b.17 is next and owns the playing gate. Evidence:
`analysis/cluster_qualification_2026-09-08.md` and
`tools/results/cluster-411b16/` (ignored, local).

### Phase-4 registration (RAR-M12, 2026-08-12)

Registered at 4.0, before any Phase-4 code moves. Caps are prospective, derived
from each cluster's PLAN §4 prior through RAR-M10's drift fit; a cap is a stop
point, never a target to run to, and none of it may be revised after games are
seen.

| Step | Cluster | Prior (nElo) | Bounds | Cap (games) |
|---|---|---:|---|---:|
| 4.5 | A — ordering, histories, LMR | 15–45 | `[3,10]` | 6,000 |
| 4.6 | B — static eval, TT, qsearch | 5–25 | `[3,10]` | 9,200 |
| 4.7 | C — main selectivity | 25–60 | `[3,10]` | 4,000 |
| 4.8 | D — extensions, depth authority | 5–25 | `[3,10]` | 9,200 |
| 4.9 | E — root search and clock | 5–20 | `[3,10]` | 9,200 |
| 4.13–4.16 | F–I — HCE structural | 15–50 each | `[3,10]` | 4,000 each |

That is roughly **55,000 STC games** of cluster gating, before ablations and
the 4.10 / 4.18 / 4.19 checkpoints. RAR-M11's completed schedule is the only
throughput anchor on this host — 320,000 games in about 79 hours, so ~4,050
games/hour — which puts the gating at roughly **14 hours**, call it 20 with
checkpoints. Treat that as an order of magnitude: it came from tune binaries
under an SPSA driver, not from final-PGO SPRT pairs.

Stop rules, all pre-registered:

1. Two fully implemented **search** clusters failing to produce an accepted
   gain stops the track and returns to 4.2–4.3. Not a third attempt.
2. Two coherent **HCE** clusters failing closes track H; go to 4.19 or Phase 5.
3. 4.10 is a real expected-value review with a close option, not a formality.
4. A cluster ends accepted or reverted. Borderline results are not carried.
5. **2.4.0** needs cumulative ≥ +40 Elo STC over 2.3.2 with the 95% lower bound
   above +25, plus positive LTC and 4T lower bounds. The programme *target* is
   ≥ +100 cumulative; a result there with a lower bound above +75 may justify a
   higher minor version.
6. HCE-changing A/Bs and every cross-engine cohort run with adjudication off.

## 3. Search and selectivity

### Search-accuracy decomposition

Recovered from branch `spsa_impr` before it was retired; the tooling these rows
produced (`tools/sprt.ps1 -Nodes`, `tools/pgn_depth_at_nodes.py`,
`tools/diag_search_quality.ps1`, `diag.rs` `cutoff_first_move`) already shipped
in 2.3.2, but the results themselves had never been entered here. They predate
RAR-O01/O02 and reach the same conclusion from a different direction, which is
why they are recorded rather than superseded. Baselines are Rarog 2.3.1 versus
Basilisk 1.9.1, so magnitudes are not comparable to the 2.3.2-era oracle rows.

| ID | Experiment and conditions | Result / disposition | Conditional lesson | Source |
|---|---|---|---|---|
| RAR-S56 | **Phase-4 step 4.7a — null-move entry contract, PREPARED AND HELD.** Candidate branch `p47a-nmp-entry` at `76e72bb`, baseline `dev` `090dedc`. Replaces Rarog's single relaxed entry test `nmp_eval >= beta − 12·depth − 35·improving` (which at depth 8 admits nodes ~131 cp below beta) with a hard `nmp_eval >= beta` primary gate, re-homing the old margin onto raw `static_eval` as a secondary floor so both tuned parameters stay live. Measured on the 4.2 suite, 50 positions, depth 8, against the identical baseline reading. | **No games. Deliberately not gated.** Mechanism moved as predicted: `nmp_attempt` 16,087 → 12,652 (**−21.4%**), `nmp_cut` 3,086 → 3,013 (−2.4%), conversion **19.2% → 23.8%**, nodes +3.7%, qnodes +3.3%. `bench 13` 6,519,711 → 6,692,786, EBF 2.449 → 2.452. fmt, all-feature clippy and 259/259 release tests clean. Held because conversion reached 23.8%, not the oracle's 83.3%, so the hard gate explains only part of the divergence and the plausible effect is **3–8 nElo**. Sizing that on RAR-M10: `[3,10]` drives a true 4–6 nElo candidate to H0, and `[0,5]` needs 20k–47k games. The registered 4.7 cap is 4,000 games because the **cluster** prior is 25–60 nElo. | A coherent mechanism change that measurably does what it claims is still not worth a gate on its own if its plausible effect sits in the harness's dead zone. Rule 2 and cluster-discipline rule 5 caught this in preparation, before the games. The forward action is to bundle with 4.7b (move-count volume, 13.35x, the largest divergence in the phase) into one coherent selectivity candidate: the 4.3 map already establishes that the leads compete for the same quiet population, so they are one contract rather than three patches. Do not gate 4.7a alone, and do not fold it silently into a later bundle without re-measuring — its −21.4%/−2.4% split is the attribution record. | **REVERTED 2026-08-18 — see RAR-S58.** It was gated only inside the 4.7a+4.7c bundle, the ablation showed 4.7c reproduces that result entirely, and a zero-game curvature probe ruled out its re-homed constants as the explanation. The attribution record above stands as the measurement; the code does not. `analysis/phase4_differential_47a_depth8.txt` is retained. | branch `p47a-nmp-entry`; `analysis/phase4_mechanism_map.md`; PLAN 4.7 |
| RAR-S57 | **Phase-4 cluster 4.7 — ACCEPTED.** The 4.7a+4.7c selectivity bundle: null-move entry moved to a hard `nmp_eval >= beta` with the old margin re-homed onto raw `static_eval`, plus a ProbCut move filter tying capture eligibility to the gap the capture must bridge (`probcut_beta - static_eval`, floored at 0), capping moves SEARCHED rather than candidates examined, and scaling that cap by `cut_node`. Candidate `dfa965e` on `p47c-probcut-filter`, baseline `dev` `aaa715a`. Final-PGO both arms, `3+0.03`, 1T, 64 MB, paired UHO_Lichess_4852_v1, registered `[3,10]` nElo, cap 16,000. **First gate run under RAR-M13 two-sided adjudication.** | **PASSED, H1 accepted at 2,838 games — a fifth of the cap.** Elo **+15.44 ± 8.06**, nElo **+24.50 ± 12.78**, LOS 99.99%, LLR 2.96 of ±2.94. W-D-L 796-1,372-670, 52.22%, draw ratio 44.47%, PairsRatio 1.26, Ptnml(0-2) [40, 309, 631, 363, 76]. **Zero time forfeits, crashes or illegal moves** in 2,838 games despite +5.16% bench nodes — the width-for-depth trade did not cost a single game on the clock. `bench 13` 6,519,711 → **6,856,329**, EBF 2.449 → **2.458**. Mechanism, on the 4.2 suite at depth 8: `probcut_attempt` −56.6% against `probcut_cut` −9.0%, conversion per move 32.6% → 68.4% against the oracle's 71.9%; 4.7a's own split was `nmp_attempt` −21.4% against `nmp_cut` −2.4%. | **The largest accepted search gain of the project, and it beat its own prior by 60%** — 24.50 nElo against a 5–15 band derived before the games. It also does what PLAN 4.7 predicted of a structural rework: RAR-S54's blind uniform 15% de-selectivity scalar measured +4.06 ± 3.71, and the two coherent contracts are ~3.8x that. **No subcomponent is credited.** 4.7a and 4.7c were gated as one cluster under rule 3 and neither has a standalone result; splitting the +15.44 between them requires the ablation rule 7 demands, and until that runs the honest attribution is "the bundle". ⚠ **Process deviation, recorded rather than hidden:** the bounds `[3,10]`, cap 16,000 and the 5–15 prior were fixed in writing before the run and used verbatim, but the `EXPERIMENTS.md` row itself was written after the result. Rule 2 wants the row filed first. Nothing was changed after seeing games; the filing lagged, not the design. | `tools/results/sprt_47bundle_vs_Head_20260818_153904.{pgn,log}`; `analysis/phase4_differential_47c_depth8.txt`; RAR-S55 v3; RAR-S56; PLAN 4.7 |

**Amendment to RAR-S57, 2026-08-18.** The bundle passed, but it is **not what shipped.** RAR-S58's ablation showed 4.7c reproduces the whole +24.50 nElo on its own (+24.90 ± 16.01, H1 in 1,810 games), leaving 4.7a at −0.40 nElo marginal, so 4.7a was reverted and the accepted head is the `47c-only` arm — byte-identical in engine source to the binary that passed that SPRT, fingerprint **6,922,439 / EBF 2.451**. Two consequences worth stating plainly. First, the shipped head was gated at `[0,10]`, a weaker bar than the bundle's `[3,10]`; its lower bound is +8.89 nElo and the bundle cleared `[3,10]` at the same point estimate, so the risk is small, but an ablation arm was used as a shipping decision and that is the exception, not the pattern. Second, RAR-S57's headline Elo of +15.44 ± 8.06 describes a configuration that no longer exists; quote **+15.56 ± 10.02** for the shipped one.

| RAR-S58 | **Phase-4 cluster 4.7 ablation — COMPLETE.** Rule 7 requires ablating a surprising integrated result before any subcomponent is credited, and RAR-S57 beat its 5–15 nElo prior by 60%. Two SPRTs, each a single mechanism against the SAME pre-bundle baseline `aaa715a` (`rarog-47base`, bench 6,519,711): **arm A** `rarog-47a-only` = null-move entry alone (`b6b0d7d`, bench 6,692,786); **arm C** `rarog-47c-only` = ProbCut move filter alone (`6407061` on `p47-ablate-c`, the bundle with 4.7a reverted, bench 6,922,439). All four binaries final-PGO with clean manifests, `git_dirty = False`. `3+0.03`, 1T, 64 MB, paired UHO_Lichess_4852_v1, RAR-M13 two-sided adjudication. **Registered bounds `[0,10]` nElo, cap 16,000 per arm, fixed before any games.** | **Arm C carries the whole result; arm A has no measured marginal contribution.** **Arm C** `47c-only` **PASSED**, H1 at 1,810 games: Elo **+15.56 ± 10.02**, nElo **+24.90 ± 16.01**, LOS 99.89%, LLR 2.96, W-D-L 502-887-421, draw 44.75%, PairsRatio 1.31, Ptnml [29, 187, 405, 242, 42]. That is statistically identical to the FULL bundle's +24.50 ± 12.78, so the ProbCut move filter alone reproduces RAR-S57 entirely and 4.7a's marginal contribution in company is **−0.40 nElo**, i.e. zero. **Arm A** `47a-only` **UNRESOLVED**, stopped by the operator at 952 games: Elo +2.19 ± 14.17, nElo +3.41 ± 22.07, LOS 61.91%, LLR −0.12 (−4.2% toward H0). That interval spans −18.7 to +25.5 and on its own proves nothing about 4.7a — the decisive evidence is C ≈ bundle, not A's own arm. **Follow-up curvature probe, zero games:** sweeping 4.7a's two re-homed constants on the accepted head leaves `nmp_cut/nmp_attempt` flat at **27.6–28.8%** across `NullMoveDepthCoeff` 4→30 and `NullMoveImprovingBonus` 0→80. | **Bounds were `[0,10]`, not `[3,10]`, deliberately.** The ablation's question is "is either half inert?", not "does either half clear a shipping bar" — the bundle already shipped. At `[3,10]` a true 4–6 nElo half is driven to H0 and reads as worthless, which is exactly the misreading RAR-S50 documented when SingReject looked inert alone and had a real marginal effect in company. RAR-M10 sizing at `[0,10]`: a true 22 nElo resolves in ~2,100 games, 12 in ~5,100, 0 reaches H0 in ~7,100; a true 1–4 nElo will NOT resolve inside the cap, and that non-resolution is the answer, not a reason to extend. ⚠ **The two results will not sum to RAR-S57's +24.50 nElo and must not be presented as a division of it.** RAR-S50 measured a 20.8-point swing between the sum of individuals and the set effect, and these two are already visibly sub-additive in tree terms: +2.65% and +6.18% bench nodes alone against **+5.16% together**, so the bundle is SMALLER than arm C by itself. Standalone value is what this measures; marginal-value-in-company would be a separate pair against the bundle. **The sub-additivity prediction held**: +24.50 for a+c against +24.90 for c alone is not a division of credit, it is c doing all of it. **4.7a is not rescuable by tuning.** Texel is the wrong domain entirely — it fits eval weights and this is a search contract. SPSA is ruled out by the probe above: 4.7a re-homed `nm_depth_coeff` and `nm_improving_bonus` from `nmp_eval` onto `static_eval`, which is exactly the "displaced continuous optimum" case PLAN rule 4 reserves SPSA for, and the surface is flat, so there is no optimum to recover. One live detail for a later owner: `NullMoveImprovingBonus` swings the tree 25% (6,225,304 nodes at 0 against 7,797,922 at 80) while conversion does not move at all, so the re-homed floor is a pure VOLUME knob, not a quality one. That makes it an activated coordinate for 4.10's consolidation fit, not a reason to keep 4.7a now. | `tools/results/sprt_47c-only_vs_Head_20260818_163403.{pgn,log}`; `tools/test_engines/rarog-47{base,bundle,a-only,c-only}-pext-pgo.exe`; RAR-S57; RAR-S50; RAR-M10. **Ablation-arm provenance, recorded so the branches could be deleted:** arm A's source is `b6b0d7d`, which is in `dev`'s history (added by the bundle, reverted by `21e5276`). Arm C's source `6407061` is NOT in `dev`'s history and is now dangling — but it needs no archive, because its engine source is **byte-identical to the accepted head** (`21e5276` onward): `git diff <head> 6407061 -- src/` was verified at 0 lines before the branch was deleted, and `rarog-47c-only-pext-pgo.exe`'s manifest fingerprint 6,922,439 matches the head's. To rebuild arm C, build the accepted head; to rebuild arm A, check out `b6b0d7d`. |
| RAR-S59 | **Phase-4 step 4.5.3 — continuation-attribution asymmetry, measured with zero games.** `update_cutoff_tables` gives a tried-and-failed quiet a malus in main, low-ply and pawn history but never in continuation history, so continuation learns only from the move that worked while the other three learn from both. Built the malus behind a default-off switch, verified the off position reproduces `bench 13` 7,467,143 exactly, then compared ordering counters on the 40-position bench corpus at stride 1. | **REJECTED on measurement.** Ordering is FLAT: first-move cutoff rate 88.04% → **88.09%**, and the rank8+ share gets slightly *worse* (0.750% → 0.793%). Meanwhile nodes fall 7.5% (7,467,143 → 6,907,848) and total cutoffs fall **9.6%** — cutoffs drop FASTER than nodes, which is the opposite of what an ordering improvement does. | **The symmetry fix is a selectivity increase in disguise, and that is why the intuition was wrong.** Continuation history feeds `quiet_hist`, which drives two of LMP's four disjuncts and the LMR reduction, so pushing it broadly negative simply prunes more quiets. Rarog's one four-times-replicated diagnosis is that it already prunes too much — RAR-S53 (2.5 plies deeper at equal nodes and still losing), RAR-S54 (+4.06 for a blind de-selectivity shift), RAR-S55, and 4.7 itself paying **+15.56 Elo for pruning LESS**. Adopting this would have moved the engine the one direction every reading forbids, under the cover of fixing an asymmetry that looks like an omission. ⚠ Generalisation for the rest of Cluster A: a history-table change is never *only* an ordering change in this engine, because the same tables gate pruning. Measure cutoffs-per-node alongside the cutoff RATE — the rate alone would have shown nothing here. The switch was built, measured and **removed** rather than left dormant. | `src/search.rs` `update_cutoff_tables`; `tools/diag/bench_counters.py`; RAR-S53; RAR-S54; RAR-S57; PLAN 4.5.3 |
| RAR-S60 | **Phase-4 steps 4.5.3/4.5.4 — the six deferred per-ply fields, disposed with zero games.** 4.5.1 deliberately landed no speculative state and handed six fields to the sub-items owning their consumers. Each was built behind a default-off switch where it had one, verified to reproduce `bench 13` 7,467,143 in the off position, and measured on the 40-position bench corpus at stride 1 using RAR-S59's method — cutoff RATE **and** cutoffs-per-node, because in this engine a history/reduction change is never only an ordering change. | **Two adopted, four rejected.** ADOPTED — *continuation key*: derived in `push_move`, and it exposed the ProbCut piece desync. *Prior reduction*: reduce 512/1024 ply less when the parent move was itself reduced. Cutoffs per node rise **faster** than nodes (+4.5% against +1.6%) and first-move cutoff improves **88.04% → 88.18%** — the mirror image of RAR-S59's rejected candidate, and the signature of a real gain rather than a disguised selectivity change. At 1024 the effect is larger still (+7.5% cutoffs/node, 88.27%) but costs +6.1% nodes; 512 is the conservative categorical default. `bench 13` 7,467,143 → **7,587,235**. REJECTED — *cutoff count*: **completely inert**, byte-identical output at 512 and 1024, because a beta cutoff BREAKS Rarog's move loop, so a per-visit count is 0 or 1 by construction. *Statistical score* and *TT/PV evidence*: already exist as `quiet_hist` and `tt_pv`, threaded into the reduction; both are node-LOCAL, read at the ply that computes them, so per-ply storage would add lifetime nothing consumes. *Previous-PV following*: structurally redundant — the previous iteration's PV move is stored in the TT and already emitted first by `Stage::TtMove`. | **The deferred-field list was a hypothesis, and measuring it was worth more than implementing it.** Four of six were wrong for this engine: one inert by a control-flow difference, two already present under other names, one duplicated by existing ordering. Had 4.5.1 landed all six as the plan listed them, the cluster would carry four unused fields, one of them a mechanism that cannot fire. ⚠ **Correction, same day:** the cutoff-count rejection above is of the IMPLEMENTATION, not the mechanism, and the original wording overstated it. Stockfish's `cutoffCnt` semantics are knowable, not guesswork: it resets `(ss+2)->cutoffCnt = 0` on node entry — zeroing the GRANDCHILD's counter, not its own — so a ply slot accumulates cutoffs across every sibling visit between resets, which is how it exceeds 3. It is a recency signal for "this ply has been cutting a lot lately", not a per-node count. The per-visit reset used here is what made it 0-or-1 and inert. The mechanism remains unpursued for a different and better reason: its consumer is `if ((ss-1)->cutoffCnt > 3) r++`, a selectivity INCREASE, which is the direction RAR-S53/S54/S55 and 4.7's +15.56 all contraindicate. Handed to 4.10, which owns second-pass selectivity, with the counter-point recorded: it is a CONDITIONAL increase aimed at plies that demonstrably cut often, which is not the same as a blanket one. Switches built, measured and **removed** — 4.5.5 inherits no dormant knobs. | `src/search.rs` `lmr_reduction_units`, `NodeContext`; `tools/diag/bench_counters.py`; RAR-S59; PLAN 4.5.3/4.5.4 |
| RAR-S61 | **Phase-4 cluster 4.5 (A) — REGISTERED, NOT YET RUN.** Candidate: `dev` at `c399435` (4.5.1 per-ply context, 4.5.2 named picker stages, 4.5.3 continuation key plus the ProbCut piece-desync fix, 4.5.4 prior-reduction authority at 512/1024 ply). Baseline: the pre-Cluster-A head `36dad5f`, fingerprint **6,922,439 / EBF 2.451** (confirmed by the baked manifest — an earlier attempt baked `aaa715a`, which predates the 4.7 merge and benched 6,519,711; the manifest's recorded fingerprint is what caught it). Candidate **7,587,235 / EBF 2.477**, +9.6% nodes. Final-PGO both arms, `3+0.03`, 1T, 64 MB, paired UHO, RAR-M13 two-sided adjudication. **Registered bounds `[3,10]` nElo, cap 16,000, prior 5–20, all fixed before any games.** | **UNRESOLVED at the 16,000-game cap — NOT promoted.** Elo **+4.50 ± 3.50**, nElo **+6.92 ± 5.38** (95% CI [1.54, 12.30]), LOS 99.41%, W-D-L 4,209-7,789-4,002, 50.65%, draw 41.83%, PairsRatio 1.07, Ptnml(0-2) [324, 1926, 3346, 2027, 377]. **LLR 0.39 of ±2.94 after the full budget.** One timeout per side, symmetric, no crashes or illegal moves. **RAR-M10 predicted this to three decimals:** the bracket midpoint is 6.5 nElo and the candidate landed at 6.92, 0.42 away, so drift ≈ 8.3e-6 × 7 × 0.42 × 16,000 = **0.390** against an observed **0.390**. The effect is almost certainly real — LOS 99.41%, CI excluding zero — and simultaneously unable to resolve, because it sits within half an nElo of the one value `[3,10]` cannot decide. | **Prior re-derived from 15–45 down to 5–20, because most of Cluster A turned out to be unnecessary rather than valuable.** PLAN sized 15–45 for a full ordering/history/LMR rework; RAR-S52/S55 had already refuted the ordering premise, and RAR-S60 then rejected four of the six planned per-ply fields. Only two strength-bearing changes shipped: the ProbCut piece fix (correctness repair of unknown sign — continuation history was trained on a mismatched piece/square pair) and prior-reduction authority (de-selectivity, the direction that paid +15.56 at 4.7). ⚠ **A fit was considered and deferred, the first time the condition was actually met.** The curvature probe leaves `cut/node` monotone but first-move cutoff peaks at `LmrPriorReductionAdj=768` (88.41 pct) between 512 (88.18) and 1024 (88.27) — a real interior optimum, where 4.7's surface was flat. 512 is kept anyway: 768 costs **+17 pct nodes** for +0.23 points of a proxy RAR-S59 proved can mislead, the default was chosen ON this sweep rather than inherited from an older structure, and PLAN 4.10 owns consolidation tuning across accepted clusters, so a cluster-local SPSA would duplicate it. **The curvature evidence is handed to 4.10.** | `tools/test_engines/rarog-45{base,cluster}-pext-pgo.exe`; RAR-S57; RAR-S59; RAR-S60; RAR-M10; PLAN 4.5.5, 4.10 |
| RAR-S62 | **Phase-4 cluster 4.5 ablation — REGISTERED, NOT YET RUN.** Isolates the marginal contribution of the 4.5.3 ProbCut piece-desync correctness fix INSIDE Cluster A, measured directly rather than inferred by subtracting two gates. Arm A `rarog-45cluster` `774000b`, bench 7,587,235. Arm B `rarog-45nofix` `46fa4c4`, bench 7,560,177 — identical except the ProbCut site writes `mv` without its piece, reintroducing the desync. Final-PGO both, `3+0.03`, 1T, 64 MB, paired UHO, RAR-M13 adjudication. **Registered bounds `[-5,5]` nElo, cap 12,000, fixed before any games.** | **H0 ACCEPTED at 4,436 games — the correctness fix COSTS strength.** Elo **−5.09 ± 6.47**, nElo **−8.04 ± 10.22** (95% CI [−18.26, +2.18]), LOS 6.15%, W-D-L 1,090-2,191-1,155, PairsRatio 0.92, LLR −2.95 of ±2.94. The bracket resolved in 4,436 games against RAR-M10's ~7,100 estimate for ±5, so the effect is at the larger end of that. Combined with RAR-S61 (+4.50 ± 3.50 against Head), the **inferred** value of Cluster A without the fix is **+9.59 ± 7.36 Elo** — an inference from two comparisons, not a measurement, and RAR-S50 plus this cluster's own node arithmetic both warn that these do not add. | **Bracket chosen from RAR-M10 rather than by habit, after `[3,10]` cost RAR-S61 its whole budget.** `[-5,5]` has midpoint 0, so it resolves in ~7,100 games if the fix is worth ±5 nElo and ~11,800 at ±3 — and if the truth is ~0 it never resolves, runs to the cap, and returns an estimate. That is the correct behaviour here: for a correctness repair costing **0.36% nodes in company** the decision only changes if the fix is clearly HARMFUL, so the test is powered to detect harm, not to prove benefit. `[0,5]` was rejected — it needs 141,687 games for a true +3. ⚠ **Pre-declared follow-up, recorded now so it cannot be invented after seeing the result:** if the fix measures negative, the first hypothesis is NOT that correctness costs Elo but that the constants consuming continuation history — `lmr_hist_div`, `quiet_hist_prune_coeff`, the LMP history thresholds — were fitted against the CORRUPTED signal and are now mis-set. That is the clearest displaced-continuous-optimum case this project has had (unlike 4.7a, whose probe was flat), and it is the condition PLAN rule 4 reserves targeted SPSA for. Removing the repair would be the last resort, not the first. Precedent for retention regardless: RAR-S51's mate clamp. | `tools/test_engines/rarog-45{cluster,nofix}-pext-pgo.exe`; RAR-S61; RAR-S51; RAR-M10. **Arm B recipe** (branch deleted; it held a deliberate bug): in the ProbCut block of `negamax`, replace the `board.moving_piece(mv)` + `push_move(ply, mv, piece)` pair with a bare `self.stack[ply].mv = mv;`, leaving piece and `cont_key` stale. Rebuild and confirm `bench 13` = **7,560,177 / EBF 2.482**. |
| RAR-S63 | **Phase-4 step 4.5.3 — ProbCut speculative-move contract, third variant. REGISTERED, NOT YET RUN.** RAR-S62 showed the correctness fix costs 5 Elo but could not say why: the desync may have carried signal, or it may have been injecting arbitrary noise into continuation history that happened to regularise it. This variant separates them by recording NOTHING at the ProbCut ply, so the child sees a null previous move and neither reads nor trains continuation history, counter-moves or continuation correction through a move the search does not yet believe in. Arm A `rarog-45nullstack` `7ea0620`, bench 7,827,899. Arm B `rarog-45cluster` `774000b`, bench 7,587,235 — the paired-write correctness fix. Final-PGO both, `3+0.03`, 1T, 64 MB, paired UHO, RAR-M13 adjudication. **Registered bounds `[-5,5]` nElo, cap 12,000, fixed before any games.** | **Dead tie, unresolved at the cap.** Elo **+0.41 ± 3.97**, nElo **+0.63 ± 6.22** (95% CI [−3.56, +4.38]), LOS 57.92%, W-D-L 3,065-5,884-3,051, PairsRatio 1.01, LLR 0.63 of ±2.94 over 12,000 games. Recording nothing at the ProbCut ply is indistinguishable from recording the correct pair — and both sit ~5 Elo behind the desync (RAR-S62). ⚠ **My registered discrimination was wrong, and the row it was written into says so.** I predicted that under the regularisation hypothesis "not writing at all captures the same benefit". It does not: injecting noise into continuation history and declining to touch it are different operations, and only the first adds entropy. Both hypotheses in fact predict null ≈ paired, so this arm could never have separated them. The test was still worth running — it establishes that the ONLY thing distinguishing the desync is the wrong continuation ROW, because `mv` itself was always written correctly and the counter-move table therefore behaves identically in the desync and paired arms. | **The two hypotheses make opposite predictions, which is what makes this worth 45 minutes.** If the desync carried usable signal, writing nothing loses it too and this arm measures ≈0 or negative against the fix — leaving targeted SPSA over `lmr_hist_div`, `quiet_hist_prune_coeff` and the LMP history thresholds as the only recovery route. If the desync was accidental regularisation, not writing at all captures the same benefit with correct code and this arm measures clearly positive — and no SPSA is owed at all. Same `[-5,5]` bracket as RAR-S62 for direct comparability, and because it resolves fast in either direction while returning an estimate if the truth is ~0. ⚠ Note the arm is compared against the FIX, not against Head: the question is which contract to ship, not whether Cluster A is worth shipping, which RAR-S61 already priced at +4.50 ± 3.50. | `tools/test_engines/rarog-45{nullstack,cluster}-pext-pgo.exe`; **Arm A recipe** (branch deleted): in the ProbCut block of `negamax`, replace the `board.moving_piece(mv)` + `push_move(ply, mv, piece)` pair with `self.clear_move(ply);`. Rebuild and confirm `bench 13` = **7,827,899 / EBF 2.484**. RAR-S62; RAR-S61 |
| RAR-S64 | **Phase-4 cluster 4.5 (A) — RE-MEASUREMENT after the stale-reduction fix. REGISTERED, NOT YET RUN.** Supersedes RAR-S61, whose candidate contained a live defect: `stack[ply].reduction` was written on the LMR branch only, so `lmr_prior_reduction_adj` read a value left by a previous sibling or an unrelated subtree. Fixed structurally in `push_move`. Candidate `rarog-45fixed` `a1642ae`, bench **7,436,275 / EBF 2.467**. Baseline `rarog-45base` `36dad5f`, bench **6,922,439 / EBF 2.451** — the same pre-Cluster-A head RAR-S61 used, so the two gates are directly comparable. ProbCut keeps the PAIRED contract by maintainer preference: RAR-S63 measured null against paired at +0.41 ± 3.97, a dead tie, so the choice is free and the incumbent stays. Final-PGO both, `3+0.03`, 1T, 64 MB, paired UHO, RAR-M13 adjudication. **Registered bounds `[0,10]` nElo, cap 20,000, fixed before any games.** | **H0 ACCEPTED at 8,088 games — Cluster A is worth NOTHING once the defect is removed.** Elo **+0.39 ± 4.89**, nElo **+0.60 ± 7.57**, LOS 56.16%, W-D-L 2,064-3,969-2,055, PairsRatio 1.01, LLR −2.95 of ±2.94. Against RAR-S61's +4.50 ± 3.50 on the identical baseline, **the entire measured gain of Cluster A was the stale-reduction bug.** `lmr_prior_reduction_adj` reading a value left by an unrelated subtree was worth ~4.5 Elo; reading the correct parent reduction is worth zero. The bracket change worked as designed — `[0,10]` resolved in 8,088 games where `[3,10]` had burned 16,000 without moving. | **This is the second time in one cluster that reading the WRONG value beat reading the right one, and the third such observation in the project.** RAR-S62: the ProbCut desync (arbitrary continuation row) beat correct indexing by ~5 Elo. RAR-S64: stale prior-reduction (a quasi-random subset of nodes reduced less) beat correct prior-reduction by ~4 Elo. RAR-S54 already measured a blind, untuned, uniform 15% de-selectivity shift at **+4.06 ± 3.71 over 14,196 games**. Three independent arrivals at the same place: **Rarog's selectivity surface is over-confident, and scattered perturbation of it gains Elo where principled mechanisms do not.** ⚠ The forward reading is NOT "ship bugs". It is that deliberate randomisation of the reduction surface is a candidate mechanism in its own right — Rarog already runs LMR jitter for SMP diversification, so the machinery exists and has a precedent. That belongs to 4.10, with its own registration. What this row settles is narrower and firm: `lmr_prior_reduction_adj` as a principled mechanism is dead, and Cluster A carries no strength. | `tools/results/sprt_45fixed_vs_Head_20260819_102841.{pgn,log}`; RAR-S61; RAR-S62; RAR-S54; `analysis/code_audit_2026_08_19.md` |
| RAR-S65 | **Audit finding 3 — bound how far a killer can travel. REGISTERED, NOT YET RUN.** `killers[ply]` is cleared once per search, so within a search a ply's killers persist across every sibling subtree reaching that depth and a node can inherit killers from a positionally unrelated subtree. The candidate clears the GRANDCHILD's slot on node entry, bounding the distance. Arm A `rarog-45killer` `1e2a30d`, bench **6,556,136 / EBF 2.443**. Arm B `rarog-45head` `1155ec3`, bench **7,467,143 / EBF 2.477** — the accepted head after Cluster A closed. Final-PGO both, `3+0.03`, 1T, 64 MB, paired UHO, RAR-M13 adjudication. **Registered bounds `[0,10]` nElo, cap 20,000, fixed before any games.** | **STOPPED at 12,522 games, not resolved, NOT promoted.** Elo **+2.80 ± 3.90**, nElo **+4.38 ± 6.09**, LLR −0.65 of ±2.94, drifting toward H0. Stopped deliberately by the operator once the projection was clear; recorded as a STOP, not a result, following RAR-S54's precedent. RAR-M10 predicted the drift throughout (−0.66 against −0.65 observed at 10,680). ⚠ **The bracket was wrong, and that is the finding.** `[0,10]` does not merely resolve slowly against a true +4.4 — it drives it to **H0 in ~35k games**. This gate was registered at bounds configured to reject what it was measuring, as were RAR-S61 and RAR-S64. Fishtest uses `[0,2]` STC / `[0,1]` LTC, narrow and anchored at zero; `[0,3]` accepts a true +4 in ~47k games and is RAR-M10's fitted regime. **The +2.80 point estimate is therefore not evidence the mechanism works, and it is not evidence it fails either — the instrument was pointed wrongly.** Rejected on the registered rule; re-testable at `[0,3]` if it is ever worth an overnight run. | **The bench signal is the strongest this cluster produced, and that is explicitly not why it is being gated.** First-move cutoff moves **88.04% → 88.70%**, +0.66 points, where RAR-S59's rejected candidate moved it 0.05 and RAR-S64's adopted-then-dead mechanism moved it 0.14. ⚠ Two cautions carried from this cluster's failures. `cut/node` is **flat** (0.0853 → 0.0856), the exact signature RAR-S59 used to unmask a disguised selectivity increase — so the tree shrinking 12.2% is not self-evidently an ordering gain. And it makes the engine **more** selective, the one direction RAR-S53/S54/S55 and 4.7 all contradict. RAR-S64 settled how much bench proxies are worth here: a mechanism adopted on a clean proxy measured exactly zero in games. `[0,10]` chosen from RAR-M10 rather than habit — it resolved RAR-S64 in 8,088 games where `[3,10]` had spent 16,000 without moving. | `tools/test_engines/rarog-45{head,killer}-pext-pgo.exe`; **Arm A recipe** (branch deleted): set `killer_clear_grandchild = 1` in `params.rs`; rebuild and confirm `bench 13` = **6,556,136 / EBF 2.443**. `analysis/code_audit_2026_08_19.md`; RAR-S59; RAR-S64; RAR-M10 |
| RAR-S66 | **Audit finding 2 — `improving` loses its fallback after a check. REGISTERED, NOT YET RUN.** When the node two plies back was in check its `static_eval` is `VALUE_NONE`, so `improving` is forced false regardless of the real trend. There is no walk-back to `ply - 4`. The candidate adds one. Arm A `rarog-46improving` `b517991`, bench **6,969,327 / EBF 2.459**. Arm B `rarog-46base` `e2fd4e0`, bench **7,467,143 / EBF 2.477** — the accepted head. Final-PGO both, `3+0.03`, 1T, 64 MB, paired UHO, RAR-M13 adjudication. **Registered bounds `[0,3]` nElo, cap 60,000, fixed before any games.** | **STOPPED at 13,882 games, not resolved, NOT promoted.** Elo **+3.48 ± 3.72**, nElo **+5.41 ± 5.78**, LLR **+1.35** of ±2.94, W-D-L 3,643-6,735-3,504. **The LLR peaked at +2.44 around 8,480 games and receded for the next 5,400**, tracking the estimate down from +8.40 → +4.89 → +3.48 Elo. Four time forfeits occurred during a Windows Update window — **2 per side, exactly balanced**, so no score impact, but the window is recorded as a contamination caveat because external load on a timed match is what `-use-affinity` exists to prevent. Stopped deliberately once the projection was clear; recorded as a STOP, per RAR-S54. | **The bracket was right this time and the candidate still did not clear it, which is the useful part.** `[0,3]` would have accepted a true +4 nElo in ~47k games; the estimate fell through that range instead of holding. ⚠ **Not accepted despite a CI of [0.35, 12.03] that barely excludes zero.** That shape is exactly what RAR-S61 had — +4.50 ± 3.50 at LOS 99.41%, every point of it a stale-read bug. Accepting on a point estimate makes the gate decorative. There is no RAR-S51 retention argument either: this is not a correctness fix, it unlocks nothing, and `improving = false` after a check is a conservative default rather than a defect — there genuinely is no comparable static eval at `ply - 2`. **A cheap lesson about candidate selection:** the audit produced four findings, two were real defects and were fixed, and both of the DESIGN-DIFFERENCE items (killers, `improving` fallback) went to a gate and neither cleared it. Design differences from Stockfish are not latent Elo. | `tools/results/sprt_46improving_vs_Head_20260820_114036.{pgn,log}`; **Arm A recipe** (branch deleted): set `improving_ply4_fallback = 1` in `params.rs`; rebuild and confirm `bench 13` = **6,969,327 / EBF 2.459**. `analysis/code_audit_2026_08_19.md`; RAR-S61; RAR-S65; RAR-M10 | **First gate registered at a bracket that can actually accept what it is measuring.** RAR-S61, S64 and S65 all used `[0,10]` or `[3,10]`, which drive a true +4 nElo to H0 in ~35k and ~20k games respectively — configured to reject their own candidates. Fishtest's shape is `[0,2]` STC / `[0,1]` LTC, narrow and anchored at zero. `[0,3]` is chosen over `[0,2]` because RAR-M10 was FITTED on `[0,3]` gates, so it is in-regime rather than an extrapolation; it accepts a true +4 in ~47k games and a true +5 in ~34k. ⚠ **The 60,000 cap is a budget decision, not a statistical one, and it cannot decide everything.** It accepts a true ≥4, and reaches H0 for a true ≤0 at ~79k — so a genuine dud will hit the cap unresolved and must be reverted anyway. A true +2 needs 236k and is out of reach at any budget this project has. The mechanism argument stands independently of the bench proxy, which is why this one earns an overnight run: 9.7% of nodes lose `improving` outright, and `improving` is worth a full ply of LMR reduction and feeds the LMP margin, so the bias is toward MORE selectivity — the direction RAR-S53/S54/S55 and 4.7 all contradict. | `tools/test_engines/rarog-46{base,improving}-pext-pgo.exe`; branch `p45-improving-ply4`; `analysis/code_audit_2026_08_19.md`; RAR-S65; RAR-M10 |
| RAR-S67 | **1T LMR-reduction jitter — REGISTERED, NOT YET RUN.** The 4.10 obligations document's strongest lead. Rarog already runs a per-thread xorshift jitter of ±64/1024 ply on the LMR reduction for SMP diversification, disabled at 1T only to keep bench deterministic. This enables it at 1T at **±128/1024 (⅛ ply)**. Arm A `rarog-47jitter` `e7965b9`, bench **6,867,326 / EBF 2.457**. Arm B `rarog-47base` `c58d82d`, bench **7,467,143 / EBF 2.477** — the accepted head. `next_jitter` now takes a magnitude and at 64 is exactly the previous expression, so the SMP path is unchanged by construction. Final-PGO both, `3+0.03`, 1T, 64 MB, paired UHO, RAR-M13 adjudication. **Registered bounds `[0,3]` nElo, cap 60,000, fixed before any games.** | **STOPPED early, tracking to H0 — NOT promoted.** See the stop note recorded below the table. | **This is the only candidate in the phase whose prior rests on game results rather than counters.** RAR-S54: a blind, untuned, uniform 15% de-selectivity shift measured **+4.06 ± 3.71 over 14,196 games**. RAR-S62: a ProbCut desync reading an ARBITRARY continuation row beat correct indexing by ~5 Elo. RAR-S64: a stale prior-reduction firing on a quasi-random subset beat the correct one by ~4.5. Twice, a bug that scattered noise into the selectivity surface beat its own correction — which is why the hypothesis is perturbation, not any particular mechanism. ⚠ **The bench cannot screen this, and that is structural, not a tooling gap.** A deterministic bench samples ONE realisation of a randomised reduction; the hypothesised value is diversification across games. The sweep behaves accordingly — nodes bounce 6.3M–7.5M with no trend, `fm` wanders 87.99–88.52, and `cut/node` slightly FALLS. So the magnitude is a judgement and is recorded as one: 128 is double the SMP value, which was itself chosen small enough not to distort the mean reduction. ⚠ **Gate before tuning.** RAR-S13 ran `cutoffCnt` plus a full LMR-family SPSA and lost **7.78 ± 8.00** because the tuner selected a sibling-local optimum that won its own self-play and then lost to the accepted head. Magnitude is the obvious SPSA coordinate, and it stays untouched until the mechanism itself clears a gate. Determinism is preserved: the 1T PRNG re-seeds per search from a fixed seed, so two bench runs agree exactly. | `tools/test_engines/rarog-47{base,jitter}-pext-pgo.exe`; branch `p410-jitter-1t`; `analysis/phase4_10_obligations.md` B1; RAR-S54; RAR-S62; RAR-S64; RAR-S13 |
| RAR-S68 | **Unconditional LMR-reduction relief — REGISTERED, NOT YET RUN.** Subtracts a fixed **336/1024 ply (15% of the 2.19-ply mean reduction)** from every LMR reduction. The DIRECTIONAL form of what RAR-S54 and RAR-S64 measured, replacing the symmetric form RAR-S67 disproved. Arm A `rarog-47relief` `5dbeb52`, bench **6,539,063 / EBF 2.449**. Arm B `rarog-47base2` `23b21b8`, bench **7,467,143 / EBF 2.477** — the accepted head. Final-PGO both, `3+0.03`, 1T, 64 MB, paired UHO, RAR-M13 adjudication. **Registered bounds `[0,3]` nElo, cap 60,000, fixed before any games.** | *Pending.* | **The magnitude comes from the evidence, not from the sweep, and the distinction matters after RAR-S64.** Mean reduction is 2.19 ply = 2,243/1024; RAR-S54 shifted its twelve selectivity constants by **15%**, which is 336. That the sweep's best `cut/node` (0.0890 against 0.0853 at zero) also lands at 336 is corroboration only — RAR-S64 adopted a value picked off a clean bench sweep and it measured exactly zero in games. ⚠ **Directly tests whether RAR-S54's headroom survived 4.7.** That +4.06 ± 3.71 over 14,196 games was measured against the 2.3.1 head; 4.7 has since banked +15.56 Elo of structural de-selectivity, so some or all of it may already be spent. A null here is therefore informative rather than merely disappointing: it would say the blind-shift headroom is gone. ⚠ **Not a bake candidate for the scalar itself.** PLAN is explicit that RAR-S54 licenses a structural rework and does not license shipping a uniform scalar. If this gates positive the right response is to find where the reduction is systematically too aggressive, as 4.7 did for ProbCut — not to ship 336 and call it done. | `tools/test_engines/rarog-47{base2,relief}-pext-pgo.exe`; branch `p410-lmr-relief`; RAR-S54; RAR-S64; RAR-S67; RAR-S57 |
| RAR-S69 | **RAR-S54's ten pruning margins, ×1.15 on the current head — REGISTERED, NOT YET RUN.** The last candidate in this line. Shifts only the **ten pruning-margin** constants RAR-S54 moved — futility (×2), razoring, LMP (×2), quiet-history, SEE (×2), quiet futility (×2) — at their CURRENT values, leaving the two LMR-table constants alone because RAR-S68 measured that half at zero. `razoring_coeff` is clamped at its declared rail of 300 (×1.095 rather than ×1.15). Arm A `rarog-47margin` `e950f03`, bench **7,483,775 / EBF 2.471**. Arm B `rarog-47base2` `23b21b8`, bench **7,467,143 / EBF 2.477**. Final-PGO both, `3+0.03`, 1T, 64 MB, paired UHO, RAR-M13 adjudication. **Registered bounds `[0,3]` nElo, cap 80,000, fixed before any games.** | *Pending.* | **Cap raised to 80,000 deliberately, because RAR-S68's 60,000 could not decide a true +3 (it needs 78,715) and would have died at the cap.** At 80,000 both a true +3 accept and a true 0 rejection land inside the budget, so this run returns an answer either way — the first in this line that can. ⚠ **This is the direct re-test of RAR-S54 on the current head.** That +4.06 ± 3.71 over 14,196 games was measured against 2.3.1, and 4.7's +15.56 came from the ProbCut move filter, a different mechanism — so these ten constants are untouched since. A null says the blind-shift headroom is spent and the whole line closes. ⚠ **Not a bake candidate for the scalar**, per PLAN: RAR-S54 licenses a structural rework, not shipping a uniform multiplier. A pass means locating which margin is systematically too aggressive, as 4.7 did for ProbCut. Worth noting on its own: `razoring_coeff` has moved 193 → 274 since 2.3.1 and now sits within 10% of its rail, so that one term has already been tuned hard in this direction. | `tools/test_engines/rarog-47{base2,margin}-pext-pgo.exe`; branch `p410-margin-relief`; RAR-S54 recipe; RAR-S68; RAR-S67 |
| RAR-E15 | **Phase-4 step 4.11b.17 — integrated board cluster playing gate. ACCEPTED, H1 at 1,950 games.** One dependency-complete verdict for every deliberate behaviour change in section 4.11b, as the leaf requires before 4.12 may build on it. Arm A (candidate) `rarog-411b-cand-pext-pgo.exe`, git `b33d3ad`, binary SHA-256 `8916725E...D958B617`, bench **7,601,220**. Arm B (baseline) `rarog-411b-base-pext-pgo.exe`, git `fd21612` — the 4.11b section entry, the last revision before any 4.11b source change — binary SHA-256 `6F1592FF...3A5F92C6`, bench **6,901,489**. Both final-PGO pext, `rustc 1.97.1`, both manifests `git_dirty: false`. `3+0.03`, 1T, Hash 64, paired UHO_Lichess_4852_v1, **no adjudication** (RAR-M17 default), concurrency 14 with affinity. | **Registered bounds `[-5,5]` nElo, cap 16,000 games, Alpha/Beta 0.05, fixed before any games.** *Contents:* the 4.11b.3 UCI/fullmove boundary repair, the **4.11b.5 SEE king-legality / created-pin / recapture-promotion repair** (the dominant behaviour change), plus the behaviour-neutral throughput work of 4.11b.9/4.11b.13/4.11b.14, which per the leaf does not require its own SPRT. *Prior, and it is genuinely two-sided.* The candidate searches **+10.14%** more nodes at fixed depth 13 (6,901,489 -> 7,601,220): the repaired SEE prunes less, which is a real headwind at fixed time. Against it, RAR-M41 measured **+1.421% [+0.953%, +1.764%]** whole-search NPS — but over `1d720af..head` only, so the SEE repair's own speed effect is **unmeasured** and not included. Net time-to-depth is therefore about **8.6% worse**. AGENTS records a measured +7.36% tree change worth **−1.49 ± 2.87 Elo**, and the project's ~2 Elo per 1% NPS constant puts the throughput work near +2.8 Elo. Combining those two calibrations gives a prior of roughly **−4 to +4 nElo centred near zero** — an unknown-sign repair, which is why the bracket is symmetric rather than `[0,3]`, following RAR-S62's precedent for exactly this situation. *Sizing from RAR-M10:* `drift/game ≈ 8.3e-6 × width × (true − midpoint)`; at width 10 and midpoint 0 a true +4.5 nElo reaches H1 in ~7,900 games, +3 in ~11,800, −5 in ~7,100, and a true **0 never resolves**. Cap 16,000 therefore resolves a true effect of about **2.2 nElo or larger** and is chosen over RAR-S62's 12,000 (which resolves 2.95 or larger) because this prior carries real mass near zero. At RAR-M16's no-adjudication 88.4 games/min the cap is about **3 hours**. **RESULT: H1 ACCEPTED at 1,950 games**, 12.2% of the cap, in 21m47s. Elo **+12.12 ± 10.17**, nElo **+18.40 ± 15.42**, LLR **2.96** of ±2.94, LOS 99.03%, W-D-L 530-958-462 (51.74%), draw ratio 41.74%, PairsRatio 1.25, Ptnml(0-2) [45, 207, 407, 267, 49], 1 timeout and 0 crashes. Provenance matches the registration exactly; nothing changed after games were seen. The single timeout is 0.051%, at or below RAR-M14's documented forfeit floor. | **The registered prior was badly wrong, and instructively so.** Predicted −4 to +4 nElo centred near zero; measured **+18.40**. The error was treating the +10.14% node increase as a tax by borrowing AGENTS' "+7.36% tree change worth −1.49 ± 2.87 Elo" — but that constant came from a change that grew the tree *without* improving the decisions inside it, whereas here the tree grew **because the search stopped pruning incorrectly**. **A node-count increase from removing wrong prunes is not comparable to one from widening a search; they have opposite expected signs and the constant does not transfer.** The second error was over-generalising RAR-S62's cost-5-Elo correctness fix from one precedent with a different mechanism. **RAR-M10 was validated outside its stated range:** at width 10, midpoint 0 and true +18.4 nElo it predicts 1,925 games; the run took **1,950**, within **1.3%** — and RAR-M10 had warned that anything beyond ±6 nElo or under different bounds was extrapolation, so this **extends** its validated range as a fourth point. **The magnitude is imprecise:** an SPRT decides, it does not estimate, and at 1,950 games the interval is ±10.17 Elo, so the honest reading is "clearly positive, size poorly determined" — do not quote +12.12 as settled. **No subcomponent is credited:** the SEE repair, the boundary repair and the throughput work were gated as one cluster, and per RAR-S57 splitting the gain requires an ablation that has not been run. An unresolved stop would have been an accepted outcome and not a pass. RAR-S61 is the standing warning against reading a point estimate as a result. **A rejection does not license reverting the repair**: per the leaf, investigate the implicated search consumers and re-register a coherent repair — do not relax the oracle or proclaim a known-bug baseline correct. The 4.11b.5 defects are real and independently fixtured (41 external cases), so the question this gate answers is what the *repaired* pruning costs or buys, not whether the bug was a bug. No bound, cap, book, clock or adjudication setting may change after games are seen, and no success threshold may be invented afterwards. H0 would not have licensed reverting the repair. **The development fingerprint 7,601,220 / EBF 2.474 now has its integrated verdict and becomes the accepted foundation for 4.12.** | `tools/test_engines/rarog-411b-{cand,base}-pext-pgo.{exe,json}`; `analysis/playing_gate_2026-09-08.md`; `tools/results/sprt_411bCluster_vs_411bBase_20260908_225308.{log,pgn,manifest.txt}`; RAR-M10; RAR-M14; RAR-M16; RAR-M17; RAR-M41; RAR-S57; RAR-S62 |
| RAR-M42 | **4.11b.18 endgame evidence refresh after the accepted board head, COMPLETE 2026-09-09; section 4.11b CLOSED.** Arms are the binaries RAR-E15 gated (`b33d3ad` bench 7,601,220 against `fd21612` bench 6,901,489); nothing rebuilt, every instrument node-budgeted and seeded so none of it depends on host load. | **Layer 1 clean, floors PASS both arms, 4.12 order verified UNCHANGED.** `endgame_truth.py` over 19 families x 100 positions at 60,000 nodes produced cohort digest `fe4866045506636f...` on both arms, matching the registered floors, and **theory verdicts are identical on every family — no clean win newly discarded**, which is the absolute veto. Floors: the 4.11 head reproduces the registered aggregate exactly (0.9300 -> 0.9300, 0 reports), the accepted head reads 0.9300 -> **0.9336** (+0.4 SE) with one non-blocking report. Eleven families moved both ways; largest **KRP-KR conversion 0.9178 -> 0.9726** (the order's top family at 10.04% occurrence) and **KQ-KR dtz +2.7 SE**. **Order rederived**: the 4.11 head reproduces `endgame_ranking_v2.json` across all twenty families exactly, and the accepted head equals it. | **Conversion alone would have produced a false negative.** Its four families are bare-king and came back byte-identical; over the frozen 83-position corpus the split is **34.5% (19/55) of both-sides positions differing against 0.0% (0/28) bare-king**, because SEE fires only where captures exist. **An assumption was caught by checking**: `endgame_measurement_layers.md` calls drawn-share bias "static", but `endgame_drawn.py` takes `--engine` and searches every position, so reusing it on that reading would have been reuse on a false premise. **A parameter error was caught by requiring self-reproduction**: the first rederivation used `--occurrence-scope all` and disagreed with registered v2, which records `Rating Tournament [engine], 10,000 games` — a rederivation that cannot reproduce its own registered baseline is a broken instrument until proven otherwise. **Floors were NOT updated**: KQ-KR qualifies as a ratchet candidate but KRP-KB fell 2.2 SE, and moving floors in the same commit as the change that moved them is forbidden. **Owed:** KRP-KB win-preserving 0.9990 -> 0.9949, reported and non-blocking, owner 4.12.6, blocking past 3 SE. **Not done and stated:** reference-results and tree-occurrence artifacts were held constant to isolate the census effect and are not established as current. No games, no Elo. | `analysis/endgame_refresh_2026-09-09.md`; `tools/diag/endgame_drawn_census_v2.json`; `tools/diag/endgame_truth_baseline_v2.json`; RAR-E15; RAR-M15; RAR-M24 |
| RAR-M43 | **SUPERSEDED 2026-09-09 by RAR-M44(d); raw session retained.** Two findings are wrong: the generation gap it reports was partly the HARNESS -- its Rarog 'legal moves' and 'legal captures' columns timed a 520-byte `MoveList` return copy that Basilisk's harness had already removed, so the two engines' columns never measured the same work -- and its Elo arithmetic priced only generation and make/unmake, omitting SEE (5.239%) and the never-compared check queries (5.179%). Current table: `analysis/board_comparison_411b19_2026-09-09.md`. Nothing below is deleted. **Board comparison re-measured after 4.11b, 2026-09-09.** Four arms in ONE session: `rarog-head` built from `c1a7713` with the RAR-M20 recipe flags (`fd4c83af...`), plus the three EXACT binaries RAR-M20 measured, re-timed rather than reused — `rarog-ca03a46` (`40f8fa53...`), Basilisk `d734766` (`7eeaff0c...`), Reckless `91b56c2` (`449897a1...`), all hash-matching the RAR-M20 manifest. Affinity mask 4, 150 ms warmup plus eleven 150 ms samples per workload, three cyclic orders, host busy 5.01–6.25% against a 12% rejection threshold. | **The control did not reproduce RAR-M20, which governs how everything else may be read.** The identical `ca03a46` binary measured **0.7% to 6.1% faster today** (perft 273.741 -> 290.493), a session-level offset on unchanged code. Comparing today's Rarog against RAR-M20's recorded Basilisk figure would have attributed that offset to 4.11b, so every figure here is within-session only. **4.11b's board delta** (head vs ca03a46, both today): make/unmake **+17.48%** — independently reproducing 4.11b.9's own +17.97% — two-ply +5.44%, perft +0.67%, legal moves −1.15%, legal captures −4.16%, threshold SEE **−8.07%**. **Gap to Basilisk, then -> now:** make/unmake 31.3% -> **11.8%** (19.5pp closed), two-ply 46.5% -> 38.9% (7.6pp), perft 39.2% -> 38.2%, legal moves 44.5% -> 46.2%, legal captures 20.9% -> 26.2%. | **The SEE column being 8.07% slower is the 4.11b.5 repair, not noise** — it added a per-candidate selected-king legality test the old kernel skipped. 4.11b.9 saw this column down 1–2% and attributed it to code layout; against the pre-repair binary the true cost is visible, and it is bought and paid for by RAR-E15's **+12.12 ± 10.17 Elo**. Generation was untouched because 4.11b.8 was withdrawn and 4.11b.10/4.11b.11 closed `NO_CHANGE`, so the unchanged generation gap is the expected outcome, not a shortfall. **The remaining board gap is worth single-digit Elo:** at RAR-M36's shares, closing generation and make/unmake to Basilisk entirely gives about **+2.9% NPS, ~+5.7 Elo** (it was ~+4.4% before 4.11b), of which RAR-M41 has already banked +1.421%. Board throughput is not search speed and no Elo is claimed from this instrument. Threshold SEE stays **not comparable across engines** (RAR-M19/M29); it is compared only between the two Rarog arms. Round spread reached 9.41% (Reckless) and 5.80% (ca03a46), so single-column differences below roughly 5% are unresolved. `cargo bench` emitted an executable with the **same filename** as the original build, distinguished only by hash — exactly the trap the recipe warns about. | `analysis/board_comparison_2026-09-09.md`; `tools/results/board-compare-20260909/`; `analysis/board_benchmark_recipe_2026-09-05.md`; RAR-M20; RAR-M36; RAR-M41; RAR-E15 |
| RAR-M44 | **4.11b.19 research: move-list delivery probe, 2026-09-09; (a) and (b) IMPLEMENTED 2026-09-09 (`55e228a`, `021dc98`); (b)'s production measurement RUN 2026-09-09 and BANKED at +2.48% NPS; (c) screen promoted 2 of 4 on the bench and its bundled run then REJECTED them in search at -0.55% NPS, both REVERTED; (d) RE-MEASURED on the reverted head and RAR-M43 superseded. LEAF CLOSED 2026-09-09.** Assembly of the fat-LTO production bench (RAR-M20 flags) shows `generate_legal_movelist` and `generate_captures` each ending their normal return path with `memcpy(out, local, 520)`; 4.11b.8 had seen the same copy in `generate_captures_pinned`. Basilisk's search and harness pass `MoveList&`, and its harness comment records that the copy had been measured as generation. Probe: two bench executables from `c1a7713` differing only by `tools/results/board-copy-probe-20260909/probe.diff` (caller-owned lists in four workloads, SEE and perft untouched as controls), affinity mask 4, host busy 0.8-6.4%, order base/variant/variant/base/base/variant, median of three. | **Legal captures +40.52%** (95.51 -> 134.21 M/s), **legal moves +11.15%** (443.54 -> 492.99), two-ply +4.62%, make/unmake +1.17%; controls **threshold SEE +0.05%, perft -0.69%**, all spreads under 3.1%. Base legal moves reproduces RAR-M43's same-day head (444.99) within 0.3%; read directionally against RAR-M43's Basilisk row, capture generation moves from 26% behind to about 11% ahead and the legal-moves gap from 46% to about 32%. **Registered for (b), frozen before any run:** pooled-PGO whole-search NPS under the RAR-M41 protocol, prediction **+0.5% to +1.5%**, practical floor +0.5%, one run, null pair reported; an interval including zero closes (b) `NO_CHANGE` while (a) and (d) still land. **(b) RESULT, one run, maintainer, idle host: +2.48% pooled-median whole-search NPS, 95% bootstrap [+2.29%, +2.65%]**, three PGO builds per arm (base 3,142,298 -> cand 3,220,173 n/s; best-of +2.81%). **Null pair** `cand-1` vs `cand-2`, same revision: **-0.21% [-0.57%, +0.23%]**, straddling zero, so the instrument carries no arm-level offset. All six binaries reproduce **7,601,220 / EBF 2.474** and all six hashes differ, so pooling is meaningful. **BANKED**: the lower bound is more than four times the +0.5% floor. **(c) SCREEN, four registered candidates, one at a time on the (a) parity bench, affinity mask 4, order base/variant/variant/base/base/variant, median of three, host busy 2.5-3.5%; a null pair of the base binary against itself measured every column inside +/-1% first.** **Candidate 1** (`#[inline(always)]` on `push_pawn_move_flags`, `compute_pinned`, `Board::is_attacked_with_occ`): legal moves **+4.68%**, two-ply **+4.59%**, legal captures +10.74%, perft +2.78%, SEE -0.84% -- **PROMOTED** `be5c02a`. **Candidate 2** (const-generic colour, one dispatch per generation, Basilisk's `template<Color Us>` shape): legal moves **+7.38%**, two-ply **+7.78%**, perft +6.91%, SEE +3.70%, legal captures +0.53% -- **PROMOTED** `c969ccd`. **Candidate 3** (unchecked `MoveList::push`): legal moves **-9.57%**, two-ply **-9.95%** -- **REJECTED and reverted**. **Candidate 4** (hoist six of seven `LazyLock` state checks): legal moves **-4.25%**, two-ply -1.54% -- **REJECTED and reverted**. Cumulative from the (b) head: legal moves +12.7%, two-ply +12.7%, legal captures +10.7%; fingerprint exact on magic and PEXT at every promoted step. **(c) BUNDLE RUN REGISTERED, frozen before any run:** same RAR-M41 instrument, base = the (b) head binaries (the same three that were (b)'s candidate arm, so the two measurements chain), prediction **+0.8% to +2.2%**, floor **+0.5%**, one run, null pair reported; below the floor or an interval including zero closes the bundle `NO_CHANGE` and **reverts both commits**, because unlike (a) nothing in (c) is owed regardless of speed. **(c) BUNDLE RESULT, one run, maintainer, idle host: -0.55% pooled-median whole-search NPS, 95% bootstrap [-0.76%, -0.30%]** (base 3,227,009 -> cand 3,209,296 n/s; best-of -0.55%), **null pair +0.06% [-0.46%, +0.44%]**. Below the +0.5% floor, and NEGATIVE with an interval excluding zero. **`be5c02a` and `c969ccd` REVERTED at `39542b7`**, `src/board/movegen.rs` restored byte-identical to the (b) head, re-verified at 7,601,220 / EBF 2.474 on magic and PEXT with 26/26 debug and release, fmt and Clippy clean. **(d) FOUR-ARM RE-MEASUREMENT, one session, RAR-M43's runner reused unchanged but for the head binary path, affinity mask 4, three cyclic rounds, host busy 4.8-5.7% against a 12% threshold.** **The control reproduced this time**: the identical `ca03a46` binary is within **-0.96% to +1.25%** of its RAR-M43 readings and Basilisk within +/-1.2%, so unlike RAR-M43 -- whose control drifted 0.7-6.1% -- this session is comparable to that one rather than within-session only. Gap to Basilisk, RAR-M43 -> now: **legal moves 46.2% -> 17.7%** (28.5pp), **legal captures 26.2% -> -16.0%, i.e. Rarog is now 19% AHEAD**, two-ply 38.9% -> 18.6%, perft 38.2% -> 16.2%, make/unmake 11.8% -> 7.3%, threshold SEE 34.9% -> 29.4%. Against the same `ca03a46` control, 4.11b+4.11b.19 now measures legal moves **+22.47%**, legal captures **+41.55%**, two-ply +22.17%, perft +19.92%, make/unmake +21.97%, threshold SEE -4.92%. **That session is SUPERSEDED by its own re-measurement**, because the (c) revert removed the code it described; it is retained as the record of what (c) did to the board. **(d) FINAL, on the reverted (b) head, same instrument, control within -1.32% to +1.81% and Basilisk within -1.21% to +0.37% of the RAR-M43 session:** gap to Basilisk **legal moves 46.2% -> 33.2%**, **legal captures 26.2% -> -5.5%, i.e. Rarog 5.8% AHEAD**, two-ply 38.9% -> 33.1%, perft 38.2% -> 27.2%, make/unmake 11.8% -> 12.3% and threshold SEE 34.9% -> 35.0% (both untouched by this leaf and both flat, which is the table's internal consistency check). Against the same `ca03a46` control, 4.11b+4.11b.19 measures legal moves +7.83%, legal captures +27.84%, two-ply +8.49%, perft +9.21%, make/unmake +18.70%, threshold SEE -8.68% -- reproducing RAR-M43's -8.07% within 0.6pp, as it must. | **Part of the RAR-M20/M43 generation gap was the harness, not the generator.** The two engines' 'legal moves' columns never timed the same work, and the fix Basilisk's author applied to its own harness was never mirrored. Corollary for the record: RAR-M43's 'about +2.9% NPS, ~5.7 Elo' priced only generation and make/unmake; adding SEE (RAR-M29, Basilisk +30% at matched values) and the never-compared check queries at a similar ratio roughly doubles the board-parity figure to ~+5% NPS, ~10 Elo at the STC constant -- still far below the 250-355 Elo search deficit, so the prioritisation stands. Read as microbenchmark and codegen evidence only; no Elo is claimed. **Calibration, appended after exposure and not rewriting the prediction: the registered band MISSED HIGH.** Predicted +0.5% to +1.5%; measured **+2.48%**. Sign, floor and the RAR-M36 ceiling argument all held -- 2.48% is well inside the 6.556% generation share that was named as the upper bound -- so the miss is in **magnitude**, not direction, mechanism or instrument. The failed assumption is identifiable: the band priced the copy as a *small fraction* of the generation share, and it was about **38%** of it. Two reasons, neither established here and both testable: the copy was not confined to generation -- ProbCut and quiescence capture delivery paid it at the caller's return slot, outside the generator symbol a share profile attributes time to -- and 520 bytes of store traffic per generation costs more than its instruction count suggests. **This is NPS, not Elo.** The project's recorded ~2 Elo per 1% NPS at STC would put this near +5 Elo, but that constant is a planning figure and nothing here measures playing strength; behaviour is unchanged, RAR-E15's verdict stands, and no game gate is owed. **The (c) screen's standing result is a negative one and it replicated: removing a "free" check made this generator SLOWER twice, independently** -- the `MoveList::push` bounds check cost 9.6% of legal generation when removed, and hoisting six of the seven `LazyLock` state checks cost 4.3%. Both changes provably removed work at the source level (whole-lib `panic_bounds_check` 140 -> 125 and the bench executable 2 KB smaller in one case; six derefs gone and 1.5 KB smaller in the other) and both made the generator slower. Mechanism is **unresolved and deliberately not chased** -- iterating on a rejected candidate is what the leaf forbids -- with two untested explanations on record: the bounds check may hand LLVM a range fact it uses for addressing and scheduling, or the smaller code may simply land on worse branch/loop alignment. **Do not retry either without new evidence about why.** What DID pay was giving the compiler more information rather than less: forcing inlining and making the colour a compile-time constant. **A microbenchmark column is not search speed**, which is why the promoted pair is registered for a pooled-PGO run before it is banked, and why candidate 2's doubling of the generator (`generate_legal_into` assembly 898 -> 1489 lines) is called out as an I-cache risk this instrument cannot see. **(d) corrects RAR-M43's Elo arithmetic, which was incomplete in two ways**: it priced only generation and make/unmake, omitting SEE (5.239%) and the never-compared check queries (5.179%), and its generation gap was partly the harness. On the measured gaps, closing the REMAINING generation (1.177x) and make/unmake (1.073x) gaps entirely is worth about **+1.5%** whole-search NPS, not RAR-M43's +2.9%; adding SEE at this session's matched-value 1.294x -- which independently reproduces RAR-M29's ~30% -- gives about **+2.7%**, and check queries are still not in that number, so it is a floor. At the project's recorded ~2 Elo per 1% NPS that is roughly 3 to 5 Elo, **no Elo is claimed**, and it remains far below the measured 250-355 Elo search deficit, so the 4.11b prioritisation stands. **RAR-M44's own directional prediction is scored on exactly the head it describes**: it said capture generation would move "from 26% behind to about 11% ahead" and legal moves "from 46% to about 32%". Legal moves measured **33.2%** -- essentially exact. Capture generation measured **5.8% ahead** -- right in direction, **overstated in magnitude by about half**. **The leaf's largest finding is about its own instrument, and it is a two-sided calibration: (b) REMOVED work and +11% legal moves converted to +2.48% of search; (c) MOVED work around, doubled the generator, and +12.7% legal moves converted to -0.55%.** A board microbenchmark column is not a proxy for search speed on this codebase, and its SIGN is not guaranteed to survive the translation. That also caps the corrected "remaining gap is worth" arithmetic -- about +2.4% for generation and make/unmake, +3.9% with SEE, check queries still uncompared -- as an on-paper upper bound: (c) closed a third of the generation gap on the bench and returned negative search speed. | `analysis/movelist_delivery_2026-09-09.md`; `tools/results/board-copy-probe-20260909/` (ignored: probe.diff, ab.py, ab.log, both executables, binaries.sha256); implementation and measurement evidence `tools/results/movelist-delivery-20260909/`, `tools/results/movelist-c-screen-20260909/` (screen, bundle binaries, `predeclared.md`, `nps_result.txt`), `tools/results/board-compare-d2-20260909/` (final four-arm session) and `tools/results/board-compare-d-20260909/` (superseded), `analysis/board_comparison_411b19_2026-09-09.md` (ignored: asm_lib_after.txt, asm_bin_before_after.txt, fingerprint.txt, harness_parity.txt, nps_result.txt, HANDOFF.md, asmcheck.py, where520.py, six PGO binaries) -- zero 520-byte memcpy sites in the fat-LTO binary after, four before, fingerprint 7,601,220 / EBF 2.474 exact on magic and PEXT; RAR-M20; RAR-M29; RAR-M36; RAR-M41; RAR-M43 |
| RAR-M45 | **A.5.1 reference pool refresh with Houdini 3, 1T - REGISTERED, NOT YET RUN.** Colosseum rating tournament, the 2026-09-04 fourteen-engine pool plus `D:/chess/engines/Houdini_3_Pro_x64.exe` (fifteen engines), `3+0.03`, 1 thread, Hash 64, UHO_Lichess_4852_v1, no adjudication, natural termination, concurrency 14, at least 400 games per pair. Rarog arm: the A.3.3 release binary (its pext tier if universal), or the A.3.1 head if the release was not cut, hashed; replaces the 2026-09-04 `rarog-v2.4.0-dev` binary. Clerical clarification before any game, 2026-09-09: the arm was `c80df74` when registered; the release binary carries the same engine source plus the toolchain bump, which A.3.1 qualifies as fingerprint-identical. | **Prediction, frozen 2026-09-09:** against Houdini 1.5a, Critter 1.6a, Fritz 16 and Rybka 4 the head scores the 2026-09-04 numbers (-216, -197, -161, -109) shifted by **+8 to +15** for the accepted board cluster and its +2.48% NPS, within +/-25; Houdini 3 at **-250 to -300**; Basilisk 1.9.3 within +/-15 of -26. Confidence moderate on direction, low on Houdini 3. | The table this run produces is the E.2 gate's starting row and the denominator for B.9 and C.11. A per-engine drift beyond +/-40 Elo on an unchanged opponent is a harness finding first (affinity, concurrency, book shuffle), not an engine finding. | PLAN A.5.1; `analysis/endgame_occurrence_tournament_2026-09-05.md` for the pool recipe; Colosseum tournament `41768fe9` as the prior |
| RAR-M46 | **A.5.2 four-thread pool against the four targets and Basilisk - REGISTERED, NOT YET RUN.** Same pool settings at `Threads=4`, no `-use-affinity`, concurrency chosen so that four times the concurrency stays at or below 14 cores, run only after a 4T null pair (the same Rarog binary on both sides, 400 games) reads inside +/-10 Elo. Opponents: Critter 1.6a, Houdini 3, Rybka 4, Fritz 16, Basilisk 1.9.3; 400 games each. | **Prediction, frozen 2026-09-09:** Rarog's 4T deficits are **10 to 40 Elo worse** than its 1T deficits against the same engines, because the current lazy SMP has never been tuned for quality and the opponents' SMP is mature; Basilisk within +/-20 of its 1T number. Confidence low. | The D.2 starting point. A null pair outside +/-10 Elo voids the run and is itself the finding. | PLAN A.5.2, D.2 |
| RAR-E16 | **A.3.2 consolidation release gate - REGISTERED, NOT YET RUN.** Candidate: the A.3.1 head (`dev` after the toolchain bump), PGO pext, hash in the manifest. Baseline: the 2.3.2 release binary `rarog-v2.3.2-windows-pext-native-pgo.exe`. `tools/sprt.ps1`, `3+0.03`, 1T, Hash 64, paired UHO, no adjudication, **`[3,10]` nElo**, cap 16,000 games; then `-Mode fixed -Games 400` at `10+0.1` 1T and at `10+0.1` 4T (no affinity, after a 4T null pair), zero forfeits required. Bracket chosen because the prior is genuinely large: the 2026-09-04 pool measured the head's predecessor at +43.7 Elo head-to-head over 2.3.2 in 400 games, and RAR-M10 sizes `[3,10]` at a few thousand games for a true +30. **BASELINE CORRECTION, 2026-09-09, before any gate game — the registered baseline artifact was wrong and is replaced.** The row named `rarog-v2.3.2-windows-pext-native-pgo.exe` as "the 2.3.2 release binary". It is neither. (i) The file sitting at that path benches **7,601,220 / EBF 2.474** — the *development head's* fingerprint — and reports `id name Rarog 2.3.2` only because `dev`'s `Cargo.toml` version has not been bumped since the release; it is a dev build made on 2026-09-09 at 12:29, not a 2.3.2 build. Running the gate as registered would have measured the head against itself and returned H0, which the row's own note would then have read as six accepted gates collapsing at once. (ii) `--native` is never released: `build.yml` ships `--arch {x86-64, avx2, pext, arm64}`, and both `xtask` and `build_test.ps1` document native as local-only. (iii) The pairing could not have run anyway — `sprt.ps1`'s build-flavor guard hard-fails `pext-pgo` against `pext-native-pgo`. **Replacement baseline, built from the release recipe:** `tools/test_engines/rarog-e16base232-pext-pgo.exe`, `cargo xtask build --arch pext --pgo` from tag `v2.3.2` (`f931722`) in a throwaway worktree via `build_test.ps1 -SourceRoot`, manifest `git_dirty: false`, SHA-256 `069E419E...C846B1DF`, **bench 6,519,711 / EBF 2.449 — byte-equal to the fingerprint RAR-M12 independently recorded for the 2.3.2 baseline**, which is what establishes the identity. Built with `RUSTUP_TOOLCHAIN=1.98.1` rather than 2.3.2's own 1.97.1 pin, because `sprt.ps1`'s compiler-equality guard hard-fails a mixed-compiler pair and RAR-P18 measured the two compilers as indistinguishable (-0.53%, CI straddling zero); the arms therefore differ in engine source only. Its hash differs from RAR-M12's `389E234E...` asset for that reason plus PGO profile luck. **Candidate:** `tools/test_engines/rarog-e16cand-pext-pgo.exe`, `dev` `a4c4f95`, `git_dirty: false`, SHA-256 `889B3179...F1856083`, bench 7,601,220 / EBF 2.474. Both arms `verify-isa --arch pext` clean, both 9 UCI options. **Live-wire check, 4 games, decides nothing and is not evidence:** flavor equality OK, compiler equality OK, adjudication none, UHO book indexed, affinity across 14 physical cores, concurrency 14, natural termination, zero crashes or timeouts. **Nothing else changed:** bounds `[3,10]`, cap 16,000, `3+0.03`, 1T, Hash 64, paired UHO, no adjudication, and the prediction below are exactly as registered. | **Prediction, frozen 2026-09-09:** STC **+30 to +50 Elo** (accepted rows since 2.3.2: ProbCut +15.6, root relief +2.3, HCE refit +22.0, TB labels +6.7, hce-v3 +11.8, board cluster +12.1, not additive), H1 within 5,000 games; both direction checks positive; version 2.4.0 with probability 0.6, else 2.3.3. | The first release gate under the new roadmap. H0 would contradict six accepted gates at once and is a harness or build finding before it is an engine finding. No bound, cap, book or clock changes after games are seen. | PLAN A.3, section 4 release rule; RAR-M10 |
| RAR-S70 | **Root-only LMR relief, 1536/1024 ply — REGISTERED, NOT YET RUN.** `lmr_reduction_units` is not passed the ply and `reducible` has no `ply == 0` term, so **the reduction formula cannot see the root**. From the third root move onward an alternative is searched at REDUCED depth and can displace the incumbent only by beating alpha *while reduced*. Measured mean root reduction **2.90 ply**. This subtracts 1536/1024 (1.5 ply) at ply 0 only; nothing else changes. Arm A `rarog-46root` `2a64941`, bench **6,977,070 / EBF 2.466**. Arm B `rarog-46base` `3bb6cf3`, bench **7,467,143 / EBF 2.477** — the accepted head. Both arms built from ONE tree differing only in the parameter default, and the candidate's fingerprint reproduces the value measured through `--rset LmrRootRelief=1536` on the base binary exactly, so the default path and the option path agree. Final-PGO both, `3+0.03`, 1T, 64 MB, paired UHO, RAR-M13 adjudication. **Registered bounds `[0,3]` nElo, cap 80,000, fixed before any games.** Stop rule: run to an LLR boundary or the cap; **no early stop on a point estimate**, per RAR-S61. | **ACCEPTED — H1 at `[0,3]` nElo. Elo +2.33 +/- 1.85, nElo +3.58 +/- 2.85, LOS 99.30%, LLR 2.95 over 56,928 games in 9h42m. Ptnml [1222, 6843, 12030, 7070, 1299], PairsRatio 1.04, DrawRatio 42.26%.** Merged to `dev`; new accepted head fingerprint **6,977,070 / EBF 2.466**. Second accepted gain of the phase, after 4.7c's +15.56. | **The prior is a zero-game instrument, and it is the first one in this phase that replicated.** `tools/diag/answer_compare.py` compares what the two searches RETURN, at held-constant evaluation. Final-move agreement with the oracle, relief 0 -> 1024 -> 1536: **d10 62/66/70%, d12 66/72/78%, d14 72/78/80%** — monotone in the parameter at every depth, and 1536 is the peak (2048 gives 76% at d12, 78% at d14). Root revisions move toward the oracle's at every depth (d14 1.62 -> 1.78 against 2.28), which is the mechanism doing exactly what it was built to do. And it costs nothing: **6.6% FEWER bench nodes**. ⚠ **RAR-S68 measured an UNCONDITIONAL LMR relief at −1.40 ± 6.24 — dead flat.** This is the same family and the adjacency must be stated. The distinction is population: RAR-S68 relieved every reduction in the tree, this one relieves only ply 0, which is ~0.3% of nodes and 100% of the answer. That is a real distinction and it is also exactly the kind of story that has been wrong before. ⚠ **Agreement with the oracle is a PROXY, not the objective**, on n = 50 positions. RAR-S64 is the standing warning: a mechanism with a clean bench signal measured exactly zero in games. ⚠ **Gated ALONE because the cluster does not compose.** 4.8.1's `LmrMinReducedDepth=1` scores 78% -> 70% when combined with this at d12, below either alone. Two members of one cluster interfering is why the fitted configuration is relief-only. ⚠ Two screens of this candidate were previously recorded as NULL results by a dead `--rset` wire, and one depth-replication was void because `cargo test --all-features` left a **texel** binary in place — the manifest says texel bypasses the eval caches and must not be used for strength. Both corrections are in `analysis/answer_harness_rset_correction.md`. | `tools/test_engines/rarog-46{base,root}-pext-pgo.exe`; branch `p46-root-relief`; recipe = `lmr_root_relief` default `0 -> 1536` in `src/params.rs`, one line; RAR-S68; RAR-S64; PLAN 4.6c |

**RAR-S70 result note, 2026-08-21 — the answer harness now has an exchange
rate, and it is small.**

The gain is real and it is modest: **+2.33 Elo** (95% CI [0.48, 4.18]),
**+3.58 nElo** (CI [0.73, 6.43]). Two things about reading it.

**First, an SPRT is a decision procedure, not a measurement.** It stopped at
56,928 games because that is when the evidence first separated "at least 3"
from "at most 0" at the registered error rates. Accepting H1 says the change is
real and positive; it never said the gain is large. And the point estimate at
an H1 boundary is if anything biased **upward** — conditioning on having
stopped for H1 selects favourable realisations — so +3.58 nElo is not an
understated reading of a bigger truth. It may be an overstated reading of a
smaller one.

**RAR-M10 predicted this exactly.** `8.3e-6 x 3 x (3.58 - 1.5) x 56,928 =
2.948` against the 2.95 the harness reported. The model has now called the
observed LLR to three decimals on every gate it has been applied to, and should
be used to size every future one before registration.

**Second, and this is the finding that matters: the proxy is now calibrated.**
Oracle agreement moved **66% -> 78%** at depth 12, monotone across three
depths, and it bought **2.33 Elo**. That is roughly **0.2 Elo per point of
agreement**. Extrapolating the remaining gap linearly — and it is only an
extrapolation — closing 78% all the way to 100% is worth **single-digit Elo**.

**So oracle agreement is not where the ~196 Elo deficit lives, and the reason
is structural.** `answer_compare.py` compares the two searches **at fixed
depth**, which factors out precisely the axis on which Rarog and the oracle
differ most: how many nodes it costs to reach that depth. Rarog runs 1.60x the
oracle's quiescence per node and builds a larger tree throughout. At equal
TIME the oracle simply gets more depth, and no fixed-depth agreement metric can
see that.

**Consequence for 4.6c.** The cluster's other member, `LmrMinReducedDepth`,
scores WORSE on the proxy and removes **18.4% of the tree at flat agreement**
— so it attacks the axis the harness is blind to, and its weak proxy showing is
now weak evidence against it rather than strong. It should be gated next, and
the answer harness should not be used to rank candidates that differ in tree
size. Agreement ranks move choice at equal depth; it does not rank strength.

**RAR-S69 stop note, 2026-08-20.** Stopped at 4,642 games: Elo **+4.64 ± 6.49**,
nElo **+7.14 ± 9.99**, LLR **+0.65**. Not promoted. The margin shift stays
unmerged; the branch `p410-margin-relief` carries it and the recipe is the ten
values in the row above.

Stopped on a budget judgement, not on the reading: even at 80,000 this line was
worth a few Elo at best against a ~196 Elo deficit, and the machine is better
spent on 4.6. **The blind-shift line is closed** — RAR-S54's headroom was
re-tested in two halves (LMR at RAR-S68, margins here) and neither half
produced a resolvable gain on the current head.

**Line summary, so nobody re-opens it.** Five consecutive candidates —
killer clearing, the `improving` fallback, 1T jitter, LMR relief, margin relief
— went to a gate and none cleared it, at a cost of ~39,500 games. Three were
design differences from Stockfish; two were blind scalar shifts. Against them,
the one change that DID pay in this phase was 4.7c: a structural contract
replacement ("can this capture bridge the margin" for "does it lose material")
that closed a measured 5.17× divergence against the oracle. **A difference from
Stockfish is not latent Elo, and neither is a scalar.** Candidate selection
should follow measured contract divergence, which is what 4.6 has in
abundance.

**RAR-S68 stop note, 2026-08-20.** Stopped at 4,716 games: Elo **−1.40 ± 6.24**,
nElo **−2.22 ± 9.92**, LLR **−0.44**. Dead flat rather than negative — unlike
RAR-S67, which was clearly the wrong direction by this point. Not promoted; the
`LmrRelief` switch stays at default 0 with this row as its owner.

**Two things this settles, cheaply.** First, my 60,000 cap could only have
decided a truth of ≥ +4 nElo: at `[0,3]`, a true +3 needs 78,715 games and a
true 0 needs the same to reach H0. Running it out would most likely have bought
an unresolved stop, which is why it was stopped at 4,716.

Second, and more useful: **I tested the wrong tenth of RAR-S54.** That probe
shifted **twelve** constants, and only **two** were the LMR table
(`LmrTableBase`, `LmrTableDiv`); the other **ten were pruning margins** —
futility, razoring, LMP, quiet-history and SEE. `LmrRelief` is the LMR half
alone, and it reads zero. So RAR-S54's +4.06 most plausibly lives in the
margins, not in the reduction — and 4.7's +15.56 came from the ProbCut move
filter, a different mechanism entirely, so those ten constants are **untouched
since RAR-S54 measured them**.

That narrows the hypothesis rather than refuting it, which is worth more than
the eight hours confirming "unresolved" would have cost.

**RAR-S67 stop note, 2026-08-20.** Stopped at 3,764 games: Elo **−5.35 ± 7.28**,
nElo **−8.17 ± 11.10**, LLR **−0.90**, tracking to H0 at ~12,900 games. Not
promoted. The switch is retained at default 0 with this row as its owner.

**The candidate was mis-derived, and that is the finding worth keeping.** I
read RAR-S54, RAR-S62 and RAR-S64 as "noise in the selectivity surface gains
Elo" and built symmetric jitter. Re-examining what those three actually changed:

| | change | direction |
|---|---|---|
| RAR-S54 | blind uniform 15% shift, +23.2% nodes | **less** selective |
| RAR-S64 | stale prior-reduction | **less** selective |
| RAR-S62 | ProbCut desync | *more* selective |

Two of three were **directional de-selectivity**. Symmetric jitter has **zero
mean effect** on the reduction — it adds variance without moving the average —
so it cannot reproduce a directional effect by construction. I turned "the bug
read the wrong value" into "noise helps" without checking what the wrong values
did to the search, which is the same reasoning error as the ProbCut episode.

RAR-S62 still points the other way and is not explained by this. Recorded as a
correction to the hypothesis, not as a new one: the supported claim is
narrower, that **reducing less** gains Elo here, which is what RAR-S54 measured
directly and what 4.7 delivered +15.56 of structurally.

**The bracket changed, and RAR-S61 is why.** That gate spent its full 16,000 games at `[3,10]` and returned LLR 0.39, because the candidate landed at 6.92 nElo — 0.42 from the bracket midpoint, the one value it cannot decide. RAR-M10 predicted that drift to three decimals after the fact; used prospectively now, it says `[3,10]` needs **101,205 games** for a true 7 while `[0,10]` needs 17,711. Moving the midpoint from 6.5 to 5.0 moves the blind spot off the value this candidate has already been measured at. `[0,5]` was rejected — 141,687 games for a true 3. ⚠ The fix is not assumed to help: it changes the tree by −2.0% and its sign is unknown, which is precisely why RAR-S61's +4.50 ± 3.50 cannot simply be carried forward. Findings 2 (`improving` has no `ply-4` fallback, 9.7% of nodes affected) and 3 (killers never cleared for descendant plies) are deliberately excluded so this measures the fix and not two untested behaviour changes alongside it. | `tools/test_engines/rarog-45{base,fixed}-pext-pgo.exe`; `analysis/code_audit_2026_08_19.md`; RAR-S61; RAR-S63; RAR-M10 |
| RAR-S52 | Search-quality ratio readout at the 2.3.1 head. `bench 13`, 1T, 40 per-position diagnostic dumps aggregated by `tools/diag_search_quality.ps1`. Counters only, bench-identical. | **Observation.** First-move cutoff rate **87.65%** (372,605 / 425,098); LMR over-reduction, re-search over applied, **1.80%** (17,900 / 996,204); cutoff nodes over interior nodes 13.75%. Captures delivered 1.86× more cutoffs than quiets. Reduction was clamped to ≥1 ply, so no late move escaped reduction; LMP discarded 3.71M moves against 3.09M interior nodes and RFP cut 21.9% of interior nodes. | 87.65% is only marginally under the ~90% healthy band, so raw move-ordering quality is **not** where a 40-Elo class deficit lives — the pre-registered "the deficit is ordering" branch is disfavoured. The rate is mostly carrying the TT/SEE-sorted head of the list, and a counter cannot separate healthy selectivity from over-selectivity; treating either ratio as a verdict would be exactly the failure this ledger's first rule forbids. The clamp, LMP and RFP figures size the selectivity surface for a game gate; they do not price it. | `tools/diag_search_quality.ps1`; `src/diag.rs`; branch `spsa_impr` at `36bced4` |
| RAR-S53 | Paired two-arm decomposition, Rarog 2.3.1 versus Basilisk 1.9.1: identical engines, book and seed, run once at `3+0.03` and once at `-Nodes 250000`, 1T. 3,000 games in the nodes arm; the **arm difference** is the measurement, because this pair's head-to-head runs 35–45 Elo worse for Rarog than its pool rating and reading a nodes head-to-head against a clock pool rating would charge that whole matchup effect to time management. NPS equality was verified on game positions first (2.81/2.58/4.47M versus 2.87/2.45/4.55M, within 2–5% with the sign alternating), so equalizing nodes granted neither side a speed subsidy. | **Observation, decisive for cycle direction.** Clock arm −62.15 ± 9.78; fixed-node arm −65.26 ± 9.88; paired arm difference **+3.11 ± 13.51**, zero time losses. Depth at exactly equal nodes over ~158k moves per engine (`tools/pgn_depth_at_nodes.py`): Basilisk mean **13.96** / median 13 at 3,051,641 nps; Rarog mean **16.47** / median 15 at 3,223,853 nps. | The deficit **survives** with speed and time management removed entirely: at most ~14 Elo of the −62 is speed plus TM combined, and the point estimate is ~0. Rarog reached **2.5 more plies** on the same node budget, at near-identical speed, and still lost by 65 Elo — it buys depth it cannot use by discarding width it needs. A free, falsifiable progress metric follows: re-run the depth script on a post-change fixed-node match; mean depth at 250k nodes should **fall** toward ~14 while Elo **rises**. A change that keeps the +2.5-ply lead has not fixed the over-selectivity, whatever its gate says. Method caveat: same-seed cross-match pairing bought almost nothing here (r = +0.056, CI ±13.90 → ±13.51, 2.9%), because `-games 2 -repeat` already plays each opening from both colours and the pair score has absorbed the opening imbalance by construction. Do not budget resolution on it. | `tools/sprt.ps1 -Nodes`; `tools/pgn_depth_at_nodes.py`; branch `spsa_impr` at `eaf0965`, design at `1696028` |
| RAR-S54 | Blind uniform 15% shift of the whole selectivity surface toward **less** pruning, on a throwaway probe branch, versus the 2.3.1 head. Both arms final-PGO with clean manifests and the same pinned rustc; bench 5,173,540 versus 6,373,363. `3+0.03`, 1T, paired UHO. | **Positive, deliberately stopped at LLR 1.68 of 2.94 and recorded as a STOP, not an H1.** +4.06 ± 3.71 Elo, nElo +6.27 ± 5.72, LOS 98.42%, 14,196 games, draw ratio 41.97%. Zero time losses, timeouts, crashes or illegal moves despite **+23.2% nodes** — the width-for-depth trade at a clock TC was the live forfeit risk and it did not materialise. Not a bake candidate; the values were registered as such before the run. | An **untuned, uniform, blind** de-selectivity shift beat the fitted values, which confirms the over-pruning diagnosis from the opposite direction to RAR-S53. The estimate was stable in +2.0…+4.2 over the final 7,000 games and nElo was about 2× `elo1`, so it was the width of the interval, not the centre, that kept the bound uncrossed. This licenses a structural selectivity rework with its own refit; it does **not** license shipping a uniform scalar, and the magnitude is a stopped point estimate, never a release claim. | `tools/test_engines/rarog-p100*-pext-pgo.exe` and their JSON manifests; **reconstruction recipe below this table** — `d472f6c` was a docs-only commit and the probe's real source `7693010d` was dangling, so the recipe replaces both |


**RAR-S54 reconstruction recipe, recovered 2026-08-18.** The probe's source
commit `7693010d` was found DANGLING — its branch `probe/10.0c-less-pruning`
no longer existed and the next `git gc` would have deleted it. The recipe is
recorded here so the experiment is reproducible from this document alone, and
no branch or tag is needed to keep it.

Baseline arm: `c907c2e8` on the then-`development`, bench **5,173,540**.
Probe arm: the 12 values below, bench **6,373,363**. Both final-PGO,
rustc 1.97.1, clean manifests, `git_dirty = False`. Rebuild both, confirm the
two bench fingerprints, and the arms are reproduced exactly — the fingerprints
are what verify the reconstruction, so do not skip them.

"Uniform 15% toward less pruning" means ×1.15 on every margin/threshold that
*permits* a prune, and the reciprocal on the two LMR table constants, so the
reduction gets smaller rather than larger:

| Parameter | Accepted (2.3.1 line) | Probe | Operation |
|---|---:|---:|---|
| `FutilityBase` | 60 | 69 | ×1.15 |
| `FutilityNotImproving` | 42 | 48 | ×1.15 |
| `RazoringCoeff` | 193 | 222 | ×1.15 |
| `LmpBase` | 88 | 101 | ×1.15 |
| `LmpNotImproving` | 63 | 72 | ×1.15 |
| `QuietHistPruneCoeff` | 5,069 | 5,829 | ×1.15 |
| `SeePruningCoeff` | 51 | 59 | ×1.15 |
| `SeePruningMax` | 869 | 999 | ×1.15 |
| `FpBase` | 184 | 212 | ×1.15 |
| `FpCoeff` | 117 | 135 | ×1.15 |
| `LmrTableBase` | 646 | 549 | ×0.85 |
| `LmrTableDiv` | 2,335 | 2,747 | ÷0.85 |

⚠ The "accepted" column is the **2.3.1-era** surface, not today's. `FpBase` and
`FpCoeff` in particular have since moved to 211 and 135 on `dev`. Re-running
this probe against the current head means recomputing the ×1.15 from the
current values, which makes it a different experiment — say so if you do it.

This is what a ledger row has to contain before its branch can be deleted: not
a pointer to code, but the recipe and a fingerprint that proves the rebuild
matched. RAR-S52's and RAR-S53's citations already meet that bar a different
way — every tool they name (`tools/diag_search_quality.ps1`, `src/diag.rs`,
`tools/pgn_depth_at_nodes.py`, `tools/sprt.ps1 -Nodes`) lives on `dev` today.

### Closed 4.6 follow-ups

These two results existed only in ignored `tools/results` artifacts when the
SearchCore line was reverted. They are recorded here so the stopped samples
cannot be mistaken for pending work. Neither reached a registered SPRT
boundary; both are dispositions, not accepted H0 claims.

| ID | Experiment and reconstruction | Result / disposition | Conditional lesson | Artifact evidence |
|---|---|---|---|---|
| RAR-S71 | **4.6.2 SearchCore rewrite, registered before games at `43d5174`.** Steps 13 and 16 were rebuilt together behind `search_core`; arm A default 1 fingerprint **3,479,169 / EBF 2.343**, arm B default 0 exact RAR-S70 **6,977,070 / EBF 2.466**. Registered `[-5,5]` nElo, cap 30,000, `3+0.03`, 1T, Hash 64, paired UHO, concurrency 14, strength-v2. Candidate/base executable SHA-256: `2DAE50778BCEB3BF89A19D1DB47BF6D14B4D78BE1664F3E568B5984626C4D4AB` / `B4DDEE13E06866B4BAF3F8D9941A44161AA09F568099867776832B58CA86979C`. Reconstruct without a branch in a disposable worktree by applying the inverse of revert `c5e451d` to the accepted head; the revert contains the complete 1,772-line removal, including `src/search/core.rs`. Build both arms from one tree by changing only `search_core` default 0 -> 1 and require the fingerprints above. | **Stopped manually before a boundary; rejected as the development route and reverted.** `tools/pgn_result.ps1` reconstructs 356 complete pairs / **712 games**, 181-204-327, score 48.596%, **-9.76 +/- 17.70 Elo**, LOS 13.76%, pentanomial `[20,101,134,81,20]`; two partial rounds excluded. Commit `c5e451d` restores exact RAR-S70 and passed debug/release tests, fmt, all-feature/all-target clippy and the accepted fingerprint. | The zero-game evidence was unusually strong—EBF 2.466 -> 2.343 and WAC 182/300 versus 167/300 on 54% fewer nodes—and still did not predict games. The rewrite changed several co-adapted policies while retaining constants fitted to the old search, so a loss could not attribute structure versus fit. Do not repeat a wholesale search rewrite; isolate one producer/consumer contract and gate it. | Local PGN/log/manifest SHA-256: `C67CB4E175EA8A2F1F12501FE545F222A230D62384C884F223E06C538DAE299B`, `37ACD07750F7600E0EC755B17FB21D111D17BA21A0B5B7492413467135F3DE4E`, `5429F1CEF20CF709EC370E2D4C72E1460AF218400984D74C7F586397C42F61F9`; opening seed `596286585`; book SHA-256 `7A7F6470615A69C6CF23D565417701D38732876F480AF90D67B42ABADE35644A` |
| RAR-S72 | **4.6.1 quiet SEE oracle screen.** Rarog ablation arm at `e438ced`, exact options `QuietSeePruneDepth=6,QuietSeePruneCoeff=25`; oracle `AblationMask=0`. Fixed 4,000-game screen, `3+0.03`, 1T, Hash 64, paired UHO, concurrency 14, strength-v2. Candidate/oracle executable SHA-256: `F8710A9A5ABD8E3CF7B708AC096E77F06F79848C8F7E8170F3F1F8A986BDF79A` / `10EB7301E01842C5FF2C70930A0BB01EB079163AEA50FB58871453F717D2A75E`. The option recipe plus current default-off implementation reproduces the arm; require accepted switch-off fingerprint **6,977,070 / EBF 2.466** before using it. | **Stopped diagnostic null; candidate remains default-off.** Complete-pair reconstruction gives 326 pairs / **652 games**, score 19.402%, **-247.39 +/- 23.69 Elo**, pentanomial `[138,130,51,7,0]`; three partial rounds excluded. Against the same-condition `G(0) = -250.77 +/- 13.12`, estimated gap closure is only **+3.38 +/- 27.08 Elo**. This is not an SPRT H0. | The 0.20x oracle activation divergence was real, but it did not identify portable headroom. This closes the shallow-selectivity continuation and does not authorize coefficient tuning or a broader Step-13 rewrite. | Local PGN/log/manifest SHA-256: `18DCB60925993AB61EF143E31082333D72763469D2BA3C41E5571DA0D8446030`, `990CA0391816563960B3D92C9E5DF08B90849B6B154F700367D6C84485D05FA5`, `4CF6B238A66D9D57C4891F6FCECC372AE8694FDA94302A3E1D6ACCC98E744FDA`; opening seed `1675297318`; book SHA-256 `7A7F6470615A69C6CF23D565417701D38732876F480AF90D67B42ABADE35644A` |

### Search-oracle observations

These experiments identify a development target; they do not accept the hybrid
as Rarog code or assign Elo to an individual Stockfish mechanism. Pairwise Elo
below is the ordinary logistic estimate from score and is approximate, not the
project's paired-pentanomial SPRT estimator. They are the evidence base for the
Phase-4 programme in `PLAN.md` §4. They size Rarog's own targets and order
Rarog's own work; nothing here makes resembling Stockfish a goal.

| ID | Experiment and conditions | Result / disposition | Conditional lesson | Source |
|---|---|---|---|---|
| RAR-S55 | **Phase-4 step 4.2 — first differential reading.** Versioned suite `phase4_suite_v1.epd`, 50 positions across five cohorts drawn from repo sources, fixed depth 8, 1 thread, both engines instrumented to the same counter contract. Rarog at `RAROG_DIAG_SAMPLE_STRIDE=1` (exact); oracle = `hybrid-diag` `de568b3`. Counters normalised by the node ratio (Rarog searches 1.861x the oracle's nodes at equal depth), so `norm` is firings per node searched and 1.00 means "in line with tree size". | **Observation.** All three spec invariants pass on both engines, so the join is trustworthy. **Corrected 2026-08-12 — see the note below this table.** Largest normalised divergences: `q_tt_cut` 4.25x, `singular_attempt` 3.21x, `singular_multicut` 2.98x, `probcut_attempt` 2.33x, `move_seen_quiet` 2.11x — and in the other direction `see_prune` **0.18x**, `nmp_cut` **0.22x**, `asp_fail_low` 0.28x, `root_best_changes` 0.31x, `quiet_futility_prune` 0.44x. `nmp_attempt` is 0.94x, so Rarog attempts null move as often and converts it 4.5x less. First-move cutoff rate is **higher in Rarog in every cohort** (86.7–93.2% vs 83.5–87.0%). | Rarog orders *better* than the reference and still loses ~196 Elo to it, which is a fourth independent arrival at the same place as RAR-S52 (ordering is not the defect), RAR-S53 (2.5 plies of depth it cannot use) and RAR-S54 (blind de-selectivity gained +4.06). The selectivity profile differs in shape: five times less SEE pruning (a scope difference — Rarog prunes captures by SEE, the reference prunes quiets), and null move that fires as often and pays far less. This is cluster-selection evidence for **4.7**, and it says the cluster's subject is the *shape* of the selectivity surface rather than its constants. It is not an Elo estimate and no individual counter is credited with anything. | `analysis/phase4_differential_v1_depth8.txt`; `tools/diag/phase4_differential.py`; PLAN 4.2 |
| RAR-O01 | Stage-1 evaluator-isolation experiment. Branch `hybrid` at `75d0d43`: Stockfish `9587eeeb` board/search/TT/time management calls the exact released Rarog 2.3.2 HCE through a checked Rust DLL ABI; the same executable with `Use Rarog HCE=false` is the exact-revision Stockfish HCE control. Colosseum round robin x200, 2,400 games, `3+0.03`, 1T, paired UHO, concurrency 14, tablebases/ponder off; draw adjudication `\|cp\| <= 5` for 10 moves after ply 80 and resign adjudication `\|cp\| >= 1000` for 5 moves. | **Observation, completed.** Hybrid–Rarog 275-111-14, score 82.63%, about **+270.9 Elo**; Hybrid–Basilisk 1.9.3 248-100-52, 74.50%, about **+186.2 Elo**; Stockfish-HCE–Hybrid 309-64-27, 85.25%, about **+304.8 Elo**. Average NPS: control 2.3M, hybrid 1.5M, Basilisk 2.5M, Rarog 2.4M. The extreme ordering is clear, but evaluator-dependent adjudication makes the exact gaps unsuitable as release claims. | The shipped Rarog HCE supports much stronger play under a mature search even while paying substantial adapter/throughput cost. This contradicts the premise that another broad HCE constant fit is the highest-value next step. It does not separate individual search or HCE mechanisms or prove that Stockfish contracts transfer independently. | Colosseum “Rarog Hybrid testing”, 2026-08-11; `hybrid/README.md`; PLAN §4 |
| RAR-O02 | No-adjudication confirmation of RAR-O01, stopped after 1,238/2,400 games because the architectural decision had already resolved. Same four engines, TC/book/1T/concurrency; max-move, draw and resign adjudication all off, tablebases/ponder off. Roughly 205 games were complete per pair and 982 of 1,238 games ended by natural checkmate. | **Observation, sufficient and deliberately stopped.** Hybrid–Rarog 118-77-12, score 75.60%, about **+196.5 Elo**; Hybrid–Basilisk 134-42-29, 75.61%, about **+196.5 Elo**; Stockfish-HCE–Hybrid 163-32-11, 86.89%, about **+328.6 Elo**; Basilisk–Rarog 77-70-59, 54.37%, about **+30.4 Elo**. No forfeits. Average NPS: control 2.3M, hybrid 1.5M, Basilisk/Rarog 2.4M. | Removing adjudication left decisive reciprocal search- and HCE-oracle signals despite the hybrid's lower NPS. The 74-Elo gap versus RAR-O01 prices the adjudication confounder directly: **cross-evaluator cohorts run with adjudication off**. More games would refine ratings, not change the decision to attempt one bounded native-Rust programme: freeze HCE while reworking search, freeze the accepted search head, then study HCE contracts under separate gates. | Colosseum “Rarog Hybrid testing” stopped 2026-08-11; PLAN §4; GUIDE Phase-4 step lifecycle |
| RAR-O03 | **A.5.3 oracle deficit meter G(0) on the release head - REGISTERED, NOT YET RUN.** `tools/sprt.ps1 -Mode fixed -Games 3000 -NoAdjudication`, the A.3.3 release binary (pext tier; `c80df74` engine source plus the A.3.1 toolchain bump, fingerprint-identical) against the `hybrid` oracle package (`75d0d43` branch: frozen Stockfish `9587eeeb` search plus `rarog_hce.dll`). **Clerical clarification before any game, 2026-09-09:** the oracle's `rarog_hce.dll` must be rebuilt from the release head's evaluation (`c80df74` source) so both arms share the head's HCE; the archived package carries the 2.3.2 evaluation and would mix about +40 Elo of accepted evaluation gains into a search meter. Hashes of both files in the manifest. `3+0.03`, 1T, Hash 64, UHO, 3,000 games, fixed length. | **Prediction, frozen 2026-09-09:** **-235 +/- 15 Elo** (RAR-S70-era 250.8 less the accepted board cluster and the NPS gains, about +12 and +5). Confidence moderate. | This is the number the search programme (PLAN B) is measured against at B.9; it is a diagnostic layer and accepts nothing. The oracle package must be built or restored from the tagged `hybrid` branch before A.2.2 deletes the branch. | PLAN A.5.3, B.9; `analysis/ablation_results.md`; RAR-O02 |

**Correction to RAR-S55, 2026-08-12.** The originally reported `lmp_prune`
divergence of **13.35x was an artifact** and is withdrawn. Rarog's `lmp_prune`
counts every quiet skipped (per MOVE); the oracle's fired once per node whose
quiets were suppressed (per NODE). The ratio compared moves against nodes — the
RAR-S25 denominator error, this time inside the Phase-4 instrumentation itself.

A per-node `lmp_nodes` was added to both engines and the suite re-run
(`analysis/phase4_differential_v2_depth8.txt`). The corrected reading **inverts
the finding**: Rarog suppresses quiets at **8.3%** of interior nodes against the
reference's **14.6%**, normalised **0.57x**. Rarog fires move-count pruning
*less* often, not an order of magnitude more.

Consequences: the 4.3 mechanism map's second-ranked lead for cluster 4.7 is
withdrawn, and no code was written against it — the artifact was caught while
designing the candidate, one step before it would have justified a change and a
game budget. `lmp_prune` is retained as a Rarog-only volume reading. Everything
else in RAR-S55 stands; its invariants passed then and pass now.

**Second correction to RAR-S55, 2026-08-14.** The reported ProbCut divergence —
`probcut_attempt` **2.33x** and conversion **22.7% against 91.2%** — is
**withdrawn as not comparable**, the same denominator class as the `lmp_prune`
correction above, one lead later.

The oracle's `probcut_attempt` fires inside its MovePicker loop, once per move
actually searched, up to `2 + 2·cutNode` at a node. Rarog's fired once per NODE
entering the ProbCut block, before capture generation, so nodes with no
eligible capture counted an attempt that could never convert. The `2.33x`
compared nodes against moves; the conversion pair divided cuts-per-node by
cuts-per-move-tried.

Compounding it, the oracle's **TT-served shortcut** — a stored entry already
above `probcutBeta`, so the node returns without searching anything — was made
to count as `probcut_attempt` at 4.1 specifically so `probcut_cut` could not
exceed it. That satisfied the invariant and concealed the population. Rarog has
no TT-served ProbCut path at all.

Both engines now carry `probcut_nodes` (per node), `probcut_attempt` (per
move), and the oracle carries `probcut_tt_served`. The runner's invariant
becomes `probcut_cut <= probcut_nodes`; the old form held on both engines while
being incapable of falsifying anything.

**The corrected reading, `analysis/phase4_differential_v3_depth8.txt`.** Same
suite, same depth, same oracle revision; all invariants pass on both engines and
the node ratio reproduces v2's 1.861 exactly. `probcut_nodes` reads 8,237 on
Rarog, the identical value the mislabelled `probcut_attempt` carried in v2, so
the rename is a pure rename and nothing else moved.

| | Rarog | oracle | note |
|---|---|---|---|
| entries per node | 3.019% | 2.046% | norm 1.48 |
| — of which TT-served | 0 | 1,305 | Rarog has no such path |
| reaches the SEARCH stage, per node | 3.019% | 1.156% | **2.61x** |
| moves searched, per node | 2.099% | 0.406% | **5.17x** |
| moves per search-stage entry | 0.695 | 0.351 | 1.98x |
| conversion per search-stage node | **22.7%** | **25.2%** | the gap is gone |
| conversion per move searched | **32.6%** | **71.9%** | 2.2x, not 4x |
| search-produced cuts per node | 0.685% | 0.292% | Rarog **2.35x** the oracle |

**The finding is not withdrawn, it is reshaped, and it moved one level down.**
Three things the old numbers had wrong:

1. **Per node, the conversion gap does not exist.** 22.7% against 25.2%. The
   headline "22.7% against 91.2%" that ranked ProbCut second in the 4.3 map is
   dead.
2. **75.3% of the oracle's ProbCut cutoffs are free.** 1,305 of 1,733 come
   straight from the TT with no search at all. That is what the 91.2% was
   mostly measuring.
3. **Rarog's ProbCut is more productive per node, not less** — 2.35x the
   oracle's search-produced cutoffs per node searched. It is the *price* that
   diverges, not the yield.

What survives is a **move-filter** contract, and it has the same failure shape
as 4.7a one level down: Rarog searches **5.17x** the normalised ProbCut moves
and converts **32.6%** of them against **71.9%**. Two thirds of Rarog's ProbCut
move-searches produce nothing. The mechanism is visible in source and needs no
counter: the oracle admits a capture only when SEE bridges `probcutBeta −
staticEval` and stops at `2 + 2·cutNode` moves, where Rarog admits any
`see_ge(mv, 0)` and tries up to 8.

A second, **separable** finding falls out of the same reading: Rarog has no
TT-served ProbCut shortcut, and the oracle takes one at 0.89% of its nodes for
zero search cost. That is a different contract from the move filter and must
not be bundled with it silently — it is cheap, it is not selectivity, and it
would have to be attributed on its own.

Consequence: 4.7c has a subject again, but **not the one the 4.3 map named**.
It is the ProbCut move filter, not the entry gate, and the TT shortcut is a
separate question. Also note what this says about 4.1's defect list: the
ProbCut issue was *seen* at 4.1 and closed by forcing the invariant to hold.
Making a cross-check pass is not the same as making two counters comparable —
that hid the 1,305 free cutoffs for two phases.

Frozen local Stage-1 package SHA-256 hashes: executable
`DA78A1455BAFE222BD6AF7EF243B8C62450B6BC0913C4AF3B09F8C68E14826E8`;
`rarog_hce.dll`
`E43B602B994A3A2EB86173675C5687E53415911DB63FA6FC5B1F74DB40A3F6D5`.

### Accepted or retained

| ID | Experiment and conditions | Result / disposition | Conditional lesson | Source |
|---|---|---|---|---|
| RAR-S01 | Pruning/margin SPSA Group B in the early search state. | **Accepted, +6.17 ± 4.88 nElo** after 19,458 SPRT games. | Joint fitting helped an under-tuned early parameter group. It does not imply repeated retunes of a mature group have similar value. | `CHANGELOG.md` 2.1.0 |
| RAR-S02 | Qsearch TT-bound stand-pat refinement. | **Accepted, about +6.5 Elo.** | Tighter TT evidence helped qsearch in that state, but Plan 4.2–4.3 must distinguish searched bounds from stand-pat estimates before expanding reuse. | `CHANGELOG.md` 2.1.0 |
| RAR-S03 | Per-move quiet futility pruning. | **Accepted, +7.98 ± 4.42 Elo** in the early baseline. | Move-local selectivity paid under that history/eval scale; thresholds require revalidation after prospective-depth unification. | `CHANGELOG.md` 2.1.0 |
| RAR-S04 | Joint pruning-family re-tune after the 2.2 HCE cycle. | **Accepted, +12.07 Elo;** separate LMR (−2.6), futility (~0) and TM (~0) retunes were reverted. | In that state, the joint pruning group captured the available retune value. Repeating adjacent fits without changed inputs had low value. | legacy plan at `757e9a3^` |
| RAR-S05 | Split history bonus/malus semantics and consumers. | **Accepted, +22.13 Elo.** | Under that history stack, separating positive and negative evidence materially improved learning. Preserve attribution and consumer normalization in Plan 4.5. | legacy plan at `757e9a3^` |
| RAR-S06 | Removal of the unconditional in-check extension from the then-current search. | **Accepted, +30.75 Elo.** | Removing the extension helped this Rarog state; Basilisk's −10.17 result for removing its differently gated extension shows the verdict is not portable across fitted consumers. Current Rarog therefore has no node-level check extension to stack with its singular extension. | legacy plan at `757e9a3^` |
| RAR-S07 | Broader history mechanism/tuning bundle. | **Accepted, +6.01 Elo.** | A coherent bundle produced a smaller additional gain after RAR-S05. Its members remain independently ablatable in future fits. | legacy plan at `757e9a3^` |
| RAR-S08 | Broad selectivity refit after the search-accuracy decomposition. | **Accepted, +15.33 ± 7.34 nElo;** broader tree, retained in 2.3.2. | In that baseline, Rarog appeared too selective for its decision quality. This result motivates better evidence, not indiscriminate tree growth. | `PLAN.md` Phase-4 disposition; legacy plan |
| RAR-S09 | Zero-reduction LMR floor. | **Accepted, +9.13 ± 5.45 nElo;** retained in 2.3.2. | Some nominally reduced moves benefited from a zero-reduction outcome in that model. Future shared-depth work must preserve the accepted floor unless a new gate replaces it. | `PLAN.md` Phase-4 disposition; legacy plan |
| RAR-S10 | Persistent `RootMove` records for mean, mean-square, PV, nodes and fail state. | **Retained infrastructure; isolated Elo unresolved.** | Collecting evidence does not help until aspiration, TM and fallback consume one coherent completed snapshot. | `PLAN.md` 4.7 |

### Rejected, neutral or deferred

| ID | Experiment and conditions | Result / disposition | Conditional lesson and retry trigger | Source |
|---|---|---|---|---|
| RAR-S11 | Direct port of a more Stockfish-like ProbCut formula into the older search. | **Rejected, −24.5 ± 8.5 Elo; reverted.** | Under that evaluator/search, copying a formula without matching TT provenance, history and revalidation harmed strength. Plan 4.2–4.4 tests evidence hygiene rather than “more ProbCut”. | `CHANGELOG.md` 2.1.0; `analysis/search_analysis.md` |
| RAR-S12 | Removed/altered history aging before and after the bonus/malus split. | **Rejected twice:** about −12.4 in the early state and −6.6 in the later wave. | In both tested stacks, unaged evidence was harmful. Reopen only if table ownership/normalization changes enough to invalidate both conditions. | `CHANGELOG.md` 2.1.0; legacy plan |
| RAR-S13 | `cutoffCnt` plus full LMR-family SPSA. | **Rejected, −7.78 ± 8.00.** Candidate searched about 16% more aggressively and won its tuning self-play before losing to the accepted head. | A tuner can select a sibling-local optimum. Future Plan-4.6/4.10 coordinates must gate against the accepted head and receive post-fit ablations. | legacy plan at `757e9a3^` |
| RAR-S14 | Post-LMR “do deeper” mechanism. | **Rejected, −7.29 Elo;** searched fewer nodes and lost; null pair −0.81 ± 9.09. | The tested eligibility/depth response hurt move quality rather than merely spending more time. Retry only if unified prospective depth pulls its coordinate off the neutral rail. | legacy plan at `757e9a3^` |
| RAR-S15 | Fail-soft qsearch against constants fitted around fail-hard bounds. | **Rejected, −5.96 Elo; reverted.** | A mechanically cleaner primitive can de-tune its pruning consumers. Plan 4.2–4.3 may retry the semantics only with explicit provenance and joint refit. | legacy plan at `757e9a3^` |
| RAR-S16 | Correction-history tune with `CorrGuardCapture=1`. | Aggregate mechanism washed at +1.43; the guarded tune discarded 59.7% of training and lost −55.98. Knobs returned neutral. | The run measured a crippled signal, not correction history's full value. Plan 4.5 must fix attribution/coverage before the single Plan-4.10 fit. | legacy plan at `757e9a3^` |
| RAR-S17 | Aspiration re-centering, verified mechanically against the tuned head. | **Rejected, −4.52 Elo.** | The change may have de-tuned aspiration/pruning consumers. Retry only through the completed root-confidence model and its own Plan-4.7 gate. | legacy plan at `757e9a3^` |
| RAR-S18 | Full FIDE-like draw/repetition bundle, then a reduced null-clock/fence variant. | **Rejected, −7.21 ± 6.03 and −11.91 ± 7.67;** only free mate precedence remained. | In that state, aggressive twofold handling was stronger even though the alternative was semantically cleaner. Keep legal correctness separate from optional search-draw policy. | legacy plan at `757e9a3^` |
| RAR-S19 | SEE pin-awareness verified against an independent legal-exchange oracle. | **Standalone rejected, −8.49 Elo;** mismatches improved 215→200. | Correcting a primitive can lose after its SEE thresholds are tuned around old behavior. Retry only inside a fit already justified by Plan 4.6, not via a dedicated low-EV tune. | legacy plan at `757e9a3^` |
| RAR-S20 | Half-run aspiration SPSA snapshot `ba3170b` (`15/148/149/9/20/8/0`) versus clean `p1043-base`; `[0,+3]`, `3+0.03`, 1T, 64 MB, paired UHO. | **Rejected by acceptance rule after manual stop:** 13,000 games, W-D-L 3,261-6,378-3,361, −2.67 ± 3.83 Elo / −4.16 ± 5.97 nElo, LLR −1.83 (bounds ±2.94). It did not formally hit H0, but did not accept H1; candidate bench was 7,047,226 versus 6,502,902. | In this incomplete fit, narrowing the initial window widened the tree without demonstrated strength. Do not resume or tail-select it; revisit aspiration only through the completed root-confidence model and consolidated Plan-4.10 fit. | snapshot `ba3170b`; Plan 4.0 |
| RAR-S21 | Phase-4.1 diagnostic `bench 13`, 1T, deterministic sampled interaction map on the retained 6,502,902-node baseline. | **Observation:** first-move cutoff 88.17%; LMR re-search 1.38%; sampled best move first 81.44%; TT sample hit 63.05% with 275 usable cuts and 113 contradictions; qsearch stores stand-pat/qmove/tail exact/tail upper 913/240/6/514; NMP verification pass/fail 533/7 with 83 nested attempts; pruning overlap 0.47%; 145,372 of 283,590 correction updates were capture-attributed. | Under this corpus, ordering, depth-0 TT authority, nested NMP verification and correction attribution deserve priority; the low observed pruning overlap gives little evidence that simple LMP/futility deduplication is a major prize. These counters are diagnostic priorities, not Elo estimates, and require game/TC validation. | `tools/diag_search_quality.ps1`; Plan 4.1–4.6 |
| RAR-S22 | Phase-4.2 opening static audit of the TT producer/consumer graph at `f35bc09`, plus a re-run of RAR-S21's reading on a freshly built diag binary. | **Observation.** The reading reproduced RAR-S21 digit-for-digit (fingerprint 6,502,902, EBF 2.449), so the sampled map is stable across a rebuild. Static findings: `TtEntry.flag_age` is **fully allocated** — 5 bits age (`0xF8`), 1 bit `is_pv`, 2 bits bound — so Plan 4.2's assumed "spare `flag_age` capacity" does not exist. 7 store sites and 13 read sites were enumerated. Sampled store mix: main 803 (7 exact / 508 lower / 288 upper), qsearch 1,673, ProbCut 14 — i.e. **67% of sampled stores are depth-0 qsearch entries and 37% are bare stand-pat**. ⚠ RAR-S23's exact census confirms the depth-0 and stand-pat shares (67.50% / 35.87%) but shows the ProbCut share here understated 2.4x; prefer the census. `singular_probcut_depth_match` was 32 of 101 sampled singular attempts, meaning a third of singular decisions read an entry at exactly ProbCut's `depth-3` + `Lower` signature, which cannot be attributed to a producer without provenance. `EvalPruneTtMinDepth` is seeded 0, so those depth-0 entries can refine pruning at any depth. | Under this state the shortage is attribution, not counting: the coincidence rate is measurable while the producer is not, which is the argument for typed provenance rather than for tightening a depth threshold blind. Also recorded: `bench` shares one table across its 40 positions and ages it by 8 per position, wrapping after 31, so **any change to the age field's width is bench-visible and is a behaviour change, not a free refactor** — the cheapest 1-bit provenance slot (age 5→4 bits) therefore needs a strength gate, not a fingerprint check. Retry/extend when 4.3–4.4 need a persisted producer class. | `src/tt.rs`; `src/search.rs`; Plan 4.2 |
| RAR-S23 | Phase-4.2 typed evidence refactor at `47f3ac6`: `OutcomeKind`/`NodeEvidence`/`MoveEvidence`, all 7 producers typed, all 13 read sites routed through named capability predicates. Ryzen 9 5950X, non-PGO, debug+release+diag. | **Retained infrastructure, behaviour-neutral.** `bench 13` = 6,502,902 / EBF 2.449 on both the normal and diag builds. Against the pre-refactor binary, 8 positions × 12 iterations produced 96 identical depth lines (nodes, seldepth, score, hashfull, tbhits, full PV) plus identical bestmove/ponder. Every pre-existing diag counter was identical, including the whole sampled interaction map. New exact producer census: full 31.18%, stand pat 35.87%, qsearch tail 21.07%, qsearch move 10.56%, ProbCut 1.32%, tablebase 0%; depth-0 total **67.50%**; reconciles with `tt_store_fresh + tt_store_same_key` at 2,659,461. | Two conditional lessons. (a) Sampled counters taken at different node classes do not share a denominator: the census confirmed RAR-S22's depth-0 share but showed its ProbCut share understated 2.4x, so a sampled producer count is not a share unless its denominator is stated. (b) Centralizing the admission rules exposed a real divergence that was invisible while each rule sat at its own call site — the main search's eval refinement enforces a depth floor and a `VALUE_NONE` test and the qsearch stand-pat path enforces neither. It is preserved as two named capabilities with a test pinning the difference, because RAR-S15 showed a cleaner primitive can de-tune consumers fitted around the looser one. Unifying them is 4.3 and needs its own gate. | `src/evidence.rs`; `47f3ac6`; Plan 4.2–4.3 |
| RAR-S24 | Phase-4.2b shadow test at `7815054`: what a confidence/depth penalty on window-contradicting inexact bounds would change. `bench 13`, 1T, sampled 1/1024, diagnostic build, no behaviour change (fingerprint 6,502,902). | **Observation, and it contradicts the hypothesis that motivated it.** Exposure is **269 of 1,447 sampled hits (18.59%)** — 2.4× the previously reported 113, which counted only the cutoff-eligible subset. Score consumers are materially exposed: 85 of 269 (31.6%) moved `eval_for_pruning`, mean shift **123.7 cp**; **41 of 101** sampled singular attempts were seeded by a contradicting score, 16 of which changed depth; 63 IIR suppressions. Depth-slack histogram (0/1/2-3/4-7/8+ = 20/19/16/22/8) prices a penalty directly: P=1 blocks 23.5% of those refinements, P=2 45.9%, P=4 64.7%, P=8 90.6%. **But the control pair reverses the ordering assumption:** a contradicting entry's move was best **91.79% (179/195)** of the time versus **84.77% (167/197)** for an agreeing entry — contradiction made the move a *better* ordering hint, not a worse one (z ≈ 2.2, p ≈ 0.03). | Under these conditions the penalty must be **split by consumer, not applied to the entry**: it belongs on the score consumers (eval refinement, singular seeding) and must leave move ordering and IIR alone. A plausible mechanism is that a `Lower` at or below alpha records an earlier fail-high whose *move* was genuinely strong while its *score* is stale for this window — so score and move staleness are not the same property, and a single per-entry confidence scalar would destroy real ordering evidence to fix a scoring problem. Caveats: one deterministic bench corpus, sampled, not independent games, so this constrains 4.3's design rather than settling its size. The 123.7 cp figure is not comparable to the whole population's 1,445 cp mean, which is inflated by mate-score refinements. Retry as a gated 4.3 arm; a contradicting entry can never cut off (unit-tested), so cutoffs need no arm at all. | `src/diag.rs`; `tools/diag_search_quality.ps1`; `7815054`; Plan 4.2b–4.3 |
| RAR-S25 | Phase-4.3a provenance-hazard census at `d354d02`: can a consumer infer a producer from entry shape, given provenance is not persisted? Exact counters in the store path, `bench 13`, 1T. | **Observation; the absolute counts are exact but every PERCENTAGE below is provisional and biased low.** ⚠ The kind census counts store ATTEMPTS (it runs in `TranspositionTable::store` before dispatch, which is why it reconciles with `fresh + same_key`), while the hazard counters run after the depth-preservation `return` and so count COMMITTED stores only. Dividing one by the other mismatches denominators. A stand-pat store is rejected exactly when it lands on a same-position entry deeper than 3, so the committed denominator is materially smaller and the true rates are HIGHER than printed. Raw findings: a moveless store inherits the resident move, and **33,712 stand-pat stores** walked away carrying a searched move, becoming byte-identical to a searched qmove; against 953,957 *attempted* stand-pat stores that reads as 3.53%. A shape test of `depth 0 + Lower + has a move` — the only provenance-free way to grant searched qmoves capability while denying stand pat — reads as a **10.71% false-positive rate** on the same mismatched basis. **62,821 horizon stores** overwrote a deeper same-position entry. Corrected rates pending committed/rejected counters. | Under this state 4.3 cannot cleanly separate stand pat from searched qmoves without persistence, which is the concrete trigger 4.2 registered for reopening the 1-bit provenance question (age 5→4 bits, `[0,3]` gate, RAR-S22). Suppressing the inheritance instead would make the inference exact but discards ordering evidence RAR-S24 measured as valuable, so it is not a free fix. The direction of the conclusion is unaffected by the denominator defect — correcting it can only raise the leak, and the leak already blocks the inference. The depth-0-versus-deeper split needs no move test and no persistence, so it remains sound at any leak rate; only the within-horizon split is blocked. Method lesson, third of this kind in Phase 4 after the ProbCut sampling and contradiction-gating cases: **a rate is only meaningful when numerator and denominator are collected at the same point in the code**. Add the matched counter before publishing the ratio, not after. | `src/tt.rs`; `tools/diag_search_quality.ps1`; `d354d02`; Plan 4.2–4.3 |
| RAR-S26 | Phase-4.3a arm sizing at `8acfd22`: four registered knobs A–D, one `tune` binary, 4 positions at fixed depth 12 with a cleared table per position. Node counts and best moves only. | **Diagnostic, not a verdict.** Versus baseline (552,764 nodes): **A `EvalPruneTtMinDepth=1` −15.25%** and **`=2` −43.75%, both with identical best moves** on all four probes; **B `SingularTtDepthMargin=2` −11.80%**, moves differ; **C `QsRefineMinDepth=1` +5.28%**, moves differ; **D `ProbCutStoreDepthAdj=4` +18.19%**, moves differ. All four defaults are inert — `bench 13` = 6,502,902 / EBF 2.449 on normal, diag and tune builds. | Arm A is the priority: a 44% node reduction with unchanged moves on the probe set suggests real waste in letting depth-0 bounds refine the pruning eval, and the knob's prior SPSA retained 0 while sitting ON a rail, where a fit is least informative. But four positions at one depth is very weak evidence and PLAN lesson 3 applies directly — a smaller tree can make worse decisions, so these are node counts to explain a gate, never to replace one. Arms C and D both COST nodes and must buy accuracy to be worth anything; C additionally guts RAR-S02's accepted mechanism. Registered gates pending; `[0,3]` per arm at `3+0.03`, 1T, no combination before individual verdicts. | `tools/test_engines/rarog-43a-tune.exe`; Plan 4.3 |
| RAR-S27 | Phase-4.3a **arm A**, `EvalPruneTtMinDepth=2` versus the seeded 0. Registered `[0,3]` nElo SPRT, `3+0.03`, 1T, 64 MB, paired UHO, concurrency 14 with `-use-affinity`, one `rarog-43a-tune.exe` both sides differing only by the option (SHA `559E0522…`), repo `040b49e`, seed 1246079384. | **Rejected by the registered acceptance rule after a manual stop at 23,044 games:** Elo −1.49 ± 2.87, nElo −2.33 ± 4.49, W-D-L 5,815-11,315-5,914, Ptnml [475, 2891, 4880, 2810, 466], LOS 15.44%, LLR −2.19 of ±2.94. It did not reach the H0 boundary, but H1 was unreachable (LLR would need +5.13 of travel against the drift), and under "H1 accepts, otherwise revert" the stop and a formal H0 imply the identical action. Default stays 0. **The interval includes zero — this establishes "not a ≥3 nElo gain", NOT a measured loss.** | Two lessons. (a) The candidate searched **43.8% fewer nodes for no measurable Elo**, so the depth-0/1 refinement path is doing a large amount of work of near-zero value — a strong signal that something is winnable there, but not via a flat depth floor. (b) The setting **overshot its own hypothesis**: the grievance is that depth-0 *qsearch* bounds refine deep pruning, but `=2` also denies depth-1 real searches, and the node split (−15.3% at 1, −43.8% at 2) shows depth-1 entries carry nearly twice the tree effect of depth-0. So this run is contaminated by a change the hypothesis never asked for and does **not** condemn the targeted `=1` setting. Retry `=1` before concluding anything about the mechanism; the project's own "non-monotone ≠ converged" lesson applies directly. Also note the LLR stalled for ~7,700 games near −2.2, so whole-run drift extrapolation is invalid for this design. | `tools/results/sprt_EvalPrune2_vs_Head_20260806_092417.*`; Plan 4.3a |
| RAR-S28 | Owed 4.2 throughput check: does the typed-evidence refactor cost NPS? `47f3ac6` versus `1cf9c51`, **three independent PGO builds per arm** (all six SHA-distinct), pooled and interleaved, `bench 13 3`, 10 cycles per direction, idle 5950X. Run in BOTH directions to cancel the estimator's slot bias. | **Retained: no measurable throughput cost.** Self-pair null on one binary read −0.25% median (CI −0.54…+0.10), so the estimator carries a slot penalty and anything under ~0.5% is artifact-prone. Forward (post as cand) −0.10% median / −0.10% best-of (CI −0.66…+0.22); reversed (pre as cand) +0.15% / +0.05% (CI −0.25…+0.54). Bias-cancelled `(fwd − rev)/2` = **−0.125% median, −0.075% best-of**, with an implied slot bias of only +0.025% — so the self-pair's −0.25% was mostly noise, not a real asymmetry. At the ~2 Elo per 1% NPS STC constant this bounds the cost at roughly 0.25 Elo, inside the noise floor. All six builds independently reproduced bench 6,502,902. | Under these conditions the eager per-node `NodeEvidence` construction did not cost deployable speed, so 4.2 is clean on both behaviour and throughput. Two method notes. (a) A single self-pair on ONE binary is a weaker bias estimate than the two-direction difference; when both are available, prefer the difference — here they disagreed by 0.28pp and the difference was the better-behaved figure. (b) Build variance dominates the effect: base medians spanned 0.20% while cand spanned 0.62%, with `post42b` alone 0.6% below its siblings. That is precisely the profile luck pooling exists to average out, and it is why a single-build PGO A/B cannot resolve anything at this scale. | `47f3ac6`; `1cf9c51`; `tools/nps_multibuild.ps1` |
| RAR-S29 | Phase-4.3a **arm A**, `EvalPruneTtMinDepth=1` — denying depth-0 entries the right to refine the main-search pruning eval. Same registered design as RAR-S27, repo `adf3f22`, one `rarog-43a-tune.exe` both sides. | **Rejected at formal H0:** 18,436 games, Elo −3.18 ± 3.23, nElo −4.95 ± 5.02, W-D-L 4,622-9,023-4,791, Ptnml [418, 2271, 3973, 2174, 382], LOS 2.66%, LLR −2.95 crossing −2.94. The Elo interval is **[−6.41,+0.05] and narrowly includes zero**; the registered sequential verdict and one-sided evidence reject promotion, but this is not a two-sided proof of a loss. Four time losses were symmetric. | Under these conditions the flat depth-0 floor did not meet its acceptance rule, so the default remains 0. RAR-S27 and RAR-S29 cannot rank values 1 and 2—their intervals overlap and value 2 never reached a formal boundary. Arm C was retired as low priority because it changes a different qsearch consumer already supported by RAR-S02, not because these games proved equivalence. Do not compile or time other work while a gate runs. Retry these floors only in the 4.10 joint fit, where consumers can move with them. | `tools/results/sprt_EvalPrune1_vs_Head_20260806_135739.*`; Plan 4.3a |
| RAR-S30 | Phase-4.3a refinement shadow at `8822cf2`, sampled 1/1024 over `bench 13`, 1T diagnostic build, no behaviour change (fingerprint 6,502,902). | **Observation with important scope limits.** Across 323 sampled nodes where refinement moved the eval, independent predicate checks counted RFP flips 37/2, NMP 36/4 and razor 0/13. These are not unique nodes: predicates can overlap, and the shadow evaluates later predicates even where real control flow would already have returned. The 64 completed-node tail comparison favored the refined value 45 versus 18 against the score reported by the same search. It excludes pruned nodes and uses an endogenous target. | The data show directional predicate sensitivity in this bench; they do **not** establish that 28.5% of unique nodes changed, causally explain RAR-S29, or prove the refined value is a genuinely better estimator. Use the counters to design controlled consumer ablations in 4.10, not to assign Elo or justify provenance policy. | `tools/diag_search_quality.ps1`; `8822cf2`; Plan 4.3, 4.10 |
| RAR-S31 | Phase-4.3a **arm B**, `SingularTtDepthMargin=2` versus 3. Former-policy `[0,3]` nElo SPRT, `3+0.03`, 1T, 64 MB, paired UHO, one non-PGO tune binary both sides, repo `3eeea89`. | **H1 reached on the tune binary:** 31,822 games, Elo +3.35 ± 2.44, nElo +5.24 ± 3.82, W-D-L 8,318-15,493-8,011, Ptnml [617, 3817, 6815, 3966, 696], LOS 99.64%, LLR 2.96, zero time losses/crashes. **Parked, not accepted into the baseline:** default remains 3 under the later material-gain/final-PGO policy. | This establishes only that margin 2 outperformed margin 3 under the tune conditions. It does not isolate speculative evidence: margin 2 also excludes legitimate full-search entries at `depth-3`, and older/deeper ProbCut entries can still qualify at shallower consumers. Retain value 2 as an inert 4.10 coordinate/ablation; explicit persisted provenance owns the producer question. | `tools/results/sprt_SingMargin2_vs_Head_20260806_172117.*`; Plan 4.3a–b |
| RAR-S32 | Build-transfer diagnostics for arm B on an idle 5950X: tune option and baked PGO fingerprints plus pooled NPS ratios. The first contended timing pass was void. | Both forms produced 6,100,099 nodes / EBF 2.437. Cand/base NPS was −1.15% in the same non-PGO binary and −1.355% across pooled PGO builds, about 0.2 percentage points apart under these measurements. | Fingerprint equality establishes matching deterministic search decisions for this corpus; aggregate NPS similarity suggests no large throughput interaction. Neither establishes final-PGO game strength or authorizes a baseline bake. Under the new policy arm B is parked because another long gate for an observed ~3 Elo is low priority. Timing evidence is valid only on an idle host. | `tools/test_engines/rarog-43a-tune.exe`; `rarog-43b-*-pext-pgo.exe`; Plan 4.3b |
| RAR-S33 | Phase-4.3c gate preparation and independent verification of the landed implementation. Baseline `d00e1ac`, candidate `1dc4bc6`, three clean-manifest PGO builds per arm on an idle 5950X; median-NPS build selected per side. | **Implementation verified; the gate carries a measurable speed headwind.** Candidate fingerprint **6,595,869 / EBF 2.447** reproduced independently on normal, diag AND tune builds; 863 blocked speculative singular seeds confirmed; producer census reconciles at 2,694,580; provenance round-trips on both TT backends; fmt, clippy ×3 and tests ×3 all pass. Per-build NPS spread was 0.15% base and 0.86% candidate. **Cross-arm the candidate is −2.45% NPS (CI −2.88…−2.06) while also needing +1.43% more nodes for bench depth 13**, i.e. roughly **4% worse time-to-depth**. | Under these conditions the 4.3c contract is correctly implemented and cheap in memory (10-byte entry preserved, per-generation replacement penalty unchanged, 16-generation wrap tested). But at the project's ~2 Elo per 1% NPS constant a ~4% time-to-depth deficit is a real headwind the semantic gain must overcome before it can clear a `[3,10]` bar, so a park outcome would not by itself indict the *contract* — only this cost/benefit balance. Two structural notes for whoever reads the verdict: 4.3c bundles the singular-rejection contract AND the change from a margin-shifted to an actual ProbCut stored score, and the score half has **no ablation switch**, so a failure cannot be attributed between them; and the age field narrowing is bench-visible on its own, so unlike arms A–D this step cannot be parked inert. | `rarog-43c-pgo.exe`; `rarog-d00e1ac-pgo.exe`; Plan 4.3c |
| RAR-S50 | Phase-4.10a: rebuild the accumulated-bundle composition from MEASURED subsets, as RAR-S45 requires. `bench 13` is deterministic and fixed-depth, so all **64 combinations** of the six inert members were measured exhaustively — one exact run each, no reps, no noise. Tune build, current head. | **Every recorded figure reproduces exactly, and the accumulation strategy fails anyway.** Individually: Prospective −8.70%, CorrSkip −4.50%, NmpSuppress −2.95%, SingReject −0.19%, RazorTtPv +0.11%, NmpDecisive 0.00% — all six match the record to the digit, and the six-member set reproduces RAR-S45's **+4.57%** exactly. Sum of individuals is **−16.23%** against a set effect of **+4.57%**, a 20.8-point swing. Marginal effect averaged over all 32 subsets of the others: only **Prospective (−4.24%)** and NmpSuppress (−0.35%) still help in company; **SingReject flips to +4.75%** (worst +22.10%) and CorrSkip to +1.18%. Pairwise interactions are almost all ANTAGONISTIC — CorrSkip+NmpSuppress +9.65%, CorrSkip+SingReject +6.94%, Prospective+NmpSuppress +5.38%. **The best of all 64 is a PAIR, Prospective+CorrSkip at −9.46%**, and every third member makes it worse (+NmpSuppress → −5.53%, +SingReject → −3.73%). | **The 4.4d 'accumulate before gating' strategy is refuted by its own accumulation.** Deferring the gate to collect a bigger bundle assumed the members would add; they subtract. These mechanisms prune overlapping regions, so each one claims savings the others were going to make, and the ceiling over the entire 64-point space is −9.46% — barely better than the single best member alone (−8.70%). ⚠ And node count is not Elo: the only calibration this project owns says a **+7.36% tree change was worth −1.49 ± 2.87 Elo over 23,044 games** (RAR-S27/S49), i.e. approximately zero. A −9.46% bundle therefore has an expected effect squarely inside the `[3,10]` dead zone §2 exists to keep off the gate queue — the same conclusion 4.4d reached, now with the composition measured instead of assumed. Also corrects RAR-S36: NmpDecisiveGuard reads 0.00% ALONE but has a real marginal effect in company (mean +0.84%, range −6.99% to +7.08%), so 'zero bench population' is true only in isolation. | `src/params.rs`; Plan 4.10a |
| RAR-S51 | NMP mate-clamp correctness repair: keep an unproven mate score from satisfying a null-move cutoff. Candidate `8557b18`, merged as `1358b19`, versus pre-clamp `d12d15d`; final-PGO `[−5,0]` nElo gate at `3+0.03`, 1T/64 MB, paired UHO. | **Gate interrupted and formally unresolved.** Last complete report: 2,836 games, **+2.45 ± 7.33 Elo / +4.27 ± 12.79 nElo**, W-D-L 752-1,352-732, LLR +0.80 of ±2.94; no anomaly or regression signal, but the interval is far too wide to claim neutrality. Fingerprint moves from 6,502,902 to **6,519,711**, EBF 2.449. | **Retained by maintainer decision as a correctness repair, not promoted as a strength improvement.** Do not resume or reinterpret the partial SPRT. The clamp is part of the 2.3.2 accepted architecture. This exception is explicit: it prevents an unproved mate from becoming an authoritative cutoff, while the partial games merely show no early alarm. | `src/search.rs`; `tools/results/sprt_MateClamp_vs_Baseline_20260811_170955.*`; `PLAN.md` Phase-4 disposition |
| RAR-S49 | Phase-4.9e: size the carried-in retry PLAN 4.9 reserved — RAR-S27's surviving hypothesis that `EvalPruneTtMinDepth=2`'s 'materially smaller tree' relieves TT write pressure in a way 1T cannot test. Paired within-process A/B at 1/4/8T (`go depth 18`, 20 pairs x 2 positions), plus the deterministic `bench 13` fingerprint for each knob value on a tune build. | **THE PREMISE IS FALSE: the tree is BIGGER, not smaller, so the hypothesis is void and no games are owed.** Against the same 6,502,902 baseline, `EvalPruneTtMinDepth=1` measures **7,384,102 (+13.55%)** and `=2` measures **6,981,350 (+7.36%)**. The record claims **−15.3% and −43.8%** — inverted in sign, and not matching in magnitude either. The mechanism agrees with the measurement, not the record: the 4.3 refinement shadow on this same head counts refinement CAUSING 73 prunes against PREVENTING 19, so denying refinement prunes LESS and searches MORE. The paired A/B is consistent (1T time ratio 1.163, nodes 1.135) and inconclusive at 4/8T (CIs [0.77,1.10] and [0.84,1.10] straddle 1). | **A retry can rest on a number nobody re-measured.** This hypothesis survived RAR-S27's rejection, was carried forward through 4.3, and was written into PLAN 4.9 as a reserved 4T/8T job — all resting on '43.8% smaller tree', a figure that reproduces with the opposite sign. There was never a smaller tree to relieve pressure, so the retry is CLOSED without spending a gate. ⚠ It also re-frames RAR-S27 itself: the honest reading is not 'we can cut 44% of the tree for free' but 'refinement buys a 7.4% smaller tree that is worth no measurable Elo'. **Re-measure a carried premise before building a job on it** — this is the fifth recorded claim this cycle that did not survive contact with its own instrument, after the multicut claim, the `Corr*Scale` seeds, the late-evasion predicate and the losing-check population. | `src/params.rs`; Plan 4.9e |
| RAR-S48 | Phase-4.9d: SIZE the in-check qsearch staging that 4.6c deferred here, before building it. An in-check qnode generates every evasion and scores ALL of them up front, so a node cutting on its first move paid for the rest; staging would emit the TT move before scoring anything, which is order-IDENTICAL because `score_moves` already gives it a dominating score. Exact counters (the population is small enough that sampling would add noise to the deciding number), `bench 13`. | **Population real, payoff below this project's own measurement floor — NOT BUILT.** 169,780 in-check qnodes score **608,608** evasions and try only **285,769**: 3.58 scored against 1.68 tried, so **53.0% of all evasion scoring is wasted**. But that is 322,839 wasted scorings across 6,502,902 nodes — **0.0496 per node** — and at a plausible 5–10% of a node's cost per move-scoring the ceiling is **0.25–0.50% NPS**. The 4.2c pooled PGO A/B resolved to ±0.125%, so the upper end is barely two noise widths and the lower end is inside it. | **A large SHARE and a small ABSOLUTE are different findings, and only the second decides.** 53% waste reads like an obvious win; 0.05 wasted scorings per node is one that could not be validated even if built, because a staged in-check picker is real code that risks perturbing move order — and an unmeasurable speed change is exactly what the bench-identical-plus-pooled-NPS rule exists to refuse. ⚠ Retry trigger is NEGATIVE, not neutral: after NNUE a node costs far more while a move-scoring costs the same, so this share of runtime SHRINKS and the case gets weaker, not stronger. Revisit only if in-check qnodes become a materially larger share of the tree. The counters are retained so the decision stays checkable rather than remembered. | `src/search.rs`; `src/diag.rs`; Plan 4.9d |
| RAR-S47 | Phase-4.7b: one `RootConfidence` snapshot per COMPLETED root iteration, consumed by time management and by aspiration behind separate switches, with worker instability pooled for TIME only. Measured on `bench 13`, 1T diag+tune build, 520 completed iterations; the 4T pooling probe is one `movetime 3000` reading per arm on the Kiwipete-style position. | **Landed inert and bench-identical (6,502,902 / EBF 2.449 on normal, diag and tune); three of the model's own inputs were measured and two of them changed a decision.** The scalar DISCRIMINATES - mean 400.7 per mille, quartiles 155/173/180/12 - so it is not a constant wearing a model's clothes. **SEPARATION is degenerate and ships weighted OUT:** the root gap is exactly 0 on **428 of 520** iterations (82.3%), and only **12** of those are 'no rival searched' - the other 416 are a rival scoring exactly level, because every root move but the best is searched on a null window and a fail-low reports the WINDOW, not a value. **EFFORT is sparse: the SHIPPED clock's effort factor sits at its endpoint on 473 of 520 iterations (91.0%)** - it is a constant, not a function, on this corpus. Steadiness (94/136/264/26 across octave buckets) and window (297 of 520 take a re-search) are well populated. `RootConfTime` is bench-invisible by construction and was sized by a TM SHADOW instead: reusing the effort endpoints made it an **8.85% budget CUT** (shorter on 507 of 520), so it was given its own endpoints seeded to measured level-neutrality - **+0.09%** total, longer on 295 and shorter on 182, i.e. redistribution. `RootConfAspiration` measures **6,699,671 nodes, +3.03%, EBF 2.449 -> 2.455**, cutting aspiration re-searches **917 -> 552 (-39.8%)**; two cheaper-looking seeds are WORSE (100/50 = +6.27%, 100/50 without the fail bump = +8.94%). At 4T the pooled channel carries a real population and reads **~0.81x** the own-thread instability in both probes. | **Three transferable findings.** (1) The gap's promising mean is a PVS-manufactured degenerate population. (2) Reusing the old effort endpoints silently converted a shape change into a 9% time cut. (3) Every aspiration variant costs nodes, consistent with two prior losses. Final selection fixes only `RootConfTime` ON, tunes its six identifiable consumers, leaves aspiration to Phase 7.3 and pooled instability to Phase 8.0, and marks the root-gap path for removal after 2.4. | `src/search.rs`; `src/search_threads.rs`; `src/diag.rs`; `src/params.rs`; Plan 4.7b |
| RAR-S46 | Phase-4.7a: cover the root abort/fallback path, which `bench` structurally cannot reach. Bench is fixed-depth and never aborts, so `root_interrupted_fallback` reads **0** across the whole 40-position corpus. New `tests/root_abort.rs` interrupts the search at swept poll budgets so the abort lands mid-iteration at many different points. | **Retained correctness infrastructure; no behaviour change.** Four properties now hold under abort at budgets 1-100 over four branching positions: the returned move is always LEGAL; no mate-range score is ever reported from an unfinished iteration; the reported depth never reaches the depth limit it did not complete; and aborting at the same point twice gives the same answer. All pass. Suite runtime trimmed 108s to 9.8s by lowering the depth cap, since abort is a mid-iteration property that small budgets already cover. | PLAN 4.7's "abort returns last completed legal evidence" and "incomplete mate/win/loss never becomes authoritative" were previously **unverifiable claims**: nothing in the fingerprint, the tactical suites or the strength gates exercises an interrupted root. This is the third such blind spot found this cycle - after null-move soundness (`tests/zugzwang.rs`) and the zero-population decisive guard - and they share a shape worth naming: **a property that only manifests under a condition the deterministic corpus excludes needs its own test, or it is merely asserted.** Determinism is included deliberately: a fallback that varied run to run would mean ownership depends on something outside the recorded root evidence, which is exactly what 4.7 is meant to rule out. | `tests/root_abort.rs`; Plan 4.7a |
| RAR-S45 | The two verifications owed after 4.6: (a) count the safe-versus-losing quiet-check population, since `CheckBonusLosing` had measured 0.00% even at 0; (b) measure the six-member 4.10a bundle as a SET rather than trusting the individual sizings to compose. `bench 13`, idle host. | **Both negative, and both changed a decision.** (a) `check_order_safe = 332,683`, `check_order_losing = **0**`. The split could never fire, and zero across 332k moves indicts the PREDICATE rather than chess: `see_ge(mv, 0)` is evidently trivially satisfied for a non-capturing move, so it is the wrong test for "the checker can be taken at a loss". The split is **reverted**; the census is kept and `CheckBonusSafe` survives as a 4.10 coordinate. (b) The bundle measures **6,800,242 nodes, +4.57%, EBF 2.450** - the individual effects summed to about **-17%**, so composition flipped the SIGN. | Two lessons, both expensive to have learned later. **A switch that is inert when off and also inert when on is dead code, not a tunable** - and the way to tell them apart is a population counter, not a node-count delta, because a zero delta is exactly what both look like. **Individually-sized arms do not compose**: this bundle was assembled from six cheap-or-negative members and is a +4.57% HEADWIND as a set, the same shape as RAR-S34's candidate that landed dead neutral. Any bundle must be sized as a set before its bounds and cap are registered, and 4.10a's composition now has to be rebuilt from measured subsets rather than from a sum of singles. | `src/search.rs`; `src/params.rs`; Plan 4.6c, 4.10a |
| RAR-S44 | Phase-4.6c: replace the flat `DIRECT_CHECK_BONUS = 32_000` with safe/losing check classes in quiet ordering, and dispose of 4.6's two remaining items. | **Mixed, and one part is NOT verified.** The class split is implemented and inert (bench 6,502,902 / EBF 2.449 on normal, diag and tune) and is free at the default because the SEE probe is skipped when the two bonuses are equal. **But `CheckBonusLosing` measures 0.00% node change at both 16000 and 0**, so it is NOT demonstrated effective: either the losing-check population is empty on this corpus or the class is not reaching the live ordering path. The duplicate `DIRECT_CHECK_BONUS` constant was removed and its test now reads the live parameter, so the two can no longer drift. | **This arm must not enter any bundle until its population is counted and it is shown to change the tree.** A switch that is inert when off AND when on is indistinguishable from dead code, and this cycle has already produced three comment/code mismatches - concluding it works because it compiles would be the same class of error. Owed: a safe-versus-losing check counter. Dispositions for 4.6's other two items, both by PRIOR evidence rather than new work: **post-LMR depth feedback is already rejected twice** (Phase 2.8 at -1.38 Elo, RAR-S14 at -7.29 Elo), so it needs a retry trigger not a third attempt; and **in-check qsearch ordering is already complete** - quiescence calls the full `score_moves` when in check - so what former 4.3d wanted was lazy STAGING, a throughput change that migrates to 4.9. | `src/search.rs`; `src/params.rs`; Plan 4.6c |
| RAR-S43 | Phase-4.6b: derive LMP, futility and SEE pruning from the same PROSPECTIVE depth LMR will search the move at, instead of from raw `depth`. One shared reduction formula extracted so both callers use it, with a `debug_assert` that they agree. Sized on `bench 13`. | **Diagnostic, and the largest cheap arm in Phase 4.** `SelectivityProspectiveDepth = 1` measures **5,937,163 nodes, −8.70%**, with EBF falling 2.449 to **2.424** — a genuinely narrower tree, not just a cheaper one. Default 0 inert; bench 6,502,902 / EBF 2.449 on normal, diag and tune. The shared formula is verified rather than asserted: a `debug_assert_eq!` at the LMR site checks it derives the same reduction units as the pre-move estimate, and it held across the whole debug test suite. | Under these conditions the audit's finding is confirmed and fixed: a move about to be reduced by 3 plies was being judged for pruning as if it were not, and making the two coherent removes 8.7% of the tree. The three consumers move together on ONE knob by design — switching them individually would recreate exactly the mixed-depth incoherence the step exists to remove. Two terms are excluded from the shared depth and documented rather than hidden: the per-thread jitter, which must be drawn once at the real reduction site and is not drawn at all at `Threads = 1`; and the singular extension, because pruning runs before the extension is known. ⚠ Pruning MORE is not automatically better — RAR-S27's arm pruned 43.8% more and lost — but that arm restricted EVIDENCE while this one aligns a decision with the depth actually searched, which is a different kind of change. It is the strongest single member of the 4.10a bundle and still needs the gate. | `src/search.rs`; `src/params.rs`; Plan 4.6b |
| RAR-S42 | Phase-4.6a: resolve the documented late-evasion contradiction. The LMR comment claimed "`!in_check` removed, so late evasions are reducible" while the live predicate still carried `&& !in_check`. Comment corrected, behaviour made testable behind `LmrReduceLateEvasions`, sized on `bench 13`. | **Observation, and it resolved in favour of the CODE.** The comment was false: evasions were never reducible. Making them reducible measures **7,467,531 nodes, +14.83%** — one of the most expensive arms in Phase 4. Default 0 is inert; bench 6,502,902 / EBF 2.449 on normal, diag and tune. Also confirmed a *correct* piece of documentation for contrast: `check_extensions` reads 0 because the extension was deliberately removed and the counter is left defined as explicit confirmation — that comment says exactly what the code does. | Under these conditions the stale comment was describing a change that would have cost ~15% of the tree, so the code being authoritative was the better state — reducing an evasion triggers far more LMR re-searches than it saves. It is categorical and therefore excluded from the final SPSA, stays OFF for final-theta ablation, and has no retry owner after 2.4. **Third comment/code mismatch this cycle** (after the false multicut claim and the `Corr*Scale` "seeds are 0" claim): in this codebase a comment asserting a mechanism's state is not evidence — read the predicate. | `src/search.rs`; `src/params.rs`; Plan 4.6a |
| RAR-S41 | Phase-4.5d: does any further correction CONTEXT carry usable signal? Exact residual buckets by halfmove clock on `bench 13`, plus a structural check of the check/evasion context. Defaults inert; bench 6,502,902 / EBF 2.449 on normal and diag. | **Observation: no new context is justified, and for two different reasons.** Halfmove clock 0-19 holds **279,741 of 283,590 updates (98.64%)**, clock 20-49 just 2,377 (0.84%) and 50+ only 1,472 (0.52%); buckets reconcile exactly against `correction_updates`. Mean residual magnitude does differ - 130.7 / 115.8 / **61.9 cp**, so high-clock residuals are less than half the size of low-clock ones - but on 0.52% of samples that cannot support a learned context. Separately, the **check/evasion context is unreachable by construction**: correction trains only where `static_eval != VALUE_NONE`, which *is* the not-in-check condition, so its population is zero without needing measurement. | Under these conditions PLAN 4.5's "add contexts only with held-out unique signal" resolves to **add none**. The instructive part is that the halfmove context fails on **population, not on signal** - its 2.1x low-versus-high ratio looks as interesting as 4.5a's 2.27x capture ratio, but 4.5a's split was 51/49 while this one is 99/0.5, so one is learnable and the other is a table of slots that never fill. **A context needs both a distinct mean and a population to learn from; checking only the mean would have justified a useless table.** That leaves the capture/quiet split as the only context with both. Retry trigger: revisit if a future corpus or TC materially shifts the clock distribution - a rule-50-heavy endgame cohort would, and Phase 5.0 freezes exactly such cohorts. | `src/search.rs`; `tools/diag_search_quality.ps1`; Plan 4.5d |
| RAR-S40 | Phase-4.5c: is the correction-uncertainty term applied to an eval the correction is no longer part of? Exact census plus a `CorrSkipWhenTtRefined` guard, landed inert. `bench 13`, idle host. | **Observation — a real mis-application, and a large one.** `corr_abs` widens the RFP and futility margins and shrinks LMR in proportion to how far the correction moved the eval. But `eval_for_pruning` can be REPLACED wholesale by a TT bound, and when it is, the corrected eval is discarded while the margins are still widened by the discarded correction's magnitude. Exact population: **360,811 nodes, 9.0% of the 4,005,332-node tree.** Switching the term off in exactly that case measures **6,210,236 nodes, −4.50%** — the only 4.5 arm that is *cheaper* than baseline. Defaults inert; bench 6,502,902 / EBF 2.449 on normal, diag and tune. Also corrected a **stale comment** that claimed these scales were seeded at 0 and the term therefore vanished: the fitted seeds are 3/3/27, so it has been live all along. | Under these conditions the term is charged for an adjustment that is not present in the number being tested, on 9% of nodes — a coherence defect rather than a tuning question, and the kind that a bench fingerprint can never reveal because it is behaviour the baseline has always had. It is a strong first-bundle candidate on both grounds: principled *and* −4.50% nodes. ⚠ Not assumed to be a gain: RAR-S30 showed TT refinement is earning strength, and a wider margin may be doing useful work for reasons unrelated to its stated rationale, so it rides the 4.4 bundle gate rather than being baked. Method note: the stale comment is the second documentation defect this cycle that would have misled a reader into thinking a live mechanism was inert — **re-read a mechanism's seeds, do not trust its comment.** | `src/search.rs`; `src/params.rs`; Plan 4.5c |
| RAR-S39 | Phase-4.5b: continuation correction extended from a single 1-ply `(piece, to)` slot to compact 2- and 4-ply distances, all three tables aged through one loop, both new weights landed inert. Sized on `bench 13` at the existing 1-ply weight of 152 for comparability. | **Diagnostic.** Inert defaults verified: `bench 13` = 6,502,902 / EBF 2.449 on normal, diag and tune builds. Each term alone: `CorrWeightCont2 = 152` **6,914,454 (+6.33%)**, `CorrWeightCont4 = 152` **7,284,605 (+12.02%)**, and 4.5a's `CorrCaptureWeightPct = 44` **6,839,617 (+5.18%)**. | Under these conditions every 4.5 arm **grows** the tree, so none is a free addition and none belongs in a bundle on node cost alone — the same test 4.4's arms had to pass. Deliberately given weights of **0 rather than a plausible-looking default**: adding an eval term with a guessed weight is how RAR-S13 went wrong, and the 4.10 fit is what decides whether distance 2 or 4 carries unique signal beyond the 1-ply term. If neither does, both stay at 0 and the tables cost nothing, since read and write are both skipped at weight 0 — the default performs no table access at all. Aging was centralized in the same commit for a specific reason: three sibling tables on different halving schedules drift out of scale with one another, and then a fitted weight stops meaning what it meant when fitted. That failure would be invisible to a fingerprint. | `src/search.rs`; `src/params.rs`; Plan 4.5b |
| RAR-S38 | Phase-4.5: is a capture-caused correction residual actually noisier than a quiet-caused one? Exact per-class residual magnitudes on `bench 13`, plus a graded `CorrCaptureWeightPct` alternative to the existing binary `CorrGuardCapture`, landed inert. | **Observation, and it SUPPORTS the premise for the first time.** Capture-caused: **145,372 updates (51.26%), mean |residual| 179.1 cp**. Quiet-caused: **138,218 updates, mean 78.8 cp**. **Ratio of means 2.274.** Both denominators reconcile exactly against `correction_updates` (145,372 + 138,218 = 283,590) and `correction_on_capture`. `CorrCaptureWeightPct = 100` is inert; `bench 13` unchanged at 6,502,902 / EBF 2.449 on normal, diag and tune builds. | This retroactively explains RAR-S16. The capture guard was **directionally right** — capture residuals really are ~2.3x larger, so the positional eval is being asked to absorb much bigger surprises from tactical cutoffs — but the **instrument was wrong**: excluding them discards 51.3% of all training, and RAR-S16 measured that at −55.98 Elo. Scaling preserves coverage while down-weighting the noisier class, which is what the plan asked for and what the evidence now justifies. Under these conditions a weight near 100/2.27 ≈ 44% would roughly equalise each class's contribution per update, but that is a **tuning** question for the 4.10 fit, not a value to bake here — RAR-S13 is the precedent against baking an untuned constant. Retry trigger: the weight enters 4.10 as a coordinate; do not gate it standalone, since a correction weight is exactly the kind of consumer constant lesson 2 says to fit after architecture. | `src/search.rs`; `tools/diag_search_quality.ps1`; Plan 4.5 |
| RAR-S37 | Phase-4.4c: potential-singularity guard, tightenable NMP material floor, and the double-extension margin as a coordinate. Landed inert, sized alone on `bench 13`, plus the registered first bundle measured as a set. Idle host; defaults inert on normal, diag and tune builds. | **Diagnostic.** Alone versus 6,502,902: `NmpSingularGuard=1` 7,239,391 (+11.33%), `NmpMinNonPawnPieces=2` 7,352,355 (+13.06%), `=3` 7,122,560 (+9.53%), `SingularDoubleMargin=60` **6,356,465 (−2.25%)**. The registered first bundle (`SingularRejectSpeculative` + `NmpSuppressNullInVerification` + `RazorAllowTtPv` + `NmpDecisiveGuard`) measures **6,401,087, i.e. 1.57% FEWER nodes than baseline**. | Two structural findings. (a) The material floor is **non-monotone** — 3 costs less than 2 — another instance of "non-monotone ≠ converged", and a warning against reading these as a smooth curve. (b) Restricting double extensions by a **larger margin saves** nodes (−2.25%) while removing them outright via `SingularMaxExtension=1` **costs** them (+6.57%), though both reduce doubles: the margin downgrades only marginal cases while the cap also downgrades strongly-singular ones, and losing a critical line costs more work elsewhere than it saves. `SingularDoubleMargin` stays inert rather than joining the bundle because 60 was an arbitrary probe and baking an untuned constant is the RAR-S13 trap; it belongs in the 4.10 fit. The bundle being *cheaper* than baseline is the point — unlike RAR-S34's candidate it will not be fighting a speed headwind. ⚠ But see the resolvability arithmetic in `PLAN.md` 4.4c: at its ~5 nElo prior this bundle sits inside the indifference region a 16,000-game `[3,10]` gate cannot resolve. | `src/params.rs`; `tests/zugzwang.rs`; Plan 4.4c |
| RAR-S36 | Phase-4.4b guards landed inert and sized: NMP cut-node guard, NMP decisive-window guard, NMP static-vs-TT-refined null threshold, and a singular extension cap. Plus a new `tests/zugzwang.rs` soundness suite. `bench 13` deterministic node counts; idle host. | **Diagnostic, no strength claim.** Each arm alone versus 6,502,902: `NmpRequireCutNode=1` **7,440,358 (+14.42%)**, `SingularMaxExtension=1` 6,930,264 (+6.57%), `NmpUseStaticEval=1` 6,675,647 (+2.66%), and **`NmpDecisiveGuard=1` 6,502,902 (0.00% — zero population on bench)**. All defaults inert. The zugzwang suite passes with every switch off, each on individually, and all ten on together. | Every 4.4b arm **costs** nodes, so none belongs in a first bundle on current evidence — RAR-S34 showed a +4.34% time-to-depth candidate landing dead neutral, and `NmpRequireCutNode` at +14.42% would need to buy a great deal. The decisive guard is the interesting case: zero bench population means the **fingerprint cannot verify it in either direction**, so it is a soundness guard rather than a strength arm, and `tests/zugzwang.rs` is the only evidence that enabling it is safe. That suite is the reusable product here: null-move unsoundness is invisible to bench fingerprints and tactical suites because a bad null cutoff yields a *plausible* move, so it needed its own instrument. Method note: the suite's first draft asserted properties of positions that did not have them — a "one legal move" position that was stalemate, a "blocked draw" the engine correctly scored −1352 because the kings were not symmetric, and two "in check" positions that were checkmate. **Probe a position's legal-move count, check status and score before asserting anything about it**; three of six tests failed on my premises, not on engine behaviour. | `tests/zugzwang.rs`; `src/params.rs`; Plan 4.4b |
| RAR-S35 | Phase-4.4a switch sizing: five mechanisms landed inert, then each measured alone on `bench 13` (deterministic node counts) with exact diagnostic populations taken with every switch OFF. Idle 5950X; `bench 13` = 6,502,902 / EBF 2.449 on normal, diag and tune builds with all defaults. | **Diagnostic, no strength claim.** Populations: the shared `tt_pv` veto blocks all four forward-pruning mechanisms at **24,754** nodes, of which RFP would reach 21,689 (87.6%), NMP 16,544 (66.8%), ProbCut 16,536 (66.8%) and razoring 8,218 (33.2%). Nested nulls inside a verification subtree: **83** exact. IIR at a PV node: **1** sampled. Node cost of each switch alone versus 6,502,902: `NmpSuppressNullInVerification` **6,310,949 (−2.95%)**, `RazorAllowTtPv` 6,509,913 (+0.11%), `NmpAllowTtPv` 6,797,234 (+4.53%), `RfpAllowTtPv` 6,936,480 (+6.67%), `ProbCutAllowTtPv` 7,577,452 (+16.52%). | Under these conditions the 4.4 bundle splits cleanly by cost, which is the lesson 4.3 paid ~100k games to learn. Cheap or free: NMP subtree suppression (−2.95% nodes), razoring `tt_pv` eligibility (+0.11%) and the already-measured 4.3c contract (−1.15% time-to-depth). Expensive: ProbCut `tt_pv` at +16.52%, RFP at +6.67%, NMP at +4.53% — each must buy a lot to survive, and RAR-S34 showed a +4.34% time-to-depth candidate landing dead neutral. Build the first bundle from the cheap set only; hold the expensive three for a second bundle if the first passes. Two items are also **de-scoped by measurement**: PV-safe IIR has a population of ~1 sampled node, so it is a correctness tidy-up and not a strength arm; and node deltas here are per-switch at fixed depth, not Elo, and compound unpredictably when combined (all four `tt_pv` switches *raise* node counts even though three of them enable pruning). | `src/params.rs`; `tools/diag_search_quality.ps1`; Plan 4.4a |
| RAR-S34 | Phase-4.3c **gate result and cost attribution.** Gate: candidate `1dc4bc6` versus baseline `d00e1ac`, final-PGO, registered `[3,10]` nElo at `3+0.03`, 1T, 64 MB, paired UHO, budget 16,000. Attribution: 4.3c peeled into its three sub-changes, three independent PGO builds each, bench fingerprints plus one interleaved 5-cycle NPS pass over all twelve binaries on an idle 5950X. | **Gate: not promoted — dead neutral.** Manually stopped at 4,960 games, Elo **+0.35 ± 6.18**, nElo +0.55 ± 9.67, W-D-L 1,261-2,443-1,256, LOS 54.42%, PairsRatio 1.00, LLR −1.71 of ±2.94 (58% toward H0 and steady). RAR-M10 predicted the drift for a truly neutral candidate to within 1% (−1.71 predicted, −1.70 observed at 4,516), so the trajectory was as designed, not anomalous. **Attribution (time-to-depth = node ratio ÷ NPS ratio):** age narrowing 5→4 bits **0.00% nodes, +0.10% NPS, −0.10% TTD — free**; plus singular rejection of speculative evidence **−0.19% nodes, +0.97% NPS, −1.15% TTD — free and slightly FASTER**; plus the actual-ProbCut-score change **+1.43% nodes, −2.79% NPS, +4.34% TTD.** The score change alone accounts for **+5.55% TTD** (v2→v3: +1.62% nodes, −3.73% NPS). | **Two of my own prior claims are refuted by this.** (a) I asserted the age narrowing was bench-visible and so 4.3c "cannot land inert" — it is bench-IDENTICAL at 6,502,902 and the bit is genuinely free. (b) I attributed the headwind to the age narrowing as the likely main cost; it contributes nothing. The entire ~4.3% deficit comes from the bundled ProbCut actual-score change, which the singular contract does not need — provenance delivers that, not the stored value. The plausible mechanism ties to RAR-S30's structural finding: a higher stored `Lower` raises `eval_for_pruning`, and refinement acts almost purely as an upward correction that *prevents* razoring, so more nodes survive and the mix shifts to expensive interior nodes. **Actionable:** the contract without the score change (v2) is strictly cheaper than baseline, so it costs nothing to retain and is the variant that should be carried. Do not read this gate as evidence against the contract — it tested contract plus a costly extra. Retry the contract only inside 4.4's bundle, where "evidence-bound singularity" needs the bit anyway and a ~5 nElo prior (RAR-S31) cannot clear `[3,10]` standalone. **Landed accordingly:** the infrastructure is retained and both behaviour changes became switches defaulting off, so the head is bench-identical to the accepted baseline at 6,502,902 / EBF 2.449 with no strength gate owed. `SingularRejectSpeculative=1` reproduces 6,490,746 and adding `ProbCutStoreActualScore=1` reproduces 6,595,869, both checked against the variants built for this attribution — so the switches are exact reconstructions, not approximations. | `tools/results/sprt_43c_vs_Baseline_20260807_112013.*`; Plan 4.3c |
| RAR-S33 | Phase-4.3c implementation candidate: persist one speculative TT bit by reducing age 5→4 bits, store the actual ProbCut fail-high and deny that class only at singular seeding. Non-PGO release/diag `bench 13`, 1T; baseline `d00e1ac`. | **Implementation complete; strength unresolved.** Candidate fingerprint 6,595,869 / EBF 2.447 versus baseline 6,502,902. Exact diagnostic count: 863 speculative entries otherwise met the singular seed predicate. Local/shared provenance round-trip, 16-generation age wrap, unchanged per-generation replacement penalty and consumer-contract tests pass. Registered final-PGO `[3,10]`, maximum 12,000 games; only H1 promotes. | Under this bench, explicit provenance reaches real consumers that the old `depth-3 + Lower` signature could not identify. The count establishes exposure, not benefit: actual-score storage, shorter age horizon and singular exclusion move together in this candidate, so the final-PGO game gate decides the bundle. If it fails, ablate those three effects rather than infer which one caused the result. | `src/tt.rs`; `src/evidence.rs`; `src/search.rs`; Plan 4.3c |

## 4. Root search, time management and SMP

| ID | Experiment and conditions | Result / disposition | Conditional lesson and retry trigger | Source |
|---|---|---|---|---|
| RAR-R01 | Early Stockfish-style clock management on the old harness. | **Accepted, reported +81 Elo,** but the harness/protocol predates current calibration. | The direction and zero-forfeit improvement were useful; the magnitude is not a current prior. Revalidate only changed clock behavior under Plan-2 gates. | legacy plan; `CHANGELOG.md` 2.1.0 |
| RAR-R02 | Clock safety reserved `2*MoveOverhead`; fixed movetime used its full budget. | **Retained:** 28 fast-TC forfeits fell to zero; cumulative close gate +2.02 ± 3.62 Elo. | Under those time controls, safety dominated nominal think-time recovery. Preserve zero-forfeit gates after root/TM changes. | `CHANGELOG.md` 2.1.0 |
| RAR-R03 | Five-change Lazy-SMP/root-result bundle versus the original 4T implementation. | **Accepted, +102.78 ± 16.38 at 4T;** externally consistent with the 2.3 boundary. | The deployed bundle was strongly better, but no individual member inherits that value because it was never decomposed. | legacy plan; `CHANGELOG.md` 2.3.0 |
| RAR-R04 | Symmetric early stop vote at 2T. | **Rejected, −15.85 Elo.** | Taking the first of two estimated expiry votes biased time downward in this design. Plan 4.7 treats soft time as confidence, while maximum time remains the hard budget. | legacy plan at `757e9a3^` |
| RAR-R05 | Pool-view instability TM. | **Rejected, −5.54 Elo.** | Raw helper instability was not a useful direct time multiplier in that state. Plan 4.7 may pool it only as one input to completed-root confidence. | legacy plan at `757e9a3^` |
| RAR-R06 | Helper-history blending and additional ordering jitter. | **Neutral/rejected:** blending −0.52; jitter reverted. Shared TT hit rate already rose strongly with thread count. | In that state, TT coupling made generic diversification/history sharing largely redundant. Reopen only with measured independent-work failure, as required by Plan 4.9. | legacy plan; `analysis/smp_analysis.md` |
| RAR-R07 | Phase-4.9 opening profile of accepted semantics at 1/2/4/8/16T on an idle 5950X. Two instruments, deliberately independent: `nps_scaling.ps1` on the pext PGO asset (2 pinned middlegame positions x 2 reps, `movetime 5000`, Hash 256, per-position ratios) for NPS and time-to-depth, and `diag_smp_sweep.ps1` on a diag build (3 reps, `movetime 3000`) for TT contention, aspiration churn and per-thread depth. | **Throughput scales almost perfectly and DEPTH DOES NOT MOVE.** NPS **1.81x / 3.95x / 7.88x / 12.31x** at 2/4/8/16T, while depth at fixed time goes **−1.0 / −1.0 / −0.8 / −0.8 plies**. The diag sweep agrees from the other side: every thread completes depth **22 at every thread count**, 1T included, for 10.3x the nodes. This reproduces the 9.7.5(b) observation (16 threads, ~13x nodes, +0 depth) on current head. **Three of the recorded hypotheses are refuted by their own counters.** Aspiration churn: re-searches per thread FALL, 34.0 → 36.0 → 34.0 → 30.2 → 28.4, so pool-seeded windows are not the cost and if anything help. Helpers not reaching main: main TT hit rate RISES 52.1 → 53.2 → 56.8 → 57.6 → **62.5%**, so helper work does arrive. TT store duplication: same-key share rises only 26.2 → **33.8%** across a SIXTEEN-fold thread increase, and 26.2% of stores already hit the same position at 1T. | **The pool contributes, does not duplicate much, does not churn its windows — and still buys no depth.** That is the finding, and it moves the question from 'is SMP working' to 'why does more TT content not make iterations cheaper'. The lead is already in the 4.1 census: at 1T only **275 of 1,447 sampled TT hits (19%) are usable for a cutoff** while 1,172 are `tt_bound_not_usable`, so a rising hit rate may be adding hits that cannot cut. Next measurement is the cutoff-usable share as a function of thread count, not a TT layout audit. ⚠ Descriptive profile, not a verdict on SMP strength: depth at fixed time is the standard SMP quality proxy but RAR-R03's +102.78 Elo was 4T-new versus 4T-old, and **no 1T-versus-4T strength measurement exists for the current engine at all**. Do not read flat depth as 'SMP is worthless'. Also note 16T returns are already diminishing on 16 physical cores (7.88x → 12.31x for a doubling), which is a bandwidth signature rather than a search one. | `tools/nps_scaling.ps1`; `tools/diag_smp_sweep.ps1`; Plan 4.9 |
| RAR-R08 | Phase-4.9b: measure the cutoff-USABLE share of TT hits against thread count, the lead RAR-R07 left. `tt_bound_not_usable` was first SPLIT by cause (shallow / PV / excluded / wrong-window), because it lumped three unrelated reasons and only one of them can grow with threads. Same sweep as RAR-R07: diag build, 1/2/4/8/16T, 2 positions x 3 reps, `movetime 3000`. | **The hypothesis is REFUTED — TT hits get BETTER with threads, and depth still does not move.** Usable share rises 11.0 → 11.0 → 12.4 → 13.5 → **15.6%** and shallow rejections FALL 74.1 → 73.8 → 70.5 → 69.8 → **67.0%**, the opposite of the prediction. Combined with the rising hit rate this is **usable cutoffs per probe 5.63% → 9.89%, +76%**. Wrong-window rejections rise only 8.3 → 10.8%; pv/excluded is flat at 6.4–7.9%; the shallow deficit is **thread-independent at 2.4–2.5 plies**. So ALL FOUR recorded hypotheses are now dead. The per-thread depths say why: at 16T they read 22,21,20,22,22,22,21,21,21,22,20,22,21,22,22,22 — **no thread ever exceeds the depth 1T reaches alone**. `thread_id` seeds only the LMR jitter and the root-move rotation; the iteration loop `for depth in 1..=max_depth` is IDENTICAL on every thread, so there is **no depth staggering** and no thread is ever ahead to populate the table with entries the others could skip to. | **The pool is diversified in WIDTH and not at all in DEPTH, and that is a missing mechanism rather than a tuning problem.** Every thread's TT hits come from its own previous iteration, which is exactly why the shallow deficit sits at ~2.4 plies regardless of thread count and why 67–74% of hits cannot cut — a structurally normal figure for iterative deepening, and NOT a defect, which is the second thing this measurement corrects. The unrealized headroom is concrete: at EBF 2.449, 10.34x the nodes is **2.61 plies** of theory at 16T (2.27 at 8T, 1.48 at 4T) and the realized gain is **0**. ⚠ PLAN 4.9 forbids reopening worker diversification 'without a specific measured independent-work failure' — this is that measurement, so the guard is satisfied rather than bypassed. Iteration staggering is a Threads>1 behaviour change and needs a 4T/8T gate; it must not be landed on this profile alone. | `src/search.rs`; `src/diag.rs`; `tools/diag_smp_sweep.ps1`; Plan 4.9b |
| RAR-R09 | Phase-4.9c: implement per-thread ITERATION STAGGERING — the depth-diversity mechanism RAR-R08 showed is absent — behind `SmpIterationSkip`, then try to size it locally. Classic Lazy-SMP skip tables, helpers only, main thread never skips. Same binary both arms, option toggled, so there is zero build variance. | **Landed inert and correct; LOCAL SIZING IS IMPOSSIBLE, and a null pair proves it rather than asserting it.** Bench 6,502,902 unchanged (inert at 1T twice over: no shared state, and thread 0 never skips). A property test pins that main never skips, no helper skips everything or nothing, and helpers disagree at every depth. Then the sizing failed, in both metrics. Depth at fixed `movetime 5000`, n=2 per cell: **−0.5 / 0.0 / 0.0 plies** at 4/8/16T on an integer metric whose own spread is ±1–2. Time-to-depth(20), n=6 per cell: ratios **1.33 / 0.63 / 1.20** — no coherent direction. **The null pair at 8T, both arms `skip=0`, returned medians 742 versus 902 ms, a 21.6% swing from nothing**, with 1207/1997/1531 ms inside a single cell. Every skip ratio sits inside that. NPS rose +3.6% at 8T and +4.6% at 16T with skipping on, but NPS is not what this mechanism is for. | **Run the null BEFORE believing the arm, not after.** Without it this would have been written up as '+37% time-to-depth at 8T' and '−33% at 4T' — both pure noise, and the first is exactly the sort of number that gets a mechanism adopted. The metric is unusable at this rep count: SMP time-to-depth carries ≥20% variance at n=6, so resolving a 5% effect needs order-100 samples per cell, and depth-at-fixed-time is too coarse to resolve half a ply at all. ⚠ So there is a strong MECHANISTIC case (RAR-R08: 2.61 plies unrealized at 16T, no thread ever ahead) and **zero local evidence the fix captures any of it**. Diversification is also 0 for 2 in this engine (RAR-R06, 9.7.5(j) at −5.54). Spending a 4T/8T gate on it is a judgement call, not a formality — and a properly powered local run (order-40 reps per cell at a shallower depth) is far cheaper than the gate and should come first. | `src/search.rs`; `src/params.rs`; Plan 4.9c |
| RAR-R10 | Phase-4.9c-i: the powered sizing RAR-R09 said was owed. Redesigned harness — ONE engine process per cell (no process-start variance), arms ALTERNATING inside it, `ucinewgame` before every rep, 40 pairs x 2 positions per thread count at `go depth 18`, analysed per-pair rather than as independent medians. Bootstrap CIs (20k) and a permutation test against an n=80 null run under the identical protocol. | **The harness now works and the answer is NO BENEFIT WHERE IT COUNTS.** The null validates the design: median per-pair ratio **exactly 1.0000**, 39/80 wins, p=0.91 — against the 21.6% swing the n=6 cross-process attempt produced. Against that baseline: **4T time 1.067, nodes 1.28/0.97; 16T time 1.065, nodes 1.00/0.95 — no benefit, if anything harm.** 8T is the only positive cell: time **0.868** (95% CI 0.766–1.013, permutation p=0.168, NOT significant) and nodes **0.845** (CI 0.740–0.944, p=**0.0493**, borderline). Sign tests are null in every cell (46/80 at 8T, p=0.22). | **The effect has the wrong SHAPE across thread counts.** Depth diversity should deepen monotonically — more threads on one iteration means more to gain from spreading them — so 16T should be strongest. Nodes ratios run **1.046 → 0.845 → 0.989** at 4/8/16T, a V rather than a trend, with 16T's CI [0.872, 1.190] straddling 1.0. The lone signal is the middle cell, significant on the quieter metric only and borderline there (p=0.049 uncorrected over six cells and two metrics). ⚠ A first draft argued instead from 'the gate runs at 4T and 4T shows nothing' — the WRONG criterion, since the objective is good scaling with threads rather than a merely workable 4T, so a real 8T or 16T gain would have counted regardless of where a gate is defined. The non-monotone shape refutes the mechanism under either objective; the gate-condition argument would not have. `SmpIterationSkip` therefore stays INERT and no gate is spent — the mechanistic case from RAR-R08 remains true and unconverted. **The harness lesson is the durable part:** pairing WITHIN one process with interleaved arms turned an unmeasurable quantity into one whose null centres on 1.0000, and the null is what licensed reading the arm at all. Nodes-to-depth is also the better metric here — same direction as time, roughly half the spread. | `src/params.rs`; Plan 4.9c-i |

## 5. Evaluation and data experiments

The historical HCE freeze ended with the 2026-08-30 plan reconciliation.
Current Phase-4 steps 4.7–4.10 qualify the data and every fitting instrument,
refit the complete existing surface, then add structure only where residuals
support it. These rows remain relevant to that analysis and to NNUE data,
teacher and measurement design, but they do not authorize retries unchanged or
make any historical parameter group exempt from the current audit and gate.

| ID | Experiment and conditions | Result / disposition | Conditional lesson and retry trigger | Source |
|---|---|---|---|---|
| RAR-E01 | Staged Texel fit over 2.19M self-play positions: king safety, threats, mobility, scalars, imbalance, material/PST and polish. | Every stage accepted; about +316 staged self-play and +240 externally over 2.1.0. | Sequential fitting worked strongly on that corpus, but only about 75% of the staged gain transferred to the external cohort. | `CHANGELOG.md` 2.2.0; legacy plan |
| RAR-E02 | Lazy HCE shortcut after the evaluator expansion. | **Accepted, about +4.4 Elo.** | The shortcut paid under that HCE distribution, but the safety margin is representation/scale dependent and is not a NNUE constant. | legacy plan at `757e9a3^` |
| RAR-E03 | Stockfish-at-60k off-policy distillation with material scale pinned. | **Rejected, −17.11 Elo,** despite 4.9% lower holdout loss and 9/10 improved buckets. | For this well-fitted HCE/corpus, lower teacher-fit loss did not predict play. Basilisk's +6.75 opposite result reinforces that transfer is engine-state dependent. | legacy plan; `analysis/hce_analysis.md` |
| RAR-E04 | 500k-game on-policy refresh yielding 2.18M unique positions; pure WDL beat blended labels on the shared holdout. | **Rejected, −1.28 ± 2.79 over 26.8k games;** pipeline and inert parameters retained. | Even on-policy lower validation loss did not improve this unchanged representation. Retry only after representation/policy changes and with a frozen external holdout. | legacy plan at `757e9a3^` |
| RAR-E05 | Narrow L2-anchored refresh from a stronger label generator, moving 57/1,204 parameters mostly by 1 cp. | **Accepted, +11.56 ± 5.19 Elo;** frozen in 2.3.2 and throughout the completed Phase-4 search track. | A narrow anchored refresh differed materially from wholesale re-derivation. The current complete-surface programme may supersede it only through its qualified, baked and game-gated fit; the result is neither a frozen-group exemption nor evidence that an undisciplined general refit will work. | `PLAN.md` §3, §4; legacy plan |
| RAR-E06 | Complete 1,218-slot current-HCE WDL refit, including traced linear coefficients and nonlinear king danger, confirmed on a fresh untouched self-play set. | **ACCEPTED.** H1 at 3,914 games: **+22.04 +/- 7.51 Elo, +32.05 +/- 10.88 nElo**, LOS 100.00%, LLR 2.95 crossing +2.94 in 44 minutes. The complete refit is the accepted HCE head at **7,226,051 / 2.460**. | A whole-surface recalibration of an unfitted HCE was worth more than eight times the bracket it was gated against, and paid for a measured -1.19% NPS. Offline loss still predicted neither sign nor magnitude: -0.63% test loss preceded +32 nElo, while Basilisk's -6.2% preceded -77.92 Elo. | verdict below; `analysis/endgame_conversion_audit_2026-09-01.md` |
| RAR-E07 | **4.8a redundancy inventory on the accepted vector.** Cross-referenced `04-final.txt` (SHA `BAD51F3E...`, the accepted fit) against the source vector and the run's own per-slot `feature-support.log`. Zero games, zero compute -- every input already existed. | **Closed without a gate; nothing to remove.** The fit drove only **5 of 1,218** slots to zero, of which 3 are whole 1-slot terms (`passed_candidate_mg`, `passed_freestop_eg_per_rank`, `threat_safe_pawn_push_eg`); it also switched **17** previously-zero slots back on. Of the 132 slots under the sparse cut, **90 are structurally unreachable** (0 activations), 12 are the nonlinear danger selectors, 12 are co-tuned safety-table entries, and the remaining 18 are rare-but-real and **all 18 held**. | Basilisk's BAS-E25 removed sixteen terms that a previous phase had **added**; Rarog's existing surface has no equivalent accumulation, so the analogue does not transfer. Two instrument confirmations fell out of this for free: the 12 zero-linear-activation fields are exactly `KS_DANGER_INPUTS`, independently reproducing 4.7.3's 1,194+12+10+2 partition from a different artifact, and the fitter froze every under-supported coefficient instead of fitting noise into it. **Unreachable is not redundant** -- a pawn PST is 64 entries because the index space is 64, and 16 of them are ranks 1 and 8. | `tools/results/hce-fit-20260831_095443/{04-final.txt,feature-support.log}`; `tools/results/hce-confirm-20260831_230548/source-vector.txt` |
| RAR-E08 | **Self-play labels versus tablebase-corrected labels on <=6-man positions. ACCEPTED.** H1 at 13,432 games: **+6.73 +/- 3.82 Elo, +10.34 +/- 5.88 nElo**, LOS 99.97%, LLR 2.95, 2h30m, **zero time forfeits**. One game set, two label sets: arm A keeps the literal self-play WDL everywhere; arm B rewrites the label of every position with 6 men or fewer to its Syzygy value, fifty-move rule kept, and changes nothing else. | **Registered; screen complete.** `hce-v2` carries **233,143 of 2,300,000 train rows at <=6 men (10.14%)**, of which **70.9% are labelled draw**. On a 20,953-row sample the tablebase disagrees with the self-play label on **13.27%** of them, so **about 1.35% of all training rows would be rewritten**. The disagreement is not one-directional: 1,506 sampled rows drew in self-play but are theoretically decisive, 1,253 were decisive but are theoretically drawn, and 22 are outright win/loss reversals. | **Which label is 'correct' is genuinely open, which is why this is an experiment and not a decision.** Texel fits the value realizable by the CONSUMING SEARCH -- Basilisk priced borrowed Stockfish labels at **-7.30 +/- 4.76**, the worst arm it ran -- and under that principle a KBN-K position Rarog converts 7% of the time really is a draw, so the self-play label is right and the tablebase one teaches the engine to enter endgames it cannot win. Against that, self-play labels are self-reinforcing: cannot convert -> labelled draw -> evaluator learns draw -> never steers there -> never learns to convert. **The arms cannot be compared by offline loss**, because their targets differ and a loss measured against different targets is not a comparison; only a head-to-head game result decides. | this registration; RAR-M18; `analysis/basilisk_audit_2026-08-30.md`; 4.10 |
| RAR-E09 | **4.9.1 post-fit residual audit of the accepted HCE.** `--report-endgames` over the accepted vector at the fit's pinned K=1.37011, on the 127,778 published-but-unused confirmation positions -- never fitted on, never selected anything, not the frozen test. Global loss 0.12241022 over 299 material classes. | **Closed: no 4.9 entry evidence found, and a label defect found instead.** The largest residual is **KR-K**: 379 positions, **284 (75%) labelled a draw** in a class that is a 100% theoretical win. The evaluator predicts **0.849** there against a label mean of 0.625 -- **it is closer to the truth than its own training data**. Mechanism confirmed directly: Rarog scores a won KR-K at **+426 cornered / +487 centralised**, while `datagen-v1`'s resign rule needs 600 cp from both engines for three moves, so it never fires; the game plays on at 8,000 nodes, fails to mate inside fifty moves, and is labelled 0.5. | **A residual the surface can represent is not structural evidence.** The surface would price KR-K correctly if the labels said 1.0, so this licenses no structural work -- 4.9 closes on it. It is strong prior evidence for RAR-E08 arm B, and it names the mechanism behind RAR-M18's 13.27% disagreement: not random, but concentrated in classes the engine scores BELOW the resign threshold while being theoretically won. It also qualifies the endgame audit's drawn-subset overconfidence -- on KR-K that overconfidence is the evaluator being right. Above 6 men the same question cannot be answered locally; do not assume it generalizes. | `analysis/hce_residuals_2026-09-01.md`; `tools/results/hce-accepted/residual-endgames-accepted.csv`; RAR-M18; RAR-E08 |
| RAR-E10 | **4.9a.4 minor-piece mate drive. ACCEPTED 2026-09-01 on maintainer judgement, with NO game gate.** The drive used Chebyshev corner distance, which is flat: over 300 won KBNK positions, 19 legal moves collapsed into a median of 3 distinct scores and **94% had a TIED best move**, median gap 0 cp. Replaced by a diagonal pull (`abs(7 - rank - file)`, mirrored for the dark corners) dominating the king-proximity terms, scoped to minor-piece bare-king mates. | **KBN-K 19.4% -> 96.9%, KBB-K 78.0% -> 100.0%** on the Syzygy truth corpus, paired position-for-position. `bench 13` **unchanged at 7,226,051 / 2.460**. The original claim that 15 of 19 families were exactly equal is **SUPERSEDED by RAR-M23**: six families change through the dispatcher's promotion closure, including net -1 conversion debt in KBP-KB and KBP-KN. KBN-K's failure mode changed rather than shrinking: 61 fifty-move losses became **zero**, and the residue is 4 positions where the engine gives away the bishop or knight. Floors ratcheted. | **Three axes were needed and the third was nearly missed.** Resolution (a flat metric cannot order its own moves; 40x a 0 cp gap is still 0), magnitude (32.7% -> 57.1% from doubling), and RATIO (the corner pull must dominate, not merely exceed, the king pull). The diagonal shape was tested first at ~1:1 against the king terms, measured WORSE than what it replaced, and recorded as non-transferring; at ~6:1 it is the whole gain. **Sweeping a mechanism's shape while holding its proportions fixed can refute the mechanism for the wrong reason.** Also: this change is bench-INVISIBLE but behaviour-changing, so `bench 13` cannot identify a build carrying it. **Acceptance departs from the stated rule that only a registered SPRT accepts a candidate, and is recorded as a judgement call rather than a gate.** What justified it: bench byte-identical, activation triply gated (`|eval| > 200` AND a bare losing king AND no pawn/rook/queen for the winner), the then-recorded isolation argument, hard theory vetoes and floors passing, and a tier-3 occurrence of 0.28% at which a `[0,3]` gate cannot resolve anything at any budget this project has. What that does NOT establish: bench-identical proves only that 40 bench positions' trees never reach a minor-piece bare-king mate within depth 13, while real games at 3+0.03 reach greater depth with endgames on the board and do fire the term in roughly 1.6% of games. **Retry trigger: any endgame-shaped strength anomaly reopens this without needing new argument.** | `analysis/endgame_conversion_audit_2026-09-01.md`; `tools/results/mopup-diag/endgame-truth.json`; 4.9a.4 |
| RAR-E11 | **SUPERSEDED IN FULL by RAR-E14/RAR-M24.** The corrected v2 reference rerun is **1361/1372 = 0.9920**, the current head is **1276/1372 = 0.9300**, and the reference is worse in **no** family. The historical v1 figures below are retained only as invalid history; both original arms were contaminated and the reference arm had to be rerun, not re-analysed. **Stockfish 18 measured on the identical truth corpus**, same 100 positions per family, same 60,000 nodes, `SyzygyPath` cleared so it evaluates rather than reads the answer. Modern Stockfish has no endgame dispatcher at all, so this measures achievable conversion at a node budget, not a rival mechanism. | **Stockfish does NOT convert everything: 90.2% weighted, not 100%.** It is below 100% in seven families and **worse than Rarog in three** -- KPP-K 75.5% against 76.5%, KBP-K 93.6% against 97.9%, KBP-KB tied at 69.2%. Weighted totals: SF **90.2%**, Rarog before 4.9a.4 **76.1%**, Rarog after **83.2%**. The mate-drive work closed half the total gap, and closed KBN-K's from -80.6 pp to **-3.1 pp**. | **Conversion targets must be measured against what is ACHIEVABLE at the budget, not against 100%.** Several apparent defects are node-budget limits: KRP-KR looked like a 52%-conversion failure, but Stockfish manages only 47.9% there, so Rarog's 43.8% is 4.1 pp off the reachable mark rather than 56 pp off a perfect one. The real gaps are where SF reaches 100% and Rarog does not: KQ-KR -25.0, KR-KP -16.3, KRP-KB -11.5, KR-KN -11.1, KR-KB -10.7, KR-K -7.0. Weighted by RAR-M15 occurrence the ranking is KR-K, KQ-K, KRP-KR, KR-KP -- so the elementary rook and queen mates, not the exotic families, carry the most recoverable value. | `tools/results/reference-sf18/endgame-truth.json`; RAR-M15; RAR-E10 |
| RAR-E12 | **Complete HCE refit on `hce-v3-tb`** -- the 4.9a.6 corpus: 602,619 non-adjudicated games from the phase-weighted `phase_book_v1.epd`, 3,500,000 train rows, <=6-man labels Syzygy-corrected. Gated against the accepted RAR-E08 head. | **H1 ACCEPTED at 7,388 games: +11.81 +/- 5.33 Elo, +17.57 +/- 7.92 nElo**, LOS 100.00%, LLR 2.95, 1h23m, 4 time forfeits (0.054%, under the ceiling). **ADOPTED 2026-09-03 at `d1d95ab`, with the KBN-K breach waived to an owner.** The registered disposition required a repair or a recorded waiver. A partial revert of `king_safety_table` was built and measured as RAR-E13, then WITHDRAWN: the 1,218 slots are jointly fitted, so reverting one block leaves the compensations the other slots made for it in place, producing a vector that is the optimum of nothing -- and it would have become the seed for every 4.10 refit cycle. The waiver instead assigns KBN-K to **4.9a.26**, which is open and already scoped as "gradient owned by 4.9a.4; verify and close", with **0.7260** as its target. This mirrors RAR-E08 handing KQ-KP's -3.8 pp to 4.9a.14. Verified before adoption: 286 debug and 264 release tests pass including the 64 frozen theory vetoes, clippy `--all-features --all-targets` clean, and a rebuild with the exact feature set reproduces 8,044,078 / 2.481. Offline: frozen test improved by **0.00042745** within its own corpus (0.08884903 -> 0.08842158), best validation epoch 22 of 60 on a flat curve. **Conversion side-note SUPERSEDED by RAR-M24.** Historical v1 aggregate conversion was **0.8345 -> 0.8477**. The matched corrected v2 pair is **1254/1372 = 0.9140 -> 1278/1372 = 0.9315**. KP-K, KQ-K and KQ-KP DTZ progress improve in the v2 pair, but **KQ-KP conversion is 96/98 -> 94/98**; the prior statement that its RAR-E08 conversion debt was repaid was overbroad. | **Two disclosed defects, registered before any games so neither can be discovered afterwards.** (1) `bench 13` **7,165,683 -> 8,044,078, +12.3%**, EBF 2.462 -> 2.481 -- far larger than RAR-E06's +3.6% or RAR-E08's -0.8%. Search pruning margins are calibrated to the old eval scale and PROCESS rule 10 defers remeasuring them to 4.11, so **this gate measures eval gain MINUS search-efficiency loss** and can land negative while the evaluation itself is better. (2) The endgame floors **FAIL**: KBN-K dtz progress **0.7260 -> 0.6753, -4.4 SE**, in the family 4.9a.4 fixed. Conversion there is 0.9184 -> 0.8980 (-2 of 98, not significant) and win-preservation improved to 0.9994, so it still mates and never discards a win -- it takes longer routes (3,178 graded moves against 2,989). The mate-drive constants are NOT in the Texel surface and the patch does not touch them; the only changed terms that fire against a bare king are king-safety ones, `king_safety_table` having lifted 414 -> 515 at the top and 316 -> 374 mid-band. **That mechanism is plausible and UNVERIFIED.** | this registration; `analysis/artifacts/rar-e12-{final-vector.txt,candidate-eval.patch}`; `tools/results/hce-fit-20260902_221030` |
| RAR-E14 | **Audit of the endgame truth instrument, 2026-09-04, prompted by Basilisk BAS-E47/BAS-E50 and verified independently against Rarog's own code and artifacts.** `endgame_truth.py` ended a playout the moment the strong side's piece count dropped. Zero games; the evidence is ten existing artifacts plus a regeneration of the position set from the current code. | **Three confirmed defects.** (A) The material abort fired **264 times** on the RAR-E08 arm: 129 on clean wins and **122 of those before the engine had played a single non-win-preserving move**, at a median abort ply of 5-20. Aggregate conversion **0.8345 has a corrected upper bound of 0.9235**; KRP-KR 36/73 bounds at 69/73 and KRP-KB 45/96 at 91/96. The reference arm carries 258 aborts and was run without `--per-position`, so it cannot be re-analysed, only re-run. (B) `b9cc252` changed which positions the harness generates: three artifacts, **including `hce-accepted`, the one PLAN cited as the baseline**, share **zero of 1,900 positions** with the current generator, and 4.9a.7 compared one of them against the reference arm from the other set. (C) The truth run behind the current `endgame_floors.json` exists nowhere -- `tools/results/` is gitignored -- so 4.9a.26's 0.7260 target has no reproducible artifact. | **Bare-king families are provably isolated, which makes RAR-E10 safe rather than merely lucky:** in all six, any strong-side material loss reaches an insufficient-material position, tested one line earlier, so the abort is unreachable there by construction -- and zero `material_lost` outcomes appear in those families across all ten artifacts. **What RAR-E10 does lose is its isolation ARGUMENT.** It recorded "15 of 19 families exactly unchanged"; per-position comparison of the same two artifacts gives **13 of 19**, and the drive reaches **KBB-K, KBN-K, KPP-K, KBP-K, KBP-KB and KBP-KN**, the last two each losing one conversion. The route is knight promotion, which manufactures exactly the material the dispatcher keys on: **a term's blast radius is its dispatcher condition's PROMOTION CLOSURE, not the condition.** RAR-E11 is superseded in full and RAR-E08's and RAR-E12's conversion side-notes with it; both Elo verdicts stand, because fastchess played those games and this harness never touched them. Repair at 4.10, re-measurement at 4.11. | `analysis/endgame_truth_instrument_audit_2026-09-04.md`; `9281435`; PLAN "Reopened work, 2026-09-04" |

### RAR-E13 — is the fitted king-safety table worth its tree cost? (registered 2026-09-03, before games)

- **WITHDRAWN 2026-09-03, unresolved, before the boundary.** Stopped at roughly
  a few thousand games with arm B leading by **about +7 Elo**. That number is
  **an observation, not evidence**: AGENTS.md's rule that an unresolved stop is
  not "probably fine" applies exactly here, and RAR-S61 is the standing example
  -- +4.50 +/- 3.50 at LOS 99.41% that turned out to be a stale-read bug.
- **Why it was withdrawn rather than finished.** Arm B could not be adopted
  whatever it measured. The 1,218 slots were fitted JOINTLY, so other terms
  carry compensations for the inflated `king_safety_table`; reverting that one
  block leaves those compensations uncorrected and yields a vector that is the
  optimum of neither the free fit nor a constrained one. PROCESS rule 5 already
  requires inspecting post-fit compensation for materially moved families. The
  same objection is why a subset of a converged SPSA vector is invalid.
  Adopting B would also have seeded every 4.10 refit cycle from a hand-edited
  point.
- **What the +7 is a hypothesis ABOUT, for 4.11.** It is not a speed effect --
  both binaries are the same code at the same nodes/second. It is tree size:
  arm A needs 8,044,078 nodes to reach depth 13 against arm B's 6,972,274, so
  arm A searches shallower at a fixed clock. **These two arms cannot separate
  "worse evaluation" from "evaluation mistuned against margins calibrated for
  the old scale"**, because in arm B those are the same change. 4.11
  recalibrates the eval-coupled margins -- reverse futility, razoring, ProbCut,
  null-move scaling, SEE pruning, aspiration -- and however much of arm A's
  penalty disappears was miscalibration. If most of it goes, arm A with retuned
  margins should beat both. If little goes, the fitted table does not earn its
  cost and 4.10's next cycle should CONSTRAIN it in the fit rather than have
  anyone revert it by hand.

- **Question.** RAR-E12's candidate grew `bench 13` by 12.3% and broke the
  KBN-K endgame floor. An ablation identified **one block as the whole cause**:
  reverting `king_safety_table` to its previously accepted values takes bench
  from 8,044,078 to **6,972,274** -- *below* the current head's 7,165,683 --
  and lifts KBN-K conversion from 0.8980 to **0.9490, above its 0.9184 floor**,
  demoting the dtz breach from blocking (-4.4 SE) to report tier (-2.2 SE).
  So: is the fitted table's middlegame value worth the search efficiency and
  endgame precision it costs?
- **Why the fit inflated it.** `hce-v3` carries **367,664 natural mates against
  `hce-v2`'s 6,428**, a 57x increase, because dropping adjudication stopped
  resigning games out. Most are king hunts, so the corpus contains far more
  evidence that attacking the king pays. **Texel's objective cannot see that a
  more volatile evaluation costs 12% of the search tree** -- it prices label
  agreement, not nodes-to-depth. This is precisely the blind spot PROCESS rule
  10 anticipates when it keeps search parameters fixed during fitting.
- **Arms.** Identical vectors except one block. Arm A is RAR-E12's candidate,
  `rarog-e09cand-pext-pgo.exe`, bench 8,044,078 / 2.481. Arm B is
  `rarog-e09noks-pext-pgo.exe`, bench **6,972,274 / 2.466**, built from
  `d306e21` plus `analysis/artifacts/rar-e13-candidate-eval.patch` (SHA-256
  `7ADC9C44...`) -- the same fit with `king_safety_table` alone restored to the
  accepted head's 40 values. Both binaries were built from a dirty tree, so
  both patches are committed; a rebuild must reproduce those fingerprints.
- **Registered gate.** Arm B versus arm A, **`[-5,5]` nElo**, alpha = beta =
  0.05, cap **30,000 games**, `3+0.03`, Threads 1, Hash 64 MB, concurrency 14,
  paired UHO random order, no adjudication. **The bracket is symmetric because
  the sign is genuinely unknown** -- the table is a fitted quantity and may
  carry real middlegame value that outweighs its tree cost. AGENTS.md's
  asymmetric `[0,3]` would presume B must earn the change; here neither arm is
  the status quo, since arm A is itself unadopted. RAR-S62 resolved a symmetric
  `[-5,5]` in 4,436 games.
- **Stop/disposition.** B wins or ties within the bracket -> **adopt B**, which
  banks RAR-E12's evaluation gain with a smaller tree than the current head and
  a passing conversion floor. A wins -> the inflated table earns its cost, and
  adopting A then requires the recorded waiver RAR-E12 called for, with KBN-K
  assigned an owner and a retry trigger. Either way the **+11.81 Elo of
  RAR-E12 is not re-litigated**; this gate only decides which of two vectors
  carries it.
- **What this gate does NOT settle.** KBN-K occurs in roughly 0.28% of games
  (RAR-E10), so neither this gate nor RAR-E12 can measure that family
  directly. The floors instrument exists because the strength gate cannot see
  rare endgames, and a passing SPRT is not evidence that a floors breach is
  harmless.

### RAR-E12 — hce-v3 complete refit gate (registered 2026-09-03, before games)

- **Question.** Does a complete 1,218-slot refit on the 4.9a.6 corpus beat the
  accepted RAR-E08 head in games? The corpus changed in three ways at once --
  row count (2,300,000 -> 3,500,000), phase mix (balanced book -> phase-weighted
  book), and label provenance (52.2% adjudicated -> 0.007%, 6,428 natural mates
  -> 367,664). **This gate prices the combination and cannot attribute the
  result to any one of them.** That is a deliberate cluster under the strength
  rule, not an oversight, and it is written here so no post-hoc attribution can
  be made later.
- **Baseline.** `tools/test_engines/rarog-e08head-pext-pgo.exe`, git `a52f4d2`,
  clean tree, bench **7,165,683 / 2.462**.
- **Candidate.** `tools/test_engines/rarog-e09cand-pext-pgo.exe`, bench
  **8,044,078 / 2.481**. Built from `d306e21` plus
  `analysis/artifacts/rar-e12-candidate-eval.patch` (SHA-256 `0A7187F8...`),
  which is the diff produced by baking
  `analysis/artifacts/rar-e12-final-vector.txt` (SHA-256 `EA932B46...`) with
  `tools/texel/bake_params.py`. **The binary was built from a dirty tree and is
  therefore not reproducible from a git SHA alone** -- the patch and vector are
  committed here precisely so the recipe does not dangle, which is the RAR-S54
  failure. A rebuild must reproduce 8,044,078 / 2.481.
- **Corpus.** `hce-v3-tb`, manifest SHA-256 `07BD88CD...`, 602,619 independent
  starts, `datagen-v2`, book `phase_book_v1.epd` SHA-256 `31E9B655...`, K pinned
  at 1.36439, frozen test opened exactly once.
- **Registered gate.** Candidate versus baseline, **`[0,3]` nElo**, alpha = beta
  = 0.05, cap **80,000 games**, `3+0.03`, Threads 1, Hash 64 MB, concurrency 14,
  paired UHO random order, **no adjudication**. The bracket is the project
  default and must be passed explicitly: `-Mode gainer` defaults to `[3,10]`,
  which AGENTS.md records would drive a true +4 to H0.
- **Stop/disposition.** H1 does **not** by itself accept the candidate. The
  KBN-K floors breach is a blocking-tier failure under a threshold set
  prospectively, so acceptance additionally requires either a repair that
  restores the floor, or an explicit waiver recorded with its reason. H0, or a
  cap-out, closes the candidate and makes the +12.3% bench the first suspect --
  in which case 4.11's search-parameter remeasurement becomes a prerequisite of
  a refit rather than a follow-up to one.

### RAR-E08 — label-contract gate (registered 2026-09-02, before games)

- **Question.** Should a Texel fit learn the literal self-play result on
  positions the tablebase can adjudicate, or the tablebase's verdict? Texel
  fits the value realizable by the consuming search, which argues for self-play
  labels; against that, self-play labels are self-reinforcing, and RAR-E09
  found the mechanism concretely -- KR-K, a 100% theoretical win, is labelled a
  draw on 75% of its `hce-v2` positions.
- **Arms.** One game set, two label sets, differing in exactly one way.
  Arm A is `hce-v2` and the accepted head's vector. Arm B is
  `hce-v2-tb` -- byte-identical rows, FENs, order and split membership, with
  only <=6-man labels replaced by Syzygy truth and cursed wins counted as
  draws. **30,480 train labels changed, 1.325% of rows.**
- **Arm B fit.** `hce-fit-20260902_094603`, started from the accepted vector
  (source SHA-256 `BAD51F3E...`, verified equal to RAR-E06's final vector).
  Final vector SHA-256
  `6BCD3AB015C410ECDE77E2ABA6BA87C14AAB6A189CD5F5389F29F082C1C18B91`,
  K pinned at 1.3806, frozen test opened once. **350 of 1,218 slots differ from
  arm A.** Candidate fingerprint **7,165,683 / 2.462** against the accepted
  head's 7,226,051 / 2.460.
- **The offline losses are NOT comparable and are recorded only as within-arm
  numbers.** Arm B improved its own frozen test by **0.000181**
  (0.11598764 -> 0.11580664); RAR-E06 improved its own by 0.00078088. The
  targets differ, so a loss measured against different targets is not a
  comparison. That arm B's improvement is the smaller of the two is consistent
  with RAR-E09 -- the accepted vector was already closer to tablebase truth
  than its own labels were -- but it is an observation, not evidence for
  either arm.
- **Registered gate.** Arm B versus arm A, **`[0,3]` nElo**, alpha = beta =
  0.05, cap **80,000 games**, `3+0.03`, Threads 1, Hash 64 MB, concurrency 14,
  paired UHO random order, **no adjudication** (the harness default since
  RAR-M17). The bracket is the project default rather than a symmetric one
  because the decision is asymmetric: self-play labels are the status quo, and
  arm B must earn the switch. A symmetric `[-5,5]` would also have no preferred
  outcome at a true zero and would run to the cap.
- **Stop/disposition.** Only H1 adopts tablebase-corrected labels as the
  contract for 4.9a.6's regeneration and every later fit. H0, or reaching the
  cap without H1, keeps self-play labels -- which is the cheaper status quo and
  a legitimate result, not a failure. No offline loss, LOS or post-hoc interval
  substitutes for the boundary.

#### RAR-E08 verdict — ACCEPTED 2026-09-02

- **Result.** H1 at **13,432 games**: W 3668 / L 3408 / D 6356, 50.97%.
  **Elo +6.73 +/- 3.82, nElo +10.34 +/- 5.88**, LOS 99.97%, DrawRatio 41.60%,
  PairsRatio 1.12, Ptnml(0-2) [273, 1580, 2794, 1752, 317], LLR 2.95 against
  +2.94. Wall time 2h30m. **Zero time forfeits in 13,432 games**, no
  adjudication, manifest complete.
- **Prediction check.** RAR-M10 projected the boundary at ~17,000 games from
  the +8.45 nElo reading at 4,842; it resolved at 13,432 with the estimate
  firming to +10.34. The model was right about the shape and slightly
  conservative about the pace.
- **Artifacts.** `sprt_E08TbLabels_vs_E08SelfPlay_20260902_100039.*`, PGN
  SHA-256 `BCEF730E54A07382C6759D3AFA2826FC2566A17FF5EB94304F3005D3F7401273`,
  log SHA-256
  `E9ACB84627B17CF4D9CF0CE39EA6234B77131BBD7C000BA6C8F7BDE9849A0511`, seed
  260902. Arm B fit `hce-fit-20260902_094603`, final vector SHA-256
  `6BCD3AB015C410ECDE77E2ABA6BA87C14AAB6A189CD5F5389F29F082C1C18B91`.
- **What is adopted, precisely.** The **post-hoc relabel** of positions with 6
  men or fewer, cursed wins counted as draws, applied to an otherwise unchanged
  corpus -- that is `tools/texel/relabel_tb.py`, and it is what won. It is
  **not** `datagen-v3`, which adjudicates the GAME on tablebase truth and
  therefore changes the recorded result of every position sampled from that
  game, including opening and middlegame ones. `datagen-v3` remains untested;
  adopting it because "tablebase labels won" would be adopting a different
  change.
- **What this says about the labels.** Texel fits the value realizable by the
  consuming search, and that principle predicted arm A would win. It lost by
  10.34 nElo. The self-reinforcing loop was the stronger effect: RAR-E09
  measured KR-K, a 100% theoretical win, labelled a draw on 75% of its corpus
  positions, with the evaluator already predicting 0.849 against a label mean
  of 0.625. Correcting 1.325% of rows was worth +6.73 Elo.
- **Conversion cost, resolved at n=400.** The endgame floors failed at n=100 on
  four of 57 comparisons. Re-measuring both binaries on 400 paired positions
  says most of that was sampling: **KBN-K 95.0% -> 95.5%** (+0.5 pp, SE 1.5 --
  the -5.1 pp reading was noise, in the very family 4.9a.4 had just fixed),
  KNN-KP -7.8 pp at 1.4 SE, KP-KP -3.0 pp at 1.25 SE. One is real:
  **KQ-KP 98.5% -> 94.7%**, -3.8 pp at 2.9 SE. That is queen versus pawn, where
  coverage is a narrow fortress partial and occurrence is 1.17% of games, so
  the expected-value cost is about 0.04 against a +6.73 Elo gain. Owner
  **4.9a.14**; retry trigger is the refit at 4.9a.27. Aggregate weighted
  conversion is flat: 83.24% -> 83.45%.
- **SUPERSEDED and corrected by RAR-M24, 2026-09-06.** The preceding v1
  aggregate is invalid. The matched v2 full cohort is **1255/1372 = 0.9147 ->
  1254/1372 = 0.9140**. The matched v2 400-position focus rerun reproduces
  every preceding family result, including **KQ-KP 390/396 -> 375/396, -15,
  -3.79 pp**. The KQ-KP conversion debt therefore survives as historical
  causal evidence, but RAR-M21 establishes that the current 60k shortfall
  closes at 200k and 600k. The old text remains above as history.
- **Which metric caught it.** The n=100 floors flagged KQ-KP on
  **DTZ-progress**, and the n=400 conversion re-measure then confirmed a real
  regression there -- while the conversion flag on KBN-K was a false positive.
  DTZ progress was the leading indicator, which is an argument for keeping it
  in the floors rather than reducing them to conversion rate.
- **Scope limitation, recorded at disposition.** `hce-v2` was generated under
  `datagen-v1`, where 52.2% of games ended by adjudication and 98% of decisive
  results were called by the resign rule. Arm B corrects only <=6-man labels,
  so every position above 6 men still carries a truncated result. This gate
  therefore establishes that TB correction pays **on an adjudicated corpus**;
  4.9a.6's regeneration removes the adjudication defect and the balance may
  differ there.

### RAR-E06 — complete HCE refit gate (registered 2026-09-01)

- **Hypothesis.** Recalibrating the complete existing HCE surface against
  phase-balanced, pure self-play WDL improves playing strength despite the
  measured speed cost. The candidate is one indivisible vector: 439/1,218
  slots differ from source and all current linear/nonlinear families may
  interact through score scale, qsearch and pruning consumers.
- **Offline qualification.** Fit vector SHA-256
  `BAD51F3E0AB56B3283C56EC4E06317AC6F4C21109DFDAEA0B833673E773F657E`.
  Fresh confirmation `hce-confirm-20260831_230548` used 150,000 independent
  pure-WDL games from unique book entries 600,001--750,000 and an untouched
  127,778-position phase-balanced test: source **0.12330291**, exact rounded
  candidate **0.12252203**, delta **-0.00078088**; every registered broad
  cohort improved. Candidate bounds/tests are valid. Candidate fingerprint is
  **7,226,051 / 2.460** versus source **6,977,070 / 2.466**.
- **Baseline / candidate.** Baseline `6357856e21219d040d5bac7cba13e95c3107e4a4`;
  candidate `5188eca576755932b31ad634af7821cae5291cf3`. Only `src/eval.rs`
  differs in engine behavior. Baseline binary
  `rarog-hce-refit-base-pext-pgo.exe`, SHA-256
  `04572BA2AC87C9A8E334D838D98A2E074C87232180DA8DEFAF1BFAFC4E5AC481`;
  candidate `rarog-hce-refit-candidate-pext-pgo.exe`, SHA-256
  `4F0465C53143C5E675E42B631AD21175E2E89E605F639FE6A94D6F678C293664`.
  Both manifests record clean `pext-pgo` builds with rustc 1.97.1
  `(8bab26f4f 2026-07-14)` and exact bench verification.
- **Pooled speed diagnostic.** Three independently profiled binaries per arm,
  10 interleaved cycles, `bench 13 3`: pooled median source **3,000,899**,
  candidate **2,965,141**, delta **-1.19%**, bootstrap 95% CI
  **[-2.29%, -0.48%]**; best-of delta -2.25%. Recipe:
  `tools/nps_multibuild.ps1 -Cycles 10 -Repeats 3` with the three
  `hce-confirm`/`hce-refit-base` binaries and three `hce-refit-candidate`
  binaries. This is diagnostic; the clock gate prices the cost.
- **Calibration disposition.** No new null is required. Basilisk and Rarog use
  the same fastchess binary, paired book, 1T `3+0.03`, concurrency 14 and exact
  physical-core affinity instrument. `-NoAdjudication` only omits draw/resign
  termination symmetrically; it changes game duration and outcome variance,
  but neither engine placement nor colour pairing. The maintainer explicitly
  accepted the shared-harness calibration on 2026-09-01 rather than spending
  another 30,000 identical-engine games. The real gate's anomaly checks remain
  mandatory.
- **Harness repair before launch (2026-09-01).** The registered command could
  not start. `d2c7788` rewrote `sprt.ps1`'s option-advertisement guard and
  dropped its empty-list early return; because `$splitOpts` unrolls an empty
  result to `$null` and `[string[]]$null` rebuilds a one-element array holding
  `$null`, every gate invoked **without** `-OptionsA/-OptionsB` threw
  `does not advertise:` with an empty name. The same `$null` also emitted a
  bare `option.` argument to fastchess on that path from `ce4a334` onward. No
  gate had run since `d2c7788`, so no recorded result is affected; the last
  options-free run (`sprt_SearchCore_vs_Head_20260822_101254`) predates the
  guard rewrite and logged no fastchess option warning. Fixed by returning the
  array with `,@(...)` and restoring the empty-list return. Verified in three
  directions: options-free now starts, a bogus option still aborts by name, and
  a valid option list still reaches the manifest.
- **No-adjudication wire proof.** `-NoAdjudication` had never played a game.
  A 20-game `-Mode fixed` run of the exact gate pair
  (`sprt_SmokeCand_vs_SmokeBase_20260901_070934`, seed 4242) ended **20/20 by a
  rules result** — 12 mates, 6 threefold, 1 fifty-move, 1 insufficient material
  — with zero adjudication terminations in either PGN or log, and the manifest
  recorded `adjudication: none`. The flag is live.
- **Registered gate.** Candidate versus baseline, `[0,3]` nElo, alpha=beta
  0.05, cap **80,000 games**, seed **918274631**, `3+0.03`, Threads 1,
  Hash 64 MB, concurrency 14 on physical CPUs
  `0,2,4,6,8,10,12,14,16,18,20,22,24,26`, paired UHO random order, and
  **no draw or resign adjudication**. Book SHA-256
  `7A7F6470615A69C6CF23D565417701D38732876F480AF90D67B42ABADE35644A`;
  fastchess alpha 1.8.0 SHA-256
  `8444E73965AE44E716CDE1BB546A7D7C8C9FC7A442A44194A0C71A3BFFA7DD0D`.
  RAR-M10 predicts about 47,200 games at true +4 nElo and about 78,700
  at true +3 or 0 under its strength-v2 calibration; applying it to no
  adjudication is explicitly an extrapolation, so the cap is conservative and
  is not extended after games begin.
- **Stop/disposition (as registered).** Only fastchess H1 accepts the entire
  vector. H0, any anomaly, or reaching 80,000 without H1 rejects and restores
  the baseline HCE. No point estimate, LOS, offline loss or post-hoc interval
  substitutes for the registered boundary.

#### RAR-E06 verdict — ACCEPTED 2026-09-01

- **Result.** H1 accepted at **3,914 games**: W 1206 / L 958 / D 1750, points
  2081.0 (53.17%). **Elo +22.04 +/- 7.51**, **nElo +32.05 +/- 10.88**, LOS
  100.00%, DrawRatio 38.68%, PairsRatio 1.38, Ptnml(0-2)
  [84, 421, 757, 553, 142]. LLR 2.95 against the (-2.94, 2.94) boundary.
  Wall time 44 minutes. It resolved far short of RAR-M10's 47,200-game
  estimate because that estimate was for a true +4 nElo and the measured
  effect is +32.
- **Artifacts.** `tools/results/sprt_HCERefit_vs_HCEBase_20260901_072106.*`.
  PGN SHA-256
  `A1B621C7CED422BA130EBEC229A4916FA361BEE20DE990AA10B4DBC265CFBA34`; log
  SHA-256
  `B1BD467E6566A198D73C7FDC3458AFECC7209494D3CA76CD72A872689988B71D`. **Both
  hashes were computed after the run, not by the runner**: the anomaly guard
  threw before `sprt.ps1` appends its completion lines, so the manifest
  carries `started_utc` but no `completed_utc`/`pgn_sha256`/`log_sha256`.
- **Anomaly and its disposition.** The match tripped `Assert-NoMatchAnomaly`
  on 3 time forfeits in 3,915 games (**0.077%**). Under the registered stop
  rule as written, any anomaly rejects. The maintainer waived that clause on
  2026-09-01 after the following analysis, and the guard was rate-limited in
  `334c084` so the clause is enforceable in future.
    - All three flagged sides were already decisively lost: round 3 HCEBase at
      -5.32, round 792 HCERefit at -8.72, round 1477 HCEBase at -8.55.
    - The split was 2 baseline / 1 candidate. Reversing all three moves the
      estimate by about 0.3 Elo against a +22.04 result.
    - The guard was added in `d2c7788` and no match had ever run under it.
      Applied to the stored logs it voids nearly every accepted gate,
      including two null calibrations of identical binaries (0.135% and
      0.172%), which is what establishes the forfeits as a harness property.
- **Consequences.** The accepted head fingerprint moves from **6,977,070 /
  2.466** to **7,226,051 / 2.460**; `AGENTS.md`'s behavior-neutral reference
  moves with it. 4.8a (post-refit redundancy removal) and 4.11 (search
  authority on the accepted HCE) are now open. RAR-S70's search counters are
  priors, not a candidate basis, because this refit changed the evaluator the
  search consumes.

## 6. Throughput, build and platforms

| ID | Experiment and conditions | Result / disposition | Conditional lesson and retry trigger | Source |
|---|---|---|---|---|
| RAR-P18 | **A.3.1 toolchain bump 1.97.1 -> 1.98.1, behaviour-neutral qualification, COMPLETE 2026-09-09 (`ca8988a`).** `rust-toolchain.toml` channel only. **Provenance correction, 2026-09-09:** the commit message says the source is `7d8b013`, which is where the 1.97.1 arm was built; a docs-only commit `2faa542` landed on `dev` mid-session, so the 1.98.1 arm was built at `2faa542`. `git diff --name-only 7d8b013 2faa542` is GUIDE.md, PLAN.md and one analysis file and **zero** paths under `src`, `xtask`, `tests`, `Cargo.toml`, `Cargo.lock`, `build.rs` — the two arms are engine-identical by construction, which is also what the shared fingerprint shows. New pin `rustc 1.98.1 (48a229cea 2026-09-01)` / `cargo 1.98.1 (797e8a9bc 2026-08-05)`, `x86_64-pc-windows-msvc`, Ryzen 5950X, host idle at 1-5% for the A/B. **Recipe:** three independent `cargo xtask build --arch pext --pgo` builds per arm, the 1.97.1 arm built BEFORE the pin was changed, then `tools/nps_multibuild.ps1 -Cycles 10 -Repeats 3`, plus a same-source null pair under the same instrument. Binaries (ignored, `tools/results/toolchain-1981-20260909/`): base `9EC19139...4D08CF13`, `4E2345C6...02A1B6AA`, `CA3FD735...B9408F86`; candidate `6871B104...FBF144FE`, `EE25D1BF...C7E1F44A`, `763B2EFF...4FA6DC4C` — all six distinct, so pooling is meaningful. | **NEUTRAL; no behaviour change and no resolvable speed change.** Fingerprint **7,601,220 / geomean EBF 2.474** exact on the 1.98.1 `x86-64`, `avx2` and `pext` plain builds and on all three `pext` PGO builds. `cargo test -p rarog` debug **and** release green, `-p xtask -p texel-tuner` green, `cargo fmt --check` and `cargo clippy --workspace --all-targets --all-features -- -D warnings` clean, `cargo xtask verify-isa` clean for `base`, `avx2`, `pext` and `pext --pgo`. **Pooled PGO NPS 1.98.1 vs 1.97.1: -0.53%** (base 3,091,812 -> cand 3,075,549 n/s; best-of -0.29%), 95% bootstrap **[-1.99%, +0.08%]**, inside the registered +/-1% band. **Null pair** `cand-198-1` vs `cand-198-2`, same source and same toolchain: **-0.42% [-1.30%, +0.33%]** — the same sign and the same magnitude as the arm difference, so the instrument cannot separate the compiler from per-build profile luck at this resolution. | **Read this as "no detected change", not as "1.98.1 is 0.5% slower".** The null pair is the whole reason: a single-build offset of about 0.4% is documented (RAR-P17, RAR-M44) and it reproduced here, so a -0.53% pooled delta whose CI straddles zero is not a compiler finding. Three builds per arm is the minimum this instrument supports and it did not resolve the effect; a claim about the compiler's speed would need more builds per arm, not more cycles. **Two obligations this bump creates and that this row does NOT discharge.** (1) RAR-P08's `rust-lld` Windows ARM64 PGO workaround is versioned debt that must be re-verified on every pinned compiler bump; RAR-P14 did that for 1.97.1 on a native Windows ARM64 host and **1.98.1 is unverified** — it needs the ARM64 compatibility host, not this one. (2) The CI matrix has not run on 1.98.1; RAR-P15 is the precedent that matrix cells catch what local checks do not. **Retry trigger:** re-run this whole row on the next pinned bump. Never bump between an SPRT baseline and its candidate — the bump landed with no experiment in flight and before RAR-E16's binaries are built, which is the only safe window. | `tools/results/toolchain-1981-20260909/` (ignored: six PGO binaries, build/test/clippy/ISA logs, `fingerprint-plain.txt`, `fingerprint-pgo.txt`, `nps-pooled.log`, `nps-null.log`); PLAN A.3.1; RAR-P08; RAR-P14; RAR-P15; RAR-P17; RAR-M41; RAR-M44 |
| RAR-P01 | Phase-9 clean-code/build program, each step bench-identical and spot-checked. | End-to-end result was about −3.2% NPS, inferred around −2 to −3 Elo; infrastructure retained. | On this host, several sub-noise regressions compounded. Every refactor program needs one pooled end-to-end NPS comparison against its starting point. | legacy plan at `757e9a3^` |
| RAR-P02 | Phase-10.3 bench-identical hot-path wave with two PGO builds/arm. | **Accepted, +10.35% NPS and +20.31 ± 7.13 Elo at `3+0.03`.** | In this x64 PEXT/fast-TC condition, speed converted near 2 Elo per 1% NPS. Do not project that constant to LTC, NNUE or another ISA without measurement. | legacy plan at `b75666e^` |
| RAR-P03 | Post-SMP duplicate-compute/index-hoist cleanup. | **Retained, +0.99% then +1.56% median NPS** in independent pooled passes; search fingerprint unchanged. | Small speed gains became credible only through clean worktrees, pooled builds, self-pair calibration and interleaving. | legacy plan at `757e9a3^` |
| RAR-P04 | Board-layer perft comparison with Basilisk. | Rarog's board layer was not the main source of the remaining search-strength gap. | A faster primitive benchmark does not identify search quality. Revisit only if a profile attributes material deployed time there. | `analysis/board_perft_compare.md` |
| RAR-P05 | Pawn-cache enlargement from the profile audit. | A 128× larger table gained about 1.1 hit-rate points but lost 4.5% NPS. | Under that workload, lookup/memory cost dominated the small hit-rate gain. Any future cache size change needs both profile and strength evidence. | `analysis/speed_profile_8_12c.md` |
| RAR-P06 | `origin/arm_fix` added AArch64 `PRFM PLDL1KEEP` and hoisted two HCE `LazyLock` accesses. | **Unverified when written; now CLOSED — both halves have had their target-native A/B.** The prefetch was isolated, ported and ACCEPTED at +1.42% (RAR-P10, RAR-P11). The HCE `LazyLock` hoists were unfrozen and measured in RAR-P16: **+0.12% median, 5/12 paired wins, inside the noise floor** — merged for consistency with the third hoist site dev already had, with no speed claim. The branch's combined +2.51% x64 figure was never reproduced as such. | Causality was correctly refused until each half was measured alone; splitting a combined patch is what let the real +1.42% be told apart from a null. | `src/tt.rs`, `src/eval.rs` on dev; recipe in RAR-P16 — branch `arm_fix` deleted, do not cite it |
| RAR-P07 | `origin/arm_fix` wrapped TT clusters in 128-byte Apple-oriented blocks. | **Unverified when written; now CLOSED and rejected — see RAR-P16,** which measured it on an M4 (-0.12% median, 4/12 paired wins, inside the noise floor) and showed the allocator already returns 128 B-aligned TT bases, so the wrapper is a no-op. No ARM timing existed here and existing cluster alignment already prevents the claimed boundary straddle. | Alignment folklore is not evidence. Compare equal-capacity layouts on actual target topology before retaining a wrapper. | recipe in RAR-P16 — branch `arm_fix` deleted, do not cite it |
| RAR-P08 | Windows ARM64 PGO with pinned Rust used `rust-lld` to work around profile-link failure. | **Retained in 2.3.1:** about +8% NPS locally, unchanged bench/search behavior. | Toolchain workarounds are versioned debt. Re-test on each pinned compiler bump and keep behavior/performance claims separate. | `CHANGELOG.md` 2.3.1 |
| RAR-P16 | Finish the two outstanding `origin/arm_fix` changes on current dev and measure them on Apple Silicon. MacBook Air M4 (4P+6E, fanless), macOS 26.6.1, rustc 1.97.1. Closes RAR-P07. **RECIPE — reconstructible without `arm_fix`, which is the only ref carrying `0ddc8e5`/`3ee4660`.** (B) eval hoist: in `src/eval.rs`, insert `let atk = &*ATTACKS;` at the top of the slider feature group and of the king-safety function, then rewrite the 9 `ATTACKS.` call sites in those two bodies to `atk.` — dev already carried the identical hoist at a third site, this makes all three consistent. (C) TT wrapper: add `LOCAL_CLUSTERS_PER_BLOCK = 4` and `SHARED_CLUSTERS_PER_BLOCK = 2` under `cfg(all(target_os="macos", target_arch="aarch64"))` and 1 otherwise; wrap in `LocalBlock { clusters: [LocalCluster; N] }` and `SharedBlock { clusters: [SharedCluster; N] }`, `repr(align(128))` on that cfg and align(32)/align(64) otherwise; change `Vec<LocalCluster>` to `Vec<LocalBlock>` and `Box<[SharedCluster]>` to `Box<[SharedBlock]>`, reaching a cluster as `blocks[i / N].clusters[i % N]`; add 4 const asserts pinning `size_of(Block) == N * size_of(Cluster)` and `align == size`. Keep dev's 4.8c comment and `AGE_MASK`; hoist `let age = table.age` above the `cluster_mut` call, which the whole-table mutable borrow now requires. **MEASUREMENT:** 4 builds via `cargo xtask build --arch arm64 --pgo` (A baseline, B, C, D = B+C), 12 interleaved `bench 13` rounds driven over stdin, 1 thread, arm order ROTATED each round so each arm holds each slot exactly 3 times. | **NEITHER CHANGE IS MEASURABLE; `3ee4660` stays REJECTED, third time.** Medians: A 5,131,610; B 5,137,678 (**+0.12%**); C 5,125,558 (**-0.12%**); D 5,129,591 (**-0.04%**) — every arm inside RAR-P13's ±0.5% resolvable floor. Paired per-round wins over baseline: B **5/12**, C **4/12**, D **3/12** — coin flips, against the 12/12 with zero distribution overlap that carried the RAR-P11 prefetch. MAD 0.20–0.47%; slot spread 0.20% after rotation. **FINGERPRINT proving each rebuild matched: all four arms 6,519,711 / EBF 2.449**, and all four pass `verify-isa --arch arm64` with 38 prefetch sites. The eval hoist is kept as CONSISTENCY ONLY, with no speed claim; the TT wrapper is NOT merged and exists in no ref — arm C is rebuildable only from the recipe in this row, which is deliberate: it is a rejected no-op and not worth a branch. | **The layout premise was directly falsifiable, and false.** `3ee4660` exists so "the allocator cannot leave the TT base only 32/64-byte aligned" — but a standalone probe allocating the exact shapes (`Vec` of `repr(align(32))` 32 B and `repr(align(64))` 64 B elements) shows macOS returns 16 MiB-granular, hence already 128 B-aligned, bases at Hash = 1, 16, 64 and 256 MB. The wrapper cannot move a single address, so arm C is flat for a mechanical reason rather than by luck. That probe cost two minutes and EXPLAINED a null that 12 rounds could only bound — when an optimisation rests on a stated premise, test the premise, not just the outcome. ⚠ The host was NOT idle (load ~2.5, including the app driving the run); rotation plus median/MAD is what makes this null trustworthy, and the floor is RAR-P13's ±0.5%, not better. ⚠ A `debug_assert_eq!` guards block divisibility in (C); it is unreachable only because `mb.max(1)` forces power >= 16384, and would truncate silently in release if that ever changed. Retry trigger: only a Threads>1 ARM result contradicting RAR-P12 reopens `3ee4660`. | `src/eval.rs` (arm B, merged to dev); arm C measured then discarded; recipe above is self-contained and cites no branch |
| RAR-P17 | **Phase-4 step 4.5.1 — typed per-ply search context, pooled-PGO NPS.** Replaces three parallel `[_; MAX_PLY]` arrays (`stack_moves`, `stack_pieces`, `stack_static_eval`) with one `NodeContext` per ply. Pure representation change: no new fields, no behaviour. Three independent PGO builds per arm, 10 interleaved cycles, `bench 13 3` per sample, machine idle. Baseline pool includes `rarog-47c-only`, whose engine source is identical to the pre-refactor head. | **Behaviour-neutral and NPS-neutral — a clean null.** `bench 13` **6,922,439 / EBF 2.451** on all three candidate builds, exactly the accepted fingerprint. Pooled median NPS base 3,153,730 against cand 3,157,326, **delta +0.11%, 95% bootstrap CI −0.14%..+0.48%**; pooled best-of delta −0.05%. Per-build medians span 3,150,860–3,160,932 across BOTH arms, i.e. the between-build spread swamps the between-arm difference. fmt, all-feature clippy, 248/248 all-feature release and 242/242 debug all clean. | **The locality argument did not pay, and that is recorded in the code rather than quietly dropped.** The change was motivated by every continuation-history lookup reading move and piece at the same ply from two arrays; merging them into one record produced no measurable speed-up, and the whole CI sits inside this machine's ±0.5% floor (RAR-P13). The step is still correct to land, because 4.5.1's purpose is the substrate 4.5.2–4.5.4 consume, not throughput — but nobody should later cite this refactor as an NPS win. ⚠ A null here is also the pass condition: PLAN 4.5.1 gates on exact fingerprint **and** pooled-PGO NPS, and 'no regression' is what a representation change owes. | `tools/nps_multibuild.ps1`; `tools/test_engines/rarog-451{base-a,base-b,ctx-a,ctx-b,ctx-c}-pext-pgo.exe`; RAR-P13; PLAN 4.5.1 |
| RAR-P15 | Phase-4.8h: first full CI matrix dispatch carrying the 4.8 work — the `verify-isa` steps added in 4.8a and the AArch64 prefetch added in 4.8b had never executed on the five-cell matrix. Manual `workflow_dispatch` of `ci.yml` against `development` at `f7f424a`. | **GREEN, 14/14 jobs, 4m 0s.** All five bench cells pass their ISA contract — linux-x86-64 and windows-x86-64 against `base`, and linux-arm64, windows-arm64 and macos-arm64 against `arm64`, which is what finally proves **`prfm` reaches every ARM64 asset** rather than only the one measured by hand on an M4. Cross-platform determinism passes with the prefetch in, so all five cells still agree on the fingerprint. debug x release tests pass on ubuntu, windows and macos; fmt/clippy and the feature-build job pass. | **The `--default-cpu` correction was load-bearing, and it was found by inspection rather than by this run.** The macOS cell builds with `cargo build --release`, whose default `target-cpu` on `aarch64-apple-darwin` enables aes, sha2 and dotprod against `generic`'s bare neon; holding it to the tier baseline would have failed the cell for instructions it is entitled to emit. Checking that BEFORE dispatching cost minutes and saved a red matrix plus the wrong diagnosis. ⚠ Scope: this is the CI matrix, NOT the production matrix. The bench cells build plain `cargo build --release`, so the tiered PGO release path is still verified on only three cells by hand (Windows x86-64, macOS ARM64, Windows ARM64) and **both Linux cells' PGO builds remain untested at this head** — carried to 4.11, where the production platform/ISA matrix is a formal gate. | `.github/workflows/ci.yml`; Plan 4.8h |
| RAR-P14 | Phase-4.8g: retest the Windows ARM64 PGO path on the pinned toolchain. PLAN 4.8 item 2 records the `rust-lld` workaround for rust-lang/rust#156675 as toolchain-versioned debt that must be re-verified on every rustc bump; it is what broke Windows ARM64 PGO before 2.3.1 and had never been checked on 1.97.1. Native Windows ARM64 host, `cargo xtask build --arch arm64 --pgo`. | **PASSES — the last release-blocking unknown in 4.8 is cleared.** The instrumented binary trained, `llvm-profdata merge` accepted the profile (`Merging 1 profile file(s)`) and the optimised build linked to `rarog-v2.4.0-windows-arm64-pgo.exe`. Fingerprint **6,502,902 / EBF 2.449**, so **three platforms now agree exactly** — Windows x86-64, macOS ARM64 and Windows ARM64. ⚠ The ISA contract check did NOT run: `verify-isa` looked for the non-PGO asset name while the PGO one sat beside it (tool bug, fixed in `2b6d2c0`), so `prfm` presence on the Windows ARM64 asset is still owed as one command. | **'The workaround still works' is not 'the workaround is still needed.'** `xtask` forces `rust-lld` for `aarch64-pc-windows-msvc` unconditionally, so this run exercised the workaround path and proves it functions on rustc 1.97.1 / LLVM 22.1.6 — it says nothing about whether upstream has fixed the underlying defect. Removing the override and retesting is the cheap way to find out, and is the right move on some future toolchain bump rather than now, since the override costs nothing while it holds. Retry trigger unchanged: re-run this on every pinned-rustc bump. | `xtask/src/main.rs` `linker_flags`; Plan 4.8g |
| RAR-P13 | Phase-4.8f: identical-binary calibration on macOS ARM64 — the null pair PLAN item 5 requires, run AFTER the prefetch was already accepted, to audit it. Same 12-round interleaved `bench` protocol as RAR-P11 with the SAME binary in both slots, first-slot ordering preserved so any first-in-round penalty would show. | **NO ORDERING ARTIFACT; the +1.42% survives.** Slot A (first) median 5,269,778 (MAD 0.28%), slot B (second) 5,278,329 (MAD 0.12%): slot bias **+0.162%** with slot B winning only **6 of 12** rounds, i.e. a coin flip, against 12/12 in the real A/B. The decisive cross-check is same-slot: slot A held the BASELINE at 5,194,011 in RAR-P11 and holds the CANDIDATE at 5,269,778 here, **+1.46% in the same slot** — reproducing the effect with only the binary changed. Conservatively subtracting the (absent) bias still leaves **+1.26%**. Machine noise floor: **MAD 0.12–0.28%**, so an effect must clear roughly ±0.5% to be resolvable here. | **Calibrate the harness even when the result already looks clean.** The A/B was accepted on 12/12 with zero overlap, which is strong — but it ran baseline-first every round, and nothing in that design could distinguish a real gain from a first-in-round penalty. The null pair is what separates them, and it cost two minutes against a conclusion already banked. Also recorded: one slot-A round read 4,893,079, **−7.2% below median**, on a fanless Air. A mean-of-N estimator would have absorbed that into the answer; median/MAD with interleaving is why it did not. This is the macOS ARM64 performance anchor, and future ARM arms are read against the ±0.5% floor it establishes. | Plan 4.8f |
| RAR-P12 | Phase-4.8e: does the Apple 128-byte cache line cost anything? `SharedCluster` is `align(64)`/64 B, so on Apple Silicon two INDEPENDENT clusters share one line (RAR-P11). Sized BEFORE building any layout change: 1/2/4-thread NPS scaling, 2 pinned middlegame positions x 2 reps, `movetime 5000`, Hash 256, per-position ratios. M4 (4P+6E, PGO arm64) against a same-protocol 5950X reference (pext, non-PGO) measured the same day. | **NO MATERIAL FALSE SHARING — question CLOSED with no code change.** ARM64 scales **1.96x at 2T and 3.89x at 4T**; x86 scales **1.87x and 4.12x**. ARM is within 6% of x86 at 4T and is BETTER at 2T. The pre-registered rule (>=3.8x at 4T closes it, <=3.0x opens the layout experiment) is met, so `3ee4660` stays rejected, `SharedCluster` keeps its 64 B layout, and the three-arm density-controlled experiment is not built. | **Sizing the population beat building the fix**, again — the same discipline as 4.4a and 4.5d. The half-a-cache-line fact is REAL and the cost of it is not measurable, so the honest output is a corrected comment rather than a layout change that would have halved TT density or altered associativity and needed a strength gate to resolve. ⚠ The comparison is a RATIO comparison, never raw NPS (PLAN forbids that across machines), and it is still soft: 4 P-cores on a fanless laptop against 16 desktop cores, PGO against non-PGO, n=2. It is strong enough to decline a speculative layout change, not strong enough to certify ARM SMP quality — time-to-depth belongs to 4.9. ⚠ Method note: the first ARM script reported `seldepth`, not `depth`, because `sed 's/.*depth .../'` is GREEDY and matched the later of the two fields; the apparent depth regression it showed was an artifact and no depth conclusion is drawn here. | `src/tt.rs` (comment only); Plan 4.8e |
| RAR-P11 | Phase-4.8c: the ARM64 verdict run PLAN 4.8 item 3 reserves, plus the Apple topology probe item 4 requires. MacBook Air M4 (4P+6E, fanless), mains power, idle; two revision-matched `--arch arm64 --pgo` builds differing in ONE line of `src/tt.rs`; 12 interleaved `bench 13` rounds, 1 thread. | **PREFETCH ACCEPTED. +1.42% NPS** — baseline median 5,194,011, candidate 5,267,640. **12/12 paired wins with ZERO distribution overlap** (baseline max 5,219,022 < candidate min 5,248,508), sign-test p = 0.00049; per-round gain +0.89% to +2.09%. The candidate is also STEADIER (spread 0.48% versus 1.12%), which is what removing memory stalls looks like. Both builds fingerprint **6,502,902**, matching x86 exactly, so the hint is behaviour-neutral as it must be. The ISA contract behaved as its own negative control: the baseline asset FAILED with `REQUIRED prefetch never appears` and the candidate passed. Topology: `hw.cachelinesize` **128**, `hw.pagesize` **16384**, L1d 64 KB, L2 4 MB. | **Two lessons.** (1) **The 128-byte Apple cache line is real, but `3ee4660` aimed at the wrong hazard.** Neither cluster type can straddle a 128 B line (32 and 64 both divide 128, both self-aligned), so the alignment wrapper addresses something that cannot happen. The REAL exposure is that `SharedCluster` is `align(64)` and documented as 'exactly one cache line' — true on x86-64, FALSE on Apple Silicon, where two independent clusters share a line and two threads can contend over unrelated entries. That is a Threads>1 ARM64 question and needs a 4T ARM A/B before anything is over-aligned, since naive padding would halve TT density. (2) **PGO does not reach the vendored C on macOS:** `cc` rejects the inherited `-fprofile-use`, so Fathom builds unoptimised there — the same 'the build contract reaches the Rust half only' shape as RAR-P09's popcnt finding, low impact because it is tablebase-probe code, but it is a gap in the release pipeline rather than a quirk. | `src/tt.rs`; Plan 4.8b, 4.8c |
| RAR-P10 | Phase-4.8b: port the AArch64 TT prefetch from `origin/arm_fix` onto current development and make its presence enforceable. Emission proved by compiling the exact `prefetch_ptr` body standalone for `aarch64-unknown-linux-gnu` at `-O` (the vendored Fathom C build blocks a full cargo cross-compile on this host). | **ACCEPTED on measurement: +1.42% NPS on an M4, and a silent three-release loss closed.** `prefetch_ptr` had an x86 body and `let _ = ptr;` for everything else, so **all three shipped ARM64 assets did no TT prefetching at all** while the x86 assets did. `prfm pldl1keep` (the ARM analogue of `_MM_HINT_T0`) now compiles in behind an exact `target_arch` cfg; the probe emits it at every call site and it survives `#[inline(always)]` at `-O`. `cargo xtask verify-isa --arch arm64` now REQUIRES the class, so every CI bench cell and every release asset proves it is present. x86-64 is untouched: bench 6,502,902 / EBF 2.449, all three x86 tiers still pass their contracts. | **A missing cache hint is invisible to every instrument this project owns.** The engine plays identically with and without a prefetch - same nodes, same moves, same fingerprint, same tests - it merely plays slower, so node agreement across CI cells cannot see it and neither can a strength gate on x86. A REQUIRED instruction class is the only check that catches it, which generalises: for a hint-shaped optimisation, verify the instruction, not the behaviour. The A/B PLAN 4.8 item 3 requires has now run (RAR-P11) and the port is KEPT. The sibling `origin/arm_fix` commit (`3ee4660`, Apple TT cache-line alignment) is deliberately NOT ported - it has no ARM timing evidence and PLAN 4.8 item 4 requires an Apple result first. | `src/tt.rs`; `xtask/src/main.rs`; Plan 4.8b |
| RAR-P09 | Phase-4.8a: freeze the per-tier ISA contract and make it EXECUTE. Disassembled every x86-64 asset with `llvm-objdump` and classified every mnemonic; added `cargo xtask verify-isa`, which asks rustc what the tier's `target-cpu` enables and holds the artifact to it. Windows x86-64, pinned rustc 1.97.1. | **Two shipped defects found, both invisible to every existing gate.** (1) The `x86-64` baseline asset contained **15 `popcntq`**, every one from `vendor/fathom`: `-C target-cpu` is a rustc flag that `cc` never sees and Fathom selected hardware popcount without a feature test. Fixed by defining `TB_NO_HW_POP_COUNT` when the target lacks the feature. (2) The startup CPU guard **could never fire in a specialized asset** because the statically required feature made detection fold to true. The dead guard was removed and README states the measured requirement per asset. | **Three transferable lessons.** (a) An ISA tier is only as strong as its weakest translation unit. (b) A runtime check for a statically-required feature is true by construction; keep specialized binaries simple and consider only a complete baseline universal dispatcher in Phase 8.1. (c) Ask the compiler, not folklore: derive contracts from `rustc --print cfg`. | `build.rs`; `src/main.rs`; `xtask/src/main.rs`; `README.md`; `.github/workflows/`; `PLAN.md` Phase 8.1 |

## 7. Correctness and protocol lessons

| ID | Experiment or failure mode | Disposition | Conditional lesson / coverage | Source |
|---|---|---|---|---|
| RAR-C01 | Self-consistency tests compared implementations sharing the same rule-50, repetition and SEE omissions. | Independent legal-exchange and external perft oracles added/required. | Green correlated tests do not establish correctness. Important invariants need an implementation-independent oracle. | legacy plan; `analysis/infra_analysis.md` |
| RAR-C02 | Rule-50 draw could override mate; null moves changed the halfmove clock; repetition lacked root/null awareness. | Free mate-precedence fix retained; optional draw-policy bundle followed RAR-S18's strength verdict. | Separate legal terminal precedence from heuristic repetition policy; they can have different acceptance criteria. | legacy plan; `analysis/search_analysis.md` |
| RAR-C03 | Multi-thread diagnostics reset/dumped inside helper-called root search and the diagnostic build had stopped compiling. | Fixed; previous multi-thread counter history was declared unreliable. | Telemetry must have one owner and a build/runtime canary before it can guide search decisions. | `CHANGELOG.md` 2.3.1; legacy plan |
| RAR-C04 | Aspiration mate-score re-search could fail to terminate; capture cutoffs could train quiet correction. | Fixed and regression-covered. | Rare control-flow and attribution faults can contaminate root/time/history together. Retain deterministic tests before strength gates. | `CHANGELOG.md` 2.3.1 |

## 8. Cross-engine evidence imported from Basilisk

These are ideas, warnings or ordering priors already incorporated where useful
in Rarog's forward plan. No additional roadmap item is created merely by
listing them here.

| ID | Basilisk evidence | Possible Rarog implication | Existing PLAN coverage |
|---|---|---|---|
| RAR-X01 | TT-bound-aware pruning evaluation gained +7.18 Elo while preserving raw/corrected eval for learning. | Strong prior for typed result evidence and producer/consumer capability separation, not for copying its TT layout. | 4.6 |
| RAR-X02 | Check-extension removal lost −10.17 ± 6.52 in Basilisk while Rarog's extension had gained +30.75. | Confirms that extensions and their LMR/pruning consumers co-adapt. Rework only inside prospective-depth/refit gates. | 4.7, 4.8 |
| RAR-X03 | Stockfish distillation gained +6.75 in Basilisk but lost −17.11 in Rarog. | Teacher/corpus/representation fit dominates transfer; a sibling success cannot reopen RAR-E03 unchanged. | 4.12–4.16 for evaluator contracts; 5.0, 6.0–6.2, 7.0–7.2 for teacher/data |
| RAR-X04 | A 6-ply continuation-history channel lost −7.70 in Basilisk. | Wider history distance can duplicate existing signals. Rarog should prove unique held-out attribution before adding contexts. | 4.5 |
| RAR-X05 | Exact/PV reward-only history and surprise scaling jointly reverified at +3.06 ± 4.35. | Result kind and confidence can be useful training inputs when sibling maluses are not misapplied. | 4.5, 4.6 |
| RAR-X06 | Root instability TM reverified at +6.46 ± 4.12, while Rarog's raw pool-view variant lost −5.54. | Instability may help only when derived from a completed authoritative root snapshot and bounded with other confidence signals. | 4.12a (retargeted from 4.9, which is now HCE structure) |
| RAR-X07 | Basilisk's +4.34% NPS wave measured +8.69 ± 6.63 Elo at STC; some cached-check/pin optimizations that helped Rarog were negative there. | Speed-to-Elo direction is corroborated near this TC, but individual hot-path optimizations are profile- and language-specific. | 4.19, 8.1 |
| RAR-X08 | Basilisk's `arm_fix` independently tried unmeasured TT over-alignment; its shipped build has clearer ISA-tier documentation but also a PEXT documentation/flag mismatch. | Corroborates the need to measure topology and audit the executable asset contract, not the proposed wrapper. | 4.19, 8.1 |
| RAR-X09 | Basilisk's SMP safety bundle gained +30.42 ± 8.77 at 4T, smaller than Rarog's five-change +102.78 bundle. | The different gains suggest different baseline defects; compare ownership/clock/TT interactions without assigning Elo to individual components. | 4.19, 8.0 |

The cross-review found no additional high-value Basilisk item missing from the
current Rarog plan. Items above are already covered, contradicted by local
evidence, or deliberately postponed to the NNUE/scaling phases.

## 9. Open retry map

| Prior IDs | Retry condition | PLAN destination |
|---|---|---|
| RAR-S11, RAR-S15, RAR-X01 | Phase-4 cluster B reaches these consumers, or NNUE scale freezes and they remain independently ablatable with material activation. | 4.6, else 7.3 |
| RAR-S13, RAR-S14, RAR-S19, RAR-X02 | Prospective depth or evidence architecture wins a categorical gate before its consumers are tuned, in Phase 4 or after NNUE. | 4.7–4.8, else 7.3 |
| RAR-S16, RAR-S38–S41, RAR-X04, RAR-X05 | Phase-4 cluster A reaches the correction/history contract, or NNUE residuals show a populated unique signal. Tune graded weights; never restore the rejected all-or-nothing capture guard. | 4.5, else 7.3 |
| RAR-S17, RAR-R04, RAR-R05, RAR-X06 | Phase-4 cluster E reaches root authority, or real-clock NNUE telemetry shows the completed root-confidence snapshot discriminates without moving total budget. The root gap stays excluded unless root searches produce comparable values. | 4.9, else 7.3 |
| RAR-R07–R10, RAR-X09 | Representative 4T/8T/16T hardware is available after NNUE; price depth diversity and retained SMP switches directly. | 8.0 |
| RAR-P01–RAR-P05, RAR-X07 | A new deployed profile identifies a material hotspot; use pooled same-target final-PGO A/B. | 8.0–8.1 |
| RAR-P06, RAR-P07, RAR-X08 | A new target-native profile identifies a topology/layout cost. The ARM prefetch itself is already accepted; do not retry rejected over-alignment from cache-line folklore. | 8.0–8.1 |
| RAR-S27, RAR-S29, RAR-S49 | **Closed for the flat TT-refinement depth-floor shape.** Reopen only through a materially different evidence model — a Phase-4 cluster-B contract or a post-NNUE fit — never the removed UCI coordinate. | 4.6, else 7.3 |
| RAR-S31 | Re-evaluate `SingularTtDepthMargin` inside Phase-4 cluster D or after NNUE, in final PGO only. The historical tune-binary H1 (+3.35 ± 2.44 Elo) did not meet the later material/final-PGO policy. | 4.8, else 7.3 |
| RAR-E03, RAR-E04, RAR-X03 | NNUE data/teacher experiment with changed representation and a frozen external holdout — not another HCE refit. The Phase-4 HCE track may study evaluator contracts but does not retry this distillation. | 4.12–4.16 for contracts; 5.0–7.2 for teacher/data |

Anything not meeting its trigger stays closed. A retry is a new experiment with
a new ID and manifest; it does not overwrite the historical row.

## 10. Template for a new experiment

```markdown
### RAR-<area><number> — <short name>

- Date / owner:
- Baseline SHA / candidate SHA / dirty-diff hash:
- Binary / compiler / PGO identity:
- Research question:
- Hypothesis / proposed mechanism:
- Competing hypotheses:
- Interacting mechanisms / consumers:
- **PRE-REGISTERED PREDICTION (freeze before exposure):**
  - Expected diagnostic movement:
  - Expected Elo sign/range, if defensible:
  - Probability positive/useful and confidence basis:
  - Most likely failure mode:
- Falsification criteria:
- Cheapest prior falsifier: test / result / implementation still justified?:
- Registered gate and stop rule:
- Full conditions / provenance: flags, manifests and hashes; book/hash, TC,
  threads, Hash, concurrency, affinity, adjudication, node budget and cohort:
- Result:
  - Diagnostics: nodes, EBF, NPS, depth, counters, suites (not the verdict):
  - Games/verdict: games, W-D-L, Elo/nElo and CI, LLR:
- Disposition: accepted / retained / rejected / neutral/inconclusive /
  observation / no-change / deferred:
- **PREDICTION CALIBRATION (append after exposure):**
  - Original prediction (do not rewrite):
  - Observed result; sign and magnitude reasonable?:
  - Proposed causal mechanism supported?:
  - Missed interaction or instrument failure?:
  - Confidence over/under-calibrated?:
- Postmortem: changed causal assumption / what did not change / alternatives:
- Conditional lesson:
- Retry trigger or `closed`:
- Artifacts / commits:
```
