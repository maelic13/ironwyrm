#!/usr/bin/env python3
"""Drawn-subset cohort: does the engine claim an advantage in dead-drawn endings?

`endgame_truth.py` measures what happens in positions that are theoretically
WON -- conversion, win-preservation, DTZ progress. That is the right instrument
for a mate drive, and the wrong one for a SCALE function, whose entire job is to
stop the evaluator from scoring a theoretically drawn position as winning.

The distinction matters concretely. RAR-E11 measured Stockfish 18 converting
KRP-KR at only 47.9% against Rarog's 43.8% at the same node budget, so the
conversion gap in that family is about four points, not the fifty-five that
"52% conversion" suggests against an imagined 100%. The defect a scale function
addresses is elsewhere: an engine that scores a drawn rook ending at +200 will
STEER INTO IT from a position it could have won differently, and no conversion
measurement taken inside the ending can see that happen.

So this measures the complementary cohort. Sample positions of a family, keep
the ones the tablebase calls a clean draw, search each at a fixed node budget,
and report how often the engine claims a meaningful advantage anyway.

  overclaim rate   fraction of drawn positions scored beyond `--threshold` cp
                   for the strong side. This is the number a scale function
                   must move, and the one to put in a ledger row.
  mean/median      the score distribution, which says whether a change moved
                   the whole cohort or clipped a tail.

Scores are from the STRONG side's perspective and come from a real search at
`--nodes`, not from a `tune`-feature static eval, so the number describes the
production engine. A mate score is clamped to `--mate-cp` rather than dropped:
claiming mate in a drawn position is the worst case, not a missing sample.

Positions are seeded from the family NAME, matching `endgame_truth.py`, so a
single-family run samples exactly what the same family samples inside a longer
run. Index seeding silently broke that comparison once already.

Usage:

  python tools/diag/endgame_drawn.py --engine target/release/rarog.exe \\
      --syzygy D:/chess/tablebases/syzygy3456 --families KRP-KR
"""

from __future__ import annotations

import argparse
import hashlib
import json
import random
import statistics
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))

try:
    import chess
    import chess.engine
    import chess.syzygy
except ImportError:
    print("ERROR: python-chess not installed. Run: pip install chess", file=sys.stderr)
    sys.exit(1)

from endgame_truth import parse_family, random_position, wdl_for_white  # noqa: E402


def main() -> int:
    ap = argparse.ArgumentParser(
        description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter
    )
    ap.add_argument("--engine", required=True, type=Path)
    ap.add_argument("--syzygy", required=True, type=Path)
    ap.add_argument("--families", required=True,
                    help="comma-separated, e.g. KRP-KR or KRP-KR,KRP-KB")
    ap.add_argument("--positions", type=int, default=400,
                    help="positions SAMPLED per family; the drawn subset is "
                         "whatever fraction of them the tablebase calls a draw")
    ap.add_argument("--nodes", type=int, default=60_000)
    ap.add_argument("--threshold", type=int, default=100,
                    help="cp beyond which a drawn position counts as overclaimed")
    ap.add_argument("--mate-cp", type=int, default=10_000,
                    help="score substituted for a mate claim")
    ap.add_argument("--seed", type=int, default=0x5E9D18,
                    help="matches endgame_truth.py, so cohorts line up")
    ap.add_argument("--output", type=Path)
    args = ap.parse_args()

    names = [n.strip() for n in args.families.split(",") if n.strip()]
    specs = {n: parse_family(n) for n in names}

    tb = chess.syzygy.open_tablebase(str(args.syzygy))
    engine = chess.engine.SimpleEngine.popen_uci(str(args.engine))
    report: dict[str, dict] = {}
    try:
        for name in names:
            strong, weak = specs[name]
            rng = random.Random(
                args.seed ^ int.from_bytes(
                    hashlib.sha256(name.encode()).digest()[:8], "big"
                )
            )
            scores: list[int] = []
            mates = 0
            sampled = 0
            for _ in range(args.positions):
                board = random_position(rng, strong, weak)
                sampled += 1
                try:
                    if wdl_for_white(tb, board) != 0:
                        continue
                except (chess.syzygy.MissingTableError, KeyError, ValueError) as exc:
                    ap.error(f"no tablebase for {name}: {exc}")
                info = engine.analyse(board, chess.engine.Limit(nodes=args.nodes))
                pov = info["score"].white()
                if pov.is_mate():
                    mates += 1
                    mate = pov.mate()
                    cp = args.mate_cp if mate is not None and mate > 0 else -args.mate_cp
                else:
                    cp = pov.score()
                scores.append(int(cp))

            if not scores:
                print(f"{name}: no drawn positions in {sampled} samples")
                continue
            over = [s for s in scores if s > args.threshold]
            entry = {
                "sampled": sampled,
                "drawn": len(scores),
                "overclaimed": len(over),
                "overclaim_rate": round(len(over) / len(scores), 6),
                "mate_claims": mates,
                "mean_cp": round(statistics.mean(scores), 2),
                "median_cp": statistics.median(scores),
                "max_cp": max(scores),
                "threshold_cp": args.threshold,
                "nodes": args.nodes,
            }
            report[name] = entry
            print(
                f"{name:<9} drawn {entry['drawn']:>4}/{sampled}  "
                f"overclaim {entry['overclaim_rate']:.4f} "
                f"({entry['overclaimed']}/{entry['drawn']} over "
                f"{args.threshold}cp)  mean {entry['mean_cp']:+.1f}  "
                f"median {entry['median_cp']:+}  max {entry['max_cp']:+}  "
                f"mate claims {mates}",
                flush=True,
            )
    finally:
        engine.quit()
        tb.close()

    if args.output:
        args.output.write_text(
            json.dumps(
                {
                    "schema": "rarog-endgame-drawn-v1",
                    # NOT one of the four layers: this measures the COMPLEMENT
                    # of conversion -- does the evaluator claim won what theory
                    # says is drawn. A SCALE function is validated here and is
                    # invisible in conversion; a VERDICT function is the other
                    # way round. Reading 4.9a.7 off conversion nearly called a
                    # working change a failure.
                    "layer": "drawn_share_bias",
                    "layer_note": (
                        "static evaluation of theoretically drawn positions; "
                        "plays no games, so it is unaffected by the RAR-E14 "
                        "playout defect. Never reports Elo."
                    ),
                    "engine": str(args.engine),
                    "seed": args.seed,
                    "families": report,
                },
                indent=2,
                sort_keys=True,
            )
            + "\n",
            encoding="utf-8",
            newline="\n",
        )
        print(f"\nReport: {args.output}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
