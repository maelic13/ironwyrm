# Rarog development plan

Updated 2026-09-01. This is the current roadmap. Historical evidence belongs
in `EXPERIMENTS.md`; current status and commands belong in `GUIDE.md`.

## 1. Current state

| Item | State |
|---|---|
| Released baseline | **2.3.2** at `f931722` on `master` |
| Accepted head | RAR-E06 on `dev`; `bench 13` = **7,226,051 nodes / EBF 2.460**, 1T. RAR-S70's 6,977,070 / 2.466 is the previous head |
| Integration state | The failed SearchCore rewrite is reverted by `c5e451d`; `d2c7788`/`e4f10ca` upgrade search evidence and `8d8f507` supplies the audited complete HCE fitting pipeline without changing accepted behavior |
| Frozen search/HCE oracle | `hybrid` at `75d0d43`, Stockfish `9587eeeb` driving the exact Rarog 2.3.2 HCE |
| Measured search deficit | **355.26 +/- 27.03 Elo at equal nodes** and **250.77 +/- 13.12 Elo at equal time**; Rarog's speed is worth a measured **104.5 Elo** |
| Accepted Phase-4 gains | ProbCut move filtering **+15.56 +/- 10.02 Elo**; root LMR relief **+2.33 +/- 1.85 Elo**; complete HCE refit **+22.04 +/- 7.51 Elo** |
| Active game job | none; RAR-E06 accepted 2026-09-01 at **+22.04 +/- 7.51 Elo**, +32.05 nElo |
| Current step | **4.9 / 4.9a — structural residuals and endgame closure** |
| HCE state | Completely refitted and accepted. The 1,218-slot surface has one whole-surface game verdict; structural gaps (4.9) and endgame closure (4.9a) remain open |
| Next release | Conditional **2.4.0** after 4.15; baseline NNUE then targets **2.5.0** |

Phase 4 remains a bounded pre-NNUE programme because the hybrid established
large search and HCE populations worth investigating. It is not a commitment
to keep working until the oracle is matched. Each dependency-complete cluster
must earn its own continuation.

## 2. Operating and evidence rules

`AGENTS.md` is authoritative for measurement, verification, documents and
gating. The following rules determine this roadmap's order.

1. Similarity to Stockfish is never an objective or acceptance criterion.
   Reference code identifies problems, dependencies and useful tests; Rarog
   implements its own answer and games decide.
2. Cross-engine ablation measures **marginal value inside each co-adapted
   engine**. It may rank questions, but it is not portable headroom and family
   losses must not be summed.
3. A strength candidate starts from the latest accepted head, is registered in
   `EXPERIMENTS.md` before games, and ends accepted or reverted before another
   candidate opens.
4. The normal unit is one smallest dependency-complete, locally fitted
   cluster. Node count, EBF, tactical suites, fit loss and oracle agreement
   explain or refute a candidate; only a clean final-PGO SPRT accepts it.
5. Default gain bounds are `[0,3]` nElo. Use wider, symmetric or
   simplification bounds only when the prospective prior and RAR-M10 sizing
   justify them. Never change the gate after seeing games.
6. Search SPSA is conditional on live coordinates, interaction and local
   curvature. HCE Texel fitting owns traced linear coefficients. HCE SPSA owns
   only activated nonlinear/global residue that the linear trace cannot fit.
7. Search and HCE coordinates never share a tune. After an HCE changes,
   cp-valued search consumers are audited and, if justified, fitted separately.
8. **Every strength A/B and cohort runs with adjudication off** (maintainer
   decision, 2026-09-01, on RAR-M16). Playing games out costs about 10% wall
   time; adjudication destroys 52.7% of all endgames before they are reached
   and priced at ~74 Elo as a cross-evaluator confounder. `-Adjudicate` opts
   back in and must be justified in the registration: wall time genuinely
   binding, and a change that provably cannot touch conversion or defensive
   holding. Label generation uses only its prospectively named, immutable
   profile; the audited corpus uses conservative `datagen-v1`. SPSA keeps its
   own patched adjudication for now -- see PROCESS.
9. Two fully implemented clusters in the same track without an accepted gain
   stop implementation and force a new evidence audit.
10. Engine, tooling and documentation changes remain separate commits.

### Minimum gates

| Change | Minimum evidence |
|---|---|
| Correctness | Independent invariant/regression test; strength gate if playing behavior changes materially |
| Behavior-neutral hot path | Exact fingerprint, debug/release tests, pooled NPS |
| Search/HCE strength | Revision-matched clean PGO A/B, paired UHO, registered SPRT and stop rule |
| Extension/depth authority | Fixed-node tree/depth profile plus tactical suite at both fixed depth and equal node cost; true correctness canaries veto, while aggregate disagreement is diagnosed rather than cherry-picked |
| Texel fit | Verified label domain; hash-complete whole-start train/validation/frozen-test splits; exact all-slot instrument coverage and bake smoke; reconstruction, activation/identifiability and semantic bounds; settled trajectory, post-fit cohort/covariance review, baked PGO and SPRT |
| SPSA | Registered live surface and immutable horizon, bounded sensitivity pilot when needed, completed full-surface theta, fresh PGO bake and SPRT |
| Release | Prior-release STC/LTC, 4T direction, NPS, platform/ISA matrix and user-facing docs |

### Independence boundary

Both engines are GPL, but Phase 4 deliberately builds an independent Rarog
design. Problems, dependencies, populations and known failure modes may cross
from a reference. Source, tuned constants, tables, identifiers and structural
transcription may not. The frozen `hybrid` branches are diagnostic artifacts;
they are never merged or shipped.

## 3. Accepted foundation through 2.3.2 and RAR-S70

| Work | Evidence / disposition |
|---|---|
| Broad selectivity fit | Accepted at +15.33 +/- 7.34 nElo |
| Zero-reduction LMR floor | Accepted at +9.13 +/- 5.45 nElo |
| Anchored HCE refresh | Accepted at +11.56 +/- 5.19 Elo, RAR-E05 |
| Typed TT evidence and provenance | Retained infrastructure; behavior-neutral at accepted defaults |
| Root abort/fallback and correctness coverage | Retained infrastructure |
| AArch64 TT prefetch | Accepted at +1.42% median NPS on M4 |
| Phase-4 ProbCut move filter | Accepted at +15.56 +/- 10.02 Elo, RAR-S57/S58 |
| Root-only LMR relief | Accepted at +2.33 +/- 1.85 Elo, RAR-S70 |

Retained default-off switches are not evidence. Each must be consumed by its
named step or removed:

| Owner | Retained surface |
|---|---|
| 4.11 | TT provenance consumers and raw/pruning/searched evaluation separation |
| 4.13 | Unconsumed continuation/capture correction and history alternatives |
| 4.13 or removal | NMP/IIR/singular provenance alternatives; extensions remain a measured null |
| 4.12 | `SelectivityProspectiveDepth` and cp-valued margins whose populations move under the fitted HCE |
| 8.0 | `RootConfPoolInstability`, `SmpIterationSkip` and high-thread ownership |

