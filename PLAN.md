# Rarog development plan

Updated 2026-09-01. This is the current roadmap. Historical evidence belongs
in `EXPERIMENTS.md`; current status and commands belong in `GUIDE.md`.

## 1. Current state

| Item | State |
|---|---|
| Released baseline | **2.3.2** at `f931722` on `master` |
| Accepted head | RAR-E12 on `dev`; `bench 13` = **8,044,078 nodes / EBF 2.481**, 1T. Includes the 4.9a.4 mate drive, which is bench-INVISIBLE |
| Integration state | The failed SearchCore rewrite is reverted by `c5e451d`; `d2c7788`/`e4f10ca` upgrade search evidence and `8d8f507` supplies the audited complete HCE fitting pipeline without changing accepted behavior |
| Frozen search/HCE oracle | `hybrid` at `75d0d43`, Stockfish `9587eeeb` driving the exact Rarog 2.3.2 HCE |
| Measured search deficit | **355.26 +/- 27.03 Elo at equal nodes** and **250.77 +/- 13.12 Elo at equal time**; Rarog's speed is worth a measured **104.5 Elo** |
| Accepted Phase-4 gains | ProbCut **+15.56 +/- 10.02**; root LMR relief **+2.33 +/- 1.85**; complete HCE refit **+22.04 +/- 7.51**; TB-corrected labels **+6.73 +/- 3.82**; hce-v3 refit **+11.81 +/- 5.33** |
| Active game job | none; RAR-E12 accepted 2026-09-03 at **+11.81 +/- 5.33 Elo**, +17.57 nElo. RAR-E13 withdrawn unresolved |
| Current step | **4.9a.7 — KRPKR scale, the highest-occurrence open family** |
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
   holding. **This now covers every instrument**: `sprt.ps1`, `gauntlet.ps1`,
   `datagen.ps1` (profile `datagen-v2`) and the SPSA tuner. Label generation
   still uses a prospectively named, immutable profile -- `datagen-v1` names
   the adjudicated contract that `hce-v2` was built under and keeps meaning
   that, so old manifests stay true.
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

Both engines are GPL, so copying was never a licence question -- Rarog is
GPLv3 and so is Stockfish. The restriction was a design choice, and the
maintainer relaxed it on 2026-09-01.

**Constants may now be ported as SEED VALUES.** Problems, dependencies,
populations, failure modes, mechanisms and their constants may all cross from a
reference. What does not change is what makes a constant Rarog's: a ported
value is a starting point, never a result.

1. A ported constant is on the DONOR's score scale. RAR-E06 refit Rarog's
   entire HCE, so its centipawn means something different; an imported eval
   weight is in the wrong units by construction.
2. So a ported constant rides the next fit. Port it, seed it, let 4.10's refit
   cycle move it, and expect the optimum to sit somewhere else.
3. Nothing about the gates changes. A ported mechanism still passes the same
   conversion floors, theory vetoes and registered SPRT as one written here.
   Provenance never substitutes for a verdict.

Search margins are the exception worth naming: they are tuned against a
specific eval scale and node distribution, and Rarog's were re-fitted after the
HCE changed. Import search constants only as SPSA seeds, never as values.

Structural transcription of whole subsystems is still avoided, for the ordinary
engineering reason that a ported subsystem carries assumptions its new host may
not satisfy -- not for licence. The frozen `hybrid` branches remain diagnostic
artifacts; they are never merged or shipped.

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
2. **4.2a.2 -- done.** Swept `tools/*.ps1` for native invocations whose
   `$LASTEXITCODE` is never read and for exit status taken through a pipe. None
   found: direct invocations check the status, and the measurement scripts
   assert on their PARSE instead, which is stronger -- it verifies the number
   exists rather than that the process returned 0.
3. **4.2a.3 -- done, `334c084`.** The match anomaly guard was zero-tolerance
   and had never run a match; it declared RAR-E06 invalid over 3 forfeits in
   3,915 games. Split by evidence: crash, illegal move, disconnect and protocol
   error stay absolute, time forfeits take a 0.5% ceiling that sits two orders
   of magnitude clear of both healthy runs (0.03-0.33%) and genuinely poisoned
   ones (5.5%, 34.9%).
