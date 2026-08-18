# SPSA tooling improvement branch

Branch: `spsa_impr`  
Base: `development` at `4a2b3da`  
Prepared: 2026-08-02

## Purpose

This branch preserved a proposed SPSA workflow improvement package for later
review. It was intentionally not merged into `development`: the package grew
beyond the immediate parameter-bake task and must be kept, pruned, or discarded
feature by feature before it drives another expensive tune.

> **Status on `dev`, 2026-08-18.** This document, `tools/sample_epd.py` and
> `tools/spsa_configs/feature_neutral_values.json` were merged so the design
> record and its two inert assets are not stranded on a side branch. The
> **runner changes were deliberately NOT merged** — `tools/spsa.ps1`,
> `tools/sprt.ps1` and `tools/setup_tools.ps1` on `dev` are newer, and the
> branch's own "Validation already performed" section records that no fresh
> setup, disposable tune or interruption test was ever run. Two specifics:
>
> - **The adjudication alignment here is superseded.** This branch moved SPRT
>   to `resign 600 twosided` on 2026-07-30 to match fishtest. On 2026-08-02
>   `tools/harness_common.ps1` replaced that with a calibrated split, measured
>   on 69,350 Rarog games: **`strength-v1` uses one-sided 600/3** because it
>   produced no chess-result reversals and changed 0.20% of triggers, while
>   **`datagen-v1` keeps two-sided 600/3** because a false resignation
>   mislabels every sampled position in that game. Merging this branch's
>   harness files would regress that to an uncalibrated setting.
>   **Superseded 2026-08-18 (RAR-M13):** both profiles are now 600/3
>   two-sided, so this branch's *value* is what the project uses — but it
>   arrives by maintainer decision on consistency, on top of the
>   calibration, not by the "match fishtest" argument this branch made.
> - **`feature_neutral_values.json` is currently inert.** Nothing on `dev`
>   reads it; the richer `-ShowValues` that consumes it lives only here. It is
>   merged as data so the rail/feature-neutral semantics are not lost, and it
>   has an owner: whoever next revisits `-ShowValues`.
>
> `src/`, `PLAN.md` and `GUIDE.md` on this branch predate the Phase-4 rewrite
> and must never be merged.

The design constraint is fixed: do not address uncertainty by asking for
longer time controls, more games, or another all-weekend SPSA. Reduce wasted
wall time and improve extraction from the games the user can afford.

## Implemented on this branch

### Read-only result extraction

`tools/spsa.ps1 -ConfigGroup <group> -ShowValues` now:

- parses complete per-iteration parameter snapshots from the append-only log;
- deduplicates repeated snapshots produced across resumed sessions;
- computes a whole-vector mean over the planned final 15% of effective,
  rounded UCI centers and rounds only once for the proposed bake;
- shows the raw endpoint beside the tail bake;
- compares early and late halves of the tail in configured SPSA-step units;
- flags movement over 0.5 step descriptively, without treating trajectory
  shape as a strength verdict;
- flags numeric rails and semantic `OFF` / `NEAR-OFF` values using
  `tools/spsa_configs/feature_neutral_values.json`;
- prints explicit recommendations for every flagged value.

Future setup writes `tuner/run_meta.json`, fixing the horizon, games per point,
tail fraction, and opening-cohort identity so `-ShowValues` does not infer the
horizon from `A`. Existing runs without metadata fall back to `A * 10`.

### Faster opening handling without deliberate reuse

`tools/sample_epd.py` creates an exact deterministic uniform reservoir sample,
shuffles it, and replaces its output atomically. Fresh setup proposes a 250k
cohort from the 2,632,036-position UHO source.

The weather-factory patch changes fastchess from `order=random` to
`order=sequential start=<cursor>`. `main.py` derives the cursor from completed
iterations. A 5,000-point tune therefore consumes 80,000 distinct paired
openings, including across resumes, while every short-lived fastchess process
indexes about one tenth as much book input. Setup rejects horizons requiring
more than 250k pairs.

This reduces book-index input, not necessarily total tune time by 10x. The
end-to-end wall-time benefit has not been measured.

### Resume correctness

Upstream weather-factory increments `spsa.t` before launching a mini-match. If
Ctrl-C interrupts that match, `finally` saves the in-flight point as completed
even though its gradient and parameter update never completed.

The proposed transactional patch computes against `next_t` and assigns
`self.t = next_t` only after the match result and parameter update complete.
Existing state remains format-compatible.

### Smaller overheads and reproducibility

- `graph.png` is written every `save_rate` checkpoint (currently 10 points),
  while every in-memory graph sample remains recorded.
- `-Unattended` uses every physical core on a fresh setup. It changes neither
  games nor schedule. A resumed run must retain its configured concurrency.
- redundant `pip install matplotlib` calls are skipped when import succeeds.
- weather-factory is pinned and verified at
  `19b4805c9a2372955c29666118070269f34aa2eb` before source patches are applied.
- source anchors, Python compilation, horizon capacity, and required patch
  markers fail closed before a fresh tune starts.

### Corrected reasoning and durable policy

`PLAN.md`, `GUIDE.md`, `tools/spsa_configs/README.md`, and
`tools/spsa_convergence_model.py` now record that:

- SPSA uses two perturbed evaluations per point independently of dimension,
  but real convergence is not dimension-free;
- a separable equal-curvature simulation cannot select the correct real-world
  parameter scope or establish a universal curvature/noise threshold;
- the earlier directional 1,500-iteration kill checkpoint was invalid;
- the +4.06 joint probe was a useful seed, not a guaranteed +4 floor;
- a jointly fitted vector must not be altered with a per-coordinate bake
  filter before its gate;