## 4. Phase 4 — strongest bounded pre-NNUE search and HCE

### Objective and measured interpretation

The clean no-adjudication RAR-O02 hybrid gave two aggregate observations:

| Contrast | Result | Meaning |
|---|---:|---|
| Stockfish search minus Rarog search, Rarog HCE held | about **+196.5 Elo** | Mature search can use Rarog's HCE much better; not an individual mechanism forecast |
| Stockfish HCE minus Rarog HCE, Stockfish search held | about **+328.6 Elo** | HCE remains a major population; not a sum of portable family gains |

The later matched search ablation initially appeared to assign 116 Elo to LMR
and 124.6 Elo to shallow pruning. Four LMR candidates then measured flat even
though Rarog's LMR base formula matched the reference within 2% and Rarog
ordered better. The corrected conclusion is the phase's central constraint:
the ablation differences measured each mechanism's marginal value inside a
different engine, not Rarog implementation headroom.

Fixed-node measurement subsequently corrected the residual too. Rarog is
**355.26 Elo behind at equal nodes**, its speed closes **104.5 Elo**, and after
the mask-160 comparison the non-LMR/non-shallow residual is about **83 Elo**,
not the obsolete 30. Current counters place qsearch and TT in that population:
Rarog runs about 1.60x the oracle's qsearch per node, hits the TT more and
converts less. Those remain high-value search questions, but Basilisk showed
that an HCE refit can materially move qsearch share while leaving other search
counters stable. Since Rarog's HCE population is larger and its complete
parameter surface has not been requalified, HCE qualification/refit is now
4.7–4.10 and the search-authority decision follows on the accepted HCE at 4.11.

### Completed steps

| Step | Status and durable conclusion |
|---|---|
| **4.0** Evidence, baseline and oracle freeze | Closed by RAR-M12 |
| **4.1** Instrumented oracle | Closed on `hybrid-diag` `de568b3` |
| **4.2** Differential observation harness | Closed by RAR-S55; all counter units and invariants must remain explicit |
| **4.3** Mechanism map and order freeze | Closed; reference divergences are questions, never target values |
| **4.4** Matched-ablation instrument and fixed-node correction | Closed; every mask bit proved live; marginal-value interpretation corrected |
| **4.5** LMR contract study | Closed with no accepted interior gain after four candidates; RAR-S70 root relief remains accepted |
| **4.6** Shallow-selectivity/rewrite continuation | **Closed with no accepted gain**; details below |

#### 4.6 closed disposition

- **4.6.1 Quiet SEE prune:** `QuietSeePruneDepth=6`, coefficient 25,
  completed 652 paired-score games against the oracle at
  **-247.39 +/- 23.69 Elo**. Against `G(0) = -250.77 +/- 13.12`, estimated gap
  closure is only **+3.38 +/- 27.08 Elo**. This is a stopped diagnostic null,
  not an SPRT boundary; the candidate stays off.
- **4.6.2 SearchCore rewrite:** Steps 13 and 16 were rebuilt together. It
  changed the fingerprint from 6,977,070 / 2.466 to 3,479,169 / 2.343 and
  solved 182/300 WAC positions against 167/300 on fewer nodes, but at the
  stopped paired sample it scored **-9.76 +/- 17.70 Elo** over 712 complete
  games, LOS 13.76%. It never reached the registered `[-5,5]` boundary. The
  wholesale rewrite was reverted by `c5e451d` because its structural and
  constant effects were inseparable and its best zero-game signals did not
  predict play.
- **4.6.3 Decision:** no selectivity SPSA and no second broad search rewrite.
  The planned SPSA entry condition was not met: neither an accepted replacement
  contract nor a shrinking matched gap exists. Re-entry requires new local
  evidence after the fitted HCE, not another Stockfish-shaped port.

### 4.2a Harness and instrument integrity sweep

Zero games. Basilisk's audit surfaced two harness failure classes that Rarog
shares: a native command whose nonzero exit was swallowed by the calling
PowerShell, and a user-facing option accepted in a mode that could not honor
it. Both produce a completed run with a plausible number that measures
something other than what was asked.

1. **4.2a.1 — done, `cb5ed2a`.** `sprt.ps1` could not start any gate invoked
   without `-OptionsA/-OptionsB`, including the registered RAR-E06 command:
   `d2c7788` dropped the empty-list return from the option-advertisement
   guard, and `$splitOpts` unrolls an empty pipeline to `$null`, which a
   `[string[]]` parameter rebuilds as a one-element array holding `$null`. The
   same `$null` also emitted a bare `option.` argument to fastchess. Fixed and
   verified in three directions. The `-NoAdjudication` wire, which had never
   played a game, was proved live over 20 games ending 20/20 by a rules
   result.
2. **4.2a.2.** Sweep `tools/*.ps1` for native invocations whose
   `$LASTEXITCODE` is never read, and for exit status taken through a pipe.
3. **4.2a.3.** Every parameter a script advertises must either be honored or
   refuse to launch. An option silently ignored in one mode is the same defect
   class as a dead `--rset`.

### 4.2b Time-forfeit margin at test concurrency

Zero games to diagnose; a gate for any fix. RAR-M14 measured the floor from
RAR-E06's 3,915-game PGN and it is not what it looked like.

Every game ends having spent **97-99% of its entire clock**. The five longest
games in the match, 367 to 494 plies, sit at 97.5-98.7% and do not forfeit.
The three that did forfeit were 90, 98 and 121 plies -- **shorter** than the
131-ply median -- and one flagged while its own reported move times summed to
only 94.5% of budget. So this is neither clock mismanagement nor long-game
exhaustion. It is the gap between engine-reported thinking time and
harness-measured wall time, set against an aggregate slack of about 2% of a
~4.9s budget: roughly 100ms for a whole game. One descheduling event of that
size, with all 14 physical cores running engines while fastchess contends for
the same silicon, is a forfeit.

`Move Overhead` defaults to 10ms. `time_manager.rs` reserves `2*overhead` only
below ~520ms of clock, and its 30ms `smp_reserve` is gated on `threads > 1`,
so a single-threaded engine under a saturated runner has no equivalent
protection. The comment recording `0/3,460 at Threads=1` no longer holds at
this concurrency.

**4.2b.1 — done.** The diagnosis above closes this step. It is a measurement,
not a repair: every fix it implies is owned by **4.12a**, because a change to
a time-management default alters playing behavior and takes its own gate.

### HCE maturity conclusion

The current-code comparison is
`analysis/hce_maturity_2026-08-25.md`. The old 2026-07 audit is historical:
its four concrete activation defects (`attacked2`, enemy rook/passers,
unstoppable passers and phalanxes) were fixed by `d5a6054` and are tested.

