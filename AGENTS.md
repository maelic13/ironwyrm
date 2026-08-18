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
  `assert old in text` before writing, and re-read the region after.
- Before claiming a behavior-neutral change, prove it: `bench 13` must
  reproduce the accepted fingerprint **6,519,711 / EBF 2.449** exactly.

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
- **SPSA is conditional, not owed.** PLAN rule 4 says "only when activation,
  interaction and curvature justify the cost". Establish that first with a
  zero-game sweep over the suite or bench; a flat or monotone surface is
  evidence *against* spending it.

## Handing work back

- Give runnable commands, in their own fenced block, and restate them rather
  than referring back.
- Report what was actually measured. If a step was skipped or a result is
  partial, say so plainly.