- future fits should contain changed, high-leverage, demonstrably active
  coordinates rather than every exposed knob;
- project weaknesses are surfaced for a joint keep/fix/defer decision instead
  of being silently accepted as constraints.

## Current parameter semantics captured

The final values below were provisional near iteration 4,994 and are included
only to explain the diagnostics. Recompute from the completed log before any
bake.

| Parameter | Provisional bake | Meaning | Proposed treatment |
|---|---:|---|---|
| `EvalPruneTtMinDepth` | 0 | Exact OFF and lower rail: no TT-depth guard | Keep 0 in the joint gate. If it passes, remove the behaviour-neutral guard and UCI knob after bench-equivalence verification; no new SPSA. |
| `CorrRfpScale` | 6 | Near OFF, but nonzero integer contributions may remain | Keep the joint value. Measure activation and contribution magnitude after the gate before proposing removal. |
| `CorrFutScale` | 5 | Near OFF, but nonzero integer contributions may remain | Same treatment as `CorrRfpScale`. |
| `LmpCountBase` | 1 | Lower rail, not feature OFF | Keep 1 in the joint gate. Zero is mechanically non-negative in the count formula but currently rejected by `params.rs`; retain 0 only as a future narrow candidate. |

At the last analysis, no coordinate moved by 0.5 configured step between the
two halves of the final-15% tail. That supports using the tail mean but is not
strength evidence; SPRT remains the verdict.

## Not activated in the current tune

None of the new runner behavior drove the ongoing 10.4.6(a) games:

- `setup_tools.ps1` was not run;
- the 250k cohort was not generated;
- opening order/cursor, concurrency, adjudication, SPSA schedule, and game
  count were not changed;
- the active Python process had already loaded its source before the temporary
  ignored graph edit, and that ignored edit was restored before returning to
  `development`.

Only the read-only `-ShowValues` analysis was exercised on the current log.

## Validation already performed

- PowerShell parser accepted `tools/spsa.ps1` and `tools/setup_tools.ps1`.
- Python compilation accepted `sample_epd.py` and the convergence model.
- JSON parsing accepted the semantic metadata.
- `sample_epd.py` produced identical hashes for two same-seed smoke samples
  and the requested line count.
- simulated in-memory applications of the opening-cursor, transactional-step,
  and main cursor patches compiled as Python.
- `-ShowValues` parsed the live ~60 MB history log in about three seconds and
  emitted tail, drift, rail, and feature-neutral diagnostics.
- `git diff --check` passed.

No full fresh setup, disposable tune, interruption test, end-to-end opening
cursor test, or wall-time benchmark has been run.

## Required review before merging

Review these independently; do not accept the branch as one indivisible
package.

1. **Choose the bake policy.** Confirm whole-vector final-15% averaging versus
   the raw final theta. If keeping the mean, decide whether 15% is fixed for all
   tunes or configured per run.
2. **Resolve adjudication first.** `setup_tools.ps1` still contains the pending
   600/3 two-sided patch. The project plan requires retrospective calibration
   of 400/500/600 one-sided after the current SPSA and before its SPRT. Do not
   run setup for a new tune until the selected shared strength-test profile is
   centralized and this old patch is replaced.
3. **Benchmark the real bottleneck.** Measure fastchess startup/index time on
   the full 2.63M book and proposed 250k cohort, plus complete point time. Keep
   the cohort/cursor only if the wall-time saving is material.
4. **Integration-test unique openings.** Run a small disposable harness,
   interrupt/resume it, and verify PGNs use the expected non-overlapping cohort
   slices and correct color-reversed pairs.
5. **Integration-test transactional resume.** Interrupt during a mini-match
   and prove `state.t` and parameters remain at the last completed point, then
   resume and complete that same scheduled point.
6. **Test graph throttling.** Confirm `graph.update` still receives every point
   and the final graph is saved on both normal exit and Ctrl-C. The present
   patch saves periodically but may leave the last partial checkpoint absent;
   decide whether `finally` should save one last graph.
7. **Measure `-Unattended`.** Compare point time at 14 versus 16 physical cores
   without changing games. Keep only if the two-wave scheduling gain is real
   and thermals do not reduce throughput.
8. **Decide cohort sizing.** Fixed 250k supports at most 15,625 points at 16
   paired openings per point. Consider generating exactly the registered need
   plus a reserve, recording the source/cohort SHA-256, and verifying line
   count before launch.
9. **Expand semantic coverage deliberately.** Audit all tunable parameters for
   values that disable features, select degenerate behavior, or hit artificial
   legality rails. The current metadata covers the known correction, TT-guard,
   capture-guard, aging, and LMP cases—not necessarily the entire UCI surface.
10. **Review dependency pin policy.** Decide whether the weather-factory pin
    belongs here or whether Rarog should vendor/fork the small runner instead
    of accumulating textual patches against an external clone.
11. **Add automated tests.** At minimum, add sampler determinism/uniformity,
    log-parser fixtures (truncated block, duplicate resume block, missing
    metadata), patch idempotence, and resume/cursor unit tests.
12. **Prune documentation on merge.** PLAN/GUIDE currently describe the whole
    proposal because this branch is its review artifact. Update them to match
    only the subset ultimately retained.

## Suggested order when work resumes

1. Finish the current SPSA workflow on `development`: final result extraction,
   resignation calibration, chosen vector bake, PGO build, and primary SPRT.
2. Return to `spsa_impr` after that strength decision is complete.
3. Resolve adjudication and bake policy.
4. Benchmark book indexing and concurrency before doing more implementation.
5. Run disposable interruption/opening integration tests.
6. Keep, prune, or discard each feature and update this document.
7. Merge only the reviewed subset back into `development`.