4. **4.2a.4 -- done, `3fb9f57`.** Every parameter a script advertises must be
   honored or refuse to launch. `sprt.ps1` accepted `-Games` in modes that
   ignore it, exactly Basilisk's defect; an option silently ignored in one mode
   is the same class as a dead `--rset`.

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

1. **4.8a.1 Inventory -- done.** From artifacts that already existed:

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

2. **4.8a.2 Removal -- no cluster exists.** The three zeroed terms are inert --
they multiply by zero -- so deleting their code is behavior-neutral and
provable by the exact fingerprint, not a strength question. **4.13.2 owns that removal, not a gate.** One caveat for
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
signals the full existing-surface refit could not represent.

**4.9 has not been skipped; its entry evidence does not exist yet.** RAR-E07
noted two observations -- the fit priced candidate passers at zero in the
middlegame and safe pawn pushes at zero in the endgame -- but those are signals
the surface DID represent, and represented as zero. That is the surface saying
the term is worthless as currently defined, which is not the same as a residual
it cannot express. The entry evidence this step requires is a post-fit
residual and cohort analysis, and nobody has run one.

1. **4.9.1 Entry evidence -- DONE, RAR-E09.** See
   `analysis/hce_residuals_2026-09-01.md`. It found **no residual the existing
   surface cannot represent**, and found a label defect instead: KR-K, a 100%
   theoretical win, is labelled a draw on 75% of its corpus positions, because
   Rarog scores a won rook ending at 426-487 cp while `datagen-v1`'s resign
   rule needs 600. The evaluator predicts 0.849 there against a label mean of
   0.625, so it is closer to the truth than its own training data and the fit
   has been pulling it down.
2. **4.9.2 Decision -- CLOSED, no cluster opened.** The cohort table in
   `analysis/hce_residuals_2026-09-01.md` covers 4.9's own named hypotheses and
   none of them licenses structure: king-attack sits +3.8% and threat +3.1%
   above global loss on very broad populations (67% and 93% of positions), and
   passer sits **6% below** it. The largest residual, KR-K, is fully
   representable and is a label defect. The opening cohort's +41% is outcome
   entropy, not evidence of mis-modelling -- separating "harder" from "wrongly
   modelled" needs an oracle's loss on the same positions, which does not
   exist here.

   **Retry trigger.** Reopen 4.9 only on a residual that a coefficient value
   cannot fix, measured on a non-degenerate cohort. Note that the passer
   bucket was itself degenerate until 2026-09-01 -- it selected 127,777 of
   127,778 positions -- so any pre-fix cohort claim about passers is void. If
   RAR-E08 accepts tablebase-corrected labels, rerun this audit afterwards:
   correcting the labels may expose a residual that the mislabelled data was
   hiding. King-safety and
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

The sub-steps are numbered in the order they are worked, so the GUIDE list can
be run top to bottom.

**The order is set by a feedback loop, not by convenience.** Self-play labels
depend on the engine's own conversion ability, which depends on the evaluator,
which depends on the fit, which depends on the labels. It closes. Two things
act on it, and they decide the sequence:

- **Below 7 men, Syzygy is an external anchor** -- truth that does not depend
  on Rarog at all. That is what RAR-E08 decides how to use, and it is the only
  non-circular input available.
- **Above 7 men nothing anchors it**, so iteration is genuinely required. 4.10's
  mandated refit loop is where that is paid for; it is not a design failure.

The consequence is that **conversion improvements must precede data
regeneration**, because they improve the engine that produces the next round of
labels. So 4.9a runs label-independent work first (4.9a.2-4.9a.4), fixes the
label contract (4.9a.5), regenerates (4.9a.6), and only then fits families.
Each turn of the loop then starts from a better generator rather than
re-deriving the same weakness. The bracketed `ref N` is the item's identity in the
20-function reference inventory, which is a different ordering and is preserved
so nothing is lost when the two are compared.

- **4.9a.1 Truth corpus -- DONE.** `tools/diag/endgame_truth.py` grades every
  strong-side move against Syzygy rather than every game against the clock,
  which is what fixes the +/-3.5 pp resolution problem of a 100-game
  conversion rate. Artifact
  `tools/results/hce-accepted/endgame-truth-accepted.json`, per-position
  records retained so a later run over the same seed is paired.
