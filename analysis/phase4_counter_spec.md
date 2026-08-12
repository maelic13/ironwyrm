# Phase-4 differential counter specification

Status: **contract**, written at PLAN step 4.1, consumed by 4.1 (oracle side)
and 4.2 (Rarog side). Both implementations must satisfy this document rather
than each other, so a disagreement is resolved here, not in whichever code was
written second.

## Why this file exists before either implementation

"Matched name for name" is meaningless without matched *definitions*. RAR-S25
recorded the failure mode directly: a rate is only meaningful when numerator and
denominator are collected at the same point in the code. Two engines emitting a
counter called `lmr_applied` from structurally different places produce a number
that looks comparable and is not, and the whole value of 4.2 is that its
divergences are trustworthy enough to choose the next cluster.

So each counter below fixes: what increments it, exactly where, and which
denominator it is read against.

## Output contract

Both sides emit, at search completion, one line per counter:

```text
info string diag <name> <value>
```

Names are `lower_snake_case` and are the literal label. Counters reset once per
`go`, on the main thread, before any helper is spawned — Rarog has already been
bitten by resetting per-thread, which silently wiped every earlier thread's
contribution and made every multi-thread diagnostic number junk. Values are
process-global; the differential suite runs one thread, but the reset rule holds
regardless.

Diagnostics are compile-time gated on both sides. With them **off**, Rarog must
reproduce `bench 13` = 6,519,711 / EBF 2.449 exactly, and the oracle must
reproduce its frozen `75d0d43` behaviour. The instrumented oracle build is a
diagnostic artifact: it never plays a rating game and never replaces the frozen
tournament binary.

## Comparability tiers

Every counter is in exactly one tier. Mixing them is the error this section
exists to prevent.

| Tier | Meaning | Use |
|---|---|---|
| **Core** | Same mechanism, same definition, both engines | Divergence selects work |
| **Rarog-only** | No analogue in `9587eeeb` | Describes Rarog; never a gap |
| **Oracle-only** | Mechanism Rarog does not have | Candidate idea; never a target |

A Rarog-only counter reading differently from nothing is not a finding. An
oracle-only mechanism existing is a *question for 4.3*, not a defect.

## Core counters

`SF step` refers to the numbered `// Step N.` comments in the vendored
`search.cpp`, which are that file's own structural skeleton.

### Group 0 — denominators

Instrument these first. Every rate below names the denominator it divides by,
and both must be collected at the same point.

| Name | Definition | Rarog site | SF site |
|---|---|---|---|
| `nodes` | Interior (non-qsearch) nodes entered | `search.rs` negamax entry | `search<>()` after Step 1 |
| `qnodes` | Quiescence nodes entered | **gap, see below** | `qsearch<>()` entry |
| `nodes_in_check` | Subset of `nodes` where side to move is in check | negamax entry | `ss->inCheck` at entry |

### Group 1 — move ordering and cutoffs (cluster 4.5)

Denominator: `cutoff_quiet + cutoff_capture`, which is every beta cutoff at a
real, non-excluded interior node.

| Name | Definition | SF step |
|---|---|---|
| `cutoff_quiet` | Beta cutoff whose move was quiet | 19 |
| `cutoff_capture` | Beta cutoff whose move was a capture or promotion | 19 |
| `cutoff_first_move` | Beta cutoff at move index 1 | 19 |
| `best_rank_1` | Cutoff at move index 1 | 19 |
| `best_rank_2_3` | Cutoff at move index 2–3 | 19 |
| `best_rank_4_7` | Cutoff at move index 4–7 | 19 |
| `best_rank_8_plus` | Cutoff at move index ≥ 8 | 19 |
| `move_seen_tt` | Move supplied by the picker as the TT move | movepick |
| `move_seen_good_capture` | Move supplied as a good capture | movepick |
| `move_seen_quiet` | Move supplied as a quiet | movepick |
| `move_seen_bad_capture` | Move supplied as a deferred bad capture | movepick |

`best_rank_*` and `cutoff_first_move` deliberately overlap: `best_rank_1` must
equal `cutoff_first_move`, which is a free cross-check that both sides agree on
what a move index is. If they disagree, the instrumentation is wrong.

### Group 2 — reductions (cluster 4.5)

