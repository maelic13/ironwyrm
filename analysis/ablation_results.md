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

## 4.5 progress: the seeded LMR contract, registered before its run

Seeds derived from **Rarog's own scale**, not the reference's constants — its
existing LMR adjustments sit at 656–887 (`LmrTtPvAdj` 887, `LmrCutNode` 780,
`LmrShallowTt` 656), and `LmrHistDiv` 8395 means ~8,400 history units is worth
one ply, with 4,000 already used in-code as the "good history" threshold.

| parameter | seed | where the number comes from |
|---|---:|---|
| `LmrTtCapture` | 780 | same order as `LmrCutNode`; both add reduction |
| `LmrSingularRelief` | 780 | same order, opposite sign |
| `LmrParentMoveCountRelief` | 780 | same order |
| `LmrParentMoveCountMin` | 12 | ⚠ **judgement, the weakest of the five.** Rarog's first-move cutoff rate is 87.65%, so a parent still searching at move 12 is genuinely late — but this is not derived from a measurement and is the first thing 4.8 should fit |
| `LmrStatSwing` | 780 | same order |
| `LmrStatSwingMargin` | 4000 | the "good history" threshold already in `search.rs` |
| `LmrMinReducedDepth` | 1 | the 4.8.1 audit: 46.7% of reductions were landing in quiescence |

Bench 12 sanity, baseline 5,073,397 / EBF 2.585 — magnitudes are in range and
nothing is pathological:

| arm | nodes | EBF | ratio |
|---|---:|---:|---:|
| `LmrTtCapture` | 4,405,131 | 2.555 | 0.868 |
| `LmrSingularRelief` | 4,918,372 | 2.584 | 0.969 |
| `LmrParentMoveCount` | 4,813,291 | 2.572 | 0.949 |
| `LmrStatSwing` | 4,120,931 | 2.546 | 0.812 |
| `LmrMinReducedDepth` | 4,310,483 | 2.558 | 0.850 |
| **all together** | **4,332,680** | **2.555** | **0.854** |

⚠ **This is a sanity check, not a fit.** Node count is not Elo — a measured
+7.36% tree change was once worth −1.49 ± 2.87 Elo in this project. The
numbers say only that the seeds are the right order of magnitude.

**The measurement that decides it is `G(0)`**, and it is registered here before
the games. Baseline **250.77 ± 13.12**, measured with this same binary at
defaults, which still benches 6,977,070 so the baseline transfers unchanged.

- `G(0)` **drops** → the contract is capturing part of the 116 Elo, and by how
  much is read directly off the drop.
- `G(0)` **flat** → the mechanisms are present but the seeds are wrong, and
  4.8 has a surface worth fitting.
- `G(0)` **rises** → the contract is worse than what it replaced, and the
  cluster is reverted rather than tuned.

`G(128)` is the CONTROL and must not move: with LMR ablated on both sides, none
of these parameters is reachable, so a change there means the ablation is not
removing what it claims to.

## 4.5 seeded contract: FLAT, and one term was mis-specified

`G(0)` with all seven seeds: **−246.58 ± 13.13** against the baseline
**−250.77 ± 13.12**. Gap closed **+4.19 ± 18.56 Elo**, 0.4 sigma. The whole
95% interval sits below +23, against a 116 Elo target.

**The wire was live.** The manifest records all seven options, fastchess
rejected none, every parameter was proved to move the tree by direct UCI probe
before the run, and the combined seeds move bench 12 by 15%. This is a real
null, not a third dead instrument.

### Why flat is the EXPECTED result here, in hindsight

The four terms do not push the same way, and the combination was dominated by
the two that ADD reduction — `LmrTtCapture` (bench x0.868) and `LmrStatSwing`
(x0.812) — while the two reliefs barely moved anything. Net, the candidate is a
**broad de-selectivity shift in the reducing direction**, and this project has
now measured that class three times:

- RAR-S54: +4.06 ± 3.71 (a blind uniform 15% shift, against the 2.3.1 head)
- RAR-S68: **−1.40 ± 6.24** (the directional form, on the current head)
- here: **+4.19 ± 18.56**

