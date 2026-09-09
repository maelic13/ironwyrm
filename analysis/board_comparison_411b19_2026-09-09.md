# Board comparison after 4.11b.19 — RAR-M44(d)

Supersedes the gap table in
[board_comparison_2026-09-09.md](board_comparison_2026-09-09.md) (RAR-M43).
Four arms, one session, zero games; board throughput only. This is the
re-measurement 4.11b.19(d) registered, run after (a) fixed the harness, (b)
removed the 520-byte return copy and (c) promoted two generator candidates.

## The control reproduced, which is what makes this session readable

RAR-M43's own control did **not** reproduce RAR-M20: the identical `ca03a46`
binary measured 0.7–6.1% faster on that day, so RAR-M43 had to be read
within-session only. Today's session does not have that problem.

| Workload | `ca03a46` today | `ca03a46` in RAR-M43 | drift |
|---|---:|---:|---:|
| legal moves | 445.83 | 450.15 | −0.96% |
| legal captures | 100.61 | 99.87 | +0.74% |
| make/unmake | 43.92 | 44.28 | −0.82% |
| threshold SEE | 49.10 | 49.01 | +0.17% |
| perft(4) startpos | 289.08 | 290.49 | −0.49% |
| two-ply simulation | 369.53 | 364.97 | +1.25% |

Every column is inside ±1.3%, so **this session and RAR-M43's are comparable**,
and the two Basilisk readings can be read against each other rather than only
within a session — Basilisk itself re-times inside ±1.2% on every column
(650.47 → 642.81 on legal moves, 404.29 → 402.91 on perft), which is the same
statement made a second way. Host busy stayed 4.8–5.7% against the recipe's 12% rejection
threshold.

## All four arms, one session, medians of three cyclic rounds (M ops/s)

| Workload | Rarog head | Rarog `ca03a46` | Basilisk | Reckless |
|---|---:|---:|---:|---:|
| legal moves | **546.01** | 445.83 | 642.81 | 345.24 |
| legal captures | **142.41** | 100.61 | 119.63 | 62.63 |
| make/unmake | **53.57** | 43.92 | 57.51 | 24.05 |
| threshold SEE | 46.68 | 49.10 | 60.41 | 41.92 |
| perft(4) startpos | **346.65** | 289.08 | 402.91 | 184.78 |
| two-ply simulation | **451.46** | 369.53 | 535.54 | 261.57 |

## The gap to Basilisk: RAR-M43's table, superseded

How much faster Basilisk is. RAR-M43's "now" column becomes the "was" column.

| Workload | RAR-M20 (`ca03a46`) | RAR-M43 | **RAR-M44(d)** | closed since RAR-M43 |
|---|---:|---:|---:|---:|
| legal moves | 44.5% | 46.2% | **17.7%** | **28.5pp** |
| legal captures | 20.9% | 26.2% | **−16.0%** | **42.2pp — Rarog is now 19% ahead** |
| two-ply simulation | 46.5% | 38.9% | **18.6%** | 20.3pp |
| perft(4) startpos | 39.2% | 38.2% | **16.2%** | 22.0pp |
| make/unmake | 31.3% | 11.8% | **7.3%** | 4.5pp |
| threshold SEE | — | 34.9% | **29.4%** | 5.5pp |

Reckless is unchanged and remains slowest in every comparable column; its
archived binary is re-timed because it costs nothing and keeps the session
self-consistent.

## What 4.11b.19 did, measured against the same `ca03a46` control

| Workload | RAR-M43 (4.11b only) | **now (4.11b + 4.11b.19)** |
|---|---:|---:|
| legal moves | −1.15% | **+22.47%** |
| legal captures | −4.16% | **+41.55%** |
| make/unmake | +17.48% | +21.97% |
| perft(4) startpos | +0.67% | **+19.92%** |
| two-ply simulation | +5.44% | **+22.17%** |
| threshold SEE | −8.07% | **−4.92%** |

