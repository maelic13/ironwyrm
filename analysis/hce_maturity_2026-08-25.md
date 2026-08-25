# Rarog HCE maturity against the classical Stockfish reference

Status: current Phase-4 decision record

Analysis date: 2026-08-25

Rarog baseline: accepted RAR-S70 search, `bench 13` 6,977,070 / EBF 2.466

Classical reference: Stockfish `9587eeeb`, frozen by `hybrid` at `75d0d43`

## Verdict

Rarog's HCE has mature infrastructure and broad classical coverage, but it is
not yet a mature *fitted model* under the standard Phase 4 now requires. It is
approximately 1,200 traced tapered parameters with caches, symmetry and
reconstruction checks, nonlinear king danger, material imbalance, passers,
threats, mobility, specialized endgames, rule-50 damping, lazy evaluation and
search correction history. The old description "Stockfish-11-class feature
list" remains fair. Feature-name coverage is not functional equivalence:
Stockfish's classical HCE conditions the same concepts more strongly on king,
material, pawn, attacker-victim and conversion context.

The reciprocal hybrid result establishes a large aggregate population, not a
recoverable per-family budget. With Stockfish search held constant,
Stockfish-HCE beat Rarog-HCE by about 328.6 Elo in the stopped no-adjudication
RAR-O02 sample. The result says that evaluation matters greatly. It does not
say which family owns that Elo, that family losses add, or that copying a
Stockfish formulation will transfer.

The pre-NNUE objective is therefore narrower and testable: finish the current
semantic/coverage classification, improve only residual-supported families,
fit the frozen representation properly, gate it in games, and leave a mature
classical fallback and teacher. Matching Stockfish's classical strength is not
an acceptance criterion.

## Corrections to the 2026-07-13 audit

The historical `analysis/hce_analysis.md` found four concrete activation
defects. All four are already fixed in current code and must not be scheduled
again:

| Historical finding | Current disposition |
|---|---|
| Two-pawn overlap missing from `attacked2` | Fixed; pawn overlap is inserted before other attack-map consumers |
| Enemy rook behind a passer depended on a friendly rook | Fixed; independent activation is regression-tested |
| "Unstoppable" passer ignored defending pieces | Fixed; the bonus is denied when the defender has non-pawn material |
| Support and same-rank phalanx were conflated | Fixed; separate phalanx tables and activation tests exist |

The remaining work is conditional representation, calibration and validation,
not another correctness sweep over those four items.

## Measured local evidence

| Evidence | What it licenses |
|---|---|
| RAR-E01 staged Texel programme: about +240 Elo on the external cohort | Linear fitting can create large value when the representation and starting weights are immature |
| RAR-E03 Stockfish-label distillation: -17.11 Elo despite 4.9% lower holdout loss | Teacher-fit loss cannot promote an HCE and Stockfish scores are not target weights |
| RAR-E04 on-policy refresh: -1.28 +/- 2.79 Elo despite better validation | More labels on an unchanged representation are not automatically useful |
| RAR-E05 anchored refresh: +11.56 +/- 5.19 Elo, moving 57/1,204 weights mostly 1 cp | Small constrained refits can still pay; preserve anchoring and attribution |
| Manta MAN-E19: 25 audited concepts plus constrained fit, +35.91 +/- 11.19 Elo at -36.2% evaluator throughput | Coverage audit -> coherent structure -> constrained fit -> one gate is a sound process; broad structure still owes an NPS price |
| Manta MAN-E05/MAN-E07: reference-family scaling and imbalance lost 16.32 and 7.00 Elo | Reference fidelity is a hypothesis, never a promotion rule |

## Current maturity matrix

This table compares contracts, not source shape or constants. A family is
complete only when current code is classified as equivalent, intentionally
different with evidence, accepted after a fitted gate, or rejected after a
dependency-complete test.

