# Integrated board cluster playing gate — RAR-E15 / 4.11b.17

**Registered 2026-09-08, before any games.** Bounds, cap, book, clock,
adjudication and stop rule are fixed here and in `EXPERIMENTS.md`; none of them
may change once games are seen.

## Arms

| Arm | Engine | Git | Bench 13 | Binary SHA-256 |
|---|---|---|---:|---|
| Candidate | `rarog-411b-cand-pext-pgo.exe` | `b33d3ad` (head) | **7,601,220** | `8916725E...D958B617` |
| Baseline | `rarog-411b-base-pext-pgo.exe` | `fd21612` | **6,901,489** | `6F1592FF...3A5F92C6` |

`fd21612` is the 4.11b **section entry** — the last revision before any 4.11b
source change. Both are final-PGO pext builds from `tools/build_test.ps1`,
`rustc 1.97.1`, and both manifests record `git_dirty: false`.

## What is being gated

Every deliberate behaviour change in section 4.11b, as one dependency-complete
verdict:

- **4.11b.3** UCI and fullmove boundary repair.
- **4.11b.5 SEE king-legality, created-pin and recapture-promotion repair** —
  the dominant change, and the reason this gate exists.
- The behaviour-neutral throughput work of 4.11b.9, 4.11b.13 and 4.11b.14,
  which per the leaf does not require a separate SPRT and rides along here.

## Prior — genuinely two-sided, and the headwind is measured

This is not a candidate expected to win. Two measured facts point in opposite
directions:

**Against.** The candidate searches **+10.14% more nodes at fixed depth 13**
(6,901,489 -> 7,601,220). The repaired SEE prunes less, because the pre-repair
kernel was returning wrong verdicts that happened to prune more. AGENTS records
a measured **+7.36% tree change worth −1.49 ± 2.87 Elo**, so a +10% tree is
plausibly worth roughly −2 Elo.

**For.** RAR-M41 measured **+1.421% [+0.953%, +1.764%]** whole-search NPS, worth
about +2.8 Elo at the project's ~2 Elo per 1% NPS constant. **Caveat:** that was
measured over `1d720af..head` only, so the SEE repair's own throughput effect is
**unmeasured** and is not in that figure.

Net time-to-depth is about **8.6% worse** (1.1014 / 1.0142). Combining the two
calibrations gives a prior of roughly **−4 to +4 nElo, centred near zero**.

That is precisely an unknown-sign repair, so the bracket is **symmetric**, not
the `[0,3]` default and not a gainer bracket. RAR-S62 is the direct precedent: a
ProbCut correctness fix, gated at `[-5,5]`, which **cost** 5 Elo and accepted H0
at 4,436 games.

## Registered design

| Setting | Value |
|---|---|
| Bounds | **`[-5,5]` nElo** |
| Cap | **16,000 games** |
| Alpha / Beta | 0.05 / 0.05 |
| Time control | `3+0.03` |
| Threads / Hash | 1 / 64 MB |
| Book | paired `UHO_Lichess_4852_v1` |
| Adjudication | **none** (RAR-M17 default) |
| Concurrency | 14 with affinity |

**Sizing from RAR-M10**, `drift/game ≈ 8.3e-6 × width × (true − midpoint)`, at
width 10 and midpoint 0:

| True nElo | Games to a verdict |
|---:|---:|
| +4.5 | ~7,900 to H1 |
| +3 | ~11,800 to H1 |
| +2 | ~17,700 — beyond the cap |
| 0 | **never resolves** |
| −5 | ~7,100 to H0 |

The cap of 16,000 therefore resolves a true effect of about **2.2 nElo or
larger**. It is chosen over RAR-S62's 12,000 (which resolves 2.95 or larger)
because this prior carries real mass near zero, and the extra 4,000 games
materially reduce the chance of an uninformative stop. At RAR-M16's
no-adjudication throughput of 88.4 games/min the cap is about **3 hours**.

## Stop rule and what each outcome means

1. **H1 accepted.** The cluster is the accepted foundation for 4.12 and the
   development fingerprint updates to 7,601,220 with this verdict as evidence.
2. **H0 accepted.** The repaired pruning costs measurable strength. **This does
   not license reverting the repair.** Per the leaf: investigate the implicated
   search consumers and re-register a coherent repair; do not relax the oracle
   and do not proclaim a known-bug baseline correct. The 4.11b.5 defects are
   real and independently fixtured across 41 external cases — this gate asks
   what the *repaired* pruning costs, not whether the bug was a bug. The
   accepted fingerprint stays at the baseline's until a re-registered repair
   passes.
3. **Unresolved at the cap.** An accepted outcome and **not a pass**. It means
   the true effect is inside roughly ±2.2 nElo. RAR-S61 is the standing warning:
   a point estimate with a high LOS is not a result. RAR-S63 hit exactly this,
   stopping at +0.63 ± 6.22 over 12,000 games.

No bound, cap, book, clock or adjudication setting changes after games are seen,
and no success threshold is invented afterwards.

## Status

**Registered, not yet run.** Command handed to the maintainer; the run is a
machine-occupying job and belongs to them.
