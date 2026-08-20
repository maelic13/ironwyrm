# Rarog development workflow guide

This is the short operational view: where Rarog stands, what is running and
what to do next. Detailed rationale, contracts, gates and phase items live in
[`PLAN.md`](PLAN.md); evidence and failed attempts live in
[`EXPERIMENTS.md`](EXPERIMENTS.md).

**This file and `PLAN.md` are the maintainer-facing pair.** `README.md`,
`CHANGELOG.md` and the GitHub release notes are user-facing and must stay free
of method, history, phase numbers and internal naming — see PLAN §2
"Documentation audiences".

## Current checkpoint

| Item | Value |
|---|---|
| Released baseline | **2.3.2** at `f931722` on `master` |
| Accepted fingerprint | **6,922,439 nodes / EBF 2.451**; 4.5 candidate 7,436,275 |
| Integration branch | `dev`, reset to `master` and carrying this plan |
| Frozen oracle | `hybrid` at `75d0d43`; never merge it into Rarog |
| Active experiment | None; 4.7 closed, 4.7c only (RAR-S57/S58, +15.56 ± 10.02) |
| Current action | Open 4.5 Cluster A against the new accepted head |
| Evaluation | Frozen through 4.10; structured fits start at 4.11 |
| Reference | Search `5062aee5`; last HCE `9587eeeb`; ideas only |
| Next releases | Conditional 2.4.0 at 4.19; then NNUE 2.5.0 |
| Work after Phase 4 | PLAN 5.0, the frozen NNUE measurement corpus |

**Phase 4 changed scope on 2026-08-12.** The old Phase 4 closed with 2.3.2 and
a cancelled SPSA; the number is reused for a different programme. The
search-oracle experiments RAR-O01/RAR-O02 measured Stockfish's last pure-HCE
search — driving Rarog's own evaluator, at 1.5M NPS against our 2.4M — beating
Rarog 2.3.2 by about **+196.5 Elo**, and the matching Stockfish HCE beating
that hybrid by another **+328.6**. So the largest measurable deficit is search
coordination, with a second in HCE feature coverage, and both can be attacked
with a public engine as an idea source instead of rediscovered blindly.

Those are logistic point estimates from a deliberately stopped run. They size
a target and order the work; they never accept a change and are never quoted
as a release claim. Search work is evaluator-agnostic, so it survives NNUE
intact — this is not work spent on a surface NNUE will replace. HCE work also
pays forward as a better NNUE teacher at 6.1.

**2026-08-18 maturity audit:** neither search nor HCE is mature yet. Search
has every headline mechanism and unusually strong diagnostics, but lacks a
coherent per-ply authority/history/LMR contract and full TT/qsearch, extension
and root integration. HCE has production-quality trace, tuning, cache and test
infrastructure, but its passer, mobility/threat, king-safety, winnability/
scaling and specialized-endgame coverage is materially weaker than the last
Stockfish HCE. Phase 4 now closes and calibrates those contracts; it does not
copy Stockfish formulas or require behavioral similarity.

The point is **acceleration, not imitation**. Reading a strong engine tells us
which problems are worth solving and in what order, which is the expensive
part Rarog has repeatedly paid for in blind cycles. The deliverable is Rarog's
own design, and where Rarog's answer is better, Rarog keeps it.

Do not run `./tools/spsa.ps1 -ConfigGroup phase4 -LaunchOnly`. That
configuration was cancelled and removed before any games. Do not resume the
interrupted mate-clamp SPRT; the clamp is retained as a correctness decision.

## Closed work through 2.3.2

Phases 0–3 built the engine, the harness, the correctness programme, the
search wave and the reproducible build/CI/PGO line, through 2.3.0 and the
2.3.1 ARM64 patch. The closed Phase-4 line shipped 2.3.2: broad selectivity
fit **+15.33 ± 7.34 nElo**, zero-reduction LMR floor **+9.13 ± 5.45 nElo**,
anchored Texel refresh **+11.56 ± 5.19 Elo**, the NMP mate-score clamp as a
correctness repair, AArch64 TT prefetch at **+1.42% NPS**, and the executable
ISA contract. Those three strength results used different estimators and are
not additive.

**Its item numbers 4.0–4.10 are retired** and are not reused by the tracker
below. Ten abandoned parameters were removed with their accepted defaults
hardwired at the call sites; the root-gap observation stays in diagnostics but
cannot enter root confidence, because null-window rival scores made it
degenerate. Full detail and the retained-inert ownership table are in PLAN §3.

## Forward tracker

<!-- FORMATTING RULES for this tracker — follow them, they get broken often:
     1. ONE step per `- [ ]` bullet. Never join two steps on one line with
        "·" (e.g. "4.4 foo · 4.5 bar") — each gets its own bullet, always.
     2. Continuation lines indent 6 spaces so they align under the text after
        "- [ ] ". SUB-ITEM INDENT IS 4 SPACES with 10-space continuations.
     3. Status boxes: `[ ]` todo · `[~]` ONLY while genuinely in flight (a
        gate running right now) · `[x]` finished — accepted, rejected,
        deferred or parked. Anything resolved is `[x]`, never `[~]`.
     4. Every item opens with its STEP NUMBER, then (for `[x]` items) a
        BRACKETED OUTCOME TAG in bold, so the reader orients by number
        first and reads the result second (number BEFORE tag, never the
        reverse):
            - [x] 4.5 **[ACCEPTED +22.13 ± 7.28, LOS 100%]** Cluster A ...
            - [x] 4.6b **[REJECTED −6.6]** retry — ...
            - [x] (b) **[DEFERRED → 4.8]** late evasions — ...
            - [x] 4.4 **[PARKED → 5.1]** dirty-piece deltas ...
            - [x] 4.2 **[DONE, no games]** Diagnostic counters ...
        Tags: ACCEPTED <elo> · REJECTED <elo> · DEFERRED → <item> ·
        PARKED → <phase> · DONE · FIXED. Put the Elo in the tag, detail
        after. The number must be ON the bullet line itself.
     5. Bullet order: step number, outcome tag, short title, then detail.
     6. NEVER renumber existing items. PLAN.md freezes item numbers because
        commits and history reference them. To insert before the first item
        use a .0; to subdivide, use letter sub-items like 4.5(a)/(b).
     7. Mirror any status/number change into PLAN.md in the same commit.
     8. Blank line AFTER the `###` phase heading, then NO BLANK LINES between
        bullets at all. A blank line splits one list into two and the
        renderer re-spaces everything around it.
     9. ONLY NUMBERED STEPS live in the tracker. A recurring procedure or a
        checklist is NOT a step — it never gets ticked, so an unticked box
        reads as outstanding work forever. Those go in
        `## Recurring procedures`, and the owning step links to them.
    10. Wrap at ~76 columns. Do not let one bullet run to 100+ columns
        because the sentence "felt continuous".
    11. Quick mechanical check after editing — all five must be zero:
        blank lines inside a list; bare `    - ` sub-bullets without a box;
        unnumbered `- [ ] **` pseudo-steps; lines over 78 columns; and
        `(a)`/`(b)` labels written mid-sentence inside a parent's own
        continuation instead of as their own `    - [ ]` line. -->