- **4.9a.2 Endgame-start cohort -- DONE.** `tools/diag/endgame_book.py` writes
  a Syzygy-verified EPD book of endgame starts, so the families that never
  arise from UHO openings can be measured at all. RAR-M15's tier 4 -- KQKR,
  KQKRPs, KRPPKRP -- occurred exactly zero times in 3,915 real games, so their
  cohort must be constructed rather than sampled.

  `endgame_cohort_v1.epd`: **788 unique positions across 21 families**, seed
  `0x4E9A2`, 60% theoretical wins and 40% theoretical draws, every position
  probed before it is written and its verdict recorded in the EPD comment.
  SHA-256 `D23A51CD01CC3ADEEB94AABE7239A6F44C14F297FF97A750203FDDB5DA9F7942`.
  `tools/books/` is gitignored, so the generator plus this seed and hash are
  the reproducible record.

  Drawn positions are included deliberately: a book of wins measures conversion
  only, and holding a draw is the other half of endgame skill -- it is also
  where the audit found the evaluator overconfident. Colour is not baked in;
  the harness plays each start from both sides, so the cohort is a paired A/B
  rather than a test of who drew the strong side.

  Three families are short and the shortfall is reported rather than padded:
  KQ-K and KR-K have **no drawn subset** to sample, and KNN-K yielded only 4
  wins in 24 because KNN vs K is drawn almost everywhere -- consistent with
  4.9a.1 finding it drawn in 100 of 100 positions. KRPPKRP is excluded
  entirely: seven men against six-man tables.

  **This book is an instrument, never training data.** Its positions are
  uniformly sampled rather than drawn from play, so feeding them to a fit would
  reweight the corpus toward a distribution the search never sees.
- **4.9a.3 Regression contract -- DONE.** The contract has two halves and they
  need opposite treatment: correctness is absolute, statistics are not.

  **Hard vetoes, in `tests/endgames.rs`.** Two EPD verdicts backed by 64
  Syzygy-verified cases spanning 17 reference families, generated once with
  seed `0xC0FFEE` and frozen into `tests/endgames.epd` so the suite needs no
  tablebases at run time. `tb-win`: a position Syzygy calls won must not score
  as drawn or lost -- sign only, because RAR-E09 measured a won KR-K at +426
  cornered and +487 centralised, so a tighter floor would be a calibration test
  that any refit trips. `tb-draw`: a drawn position must not be claimed as
  forced mate -- the mate claim alone, because a drawn KR-KP really is a rook
  up and demanding a small score there would assert a recognizer that does not
  exist. These span families deliberately: the audit's complaint about the
  older cases was that they test "direction and local mate recognition, not
  class-wide conversion".

  **Aggregate floors, in `tools/diag/endgame_floors.py`.** Conversion,
  win-preserving and DTZ-progress rates per family, compared with a 5-point
  noise allowance -- at n=100 the binomial standard error is about 3.5 points,
  so a bare "must not decrease" test fails on resampling alone. Floors
  **ratchet**: `--update` raises them from a passing run and refuses to lower
  any floor without `--allow-lower`, which exists to make "never relax a
  correctness test in the implementation commit" awkward to do by accident.
  Baseline floors are committed at `tools/diag/endgame_floors.json`, taken from
  the accepted HCE's truth corpus.

  Long fixed-search trajectories stay diagnostic: investigate individual
  movement, accept or reject in context, and never require every position to
  keep the same PV or mate length.

- **4.9a.4 Mate-drive cluster -- DONE, candidate unregistered.** RAR-E10.
  KBN-K **19.4% -> 96.9%**, KBB-K **78.0% -> 100.0%**, `bench 13` unchanged at
  7,226,051 / 2.460, floors ratcheted. Three axes were needed -- resolution,
  magnitude and ratio -- and the ratio was nearly missed: the diagonal shape
  was first tested at ~1:1 against the king terms, measured worse than the
  Chebyshev version it replaced, and recorded as non-transferring. At ~6:1 it
  is the entire gain. Sweeping a mechanism's shape while holding its
  proportions fixed can refute it for the wrong reason.

  KBN-K's residue is now 4 positions where the engine gives away the bishop or
  knight and 1 stalemate, with zero fifty-move losses where there were 61. That
  is a different defect and no drive weight addresses it.

  **ACCEPTED 2026-09-01 on maintainer judgement, with no game gate.** This
  departs from the stated rule that only a registered SPRT accepts a candidate,
  and is recorded as a judgement call rather than a gate. What justified it:
  bench byte-identical, activation triply gated (`|eval| > 200` AND a bare
  losing king AND no pawn, rook or queen for the winner), 15 of 19 families
  measured exactly unchanged, theory vetoes and floors passing, and a tier-3
  occurrence of 0.28% at which a `[0,3]` gate cannot resolve anything at any
  budget this project has.

  What that does not establish: bench-identical proves only that the 40 bench
  positions' trees never reach a minor-piece bare-king mate within depth 13,
  while real games at 3+0.03 reach greater depth with endgames on the board and
  do fire the term in roughly 1.6% of games. **Retry trigger: any
  endgame-shaped strength anomaly reopens this without needing new argument.**