Rarog already has a broad approximately 1,200-parameter tapered HCE with
trace reconstruction, caches, material/PST, mobility, threats, nonlinear king
danger, imbalance, passers, specialized endgames, lazy evaluation, rule-50
damping and correction history. It is not yet mature under the Phase-4 bar
because current conditional semantics and residual calibration are incomplete,
and no whole-HCE fit has been run after the representations this programme may
change.

No HCE parameter family is frozen by historical acceptance. Material, PSTs,
mobility, threats, imbalance, sparse terms, king-danger inputs and every other
current `EvalParams` surface are covered by the exact 4.7.3 audit. The ten
material/PST gauge anchors and two invariant king values are the only fixed
slots. Non-parameterized scaling/endgame contracts remain structural questions,
not silently frozen coordinates.

| Family | Current maturity question |
|---|---|
| Score foundation | Phase/tempo/rule-50/lazy ordering and sign-preserving winnability |
| Material/PST/imbalance | Material-conditioned residuals versus compensating correlated terms |
| Pawns/passers | Blocker, file, exchange, race and conversion conditionality |
| Activity/space/threats | Pin-aware legal activity, usable space, safe pawn pushes and exact relations |
| King safety | Shelter/storm dimensionality, castling destination, pinned defenders, weak/flank inputs |
| Scaling/endgames | Exact material conversion, OCB scope, Syzygy-backed won/drawn/cursed separation |
| Calibration/data | Archive/content and all-slot audits complete; publish/freeze the qualified split, fit the full surface, review movement/cohort covariance, then measure full/lazy/search residuals on the accepted HCE |

Stockfish comparison may enumerate and test these contracts. Reciprocal family
ablation is optional coarse sensitivity evidence only; it cannot rank build
order by itself or assign recoverable Elo.

Manta strengthens the process, not the expected-value estimate. MAN-E19's
coherent coverage-plus-constrained-fit bundle won +35.91 +/- 11.19 Elo while
costing 36.2% evaluator throughput; MAN-E18/E20 show that a lower static loss
cannot rescue a semantically wrong or weak feature, and MAN-E21 shows that a
plausible faster mechanism can still lose games. Therefore categorical
semantics and instrument contracts precede their fits, while a complete
existing-surface refit precedes new structure. Static/NPS filters may reject
but never promote, and the whole fitted cluster pays its own game and
search-NPS gate.

Basilisk supplies a new ordering prior, not portable values. Its recent HCE
programme added sixteen terms and lost 77.92 Elo, then gained about 12 Elo by
removing those terms and refitting two old surfaces that had been incorrectly
excluded: nonlinear king safety (+2.64) and 768 PST weights (+9.52). A fit
reported as 348/1,190 parameters had hidden the omission. Its larger holdout
improvement lost while the 14-times-smaller one won. Therefore Rarog audits and
fits the complete existing representational surface before adding features,
and never ranks candidates by validation delta.

### 4.7 HCE data and instrument qualification

#### 4.7.1 Archive provenance, labels and capacity — COMPLETE

`analysis/hce_archive_audit_2026-08-31.md` is the durable record. The two
archives contain 600,000 independent starts, zero replays/parse errors,
6,501,318 unique eligible positions, exact WDL labels, 6,428 natural mates and
26,935 material signatures. Their manifests bind one clean predecessor engine,
one book/seed, disjoint ranges, 8,000 nodes/move and conservative
`datagen-v1` adjudication.

The previous 3,000,000-row target is impossible: train openings stop at
460,752 instead of the required 600,000. The measured exact contract is
**2,300,000 train + 127,778 validation + 127,778 frozen test**, equally
phase-balanced. More datagen is not owed before this corpus receives a verdict.

#### 4.7.2 Atomic corpus publication and hash freeze — COMPLETE

`hce-v2` contains 2,300,000 / 127,778 / 127,778 rows with every input/output
hash frozen. All targets are literal white-perspective self-play results
(`0`, `0.5`, `1`; blend 1.0). Its 600,000 games use disjoint entries 1–600,000
of `beast_seed.epd`; the book has 750,000 unique four-field FENs and no
duplicates. The runner now rechecks the CSV label domain and row counts, book
hash/cardinality/uniqueness, sidecars, non-wrapping ranges and replay count.

#### 4.7.3 Complete parameter-to-instrument audit — COMPLETE

`8d8f507` enumerates all 1,218 `EvalParams` scalars with an exact partition:

| Primary disposition | Slots | Instrument |
|---|---:|---|
| Identifiable linear surface | 1,194 | Sparse traced Adam fit |
| Nonlinear king-danger selectors | 12 | Integer coordinate re-evaluation |
| Material/PST algebraic gauge | 10 | Pin square 0 for pawn–queen in each phase |
| Invariant king material | 2 | Fixed at zero |

The nonlinear instrument also co-tunes the 40-entry king-safety table. The
`complete` group includes PSTs and every historically staged/sparse family;
“already tuned” freezes nothing. Strict complete vectors carry each stage into
the next. Reconstruction now compares directly with independently accumulated
`EvalTrace::raw`, rather than the former tautological residual check.

The end-to-end smoke moved nonlinear values through both linear stages, baked a
changed source and 7,170,826 / 2.468 candidate fingerprint, restored the source
byte-for-byte, forced a recompilation and recovered exact RAR-S70 at
6,977,070 / 2.466. CSR storage measured about 76 nonzero coefficients per
position, making the whole-surface run practical.

#### 4.7.4 Current-source Stockfish maturity map — COMPLETE

`analysis/hce_maturity_2026-08-25.md`, updated by the Manta/Basilisk and native
audits, classifies every existing family. It licenses the complete current
surface fit, not a Stockfish feature port. Missing richer king, conversion,
passer/threat and winnability conditionality remains post-fit structural
residue. Raw/lazy/corrected/qsearch/depth-N search interaction is intentionally
measured at 4.11 on the accepted HCE, because this fit can move those
populations.

### 4.8 Refit the complete existing HCE surface — GAME GATE NEXT

This step tests whether Rarog is mis-calibrated before assuming it is
under-featured.

#### 4.8.1 Registered offline fit

The immutable schedule executed by `tools/texel/fit_complete.ps1` is:

| Setting | Registered value |
|---|---|
| Initialization | Current clean source vector at the invoking commit |
| Corpus | 2,300,000 train; 127,778 validation; 127,778 frozen test; equal five-phase reservoirs; seed 42 |
| Calibration | Fit K once on baseline validation, then pin it for every stage |
| Gauge/invariants | 10 PST anchors and 2 king-material zeros from 4.7.3 |
| Linear optimizer | Complete 1,194-slot sparse Adam, 200 epochs, learning rate 0.3, L2-to-stage-prior `1e-7` |
| Nonlinear optimizer | 12 danger selectors + 40 safety-table entries; 200,000 train positions; integer coordinate descent, at most 40 epochs |
| Alternation | nonlinear -> complete linear -> nonlinear -> 60-epoch complete linear polish |
| Selection | Best validation checkpoint within each fixed stage; serialize/reload the integer vector, then compare it by full re-evaluation with an explicit saved source vector; frozen test opened once after final selection |
| Stop | Exactly two nonlinear/linear cycles; no post-hoc epoch, schedule, data or K change |

