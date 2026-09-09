# Time forfeits at `3+0.03` — diagnosis and repair (PLAN A.3.3, RAR-R11)

## The defect

Rarog forfeits on time about once per thousand games at `3+0.03`, 1T, fourteen
concurrent games, on every recent SPRT and on both arms: RAR-E16's STC run
(game 114 of 743, the candidate), RAR-E15 (1 of 1,951), RAR-E09's runs (4 of
7,389 and 1 of 1,501). RAR-M14 had measured the floor at 0.08–0.17% and found
that games end having spent 97–99% of their clock by the engine's own
accounting.

## The seven forfeited games, reconstructed

For each forfeited game the loser's clock was rebuilt from the PGN move
comments (engine-reported time per move, base 3 s, increment 30 ms), and the
engine's own hard ceiling for the stalled move computed from
`time_manager.rs`: `min(0.8097 × time − 10, time − 20)` ms.

| Game | Remaining before the stalled move | Engine maximum | Engine-reported last time | Reading |
|---|---:|---:|---:|---|
| RAR-E16 g130 | 181 ms | 137 ms | 21 ms | inside budget; silent for > 180 ms |
| RAR-E15 g145 | 225 | 172 | 253 | reported time past the ceiling by 80 ms |
| RAR-E09 g25 | 91 | 64 | 163 | past by 99 |
| RAR-E09 g367 | 148 | 110 | 183 | past by 73 |
| RAR-E09 g4555 | 55 | 35 | 85 | past by 50 |
| RAR-E09 g5578 | 212 | 162 | 323 | past by 161 |
| RAR-E09b g117 | 527 | 417 | 43 | inside budget; silent for > 500 ms |

The reported time is stamped when an `info` line is printed at the end of a
completed iteration. A completed iteration stamped 80–160 ms past the hard
ceiling means the search ran past the ceiling without its every-2048-node
clock check firing, which cannot happen while the search is executing. It can
happen if the process was **stalled**: descheduled, or blocked inside a
`println!` whose pipe the harness had not drained. After a stall the next
tiny TT-driven iteration can complete inside one check window and stamp the
line late. The two "silent" cases are the same stall with nothing left to
print. The seven are one population.

## Reproduction

`tools/results/time-forfeit-20260909/tm_probe.py` and `tm_load.py` drive the
fixed-feature PEXT build at bullet clocks over UHO positions and compare wall
time (`go` to `bestmove`) with the engine's reported time and its own maximum.

| Condition | Searches | Worst wall overrun past the maximum | Moves past clock + 20 ms |
|---|---:|---:|---:|
| Idle host, 1 process, clocks 60–400 ms | 1,000 | 1.2 ms | 0 |
| Idle host, 14 processes in parallel | 5,600 | 2.0 ms | 0 |
| 4 processes, reader lagging 20 ms per line, **before the fix** | 400 | **194 ms**; every move overran its clock | **400** |
| 4 processes, reader lagging 20 ms per line, **after the fix** | 400 | 42 ms (one reader lag on the final line plus `bestmove`) | **0** |

CPU contention alone does not reproduce the forfeit on this host. A lagging
reader reproduces it exactly: the engine wrote about thirteen `info` lines per
bullet move, each `println!` blocks when the pipe is full, and the search
cannot see time spent blocked. Under fourteen concurrent games the single
fastchess process drains twenty-eight engine pipes; when it lags by tens of
milliseconds, that lag becomes engine wall time.

## The repair (`d93f808`)

`search_root` now prints depth 1, then at most one `info` line per 250 ms
(`INFO_THROTTLE_MS`), and always the last completed iteration before
`bestmove`, from a PV snapshot taken when the iteration completed so the final
line describes the iteration `bestmove` came from even if a later one was
aborted. At bullet this is two lines per move instead of thirteen; at longer
controls GUIs still see progress every quarter second. `send_info` is folded
into `send_info_line`. A test (`short_search_throttles_info_lines_and_reports_the_final_depth`)
asserts depth 1 first, at most three lines for a 100 ms search, and strictly
increasing depths.

Not changed, and why: the low-time reserve (`2 × MoveOverhead` at 1T, plus
30 ms above one thread). The reconstructed stalls are 50–500 ms; a 30–40 ms
reserve would have rescued at most one of seven while costing thinking time on
every last move. It stays a D.1 question with the rest of time management.
The harness-side `Move Overhead` sweep from RAR-M14 stays available if RAR-R11
shows a residual rate.

## Verification

`cargo fmt --check`, `cargo clippy --all-features --all-targets -D warnings`,
`cargo test` debug and release (567 tests) all clean on the final text. Fresh
no-feature builds: PEXT native and magic both **7,601,220 / EBF 2.474**; the
change is bench-invisible by construction (output only). The registered
RAR-R11 run decides the forfeit rate and bounds the Elo effect.

## Evidence

`tools/results/time-forfeit-20260909/` (ignored): the two probe scripts,
`tm_probe.log` (idle), `tm_load_prompt.log` (14 processes), `tm_load_lag.log`
(before), `tm_load_lag_fixed.log` (after), `verify_a33b.log`, and the bench
outputs of both builds. The seven forfeited games are in the SPRT PGNs named
above under `tools/results/`.