The model implements and verifies locally; the maintainer runs only the long
game jobs. One item is open at a time and each candidate gates against the
then-current accepted head. Macro-order: **A** search work (4.0–4.10) → **B**
HCE work (4.11–4.18) → **C** transfer and release (4.19) → **D** NNUE (5
runway → 6 baseline → 7 frontier) → **E** scaling (8) → **F** contingent
classical fallback (9, last, may never run). Per-item rationale is in
`PLAN.md` §4–§9.

### Phase 4 — Reference-accelerated search and HCE work (→ conditional 2.4.0)

- [x] 4.0 **[DONE, no games]** Evidence, baseline and oracle freeze — RAR-M12.
      2.3.2 reproduced from `dev` `5294e2c` (code byte-identical to `master`,
      doc-only diff), rustc 1.97.1 as pinned. fmt and all-feature clippy
      clean; tests 258/0 debug and 259/0 release, the one-test gap being a
      documented release-only `cfg`. Bench **6,519,711 / 2.449**; tune build
      advertises 101 options with all ten removed absent and the inert ones
      present; PGO PEXT asset reproduces the fingerprint at SHA-256
      `389E234E…05046E28` and passes `verify-isa`. Oracle binaries re-hashed
      byte-exact. Budget and stop rules registered. `hybrid` and `spsa_impr`
      pushed, so the oracle is no longer single-machine.
- [x] 4.1 **[DONE, no games]** Instrumented oracle — `hybrid-diag` at
      `de568b3`, implementing `analysis/phase4_counter_spec.md`. Verified:
      diag OFF reproduces the frozen binary exactly (bench 136,903, bestmove
      e7e3, zero diag lines); diag ON gives the same 136,903, so the
      instrumentation does not perturb the tree. Built-in invariants hold —
      `best_rank_1` == `cutoff_first_move`, rank buckets sum to the cutoff
      total, `main_tt_probes` == `nodes`. Build with `make build
      ARCH=x86-64-bmi2 COMP=mingw diag=yes`. **The suite must drive `bench`; a
      piped `go … quit` aborts before the search starts.**
- [x] 4.2 **[DONE, no games]** Differential observation harness — RAR-S55.
      Versioned fixed suite (UHO openings, quiet middlegames, tactics, checks,
      zugzwangs, endgames) at fixed depth/nodes, 1T. Counters for TT
      producer/consumer kind, **prune recall and overlap** (not node savings —
      a smaller tree can be worse), **correction attribution**, history
      attribution, move source, cutoff index, LMR/re-searches, pruning,
      extensions, aspiration and root ownership. Off ⇒ bench 6,519,711
      exactly; on ⇒ same best move and nodes. Run it against the 4.1 oracle:
      the counters that diverge most select the work. **Shadow-record**
      stand-pat, ProbCut, NMP/IIR/singular, checking-move LMR and
      root-confidence concerns — each is owned by the cluster that reaches it,
      else 7.3. Recording is mandatory; acting here is not permitted.
- [x] 4.3 **[DONE, no games]** Mechanism map and order freeze —
      `analysis/phase4_mechanism_map.md`. **Execution order changed on the
      evidence: 4.7 runs first**, then 4.5, 4.6, 4.8, 4.9; numbers unchanged.
      Ordering premise refuted (Rarog's first-move cutoff beats the reference
      in every cohort), so 4.5 drops to low expectation. Six mechanisms are
      classed UNKNOWN and their owning cluster must measure before designing.
      Classify each reference contract as equivalent / intentionally different
      (with reason) / missing / coupled to a later consumer, against the Rust
      owners in PLAN §4. If the evidence contradicts the cluster order, edit
      `PLAN.md` **before** implementing — never after seeing games.
- [x] 4.4 **[DONE, nothing required]** Search-consumed board state. Audited
      against 4.7's three leads (null-move entry, move-count volume, ProbCut
      entry): they consume `improving`, `eval_for_pruning`, depth and beta,
      all already available, and the one board-state input their pruning block
      uses is `CheckInfo` — already a per-node lazy cache (`node_ci`) with
      per-move memoisation. Building persistent
      pins/blockers/`plies_from_null` now would be speculative state with no
      consumer, which rule 2 forbids and the step itself warns against.
      Deferred to whichever cluster needs one, else 5.1. Cache only the
      per-ply state a 4.5–4.9 contract actually consumes: `CheckInfo`,
      pins/blockers, check squares, `plies_from_null`, repetition distance.
      Bench-identical where behavior-neutral; pooled-PGO NPS gate where it is
      a layout change. The evaluator-facing dirty-piece delta contract stays
      owned by 5.1 — do not let this grow into the NNUE runway.