Before fitting, the runner re-audits/publishes the corpus, verifies exact
1,218-slot coverage and trace reconstruction, emits full feature support plus
baseline cohort losses, and checks the accepted benchmark. After fitting it
emits every trajectory, parameter delta, validation/cohort table, one-shot test
loss, final complete vector, source patch, candidate binary/benchmark and
hashes. It tests the baked candidate in debug/release and clippy, then restores
source and the normal release binary. Offline loss is evidence, not Elo.

The first production invocation (`hce-fit-20260831_095443`) completed every
optimizer stage, but its frozen report compared stage 3 with the floating-point
polish vector rather than source with the persisted integer candidate. Its
captured patch was also text-corrupted. Those are harness failures, so the
reported delta is retired and no game gate is licensed. The repaired runner
saves source defaults explicitly, reloads the rounded final vector, evaluates
both with the full nonlinear evaluator, fixes cohort membership to the source,
and verifies that the raw UTF-8 patch applies after restoration. Because
`hce-v2/test.csv` was consumed, an untouched confirmation set from unused
opening starts was required; do not reopen or rename the original test.
`tools/texel/confirm_hce_fit.ps1` freezes the completed candidate and K, builds
a clean source engine, generates one game from each unused book entry
600,001--750,000, and hash-splits those 150,000 independent starts 50/50. It
extracts 127,778 equal-phase test positions from the held-out half (the other
127,778 positions are published but select nothing), mechanically verifies
literal WDL targets and provenance, then runs only the repaired one-shot exact
source-to-rounded-candidate comparison. Stockfish evaluations are never read.

That confirmation completed in `hce-confirm-20260831_230548`: 150,000 new
pure-WDL games from starts 600,001--750,000 produced 127,778 untouched test
positions with zero parse failures or replayed starts. The exact rounded
candidate improved loss from **0.12330291 to 0.12252203** (delta
**-0.00078088**, about **-0.63%**) and improved every registered broad cohort.
This closes 4.8.1 and licenses review and a prospective game gate; it does not
establish Elo.

SPSA is skipped here: deterministic traced and re-evaluation instruments own
every existing coordinate, so there is no unexplained live nonlinear residue
to justify it. A later small residue may enter SPSA only with activation,
interaction and curvature evidence.

#### 4.8.2 Review, bake and register the game gate

Review `summary.json`, every stage log, feature support, parameter movement,
semantic bounds, cohort regressions and the one-shot test result. A malformed,
unsettled or semantically invalid fit is rejected without games. If it passes,
apply the recorded eval patch on a clean branch, build final PGO, measure pooled
NPS and register one no-adjudication SPRT against the pre-refit HCE. Bounds and
budget are chosen prospectively from the observed prior using RAR-M10; offline
loss magnitude does not choose them.

Completed by RAR-E06. The exact vector changed 439/1,218 slots, stayed within
declared bounds, passed debug/release/all-target verification, and reproduced
**7,226,051 / 2.460** in clean PGO against **6,977,070 / 2.466**. Three-build
pooled PGO measurement found a real **-1.19% NPS** cost (95% bootstrap CI
**-2.29% to -0.48%**), which the clock gate must price. The prospective gate
is `[0,3]` nElo, 80,000 games, no adjudication. The common Basilisk/Rarog
fastchess, book, TC, concurrency and affinity instrument already owns the null
calibration; symmetrically omitting draw/resign termination changes duration
and variance, not arm placement, so the maintainer waived a duplicate 30k
null. Exact hashes, seed and stop rules are frozen in `EXPERIMENTS.md` RAR-E06.

#### 4.8.3 Strength verdict — ACCEPTED

RAR-E03/RAR-E04 and Basilisk established why the gate was mandatory: large
loss improvements can be neutral or catastrophically wrong, while Basilisk's
accepted +9.52 Elo came from only -0.43% holdout loss.

The gate resolved on 2026-09-01. H1 at **3,914 games**: **+22.04 +/- 7.51
Elo, +32.05 +/- 10.88 nElo**, LOS 100.00%, LLR 2.95, 44 minutes of wall time.
The complete vector is accepted whole and the accepted fingerprint moves to
**7,226,051 / 2.460**. Full record, artifact hashes and the anomaly
disposition are in `EXPERIMENTS.md` RAR-E06.

Two things are worth carrying forward. The effect was eight times the
bracket's upper bound, so RAR-M10's 47,200-game sizing overshot by an order of
magnitude -- sizing from an expected value is right, but a large true effect
resolves in a fraction of the budget, and the cap is not a schedule. And
offline loss again predicted neither sign nor magnitude: -0.63% test loss
preceded +32 nElo here, while Basilisk's -6.2% preceded -77.92 Elo.

### 4.8a Post-refit redundancy removal — CLOSED, NO GATE OWED

Basilisk's BAS-E25 removed terms that a complete covariant fit had shown to be
redundant and gained `+0.49 +/- 2.96` Elo as a simplification. The inventory
ran on the accepted vector (RAR-E07) and **the analogue does not transfer**:
BAS-E25 removed sixteen terms a previous Basilisk phase had *added*, and
Rarog's existing surface has no equivalent accumulation.

What the inventory found, from artifacts that already existed:

- The fit drove only **5 of 1,218** slots to zero, and switched **17**
  previously-zero slots back on. Three of the five are whole 1-slot terms:
  `passed_candidate_mg`, `passed_freestop_eg_per_rank`,
  `threat_safe_pawn_push_eg`.
- Of the 132 slots under the sparse cut, **90 have zero activations and are
  structurally unreachable** -- pawn PST ranks 1 and 8, passers and phalanxes
  on those ranks, threats against a king, impossible imbalance combinations,
  and the two king-material gauge zeros. **Unreachable is not redundant.** A
  pawn PST carries 64 entries because the index space is 64; removing the 16
  that cannot occur restructures an index space for no runtime gain.
- 12 more are the nonlinear danger selectors and 12 are co-tuned safety-table
  entries. The remaining 18 are rare but real, and **all 18 held** -- the
  fitter froze every under-supported coefficient rather than fitting noise
  into it.

Two instrument confirmations came free. The 12 fields with zero linear
activation are exactly `KS_DANGER_INPUTS`, independently reproducing 4.7.3's
1,194 + 12 + 10 + 2 partition from a different artifact; and the freeze
behaviour above is the sparse-cut contract working as specified.

The three zeroed terms are now inert -- they multiply by zero -- so deleting
their code is behavior-neutral and provable by the exact fingerprint, not a
strength question. **4.13.2 owns that removal, not a gate.** One caveat for
whoever does it: `eval.rs`'s `new_terms_activate_on_curated_positions` asserts
that `passed_freestop_eg_per_rank` still *traces*, which it does even at a
zero coefficient. Deleting the feature breaks that test, so the test's
precondition changes in its own commit.