All three are flat within their intervals. A broad shift of Rarog's selectivity
is worth approximately nothing, and that is now a settled result rather than a
suspicion.

### `LmrStatSwing` is fed the wrong quantity

Three terms are faithful to the reference's mechanism. This one is not, and the
error is the INPUT rather than the constant:

- The reference compares a specific composite — main history plus three
  continuation histories, minus a fixed offset — against **absolute thresholds
  near zero**, in both directions. It selects a narrow transition: *my move
  looks acceptable and the move that led here looked bad*, or the reverse.
- What I built compares Rarog's `quiet_history` **difference** against a
  symmetric margin. And `quiet_history` is not that composite: it is the full
  move-ORDERING score — main, pawn, low-ply, continuation, plus the check-class
  bonuses that `quiet_history_score` itself describes as *enormous*.

So the term fires on a much wider distribution than the mechanism intends, at a
margin (4,000) chosen for a scale it does not have. That makes it a broad shift
wearing a transition detector's name, which is exactly the class the three
results above price at zero.

Rarog's existing continuous term, `r -= quiet_hist * 1024 / lmr_hist_div`, uses
the same composite — but `lmr_hist_div` was SPSA-fitted **for** that scale. The
new term was not.

### What this does and does not say about 4.5

It does NOT say the contract is wrong. It says one first guess at a
six-dimensional point, seeded by analogy, is worth zero — which was registered
in advance as the "flat" outcome, and is the least surprising of the three.

The combined run cannot say which term carries sign, and that is the next
measurement: **one `G(0)` per term, ~20 minutes each.** At that price the
one-at-a-time form is affordable, and it is the only thing that separates "this
mechanism does not transfer to Rarog" from "this mechanism needs a different
constant".

`LmrStatSwing` is excluded from that sweep until its input is rebuilt on a
statScore-shaped composite. Fitting a mis-specified term would only find the
constant that best hides the misspecification.

## Per-term runs are flat too — and they were the wrong measurement

| term | gap closed | games |
|---|---:|---:|
| `LmrTtCapture=780` | −2.0 ± 23.7 | 880 |
| `LmrMinReducedDepth=1` | −12.7 ± 26.5 | 678 (stopped) |

### 1. The instrument was used below its resolution. That is my error.

At ~800 games a single arm carries ±20–26 Elo. An individual LMR term is worth
5–15 Elo at best, so these runs could not have detected one. Resolving ±5 Elo
needs ~13,800 games per arm — 2.3 hours each, five terms, for numbers that a
registered gate would produce more honestly.

`PROCESS.md` already says this: *"Mechanisms under ~10 Elo are NOT measurable
this way at 3+0.03."* I wrote that rule and then proposed exactly that sweep.
The matched-ablation instrument is for 40–250 Elo effects. It does not scale
down, and no further per-term runs should be made.

### 2. Rarog's LMR was ALREADY the reference's contract

The base reduction formula, converted to the same units:

    reference   0.521 + 0.468 · ln(depth) · ln(moveCount)
    Rarog       0.631 + 0.439 · ln(depth) · ln(moveCount)   (SPSA-fitted)

At depth 20, move 10: **3.75 plies against 3.66.** The same functional form, the
same magnitude, already locally fitted. Rarog also already had the two largest
adjustments — the continuous history term (~30 Elo in the reference) and the
tt-pv and cut-node terms (~10 each).

**So there was no contract to replace.** 4.5.1's premise — that Rarog's LMR is
structurally behind — is false, and that is why adding the remaining ~23 Elo of
small adjustments measures zero. The refactor into `ReductionInputs` was worth
having; the hypothesis behind it was not.

### 3. What the 116 Elo actually is

`G(0) − G(128) = 116` says the LMR mechanism has three times the marginal value
inside the oracle that it has inside Rarog. It does **not** say Rarog is missing
116 Elo of LMR, and this document previously called it a "target", which
oversold it.

The likely cause is now visible and does not involve the reduction formula at
all. At 300k nodes:

| | depth reached | effective EBF |
|---|---:|---:|
| Rarog | 24.68 | **1.667** |
| oracle | 28.86 | **1.548** |