- [x] 4.5 **[REJECTED, no gain]** Cluster A — per-ply authority, ordering,
      histories and LMR. All five sub-items closed. RAR-S64 took H0 at
      +0.39 ± 4.89 once the stale-reduction defect was fixed, so the
      +4.50 RAR-S61 saw was the defect. Structural work retained at no
      strength claim; `lmr_prior_reduction_adj` removed. Head
      7,467,143 / EBF 2.477. **Unexecuted scope was numbered, not
      dropped: history semantics → 4.9b, reduction contract → 4.8.1.**
      One dependency-complete cluster. Prior 15–45 nElo; it rests on evidence,
      reduction and re-search coordination, not raw ordering quality.
    - [x] (1) **[DONE, no games]** Rarog search context: `NodeContext`
          replaces the three parallel per-ply arrays. Behaviour-neutral —
          bench 6,922,439 / 2.451 exactly — and NPS-neutral at +0.11%,
          CI −0.14%..+0.48% over three PGO builds per arm (RAR-P17). TT/PV
          evidence, prior reduction, statistical score, cutoff count,
          previous-PV following and continuation keys are deliberately NOT
          added: nothing consumes them yet, so they land with 4.5.2–4.5.4.
    - [x] (2) **[DONE, no games]** Move-picker contract: a named `Stage`
          enum replaces three implicit cursor comparisons, and
          `Stage::GenerateQuiets` replaces the `quiets_generated` bool.
          Behaviour-neutral, bench 6,922,439 / 2.451. Legality and duplicate
          guarantees are now asserted, not assumed — three tests cover both
          paths including in-check, 251/245 total. Quiet suppression is
          CLOSED as intentionally different; see the note under this
          cluster. **Previous-PV following moves to 4.5.3**, whose evidence
          work is where a previous-PV move would be scored.
    - [x] (3) **[DONE, no games]** Evidence ownership. Continuation key in
          `NodeContext`, derived by `push_move`, which is now the only way to
          put a move on the stack — that found and fixed the ProbCut piece
          desync (bench 6,922,439 → 7,467,143). Every continuation site reads
          the stored key. Continuation-malus asymmetry measured and REJECTED
          (RAR-S59): it is a disguised selectivity increase, not an ordering
          fix.
    - [x] (4) **[DONE, no games]** Reduction/re-search contract. Prior-
          reduction authority ADOPTED at 512/1024 ply (RAR-S60): cutoffs per
          node rise faster than nodes and first-move cutoff improves 88.04% →
          88.18%. bench 7,467,143 → 7,587,235. Cutoff count REJECTED as inert
          — a cutoff breaks the move loop, so a per-visit count is 0 or 1.
          Statistical score and TT/PV evidence REJECTED as per-ply fields:
          both are node-local (`quiet_hist`, `tt_pv`) and already threaded.
          Previous-PV following REJECTED as redundant with `Stage::TtMove`.
          **All six of 4.5.1's deferred fields are now disposed**, so (5) may
          close. No dormant switches were left behind.
    - [x] (5) **[REJECTED, no gain]** Fit, gate and ablation. RAR-S61 was
          unresolved at `[3,10]`; RAR-S64 re-measured after the
          stale-reduction fix and took H0 at `[0,10]` in 8,088 games,
          +0.39 ± 4.89 Elo. The whole +4.50 RAR-S61 saw was the defect.
          `lmr_prior_reduction_adj` removed. Structural work retained at no
          strength claim.
      **SCOPE NOT FULLY EXECUTED — both halves now have owners.** Cluster A
      aimed at three maturity contracts. Per-ply authority closed, though as
      "Rarog needs fewer fields than the plan listed" — three of four rejected
      as node-local, inert or redundant, which the plan's own rule allows as
      an intentionally different answer. The other two were marked done
      prematurely and are now numbered rather than left to a catch-all:
      **history semantics → 4.9b** (ageing, decay and seed policy, update
      attribution, check/capture context in indexing, evaluation-difference
      training). Placed before 4.10 because 4.10 re-runs the 4.2 suite as its
      evidence base and that is not meaningful while the history contract is
      unsettled.
      **reductions and re-search as ONE contract → 4.8.1**, where 4.8 already
      owns LMR and depth authority. `lmr_reduction_units` still takes eleven
      loose arguments; the zero-reduction floor and full-depth verification
      were never audited.
- [ ] 4.6 **Cluster B — static eval, TT and quiescence.** Keep raw, corrected/
      pruning and searched evidence distinct. Audit TT admission/replacement,
      PV/bound propagation, qsearch stand-pat, corrected eval, prior-square
      futility, capture/promotion ordering, evasions and checks. Derive
      opponent-worsening from 4.5. Measure, never import, reference blends and
      thresholds. Preserve draw and mate-distance semantics; finish with any
      justified cluster-only fit, final-PGO SPRT, NPS and ablation.