Two residuals go to 4.9 as observations, not candidates: the fit priced
candidate passers at zero in the middlegame and safe pawn pushes at zero in
the endgame. Those are statements about the current representation, and 4.9
decides whether a different representation expresses them better.

### 4.9 Structural HCE upgrades — CONDITIONAL

Open at most two dependency-complete structural clusters, and only for residual
signals the full existing-surface refit could not represent. King-safety and
passer/threat conditionality remain hypotheses, not an order. Specialized
endgame knowledge is not optional here: the direct conversion audit already
established a defect, and 4.9a owns its systematic repair.

For each cluster:

1. define categorical semantics and directional/counterfactual tests;
2. reconstruct every changed feature exactly through `EvalTrace` or a named
   nonlinear instrument;
3. locally refit the changed feature and **all materially covariant existing
   parameters**—historical group boundaries do not freeze them;
4. apply prospective semantic/support/loss/NPS filters as refutation only;
5. bake final PGO and run the registered no-adjudication SPRT;
6. accept or revert before selecting the next cluster.

Two fully fitted cluster failures close structural expansion and force a 4.7
re-audit; they do not authorize more feature inventory.

### 4.9a Endgame conversion and reference-function closure

**Why this is in Phase 4, not Phase 5.** It was placed in Phase 5 on
2026-09-01 by following Basilisk's phase layout, which is not a reason. Three
Rarog-specific facts put it here:

1. These are HCE value and scale functions. Phase 4 is the phase that owns the
   HCE; Phase 5 is the NNUE runway and is required to be behavior-neutral.
2. 4.15 releases 2.4.0. Shipping a release that converts KBNK at 15% and then
   fixing it afterwards is the wrong order.
3. 4.10 consolidates the whole HCE after the last structural change. If
   endgame structure landed in Phase 5, 4.10 would be invalidated and a second
   whole-surface consolidation would have to be paid for.

Phase-4 step numbers 4.10-4.15 are cited in `EXPERIMENTS.md` and in commit
messages, so this enters as `4.9a` rather than renumbering them.

`analysis/endgame_conversion_audit_2026-09-01.md` is the baseline. Its
original table measured the pre-refit source; it has been re-run on the
accepted HCE with the same seed and positions. At 60,000 nodes/move over 100
fixed-seed random positions per family, KQK/KRK/KBBK/KBNK converted at
**94%/86%/76%/15%** before the refit and **94%/91%/86%/19%** after, against
Basilisk's latest **100%/100%/87%/54%**. At n=100 the binomial standard error
is ~3.5 pp, so only the KBBK movement is comfortably outside noise, and the
aggregate artifacts do not retain per-position outcomes for a paired test. **KBNK remains catastrophic at 19%**, with 73 of 100 games still dying on the
fifty-move rule. That is the audit's prediction holding: a complete
recalibration of every coefficient moved KBNK by about one standard error,
because the defect is gradient magnitude against the pruning margins that
consume it, not coefficient calibration. Fitting cannot repair a term whose
actionable signal is smaller than the margins it must survive. KBNK has
Basilisk's same Chebyshev plateaus, sub-pruning gradients and missing
piece-coordination guidance, and the complete HCE fit did not remove the
material-specific drawn-subset bias either. This is a
mandatory engineering program, not a feature-shopping license. The 4.9 cap of
two structural clusters does not apply: that cap bounds speculative feature
addition against residual signals, while this is defect repair against a
measured conversion failure and an explicit reference inventory.

- **4.9a.1 Truth corpus and baseline.** Extend the fixed-seed conversion
  runner to all relevant winning families, add no-adjudication endgame-start
  games, and use Syzygy WDL/DTZ as exact truth for every supported case.
  Preserve position seeds, engine/hash/TT/node policy and outcome reasons.
  Record theoretical verdict, static direction, conversion and game strength
  separately; a played draw is statistical evidence, not theoretical truth.
  The local set is complete 3-4-5-6-man Syzygy at
  `D:\chess\tablebases\syzygy3456` (510 WDL + 510 DTZ), which the engine loads
  and which covers every family in the reference inventory below.
- **4.9a.2 Contextual regression contract.** Hard-veto legality, termination,
  exact-theory and near-mate correctness regressions. Use reproducible random
  family conversion rates, rule-50/stalemate/material-loss counts and
  Syzygy-DTZ progress as aggregate floors. Long fixed-search trajectories are
  diagnostic: investigate individual movement, but accept/reject changes in
  context rather than requiring every position to retain the same PV or mate
  length. Tighten floors after accepted improvements; never relax a
  correctness test in the implementation commit.
- **4.9a.3 Search-visible magnitude audit.** Measure every guidance gradient
  against the pruning margins and resolution that consume it. Begin with
  KBNK/KXK mate drive and passed-pawn king approach. Texel may correctly fit a
  rare multi-ply guidance term toward zero for static WDL loss while search
  requires an actionable magnitude; use systematic sweeps, conversion/DTZ
  progress and games to resolve that conflict rather than freezing either the
  old or fitted value.
- **4.9a.4 through 4.9a.23 — reference-function closure.** Audit, implement
  where absent, and test every specialized function in the final pre-NNUE
  Stockfish HCE table. The reference set is **20, not 18**; Stockfish 11 had
  two additional functions later removed, while current NNUE
  Stockfish/Reckless no longer provide the comparable dispatcher. Reference
  code supplies cases and failure modes, not portable constants or
  implementation. Each item records whether current coverage is full, partial
  or absent and adds theory/Syzygy tests plus its conversion or drawn-subset
  cohort. Rarog's present meaningful coverage is **7/20**; generic
  insufficient-material and OCB logic are retained useful extras, not
  substitutes for this closure.

  | Step | Function | Current coverage |
  |---|---|---|
  | 4.9a.4 | KNNK value/draw classification | present |
  | 4.9a.5 | KNNKP value and conversion boundary | absent |
  | 4.9a.6 | KXK value, KQK/KRK/KBBK conversion floors | present, 94/86/76% |
  | 4.9a.7 | KBNK value, corner/king/minor gradients | present, converts 15% |
  | 4.9a.8 | KPK exact bitbase, value, rule-50 | present |
  | 4.9a.9 | KRKP value | partial, conservative scale |
  | 4.9a.10 | KRKB value | absent |
  | 4.9a.11 | KRKN value | absent |
  | 4.9a.12 | KQKP fortress-aware value | partial, rook/bishop pawn |
  | 4.9a.13 | KQKR value | absent |
  | 4.9a.14 | KBPsK scale, wrong-bishop rook pawn | partial, wrong corner |
  | 4.9a.15 | KQKRPs scale | absent |
  | 4.9a.16 | KRPKR scale | absent |
  | 4.9a.17 | KRPKB scale | absent |
  | 4.9a.18 | KRPPKRP scale | absent |
  | 4.9a.19 | KPsK scale | absent |
  | 4.9a.20 | KBPKB scale | absent |
  | 4.9a.21 | KBPPKB scale | absent |
  | 4.9a.22 | KBPKN scale | absent |
  | 4.9a.23 | KPKP scale | absent |

