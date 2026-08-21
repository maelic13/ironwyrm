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

## The selectivity family (mask 163) — SATURATED, and it decides nothing

| | delta | score | games |
|---|---:|---:|---:|
| oracle | −534.80 ± 27.11 | 4.40% | 2,000 |
| Rarog | −430.27 ± 22.37 | 7.75% | 2,000 |

Gap 104.5 ± 35.1. **This number should not be used.** Both ablated arms are
being crushed, and in that tail the Elo scale amplifies: 30.8 Elo per score
point at 6%, against 6.9 near parity, a **4.4x** amplification. The reported
104.5 Elo is a **3.35 percentage-point** score difference between two engines
that have both collapsed. It measures how a no-pruning engine dies, not what
the mechanism is worth.

That was a design error in this document's previous section: removing four
mechanisms at once does not produce a weaker engine, it produces a broken one.
Ablation deltas are only readable while the ablated arm stays inside a
measurable band, roughly 20–80%.

## The design that replaces it: cross-engine at MATCHED ablation

Stop comparing each engine to itself. Play **Rarog against the oracle** with
the SAME mask on both sides, and watch how the gap between them moves.

    G(mask) = Elo(oracle at mask) − Elo(Rarog at mask)

`G(0)` is the deficit itself. If a mechanism explains part of it, removing that
mechanism from BOTH engines shrinks the gap by the amount the oracle's version
was outperforming Rarog's.

This fixes both defects of the self-play form at once. There is no self-play
inflation, because every number is one head-to-head Elo on a single scale. And
there is no saturation, because both sides are equally crippled and the score
stays near where it already was instead of collapsing to 4%.

**It also makes the LMR result falsifiable.** The self-play deltas say the
oracle's LMR outperforms Rarog's by 125 Elo. If that is real and the two
self-play scales are comparable, then removing LMR from both should shrink the
gap by ~125:

    G(0)   ≈ 180   (to be measured on the current head; the ~196 figure
                    predates 4.7c's +15.56 and RAR-S70's +2.33)
    G(128) ≈ 55    PREDICTED, if LMR explains the 125

If `G(128)` comes back near `G(0)`, the 125 Elo was an artifact of comparing two
self-play scales and there is no LMR headroom. If it comes back near 55, the
localisation holds and it is measured on one scale with no saturation.

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
