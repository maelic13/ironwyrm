# Rarog/Stockfish HCE Stage 1 hybrid

This experiment isolates the question that matters: how much of Rarog's gap is
caused by search/engine infrastructure when its shipped HCE is held constant?

The executable uses the board, search, move ordering, pruning, transposition
table, time management and UCI implementation from Stockfish commit
`9587eeeb5ed29f834d4f956b92e0e732877c47a7`, the last pure-HCE master commit
immediately before Stockfish merged NNUE. Stockfish 11 was the last official
pure-HCE release, but this later revision is the more appropriate strongest-HCE
reference. Evaluation is supplied by the unchanged Rarog 2.3.2 HCE.

## Build and run

From the repository root on Windows:

```powershell
.\hybrid\build.ps1
```

The PGO/BMI2 package is written to:

```text
hybrid\dist\rarog-stockfish-hce-hybrid.exe
hybrid\dist\rarog_hce.dll
```

Both files must remain in the same directory. `-NoPgo` provides a quicker
developer build. The normal build requires Rust and an MSYS2 MinGW64 GCC/make
installation under `C:\msys64`.

## What crosses the boundary

Stockfish sends twelve piece bitboards, side to move, castling rights and the
rule-50 clock to a Rust DLL. The DLL rebuilds only Rarog's derived board state
and runs `Evaluator::evaluate`. En-passant is omitted because no production
Rarog HCE term reads it. Each Stockfish worker gets a thread-local Rarog
evaluator, preserving its pawn/evaluation caches without locks.

Rarog returns centipawns from the side-to-move point of view. The adapter maps
those to Stockfish's internal units using `PawnValueEg / 100` and clamps only at
the boundary reserved for proven wins. The adapter test confirms identical raw
scores between ordinary Rarog boards, reconstructed snapshots and the exported
C ABI.

## Required diagnostic comparison

Add the executable to Colosseum twice:

| Colosseum name | `Use Rarog HCE` | Meaning |
|---|---:|---|
| Rarog-SF-Hybrid | `true` (default) | Stockfish search plus Rarog 2.3.2 HCE |
| SF-9587eeeb-HCE-control | `false` | The original evaluator and search from the exact same Stockfish revision |

The control option changes only which evaluator `Eval::evaluate` calls. It is
not intended as a product option; it removes compiler, revision and UCI setup
as confounders in the experiment. `Contempt` defaults to zero and Analysis
Contempt defaults to Off because Rarog's HCE does not consume Stockfish's
thread-local contempt score.

Run both against Rarog 2.3.2 and Basilisk 1.9.3 with identical Threads, Hash,
opening positions, colors and time control. Use paired openings. First compare
hybrid versus Rarog: that directly measures the value of the Stockfish search
stack with Rarog's HCE. Then compare hybrid versus the exact Stockfish control:
that measures the remaining evaluator/co-adaptation gap.

## Source and licensing

`stockfish/` is an attributed snapshot of the commit above, including its
`AUTHORS` and `Copying.txt`. The adapter and Rarog are GPLv3-or-later. This
branch is an experimental diagnostic artifact, not a Rarog release.