- **4.9a.4 (original scope) Search-visible magnitude audit.** Measure every guidance gradient
  against the pruning margins and resolution that consume it, starting with
  KBNK/KXK mate drive and passed-pawn king approach. This is where KBNK's
  actual defect lives -- the truth corpus shows its technique is within 60% of
  optimal on what it converts (efficiency 1.58) while DTZ progress is 0.277,
  the lowest of any pawnless family -- so the fix is gradient magnitude, not
  another refit. Texel may correctly fit a rare multi-ply guidance term toward
  zero for static WDL loss while search needs an actionable magnitude; resolve
  that with sweeps, conversion and DTZ progress, not by freezing either value.
- **4.9a.5 RAR-E08: label contract -- ACCEPTED, +6.73 +/- 3.82 Elo.** H1 at
  13,432 games, LOS 99.97%, zero forfeits. Arm B's vector was the accepted head
  at **7,165,683 / 2.462**, superseded by RAR-E12 at 8,044,078 / 2.481. Texel theory predicted arm A and lost by 10.34
  nElo: the self-reinforcing label loop was the stronger effect, and correcting
  1.325% of rows paid.

  **What is adopted is the post-hoc relabel of <=6-man positions**
  (`tools/texel/relabel_tb.py`), cursed wins as draws, on an otherwise
  unchanged corpus. It is NOT `datagen-v3`, which adjudicates the game on
  tablebase truth and so changes the recorded result of every position sampled
  from it, including openings. `datagen-v3` remains untested; adopting it
  because "tablebase labels won" would be adopting a different change.

  Conversion cost, resolved at n=400: KBN-K's -5.1 pp at n=100 was noise
  (+0.5 pp, SE 1.5), and the one real regression is **KQ-KP -3.8 pp at 2.9 SE**
  -- owner 4.9a.14, retry at 4.9a.27. Aggregate weighted conversion is flat,
  83.24% -> 83.45%.