| Name | Definition | SF step |
|---|---|---|
| `lmr_applied` | A reduced-depth search was performed for a move | 16 |
| `lmr_research` | A reduced search failed high and was re-searched at full depth | 17 |
| `reduction_depth_sum` | Sum of applied reduction in plies | 16 |

Read `reduction_depth_sum / lmr_applied` as mean reduction. Do not read
`lmr_research / lmr_applied` as an error rate without also reading Group 1: a
low re-search rate with poor `best_rank` distribution means moves are being
reduced that never get a chance to fail high, which is the failure RAR-S53
priced at 2.5 plies of unusable depth.

### Group 3 — selectivity (cluster 4.7)

| Name | Definition | SF step |
|---|---|---|
| `razor_drop` | Razoring dropped the node into qsearch | 7 |
| `rfp_cut` | Reverse/child futility returned early | 8 |
| `nmp_attempt` | A null-move search was started | 9 |
| `nmp_cut` | Null move produced a cutoff | 9 |
| `nmp_verify_attempt` | Verification search started | 9 |
| `nmp_verify_pass` | Verification confirmed the cutoff | 9 |
| `nmp_verify_fail` | Verification refuted the cutoff | 9 |
| `probcut_attempt` | A ProbCut search was started | 10 |
| `probcut_cut` | ProbCut produced a cutoff | 10 |
| `lmp_prune` | A move was skipped by move-count pruning | 13 |
| `quiet_futility_prune` | A quiet move was skipped by futility | 13 |
| `see_prune` | A move was skipped by an SEE threshold | 13 |

### Group 3b — prune recall and overlap (cluster 4.7)

The point of 4.2, and the reason node savings are not enough: a smaller tree can
be worse. These are counted per *move considered at a prunable node*, before any
prune is applied, so overlap is observable.

| Name | Definition |
|---|---|
| `prune_shadow_moves` | Moves reaching the shallow-pruning stage — the denominator |
| `prune_shadow_lmp` | Of those, would be pruned by move count |
| `prune_shadow_futility` | Of those, would be pruned by futility |
| `prune_shadow_see` | Of those, would be pruned by SEE |
| `prune_shadow_check_exempt` | Of those, exempted because the move gives check |
| `prune_shadow_overlap_two_plus` | Of those, would be pruned by two or more families |

`prune_shadow_overlap_two_plus / prune_shadow_moves` is redundancy. A high value
means the families are paying for each other's work rather than covering
different populations. Rarog measured this at 0.47% once (RAR-S21), which was
weak evidence against deduplication being a prize — that measurement predates
this spec and is not directly comparable.

### Group 4 — extensions and depth authority (cluster 4.8)

| Name | Definition | SF step |
|---|---|---|
| `check_extensions` | Node extended because the move gives check | 14 |
| `singular_attempt` | A singular verification search was started | 14 |
| `singular_extend_one` | Extended by one ply | 14 |
| `singular_extend_two` | Extended by two plies | 14 |
| `singular_multicut` | Singular search produced a multi-cut return | 14 |
| `singular_negative_extension` | Depth reduced by the singular path | 14 |

### Group 5 — transposition table (cluster 4.6)

| Name | Definition | SF step |
|---|---|---|
| `main_tt_probes` | Interior-node TT probes | 4 |
| `main_tt_hits` | Probes returning an entry for this position | 4 |
| `tt_cut_exact` | Cutoff taken on an exact bound | 4 |
| `tt_cut_lower` | Cutoff taken on a lower bound | 4 |
| `tt_cut_upper` | Cutoff taken on an upper bound | 4 |
| `tt_bound_not_usable` | Hit whose bound could not serve this window | 4 |
| `main_store_exact` | Interior store with an exact bound | 19 |
| `main_store_lower` | Interior store with a lower bound | 19 |
| `main_store_upper` | Interior store with an upper bound | 19 |

### Group 6 — quiescence (cluster 4.6)

| Name | Definition |
|---|---|
| `q_in_check` | Qnodes entered in check |
| `q_tt_hit` | Qsearch TT probe returned an entry |
| `q_tt_cut` | Qsearch cutoff taken from the TT |
| `q_stand_pat_cut` | Qsearch cutoff taken from stand pat |
| `q_move_cut` | Qsearch cutoff taken from a searched move |

### Group 7 — root and aspiration (cluster 4.9)