Same reduction formula, and Rarog's other pruning fires HARDER (`rfp_cut` 1.41x,
`razor_drop` 1.65x) — yet its tree grows 7% faster per ply, costing it four
plies at equal nodes. A reduction is worth more in a deeper search, so the
oracle's identical formula is simply applied deeper. The 4.6 audit already
measured a candidate cause: Rarog runs **1.60x** the oracle's quiescence per
node.

### 4. The discriminating measurement

**Run `G(0)` at fixed NODES per move instead of a clock.** `sprt.ps1 -Nodes`
exists for exactly this and is documented as a cross-engine search-accuracy
question, never a gate.

- Gap **collapses** at equal nodes → Rarog's deficit is tree efficiency, not
  decision quality. It reaches the same conclusions when given the same tree
  budget, and the work is in what its nodes are spent on — quiescence first.
- Gap **stays near 250** → Rarog's decisions are genuinely worse at equal
  budget, and tree efficiency is a side issue.

One 20-minute run separates the two, and it is the difference between spending
the phase on quiescence and spending it on decision quality.

## Fixed nodes: the deficit is 355, not 251 — and my discriminator was backwards

| condition | `G(0)` |
|---|---:|
| fixed TIME, `3+0.03` | −250.77 ± 13.12 |
| fixed NODES, 300k | **−355.26 ± 27.03** |

**The framing I registered was wrong.** I wrote that a gap collapsing at equal
nodes would mean tree efficiency, and a gap staying near 250 would mean worse
decisions. That is backwards: if Rarog converts nodes to depth less
efficiently, taking away its speed makes it WORSE at equal nodes, so the gap
must grow. Both of my named outcomes were compatible with one hypothesis and
the test could not discriminate. It still produced the two most useful numbers
of the day.

### Rarog's speed is worth 104.5 ± 30.0 Elo, measured

The difference between the two conditions is the whole value of Rarog's 1.80x
NPS advantage at this time control. My earlier rule-of-thumb conversion — ~60
Elo per doubling, giving ~51 — was **low by a factor of two**. Every
speed-adjusted figure in this document that used ~51 should read ~105.

This is a real asset and it is already banked. It is also the reason the
fixed-time deficit looked smaller than it is.

### The true equal-budget search deficit is ~355 Elo

Everything this phase has quoted — including PLAN's headline — is the
fixed-TIME number, which flatters Rarog by ~105 Elo of speed. Against 355 the
decomposition re-bases:

| mechanism | Elo | share of 251 | share of **355** |
|---|---:|---:|---:|
| shallow-depth pruning | 124.6 | 50% | **35%** |
| LMR | 116.0 | 46% | **33%** |
| both together | 272.2 | 109% | **77%** |

⚠ Those ablation numbers were themselves measured at fixed TIME, so they carry
the same confound. Re-measuring the two big ones at fixed nodes is the way to
get a decomposition free of it.

### Depth alone does not explain it

At 300k nodes Rarog reaches 24.68 plies and the oracle 28.86 — 4.18 plies. For
depth to account for 355 Elo it would have to be worth 85 Elo per ply, which is
far above any plausible figure at this depth. **So the oracle is better than
"Rarog plus four plies."** Its advantage is not only that it searches deeper on
the same nodes; the tree it builds is better at every depth.

That is a harder problem than tree efficiency, and it is the honest reading.

### What it means for 4.5, and what it does not

It does not revive 4.5. Rarog's LMR is still contract-equivalent to the
reference's, and the small terms still measure zero. What changed is the size
of the prize and where it is likely to sit: not in the reduction formula, and
not merely in reaching the same depth.

Next measurement, and it is cheap: **re-run the two large ablations at fixed
nodes.** If LMR and shallow pruning still explain ~77% of the deficit with the
speed confound removed, selectivity remains the answer and the question becomes
what INPUTS make the same formula worth three times more elsewhere. If their
share collapses, the mass is somewhere the fixed-time measurement hid — with
quiescence, at 1.60x the oracle's per node, the first candidate.
