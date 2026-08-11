# Rarog 2.3.2

Rarog 2.3.2 is a consolidation release. It carries forward the search and
evaluation improvements accepted since 2.3.1, fixes a mate-score correctness
edge case, speeds up ARM64 builds, and strengthens release-binary validation.

## Highlights

- Accepted broad selectivity, zero-reduction LMR and anchored evaluation
  updates are included. Their individual confirmation results were
  +15.33 ± 7.34 nElo, +9.13 ± 5.45 nElo and +11.56 ± 5.19 Elo. These results
  used different sequential tests and should not be added into one Elo claim.
- Null-move pruning can no longer turn an unproven mate-range score into an
  authoritative cutoff.
- ARM64 TT prefetching improved median `bench 13` speed by 1.42% on an Apple M4
  in a 12-round paired test, with unchanged search behavior.
- Every release asset is checked for its documented instruction-set contract,
  UCI startup and deterministic benchmark before upload.
- The portable x86-64 build no longer contains accidental POPCNT instructions
  from tablebase code and retains its documented SSE3 compatibility baseline.

The deterministic one-thread `bench 13` fingerprint is **6,519,711 nodes**
with geomean EBF **2.449**.

## Downloads

- `pext`: modern Intel and AMD Zen 3+; usually fastest.
- `avx2`: the same AVX2/BMI2/FMA requirement, often preferable on Zen 1/2.
- `x86-64`: portable SSE3 build for older x86-64 CPUs.
- `arm64`: Linux ARM64, Windows on ARM and Apple Silicon.

All published builds are profile-guided optimized. Add the executable to any
UCI-compatible chess GUI; no installation is required.
