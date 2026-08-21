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

## RESULT: matched cross-engine, and the prediction held

| run | Elo | Rarog's score | games |
|---|---:|---:|---:|
| `G(0)` Rarog vs oracle, both full | **−250.77 ± 13.12** | 19.10% | 2,000 |
| `G(128)` both without LMR | **−134.75 ± 12.13** | 31.52% | 2,000 |

**Removing LMR from both engines closes 116.0 ± 17.9 Elo of the gap.**

The prediction registered before these games was 125.5 ± 15.3, derived from the
self-play pair. Measured 116.0 ± 17.9 — **0.8 sigma apart.** Two methods that
share no games and have different failure modes agree, so:

- **LMR accounts for ~46% of the deficit**, and it is now measured on a single
  head-to-head scale with no self-play inflation.
- **The self-play paired ablation is validated as an instrument.** Its scales
  WERE comparable across the two engines, which was the open question. It can
  be used for the remaining mechanisms at a quarter of the cost.

Neither run is saturated: 11.2 and 8.0 Elo per score point against 6.9 at
parity, so 1.6x and 1.2x amplification. Both sit inside the readable band that
the mask-163 run left.

## The deficit is 250.8, not 196

`G(0)` is a direct, adjudicated, 2,000-game measurement of the thing this whole
phase is chasing, and it comes out at **250.77 ± 13.12** on the current head.

PLAN quotes ~196 from RAR-O02. That row records ~205 games per pair with all
adjudication OFF, so its interval is on the order of ±40 and the two are not
strictly in conflict — but the new number is better powered by an order of
magnitude and should be the one quoted. It is also the more conservative
reading, because the current head has gained ~18 Elo since RAR-O02 (4.7c +15.56
and RAR-S70 +2.33), so the deficit should have SHRUNK.

## Where that leaves the work

One mechanism, LMR, is 46% of the deficit, measured twice. That is the first
time this project has had a build target with a number on it.

The remaining ~135 Elo is spread across the rest, and the cheap way to map it
is now the self-play form, since this run validated it: one 2,000-game pair per
mechanism at ~20 minutes each. Stockfish's own annotations give the order to
try — shallow pruning ~200, extensions ~75, futility-child ~50, nullmove ~40.

**This does not license a whole-search rewrite.** It licenses reworking ONE
mechanism against a measured 116 Elo target, which is the scoped version of the
"one sweep" hypothesis rather than the whole-engine version.

## Registered before the games: the four remaining mechanisms

Matched cross-engine, same mask on both sides, against `G(0) = 250.77 ± 13.12`.
Deficit explained by M is `G(0) − G(mask_M)`. One run each, not two: the
cross-engine form needs a single match per mechanism, where the self-play form
needs one per engine.

| mechanism | mask | SF's annotation | registered expectation |
|---|---:|---:|---|
| shallow pruning | 32 | ~200 | the only candidate that could rival LMR |
| extensions | 64 | ~75 | ⚠ ASYMMETRIC BIT, see below |
| futility-child | 2 | ~50 | modest |
| nullmove | 4 | ~40 | modest |

**Registered structural expectation.** LMR already explains 116.0 of 250.77. If
the four closures sum to much more than the remaining ~135, that is not an
error — it proves strong interaction, since each is measured with all the
others still present. The sum overshooting is the additivity check the
bisection idea assumes, and it is being taken here for free.

**Bit 6 cannot be read as "singular extensions".** On the oracle it removes
singular AND check extensions; on Rarog it removes singular only, because Rarog
has no check extension — it gained +30.75 Elo deleting one. So a matched mask-64
run ablates different things on the two sides, and a large closure there cannot
be attributed to singular. Splitting the oracle's bit 6 in two is a few minutes
of rebuild and should be done before that number is trusted.

**Razoring (bit 0) and IID (bit 4) are deliberately excluded.** Both are
annotated at ~1 Elo, and the oracle run showed 10 time forfeits in 3,000 games
— worth ~1 Elo. The forfeit noise equals the entire signal, so at `3+0.03`
these two are unmeasurable and would only produce a number that looks like
data.

## The map so far — the deficit is CONCENTRATED

All matched cross-engine against `G(0) = 250.77 ± 13.12`.

| mechanism | mask | G(mask) | deficit explained | |
|---|---:|---:|---:|---|
| shallow pruning | 32 | −126.18 ± 11.82 | **124.6 ± 17.7** | 13.8σ |
| LMR | 128 | −134.75 ± 12.13 | **116.0 ± 17.9** | 12.7σ |
| extensions | 64 | −238.93 ± 13.40 | 11.8 ± 18.8 | 1.2σ — **zero** |