- **4.9a.6 Regenerate on the winning contract.** Only after RAR-E08 reports.
  Hash-freeze under a new name; never edit `hce-v2` in place, since it is the
  corpus the accepted head was fitted on and has to stay reproducible.

  **The start book was the constraint, not the schedule.** `beast_seed.epd`
  holds exactly 150,000 positions in each of the five buckets, and the
  extractor's phase is MATERIAL, not ply, so a game started below phase 20 can
  never produce an `opening` row. Splitting the RAR-E08 pilot by the phase of
  each game's own start and preflighting each split shows only opening starts
  feed the opening bucket (3.4392 rows/game; every other start bucket is zero)
  while also being the most productive overall (13.31 rows across all buckets,
  because one game traverses every phase on the way down). Four fifths of a
  balanced book therefore cannot contribute to the bucket that binds, and the
  preflight -- which sizes GAMES -- asked for 1,113,504, more than the 750,000
  openings in the book. Unreachable at any schedule.

  Supply was never short: the read-only store `A:\Chess\Beast\data\txt\
  positions.txt` is ~125M positions, 36.8% of them opening-bucket, duplicate
  rate 0.02%. 150,000 was a quota.

  `tools/texel/build_book.py` builds a phase-WEIGHTED book, defaulting to the
  hedged **50/10/10/10/20**. The yield-maximising corner is 68/10/0/0/22; the
  hedge is taken instead because the corner makes every middlegame and endgame
  row a *reached* position correlated with the opening play that led there.
  `phase_book_v1.epd` (1,000,000 positions, seed `0x5EED2`, SHA-256
  `31E9B655...`) measures **1.6227** rows/game in the binding bucket against a
  predicted 1.720, and sizes 3.5M rows at **602,619 games**, about 5.3 hours at
  the pilot's 1,905 games/min. Openings 997,001-1,000,000 were consumed by the
  validation run, so the real segment starts at 1.

  Two secondary levers were measured and NOT taken: `--skip-start 2` costs 9%
  of opening rows (3.7796 -> 3.4392 on opening starts; Basilisk uses 0), and
  raising `--max-per-game` from 16 buys rows with within-game correlation
  rather than with games. Both are held so the corpus contract differs from
  `hce-v2` in the book alone. Derivation and reproduction commands:
  `analysis/texel_corpus_book_shape_2026-09-02.md`; the matrix regenerates with
  `python tools/diag/book_yield.py <datagen.pgn>`. The whole pipeline --
  resources, tools, settings, contract gates and traps -- is
  `analysis/texel_fitting_handbook.md`.

  **Done: `hce-v3` and `hce-v3-tb` are published and gate-verified.** 602,619
  games generated in 5 h 10 m (1,941 games/min), 3,888,888 rows extracted
  (3,500,000 / 194,444 / 194,444), every per-phase quota met exactly with the
  tightest bucket at 1.44x headroom, 0 parse errors and 0 replayed starts. The
  Syzygy relabel changed 113,046 train labels (3.230%) with 0 probe failures.

  The corpus is materially better than `hce-v2`, not merely bigger: 40
  adjudicated games against 312,918 (52.2%), 367,664 natural mates against
  6,428, mean 91.0 plies against 66.4. `hce-v2` resigned most games out, so the
  evaluator learned from outcomes that were asserted rather than played. **A
  gate on this fit therefore conflates row count, phase mix and label
  provenance** -- a legitimate cluster, but it must be registered as one.

  Two gaps found and fixed on the way, each in its own commit: `relabel_tb.py`
  never emitted a corpus manifest, so its output was not fittable and RAR-E08's
  was hand-built (`90e8939`); and `fit_complete.ps1`'s contract gates were
  pinned to `datagen-v1` at exactly 600,000 starts, so they are now a NAMED
  list of `(profile, starts)` pairs, verified to still reject an off-by-one
  count, a crossed pair and the untested `datagen-v3` (`ec34a34`).


