# Agent operating rules for Rarog

Read `GUIDE.md` for what to work on and `PLAN.md` for why. This file is only
about **not producing wrong results**. Every rule below exists because it was
violated and cost real work — none of them are precautionary.

## The one failure mode

Almost every mistake made in this repo by an agent has the same shape: *the
check that was run did not check what it was thought to check*. A stale binary
was measured, a parser silently read one record instead of forty, an exit code
came from the wrong end of a pipe. The engine was never the problem.

So: **verify mechanically, never by eyeballing, and never by assuming the tool
did what its name suggests.**

## Measurement

- **`--all-features` enables `texel`, which must never be measured.** The
  manifest says it bypasses the eval and pawn caches. `cargo test --release
  --all-features` leaves that binary in `target/release/rarog.exe`, and a
  depth sweep run on it produced a confident, wrong conclusion — reversed
  once rebuilt. The tell was that the BASELINE moved between two sweeps; if a
  number you are not changing changes, stop and check the binary.
- **Rebuild immediately before measuring, with the exact feature set.**
  `cargo test`, `cargo clippy` and `cargo bench` all build the `rarog` binary
  too, with *their* features, and leave it in `target/release/rarog.exe`. A
  differential run was voided this way. There is no such thing as "the binary
  is probably still right".
- `bench` dumps diagnostic counters **once per position** — 40 lines per name
  for `bench 13`, 47 for the oracle. They must be **summed**. Reading the last
  one gives a single position's numbers that look plausible and are wrong.
- **Never hand-roll a counter parser.** Use `tools/diag/bench_counters.py` for
  bench and `tools/diag/phase4_differential.py` for the suite. Both aggregate
  correctly.
- Counter ratios are only valid at `RAROG_DIAG_SAMPLE_STRIDE=1`. Half the core
  counters are sampled and half are exact, deliberately; see
  `analysis/phase4_counter_spec.md`.
- Before differencing two counters, check they are in the **same unit**. Per
  node vs per move has produced three false findings in this project
  (RAR-S25, and twice inside the Phase-4 instrumentation itself). A passing
  invariant does not prove comparability — `probcut_cut <= probcut_attempt`
  held for two phases while the two counters counted different things.

## Verification

- Run tests in **debug and release**. CI is a matrix; `--release` alone has
  missed real bugs, and a debug-only failure appeared again in 4.7c.
- `cargo fmt --check` and `cargo clippy --all-features --all-targets` must be
  clean. Zero warnings.
- **Suppress lints with `#[expect(...)]`, not `#[allow(...)]`.** An
  expectation warns when it stops being needed, so the suppression list
  cleans itself; an `allow` sits there forever. Converting the crate's 25
  found six that had been dead for some time. Use `allow` only when the
  lint fires in one feature configuration and not another — there is
  exactly one such site, in `search_options.rs`, and it says so. Every
  suppression still needs a written reason.
- **Check exit status directly**, never through a pipe: `cmd > out 2>&1; echo
  $?` and then read `out`. `cmd | tail` reports `tail`'s status, which is
  always 0.
- **Every scripted edit must assert its anchor matched.** A `str.replace` that
  finds nothing changes nothing and reports success. If you edit with a script,
  `assert old in text` before writing, and re-read the region after. Assert the anchor is **unique** and lands in executable code:
  a `--rset` block anchored on a line that also appears in the module
  docstring was inserted as prose, parsed fine, and silently measured
  default parameters in every run for two screens.
- **Prove a harness wire is live before trusting a null from it.** Set a
  deliberately absurd value and require the numbers to move. Two candidates
  were recorded as null results by a dead `--rset`; one of them, re-measured,
  moved oracle agreement 66% -> 78%. Verifying the ENGINE responds is not the
  same check -- a standalone probe confirmed the option worked while the
  instrument reporting on it did not.