- [x] 4.7 **[ACCEPTED +15.56 ± 10.02, LOS 99.89%]** Cluster C — main
      selectivity; nElo +24.90 ± 16.01 (RAR-S57 gated the a+c bundle at
      +24.50 ± 12.78; RAR-S58 showed 4.7c carried all of it, so 4.7a was
      reverted and the shipped contract is 4.7c alone). Razoring,
      reverse futility, NMP verification, ProbCut, move-count and history
      pruning, quiet/capture futility, in dependency order with prospective
      searched depth used consistently. Categoricals before constants; no
      broad SPSA — and none was run: the curvature probe in (e) ruled it out.
      The prior was re-derived to 5–15 nElo once 4.7b was withdrawn; the
      result beat it by 60%, and is ~3.8x RAR-S54's blind uniform scalar
      (+4.06 ± 3.71), which is what PLAN 4.7 predicted of a structural
      rework. Owns the NMP/IIR provenance switches and
      `SelectivityProspectiveDepth`. Merged to `dev`; the candidate branch
      `p47c-probcut-filter` can be deleted.
    - [x] (a) **[DONE, no games]** Counter comparability and the corrected
          reading. `probcut_nodes` (per node) and `probcut_attempt` (per move)
          now exist on both engines, and the oracle's TT-served returns are
          split out as `probcut_tt_served`. Re-ran the 4.2 suite as
          `analysis/phase4_differential_v3_depth8.txt`. Oracle side `2682f64`
          on `hybrid-diag`; Rarog side `cf4e475`; reading `8142d5a`.
    - [x] (b) 4.7a **[REVERTED]** null-move entry.
          Primary gate becomes `nmp_eval >= beta`, the old margin re-homed
          onto raw `static_eval`. `nmp_attempt` −21.4%, `nmp_cut` −2.4%,
          conversion 19.2% → 23.8%. RAR-S56. Do not gate standalone: PLAN
          rule 3 says substeps are not expected to win alone.
    - [x] (c) 4.7b **[REJECTED, no games]** Move-count volume. The 13.35x
          divergence was a per-move against per-node artifact; corrected,
          Rarog fires LMP at 0.57x the reference's per-node rate. Withdrawn
          before any code was written against it.
    - [x] (d) 4.7c **[ACCEPTED in the bundle]** ProbCut move filter. SEE
          threshold tied to `probcut_beta − static_eval`, cap counts moves
          searched and scales with `cut_node`. Moves −56.6% while keeping
          91% of cutoffs; conversion per move 32.6% → 68.4% against the
          oracle's 71.9%. Cost is not uniform — one endgame got 46% cheaper.
    - [x] (e) **[DONE, no games]** Fit decision: **no SPSA.** PLAN rule 4
          makes it conditional on curvature. A zero-game sweep shows
          `ProbCutMargin`'s conversion surface flat at 61.8–65.5% across a
          2x range, the move cap inert at 0.72–0.74% moves per node, and
          only the gap scale monotone. The condition is not met.
    - [x] (f) **[DONE]** Registered as RAR-S57 at `[3,10]` nElo, cap 16,000,
          on a re-derived 5–15 prior. ⚠ The bounds and prior were fixed in
          writing before the run and used verbatim, but the ledger row was
          filed after the result. Rule 2 wants it filed first.
    - [x] (g) **[ACCEPTED]** Bundle gate closed at 2,838 games, a fifth of
          the cap, zero time forfeits. The rule 7 ablation (RAR-S58) then
          found 4.7c reproduces the whole effect and 4.7a contributes −0.40
          nElo, so 4.7a was reverted. Shipped head is the 47c-only arm that
          passed its own SPRT: fingerprint 6,922,439 / EBF 2.451.
- [ ] 4.8 **Cluster D — extensions and depth authority.** Check, singular,
      double/possible-higher/negative extension, IIR and excluded-move
      semantics against TT provenance, 4.5 context and LMR. Add locally
      justified TT-move reliability, multi-cut correction and shuffling
      guards. Preserve mate/abort and NMP-clamp correctness. Refit only the
      activated surface, then gate the integrated contract.
- [ ] 4.9 **Cluster E — root search and clock handoff.** Aspiration retries,
      completed-root and interrupted-fallback authority, stability and the
- [ ] 4.9b **History semantics — inherited from 4.5.3.** Ageing, decay and
      seed policy; update attribution; check/capture context in history
      indexing; evaluation-difference training as a Rarog candidate. The
      unexecuted half of Cluster A. Runs BEFORE 4.10, whose 4.2 suite re-run
      is not meaningful while this is unsettled. Stockfish's history events
      are candidates, not defaults — RAR-S59 caught one that looked like a
      plain omission and measured as a disguised selectivity increase.
      decision to start another iteration. Measure extra-iteration behavior
      against Rarog's root-confidence model. Settle root evidence before total
      time; tune and gate real-clock changes separately.
- [ ] 4.10 **Search integration, second selectivity pass, fit and freeze.**
      Re-run 4.2 after 4.5/4.6/4.8/4.9 and close every search-map contract.
      **Read `analysis/phase4_10_obligations.md` FIRST.** Every deferral
      to this step is collected there with its evidence: the structural
      work 4.10 does NOT own (history ageing/decay/attribution, and the
      reduction contract — both unexecuted halves of 4.5), the live
      leads (deliberate selectivity randomisation,
      `NullMoveImprovingBonus` as a volume knob, TT-served ProbCut), the
      rejections with their evidence so they are not retried blind, and
      the two live switches owed a disposition. Written because a catch-
      all clause is where structural work goes to be forgotten.
      **Inherited from 4.5.4 (RAR-S60): Stockfish `cutoffCnt`.** A ply-
      slot recency counter — reset via `(ss+2)->cutoffCnt = 0`, so it
      accumulates across sibling visits — consumed as `if
      ((ss-1)->cutoffCnt > 3) r++`. Rejected at 4.5.4 because that is a
      selectivity increase and every reading this project owns says
      Rarog prunes too much. Re-open here only if the second-pass
      evidence changes that, and note the counter-point: it is
      CONDITIONAL on plies that demonstrably cut often, unlike the
      blanket increases already rejected.
      Own any 4.7-adjacent issue intentionally excluded from protected 4.7,
      including changed NMP/ProbCut/futility populations. If structural work
      moved continuous optima, run one targeted search SPSA over activated
      coordinates only; complete theta, PGO and SPRT it. Compare directly with
      2.3.2 at 1T STC and re-run RAR-S53: at `-Nodes 250000`, mean depth
      fall toward ~14 while Elo rises. Freeze the head, then re-review EV
      before 4.11.
- [ ] 4.11 **HCE baseline and reciprocal-oracle freeze.** Make the 4.10 head
      the immutable HCE baseline; record source/binary hashes, benchmark and
      NPS, and a no-adjudication reproduction slice of Stockfish-HCE versus
      Rarog-HCE under the frozen oracle search. Register the HCE budget and
      stop rules before changing evaluation code.
- [ ] 4.12 **Differential evaluator harness and contract map.** Versioned
      legal corpus with scores, phase, terms, activation, covariance and cost.
      Map score/lazy, mobility/x-rays, pawn shelter/storm, king danger,
      passers, winnability/scaling and specialized endgames from `9587eeeb`.
      Classify every contract; generic ideas need separate local evidence.
      Off ⇒ 4.11 fingerprint exactly. Teacher fit cannot accept a candidate.