- **4.9a.7 through 4.9a.26 -- the 20 reference functions, in working order.**
  Audit, implement where absent, and test each one. The reference set is 20,
  not 18: Stockfish 11 carried 22 and `KNPK`/`KNPKB` were later removed, while
  current NNUE Stockfish and Reckless no longer provide a comparable
  dispatcher, so the final pre-NNUE Stockfish table is the reference.
  Reference code supplies cases, failure modes and seed constants (see the
  independence boundary in section 2; a seed is not a result).
  Each item records whether coverage is full, partial or absent and adds its
  theory/Syzygy tests and conversion or drawn-subset cohort.

  **The order is by expected value -- occurrence times defect -- not by
  drama.** RAR-M15 measured occurrence in real games and 4.9a.1 measured
  conversion, and the product reorders the list sharply: KRPKR converts 52% at
  10.04% of games while KBNK converts 7% at 0.28%, so KRPKR is worth about
  36 times more attention despite being the less alarming number. KBNK
  therefore sits at 4.9a.18, and its actual mechanism is addressed earlier and
  separately at 4.9a.4.

  | Step | Function | ref | Coverage | Conversion | Occurrence |
  |---|---|---:|---|---:|---:|
  | 4.9a.7 | KRPKR | 13 | absent | 52% | 10.04% |
  | 4.9a.8 | KRPKB | 14 | absent | 56% | 1.23% |
  | 4.9a.9 | KPsK | 16 | absent | - | 4.19% |
  | 4.9a.10 | KPK | 5 | present bitbase | 95% | 2.84% |
  | 4.9a.11 | KRKP | 6 | partial | 93% | 2.40% |
  | 4.9a.12 | KBPsK | 11 | partial | - | 1.92% |
  | 4.9a.13 | KPKP | 20 | absent | 94% | 1.23% |
  | 4.9a.14 | KQKP | 9 | partial fortress | 96% | 1.17% |
  | 4.9a.15 | KBPKB | 17 | absent | 81% | 0.89% |
  | 4.9a.16 | KBPPKB | 18 | absent | 79% | 0.66% |
  | 4.9a.17 | KRKN | 8 | absent | 83% | 0.61% |
  | 4.9a.18 | KRKB | 7 | absent | 94% | 0.51% |
  | 4.9a.19 | KBPKN | 19 | absent | 79% | 0.28% |
  | 4.9a.20 | KNNKP | 2 | absent | 15% | 0.05% |
  | 4.9a.21 | KNNK | 1 | present | drawn 100/100 | 0.03% |
  | 4.9a.22 | KQKR | 10 | absent | 83% | **0%** |
  | 4.9a.23 | KQKRPs | 12 | absent | - | **0%** |
  | 4.9a.24 | KRPPKRP | 15 | absent | - | **0%** |
  | 4.9a.25 | KXK | 3 | present | 94/91/86% | 37.34% |
  | 4.9a.26 | KBNK | 4 | present | **7%** | 0.28% |

  **Two recorded regressions are owned inside this list and must not be lost.**
  RAR-E08 cost KQ-KP 3.8 pp of conversion, owned by **4.9a.14**. RAR-E12 cost
  KBN-K dtz progress 0.7260 -> 0.6753 at -4.4 SE, owned by **4.9a.26**, whose
  acceptance target is to restore 0.7260. Neither blocks the families ahead of
  it, and neither was measurable by the gate that caused it -- KBN-K occurs in
  0.28% of games, so the 7,388-game RAR-E12 gate held perhaps twenty. That is
  the whole reason the floors instrument exists alongside the SPRT.

  **KXK and KBNK sit last because their mechanism is handled at 4.9a.4, not
  because they are unimportant.** They share one defect -- a corner-drive
  gradient of 8 cp per corner step and 4 cp per king-distance step, against the
  100-500 cp pruning margins that must not swallow it -- so one change fixes
  both, and KXK's 37.34% occurrence makes the bundle the most gateable item in
  the whole endgame programme, which KBNK's 0.28% never could be. Their entries
  here are verification and closure, not fresh work.

  **KRPPKRP (4.9a.24) cannot be verified locally at all.** It is seven men and
  the tables stop at six, and RAR-M15 found it occurring zero times in 3,915
  real games -- so it is reachable neither by sampling play nor by verified
  construction. Record it as a gap; do not close it on unverified positions.

  Rarog's present meaningful coverage is 7/20. Generic insufficient-material
  and OCB logic are retained useful extras, not substitutes for this closure.
- **4.9a.27 Dependency-complete family gates, tiered by occurrence.** Group
  mutually dependent value, scale, search-guidance and generic HCE terms and
  refit every materially covariant current parameter. Do not freeze historical
  parameters and do not SPRT each recognizer alone. RAR-M15's tiers decide what
  can accept a change: tier 1 (>2% of games) takes a normal no-adjudication
  STC SPRT; tier 2 (0.5-2%) an endgame-start cohort; tiers 3 and 4 accept on
  theory, Syzygy WDL and DTZ progress with the whole-match run demoted to a
  loss-permitting `[-1.75, 0.25]` no-regression check, because a change
  confined to 0.28% of games cannot produce a detectable whole-match Elo at any
  budget this project has.
- **4.9a.28 Closure.** All 20 reference functions present or excluded with a
  recorded theory-backed reason, their hard tests passing, aggregate floors
  materially improved, and accepted families transferring through STC/LTC plus
  an explicit endgame-start cohort. Archive the exact harness and defects so
  the NNUE path does not erase classical fallback knowledge.

### 4.10 Iterated no-adjudication refit cycles

**At least one full refit cycle on no-adjudication data is owed
unconditionally** (maintainer decision, 2026-09-01), and further cycles run
while they keep paying. This is how the maintainer's previous Texel programme
worked: fit, gate, refit, gate, each cycle returning less, stopping when the
return fell away. The screen below still runs, because it sizes the corpus and
because "no refit" was previously its possible output -- it no longer is.

The evidence for doing it at all is much stronger than "the data is older".
`hce-v2`'s own termination cross-check says **312,918 of 600,000 games (52.2%)
ended by adjudication**, and of its 313,852 decisive games **307,424 (98.0%)
were called by the resign rule while only 6,428 (2.0%) were played to mate.**
The corpus taught the evaluator that winning means +600 cp held for three
moves. It almost never showed it what converting looks like. That is the
mechanism behind the drawn-subset overconfidence in the endgame audit, and it
is not something a better optimizer on the same data can repair.

