/*
  Rarog Phase-4 differential diagnostics for the Stage-1 oracle.

  This file is NOT upstream Stockfish. It is instrumentation added on the
  `hybrid-diag` branch so the oracle can be compared counter-for-counter with
  Rarog under PLAN step 4.1. It implements the contract in
  `analysis/phase4_counter_spec.md` on the Rarog side of the repository:
  counter names are Rarog's, lower_snake_case, emitted verbatim as

      info string diag <name> <value>

  Compile-time gated on RAROG_DIAG. Without it every macro expands to nothing,
  so the default oracle build is behaviourally and structurally the frozen
  `75d0d43` engine. The instrumented build is a diagnostic artifact: it never
  plays a rating game and never replaces the frozen tournament binary.

  Counters are process-global atomics and are reset ONCE per `go`, on the main
  thread, before any helper is spawned. Rarog has already paid for getting that
  wrong: a per-thread reset silently wiped every earlier thread's contribution.
*/

#ifndef RAROG_DIAG_H_INCLUDED
#define RAROG_DIAG_H_INCLUDED

#ifdef RAROG_DIAG

    #include <atomic>
    #include <cstdint>

// The core comparable set from the spec. Order here is the dump order.
// Grouping comments match the spec's group numbers so the two can be diffed.
    #define RAROG_DIAG_COUNTERS(X) \
        /* 0 — denominators */ \
        X(nodes) X(qnodes) X(nodes_in_check) \
        /* 1 — move ordering and cutoffs */ \
        X(cutoff_quiet) X(cutoff_capture) X(cutoff_first_move) \
        X(best_rank_1) X(best_rank_2_3) X(best_rank_4_7) X(best_rank_8_plus) \
        X(move_seen_tt) X(move_seen_good_capture) X(move_seen_quiet) \
        X(move_seen_bad_capture) \
        /* 2 — reductions */ \
        X(lmr_applied) X(lmr_research) X(reduction_depth_sum) \
        /* 3 — selectivity */ \
        X(razor_drop) X(rfp_cut) \
        X(nmp_attempt) X(nmp_cut) \
        X(nmp_verify_attempt) X(nmp_verify_pass) X(nmp_verify_fail) \
        X(probcut_nodes) X(probcut_attempt) X(probcut_cut) \
        X(probcut_tt_served) \
        X(lmp_nodes) X(quiet_futility_prune) X(see_prune) \
        /* 3b — prune recall and overlap */ \
        X(prune_shadow_moves) X(prune_shadow_lmp) X(prune_shadow_futility) \
        X(prune_shadow_see) X(prune_shadow_check_exempt) \
        X(prune_shadow_overlap_two_plus) \
        /* 4 — extensions and depth authority */ \
        X(check_extensions) X(singular_attempt) \
        X(singular_extend_one) X(singular_extend_two) \
        X(singular_multicut) X(singular_negative_extension) \
        /* 5 — transposition table */ \
        X(main_tt_probes) X(main_tt_hits) \
        X(tt_cut_exact) X(tt_cut_lower) X(tt_cut_upper) X(tt_bound_not_usable) \
        X(main_store_exact) X(main_store_lower) X(main_store_upper) \
        /* 6 — quiescence */ \
        X(q_in_check) X(q_tt_hit) X(q_tt_cut) X(q_stand_pat_cut) X(q_move_cut) \
        /* 7 — root and aspiration */ \
        X(root_iterations) X(root_best_changes) \
        X(asp_fail_high) X(asp_fail_low) \
        /* oracle-only: a mechanism Rarog does not have. Rarog implements IIR \
           instead, which is a DIFFERENT mechanism — never difference these. */ \
        X(iid_applied)

namespace RarogDiag {

    #define RAROG_DIAG_DECLARE(name) extern std::atomic<std::uint64_t> name;
RAROG_DIAG_COUNTERS(RAROG_DIAG_DECLARE)
    #undef RAROG_DIAG_DECLARE

// Reset every counter. Call once per `go`, from the main thread only.
void reset();

// Emit one `info string diag <name> <value>` line per counter.
void dump();

}  // namespace RarogDiag

    #define DIAG_COUNT(name) \
        (RarogDiag::name.fetch_add(1, std::memory_order_relaxed))
    #define DIAG_ADD(name, value) \
        (RarogDiag::name.fetch_add(static_cast<std::uint64_t>(value), \
                                   std::memory_order_relaxed))

#else  // !RAROG_DIAG

namespace RarogDiag {
inline void reset() {}
inline void dump() {}
}  // namespace RarogDiag

    #define DIAG_COUNT(name) ((void) 0)
    #define DIAG_ADD(name, value) ((void) 0)

#endif  // RAROG_DIAG

#endif  // RAROG_DIAG_H_INCLUDED
