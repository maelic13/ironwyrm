# Paired ablation, results

Method and caveats: `analysis/ablation_design.md`. Both engines run Rarog's
HCE, `3+0.03`, 1T, 64 MB, paired UHO, RAR-M13 adjudication. Each delta is
self-play: the ablated arm against the SAME binary at `AblationMask=0`.

## LMR (bit 7)

| | delta | games | Stockfish's own annotation |
|---|---:|---:|---:|
| oracle | **−188.39 ± 9.90** | 3,000 | ~200 Elo |
| Rarog | **−62.89 ± 11.66** | 1,720 (stopped early) | -- |

**Difference 125.5 ± 15.3 Elo, 16 sigma.** Rarog's LMR is worth **33%** of what
the same mechanism is worth to the oracle.

Stockfish's published ~200 for LMR reproduced at 188 under Rarog's evaluation,
first attempt. That is the instrument validating itself, and it means the
other annotations can be used as a prior for ordering.

## What this does NOT yet establish

125 Elo is a gap in MARGINAL value. Three explanations fit it and only one is
headroom:

1. **Headroom.** Rarog's LMR really is a weaker mechanism and there is Elo in
   fixing it.
2. **Redistribution.** Rarog prunes harder elsewhere -- the differential
   measured `rfp_cut` 1.41x and `razor_drop` 1.65x against the oracle -- so
   when LMR is removed, other pruning already covers much of the same ground.
   Total selectivity would then be comparable and differently distributed,
   and there is no 125 Elo to collect.
3. **Operating depth.** The oracle reaches depth 28.9 at 300k nodes against
   Rarog's 24.7. LMR is worth more the deeper you search, so part of the gap
   may be where each engine sits rather than how each implements it.

A fourth, smaller caveat: each delta is self-play against its own baseline, and
self-play Elo is inflated relative to a common pool by an amount that need not
match across two engines. Putting both ablated arms against one common
reference would remove that, at the cost of more runs.

## The discriminating experiment

Ablate the whole selectivity FAMILY on both engines -- razoring, futility-child,
shallow-pruning and LMR together, `AblationMask=163` (bits 0+1+5+7).

- If Rarog's family delta ≈ the oracle's, explanation 2 wins: the selectivity
  is there, distributed differently, and LMR-shaped work buys nothing.
- If Rarog's family delta is still far below, explanation 1 survives and the
  headroom is real.

Running the complement, `AblationMask=92` (nullmove, probcut, iir, extensions),
costs one more pair and gives the additivity check that any bisection over
these mechanisms silently assumes: if effects were additive, the two family
deltas would sum to the delta of removing all eight. The gap between them is
the interaction, measured rather than assumed.

## Bit-assignment asymmetry to remember

Bit 6 is not symmetric. On the oracle it removes singular AND check extensions;
on Rarog it removes singular only, because Rarog has no check extension -- it
gained +30.75 Elo removing one. Any bit-6 comparison must state this.

## Measurement floor

10 time forfeits in the 3,000-game oracle run (7 ablated, 3 full). Worth ~1 Elo
against a 188 Elo effect and irrelevant there. But razoring and IID are
annotated at ~1 Elo each, so forfeit noise EXCEEDS the signal for the small
mechanisms at this time control. Those need fixed nodes or a longer TC, and
should not be run at `3+0.03` as-is.
