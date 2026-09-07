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

## Result — `NO_CHANGE`, 2026-09-07

**Disposition: the frozen retention rule rejects the candidate. The production
path is withdrawn; `src/` is byte-identical to `af83abf`.** The targeted
relocation test is retained because it covers baseline behaviour that no
existing test reached. No games, no Elo claim, no adjacent optimization.

### Identity — mandatory gate, passed

Both no-feature executables reproduce `bench 13` at exactly **7,601,220 nodes /
EBF 2.474**, asserted inside the runner before any timing. Across all **12
pairs x 20 roots = 240 paired root answers**, the candidate matched the
baseline on name, repeat, depth, seldepth, reported nodes, score type, score,
best move, **full PV and ponder move**. Measured artifacts are distinct:

| Artifact | SHA-256 |
|---|---|
| `baseline.exe` | `fde1ed0e...bf59a4` (matches the registered baseline) |
| `candidate.exe` | `0da54ca9...cfd9dcf3` |
| `board-baseline.exe` | `72e8be2c...950274` |
| `board-candidate.exe` | `2166a33e...438be192` |
| frozen suite | `0c8cefdf...6b153e3` |

### Isolated make/unmake — registered condition met

Every candidate round beat its paired baseline round, as required:

| Round | Baseline ops/s | Candidate ops/s | Delta |
|---|---|---|---|
| 0 | 41,896,188 | 48,717,977 | **+16.28%** |
| 1 | 42,284,732 | 48,716,613 | **+15.21%** |
| 2 | 42,163,119 | 48,601,553 | **+15.27%** |

The unchanged noise-control columns moved by mixed sign and smaller magnitude
(`legal generation` +1.75/+0.67/-4.93%, `capture generation` +0.77/+0.85/-0.87%,
`threshold SEE only` -0.90/-2.29/-1.31%), so the make/unmake column is the only
one that separates cleanly.

### Full search — registered condition NOT met

Twelve alternating pairs, one discarded warm-up per arm, 20 roots, 600,000
nodes, seed 4119, all pairs retained:

| Quantity | Value |
|---|---|
| Baseline median | 2,182,590 nps |
| Candidate median | 2,204,757 nps |
| Median delta | **+1.016%** |
| Bootstrap 95% interval | **-0.450% to +3.609%** |
| Candidate-faster pairs | 10 / 12 |
| Max host CPU busy | 53.43% |

The interval **includes zero**, so the frozen rule — "retain only if the
candidate median is higher *and* the interval excludes zero" — rejects. Ten of
twelve faster pairs and a positive median are not the registered criterion and
are not treated as one.

### Emitted code — the leading alternative is confirmed, not the mechanism

The registration named "LLVM already combines the old operations, or larger
generated code repays the saved work" as the leading alternative. Comparing the
two release `.s` dumps:

| Symbol | Baseline instrs | Candidate instrs | Delta |
|---|---|---|---|
| `Board::make_move_inner` | 468 | 568 | **+100 (+21.4%)** |
| whole-crate emitted | 87,294 | 87,994 | +700 (+0.80%) |
| defined symbols | 947 | 947 | 0 |

`unmake_move` is inlined and has no standalone symbol in either dump. The fused
path is therefore **not** smaller code: its isolated win comes from fewer
dependent memory operations, while the flag branching costs instructions. That
is consistent with a real but small primitive gain rather than a free one.

### Calibration — what the prediction got right and wrong

- **Isolated gain: HIT.** Predicted "repeatable isolated make/unmake gain,
  medium confidence"; measured +15.2% to +16.3% in all three rounds.
- **Full-search magnitude: HIT on sign and size, MISS on resolvability.**
  Predicted 0-1.5% at low confidence; the point estimate +1.016% lands inside
  that band. RAR-M30 weights make/unmake at **7.143%** of process time, so a
  +15.27% primitive gain projects to **+0.96%** whole-search — against a
  measured +1.02%. The mechanism and its magnitude behaved as reasoned.
- **The miss is in the instrument, not the hypothesis.** Twelve 600,000-node
  pairs cannot separate a ~1% effect from zero; the interval half-width is
  roughly twice the effect. The registration chose a budget too small to
  resolve the size it predicted. That was foreseeable before exposure and was
  not foreseen — record it as a *power* failure, not a mechanism failure.
- **Emitted-code alternative: partially confirmed.** Code grew rather than
  shrank, so "LLVM already fuses this" is refuted while "larger code repays
  part of the saving" is supported.

This is insufficient evidence of deployable value under the registered budget.
It is **not** proof of zero benefit, a regression, or a defect.

### Retry trigger

Do not re-run this candidate as-is. A retry needs, registered prospectively
before any timing: a pooled-PGO production build, an instrument with enough
power for a ~1% effect (materially more pairs or longer searches, with the
required precision computed *before* the run), and a whole-search floor plus
stop rule. The natural place is the **4.11b.16** integrated board candidate,
where several small board effects can be qualified together instead of each
one fighting this noise floor alone. Reviving it standalone before then repeats
a measurement already known to be underpowered.

### Retained verification

`ordinary_relocation_updates_and_restores_every_piece_class` in
`tests/board_correctness.rs` is kept. It exercises pawn, knight, bishop, rook,
queen and king ordinary relocation, asserts the move is not a capture,
promotion or castle, checks origin/target mailbox contents plus full board
consistency after make, and asserts complete snapshot restoration after unmake.
The baseline had no such per-piece-class ordinary-relocation coverage.

Closure checks on the restored tree (`tools/results/relocation-411b9/closure/`):
`cargo fmt --check` exit 0; `cargo test` exit 0 with **275 passed / 0 failed**;
`cargo test --release` exit 0 with **276 passed / 0 failed**; `cargo clippy
--all-features --all-targets` exit 0 with **zero warnings**. The retained test
was confirmed present and passing in both the debug and release logs. No
neutrality bench was run because the committed diff contains no engine inputs
(`git diff --numstat HEAD -- src/ Cargo.toml Cargo.lock build.rs` is empty);
the registered `baseline.exe`, built from this exact source, already asserted
the 7,601,220 / EBF 2.474 fingerprint.