- Before claiming a behavior-neutral change, prove it: `bench 13` must
  reproduce the accepted fingerprint **6,901,489 / EBF 2.458** exactly
  (RAR-E08; it was 7,226,051 / EBF 2.460 under RAR-E06 plus the 4.9a.4 mate
  drive, 6,977,070 / EBF 2.466 under RAR-S70, and 7,467,143 / EBF 2.477 before
  that). A fingerprint identifies the SEARCH, so it cannot see
  a change confined to positions the bench suite never reaches: the accepted
  head carries a 4.9a.4 mate drive that moves KBN-K conversion from 19.4% to
  96.9% and leaves this number byte-identical. Never read "bench unchanged" as
  "behaviour unchanged" for an evaluation term with a narrow activation. The
  fingerprint is
  platform-INDEPENDENT: RAR-P14 recorded three platforms agreeing exactly,
  and `aaa715a` rebuilds to 6,519,711 on x86 which is the number RAR-P16
  recorded on ARM64. A differing number means differing CODE.

## Changes

- Engine changes and tooling/doc changes go in **separate commits**.
- Commit after each finished **and verified** step, not after each edit.
- **No `Co-Authored-By` trailers.**
- A correctness test is never relaxed in the same commit as the change that
  made it fail. Fix its precondition, in its own commit, with the measurement
  that justifies it.
- Counters explain a candidate; only a registered SPRT accepts one. Node counts
  are not Elo: a measured +7.36% tree change was worth −1.49 ± 2.87 Elo.

## Evidence

- **A ledger row must reproduce its artifact without the branch it came from.**
  Record the recipe — exact parameter values, or the diff when it is small —
  plus a fingerprint that proves a rebuild matched. A bare SHA is not evidence;
  it is a promise that someone else is still storing your evidence.
- Before deleting any branch or tag, check what the ledger cites on it. A SHA
  with no output from `git branch -a --contains <sha>` is **dangling** and will
  disappear at the next `gc`.
- This is not hypothetical. RAR-S54 — the +4.06 result the whole 4.7 cluster's
  prior rests on — cited a commit that turned out to be docs-only; its real
  source sat on a deleted branch, dangling, and the archive tag that was
  supposed to protect it covered the other arm of the experiment. The probe was
  twelve parameter values and now lives in `EXPERIMENTS.md`, where it should
  have been from the start.

## Gating

- The strength unit is one dependency-complete, locally fitted **cluster**.
  Internal substeps are not expected to win standalone and do not get their own
  gates.
- Register in `EXPERIMENTS.md` — hypothesis, baseline SHA, gate, cap, stop rule
  — **before any games**. Never change bounds, cap, book or adjudication after
  seeing games.
- **`[0,3]` nElo is the DEFAULT bracket.** Widen only when the prior is
  genuinely large, and say why in the registration. This is not "narrow is
  better": 4.7 had a 25–60 nElo prior, measured +24.90, and `[3,10]` resolved
  it in **2,838 games** — a wide bracket is the right instrument for a large
  effect. The error is using one for a small candidate. Compute the games at
  the EXPECTED value from RAR-M10 before choosing, every time.
- **A removal or simplification needs a bracket that permits a small loss**,
  fishtest-style (`[-1.75, 0.25]`), not `[0,3]`. A repair of unknown sign wants
  a symmetric bracket that can detect harm — RAR-S62 used `[-5,5]` and resolved
  in 4,436 games.
- **The harness already runs GSPRT; nothing to change.** `tools/sprt.ps1`
  passes `model=normalized` to fastchess and the output carries `Ptnml(0-2)`,
  so it is the pentanomial GSPRT — the same mathematics fishtest uses, with
  nuisance parameters replaced by maximum-likelihood estimates. The gap between
  this project and fishtest is bounds and budget, never the test.
- **Wide bounds anchored high REJECT small gains — size them from RAR-M10
  before registering.** Fishtest uses `[0, 2]` STC and `[0, 1]` LTC in
  normalized Elo — narrow, near zero — with large budgets. Wide brackets do not
  merely resolve slowly on a small gain, they **reject it**: `[0,10]` drives a
  true +4 nElo to H0 in ~35k games and `[3,10]` does it in ~20k. Three gates
  were registered at those bounds against candidates measuring 4–7 nElo, so
  they were configured to reject what they were measuring. `[0,3]` is the
  bracket to prefer here — RAR-M10 was fitted on `[0,3]` gates, so it is the
  in-regime choice rather than an extrapolation — and it accepts a true +4 in
  ~47k games.
- **A real gate is now an overnight run, and that is the honest price.** At
  ~98 games/min, `[0,3]` needs roughly 8 hours for a true +4 and 13 for a
  true +3, and about the same to reject a dud. Zero-game bench and counter
  screening is what decides WHICH candidates earn that budget; it never
  decides whether one works (RAR-S64: a mechanism with a clean bench signal
  measured exactly zero in games).