What it does **not** rest on is "the engine is stronger now, so its labels are
better". Basilisk tested that three ways and it failed all three: the same fit
on its own 8k-node outcomes measured -2.85 +/- 3.11, on its own 25k-node
outcomes **+1.00 +/- 2.11** stopped unresolved with LTC +0.29 +/- 5.46, and on
Stockfish outcomes **-7.30 +/- 4.76**, the worst arm. Evaluation models the
value realizable by its own consuming search, so a stronger player is not
automatically a better teacher. Expect the gain to come from *what the games
show*, not from *who played them*.

**Three contaminations, and regeneration only fixes two of them.**

1. **Label truncation.** `hce-v2` was generated under `datagen-v1`: 52.2% of its
   600,000 games ended by adjudication and 98% of its decisive results were
   called by the resign rule rather than played to mate. Regeneration under
   `datagen-v2`/`v3` fixes this.
2. **Position distribution.** Its positions came from games played by an
   evaluator fitted on that same truncated data. Regeneration fixes this too,
   and it is why conversion improvements precede regeneration in 4.9a -- each
   turn of the loop should start from a better generator.
3. **Initialization.** The fit starts from the current accepted vector, which
   was itself fitted on contaminated data, and **regeneration does not fix
   this**. The mechanism is explicit in the tuner: the linear gradient is
   `grad/n + 2*lambda*(w - base_w)`, so the L2 term pulls toward the STARTING
   vector, not toward zero. The nonlinear king-danger stage is integer
   coordinate descent, local by construction, and stages select a best
   validation checkpoint within a fixed epoch budget rather than converging.

   The pull looks weak in practice -- at `lambda = 1e-7` it did not stop 439 of
   1,218 slots moving in RAR-E06 or 350 in RAR-E08's arm B -- but "looks weak"
   is an impression, not a measurement.

**The initialization question can be settled OFFLINE, and cheaply.** This is
the one place a loss comparison is valid: two fits on the SAME corpus with the
SAME labels differ only in where they started, so their losses are measured
against the same target and are directly comparable. That is exactly what makes
RAR-E08 need games and this not. One control fit from a neutral start,
compared on the same frozen test, answers it for the price of one fit and zero
games.

Do not adopt a from-scratch fit as the default without that evidence. The ten
PST gauge anchors and two invariant king values exist because the surface is
not fully identifiable, so a fresh fit can land in a differently gauged place,
and the current vector encodes accepted, gate-verified structure that a restart
discards. Basilisk's +9.52 came from unfreezing PSTs inside a full-surface fit
that started from existing values, not from a restart.

0. **4.10.0 Initialization control.** Run one cycle from a neutral start
   alongside the normal one, on the same regenerated corpus and labels, and
   compare frozen-test loss. If the neutral start is not better, initialization
   carries no material bias and the loop proceeds from the accepted vector.
   Record the number either way; this closes the question rather than leaving
   it a standing doubt.

1. **4.10.1 Opening supply -- reusable, and this was previously overstated as
   a blocker.** `beast_seed.epd` holds 750,000 unique openings and all were
   used once: 1-600,000 for `hce-v2`, 600,001-750,000 for the confirmation
   set. That does **not** exhaust them. The engine has changed, so the same
   opening produces different games and different positions, and the split is
   a deterministic hash of the game's start key
   (`extract.py::split_for_key`) -- so an opening always lands in the same
   split, and regenerating from the same book cannot migrate a position from
   test into train. Reuse is not merely allowed, it is the clean option.

   Two things do still hold. Within a corpus, an opening may be used once
   (`datagen.ps1` already refuses reuse; Basilisk's 93.3%-duplicate corpus is
   why). And a *fresh* set of openings buys a genuinely independent test
   rather than one covering the same starts as the previous cycle's, which is
   a weak but real form of familiarity. Preparing new openings is therefore
   worthwhile and approved -- it is a nice-to-have, not a precondition, and it
   must not hold up cycle 1.
2. **4.10.2 Composition screen.** Generate a pilot under `datagen-v2` on a
   disjoint segment and compare composition with the matching `datagen-v1`
   archive segment: endgame-phase unique yield, coverage over the 20 reference
   classes, decisive/draw ratio, natural mate count, mean game length. Zero
   fitting. This sizes the full run and predicts which families gain support;
   it no longer decides whether the run happens.
