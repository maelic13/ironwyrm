//! 4.6.6 — the search node, rebuilt on the reference's step structure.
//!
//! **Why this is a copy first.** `negamax` is 1,682 lines, and most of it is
//! bookkeeping that has nothing to do with search quality: draw detection,
//! Syzygy, TT semantics, PV assembly, the SMP root pool, and the diagnostic
//! instrumentation the whole phase depends on. Rewriting THAT from scratch
//! would risk losing a correctness property in exchange for nothing.
//!
//! So this file starts as a verbatim copy whose fingerprint is proved
//! identical, and the DECISION logic is then replaced wholesale — the
//! pre-move pruning order, the shallow-depth pruning families, the extension
//! rule, the reduction and the re-search policy. Those are the parts the
//! reference's ~200 Elo of Step 13 and ~200 of Step 16 actually live in, and
//! they are rewritten as one coherent contract rather than patched.
//!
//! Selected by `SearchCore`, so the accepted head is one branch away and the
//! two can be compared in a single binary.

use super::*;

impl Searcher {
    pub(crate) fn negamax_core<P: FnMut() -> SearchEvent + ?Sized>(
        &mut self,
        board: &mut Board,
        mut depth: i32,
        mut alpha: i32,
        beta: i32,
        ply: usize,
        is_pv: bool,
        allow_null: bool,
        excluded: Move,
        cut_node: bool,
        poll: &mut P,
    ) -> i32 {
        if self.check_stop(poll) {
            return 0;
        }
        if ply >= MAX_PLY - 1 {
            return self.corrected_eval(board, ply);
        }
        self.pv_len[ply] = ply;
        self.seldepth = self.seldepth.max(ply);

        if ply > 0 && board.can_declare_draw_in_search() {
            return 0;
        }

        let in_check = board.is_in_check();
        if in_check {
            // Phase 8.2(a): the unconditional in-check extension (`depth += 1`)
            // is REMOVED. It was the first of five stacked protections around
            // checked nodes and the prime EBF suspect — every check bought a
            // full extra ply regardless of whether the check was forcing.
            // Checked nodes now search at their natural depth; a checked node
            // at depth 0 falls through to qsearch, which is safe because
            // qsearch generates the FULL legal movelist when in check (not just
            // captures) and detects mate, so evasions are never missed.
            // The `check_extensions` diag counter is intentionally left defined
            // in diag.rs and now reads 0 — an explicit confirmation the
            // extension is off. Restore this line to revert on H0.
        }

        let mate_alpha = -MATE_SCORE + infra::to_i32(ply);
        let mate_beta = MATE_SCORE - infra::to_i32(ply) - 1;
        alpha = alpha.max(mate_alpha);
        let beta = beta.min(mate_beta);
        if alpha >= beta {
            return alpha;
        }

        if depth <= 0 {
            return self.quiescence(board, alpha, beta, ply, 0, poll);
        }

        // 4.2: counted HERE, after the depth<=0 hand-off, so `nodes` means
        // interior nodes actually searched — which is the oracle's population.
        // Counting at function entry inflated it by every node that
        // immediately became a qnode, and that same node was then counted a
        // second time as `qnodes`. That double count silently deflated every
        // rate taken against `nodes`.
        crate::diag_count!(nodes);
        #[cfg(feature = "diag")]
        if in_check {
            crate::diag_count!(nodes_in_check);
        }

        let original_alpha = alpha;
        let hash = board.hash;
        #[cfg(feature = "diag")]
        let diag_sample = crate::diag::sampled(hash, ply, crate::diag::SAMPLE_MAIN);
        #[cfg(feature = "diag")]
        if diag_sample {
            crate::diag_count!(sampled_main_nodes);
        }
        if let Some(score) = self.syzygy_wdl_score(board, depth, ply, excluded) {
            self.tt.store(TtStore {
                key: hash,
                depth,
                score,
                bound: Bound::Exact,
                mv: Move::NULL,
                ply,
                static_eval: VALUE_NONE,
                is_pv,
                kind: OutcomeKind::Tablebase,
            });
            return score;
        }
        let tt_entry = self.tt.probe(hash);
        // 9.7.5(b): main thread only. If helper work is reaching the thread
        // that owns the answer, this hit rate must RISE with thread count; a
        // flat rate means the helpers are filling a table nobody reads.
        if self.thread_id == 0 {
            crate::diag_count!(main_tt_probes);
            if tt_entry.is_some() {
                crate::diag_count!(main_tt_hits);
            }
        }
        // 4.2: one decode of the probe for the whole node. Mate distance and
        // rule-50 are resolved exactly once here — the pre-4.2 code decoded the
        // same entry twice, at `tt_score` and again inside the cutoff block.
        let ev = NodeEvidence::from_probe(tt_entry, ply, board.halfmove_clock);
        let tt_pv = ev.pv_line(is_pv);
        // 4.2b: captured at node entry, against the window this node was ASKED
        // to resolve. `alpha` is raised by the move loop below, so reading it
        // later would ask a different question.
        #[cfg(feature = "diag")]
        let diag_contradicts = ev.contradicts_window(alpha, beta);
        #[cfg(feature = "diag")]
        if diag_sample {
            if ev.hit {
                crate::diag_count!(tt_sample_hit);
                crate::diag_count!(shadow_4_2_evidence);
                if diag_contradicts {
                    crate::diag_count!(contradict_hits);
                }
                if !is_pv && excluded.is_null() && ev.depth >= depth {
                    match ev.bound {
                        Some(Bound::Exact) => {
                            crate::diag_count!(tt_cut_exact);
                        }
                        Some(Bound::Lower) if ev.score >= beta => {
                            crate::diag_count!(tt_cut_lower);
                        }
                        Some(Bound::Upper) if ev.score <= alpha => {
                            crate::diag_count!(tt_cut_upper);
                        }
                        Some(_) => {
                            crate::diag_count!(tt_bound_not_usable);
                        }
                        None => {}
                    }
                    if ev.contradicts_window(alpha, beta) {
                        crate::diag_count!(tt_bound_contradicts_window);
                    }
                } else if ev.bound.is_some() {
                    // 4.9b: this `else` used to add to `tt_bound_not_usable`
                    // as well, which made one counter answer three unrelated
                    // questions — and the one 4.9 needs is the third. A PV node
                    // and an excluded-move search can never cut whatever the
                    // entry says, and neither population moves with thread
                    // count; a SHALLOW entry is the only cause the "helpers add
                    // entries that cannot cut" hypothesis predicts should grow.
                    // Lumped together they cannot be told apart.
                    //
                    // PV is attributed before depth on purpose: at a PV node the
                    // entry is refused regardless of how deep it is, so PV is
                    // the binding reason even when the entry is also shallow.
                    if is_pv {
                        crate::diag_count!(tt_reject_pv);
                    } else if !excluded.is_null() {
                        crate::diag_count!(tt_reject_excluded);
                    } else {
                        crate::diag_count!(tt_reject_shallow);
                        // How far short, so "marginally too shallow" and
                        // "hopelessly too shallow" are distinguishable: the
                        // first is a replacement-policy question, the second is
                        // not worth chasing at all.
                        crate::diag_add!(
                            tt_reject_shallow_deficit,
                            u64::try_from(depth - ev.depth).unwrap_or(0)
                        );
                    }
                }
            } else {
                crate::diag_count!(tt_sample_miss);
            }
        }
        if !is_pv
            && excluded.is_null()
            && let Some(score) = ev.cutoff_score(depth, alpha, beta)
        {
            // 8.4(a): the TT move just produced a beta cutoff without a
            // search - today it gets zero feedback. Reward it (quiet
            // moves only, main/low-ply/pawn histories) at a tunable
            // fraction of the cutoff bonus. Seed 0 = skip entirely.
            //
            // 4.2 note: still unconditional on provenance, so a depth-0 stand
            // pat can train quiet history through this path. 4.5 owns the
            // attribution guard; changing it here would be an ungated edit.
            if matches!(ev.bound, Some(Bound::Lower))
                && score >= beta
                && self.params.tt_cutoff_bonus_pct != 0
                && let Some(mv) = ev.mv.and_then(|m| board.legal_move(m))
                && !mv.is_capture()
                && !mv.is_promo()
            {
                let bonus = self.history_bonus(depth) * self.params.tt_cutoff_bonus_pct / 100;
                self.update_quiet_history(
                    board.side_to_move(),
                    mv,
                    board.moving_piece(mv),
                    board.pawn_key(),
                    ply,
                    bonus,
                );
            }
            return score;
        }
        let mut tt_move = ev
            .mv
            .and_then(|mv| board.legal_move(mv))
            .unwrap_or(Move::NULL);
        if ply == 0 && !self.root_moves.is_empty() && !self.root_moves.contains(&tt_move) {
            tt_move = Move::NULL;
        }

        #[cfg(feature = "diag")]
        let mut diag_iir_applied = false;
        // 4.2b: a contradicting entry that is deep enough to SUPPRESS IIR — the
        // search trusts it to order this node even though it resolved a
        // different window. A depth penalty would let IIR fire here instead.
        #[cfg(feature = "diag")]
        if diag_sample
            && diag_contradicts
            && excluded.is_null()
            && depth >= 4
            && !tt_move.is_null()
            && !(!is_pv && ev.too_shallow_to_order(depth))
        {
            crate::diag_count!(contradict_iir_suppressed);
        }
        // IIR: reduce depth when we lack a good TT entry to guide move ordering
        if !self.ablated(4)
            && excluded.is_null()
            && depth >= 4
            && (tt_move.is_null() || (!is_pv && ev.too_shallow_to_order(depth)))
        {
            #[cfg(feature = "diag")]
            if diag_sample {
                diag_iir_applied = true;
                crate::diag_count!(iir_applied);
                crate::diag_count!(shadow_4_4_selectivity);
                if is_pv {
                    crate::diag_count!(iir_pv);
                }
                if tt_move.is_null() {
                    crate::diag_count!(iir_no_tt_move);
                } else {
                    crate::diag_count!(iir_shallow_tt);
                }
            }
            depth -= 1;
        }

        // 4.2: the pre-4.2 form spelled out three branches whose two `else`
        // arms were identical, because a probe MISS and a hit carrying no
        // stored eval both fall back to a fresh raw eval. `NodeEvidence::MISS`
        // already reports `VALUE_NONE`, so one test covers both.
        let (static_eval, raw_static_eval) = if in_check {
            (VALUE_NONE, VALUE_NONE)
        } else {
            let raw = if ev.raw_static_eval == VALUE_NONE {
                self.raw_eval(board)
            } else {
                ev.raw_static_eval
            };
            (self.corrected_eval_from_raw(board, raw, ply), raw)
        };
        self.stack[ply].static_eval = static_eval;
        // 8.5(b): magnitude of the correction applied to this node's static
        // eval. A large |corr| means the raw eval is being heavily adjusted and
        // is less trustworthy, so the margin/reduction knobs below prune and
        // reduce less. Zero in check (no static eval).
        //
        // ⚠ The comment here used to say "seeds leave every scale at 0, so this
        // term vanishes". That is no longer true and had gone stale: the fitted
        // seeds are `CorrRfpScale = 3`, `CorrFutScale = 3` and
        // `CorrLmrScale = 27`, so this term is LIVE in the accepted baseline.
        //
        // 4.5c: it is also applied to a number the correction may no longer be
        // part of. `eval_for_pruning` below can be REPLACED wholesale by a TT
        // bound (28.5% of sampled hits refine it, RAR-S30), and when that
        // happens the corrected eval is discarded — yet these margins are still
        // widened by the discarded correction's magnitude. That mismatch is what
        // `CorrSkipWhenTtRefined` measures and can switch off.
        let corr_abs = if static_eval == VALUE_NONE {
            0
        } else {
            (static_eval - raw_static_eval).abs()
        };
        // A `ply - 4` fallback for an unusable `ply - 2` was measured and
        // rejected: RAR-S66 stopped at 13,882 games with the LLR receding from
        // a +2.44 peak. `improving = false` after a check is a conservative
        // default, not a defect — there is genuinely no comparable static eval
        // two plies back when that node was in check.
        let improving = !in_check
            && ply >= 2
            && self.stack[ply - 2].static_eval != VALUE_NONE
            && static_eval > self.stack[ply - 2].static_eval;
        let improving_i = if improving { 1 } else { 0 };
        let not_improving_i = 1 - improving_i;
        // 9.7.5 lead: the TT may only stand in for the static eval here if its
        // entry is deep enough to be worth trusting — see the param doc. At the
        // seeded 0 this admits everything, exactly as before.
        let eval_for_pruning = if in_check {
            static_eval
        } else {
            ev.refine_eval(static_eval, 0)
        };
        // 4.5c: when a TT bound replaced the corrected eval, the correction is
        // no longer present in the number the margins test, so charging an
        // uncertainty penalty for it is charging for an adjustment that is not
        // there. At the seeded 0 this is exactly the prior behaviour.
        let corr_abs =
            if self.params.corr_skip_when_tt_refined != 0 && eval_for_pruning != static_eval {
                0
            } else {
                corr_abs
            };
        #[cfg(feature = "diag")]
        if corr_abs != 0 && eval_for_pruning != static_eval {
            crate::diag_count!(corr_applied_to_replaced_eval);
        }
        #[cfg(feature = "diag")]
        if diag_sample
            && eval_for_pruning != VALUE_NONE
            && static_eval != VALUE_NONE
            && eval_for_pruning != static_eval
        {
            crate::diag_count!(tt_eval_refined);
            let delta = u64::from(eval_for_pruning.saturating_sub(static_eval).unsigned_abs());
            crate::diag_add!(tt_eval_delta_sum, delta);
            // 4.2b: an entry that told this node nothing still moved the eval
            // its forward pruning runs on. Slack is measured against the knob
            // that actually gates the refinement.
            if diag_contradicts {
                crate::diag::record_contradiction_refine(ev.depth, delta);
            }
        }
        // 8.3 diagnostic: a non-PV, non-check node where the *stored* PV bit
        // (tt_pv true while is_pv false) is what keeps the whole forward-pruning
        // block below from running.
        if tt_pv && !is_pv && !in_check && excluded.is_null() {
            crate::diag_count!(tt_pv_veto);
            // 4.4a sizing: of the nodes this shared veto blocks, how many would
            // each mechanism actually reach if its own switch handed them back?
            // Depth preconditions only — the margin tests need the eval, which
            // is what the veto is denying them.
            #[cfg(feature = "diag")]
            {
                if depth <= 8 {
                    crate::diag_count!(tt_pv_veto_rfp_eligible);
                }
                if depth <= 3 {
                    crate::diag_count!(tt_pv_veto_razor_eligible);
                }
                if allow_null && depth >= 3 && board.has_non_pawn_material(board.side_to_move()) {
                    crate::diag_count!(tt_pv_veto_nmp_eligible);
                }
                if depth >= 4 {
                    crate::diag_count!(tt_pv_veto_probcut_eligible);
                }
            }
        }
        // 4.4a: the shared `!tt_pv` veto becomes four per-mechanism predicates.
        // At the seeded zeros `tt_pv_allows_any` is false, so this outer test is
        // exactly the old `!tt_pv && ...` — including the fast path, so a
        // `tt_pv` node still skips the margin arithmetic entirely.
        let rfp_tt_pv_ok = !tt_pv || self.params.rfp_allow_tt_pv != 0;
        let razor_tt_pv_ok = !tt_pv || self.params.razor_allow_tt_pv != 0;
        let nmp_tt_pv_ok = !tt_pv || self.params.nmp_allow_tt_pv != 0;
        let probcut_tt_pv_ok = !tt_pv || self.params.probcut_allow_tt_pv != 0;
        let tt_pv_allows_any = rfp_tt_pv_ok || razor_tt_pv_ok || nmp_tt_pv_ok || probcut_tt_pv_ok;
        if tt_pv_allows_any && !in_check && excluded.is_null() {
            let futility_margin = (self.params.futility_base
                + self.params.futility_not_improving * not_improving_i)
                * depth
                + corr_abs * self.params.corr_rfp_scale / 128; // 8.5(b)
            // 4.3 shadow, part 1. Evaluate all three forward-pruning predicates
            // twice — once on the refined eval the search will actually use, once
            // on the unrefined static eval — and count the disagreements. Placed
            // here, before RFP can return, so every consumer is covered by one
            // block and the sample set is identical for all three. Diagnostic
            // only: nothing below reads these, and `eval_for_pruning` is
            // untouched.
            #[cfg(feature = "diag")]
            if diag_sample && eval_for_pruning != static_eval {
                crate::diag_count!(refine_flip_nodes);
                let nmp_bar = beta
                    - self.params.nm_depth_coeff * depth
                    - self.params.nm_improving_bonus * improving_i;
                let nmp_gated =
                    allow_null && depth >= 3 && board.has_non_pawn_material(board.side_to_move());
                // Written out per consumer rather than as an array keyed by an
                // index: a `(refined, plain, which)` tuple plus a `_` arm is the
                // positional-sentinel shape the clean-code policy rules out, and
                // it would silently mislabel a fourth consumer as NMP.
                if depth <= 8 {
                    match (
                        eval_for_pruning - futility_margin >= beta,
                        static_eval - futility_margin >= beta,
                    ) {
                        (true, false) => crate::diag_count!(refine_flip_rfp_on),
                        (false, true) => crate::diag_count!(refine_flip_rfp_off),
                        _ => {}
                    }
                }
                if depth <= 3 {
                    let bar = self.params.razoring_coeff * depth;
                    match (eval_for_pruning + bar < alpha, static_eval + bar < alpha) {
                        (true, false) => crate::diag_count!(refine_flip_razor_on),
                        (false, true) => crate::diag_count!(refine_flip_razor_off),
                        _ => {}
                    }
                }
                if nmp_gated {
                    match (eval_for_pruning >= nmp_bar, static_eval >= nmp_bar) {
                        (true, false) => crate::diag_count!(refine_flip_nmp_on),
                        (false, true) => crate::diag_count!(refine_flip_nmp_off),
                        _ => {}
                    }
                }
            }
            if !self.ablated(1)
                && rfp_tt_pv_ok
                && depth <= 8
                && eval_for_pruning - futility_margin >= beta
            {
                crate::diag_count!(rfp_cut);
                return eval_for_pruning;
            }
            if !self.ablated(0)
                && razor_tt_pv_ok
                && depth <= 3
                && eval_for_pruning + self.params.razoring_coeff * depth < alpha
            {
                crate::diag_count!(razor_drop);
                return self.quiescence(board, alpha, beta, ply, 0, poll);
            }
            // 4.4b: which eval the null threshold may read. At the seeded 0
            // this is `eval_for_pruning`, exactly as before.
            let nmp_eval = if self.params.nmp_use_static_eval != 0 {
                static_eval
            } else {
                eval_for_pruning
            };
            if !self.ablated(2)
                && allow_null
                && nmp_tt_pv_ok
                // 4.4a: with the switch on, a null-verification subtree may not
                // null-prune anywhere inside itself, not merely at its root.
                && (self.params.nmp_suppress_null_in_verification == 0
                    || self.nmp_verify_nesting == 0)
                // 4.4b: restrict to nodes the caller expects to fail high.
                && (self.params.nmp_require_cut_node == 0 || cut_node)
                // 4.4c: a node that hinges on one move is the worst place to
                // trust a null refutation. Evidence-only, so it slightly
                // over-approximates - the conservative direction.
                && (self.params.nmp_singular_guard == 0
                    || !(depth >= 4
                        && ev.mv.is_some()
                        && ev.allows_singular(
                            depth,
                            self.params.singular_tt_depth_margin,
                            self.params.singular_reject_speculative != 0,
                        )))
                && depth >= 3
                && nmp_eval
                    >= beta
                        - self.params.nm_depth_coeff * depth
                        - self.params.nm_improving_bonus * improving_i
                && self.nmp_material_ok(board)
            {
                #[cfg(feature = "diag")]
                if self.nmp_verify_nesting > 0 {
                    // Exact, not sampled, and this IS the population
                    // `NmpSuppressNullInVerification` refuses: same predicate,
                    // so the counter and the switch cannot drift apart.
                    crate::diag_count!(nmp_nested_attempt);
                }
                // 4.10a: NMP running at a mate-range WINDOW. This was the
                // `NmpDecisiveGuard` population; the switch is gone (4.10a
                // removed it as an efficiency guard worth 0.004% of nodes) but
                // the count is kept, because it is the context for the unproven
                // -mate question below, which is a different condition.
                #[cfg(feature = "diag")]
                if beta.abs() >= MATE_SCORE - infra::to_i32(MAX_PLY) {
                    crate::diag_count!(nmp_decisive_population);
                }
                #[cfg(feature = "diag")]
                if diag_sample {
                    crate::diag_count!(nmp_attempt);
                    crate::diag_count!(shadow_4_4_selectivity);
                    if eval_for_pruning != static_eval {
                        crate::diag_count!(nmp_eval_tt);
                    } else if static_eval != raw_static_eval {
                        crate::diag_count!(nmp_eval_corrected);
                    } else {
                        crate::diag_count!(nmp_eval_raw);
                    }
                }
                let reduction = 4 + depth / 4 + ((nmp_eval - beta) / 200).clamp(0, 3);
                board.make_null_move();
                self.tt.prefetch(board.hash);
                let score = -self.negamax_core(
                    board,
                    depth - reduction,
                    -beta,
                    -beta + 1,
                    ply + 1,
                    false,
                    false,
                    Move::NULL,
                    true,
                    poll,
                );
                board.unmake_null_move();
                if self.stopped || self.quit {
                    return 0;
                }
                if score >= beta {
                    crate::diag_count!(nmp_cut);
                    // 4.10a CORRECTNESS REPAIR. A null-move cutoff
                    // establishes only "at least beta"; when the reduced null
                    // search comes back in mate range, Rarog returns that mate
                    // score, claiming a forced mate no real line demonstrated.
                    // It then travels through the TT as a Lower bound.
                    // Stockfish clamps this to beta
                    // (`if (nullValue >= VALUE_MATE_IN_MAX_PLY) nullValue = beta;`).
                    //
                    // Measured: 11 such returns at `bench 13`, 195 at `bench 18`.
                    // The removed `NmpDecisiveGuard` did NOT cover it — its
                    // predicate was on the WINDOW, and its population at depth 13
                    // is ZERO while these 11 still occurred.
                    // CLAMPED: a fail-high asserts only "at least beta", so
                    // that is what is returned. The cutoff is preserved exactly
                    // — the value is still >= beta — while the unproven mate is
                    // refused. Measured cost: bench 13 6,502,902 -> 6,519,711
                    // (+0.26%), bench 18 +22.9%, because a weaker fail-high
                    // cuts less higher up in mate-heavy subtrees. That cost is
                    // what the registered non-inferiority gate is deciding.
                    let score = if score >= MATE_SCORE - infra::to_i32(MAX_PLY) {
                        crate::diag_count!(nmp_cut_unproven_mate);
                        beta
                    } else {
                        score
                    };
                    #[cfg(feature = "diag")]
                    if diag_sample {
                        crate::diag_count!(nmp_sample_cut);
                    }
                    if depth >= 10 {
                        crate::diag_count!(nmp_verify_attempt);
                        let verify_depth = (depth - reduction).max(1);
                        self.nmp_verify_nesting += 1;
                        let verified = self.negamax_core(
                            board,
                            verify_depth,
                            beta - 1,
                            beta,
                            ply,
                            false,
                            false,
                            Move::NULL,
                            false,
                            poll,
                        );
                        self.nmp_verify_nesting -= 1;
                        if self.stopped || self.quit {
                            return 0;
                        }
                        if verified < beta {
                            crate::diag_count!(nmp_verify_fail);
                            // Continue normally when the null cutoff is not stable
                            // under a verification search with null move disabled.
                        } else {
                            crate::diag_count!(nmp_verify_pass);
                            return score;
                        }
                    } else {
                        return score;
                    }
                }
            }

            if !self.ablated(3) && probcut_tt_pv_ok && depth >= 4 {
                // Per NODE entering the block, before capture generation, so
                // nodes with no eligible capture are counted here too. This
                // carried the `probcut_attempt` name until 4.7c prep, and was
                // differenced against the oracle's per-MOVE counter of the same
                // name -- the RAR-S25 denominator shape. See the RAR-S55
                // correction.
                #[cfg(feature = "diag")]
                if diag_sample {
                    crate::diag_count!(probcut_nodes);
                    crate::diag_count!(shadow_4_4_selectivity);
                }
                let probcut_beta = beta + self.params.probcut_margin;
                // 4.7c PROBCUT MOVE FILTER. The entry contract for the
                // speculative capture search moves from "this capture does not
                // lose material" to "this capture can plausibly bridge the gap
                // to probcut_beta".
                //
                // RAR-S55 v3 measured what the old contract costs. Per node the
                // two engines convert alike -- 22.7% against the reference's
                // 25.2% -- so the yield was never the divergence. The PRICE was:
                // Rarog searched 5.17x the normalised ProbCut moves and
                // converted 32.6% of them against 71.9%. Two in three of its
                // ProbCut move-searches produced nothing.
                //
                // `see_ge(mv, 0)` admits any capture that is not outright
                // losing, which is unrelated to the question this search asks.
                // The gap `probcut_beta - static_eval` IS that question in
                // material terms, and it is floored at 0 so the filter can only
                // tighten the old contract, never loosen it -- a negative
                // threshold would admit losing captures at nodes already above
                // probcut_beta, which is de-selectivity nothing here motivates.
                //
                // `static_eval` is real: the whole block is under `!in_check`.
                // i32 throughout: the gap is bounded by the mate range, so
                // gap * 100 cannot approach i32's limit.
                let see_threshold =
                    ((probcut_beta - static_eval) * self.params.probcut_see_gap_scale / 100).max(0);
                // The flat cap of 8 had no stated derivation. Scale it by the
                // node's own prediction instead: a cut node is where a fail-high
                // is expected and the speculative search is likeliest to pay.
                let move_cap = self.params.probcut_move_cap_base
                    + if cut_node {
                        self.params.probcut_move_cap_cut_bonus
                    } else {
                        0
                    };
                let captures = board.generate_legal_captures();
                let mut scored = self.score_tactical_moves(board, captures.as_slice(), tt_move);
                let mut searched_here = 0i32;
                for index in 0..scored.len() {
                    if searched_here >= move_cap {
                        break;
                    }
                    let picked = pick_next(scored.as_mut_slice(), index);
                    let mv = picked.mv;
                    if !board.see_ge(mv, see_threshold) {
                        continue;
                    }
                    searched_here += 1;
                    // Per MOVE: a ProbCut search is about to start. This is the
                    // counter the oracle's `probcut_attempt` can be differenced
                    // against -- placed after the eligibility filter and before
                    // the qsearch, exactly where the oracle places its own.
                    // Up to 8 of these can fire at a single node.
                    #[cfg(feature = "diag")]
                    if diag_sample {
                        crate::diag_count!(probcut_attempt);
                    }
                    let probcut_piece = board.moving_piece(mv);
                    // ProbCut's child is a verification search, not a
                    // reduced sibling, so it consumes neither selectivity
                    // input. Written explicitly rather than left stale.
                    self.push_move(ply, mv, probcut_piece, 0, 0);
                    board.make_move_unchecked(mv);
                    self.tt.prefetch(board.hash);
                    let score =
                        -self.quiescence(board, -probcut_beta, -probcut_beta + 1, ply + 1, 0, poll);
                    let score = if score >= probcut_beta {
                        #[cfg(feature = "diag")]
                        if diag_sample {
                            crate::diag_count!(probcut_qpass);
                        }
                        -self.negamax_core(
                            board,
                            depth - 4,
                            -probcut_beta,
                            -probcut_beta + 1,
                            ply + 1,
                            false,
                            false,
                            Move::NULL,
                            true,
                            poll,
                        )
                    } else {
                        score
                    };
                    board.unmake_move(mv);
                    self.clear_move(ply);
                    if self.stopped || self.quit {
                        return 0;
                    }
                    if score >= probcut_beta {
                        // Deliberately EXACT, like every other `*_cut` counter
                        // (`rfp_cut`, `nmp_cut`, `see_prune`). The core set is
                        // guarded inconsistently on purpose: the spec's chosen
                        // resolution is `RAROG_DIAG_SAMPLE_STRIDE=1`, which
                        // makes the sampled half exact in one place rather than
                        // lifting counters out of guards in the hottest file.
                        // Never read this against `probcut_attempt` at the
                        // default stride.
                        crate::diag_count!(probcut_cut);
                        let cutoff_score = score - (probcut_beta - beta);
                        self.tt.store(TtStore {
                            key: hash,
                            depth: depth - 3,
                            // Which value to persist is an ablation, NOT part of
                            // the speculative contract: the producer bit keeps
                            // this result out of singular seeding either way.
                            // Storing the actual fail-high costs +5.55%
                            // time-to-depth on its own (RAR-S34), so the
                            // conservative margin-shifted value is the default.
                            score: if self.params.probcut_store_actual_score != 0 {
                                score
                            } else {
                                cutoff_score
                            },
                            bound: Bound::Lower,
                            mv,
                            ply,
                            static_eval: raw_static_eval,
                            is_pv: false,
                            kind: OutcomeKind::ProbCut,
                        });
                        #[cfg(feature = "diag")]
                        if diag_sample {
                            crate::diag_count!(probcut_tt_store);
                        }
                        return cutoff_score;
                    }
                }
            }
        }

        let mut move_picker = if in_check || ply == 0 || !excluded.is_null() {
            let legal_moves = board.generate_legal_movelist();
            if legal_moves.is_empty() {
                return if in_check {
                    -MATE_SCORE + infra::to_i32(ply)
                } else {
                    0
                };
            }

            let root_moves;
            let legal_moves = if ply == 0 && !self.root_moves.is_empty() {
                root_moves = legal_moves
                    .iter()
                    .copied()
                    .filter(|mv| self.root_moves.contains(mv))
                    .collect::<Vec<_>>();
                if root_moves.is_empty() {
                    legal_moves.as_slice()
                } else {
                    root_moves.as_slice()
                }
            } else {
                legal_moves.as_slice()
            };

            let mut scored = self.score_moves(board, legal_moves, tt_move, ply);
            // 8.13: order the root list from the POOL's view. A move another
            // thread has already proven good at a deeper depth is tried first
            // here too, so threads stop re-deriving each other's refutations.
            // Applied BEFORE the rotation below, which diversifies on top.
            if ply == 0 && scored.len() > 1 {
                // No-op serially: with no shared state there are no pool
                // scores to fold in.
                self.apply_shared_root_scores(legal_moves, &mut scored);
            }
            // Helpers rotate their root list on top of the pool ordering, so
            // the pool's shared view refines the ordering without collapsing
            // every thread onto the same tree.
            let rotate = self.root_move_offset > 0;
            if ply == 0 && rotate && scored.len() > 1 {
                let offset = self.root_move_offset % scored.len();
                diversify_root_scores(scored.as_mut_slice(), offset);
            }
            MovePicker::full(scored, tt_move)
        } else {
            MovePicker::staged(self, board, tt_move, ply)
        };
        let mut best_move = Move::NULL;
        let mut best_score = -INF_SCORE;
        let mut searched = 0usize;
        // 4.5.5: every move the loop LOOKED at, pruned or not. `searched`
        // counts only those actually searched and keeps that meaning, because
        // the PVS first-move logic and the mate/stalemate test depend on it.
        let mut considered = 0usize;
        #[cfg(feature = "diag")]
        let diag_order_sample = diag_sample && excluded.is_null();
        #[cfg(feature = "diag")]
        let mut diag_best_rank = 0usize;
        #[cfg(feature = "diag")]
        let mut diag_best_stage = MoveClass::BadCapture;
        #[cfg(feature = "diag")]
        let mut diag_best_reduced = false;
        let mut legal_move_seen = false;
        // 10.3: per-node check masks, built at most once and reused by every
        // move at this node — for the pruning-side `move_gives_check` calls
        // and for the `make_move` check hint below. `board` is restored by
        // `unmake_move` each iteration, so these stay valid for the whole loop.
        let mut node_ci: Option<CheckInfo> = None;
        // 4.5.1: set when the TT move is proved singular at THIS node. Read by
        // the node's later moves, which is safe because the TT move is searched
        // first, so the flag is settled before any move LMR can apply to.
        let mut tt_move_singular = false;
        // 4.6.5: latch so `skip_quiets_nodes` counts NODES, not calls.
        let mut skip_quiets_latched = false;
        // 4.7b: latch so `lmp_nodes` counts NODES, not moves -- the oracle can
        // only observe the per-node event, so that is the comparable unit.
        #[cfg(feature = "diag")]
        let mut diag_node_lmp_seen = false;
        let mut quiets = crate::board::MoveList::new();
        let mut good_caps = BadCaptureList::new();
        let mut bad_caps = BadCaptureList::new();
        let previous_move = if ply > 0 {
            self.stack[ply - 1].mv
        } else {
            Move::NULL
        };
        while let Some(picked) = move_picker.next(self, board) {
            let mv = picked.mv;
            if mv == excluded {
                continue;
            }
            legal_move_seen = true;
            // 4.5.5: incremented HERE, where the reference increments, so a
            // move pruned below still advances the index.
            considered += 1;
            // The index the selectivity mechanisms use. Behind a switch, so
            // the accepted fingerprint holds while it is 0.
            let move_index = if self.params.selectivity_count_considered != 0 {
                considered - 1
            } else {
                searched
            };
            let is_capture = mv.is_capture();
            let is_quiet = board.is_quiet_move(mv);
            let mut see = if is_capture { picked.see as i32 } else { 0 };
            let moving_piece = board.moving_piece(mv);
            let captured_piece = board.captured_piece(mv);
            // 4.2: the pre-move evidence snapshot, taken at pick time. It
            // replaces a bare `0..3` stage integer, and 4.6 extends it with the
            // check/evasion taxonomy and the shared prospective depth. `see` is
            // captured here deliberately: the local below is refined for some
            // moves, and classification must not depend on where it is read.
            let move_ev = MoveEvidence::new(
                mv == tt_move,
                is_capture,
                is_quiet,
                see,
                if is_quiet { picked.quiet_history } else { 0 },
            );
            let quiet_hist = move_ev.quiet_history;
            let mut gives_check = None;
            #[cfg(feature = "diag")]
            if diag_order_sample {
                match move_ev.class {
                    MoveClass::TtMove => {
                        crate::diag_count!(move_seen_tt);
                    }
                    MoveClass::GoodCapture => {
                        crate::diag_count!(move_seen_good_capture);
                    }
                    MoveClass::Quiet => {
                        crate::diag_count!(move_seen_quiet);
                    }
                    MoveClass::BadCapture => {
                        crate::diag_count!(move_seen_bad_capture);
                    }
                }
            }
            #[cfg(feature = "diag")]
            let mut diag_move_reduced = false;

            // 4.6b: ONE prospective depth for LMP, futility, SEE pruning and
            // LMR. The audit's finding was that later pruning did not use the
            // depth the move would actually be searched at — LMR reduced it,
            // while the pruning tests all read raw `depth`, so a move about to
            // be searched 3 plies shallower was still judged as if it were not.
            //
            // `r_units_estimate` is the shared reduction; `prospective_depth` is
            // what remains after it. Both are computed pre-move so the pruning
            // consumers can see them, and the LMR site debug-asserts it derives
            // the same units.
            // 4.5.1: built ONCE and used at both sites, so the prospective
            // depth and the applied reduction cannot drift apart by
            // construction rather than by assertion. At the root there is no
            // parent, and 0 is the inert value for both parent inputs.
            let reduction_inputs = ReductionInputs {
                depth,
                searched: move_index,
                is_quiet,
                see,
                tt_pv,
                cut_node,
                quiet_hist,
                corr_abs,
                ev_is_exact: ev.is_exact(),
                tt_move_is_null: tt_move.is_null(),
                improving,
                is_root: ply == 0,
                parent_move_count: if ply > 0 {
                    self.stack[ply - 1].move_count
                } else {
                    0
                },
                parent_stat_score: if ply > 0 {
                    self.stack[ply - 1].stat_score
                } else {
                    0
                },
                tt_move_is_capture: !tt_move.is_null() && tt_move.is_capture(),
                tt_move_singular,
            };
            let r_units_estimate = self.lmr_reduction_units(reduction_inputs);
            // `depth - 1` is the child's nominal depth; subtract the estimated
            // reduction and floor at 1 so a consumer never reads a depth that
            // would make its own `depth <= N` guards nonsensical.
            // 4.6.6: the prospective depth is computed inside Step 13 now,
            // as `lmr_depth`, floored at 0 rather than 1 — a move reduced to
            // nothing should be judged as depth 0, not as depth 1.
            // At the seeded 0 every consumer keeps reading raw `depth`, so this
            // lands inert; 1 switches all four onto the shared depth together,
            // because switching them one at a time would recreate exactly the
            // incoherence 4.6 exists to remove.
            // 4.6.6: `sel_depth` and `SelectivityProspectiveDepth` are GONE from
            // the rebuilt node. The switch existed because the pruning consumers
            // and the reduction disagreed about which depth a move would be
            // searched at; Step 13 now keys on `lmr_depth` unconditionally, so
            // there is no longer a question for a flag to answer.
            // ─────────────────────────────────────────────────────────────
            // Step 13. Pruning at shallow depth.
            //
            // Rebuilt as one contract. Four things changed against the node
            // this was copied from, and each was measured or read out of the
            // differential rather than guessed:
            //
            //  1. Every gate is keyed on `lmr_depth`, the depth this move will
            //     ACTUALLY be searched at, not on the node's raw depth. The old
            //     node had this available and switched OFF, so a move about to
            //     be searched three plies shallower was judged as if it were
            //     not. This is the single largest piece: x0.866 of the tree on
            //     its own.
            //  2. `move_index` counts every move CONSIDERED. The old counter
            //     advanced only for moves actually searched, so pruning a move
            //     withheld the increment that drives more pruning — a
            //     mechanism suppressing its own trigger. `LmpCountBase` pinned
            //     at its lower rail is what that looked like from the tuner.
            //  3. Once move-count pruning fires the picker is told to stop
            //     emitting quiets, instead of generating and scoring every one
            //     of them and rejecting them individually.
            //  4. The quiet branch gains a SEE prune and the capture branch
            //     gains history and futility prunes. Rarog had one SEE prune in
            //     total, in the capture branch, behind `see < 0` — which is why
            //     `see_prune` ran at 0.20x the reference's per-node rate.
            //
            // The gate itself follows the reference: no `!tt_pv` term, because
            // the reference prunes on PV lines too, and `best_score` above the
            // mate floor stands in for "at least one move has returned".
            if !in_check
                && ply > 0
                && best_score > -MATE_SCORE + infra::to_i32(MAX_PLY)
                && board.has_non_pawn_material(board.side_to_move())
            {
                let new_depth = depth - 1;
                let lmr_depth = (new_depth
                    - lmr_reduction(
                        r_units_estimate,
                        new_depth,
                        self.params.lmr_min_reduced_depth,
                    ))
                .max(0);

                if is_quiet {
                    // Move count. Computed first because it also closes the
                    // quiet stage of the picker.
                    let move_count_pruning = move_index
                        > late_move_prune_count(lmr_depth, improving, self.params.lmp_count_base);
                    if move_count_pruning && !self.ablated(5) {
                        if !skip_quiets_latched {
                            skip_quiets_latched = true;
                            crate::diag_count!(skip_quiets_nodes);
                        }
                        move_picker.skip_quiets();
                    }

                    // Eval-based move-count prune, kept from the old node: a
                    // late quiet in a position already far below alpha.
                    let prune_margin = (self.params.lmp_base
                        + self.params.lmp_not_improving * not_improving_i)
                        * lmr_depth;
                    let lmp = move_count_pruning
                        || (lmr_depth <= 3 && eval_for_pruning + prune_margin <= alpha);

                    // History. One threshold on Rarog's own composite rather
                    // than the reference's two continuation slots — the same
                    // information, expressed in the units this engine already
                    // tunes.
                    let history_prune = lmr_depth < self.params.quiet_hist_prune_depth
                        && quiet_hist < -(self.params.quiet_hist_prune_coeff * (lmr_depth + 1));

                    if (lmp || history_prune)
                        && !self.ablated(5)
                        && !move_gives_check(board, &mut node_ci, mv, &mut gives_check)
                    {
                        #[cfg(feature = "diag")]
                        if !diag_node_lmp_seen {
                            diag_node_lmp_seen = true;
                            crate::diag_count!(lmp_nodes);
                        }
                        crate::diag_count!(lmp_prune);
                        continue;
                    }

                    // Futility, with the history conjunct the old node lacked:
                    // a quiet move that looks hopeless on eval is still worth a
                    // look when its history is good.
                    if !self.ablated(5)
                        && lmr_depth < self.params.fp_depth
                        && eval_for_pruning
                            + self.params.fp_base
                            + self.params.fp_coeff * lmr_depth
                            + corr_abs * self.params.corr_fut_scale / 128
                            <= alpha
                        && quiet_hist < self.params.fp_hist_cap
                        && !move_gives_check(board, &mut node_ci, mv, &mut gives_check)
                    {
                        crate::diag_count!(quiet_futility_prune);
                        continue;
                    }

                    // A quiet move that hangs material. Requires the
                    // quiet-aware SEE: the plain one answers every non-capture
                    // with its immediate gain, which is 0, so it can never
                    // report that a quiet move loses a piece.
                    if !self.ablated(5)
                        && lmr_depth < self.params.quiet_see_prune_depth
                        && !board.see_ge_quiet_aware(
                            mv,
                            (-self.params.quiet_see_prune_coeff * lmr_depth * lmr_depth)
                                .max(-self.params.see_pruning_max),
                        )
                        && !move_gives_check(board, &mut node_ci, mv, &mut gives_check)
                    {
                        crate::diag_count!(quiet_see_prune);
                        continue;
                    }
                } else if is_capture {
                    let cap_hist = captured_piece.map_or(0, |cap| {
                        self.cap_history[moving_piece as usize][mv.to_sq().index()][cap as usize]
                            as i32
                    });

                    // Capture history. A capture the history says has never
                    // worked, at a depth where it will barely be searched.
                    if !self.ablated(5)
                        && lmr_depth < 1
                        && cap_hist < 0
                        && !move_gives_check(board, &mut node_ci, mv, &mut gives_check)
                    {
                        crate::diag_count!(capture_hist_prune);
                        continue;
                    }

                    // Futility for captures: even winning the captured piece
                    // outright does not reach alpha.
                    let captured_value = captured_piece.map_or(0, piece_value);
                    if !self.ablated(5)
                        && lmr_depth < self.params.fp_depth
                        && eval_for_pruning
                            + self.params.cap_fp_base
                            + captured_value
                            + self.params.fp_coeff * lmr_depth
                            <= alpha
                        && !move_gives_check(board, &mut node_ci, mv, &mut gives_check)
                    {
                        crate::diag_count!(capture_futility_prune);
                        continue;
                    }

                    // SEE. No `see < 0` precondition any more: the threshold
                    // scales with depth and decides on its own, so a capture
                    // that merely looks even is still tested.
                    let see_threshold = (-self.params.see_pruning_coeff * depth - cap_hist / 8)
                        .max(-self.params.see_pruning_max);
                    if !self.ablated(5)
                        && !board.see_ge(mv, see_threshold)
                        && !move_gives_check(board, &mut node_ci, mv, &mut gives_check)
                    {
                        crate::diag_count!(see_prune);
                        continue;
                    }
                }
            }

            let child_is_pv = is_pv && searched == 0;
            let mut extension = 0;
            let singular_move_candidate =
                !self.ablated(6) && ply > 0 && mv == tt_move && excluded.is_null() && depth >= 4;
            #[cfg(feature = "diag")]
            if singular_move_candidate
                && ev.speculative_singular_seed_blocked(depth, self.params.singular_tt_depth_margin)
            {
                crate::diag_count!(singular_speculative_seed_blocked);
            }
            if singular_move_candidate
                && ev.allows_singular(
                    depth,
                    self.params.singular_tt_depth_margin,
                    self.params.singular_reject_speculative != 0,
                )
            {
                #[cfg(feature = "diag")]
                if diag_sample {
                    crate::diag_count!(singular_attempt);
                    crate::diag_count!(shadow_4_4_selectivity);
                    if ev.depth == depth - 3 && matches!(ev.bound, Some(Bound::Lower)) {
                        // Since 4.3c this is explicitly only the historical
                        // ProbCut-shaped signature; tagged ProbCut producers
                        // have already been rejected above.
                        crate::diag_count!(singular_probcut_depth_match);
                    }
                    // 4.2b: the verification window is seeded from a score that
                    // resolved a different window.
                    if diag_contradicts {
                        crate::diag_count!(contradict_singular_attempt);
                    }
                }
                let singular_beta = ev.score - self.params.singular_beta_mult * depth;
                let singular_depth = (depth - 1) / 2;
                let singular_score = self.negamax_core(
                    board,
                    singular_depth,
                    singular_beta - 1,
                    singular_beta,
                    ply,
                    false,
                    false,
                    mv,
                    false,
                    poll,
                );
                if self.stopped || self.quit {
                    return 0;
                }
                if singular_score < singular_beta {
                    tt_move_singular = true;
                    extension = if !is_pv
                        && singular_score < singular_beta - self.params.singular_double_margin
                    {
                        #[cfg(feature = "diag")]
                        if diag_sample {
                            crate::diag_count!(singular_extend_two);
                        }
                        2
                    } else {
                        #[cfg(feature = "diag")]
                        if diag_sample {
                            crate::diag_count!(singular_extend_one);
                        }
                        1
                    };
                } else if singular_beta >= beta {
                    #[cfg(feature = "diag")]
                    if diag_sample {
                        crate::diag_count!(singular_multicut);
                        // 4.2b: counted HERE, not below. This arm returns, so
                        // the post-block counter never sees a multi-cut — the
                        // single largest tree effect a contradicting seed can
                        // have was silently missing from the shadow until now.
                        if diag_contradicts {
                            crate::diag_count!(contradict_singular_multicut);
                        }
                    }
                    return singular_beta;
                } else if ev.score >= beta {
                    #[cfg(feature = "diag")]
                    if diag_sample {
                        crate::diag_count!(singular_negative_extension);
                    }
                    extension = -1;
                }
                #[cfg(feature = "diag")]
                if diag_sample && diag_iir_applied && extension != 0 {
                    crate::diag_count!(iir_extension_debt);
                }
                // 4.2b: did that seed change the DEPTH? Extensions and negative
                // extensions only — the multi-cut path returns above and is
                // counted there. Sum the two for total tree effect.
                #[cfg(feature = "diag")]
                if diag_sample && diag_contradicts && extension != 0 {
                    crate::diag_count!(contradict_singular_changed_depth);
                }
            }

            let checking_move =
                if depth >= 3 && move_index >= 2 && (is_quiet || see < 0) && !mv.is_promo() {
                    move_gives_check(board, &mut node_ci, mv, &mut gives_check)
                } else {
                    gives_check.unwrap_or(false)
                };

            // 4.5.1: the child reduces using its parent's move count and the
            // history of the move that led there. `searched` is the count
            // BEFORE this move, so +1 makes it this move's index.
            let searched_i32 = i32::try_from(searched).unwrap_or(i32::MAX);
            self.push_move(
                ply,
                mv,
                moving_piece,
                searched_i32.saturating_add(1),
                if is_quiet { quiet_hist } else { 0 },
            );
            let nodes_before_move = if ply == 0 { self.nodes } else { 0 };
            // 10.3: the check predicate is cheap here (node masks + two
            // bitboard tests) and lets `make_move` skip `calculate_checkers`
            // for the overwhelmingly common non-checking move.
            let mv_gives_check = move_gives_check(board, &mut node_ci, mv, &mut gives_check);
            board.make_move_with_check(mv, mv_gives_check);
            self.tt.prefetch(board.hash);
            let new_depth = depth - 1 + extension;
            #[cfg(feature = "diag")]
            if diag_sample {
                crate::diag_add!(
                    prospective_depth_sum,
                    u64::try_from(new_depth.max(0)).unwrap_or(0)
                );
            }
            let mut score;

            if searched == 0 {
                // Step 17, first move: full window at full depth. The PV is
                // established here and every later move is tested against it.
                score = -self.negamax_core(
                    board,
                    new_depth,
                    -beta,
                    -alpha,
                    ply + 1,
                    child_is_pv,
                    true,
                    Move::NULL,
                    !child_is_pv && !cut_node,
                    poll,
                );
            } else {
                // ─────────────────────────────────────────────────────────
                // Step 16. Reduced depth search.
                //
                // Three changes against the node this was copied from.
                //
                //  1. CAPTURES are reducible. The reference reduces a capture
                //     when the quiet stage is already closed, when winning the
                //     captured piece outright still does not reach alpha, or
                //     at a cut node. The old node reduced a capture only when
                //     its SEE was already negative, so every even-looking
                //     capture was searched at FULL depth no matter how late it
                //     appeared — and captures are the population that arrives
                //     first, so "late capture" means "the ordering already
                //     doubts it".
                //  2. The reduced search keeps at least one ply. The 4.8.1
                //     audit found 46.7% of the old node's reductions landing at
                //     depth 0, answered by quiescence: a prune wearing a
                //     reduction's name, counted in no pruning family.
                //  3. The reduced search FEEDS BACK into continuation history.
                //     A move that was reduced and then beat alpha anyway was
                //     mis-ordered, and one that was reduced and stayed below it
                //     was correctly doubted. The old node threw that signal
                //     away; it is the cheapest ordering evidence in the search,
                //     because the work has already been done.
                let captured_value = captured_piece.map_or(0, piece_value);
                let capture_reducible =
                    skip_quiets_latched || eval_for_pruning + captured_value <= alpha || cut_node;
                // The root gets two extra full-depth moves, as the reference
                // does: the answer is chosen here and a reduced root move can
                // only displace the incumbent by beating alpha while reduced.
                let first_reducible = if ply == 0 { 4 } else { 2 };
                let reducible = !self.ablated(7)
                    && depth >= 3
                    && move_index >= first_reducible
                    && (is_quiet || capture_reducible)
                    && !mv.is_promo()
                    && !in_check
                    && !checking_move;
                let mut did_lmr = false;
                if reducible {
                    let mut r = self.lmr_reduction_units(reduction_inputs);
                    debug_assert_eq!(
                        r, r_units_estimate,
                        "4.6.6: Step 13 and Step 16 must read ONE reduction"
                    );
                    if self.shared_state.is_some() {
                        r += self.next_jitter(64);
                    } else if self.params.lmr_jitter_1t != 0 {
                        r += self.next_jitter(self.params.lmr_jitter_1t);
                    }
                    let reduction = lmr_reduction(r, new_depth, 1);
                    #[cfg(feature = "diag")]
                    {
                        if r < 0 {
                            crate::diag_count!(lmr_floor_clamped);
                        }
                        if ply == 0 {
                            crate::diag_count!(lmr_root_applied);
                            crate::diag_add!(
                                lmr_root_reduction_sum,
                                u64::try_from(reduction).unwrap_or(0)
                            );
                        }
                        diag_move_reduced = reduction > 0;
                        crate::diag_add!(
                            reduction_depth_sum,
                            u64::try_from(reduction).unwrap_or(0)
                        );
                        if reduction == 0 {
                            crate::diag_count!(lmr_zero_reduction);
                        } else {
                            crate::diag_count!(lmr_applied);
                        }
                    }
                    did_lmr = true;
                    score = -self.negamax_core(
                        board,
                        new_depth - reduction,
                        -alpha - 1,
                        -alpha,
                        ply + 1,
                        false,
                        true,
                        Move::NULL,
                        true,
                        poll,
                    );
                    // Verify at full depth only when the search was actually
                    // shortened. A zero reduction already WAS the full-depth
                    // search, and re-running it is pure waste.
                    if reduction > 0 && score > alpha {
                        crate::diag_count!(lmr_research);
                        score = -self.negamax_core(
                            board,
                            new_depth,
                            -alpha - 1,
                            -alpha,
                            ply + 1,
                            false,
                            true,
                            Move::NULL,
                            !cut_node,
                            poll,
                        );
                    }
                } else {
                    score = -self.negamax_core(
                        board,
                        new_depth,
                        -alpha - 1,
                        -alpha,
                        ply + 1,
                        false,
                        true,
                        Move::NULL,
                        true,
                        poll,
                    );
                }
                // Ordering feedback from the reduction, using the score the
                // move finally returned.
                if did_lmr && is_quiet {
                    let feedback = if score > alpha {
                        self.history_bonus(new_depth)
                    } else {
                        -self.history_malus(new_depth)
                    };
                    self.update_continuation_histories(
                        ply,
                        moving_piece,
                        mv.to_sq().index(),
                        feedback,
                    );
                }
                // PV re-search. Guarded on `is_pv` explicitly rather than
                // relying on a null window making it unreachable, and the root
                // re-searches even on a fail-high because it must report a
                // score it can play.
                if is_pv && score > alpha && (ply == 0 || score < beta) {
                    score = -self.negamax_core(
                        board,
                        new_depth,
                        -beta,
                        -alpha,
                        ply + 1,
                        true,
                        true,
                        Move::NULL,
                        false,
                        poll,
                    );
                }
            }
            board.unmake_move(mv);
            self.clear_move(ply);

            if self.stopped || self.quit {
                return 0;
            }

            let move_nodes = if ply == 0 {
                self.nodes.saturating_sub(nodes_before_move)
            } else {
                0
            };
            searched += 1;
            // 8.13: publish EVERY searched root move to the pool with its
            // real bound — a fail-low ("true <= score", Upper) is exactly the
            // "stop re-deriving each other's refutations" knowledge a
            // best-move-only summary would lose. `alpha` is still pre-update
            // here, so the classification reads: cutoff = Lower, raised
            // alpha = Exact, else Upper. Serial searches have no shared state.
            if ply == 0 {
                self.record_root_move_search(mv, depth, score, alpha, beta, move_nodes);
            }
            if score > best_score {
                best_score = score;
                best_move = mv;
                #[cfg(feature = "diag")]
                if diag_sample {
                    diag_best_rank = searched;
                    diag_best_stage = move_ev.class;
                    diag_best_reduced = diag_move_reduced;
                }
                if ply == 0 {
                    self.root_best_nodes = move_nodes;
                }
            }
            if score > alpha {
                alpha = score;
                self.pv_table[ply][ply] = mv;
                let child_len = self.pv_len[ply + 1].max(ply + 1);
                for next_ply in ply + 1..child_len {
                    self.pv_table[ply][next_ply] = self.pv_table[ply + 1][next_ply];
                }
                self.pv_len[ply] = child_len;

                if score >= beta {
                    if excluded.is_null() {
                        // 10.0(a): `searched` was incremented for this move
                        // above, so `== 1` means the node's FIRST move failed
                        // high. Denominator is cutoff_quiet + cutoff_capture,
                        // both counted in this same block. `cfg`-gated rather
                        // than relying on `diag_count!` expanding to nothing,
                        // because the condition would leave an empty `if` in
                        // the default build.
                        #[cfg(feature = "diag")]
                        if searched == 1 {
                            crate::diag_count!(cutoff_first_move);
                        }
                        // 4.2 core: cutoff rank, exact, same block and same
                        // denominator as cutoff_quiet + cutoff_capture. The
                        // buckets must sum to that total and best_rank_1 must
                        // equal cutoff_first_move; both are cross-checks the
                        // oracle satisfies too.
                        #[cfg(feature = "diag")]
                        match searched {
                            1 => crate::diag_count!(best_rank_1),
                            2 | 3 => crate::diag_count!(best_rank_2_3),
                            4..=7 => crate::diag_count!(best_rank_4_7),
                            _ => crate::diag_count!(best_rank_8_plus),
                        }
                        // 8.4(e): the cutoff REWARD is scaled when the node
                        // static eval sat below beta - the search found a good
                        // move the eval did not credit. 100 = neutral; maluses
                        // stay unscaled.
                        let bonus_pct = if static_eval != VALUE_NONE && static_eval < beta {
                            self.params.surprise_bonus_pct
                        } else {
                            100
                        };
                        if !is_capture {
                            crate::diag_count!(cutoff_quiet);
                            self.update_cutoff_tables(
                                board,
                                mv,
                                moving_piece,
                                previous_move,
                                ply,
                                depth,
                                bonus_pct,
                                quiets.as_slice(),
                                &good_caps,
                                &bad_caps,
                            );
                        } else {
                            crate::diag_count!(cutoff_capture);
                            self.update_capture_history(
                                moving_piece,
                                mv.to_sq().index(),
                                captured_piece,
                                self.history_bonus(depth) * bonus_pct / 100,
                            );
                            let malus = self.history_malus(depth);
                            for gc in good_caps.as_slice() {
                                self.update_capture_history(
                                    gc.attacker,
                                    gc.to as usize,
                                    gc.captured,
                                    -malus,
                                );
                            }
                            // 8.4(c): a capture cutoff today penalizes only the
                            // earlier good captures - the searched quiets and
                            // bad captures that failed to cut escape unscathed.
                            // Cross-category malus at a tunable fraction; seed 0
                            // = skip. Good-SEE captures keep the existing malus
                            // only (the all-capture form was bench-vetoed in the
                            // Basilisk cross-review).
                            if self.params.capture_malus_pct != 0 {
                                let xmalus = malus * self.params.capture_malus_pct / 100;
                                let color = board.side_to_move();
                                let pawn_key = board.pawn_key();
                                for &quiet in quiets.as_slice() {
                                    self.update_quiet_history(
                                        color,
                                        quiet,
                                        board.moving_piece(quiet),
                                        pawn_key,
                                        ply,
                                        -xmalus,
                                    );
                                }
                                for bc in bad_caps.as_slice() {
                                    self.update_capture_history(
                                        bc.attacker,
                                        bc.to as usize,
                                        bc.captured,
                                        -xmalus,
                                    );
                                }
                            }
                        }
                        self.tt.store(TtStore {
                            key: hash,
                            depth,
                            score,
                            bound: Bound::Lower,
                            mv,
                            ply,
                            static_eval: raw_static_eval,
                            is_pv: tt_pv,
                            kind: OutcomeKind::Full,
                        });
                        #[cfg(feature = "diag")]
                        if diag_sample {
                            crate::diag_count!(main_store_lower);
                        }
                        if static_eval != VALUE_NONE
                            && score.abs() < MATE_SCORE - infra::to_i32(MAX_PLY)
                            && score > static_eval
                        {
                            crate::diag_count!(correction_updates);
                            // 8.5a diagnostic: correction trained by a *capture*
                            // beta cutoff — the eval learning to absorb search
                            // tactics that then feed back into pruning.
                            if is_capture {
                                crate::diag_count!(correction_on_capture);
                            }
                            let residual = self.attributed_residual(
                                score - static_eval,
                                is_capture,
                                board.halfmove_clock,
                            );
                            self.update_correction(board, residual, depth, ply);
                        }
                    }
                    #[cfg(feature = "diag")]
                    if diag_order_sample && diag_best_rank > 0 {
                        crate::diag::record_best_move(
                            diag_best_rank,
                            diag_best_stage,
                            diag_best_reduced,
                        );
                        if !tt_move.is_null() {
                            crate::diag::record_contradiction_ordering(
                                diag_contradicts,
                                ev.hit,
                                diag_best_stage == MoveClass::TtMove,
                            );
                        }
                    }
                    return score;
                }
            }

            if is_quiet {
                quiets.push(mv);
            } else if is_capture {
                if see == SEE_UNKNOWN as i32 {
                    see = if board.see_ge(mv, 0) { 0 } else { -1 };
                }
                if see >= 0 {
                    good_caps.push(moving_piece, mv.to_sq().0, captured_piece);
                } else {
                    bad_caps.push(moving_piece, mv.to_sq().0, captured_piece);
                }
            }
        }

        if !legal_move_seen {
            return if in_check {
                -MATE_SCORE + infra::to_i32(ply)
            } else {
                0
            };
        }

        let bound = if best_score > original_alpha {
            Bound::Exact
        } else {
            Bound::Upper
        };
        if excluded.is_null()
            && static_eval != VALUE_NONE
            && best_score.abs() < MATE_SCORE - infra::to_i32(MAX_PLY)
        {
            let diff = best_score - static_eval;
            // Update correction for PV nodes (Exact) and fail-lows where score < static_eval
            if bound == Bound::Exact || (bound == Bound::Upper && diff < 0) {
                crate::diag_count!(correction_updates);
                if best_move.is_capture() {
                    crate::diag_count!(correction_on_capture);
                }
                let residual =
                    self.attributed_residual(diff, best_move.is_capture(), board.halfmove_clock);
                self.update_correction(board, residual, depth, ply);
            }
        }
        if excluded.is_null() {
            // 8.4(b): an Exact (PV) node best move improved alpha without
            // cutting - today it gets zero feedback. Reward the QUIET best
            // move at a tunable fraction of the cutoff bonus. REWARD-ONLY by
            // design: no sibling malus, no killer/countermove write, no
            // capture reward (Basilisk cross-review: reward-only +4.90, the
            // sibling-malus form -84.21). Seed 0 = skip.
            if bound == Bound::Exact
                && self.params.exact_bonus_pct != 0
                && !best_move.is_null()
                && !best_move.is_capture()
                && !best_move.is_promo()
            {
                let bonus = self.history_bonus(depth) * self.params.exact_bonus_pct / 100;
                self.update_quiet_history(
                    board.side_to_move(),
                    best_move,
                    board.moving_piece(best_move),
                    board.pawn_key(),
                    ply,
                    bonus,
                );
            }
            self.tt.store(TtStore {
                key: hash,
                depth,
                score: best_score,
                bound,
                mv: best_move,
                ply,
                static_eval: raw_static_eval,
                is_pv: tt_pv,
                kind: OutcomeKind::Full,
            });
            #[cfg(feature = "diag")]
            if diag_sample {
                match bound {
                    Bound::Exact => {
                        crate::diag_count!(main_store_exact);
                    }
                    Bound::Upper => {
                        crate::diag_count!(main_store_upper);
                    }
                    Bound::Lower => {}
                }
                // 4.3 shadow, part 2. This node refined its pruning eval and
                // then completed, so compare which estimate sat closer to the
                // score it reported. Only reachable when the node was NOT
                // pruned — see the counter docs for why that biases it.
                if eval_for_pruning != static_eval && static_eval != VALUE_NONE {
                    crate::diag::record_refine_agreement(static_eval, eval_for_pruning, best_score);
                }
            }
        }
        #[cfg(feature = "diag")]
        if diag_order_sample && diag_best_rank > 0 {
            crate::diag::record_best_move(diag_best_rank, diag_best_stage, diag_best_reduced);
            if !tt_move.is_null() {
                crate::diag::record_contradiction_ordering(
                    diag_contradicts,
                    ev.hit,
                    diag_best_stage == MoveClass::TtMove,
                );
            }
        }
        best_score
    }
}