- **Do not invent an acceptance rule after seeing a result.** A threshold like
  "accept if the CI excludes zero at 20,000 games" is arbitrary and is the same
  act as moving the bounds. If small gains need to be bankable, register the
  narrower bracket PROSPECTIVELY.
- **An unresolved stop is not "probably fine".** RAR-S61 measured
  +4.50 ± 3.50 at LOS 99.41% and the entire effect turned out to be a stale-read
  bug (RAR-S64 re-measured it at +0.39 once fixed). A high LOS on a point
  estimate is not evidence that a mechanism works.
- **SPSA is conditional, not owed.** PLAN rule 4 says "only when activation,
  interaction and curvature justify the cost". Establish that first with a
  zero-game sweep over the suite or bench; a flat or monotone surface is
  evidence *against* spending it.

## Documents

- **`GUIDE.md` and `PLAN.md` are updated in the SAME commit.** GUIDE is the
  overview of PLAN — current state and the ordered steps, nothing else. A
  GUIDE that disagrees with PLAN is worse than no GUIDE, because it is the
  file that says what to do next and it will be believed.
- **GUIDE carries STATUS, not just a list.** Its Phase-4 checkboxes are how
  the maintainer sees what is done. Tick one only when the step is finished
  AND verified, in the commit that finishes it — never in advance.
- **Tick the PARENT when its last sub-step is ticked.** A step whose sub-steps
  are all done is done; leaving it open makes finished work look outstanding.
- **Sub-items indent by 4 spaces, never 6.** Under a `- ` parent the content
  column is 2, so 6 spaces is the indented-code threshold and the sub-list
  silently renders as a code block. Both rules are checked mechanically —
  run it rather than reading the file:

  ```bash
  python tools/diag/check_guide.py
  ```
- **Keep GUIDE short.** If a change to GUIDE runs past a few lines, it belongs
  somewhere else: what a step INVOLVES goes in `PLAN.md`; a completed step's
  record goes in `TRACKER.md`; a repeatable procedure goes in `PROCESS.md`;
  durable evidence goes in `EXPERIMENTS.md`; a measurement's derivation goes in
  `analysis/`. GUIDE grew to 898 lines by absorbing all five and stopped being
  readable as an overview.
- `TRACKER.md` is HISTORY. Its numbering is retired and does not correspond to
  PLAN's. Never take a next step from it.
- When two documents disagree, source, defaults and reproducible artifacts
  outrank prose. Fix the prose in the same change.

## Step sequencing and explicit holds

- Work one executable leaf at a time: verify proportionately, update PLAN and
  GUIDE in the same documentation commit, report and stop. Engine and
  tooling/doc changes still go in separate commits; intermediate commits do
  not falsely mark an unfinished cluster accepted.
- Read GUIDE's current/held overview and PLAN's dependency register before
  selecting work. The earliest open leaf may be held. Keep its checkbox/ID,
  reason, unblock condition and latest required completion point visible.
  Review holds each handoff; resume the earliest eligible one. Never silently
  skip, move or tick missing verification.
- The agent may use check_guide.py internally for structural consistency.
  Its raw open-item list is not a scheduler and does not resolve holds or
  dependencies. The maintainer need not run it; always state the next
  executable step and any held obligation that matters.
- Report confounds when found. Correct contradicted current claims in
  PLAN/GUIDE/analysis/EXPERIMENTS where applicable; preserve historical
  measurements with explicit supersession rather than silently deleting them.
- Test constructs and behavior, not a word in a comment/disclaimer. Check
  each command's actual exit status and require every intended check to have
  run successfully before committing; do not rely on a chained command's
  final status as proof of earlier checks.
- The accepted fingerprint is revision-specific. For a neutral change compare
  to the exact immediate accepted baseline (currently 6,901,489 / EBF 2.458)
  and targeted cases. A deliberately accepted behavior change updates the
  fingerprint record; never preserve a known defect to force an obsolete count.

## Handing work back

- When maintainer action is needed, give runnable commands in their own fenced
  block and restate them rather than referring back. Routine internal checks
  need not become user chores. Always name the next executable leaf.
- Report what was actually measured. If a step was skipped or a result is
  partial, say so plainly.