3. **4.10.3 Regenerate and republish, under `datagen-v3`.** Generate the full
   corpus, re-audit provenance and content to the 4.7.1/4.7.2 standard and
   hash-freeze it under a new name. Never edit `hce-v2` in place: it is the
   corpus RAR-E06 was fitted on and has to stay reproducible.

   **Which label contract to use is RAR-E08's question, not a settled call.**
   `datagen-v3` exists and works, but do not adopt it by default before that
   experiment reports. The argument for it, and the argument against, are both
   strong. For it: removing eval adjudication does not
   by itself make labels truthful; it makes them reflect what the datagen
   engine can actually convert at 8,000 nodes. 4.9a.1 measured that at 60,000
   nodes -- KBN-K 7%, KRP-KR 52%, KBB-K 86% -- and 8,000 is worse. A
   theoretically won endgame then gets played out, drawn on the fifty-move
   rule, and recorded as a draw, mislabelling every position sampled from that
   game. That is the same defect eval adjudication was accused of, arriving
   from the opposite direction. `datagen-v3` adds Syzygy adjudication at 6 men
   (`-tb -tbpieces 6 -tbadjudicate BOTH`), which is not an opinion but the
   position's true value, and deliberately keeps the fifty-move rule so a
   cursed win is labelled the draw it really is. A 40-game probe ended 20 of
   40 games on tablebase truth, 12 of them decisively.

   Against it: Texel fits the value realizable by the **consuming search**, and
   under that principle a KBN-K position Rarog converts 7% of the time really
   is a draw. Labelling it a win teaches the evaluator to steer into endgames
   it cannot convert -- which is the same failure mode as borrowing a stronger
   engine's labels, and Basilisk priced that at **-7.30 +/- 4.76**, the worst
   arm it ran. The counter-argument is that self-play labels are
   self-reinforcing: cannot convert, so labelled a draw, so the evaluator
   learns draw, so it never steers there, so it never learns to convert.

   RAR-E08 settles it by running both. Note that its design is a **post-hoc
   relabel at extraction, not `datagen-v3` adjudication**, and that is the
   better instrument: TB adjudication ends the game and so changes the result
   of every position sampled from it, including the opening ones, while a
   relabel touches only the <=6-man positions themselves. It also needs no
   regeneration -- one game set, two label sets, perfectly paired.

   **Do not carry tablebase adjudication into a strength gate either way.** A
   gate measures realized conversion skill; adjudicating on tablebase truth
   would credit both arms equally for an endgame only one of them can win.

   **More nodes per move is NOT the answer to the same problem, and this is
   measured rather than assumed.** Basilisk ran exactly that experiment: the
   same fit on its own 8k-node outcomes measured -2.85 +/- 3.11, and on its own
   25k-node outcomes **+1.00 +/- 2.11, stopped unresolved**, with LTC
   **+0.29 +/- 5.46**. Roughly 3x the datagen compute bought a result
   indistinguishable from zero. Treating that +1.00 point estimate as an
   improvement is the RAR-S61 error -- accepting on a point estimate whose
   interval contains zero. Tablebase truth fixes the endgame-label problem for
   free; node count does not fix it at 3x the price.
4. **4.10.4 Cycle 1.** Rerun the complete 4.8 linear/nonlinear schedule on the
   new corpus and the current model, open that cycle's own frozen test once,
   bake final PGO and run the registered no-adjudication SPRT against the
   accepted head.
5. **4.10.5 Loop and stop rule, registered before cycle 1 begins.** Run another
   cycle while the previous one **accepted its gate**; stop at the first cycle
   that does not. The stop rule is the gate itself rather than an Elo
   threshold, because an Elo threshold invented mid-loop is the same act as
   moving bounds -- and because a `[0,3]` nElo gate already encodes "is this
   still worth keeping". Each cycle needs its own untouched test and its own
   registration. Cap the loop at a game budget decided before cycle 1.
6. **4.10.6 Close.** Record the cycle table -- corpus, test, fit loss, gate
   result, cumulative Elo -- so the diminishing return is visible rather than
   remembered.

A second data cycle beyond this loop requires a prospective changed-data
hypothesis supported by the preceding fit and game verdict. More games, labels
or epochs are not a default response to a failed fit.

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