- **4.9a.24 Dependency-complete family gates, stratified by occurrence.**
  Group mutually dependent value, scale, search-guidance and generic HCE
  terms; refit every materially covariant current parameter. Do not freeze
  historical parameters and do not SPRT each recognizer alone.

  RAR-M15 measured how often each reference family actually occurs, in the
  3,915 games of RAR-E06 itself. **52.7% of games reach a <=6-piece position**,
  so endgames are not rare in aggregate -- but the per-family spread is three
  orders of magnitude, and **one gating policy cannot cover it**:

  | Tier | Occurrence | Families | What can accept a change |
  |---|---|---|---|
  | 1 | >2% of games | KXK 37.34%, KRPKR 10.04%, KPsK 4.19%, KPK 2.84%, KRKP 2.40% | A normal no-adjudication STC SPRT can see these |
  | 2 | 0.5-2% | KBPsK, KRPKB, KPKP, KQKP, KBPKB, KBPPKB, KRKN, KRKB | Endgame-start cohort SPRT; whole-match is impractical |
  | 3 | <0.5% | KBNK 0.28%, KBPKN 0.28%, KNNKP 0.05%, KNNK 0.03% | Theory/Syzygy tests plus conversion and DTZ-progress floors; a whole-match SPRT is structurally incapable |
  | 4 | never observed | KQKR, KQKRPs, KRPPKRP | As tier 3, and the cohort must be **constructed** -- these do not arise from UHO openings at all |

  A tier-3 change confined to 0.28% of games cannot produce a detectable
  whole-match Elo at any budget this project has; asking for one is asking for
  a null result. Its acceptance evidence is correctness: exact theory against
  Syzygy WDL, DTZ progress, and the conversion floors of 4.9a.2. The
  whole-match SPRT's job for tiers 3 and 4 is only to show **no regression**,
  so it takes a loss-permitting `[-1.75, 0.25]` bracket, never `[0,3]`.

  **Adjudication is not merely discouraged here, it is disqualifying.** Under
  simulated `strength-v2`, the same 3,915 games reach a <=6-piece position only
  24.9% of the time instead of 52.7%: adjudication destroys **52.7% of all
  endgames before they are reached**. It removes the measurement, not just
  some games. Every endgame gate, cohort and datagen run is no-adjudication.

  Prospective semantic, support, loss, conversion and NPS screens may reject a
  candidate at any tier; clean final-PGO no-adjudication runs accept.
- **4.9a.25 Closure.** All 20 reference functions are present or have a
  recorded theory-backed reason for exclusion, their hard tests pass,
  aggregate floors materially improve, and accepted families transfer through
  STC/LTC plus an explicit endgame-start cohort. Archive the exact harness and
  defects so the NNUE path does not erase classical fallback knowledge.

### 4.10 Post-structure whole-HCE consolidation

If 4.9 accepts any representation, rerun the complete 4.8 linear/nonlinear
instrument schedule over the new model, retain the trajectory, open the frozen
test once and gate the baked vector against the pre-consolidation accepted HCE.
If 4.9 accepts no representation, close 4.10 as already satisfied by 4.8.

A second data cycle requires a prospective changed-data hypothesis supported by
the first fit and game verdict. More games, labels or epochs are not a default
response to a failed fit.

### 4.11 Post-HCE qsearch, TT and evaluation authority

HCE fitting can change score scale, qsearch share and pruning populations.
Basilisk's +12-Elo HCE refit moved qsearch share from 30.8% to 35.1% while most
ordering/LMR statistics held; which metrics move is engine-specific. Therefore
the old RAR-S70 counters are priors, not a candidate basis.

#### 4.11.1 Observation and baseline

1. Compare the accepted HCE head with exact RAR-S70 at fixed nodes/time, then
   re-run the revision-matched oracle differential at sample stride 1.
2. Profile cumulative and per-iteration nodes over a full-suite shallow/mid
   segment and a fixed representative deep segment that reaches playing depth.
   Report aggregate and per-position median/min/max. Do not infer a target from
   one endpoint, cumulative shallow ratios, absolute cross-engine node counts
   or outlier-sensitive mean depth.
3. Measure main/qsearch TT probe, hit, cutoff and store authority; qsearch entry,
   stand-pat, generated/searched/pruned move reasons; raw/corrected/pruning/
   stand-pat/searched score ownership; and explicit same-unit denominators.
4. Prove each wire and UCI option live with an absurd value. Parameter sweeps
   use a real `go nodes`/`go depth` path; `bench` is valid only after proving it
   consumes that option.
5. If the evidence reopens extension/depth authority, pair average depth at
   fixed nodes with WAC at fixed depth **and equal node cost**. Register the
   screens and how disagreement is handled before the sweep. A true mate,
   legality or termination canary can veto; conflicting aggregate WAC/depth
   results are inconclusive until per-position/equal-cost analysis resolves the
   work-per-nominal-depth confound.
6. Write `analysis/phase4_qsearch_tt_authority.md` with the dependency map and
   an explicit candidate/no-candidate decision.

#### 4.11.2 Candidate and gate, only if 4.11.1 isolates one

The design prior is a Rarog-native authority bundle: preserve exact raw HCE;
keep a separate pruning value; refine only from compatible searched evidence;
and retain qsearch stand-pat/search/store provenance. Manta MAN-S19's +13.02
nElo corroborates the question, not a formula or expected value. Basilisk's
recent contract inventory likewise shows why internal coherence and actual
consumer semantics outrank feature parity or reference constants.

Implement the smallest dependency-complete change, prove switch-off identity,
fit only a justified continuous residue and run the registered `[0,3]` PGO
SPRT. If no unique signal exists, close without code.

The Basilisk 5.7.3 defect is **not present in current Rarog**. Rarog removed its
unconditional node-level in-check extension for +30.75 Elo, so it cannot compose
with a singular double extension into Basilisk's three-ply stack; Rarog also has
no equivalent `double_ext_max` path cap. RAR-S37 already found that tightening
the singular-double margin saved nodes while eliminating doubles cost nodes,
without a strength verdict. That old alternative remains a measured null for
4.13 removal unless fresh post-HCE evidence selects it under the extension gate
above.

### 4.12 Optional post-HCE search SPSA

Open only if several live cp-valued RFP, null, futility, ProbCut, qsearch,
correction or LMR coordinates show a displaced interacting optimum. First run
a registered bounded sensitivity pilot, then audit the entire active
interacting surface. Pilot theta is neither candidate nor seed; the full tune
starts from accepted defaults and preserves its registered horizon under any
staged `StopAfter`. Never mix HCE and search coordinates.

### 4.12a Time management — review, repair and gate