The threshold-SEE line is the 4.11b.5 repair's standing cost, which RAR-M43
measured at −8.07% and RAR-E15 has already paid for at +12.12 ± 10.17 Elo. It
reads −4.92% today because (c)'s const-generic colour gave part of it back.

## Calibration against RAR-M44's directional prediction

RAR-M44 read its probe against RAR-M43's Basilisk row and predicted, for (b)
alone, "capture generation moves from 26% behind to about 11% ahead and the
legal-moves gap from 46% to about 32%". Those were labelled directional and
cross-session, and they are now measurable — but only for (b)+(c) together,
because no four-arm session was run between them. Capture generation is
**19% ahead**, not 11%, and legal moves is at **17.7%**, not 32%; (c) accounts
for part of the difference and the amount is not separable from this session.
The direction was right on both and the magnitude was understated on both.

## What the remaining gap is worth — corrected

RAR-M43 computed "about +2.9% whole-search NPS, ~+5.7 Elo" from closing
generation and make/unmake. **That arithmetic was incomplete in two ways**: it
priced only two of the four board regions, omitting SEE and the never-compared
check queries; and its generation gap was partly the harness, not the
generator. Both are corrected here.

RAR-M36's shares: generation and legality 6.556%, make/unmake 6.677%,
SEE 5.239%, check queries 5.179% — 23.65% of process time in total.

Closing the **remaining** generation (1.177x) and make/unmake (1.073x) gaps
entirely:

```
1 / [ 1 − 0.06556 − 0.06677 + 0.06556/1.177 + 0.06677/1.073 ] = 1.0146
```

**about +1.5% whole-search NPS.** Adding SEE, which this session measures at a
matched-value 1.294x and so independently reproduces RAR-M29's ~30%:

```
1 / [ 1 − 0.06556 − 0.06677 − 0.05239
      + 0.06556/1.177 + 0.06677/1.073 + 0.05239/1.294 ] = 1.0272
```

**about +2.7%.** Check queries are 5.179% and have still never been compared,
so they are not in either number and the total is a floor, not a ceiling.

At the project's recorded ~2 Elo per 1% NPS at STC that is roughly 3 to 5 Elo —
**and no Elo is claimed here; that constant is a planning figure and this
instrument measures board throughput, not playing strength.** It is far below
the measured 250–355 Elo search deficit, so 4.11b's prioritisation stands, and
there is now materially less board work on the table than RAR-M43 thought:
4.11b.19 has taken the generation gap from 46% to 18% and turned the capture
gap into a lead.

## Limits

- **Board throughput is not search speed and is not Elo.** (b)'s +2.48% and
  the (c) bundle's pending run are the only whole-search numbers here.
- Threshold SEE is comparable across engines **only** because this harness
  injects the frozen 100/300/300/500/900/20000 vector (RAR-M29). Native-value
  SEE comparisons remain superseded.
- Round range reached 7.70% on Basilisk's capture column and 9.46% on
  Reckless's SEE column, so single-column differences below roughly 5% between
  arms are not resolved. The head's own columns sat at 0.15–3.86%.
- The (c) bundle's pooled-PGO run has not been run. If it lands below its
  registered +0.5% floor, `be5c02a` and `c969ccd` are reverted and **this
  table must be re-measured**, because it would then describe code that is
  gone.

## Evidence

`tools/results/board-compare-d-20260909/` (ignored): `run_comparison.py`
reused unchanged from RAR-M43 except for the head binary path, `manifest.json`
with all twelve runs and their host-busy readings, twelve raw round files,
`run.log`. Head binary SHA-256 `5aab250f…`, built with
`RUSTFLAGS='-C target-cpu=native --cfg rarog_pext'`, no features. The three
archived peer binaries hash-match the RAR-M20 manifest: `ca03a46`
`40f8fa53…`, Basilisk `7eeaff0c…`, Reckless `449897a1…`.
Recipe: `board_benchmark_recipe_2026-09-05.md`.
