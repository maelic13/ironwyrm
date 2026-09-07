# Fused ordinary relocation — 4.11b.9

## Prospective registration

This is a behavior-neutral board-throughput candidate against `af83abf` on
`dev`. The baseline no-feature executable is
`fde1ed0edf2658f487d9959784dbe3634ee830347153feb80f940b8b38bf59a4`.
Only ordinary `QUIET` make/unmake relocations change: mailbox endpoints, the
piece and color occupancies, `all_occ`, and the applicable pawn/minor/non-pawn
keys are updated with one from/to mask and one paired key. The position hash
remains caller-owned. Captures, double pushes, en passant, promotions, castling
and null moves keep their existing paths. Board has no stored PST field, so
there is no PST bookkeeping to maintain in this representation.

The mechanism is less repeated field/key work in the 2.998% full-search
relocation-helper region measured by RAR-M30. The leading alternative is that
LLVM already combines the old remove/add operations, or that flag branching
and larger generated code repay the saved work. The prospective prediction is
a repeatable isolated make/unmake gain (medium confidence) and a 0–1.5%
full-search NPS gain (low confidence; RAR-M30's 1.52% estimate is a ceiling,
not a promised effect). The candidate interacts with evaluation/cache keys,
legality, check detection, TT/repetition identity and undo, but changes no
search or chess policy.

Qualification is frozen before timing:

- The targeted relocation test covers every piece class and checks every
  reconstructed field after make and unmake. Existing seeded differential,
  board-v2, debug/release, formatting and Clippy gates remain mandatory.
- A fresh no-feature build must reproduce `bench 13` at exactly 7,601,220 nodes
  and match the baseline across the frozen 20-root, 600,000-node board-search
  cohort, including depth, seldepth, nodes, score, full PV, best move and ponder.
- Run three alternating board-v2 rounds. Every candidate round must beat its
  paired baseline round for `make/unmake only`; unchanged columns are noise
  controls.
- Run 12 alternating full-search pairs after one discarded warm-up per arm.
  Aggregate each arm as total reported nodes divided by total reported search
  time. Retain only if the candidate median is higher and the independently
  resampled 95% bootstrap interval for the median delta excludes zero. Seed is
  4119 and every pair is retained.

Any semantic mismatch rejects the candidate. A flat/noisy performance result
also rejects it without a game gate or adjacent optimization. Raw executables,
transcripts and recipes remain in ignored `tools/results/relocation-411b9/`.