- [ ] 4.13 **Cluster F — score, winnability and endgame dispatch.** Material/
      PST and phase ownership, tempo, score grain/POV, rule-50 damping, space
      gating, winnability/complexity and a mature queen/rook/pawn/bishop
      endgame registry. Reference formulas are hypotheses, not correctness.
      Structural and Syzygy invariants precede local Texel, PGO/NPS and
      no-adjudication SPRT.
- [ ] 4.14 **Cluster G — pawns, passers and pawn-dependent scaling.** Pawn
      cache, weak-unopposed/lever/doubled/backward/opposed semantics, blocked
      support, edge files, path safety, progression and king distances.
      Activation/covariance first, joint pawn/passer Texel after structure.
      Repeat only after validation and SPRT both improve; reject prettier fits
      that lose games.
- [ ] 4.15 **Cluster H — activity, threats and space.** Pin-aware mobility,
      bishop/rook x-rays, reachable/bad outposts, bishop-pawn severity,
      trapped-rook geometry, files, king-ring/queen pressure, weak/restricted/
      hanging/king threats and material-gated space. Pooled-PGO NPS on attack
      changes; refit mobility tables and traced terms before SPRT.
- [ ] 4.16 **Cluster I — king safety and nonlinear imbalance.** Rank/file and
      blocked/unblocked shelter/storm, castling destinations, attack units,
      single/multiple safe and unsafe checks, weak squares, pinned defenders,
      flank camp, pawnless flanks and mobility/score feedback. Texel fits
      direct and one-hot table weights; targeted SPSA fits bucket selectors
      only after structure passes. Bake theta, PGO and SPRT it.
- [ ] 4.17 **HCE convergence, search compatibility and cost.** Measure lazy/
      cache behavior, parent-child stability, pruning bounds, NPS and endgame
      pathologies.
      Run anchored whole-HCE Texel cycles on activated weights with fixed
      train/validation/untouched splits; PGO/SPRT each final fit. Repeat only
      while validation and the baked SPRT both improve; stop at the first
      no-gain/failed cycle. Then separately SPSA only moved search margins
      with HCE frozen. Never mix search and HCE coordinates.
- [ ] 4.18 **Cumulative HCE checkpoint and ablation.** Revision-matched
      final-PGO comparison against 4.10, adjudication off. Ablate surprises,
      remove unowned alternatives, close every 4.12 classification and record
      search-versus-HCE attribution. UNKNOWN or first-draft contracts fail the
      maturity bar even with positive Elo.
- [ ] 4.19 **Transfer, portability, SMP and release gate.** Direct comparison
      with 2.3.2; confirm LTC `10+0.1` and 4T direction, benchmark and pooled
      NPS, the platform/ISA matrix, UCI conformance and the correctness suite.
      Require zero UNKNOWN maturity-map items, remove unowned scaffolding and
      resolve obsolete switches. Final no-adjudication target cohort includes
      Basilisk 1.9.3 and the 4.1 oracle. Drop `-use-affinity` for 4T and
      re-calibrate the null pair. **2.4.0** needs ≥ +40 Elo STC with the 95%
      lower bound above +25, plus positive LTC/4T lower bounds; ≥ +100 with a
      lower bound above +75 may justify a higher minor version.

### ━━━ NNUE CUTOFF ━━━ (Phase 5 opens the NNUE line)

### Phase 5 — NNUE runway (bench-identical or NPS-gated per step; no games)

- [ ] 5.0 **Frozen measurement corpus.** Quiet, tactical, endgame, rule-50,
      phase-balanced and search-disagreement cohorts with deep **external**
      teacher cp/WDL labels plus Syzygy WDL/DTZ, by-game train/validation/
      untouched-test separation, exact cohort labels, paired counterfactuals
      and per-candidate residual reports. No engine footprint, so it may be
      pulled forward into Phase-4 SPRT downtime.
- [ ] 5.1 **Per-ply state and dirty pieces.** Extend 4.4's structure to the
      full reversible state, then add the dirty-piece delta contract for
      quiets, captures, EP, promotions, castling and null. Adopt the Reckless
      `BoardObserver` shape: three events emitted at the exact mutation
      points, a generic `make_move<T>` so the null observer costs nothing, a
      compact pre-make stack channel for accumulators and a during-make
      observer channel for threat features. Randomized make/unmake compares
      against a full refresh every ply.
- [ ] 5.2 **Accumulator scaffolding.** Per-thread and per-ply ownership,
      refresh markers and debug full-recompute seams. The accumulator lives
      with the search worker, not inside the copyable `Board`. HCE keeps
      running unchanged and the search stays fingerprint-identical. No
      inference yet; reserve the king-bucket refresh-cache slot for 6.5.
- [ ] 5.3 **Trainer preflight.** Pin trainer, Bullet, toolchain and GPU;
      verify conversion, shuffle, deterministic splits and manifests,
      reference vectors and resume semantics. Malformed or lossy input fails
      loudly.
- [ ] 5.4 **Runway gate.** Exact benchmark, fmt, tests in debug and release,
      randomized unwind, reproducible pilot corpus and trainer conformance.
      Create an NNUE integration branch only after this passes.
- [ ] 5.5 **Threat-map hooks (optional).** Reserve the dirty-threat interface
      so threat inputs can land in 7.2 without another make/unmake rewrite.

### Phase 6 — Baseline NNUE via net_trainer (→ 2.4.0 or 2.5.0)

- [ ] 6.0 **Trainer hardening.** Strict CLI, train/validation/untouched-test
      splits, checkpoint selection, hashes, seeds and exact references.
- [ ] 6.1 **Controlled data.** 30–60M unique teacher positions at 10–20
      sampled positions per game, label blend λ selected on validation, seeded
      from a diverse EPD book. Add by-game/trajectory splits, dedup, the
      frozen 5.0 test set and a dataset manifest (source engine and net SHA,
      search budget, book, λ, seed, trainer commit). Do not train mainly on
      positions adjudicated early by the same evaluator.
- [ ] 6.2 **Baseline networks.** Documented widths and buckets, at least two
      seeds; validation chooses within a run, untouched cohorts are used once.