| Family | Current Rarog state | Difference from `9587eeeb` worth resolving | Required evidence |
|---|---|---|---|
| Score foundation | Tapered MG/EG, tempo, rule-50 damping, lazy/full paths and traced linear deltas exist | One global phase interpolation, coarse initiative that increases score magnitude, and possible train/serve drift from lazy evaluation | Full-versus-lazy cohort residuals; material/phase calibration; sign-preserving winnability tests |
| Material, PST and imbalance | Broad and fitted, including quadratic imbalance | Fewer material-conditioned interactions; fitted compensation may alias activity and pawn terms | Exact-material residuals, activation/covariance, constrained joint refit rather than copied imbalance coefficients |
| Pawns and passers | Extensive pawn cache, support/phalanx, candidate/passed terms, paths, blockers, rook relations and king proximity | Conditionality remains shallower for blocker ownership/type, file effects, exchange safety and conversion/race context | Passer-rank/file/material cohorts, paired counterfactuals and Syzygy-labelled conversion sets |
| Mobility, activity and space | Per-count mobility, outposts, x-rays, closedness and space terms exist | Pin-aware mobility and usable/reachable space are incomplete; several space coefficients are zero | Legal-versus-geometric mobility traces, activation/covariance and NPS before any hot-path expansion |
| Threats | Attacker/victim tables, hanging, weak and restricted relations exist | Safe pawn pushes, overload/pin context and exact attacker-defender relations are less conditional | Tactical counterfactuals, qsearch/depth-N disagreement and SEE-safe activation reports |
| King safety | Nonlinear danger table, attacker units, safe checks, shelter/storm and flank inputs exist | Shelter/storm is low-dimensional; castling destination, pinned defenders and several flank/weak-ring inputs are absent, broad or fitted to zero | Queen/phase/shelter cohorts, legal safe-check tests, activation/covariance and a pooled-PGO cost gate |
| Scaling and endgames | KPK/KBNK, fortress/partial scalers, OCB and rule-50 handling exist | Dispatcher and material-specific conversion are narrower; OCB scope and generic winnability remain coarse | Exact material signatures, Syzygy WDL/DTZ, non-amplification and won/drawn/cursed separation |
| Calibration and data | EvalTrace, Texel reconstruction, staged/self-play history and anchored fit exist | No current frozen residual corpus spans the post-2.3.2 representation with train/validation/untouched-test discipline | Versioned corpus and manifests, by-game splits, activation/covariance, external cohorts and one-time untouched-test reporting |

## How the Stockfish comparison may be used

Use the reference to enumerate a family, map its dependencies and design
counterfactual tests. Do not use equal-mask Elo loss as Rarog headroom. A
matched ablation measures a family's marginal value inside each evaluator,
including its scale, overlap and downstream search interaction. Search Phase
4 already demonstrated that this quantity is not transplantable.

The reciprocal HCE instrument is still useful as a coarse sensitivity map if:

1. every mask bit is mechanically proved live;
2. each arm stays in a readable score band;
3. outputs are normalized for score scale and checked for gross calibration;
4. the result ranks questions only, never implementations or expected Elo;
5. local residual, activation, covariance and NPS evidence must agree before a
   Rarog structural candidate is registered.

## Improvement and fitting programme

1. **Freeze the evidence base.** Build the Phase-4.9 corpus with by-game
   train/validation/untouched-test separation, exact material and feature
   cohorts, Syzygy labels, paired counterfactuals and raw/lazy/corrected/
   qsearch/depth-N outputs. Record hashes and label recipes.
2. **Finish the family map on current source.** Classify every matrix row and
   explicitly close historical defects already fixed. Reciprocal Stockfish
   ablation is optional coarse evidence, not the ordering algorithm.
3. **Select at most two structural clusters.** Selection requires a missing or
   misspecified local signal, meaningful activation, manageable covariance and
   an acceptable projected NPS cost. Likely areas are king-safety
   conditionality, material-specific winnability/endgames, and passer/threat
   conditionality, but the corpus chooses.
4. **Fit each changed cluster locally.** Freeze categorical semantics, fit all
   moved and materially covariant linear weights with anchoring, validate on
   frozen cohorts, bake clean PGO and run the registered SPRT. Untuned draft
   weights are not a fair test.
5. **Run one whole-HCE Texel consolidation.** After representations freeze,
   fit the identifiable active vector with fixed splits, anchoring and explicit
   free/fixed groups. A second cycle is allowed only if registered beforehand
   and both held-out loss and the first game gate justify fresh on-policy data.
6. **Use SPSA only for the residue it can identify.** HCE SPSA is optional and
   limited to activated nonlinear/global parameters that the linear trace
   cannot fit. Search-margin SPSA is separate and occurs only after the HCE is
   frozen, over cp-valued consumers whose populations demonstrably moved.
7. **Final checkpoint.** Compare the accepted fitted HCE with the frozen
   search baseline using revision-matched final-PGO binaries, no adjudication,
   NPS, STC and an LTC direction check. Ablate surprising contributors.

## Stop rules and maturity bar

- Two fully fitted structural HCE clusters without an accepted gain stop new
  representation work and force a re-audit of the corpus and family map.
- Lower Texel or teacher loss never accepts a candidate.
- Zero or sign-flipped fitted weights trigger activation/covariance review;
  they do not by themselves prove a chess concept useless.
- A material NPS loss must be priced in the same game gate; evaluator
  throughput alone is not search NPS.
- Phase 4 may call the HCE mature only when the family matrix has no unknown or
  first-draft row, accepted representations reconstruct through `EvalTrace`,
  the frozen whole-HCE fit has a game verdict, and any SPSA is either completed
  and gated or explicitly skipped for lack of identifiability.