**This step owns all time-management work in Phase 4.** TM had no owner: its
findings were scattered across the ledger, RAR-X06's owner cell still pointed
at 4.9 (which is now HCE structure), and RAR-S47 left `RootConfTime` shipping
ON with six untuned consumers and nobody named. Anything touching the clock
enters here.

**Why here.** TM consumes root scores and confidence signals, and 4.8 just
changed the score scale those signals are expressed in. Measuring TM before
the accepted HCE would price a surface that no longer exists. It sits after
4.11's authority work, and before 4.13's cleanup and the 4.15 release gate,
so a clock change cannot arrive after the checkpoint that is supposed to
describe it.

1. **4.12a.1 Revalidate accepted clock behavior.** RAR-R01's +81 Elo and
   RAR-R02's `2*MoveOverhead` reserve were measured on the old harness and the
   pre-refit evaluator. The direction is retained; the magnitudes are not
   current priors. Re-measure on the accepted HCE before changing anything.
2. **4.12a.2 Forfeit margin.** From RAR-M14: sweep `Move Overhead` against
   forfeit rate on a null pair. The background rate is ~0.08-0.17%, so
   distinguishing two values needs tens of thousands of games -- size it
   before running. `PROCESS.md` prices ~10 forfeits per 3,000 games at ~1 Elo,
   so at the observed rate the entire prize is ~0.2 Elo. This is tournament
   robustness, not a strength lever, and RAR-E06's three forfeits were all in
   positions already lost by 5 to 9 pawns. The specific gap to close is that
   `time_manager.rs` gates its 30ms `smp_reserve` on `threads > 1`, leaving a
   single-threaded engine under a saturated runner with only `2*overhead`.
3. **4.12a.3 `RootConfTime` consumers.** RAR-S47 shipped the completed-root
   confidence clock ON after sizing it to level-neutrality (+0.09% total
   budget, longer on 295 iterations and shorter on 182). Its six identifiable
   consumers were never tuned. Tune them or remove the path; an inert
   mechanism with no owner is 4.13 material.
4. **4.12a.4 Root-instability TM.** RAR-X06 reverified +6.46 +/- 4.12 in the
   reference engine while Rarog's own raw pool-view variant lost 5.54
   (RAR-R05). It may therefore enter only as one bounded input to a completed
   authoritative root snapshot, never as a direct multiplier. Retargeted here
   from 4.9.
5. **4.12a.5 Gate.** One registered SPRT for the dependency-complete clock
   change. **Zero forfeits is a precondition, not the verdict** -- RAR-S54 and
   RAR-S57 both passed with zero forfeits while changing node counts by +23%
   and +5%, so a clean forfeit count proves only that the change is safe to
   measure. Never accept a TM change on a forfeit count alone.

### 4.13 Search cleanup and checkpoint

- **4.13.1 Dead and unreachable mechanism inventory.** Basilisk-derived. It
  found history pruning nearly unreachable, and `double_ext_max` never binding
  even when cut from 200 to 16. A dead mechanism is an anomaly to explain, not
  automatically headroom: measure the population first, then either remove the
  safeguard or redesign it under this step. Report reachability for every
  retained switch in the §3 table.
- **4.13.2 Removal.** Remove every unconsumed 4.6 and retained default-off
  alternative without a future owner. Preserve only diagnostics with a named
  Phase-5/7 owner.
- **4.13.3 Checkpoint.** Re-run debug/release tests, all-feature/all-target
  clippy, exact benchmark, pooled-PGO NPS, fixed-time/fixed-node deficits and
  the accepted 4.11/4.12 game verdicts.

### 4.14 Final HCE/search checkpoint

Compare final head with exact RAR-S70 using revision-matched final-PGO binaries
and no adjudication. Record separately attributed HCE and post-HCE-search Elo,
NPS, fixed-node behavior, STC and LTC direction. Ablate surprising integrated
contributors and close every maturity classification.

The HCE is mature for this release only when:

- the current-source family map contains no unknown or first-draft row;
- every accepted representation reconstructs through `EvalTrace` and has
  activation/covariance plus a game verdict;
- every real parameter slot has a named, verified fitting instrument or a
  written invariant/gauge/unidentifiable disposition;
- the complete existing HCE refit and any post-structure consolidation have
  clean game verdicts;
- optional HCE/search SPSA is completed and gated or explicitly skipped;
- the fitted HCE remains a tested fallback and suitable datagen baseline.

### 4.15 Transfer, portability, SMP and release gate

1. Compare final head directly with 2.3.2 at STC, LTC `10+0.1` and 4T.
2. Record pooled-PGO NPS, benchmark, UCI, correctness, platform and ISA matrix.
3. Drop `-use-affinity` for multi-thread cells and calibrate a null pair under
   that topology.
4. Run a final no-adjudication target cohort including Basilisk and the oracle
   as diagnostic reference points.
5. Remove diagnostic scaffolding without a future owner; retain the ablation
   instrument and frozen oracle branches.

#### Release rule

- 2.4.0 requires cumulative STC point estimate at least **+40 Elo** over 2.3.2,
  95% lower bound above **+25 Elo**, positive LTC and 4T lower bounds, and all
  release gates.
- A cumulative result at or above +100 Elo with lower bound above +75 may
  justify a higher minor version by maintainer decision.
- Below the bar, ship 2.3.x only by explicit decision or close Phase 4 without
  a release. NNUE follows either way.

## 5. Phase 5 — NNUE runway

Phase 5 creates the behavior-neutral runway for NNUE. Nothing here may change
playing behavior: the accepted Phase-4 fingerprint must survive every step.
Work that 4.7 already completed is reused and extended, not rebuilt. Endgame
knowledge moved to 4.9a on 2026-09-01, because it is HCE work that must ship
in 2.4.0 and must precede the 4.10 whole-HCE consolidation.

- **5.1 Measurement corpus handoff.** Freeze the accepted 4.7 corpus and
  manifests as the NNUE residual/stage-gate source. Add only NNUE-specific
  labels or scale; preserve untouched splits.
- **5.2 Per-ply state and dirty pieces.** Define exact deltas for quiets,
  captures, EP, promotion, castling and null. Randomized make/unmake compares
  board, keys, attacks and state against full refresh every ply.
- **5.3 Accumulator scaffolding.** Per-thread/per-ply ownership, refresh
  markers, debug full-recompute seams and reserved king-bucket refresh cache.
  HCE remains active and search stays fingerprint-identical.
- **5.4 Trainer preflight.** Pin `D:/code/net_trainer`, Bullet, toolchain and
  GPU; verify conversion, shuffle, splits, manifests, reference vectors and
  resume semantics.
- **5.5 Runway gate.** Exact benchmark, debug/release tests, randomized unwind,
  reproducible pilot corpus and trainer conformance.
- **5.6 Threat-map hooks, optional.** Reserve only if Phase-7 relation inputs
  would otherwise require another make/unmake rewrite.

Boundary rule: search consumes an evaluator score and evidence class, never
evaluator internals.