- [ ] 6.3 **Scalar integration.** Implement `nnue_format.md` in Rarog from the
      reference Rust example: chess768 → (H×2, perspective, SCReLU) → 8
      material output buckets, QA=255, QB=64, SCALE=400. **Acceptance gate is
      the integer-exact conformance vectors**, which replaces any custom
      header scheme; embed the net hash for provenance. Require layout
      validation, malformed/truncated rejection and a clean HCE fallback.
- [ ] 6.4 **Incremental and SIMD.** Dirty deltas per ply and thread,
      randomized incremental-versus-full parity across castling/EP/promotion/
      null, integer bound proof, and portable/x86/ARM64 kernels bit-exact and
      target-native PGO-smoked. Hard pooled-PGO NPS gate before any games.
- [ ] 6.5 **Architecture loop.** One axis at a time with two seeds. Step to
      king-conditioned inputs (trainer v2, mirrored king buckets) as the
      minimum serious architecture, consuming 5.2's reserved refresh-cache
      slot. Capacity follows data; data-scale comparisons hold architecture
      fixed and vice versa. Do not declare NNUE complete without testing king
      conditioning.
- [ ] 6.6 **Gross search-scale safety.** Adjust only clearly invalid margins
      or clock scale. The broad fit waits for 7.3.
- [ ] 6.7 **Baseline release.** Beats the accepted pre-NNUE master at STC and
      LTC, transfers at 4T, passes external checks, zero
      incremental-versus-reference mismatch. Archive the 6.1 manifest and
      trainer commit with each accepted `quantised.bin`. **2.4.0 only if Phase
      4 did not use it**; otherwise 2.5.0.

### Phase 7 — NNUE frontier and final search fit

- [ ] 7.0 **Residual and disagreement analysis.** By phase, material, king,
      tactical and endgame cohort, plus calibration, refresh cost and
      teacher-search disagreement.
- [ ] 7.1 **Data frontier.** Scale and deduplicate, natural finishes,
      hard-position mining, controlled label/depth A/Bs against untouched
      sets, and fresh on-policy data with each clearly stronger net.
- [ ] 7.2 **Architecture ladder.** King/perspective buckets, threat and
      material inputs, width and activation, refresh-friendly variants. Each
      relation-input family is a full architecture revision, not an
      engine-side patch; add one family at a time and pick by measured
      residuals.
- [ ] 7.3 **One post-NNUE search fit.** First resolve the retained categorical
      switches no Phase-4 cluster reached, then register only the continuous
      coordinates whose optimum likely moved. cp margins do not transfer
      across evaluators; structural mechanisms do. Coordinate count and
      horizon come from activation, curvature and budget — not a remembered 24
      or 5,000.
- [ ] 7.4 **Frontier gate.** Direct comparison of 2.3.2, the Phase-4 head and
      the baseline NNUE, plus calibrated matches against contemporary target
      engines. This is where the Basilisk gap is re-measured.

### Phase 8 — Scaling, platforms and product completeness

- [ ] 8.0 **High-thread and NUMA.** Price the depth-diversity deficit at
      4T/8T/16T; test the retained pool-instability and iteration-skipping
      switches, first-touch placement, TT/accumulator sharing and false
      sharing. Keep the score/depth-weighted vote merge. Measure helper TT
      write policy and helper diversity rather than intuiting them.
- [ ] 8.1 **Runtime dispatch and memory.** Consider a baseline universal
      binary selecting specialized kernels, plus TT/network placement and
      large pages. No specialized-binary startup CPU guard — see Recurring
      procedures.
- [ ] 8.2 **Product and platform.** Demand-led Chess960 and FRC coverage or
      other platform work; also holds the parked large-page/NUMA TT, shared-TT
      atomic packing, AVX-512/VNNI kernels, match-manifest schema and
      distributed testing.
- [ ] 8.3 **Scaling release.** Full topology, clock, net, ISA and user-doc
      gate.

### Phase 9 — Contingent classical fallback (only if NNUE is abandoned)

- [ ] 9.0 **King-safety semantic rework.** Closed without retry if 4.16 landed
      it. Otherwise: activation instrumentation by queen presence and phase,
      legal versus geometric safe checks, storm conditioning, reachable
      shelter, and joint danger-input fits.
- [ ] 9.1 **Winnability and material-specific scaling.** Replace the sign-only
      initiative term; residual tables by exact material signature, Syzygy
      WDL/DTZ as direct evidence, sign-preserving non-amplifying scalers only.
- [ ] 9.2 **Passer and pawn conditionality.** Blocker ownership and type,
      rear-line openness, connected-passer semantics, candidate-passer
      exchange conditioning, and a short-horizon race diagnostic.
- [ ] 9.3 **Threat conditionality.** SEE-safe pawn pushes, restricted mobility
      per affected piece rather than board-global, cheap pin/overload
      relations. NPS-check first; do not hand-write a threat net one scalar at
      a time.
- [ ] 9.4 **Broad positional repairs.** Queen infiltration on the full enemy
      attack map, bad-bishop conditioning, space usability (all three weights
      fit to zero, so the representation is the problem) and conditioned
      rook-on-seventh.
- [ ] 9.5 **Material and phase specialization.** Bucketed coefficients,
      king-bucketed PSTs, queen-presence gates. Worst time-to-Elo on the list;
      only if NNUE is abandoned outright.
- [ ] 9.6 **Lazy-margin conditioning.** Only if dual-eval data shows a
      material sign-flip cohort; margin by non-pawn material and king danger.
- [ ] 9.7 **OCB material-scope refinement.** A small material hierarchy for
      the opposite-coloured-bishop scaler with non-amplification, sign,
      pure-OCB, plus-minor and plus-major tests. Cheap and high-confidence, so
      it is the natural first item here.

## What you run now

1. Finish the already-open 4.7 scope without adding findings from the maturity
   audit to its candidate. Its latest owner remains the corrected ProbCut
   differential and registered 4.7 bundle.
2. Close or revert 4.7 under its existing gate. Only then begin 4.5 with the
   behavior-neutral Rarog per-ply search context.
