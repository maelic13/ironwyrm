# Rarog recurring procedures

Agent-facing process detail. Split out of `GUIDE.md` on 2026-08-21.
`AGENTS.md` holds the rules that stop wrong results; this holds the
step-by-step procedures those rules assume.

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