## 6. Phase 6 — baseline NNUE

- **6.0 Trainer hardening.** Strict CLI, deterministic splits, hashes, seeds,
  checkpoint selection and exact references.
- **6.1 Controlled data.** Generate 30–60M unique positions with by-game
  splits, deduplication, external and tablebase cohorts, manifest provenance
  and a validated score/result blend.
- **6.2 Baseline networks.** At least two seeds for documented widths and
  buckets; validation selects, untouched cohorts report once.
- **6.3 Scalar integration.** Implement the documented `quantised.bin`
  contract with integer-exact engine/NumPy/reference conformance and clean HCE
  fallback.
- **6.4 Incremental and SIMD.** Randomized incremental/full parity, integer
  bounds, portable/x86/ARM bit identity and pooled-PGO NPS gate.
- **6.5 Architecture loop.** Controlled data-versus-capacity experiments,
  progressing from output buckets to mirrored king buckets and then justified
  relation/multilayer inputs.
- **6.6 Gross search-scale safety.** Repair only clearly invalid scale/margins;
  broad search fitting waits for 7.3.
- **6.7 Baseline release.** Beat the accepted pre-NNUE master at STC/LTC,
  transfer at 4T, pass platform gates and archive every accepted net with its
  reproducible training manifest.

## 7. Phase 7 — NNUE frontier and final search fit

- **7.0** Residual and disagreement analysis by phase, material, king,
  tactical/endgame cohort, calibration and refresh cost.
- **7.1** Data frontier: scale, deduplicate, mine hard positions and refresh
  on-policy data only when a clearly stronger net changes the policy.
- **7.2** Architecture ladder: king/material/threat/pawn relation inputs,
  width/activation and refresh-friendly variants, one axis at a time.
- **7.3** One post-NNUE search fit over demonstrably displaced live
  coordinates, followed by PGO, SPRT, LTC and 4T.
- **7.4** Frontier gate against 2.3.2, the Phase-4 head and target engines.

## 8. Phase 8 — scaling, platforms and product completeness

- **8.0 High-thread and NUMA.** Price the measured depth-diversity deficit at
  4/8/16T; test helper depth/ordering/TT ownership and retained SMP switches.
- **8.1 Runtime dispatch and memory.** Universal dispatch, TT/net placement and
  large pages only as complete architectures with target-native evidence.
- **8.2 Product/platform.** Demand-led Chess960 and platform work; consider
  OpenBench-style distributed testing when typical gains reach 1–3 Elo.
- **8.3 Scaling release.** Full topology, clock, net, ISA and user-doc gate.

## 9. Optional post-NNUE classical fallback

Enter only if a serious king-conditioned NNUE, inference optimization and
data-scale retry fail and the maintainer explicitly abandons NNUE. Reuse the
4.7 residual corpus. Any family accepted in 4.9 or 4.9a is closed here.

- **9.1** King-safety semantic rework.
- **9.2** Material-specific winnability and scaling.
- **9.3** Passer/pawn conditionality.
- **9.4** Threat and usable-activity conditionality.
- **9.5** Material/phase specialization only as a last classical step.

Every fallback item is structure plus fit plus one gate, not additive term
accretion.

## 10. Release checklist

- [ ] Version, README, CHANGELOG and release notes agree.
- [ ] `cargo fmt --check` passes.
- [ ] Workspace/all-target tests pass in debug and release.
- [ ] All-feature/all-target clippy passes with zero warnings.
- [ ] Feature builds and tune-option inventory are correct.
- [ ] Benchmark fingerprint is recorded and every move explained.
- [ ] Local PGO asset passes UCI, benchmark and ISA verification.
- [ ] Prior-release STC/LTC and 4T direction pass the release rule.
- [ ] Hosted platform/CI matrix passes on the release commit.
- [ ] Commit locally; tag, push and publish only on maintainer instruction.

## 11. Documentation ownership

`GUIDE.md` is a status board and nothing else: every phase, step and sub-step
with a checkbox, the current checkpoint, and the command to run next. Anything
longer than a line belongs in one of the files below. GUIDE lost Phases 6-9
during a shortening pass on 2026-08-30 and endgame work was filed under the
NNUE runway; `tools/diag/check_guide.py` now fails when a phase heading is
missing, so that class of drift is caught mechanically rather than by reading.

| File | Purpose |
|---|---|
| `GUIDE.md` | Status board: all phases, steps, sub-steps, checkpoint, next command |
| `PLAN.md` | Rationale, gates, roadmap and what each step involves |
| `EXPERIMENTS.md` | Durable evidence, failures, retry triggers and reproducible recipes |
| `PROCESS.md` | Recurring build, Texel, SPSA and release procedures |
| `TRACKER.md` | History only; never a source of the next step |
| `analysis/hce_maturity_2026-08-25.md` | HCE/Stockfish maturity comparison and fitting policy |
| `analysis/hce_archive_audit_2026-08-31.md` | Archive provenance, content, capacity and quota |
| `analysis/endgame_conversion_audit_2026-09-01.md` | Conversion rates, 20-function inventory, defects, test policy |
| `analysis/basilisk_audit_2026-08-30.md` | Basilisk method/results audit and Rarog consequences |
| `analysis/manta_tooling_audit_2026-08-25.md` | Manta tool dispositions and imported measurements |
| `analysis/ablation_results.md` | Search-deficit measurements and corrected interpretation |

`GUIDE.md` and `PLAN.md` change in the same commit. Source, defaults and
reproducible artifacts outrank prose whenever documents disagree.

## 12. Reference tools and commands

| Tool / path | Purpose |
|---|---|
| `tools/sprt.ps1` | Paired pentanomial GSPRT; default 1T `3+0.03`, Hash 64, UHO |
| `tools/diag/phase4_differential.py` | Same-unit Phase-4 suite aggregation |
| `tools/diag/bench_counters.py` | Sum all per-position bench counter dumps |
| `tools/branching_profile.ps1` | Hash-bound per-position and per-iteration depth/branching shape with robust aggregates; refutation evidence only |
| `tools/pgn_result.ps1` | Reconstruct complete-pair PGN results |
| `tools/build_test.ps1` | Hash-bound build manifests and exact benchmark qualification |
| `tools/spsa.ps1` | Registered targeted SPSA with immutable horizon and staged stop |
| `tools/texel/extract.py`, `extract_parallel.py` | Leak-resistant three-way phase-balanced extraction |
| `cargo xtask build --arch <arch> --pgo` | Production PGO asset |
| `cargo xtask verify-isa --arch <arch>` | Executable ISA contract |
| `hybrid/build.ps1` | Frozen diagnostic oracle package, hybrid branch only |
| `D:/code/net_trainer` | Phase-6 NNUE data/training stack |

```powershell
cargo fmt --check
cargo test --workspace --all-targets
cargo test --workspace --all-targets --release
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo build --release
'bench 13' | .\target\release\rarog.exe
```