3. Follow execution order 4.7 → 4.5 → 4.6 → 4.8 → 4.9 → 4.10. Item numbers
   stay stable because evidence and commits already refer to them.

The hosted release workflow remains the final production check, because one
machine cannot create all Linux/macOS/Windows and x86/ARM assets. Use the
reproduction procedure to verify the baseline, not to modify or republish
2.3.2.

## Recurring procedures

### Phase-4 step lifecycle (4.4–4.18)

For every behavioral Phase-4 step:

**Gate the fitted dependency-complete cluster, not each feature and not the
whole phase at once.** Internal substeps may be too sparse or coupled to win
before their consumers and weights move together. Conversely, postponing all
games until the end destroys attribution and lets losing structures hide.

1. **Audit** — name the problem, its Rust owner, all interacting consumers and
   the local diagnostic population. Update `PLAN.md` first if the evidence
   contradicts the planned order.
2. **Register** — add an `EXPERIMENTS.md` ID with hypothesis, baseline SHA,
   candidate scope, expected direction, gate, cap and stop rule, before games.
   Bounds default to `[0,3]` nElo; widen only for a genuinely large prior and
   justify it in the row. Removals need a bracket permitting a small loss;
   unknown-sign repairs need a symmetric one. Size from RAR-M10 at the
   EXPECTED value before choosing, and use the PLAN §2 sizing table.
3. **Implement** — the smallest dependency-complete cluster. Substeps may be
   compiled and diagnosed separately, but are not expected to pass standalone
   and no incomplete cluster becomes the next strength baseline.
4. **Prove correctness** — fmt, workspace tests in debug and release,
   all-feature clippy and targeted invariants. A behavior-neutral diagnostic
   seam must preserve the exact accepted fingerprint when disabled.
5. **Explain** — use the frozen suite at fixed depth/nodes to compare nodes,
   qnodes, move source, cutoff index, TT use, reductions and re-searches,
   pruning, extensions and aspiration against the oracle. Counters explain a
   candidate; they cannot accept it.
6. **Fit** — after structural and categorical choices freeze, fit the moved
   cluster surface: local Texel for HCE; targeted SPSA for search only when
   justified. Complete theta; do not select a checkpoint retrospectively.
7. **Gate** — bake the fitted candidate and revision-matched baseline through
   clean final PGO, then run the registered paired UHO SPRT. Do not change the
   candidate, bounds, cap, book or adjudication after observing games.
8. **Close** — accept and commit only a passing result. Otherwise revert the
   behavior, keep the evidence row and restore the prior fingerprint. Ablate a
   surprising integrated result before crediting a subcomponent.
9. **Advance** — start the next item only after the preceding one is accepted,
   rejected or explicitly closed.

A separable categorical alternative may have a preliminary SPRT, but that
never replaces the locally fitted integrated cluster SPRT. After accepted
clusters, 4.10 and 4.17 own consolidation tuning and separate confirmation
SPRTs; they may not rescue an earlier losing cluster.

Two failed coherent search clusters trigger a return to 4.2–4.3. Two failed
HCE clusters trigger a 4.12/order re-audit, not silent closure. Track H may
close early only by explicitly conceding the Phase-4 HCE maturity target; no
UNKNOWN or first-draft contract may be presented as mature.

### The independence boundary

Rarog takes **ideas** from Stockfish and builds its own answer. It does not
take code, and it does not aim to resemble it. Both engines are GPLv3, so
copying would be legally permissible — this boundary is a product decision and
is deliberately stricter than the licence requires. PLAN §4 holds the full
table; the working rules are:

- What may cross: the problem a mechanism solves, that the problem exists at
  all, which mechanisms interact and in what order, which populations are
  worth measuring, and known failure modes.
- What may not cross: source code in any language or amount, line-by-line or
  structure-for-structure transcription, tuned constants and margins, copied
  identifiers or file layout, and behavioral equivalence as a goal.
- Read, understand, close the file, then design from Rarog's own code and 4.2
  evidence. If a change cannot be justified without pointing at the reference,
  it is not understood well enough to ship.
- No upstream code is copied, so Rarog is not a derivative work. `README.md`
  already states the correct posture — an independent engine, with thanks for
  the inspiration. Do not restyle that into an attribution of derived code.
- Do not merge the `hybrid` branch, copy its FFI boundary into Rarog, replace
  native Rust with C++/FFI, or read the oracle as permission for a wholesale
  unmeasured rewrite.
- Similarity is never a reason to accept anything, and a counter that diverges
  from the oracle is a question, not a defect. Closing a counter gap is not an
  outcome; winning games is.
- Rarog solving a problem differently, or deciding it does not apply here, is
  a first-class result — record it with its reason and move on.

Search-only candidates keep the `strength-v2` adjudication (600/3 two-sided,
unified with datagen on 2026-08-18) because
both arms share Rarog's score scale. **HCE-changing candidates and every
cross-engine cohort run with adjudication off**, because evaluator scales
differ; RAR-O01 versus RAR-O02 priced that confounder at about 75 Elo. Enable
it for an HCE A/B only after a registered calibration proves it safe for both
arms. Use fixed movetime or nodes only for the deterministic diagnostic suite,
never as the strength verdict.

### Toolchain and harness notes

If a PGO build dies with "target must match host", the rustup default host has
drifted to windows-gnu, so the pinned toolchain resolves to its gnu variant
and PGO training refuses. `rust-toolchain.toml` pins the channel, not the host
triple, so it cannot catch this — check `rustup show active-toolchain` first.

`fastchess -use-affinity` with concurrency 14 is mandatory for 1T gates;
unpinned Zen 3 runs carry a hidden per-run offset of roughly ±10 nElo. It pins
one core per game and starves `Threads>1`, so drop it for multi-thread runs
and re-calibrate the null pair under that configuration. Validate any harness
change on a null pair — the same executable on both arms — before trusting a
verdict.