**Extensions are not a lead, and that is a clean null.** Rarog's score barely
moved, 19.10% at `G(0)` against 20.18% here. It holds despite the asymmetric
bit favouring a positive reading — the oracle also loses its check extension
there, and Rarog has none — and it is consistent with Rarog having gained
+30.75 Elo by deleting its own check extension. A candidate retired for 20
minutes.

**The two selectivity mechanisms sum to 96% of the whole deficit.** That is
either the answer or a double count, and the two readings are very different
things:

- **Additive** — they are separate failures, together explaining essentially
  all of it, and `G(32|128)` would come back near zero.
- **Overlapping** — they are one selectivity failure measured twice, each
  ablation shifting the same work onto the other, and `G(32|128)` would come
  back near 120.

## Registered before its games: mask 160

One matched cross-engine run at `AblationMask=160` (bits 5 and 7 together)
separates them.

- `G(160) ≈ 0–30` → **LMR and shallow pruning explain the entire deficit.**
  Move ordering, TT, qsearch, extensions and time management would then be
  collectively worth almost nothing, and the whole build programme is one
  selectivity rework.
- `G(160) ≈ 120` → they overlap heavily. There is ~120 Elo of shared
  selectivity value and only one thing to fix, not two.

Either answer is decisive and neither is expensive. Saturation is not a risk
here: both engines lose the same mechanisms, so the score stays in the band
rather than collapsing the way the mask-163 SELF-PLAY runs did.

This also settles the "one sweep" question with evidence. The deficit is
concentrated in selectivity, so the sweep that is justified is a selectivity
rework — not a whole-search rewrite, and not the mechanism-at-a-time
transplants that went 0 for 5.

## RESULT, mask 160: the deficit IS selectivity, and nothing else is

`G(160) = **+21.39 ± 12.16 in Rarog's favour**`. Score 53.08% -- the
best-centred, most reliable run of the whole series.

**LMR and shallow-depth pruning together explain 272.2 ± 17.9 Elo**, against a
measured deficit of 250.8. That is 109% of it.

**They are near-additive, not a double count.** Sum of the parts 240.6 ± 25.2,
joint 272.2 ± 17.9, interaction **+31.6 ± 30.9** -- 2 sigma, mildly synergistic.
Two separate failures, not one seen twice. That was the question mask 160 was
registered to answer and it answered it cleanly.

## The speed confound, and what it does and does not touch

Rarog runs **2.95 Mnps against the oracle's 1.64 -- 1.80x faster**, because the
oracle calls Rarog's HCE across an FFI boundary on every evaluation. At the
usual ~60 Elo per doubling that is roughly **51 Elo of pure speed** handed to
Rarog in every one of these matches.

**It does not touch the headline.** The offset is present in `G(0)` and `G(160)`
alike, so it cancels in the difference. The 272.2 stands.

**It reverses one tempting reading.** "Rarog's non-selectivity search beats
Stockfish's by 21 Elo" is false. Net out the speed and it is roughly **30 Elo
BEHIND**. Better than expected, still behind.

**And it makes the real deficit larger.** At equal speed the gap is nearer
**~302 Elo**, of which selectivity explains 272 and everything else ~30. Those
numbers close, which is a coherence check the raw figures do not provide.

⚠ The 60-Elo-per-doubling conversion is a rule of thumb, not a measurement on
this hardware, and 1.8x is far outside the range where the project's own
"~2 Elo per 1% NPS" figure is valid (that would give 160 Elo and is plainly an
extrapolation failure). Treat ~51 as an estimate with real uncertainty; the
sign and rough magnitude are what matter here.

## What this settles

1. **The entire build programme is a selectivity rework.** LMR and shallow-depth
   pruning, both of them, separately.
2. **Nothing else is worth touching for strength.** Move ordering, TT,
   quiescence, extensions, null move, futility, aspiration and time management
   are collectively ~30 Elo, and that bucket is measured, not assumed.
3. **The remaining registered runs are cancelled.** Futility-child (mask 2) and
   nullmove (mask 4) were ON during the mask-160 run, so they sit inside that
   ~30 Elo bucket and cannot individually be large. Two runs saved, and the
   reason is a measurement rather than a guess.
4. **Extensions were retired for 20 minutes**, and the whole 4.6 answer-led
   programme -- which spent this phase on quiescence, TT bound composition and
   opponent-worsening -- was aimed at that ~30 Elo bucket the entire time.

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