| Name | Definition |
|---|---|
| `root_iterations` | Completed iterative-deepening iterations |
| `root_best_changes` | Root best-move changes across iterations |
| `asp_fail_high` | Aspiration re-searches after a fail high |
| `asp_fail_low` | Aspiration re-searches after a fail low |

## Rarog-only — do not port, and do not read as gaps

`9587eeeb` predates all of these, or Rarog solves something it does not have.
Their absence on the oracle side is a fact about 2020 Stockfish, not a finding.

| Family | Why there is no analogue |
|---|---|
| `correction_*` | Correction history postdates this revision entirely |
| `rootconf_*` | Rarog's root-confidence model has no counterpart |
| `lazy_*` | Rarog HCE lazy evaluation; the oracle calls the same HCE without it |
| `store_kind_*`, `tt_move_inherited*` | Rarog's typed TT provenance; the oracle has no producer field |
| `tt_pv_veto_*`, `contradict_*`, `refine_*` | Rarog's TT-bound evidence model |
| `shadow_4_*` | Retired shadow slots from the closed Phase-4 line |
| `worker_*` | Rarog's SMP vote merge |

## Learned at 4.1, from the first instrumented reading

Recorded here because each one would otherwise be misread as a finding.

| Observation | Consequence |
|---|---|
| `singular_extend_two` and `singular_negative_extension` are **structurally absent** in `9587eeeb` — both mechanisms postdate it | They read 0 always. Never read as "the oracle extends less": Rarog has them, the reference does not. These are Rarog-only in the oracle's direction |
| `nmp_verify_*` only fires at depth ≥ 13 | A short-depth suite reads 0. Not a gap; size the suite before reading them |
| `prune_shadow_moves` excludes quiets the picker withheld under LMP | The threshold is enforced in the move picker, not by a `continue`, so LMP-removed moves never reach the shadow stage. LMP's overlap with futility and SEE is observable only at the boundary move. `lmp_prune` is therefore counted in the picker |
| A piped `go … quit` aborts the search before it starts | The suite must drive `bench`, which is synchronous. A naive `go depth 10` returns `bestmove a2a3` on the *frozen* binary too — this is a harness trap, not an engine defect |
| `probcut_cut` initially exceeded `probcut_attempt` | The TT-served ProbCut shortcut returns without running a search. It now counts as both. A rate above 1 discredits the whole diagnostic |

### Invariants that must keep holding

These are free cross-checks. If one separates, the instrumentation is wrong,
not the engine. Verified on the oracle at bench depth 9:

| Invariant | Reading |
|---|---|
| `best_rank_1` == `cutoff_first_move` | 1433 == 1433 |
| `best_rank_1 + best_rank_2_3 + best_rank_4_7 + best_rank_8_plus` == `cutoff_quiet + cutoff_capture` | 1726 == 1726 |
| `main_tt_probes` == `nodes` | 6967 == 6967 |
| `main_store_lower` == cutoff total | 1726 == 1726 |
| `probcut_cut` ≤ `probcut_attempt` | holds in all 44 bench positions |

## Oracle-only — questions for 4.3, never targets

| Name | Mechanism | Note |
|---|---|---|
| `iid_applied` | Internal iterative deepening, SF step 11 | Rarog implements **IIR** instead. These are different mechanisms and must not share a name or be differenced. SF's own annotation prices IID at ~1 Elo, so this is a low-value question |

## Known gaps on the Rarog side (4.2 work)

| Gap | Detail |
|---|---|
| `qnodes` | Rarog has only `sampled_qnodes`, a 1/1024 deterministic sample. The differential suite needs an exact count on both sides |
| `prune_shadow_*` | Present, but predate this spec; re-verify each site collects at the stage boundary this document names |
| `reduction_depth_sum` | Present; confirm it sums applied reduction, not prospective depth |

None of these are oracle-side work. They are recorded here so 4.2 starts from a
list rather than a rediscovery.

## What these counters may not be used for

- They may not accept or reject a candidate. Only a registered final-PGO SPRT
  does that.
- A divergence is a question about where to look next, never a target value.
  Closing a gap in a counter is not an outcome.
- Node savings are not strength. Group 3b exists precisely because the tempting
  reading — fewer nodes is better — is the one that has already cost Rarog Elo.
- Values from different suites, depths or node budgets are not comparable. The
  4.2 suite is versioned for this reason.