NPS work: validate on a self pair first (it must read about 0.00%), pool
several PGO builds per arm because two PGO builds of identical source differ
by about 0.36%, and keep compilation, profiling and unrelated load off the
match host. Roughly 2 Elo per 1% NPS at `3+0.03`.

### Texel convergence procedure

Texel is cheap enough to run locally after structural HCE work, but its static
loss is not a strength verdict:

1. Trace every changed term exactly and verify full evaluation reconstruction.
2. Report activation, covariance and identifiability before selecting weights.
3. Keep fixed by-game train/validation/untouched splits. Never tune on the
   untouched test set.
4. Run a local family fit after each structural HCE cluster. Bake the fit into
   clean PGO and SPRT the cluster; a lower loss alone accepts nothing.
5. At 4.17, run an anchored whole-HCE consolidation over only activated,
   identifiable weights. Repeat a cycle only if validation and the baked SPRT
   both improve. Stop at the first no-gain/failed cycle; never choose a lucky
   intermediate checkpoint retrospectively.
6. Keep search parameters frozen during HCE Texel cycles.

### SPSA go/no-go procedure

The generic harness is retained. Phase 4 uses it only for justified cluster-A
search coordinates at 4.5, the targeted search fit at 4.10, king-danger bucket
selectors at 4.16, and HCE-induced search compatibility at 4.17. It still
forbids an undirected broad tune. Before any SPSA:

1. Name the strength-bearing mechanism and show local evidence that its
   consumers are misfit.
2. Estimate plausible Elo and opportunity cost before optimizing schedule
   details. Cancel if the plausible gain is inside the gate's dead zone.
3. Gate categorical switches separately and freeze the winner in both arms.
   Never pin a binary knob as an SPSA constant — a pinned A/B knob is an
   unmeasured assumption.
4. Select continuous coordinates from activation and interaction evidence. Do
   not target 24 merely because an old plan said 24.
5. Choose the horizon from gradient quality, integer resolution and compute
   budget. 5,000 is a prior calibration, not a universal answer.
6. Run `./tools/audit_spsa_coverage.ps1` and register surface, fixed values,
   iterations, games, gain and estimator before launch.
7. Complete the final theta without post-hoc checkpoint selection; bake it
   into a fresh clean PGO binary and run a paired SPRT, then LTC/4T where
   appropriate.

### Opening book

SPSA and the default SPRT both use `tools/books/UHO_Lichess_4852_v1.epd`,
paired and reversed, at `3+0.03`. That alignment is the point: the optimizer
and the confirmation gate see the same opening and clock distributions. Use a
second book or LTC as an extra robustness check for a mechanism suspected of
condition sensitivity; do not create an unnecessary tuning/confirmation
mismatch.

### CPU compatibility design

There is deliberately no startup CPU guard inside specialized assets. When the
compiler is told that BMI2/AVX2/FMA are mandatory, ordinary feature-detection
macros fold those checks to true and the guard is removed. A working
in-process guard would require baseline-compiled CPUID code to execute before
specialized code, adding a separate dispatch boundary. The current design is
close to the specialized-binary model: users choose `x86-64`, `avx2`, `pext`
or `arm64`, the README states exact requirements, and release tooling
disassembles each asset to enforce the promise. If a single universal binary
becomes a product goal, 8.1 may add a Stockfish-style baseline dispatcher.

### Experiment discipline

- Begin from a clean revision and record both binary hashes.
- Register hypothesis, interactions, gate, stop rule and budget before games.
- Treat tune and non-PGO results as diagnostics unless the experiment says
  otherwise; final-PGO games decide promotion.
- Do not turn node reduction into Elo. Use diagnostics to explain a game
  result.
- Record rejected and neutral outcomes in `EXPERIMENTS.md`; never silently
  rewrite them into a later success story.
- A correctness exception must name the invariant, the tests and the
  incomplete strength evidence honestly.

## Decision rules

- One item open at a time; each candidate gates against the current accepted
  head, never against a stale baseline or another unresolved candidate.
- Categorical architecture is gated before its constants are fitted.
- A touched dormant switch must be removed, kept inert with a named owner, or
  separately gated. It is never activated opportunistically.
- Borderline results are not accumulated as hidden debt. Accept or revert.
- Commit after each finished and verified step, and keep tooling changes in
  separate commits from engine changes.
- Mirror any tracker status or number change into `PLAN.md` in the same
  commit.

## Common commands

```powershell
cargo fmt --check
cargo test --workspace --all-targets
cargo test --workspace --all-targets --release
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo build --release
"bench" | ./target/release/rarog.exe
cargo xtask build --arch pext --pgo
cargo xtask verify-isa --arch pext
```

```powershell
# Primary SPRT [3,10] nElo; add -TC "10+0.1" for the LTC confirmation
./tools/sprt.ps1 -EngineA <candidate.exe> -EngineB <baseline.exe> `
  -NameA candidate -NameB baseline -Elo0 3 -Elo1 10

# Harness calibration after any runner change — same binary on both sides
./tools/sprt.ps1 -EngineA <same.exe> -EngineB <same.exe> -NameA a -NameB b

# Test/tune binaries and the SPSA coverage audit
./tools/build_test.ps1 -Suffix <s>
./tools/audit_spsa_coverage.ps1
```

## Documentation ownership

| File | Audience / purpose |
|---|---|
| `README.md` | Users: install, CPU choice, UCI and build basics |
| `CHANGELOG.md` | Users: visible release deltas and measured claims |
| `RELEASE_NOTES_2.3.2.md` | Copy-ready GitHub release text |
| `PLAN.md` | Maintainers: current state, ownership and ordered roadmap |
| `GUIDE.md` | Maintainers/agents: tracker, commands and operating rules |
| `EXPERIMENTS.md` | Durable evidence, failures and retry triggers |
| `analysis/phase4_counter_spec.md` | Shared 4.1/4.2 counter contract |
| `analysis/phase4_mechanism_map.md` | 4.3 problem/answer/verdict map |
| `tools/spsa_configs/README.md` | Tuning-specific mechanics and lessons |

When facts disagree, source, defaults and reproducible artifacts outrank
prose; fix the prose in the same change.
