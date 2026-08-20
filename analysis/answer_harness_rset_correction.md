# Correction: every `--rset` screen before this commit measured the defaults

## What happened

`--rset` was added to `tools/diag/answer_compare.py` by a scripted edit that
anchored on the line

    if not line or line.startswith("uciok"):

and inserted the `setoption` block after it. That string occurs **twice** in
the file: once in `run_engine`, and once earlier in the **module docstring**,
which documents the streaming-stdin contract. The first match won, so the
block was inserted into the docstring as prose. It never executed.

`ast.parse` passed, because the inserted lines were inside a string literal.
`--rset` therefore ran without error and silently measured default parameters
on every invocation.

## What it invalidated

Two screens, both of which had been recorded as null results:

- **4.6.7 `LmrRootRelief`.** Reported "settle depth, revisions and volatility
  identical at 0, 512 and 1024" and was committed default-off as a null.
- **4.8.1 `LmrMinReducedDepth`.** Reported bit-identical answers against an
  18.4% node change, which is what finally exposed the bug: no parameter
  moves the tree that much and leaves fifty answers untouched.

The engine was never at fault in either case. A standalone probe HAD confirmed
`LmrRootRelief` changes the search (169,667 nodes against 154,319 at one
position) -- but that probe drove the engine directly, not through the harness,
so it verified the option and not the instrument that was reporting on it.

## The re-measurement

Depth 12, 50 positions, through a wire proved live first by checking that a
deliberately absurd value moves the numbers.

| relief | agreement | cohorts | revisions | volatility | bench 13 nodes |
|---:|---:|---:|---:|---:|---:|
| 0 | 66% | 33/50 | 1.50 | 421.9 | 7,467,143 |
| 512 | 64% | 32/50 | 1.56 | 369.9 | -- |
| 1024 | 72% | 36/50 | 1.64 | 269.1 | 6,848,629 |
| **1536** | **78%** | **39/50** | **1.70** | 269.7 | 6,977,070 |
| 2048 | 76% | 38/50 | 1.64 | 269.9 | 7,296,339 |

Oracle reference: revisions 2.16, volatility 198.8. Mean root reduction falls
2.90 -> 1.95 -> 1.55 -> 1.16 plies across the sweep.

So 4.6.7 is the opposite of the null it was recorded as: the relief moves
agreement, revisions, volatility and median |dcp| (38 -> 28) all toward the
oracle, improves every cohort at 1536, and costs no nodes.

**The two cluster members do not compose.** Relief 1536 alone scores 78% and
39/50 cohorts; relief 1024 + floor 1 together score 70% and 34/50, below
relief 1024 alone (72%, 36/50). This is the co-adaptation the cluster rule
exists for, showing up between two members of the same cluster.

## The rule this cost

`AGENTS.md` already says every scripted edit must assert its anchor matched.
It did assert -- `next()` would have raised. The assertion that was missing is
that the anchor was **unique**, and that the edit landed in **executable
code**. Both screens then reported a plausible null and nothing contradicted
them until a change large enough to be impossible came along.

A default-off parameter screened through a harness needs the harness proved
live in the same run: set an absurd value first and require the numbers to
move. That check is now cheap and it is the only thing that would have caught
this on the first screen rather than the second.

## Standing caveat, unchanged

Agreement with the oracle is a **proxy**, not the objective. n = 50, the
sweep is non-monotone at 512, and none of these numbers is a gate. The
cluster still owes a registered `[0,3]` SPRT.
