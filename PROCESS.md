# Rarog recurring procedures

Agent-facing process detail. Split out of `GUIDE.md` on 2026-08-21.
`AGENTS.md` holds the rules that stop wrong results; this holds the
step-by-step procedures those rules assume.

## Recurring procedures

### Phase-4 step lifecycle (4.5–4.14)

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
clusters, **4.8/4.9** own search consolidation and **4.13** owns HCE
consolidation, with separate confirmation SPRTs; they may not rescue an
earlier losing cluster.

Two failed coherent search clusters trigger a return to 4.2–4.3. Two failed
HCE clusters trigger a **4.11** decomposition re-audit, not silent closure. Track H may
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
match host. Roughly 2 Elo per 1% NPS at `3+0.03` — **for SMALL deltas only.**
That figure does not extrapolate: applied to the oracle's 1.80x NPS deficit it
predicts 160 Elo, where the standard ~60 Elo per doubling gives ~51. Above a
few percent, convert through doublings and say which conversion was used.

### Matched ablation (the Phase-4 measurement instrument)

How the deficit was decomposed, and the procedure for every later use.
`analysis/ablation_design.md` holds the reasoning; this is the operation.

One shared bitmask on both engines — 0 razoring, 1 futility-child, 2 nullmove,
3 probcut, 4 iir, 5 shallow-pruning, 6 extensions, 7 lmr — so the same number
ablates the same mechanism on each side. Oracle: branch `hybrid-ablate`.
Rarog: `--features ablate`, which compiles every guard away in a shipped build.

0. **The harness now refuses to start when an engine does not expose an option
   being set.** fastchess only WARNS and then plays the whole match at the
   DEFAULT, which is a completed run that measures nothing. That happened twice
   here — once from a malformed option name, once from a binary that predated
   the switch. If `sprt.ps1` aborts with "does not expose", rebuild the arm;
   never work around it by dropping the option.
1. **Prove every bit live before trusting any number from it.** Nodes to a
   fixed depth must MOVE for each bit. A guard that reads x1.00 is dead — one
   did, because its anchor landed on the diagnostic `prune_shadow_*` block
   instead of the live site.
2. **Prefer matched CROSS-ENGINE runs to self-play deltas.** Play Rarog against
   the oracle at the SAME mask and read `G(0) − G(mask)` as the Elo that
   mechanism explains. One run per mechanism, one scale, no self-play
   inflation.
3. **Keep the ablated arm inside roughly 20–80%.** Outside it the Elo curve
   saturates: at 6% it runs 30.8 Elo per score point against 6.9 near parity,
   a 4.4x amplification, and a 3-point score difference reads as 105 Elo of
   nothing. Ablating four mechanisms at once collapsed both arms and produced
   exactly that.
4. **~2,000 games is enough.** These are 40–250 Elo effects; stop as soon as
   the intervals separate. Large effects are cheap, which is the whole reason
   this instrument beats gating candidates one at a time.
5. **Mechanisms under ~10 Elo are NOT measurable this way at `3+0.03`.**
   Roughly 10 time forfeits per 3,000 games is worth ~1 Elo, so for razoring
   and IID the noise equals the signal. Use fixed nodes or a longer TC.
6. **Net out NPS before comparing engines.** Rarog runs 1.80x the oracle's
   speed, worth ~51 Elo. It cancels in `G(0) − G(mask)` but not in any absolute
   statement about which search is better.

### Texel convergence procedure

Texel is cheap enough to run locally after structural HCE work, but its static
loss is not a strength verdict:

1. Trace every changed term exactly and verify full evaluation reconstruction.
2. Report activation, covariance and identifiability before selecting weights.
3. Keep fixed by-game train/validation/untouched splits. Never tune on the
   untouched test set.
4. Run a local family fit after each structural HCE cluster. Bake the fit into
   clean PGO and SPRT the cluster; a lower loss alone accepts nothing.
5. At 4.13, run an anchored whole-HCE consolidation over only activated,
   identifiable weights. Repeat a cycle only if validation and the baked SPRT
   both improve. Stop at the first no-gain/failed cycle; never choose a lucky
   intermediate checkpoint retrospectively.
6. Keep search parameters frozen during HCE Texel cycles.

### SPSA go/no-go procedure

The generic harness is retained. **Phase 4 budgets exactly one search SPSA:
the seeded selectivity fit at 4.8**, entered only after 4.5 and 4.6 have landed
their contracts and both `G(128)` and `G(32)` have measurably shrunk. Seed its
constants from the reference rather than from Rarog's current values — those
are the values that produced the measured 188 and 200 Elo, so it becomes a
local refit rather than a search. On the HCE side, 4.12's king-danger bucket
selectors and 4.13's HCE-induced search-margin compatibility are the only other
admissible surfaces, each separate and narrow. An undirected broad tune stays
forbidden, and HCE and search coordinates are never mixed in one run.
Before any SPSA:

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
- Mirror any status or number change into **both `GUIDE.md` and `PLAN.md` in
  the same commit**. `TRACKER.md` is history and is not updated for new work.

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
# Primary SPRT [0,3] nElo — the DEFAULT bracket. Add -TC "10+0.1" for LTC.
# [3,10] is the harness default and is WRONG for a small candidate: wide bounds
# anchored high drive a true +4 to H0. Size from RAR-M10 before registering.
./tools/sprt.ps1 -EngineA <candidate.exe> -EngineB <baseline.exe> `
  -NameA candidate -NameB baseline -Elo0 0 -Elo1 3 -MaxGames 80000

# Harness calibration after any runner change — same binary on both sides
./tools/sprt.ps1 -EngineA <same.exe> -EngineB <same.exe> -NameA a -NameB b

# Test/tune binaries and the SPSA coverage audit
./tools/build_test.ps1 -Suffix <s>
./tools/audit_spsa_coverage.ps1
```

