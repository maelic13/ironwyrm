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
does. They select the next **measurement**, not a hardcoded rule. Phase 5 must
re-measure candidate classes against Syzygy WDL/DTZ and no-adjudication
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

## Test policy carried into Phase 5

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

## Additional Basilisk Phase-5 disposition

Basilisk's remaining useful method additions are already owned elsewhere in
Rarog: post-HCE qsearch/TT/extension authority is Phase 4.11; STC/LTC/4T,
portability and release transfer are 4.15; high-thread Elo scaling is 8.0.
They should not be duplicated in Phase 5. The missing item was the explicit
endgame conversion/recogniser program and its systematic gradient-magnitude
audit; those now lead Phase 5 before the NNUE runway.
