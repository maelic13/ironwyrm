# Rarog endgame conversion and recogniser audit — 2026-09-01

## Verdict

Rarog is **not** ahead of current Basilisk at bare-king conversion. On the same
instrument class — 100 fixed-seed random legal positions per family, engine on
both sides, 60,000 nodes/move, persistent TT per game, 100-ply rule-50 horizon —
Rarog is worse in every measured family and catastrophically weak in KBNK.

| Family | Rarog | Basilisk latest | Rarog dominant failure |
|---|---:|---:|---|
| KQ-K | 94% | 100% | 4 rule-50, 2 stalemate |
| KR-K | 86% | 100% | 13 rule-50, 1 other |
| KBB-K | 76% | 87% | 24 rule-50 |
| KBN-K | **15%** | **54%** | **77 rule-50, 4 stalemate, 4 other** |

Raw Rarog artifact:
`tools/results/hce-confirm-20260831_230548/endgame-conversion-source.json`.
Recipe:

```powershell
python tools/diag/endgame_conversion.py `
  --engine tools/test_engines/rarog-hce-confirm-20260831_230548-pext-pgo.exe `
  --positions 100 --nodes 60000 --max-plies 100 `
  --output tools/results/hce-confirm-20260831_230548/endgame-conversion-source.json
```

## Re-baselined on the accepted HCE (RAR-E06), 2026-09-01

The table above measured the **pre-refit source**. Re-run on the accepted
candidate with the same seed (6200600), the same 100 positions per family and
the same 60,000-node / 100-ply policy, so the two runs are paired at the
position level:

| Family | Pre-refit | Accepted HCE | Delta | Basilisk latest |
|---|---:|---:|---:|---:|
| KQ-K | 94% | **94%** | +0 pp | 100% |
| KR-K | 86% | **91%** | +5 pp | 100% |
| KBB-K | 76% | **86%** | +10 pp | 87% |
| KBN-K | 15% | **19%** | +4 pp | 54% |

Accepted-HCE failure modes: KQ-K 5 rule-50 + 1 stalemate; KR-K 9 rule-50;
KBB-K 13 rule-50 + 1 stalemate; **KBN-K 73 rule-50, 5 stalemate, 3 material
lost**.

**Read this cautiously.** At n=100 per family the binomial standard error is
about 3.5 pp, so only the KBB-K movement is comfortably outside noise; the
KR-K and KBN-K deltas are suggestive at best. The runs share positions, which
makes the comparison stronger than two independent samples, but the aggregate
JSON does not retain per-position outcomes, so no paired test is possible from
these artifacts. If 4.9a.1 needs a resolved answer it must record per-position
results and use a paired test, not more aggregate runs.

What is not ambiguous is the shape. **KBN-K is still catastrophic at 19%**, and
73 of its 100 games still die on the fifty-move rule. This is the prediction
below holding up: a complete recalibration of every coefficient moved KBN-K by
about one standard error, because the defect is gradient magnitude against the
pruning margins that consume it, not coefficient calibration. Fitting cannot
repair a term whose actionable signal is smaller than the margins it must
survive. Raw artifact:
`tools/results/hce-accepted/endgame-conversion-accepted.json`.

## Syzygy truth corpus, accepted HCE (4.9a.1), 2026-09-01

`tools/diag/endgame_truth.py`, 100 positions per family, 60,000 nodes/move,
100-ply horizon, seed `0x5E9D18`, engine `SyzygyPath` cleared so this measures
the evaluation and not the tables. Artifact:
`tools/results/hce-accepted/endgame-truth-accepted.json` (per-position records
retained, so a later run over the same seed is paired).

Conversion is over the **theoretically won** subset only. `won` is how many of
the 100 generated positions Syzygy calls a clean win.

| Family | won | conv | eff | dtz prog | win-preserving | graded moves |
|---|---:|---:|---:|---:|---:|---:|
| KQ-K | 100 | 95.0% | 1.22 | 0.496 | 0.9983 | 1,167 |
| KR-K | 100 | 96.0% | 1.25 | 0.571 | 0.9994 | 1,720 |
| KBB-K | 100 | 73.0% | 1.50 | 0.400 | 0.9967 | 2,759 |
| KBN-K | 99 | **7.1%** | 1.58 | 0.277 | 0.9969 | 4,818 |
| KNN-K | **0** | n/a | n/a | n/a | n/a | 0 |
| KP-K | 74 | 94.6% | - | 0.378 | 1.0000 | 1,358 |
| KPP-K | 98 | 77.6% | - | 0.078 | 1.0000 | 756 |
| KBP-K | 96 | 91.7% | - | 0.301 | 1.0000 | 1,029 |
| KR-KP | 88 | 93.2% | - | 0.523 | 0.9982 | 1,699 |
| KR-KB | 35 | 94.3% | - | 0.631 | **0.9625** | 586 |
| KR-KN | 54 | 83.3% | - | 0.497 | **0.9726** | 1,423 |
| KQ-KP | 99 | 96.0% | - | 0.475 | 0.9992 | 1,211 |
| KQ-KR | 100 | 83.0% | - | 0.391 | 0.9984 | 1,912 |
| KNN-KP | 26 | **15.4%** | - | 0.421 | **0.8088** | 1,119 |
| KRP-KR | 67 | **52.2%** | - | 0.319 | 0.9863 | 584 |
| KRP-KB | 98 | **56.1%** | - | 0.160 | 0.9992 | 1,184 |
| KBP-KB | 47 | 80.9% | - | 0.367 | 1.0000 | 518 |
| KBP-KN | 57 | 78.9% | - | 0.369 | 0.9956 | 681 |
| KP-KP | 32 | 93.8% | - | 0.374 | 1.0000 | 583 |

`eff` (plies taken / optimal DTZ, median, paired per position) is reported only
where the weak side is bare and the strong side pawnless, so DTZ equals DTM.
`dtz prog` is a valid comparison between two engine versions in every family
but only reads as *technique* under that same condition -- elsewhere it counts
progress toward the next pawn push or capture, which is why KPP-K shows 0.078.

### What this says that the conversion runner could not

**KRP-KR is the high-value target, not KBN-K.** It converts **52.2%**, and
RAR-M15 measured it occurring in **10.04%** of real games against KBN-K's
**0.28%** -- 36x more often. A KRP-KR improvement is inside tier 1 and a
normal SPRT can see it; a KBN-K improvement is tier 3 and no whole-match SPRT
ever will. KRP-KB is the same shape: 56.1% conversion, 1.23% occurrence.

**The win-preserving rate finds defects the conversion rate hides.** KNN-KP
discards a clean theoretical win on **19.1%** of graded moves (0.8088 over
1,119 moves), and KR-KB and KR-KN sit at 0.9625 and 0.9726. All three are
reference functions Rarog does not implement at all (inventory items 2, 7, 8).
In the bare-king families the metric saturates near 1.0 -- the engine rarely
throws a win away there, it simply never finishes -- so the two metrics are
complementary and neither is sufficient alone.

**KBN-K's defect is localized, not diffuse.** Efficiency is 1.58 on the
positions it converts, so its technique when it works is within 60% of optimal;
the failure is that DTZ progress runs at 0.277, the lowest of any pawnless
family, so it shuffles without approaching the zeroing move and dies on the
fifty-move rule.

**KNN-K is theoretically drawn in 100 of 100 positions**, which is correct
chess and validates the theory gate: the tool assigned it zero graded moves
rather than scoring the engine for failing to win a draw. It belongs in the
drawn-subset cohort, not the conversion cohort.

### Do not difference these against the earlier tables

This run and the conversion-runner runs above use **different position
generators**, so their samples differ and the numbers are not paired. KBN-K
reads 7.1% here and 19% there; KBB-K reads 73% against 86%. At n=100 that is
2-3 standard errors and is consistent with sampling, but it is not evidence of
a change and must not be reported as one. Only two `endgame_truth.py` runs over
the same seed are comparable, and those compare per-position.

## Same defect as Basilisk, directly visible in source

Rarog's KBNK drive uses Chebyshev corner distance and gradients of only **8 cp
per corner step and 4 cp per king-distance step**. Its generic KXK drive uses
**5 and 4 cp/step**. These are below the 100–500 cp pruning margins they must
survive. The KBNK term also has no bishop/knight proximity or escape-control
gradient. Basilisk measured this exact signature: Chebyshev plateaus plus
sub-pruning gradients made a present and correctly oriented recogniser fail to
steer search. Replacing the plateaued geometry and raising the actionable
gradient took Basilisk KBNK from 13.0% to 54.5%; it is still unfinished there.

Rarog's current hand-picked test stays green because it contains one KBNK
mate-in-one and a small set of near-corner routes. It tests direction and local
mate recognition, not class-wide conversion. A recogniser being present,
wired, and directionally correct therefore does not establish maturity.

## Gradient audit (4.9a.4): the drive has no gradient, not a small one

The claim above -- "gradients of only 8 cp per corner step and 4 cp per
king-distance step ... below the 100-500 cp pruning margins" -- is true but
frames the defect wrongly, and the wrong frame implies the wrong fix.

Measured over **300 theoretically won positions per family**, replaying the
exact source formula across every legal move:

| Family | legal moves | distinct mop-up values | best-vs-second gap | **best move TIED** | term span |
|---|---:|---:|---:|---:|---:|
| KBN-K | 19 (median) | **3** | median **0 cp** | **94%** | 8 cp |
| KR-K | — | 2 | median **0 cp** | **97%** | 4 cp |
| KQ-K | — | 2 | median **0 cp** | **96%** | 4 cp |

Depth-1 quiet futility margin, for scale: `fp_base + fp_coeff` = **346 cp**.

Two things follow, and the second is the one that matters.

**The whole term is smaller than the smallest pruning margin.** Its entire
range across all legal moves is 4-8 cp against a 346 cp margin at depth 1.

**But raising the magnitude would have fixed nothing.** In 94% of won KBNK
positions the best move is *tied*, with a median best-vs-second gap of **0 cp**
-- 19 legal moves collapse into 3 distinct scores, 7.6 moves per score. A term
that cannot order its own moves cannot steer a search at any magnitude:
40x zero is zero. The defect is Chebyshev distance, which is flat by
construction -- every square in a ring around the target scores identically.

The fix is therefore resolution first and magnitude second: combine Chebyshev
with Manhattan so the rings break. On the same 300 positions that takes the
tied-best rate from **94% to 11%** and distinct values from 3 to 6. The
residual 11% is mostly bishop and knight moves that do not move either king,
which a king-distance metric cannot order and arguably should not.

Adding explicit minor-piece coordination terms was tried and **rejected on
measurement**: with weights small enough not to dominate, different
(king, knight, bishop) combinations alias to equal sums, and the tied-best rate
went *up*, to 19-28%. Coordination may still be worth having for play quality,
but it must be justified by conversion rate rather than by this proxy.

## Drawn-subset measurement

The other Basilisk correction also reproduces. Frequency and whole-class mean
loss are the wrong instruments for scaling functions: decisive positions can
hide systematic overconfidence on the drawn subset. On 127,778 phase-balanced,
previously unused pure-WDL positions from the fresh confirmation games, the
accepted Rarog source predicts the following strong-side win probabilities on
positions whose games actually ended drawn:

| Exact material class | Draw n | Predicted | Bias from 0.500 |
|---|---:|---:|---:|
| KRPP-KRP | 608 | 0.625 | +0.125 |
| KRP-KR | 626 | 0.666 | +0.166 |
| KR-KP | 301 | 0.704 | +0.204 |
| KBP-KB | 619 | 0.626 | +0.126 |
| KBP-KN | 150 | 0.719 | +0.219 |
| KR-KB | 157 | 0.674 | +0.174 |
| KR-KN | 108 | 0.737 | +0.237 |
| KBPP-KB | 55 | 0.792 | +0.292 |
| KQ-KR | 20 | 0.819 | +0.319 |
| KBN-K | 13 | 0.843 | +0.343 |
| KNN-KP | 10 | 0.753 | +0.253 |

The complete fitted candidate does not remove the pattern. Some rows improve
slightly and others worsen; KBNK is unchanged. These are game-result labels,
not theoretical verdicts, and datagen adjudication can end games before theory
does. They select the next **measurement**, not a hardcoded rule. Step 4.9a
must re-measure candidate classes against Syzygy WDL/DTZ and no-adjudication
endgame-start play before implementation.

## Reference inventory: 20, not 18

The final Stockfish HCE-era `endgame.h` carries **20** specialized functions:
10 value functions and 10 scale functions. Stockfish 11 carried 22; `KNPK` and
`KNPKB` were subsequently removed. Current NNUE Stockfish and Reckless no
longer provide a comparable HCE dispatcher, so the final pre-NNUE Stockfish
table is the correct reference.

| Value functions | Scale functions |
|---|---|
| KNNK, KNNKP, KXK, KBNK, KPK | KBPsK, KQKRPs, KRPKR, KRPKB, KRPPKRP |
| KRKP, KRKB, KRKN, KQKP, KQKR | KPsK, KBPKB, KBPPKB, KBPKN, KPKP |

Rarog has meaningful coverage of **7/20**: KNNK, KXK, KBNK, KPK, a narrow
KRKP partial scaler, narrow KQKP fortress handling, and a narrow wrong-corner
subset of KBPsK. OCB and generic insufficient-material handling are useful
extras but do not replace the missing material functions.

## Test policy carried into step 4.9a

1. Separate theoretical classification, static direction, conversion, and
   strength. No one instrument owns all four.
2. Use fixed-seed random family floors with a node budget and persistent TT;
   record stalemate, rule-50, material-loss and mate outcomes separately.
3. Keep near-mate and exact-theory cases as hard correctness tests. Treat long
   fixed-search trajectories as diagnostics plus aggregate floors, not sacred
   per-position vetoes.
4. Use Syzygy WDL/DTZ for exact ≤7-piece verdicts. A game that happened to draw
   is only statistical evidence and must not become a per-position assertion.
5. Tighten an aggregate floor after an accepted improvement. Do not leave the
   old floor behind, and do not relax correctness in the implementation commit.
6. Gate dependency-complete families, not sub-Elo functions one at a time.
7. Audit gradient magnitude against the pruning margins that consume it.
   Texel loss can fit a multi-move guidance term toward zero even when search
   needs a larger actionable gradient; passed-pawn king approach is the first
   non-mate family to check.

## Additional Basilisk endgame-programme disposition

Basilisk's remaining useful method additions are already owned elsewhere in
Rarog: post-HCE qsearch/TT/extension authority is Phase 4.11; STC/LTC/4T,
portability and release transfer are 4.15; high-thread Elo scaling is 8.0.
They should not be duplicated in 4.9a. The missing item was the explicit
endgame conversion/recogniser program and its systematic gradient-magnitude
audit; those are step 4.9a, inside Phase 4, before the 2.4.0 release gate.
