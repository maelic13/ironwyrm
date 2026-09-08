# Integrated board cluster qualification — RAR-M41 / 4.11b.16

**Registration frozen 2026-09-08, before the performance run.** Correctness is
complete and reported below. The performance arm is prepared and handed to the
maintainer to run on an idle host; no pooled-PGO A/B result exists yet.

## Arms

| Arm | Revision | Contents |
|---|---|---|
| Baseline | `1d720af` | state at the end of 4.11b.9's registration, before the fused relocation landed |
| Candidate | `1be34ac` (head) | plus `5c439da` fused relocation, `f70ac19` history reservation, `20ee114` footprint assertions |

`1d720af..HEAD` contains exactly three engine-source commits; everything else in
that range is documentation or tooling. Both arms are **behaviour-identical** —
all six built binaries reproduce `bench 13` at **7,601,220 nodes / EBF 2.474** —
so the trees are the same and NPS at fixed nodes is a clean throughput
comparison rather than a confounded one.

**Why not section entry.** The leaf asks to explain differences from section
entry. The one behaviour change inside 4.11b is the 4.11b.5 SEE repair
(`fce0b44`), which established the current fingerprint. Measuring across it
would compare different trees and confound throughput with a correctness fix
whose value belongs to the 4.11b.17 playing gate. The baseline is therefore the
last point at which the fingerprint already equalled today's, isolating exactly
the throughput work this leaf can bank.

## Correctness matrix — complete

| Check | Result |
|---|---|
| Debug suite | **282 passed**, 0 failed, 0 warnings |
| Release suite | **283 passed**, 0 failed, 0 warnings |
| Board / SEE / parser / draw tests | included above; `see_contract` 8/8 with all 41 external fixtures, `see_pins` 6/6, `draw_semantics` 8/8 |
| Randomized move/state comparison | `board_differential`, `fuzz_lite` in both suites |
| Second slider backend (PEXT) | **72 passed** across `slider_backends`, `board_correctness`, `board_differential`, `see_contract`, `see_pins`, `draw_semantics` under `--cfg rarog_pext -C target-cpu=native` |
| `cargo fmt --check` | clean |
| Clippy `--all-features --all-targets` | zero warnings |
| Feature-off behaviour | default build is feature-off; suites above run it |
| Production fingerprint | **7,601,220 / EBF 2.474** on all six PGO binaries |

**Differences from section entry, explained.** Exactly one: the 4.11b.5 SEE
repair changed the fingerprint to its current value. Every board change after it
— 4.11b.6, 4.11b.9, 4.11b.13, 4.11b.14 — is behaviour-neutral and preserved it,
which the six independent fingerprint checks confirm.

**One check not runnable on this host, stated rather than claimed.** The
supported-target check for touched code could not be run here:
`aarch64-pc-windows-msvc` is not an installed target, and a cross build fails in
the vendored fathom C code for want of a cross `cl.exe`, not for any Rust
reason. Touched code is architecture-neutral, and the two `const` assertions
added at 4.11b.14 are deliberately upper bounds so 64-bit padding differences
cannot break them. Per AGENTS the ARM64 machine is a compatibility-test host;
this check belongs there.

## Performance instrument

Six PGO binaries, `cargo xtask build --arch pext --native --pgo`, three per arm.
**PGO build variance is real and was verified, not assumed**: two builds from
identical source produced different hashes, so all six differ and pooling is
meaningful.

| Setting | Value |
|---|---|
| Suite | frozen 20-root board-search cohort |
| Nodes per root | 2,000,000 |
| Main pairs | 96, alternating order, rotating builds so no single build carries an arm |
| Null pairs | 32 |
| Warm-up | one discarded full run per binary |
| Threads / Hash | 1T / 16 MiB |
| Bootstrap | 5,000 resamples, seed 4119 |
| Compiler / ISA | rustc 1.97.1, `x86_64-pc-windows-msvc`, PEXT, `target-cpu=native` |
| Affinity | none, matching RAR-M33 so its measured noise remains comparable |

**The null pair is the point of this design.** `cand-1` against `cand-2` are two
PGO builds of the *same* revision, so any difference they show is instrument
noise plus PGO build variance. That makes the noise floor **measured** rather
than asserted, which is what the leaf's "predeclared noise/practical floor"
requires.

## Decision rule — frozen

The rule below was written into `tools/results/cluster-411b16/qualify.py` before
any A/B number was produced. A two-pair, 120,000-node smoke run was executed
afterwards purely to prove the pipeline and the gates fire; its numbers are
statistically meaningless and are not treated as data.

1. **Mandatory.** All six binaries fingerprint 7,601,220 / EBF 2.474; every
   paired root answer matches on depth, seldepth, nodes, score type/value, best
   move, full PV and ponder; every arm's host CPU busy stays **<= 15%**. An
   over-threshold arm makes the run **VOID**, never a rejection.
2. **Instrument validity.** The null pair's 95% interval must **contain zero**.
   If two builds of the same revision differ significantly, the instrument is
   biased and the run is void.
3. **Practical floor: +0.5%.** Set from Elo relevance, not from the observed
   non-PGO figure: at the project's measured ~2 Elo per 1% NPS at STC, 0.5% is
   about 1 Elo, the smallest gain worth banking.
4. **Effective floor** is `max(0.5%, null-pair 95% upper bound)`.
5. **Bank a speed claim only if** the candidate median is higher **and** the
   95% bootstrap interval excludes zero **and** the point estimate exceeds the
   effective floor.
6. **Otherwise no speed claim is banked.** The cluster's correctness stands
   regardless, and 4.11b.9's non-PGO +0.876% remains recorded but unbanked.

**A candid limitation.** RAR-M33 measured a bootstrap half-width of 1.003% at 32
pairs and 1.2M nodes; 96 pairs at 2M nodes projects roughly 0.5%, against an
expected effect near +0.9%. If PGO build variance pushes the null-pair upper
bound above the effect, rule 4 makes the candidate unbankable **by
construction**. That is a legitimate and honest outcome — it means the effect is
smaller than the production build's own variance — and it is registered here
before the fact rather than discovered afterwards.

## Status

Correctness: **complete and passing**. Performance: **prepared, not yet run**.
No Elo is claimed or inferred, and no cross-engine throughput ratio is involved.
The playing gate for deliberate behaviour changes remains 4.11b.17.
