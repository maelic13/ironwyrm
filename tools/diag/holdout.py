#!/usr/bin/env python3
"""Development / held-out selection contract (PLAN 4.10.7).

Selecting on a set and then confirming on the same set is not confirmation. It
is the most expensive mistake available here, because it costs a whole step and
looks like success right up until someone checks: Basilisk selected on 60
positions across 3 rounds and 42 arms, watched a development estimate of 85.0%
shrink to 74.6% on a held-out 138, and had its selected candidate REJECTED
there (BAS-E39, BAS-E41).

Four things this file enforces, none of which survives as a habit:

1. **The split is made BEFORE any candidate runs, and which half decides is
   registered in advance.** `register()` writes that down and refuses to be
   rewritten once results exist.
2. **A runner-up is carried into confirmation.** The leader can be rejected on
   held-out data, and without a second arm the step ends with nothing.
   Basilisk's did not, only because it had one.
3. **A cohort that has produced a verdict is SPENT for selection.** It stays
   valid as a VETO, because a safety property is not an estimate: "this
   candidate discards a won position" does not get less true from reuse, while
   "this candidate converts 74%" does.
4. **A plateau is reported as a plateau.** `separation()` says whether the
   leader is distinguishable from its neighbours at all; Basilisk's "winner"
   never was (paired z +0.76 to +1.70).

The paired test is McNemar's, on the DISCORDANT positions only. Comparing two
aggregate rates over a shared cohort throws away the pairing and is the weaker
test; what carries information is the positions where the two arms disagree.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import math
import sys
from pathlib import Path

# A z from very few discordant positions is noise wearing a number. Below this
# the comparison is reported as indeterminate rather than as a value -- the
# same thin-sample discipline as `endgame_floors.MIN_ELIGIBLE`, for the same
# reason: the failure to prevent is a CONFIDENT reading, not a wrong one.
MIN_DISCORDANT = 6


def split_cohort(
    fens: list[str], holdout_fraction: float = 0.5, seed: int = 0x4E9A2
) -> tuple[list[int], list[int]]:
    """Partition position INDICES into (development, held-out).

    Assignment is a hash of the position itself, not of its index, so the split
    is stable when the cohort is extended or reordered and cannot be nudged by
    regenerating in a different order. Same reason the family seed derives from
    the family name (`endgame_truth.family_seed`).
    """
    if not 0.0 < holdout_fraction < 1.0:
        raise ValueError(f"holdout_fraction must be in (0,1), got {holdout_fraction}")
    dev, held = [], []
    for index, fen in enumerate(fens):
        digest = hashlib.sha256(f"{seed}:{fen}".encode("ascii")).digest()
        # 32 bits of the digest as a uniform draw in [0,1).
        draw = int.from_bytes(digest[:4], "big") / 2**32
        (held if draw < holdout_fraction else dev).append(index)
    return dev, held


def paired_counts(a: list[bool], b: list[bool]) -> tuple[int, int, int, int]:
    """(both, a_only, b_only, neither) over paired per-position outcomes."""
    if len(a) != len(b):
        raise ValueError(f"unpaired inputs: {len(a)} against {len(b)}")
    both = sum(1 for x, y in zip(a, b) if x and y)
    a_only = sum(1 for x, y in zip(a, b) if x and not y)
    b_only = sum(1 for x, y in zip(a, b) if y and not x)
    neither = sum(1 for x, y in zip(a, b) if not x and not y)
    return both, a_only, b_only, neither


def mcnemar(a: list[bool], b: list[bool]) -> dict:
    """Continuity-corrected McNemar z for two arms over the SAME positions.

    Positive z favours arm A. Only the discordant positions carry information,
    which is the whole point of pairing: 200 positions both arms convert say
    nothing about which is better.
    """
    both, a_only, b_only, neither = paired_counts(a, b)
    discordant = a_only + b_only
    out = {
        "both": both, "a_only": a_only, "b_only": b_only, "neither": neither,
        "discordant": discordant, "n": len(a),
    }
    if discordant < MIN_DISCORDANT:
        out["z"] = None
        out["indeterminate"] = (
            f"only {discordant} discordant positions (need {MIN_DISCORDANT}); "
            "reported as indeterminate rather than as a z"
        )
        return out
    magnitude = (abs(a_only - b_only) - 1) / math.sqrt(discordant)
    out["z"] = round(math.copysign(max(magnitude, 0.0), a_only - b_only), 4)
    return out


def separation(arms: dict[str, list[bool]], leader: str) -> dict:
    """Is the leader distinguishable from the others, or is this a plateau?

    Reported honestly either way. "Best of N" without separation is a plateau,
    and calling it a winner is how a step ends with a result it does not have.
    """
    out = {"leader": leader, "against": {}}
    separated = []
    for name, outcomes in arms.items():
        if name == leader:
            continue
        test = mcnemar(arms[leader], outcomes)
        out["against"][name] = test
        if test["z"] is not None and test["z"] >= 2.0:
            separated.append(name)
    others = [n for n in arms if n != leader]
    out["separated_from"] = separated
    out["is_plateau"] = bool(others) and len(separated) < len(others)
    return out


def register(path: Path, doc: dict) -> None:
    """Write the registration, once, before any candidate runs.

    Refuses to overwrite. Changing which half decides after seeing results is
    the same act as moving SPRT bounds, and is equally invisible afterwards.
    """
    required = {"cohort_sha256", "deciding_half", "arms", "runner_up", "policy"}
    missing = required - set(doc)
    if missing:
        raise ValueError(f"registration missing {sorted(missing)}")
    if doc["deciding_half"] not in ("development", "holdout"):
        raise ValueError("deciding_half must be 'development' or 'holdout'")
    if doc["runner_up"] is None:
        raise ValueError(
            "a runner-up must be nominated before results exist; the leader "
            "can be rejected on held-out data and without a second arm the "
            "step ends with nothing (BAS-E41)"
        )
    if doc["runner_up"] not in doc["arms"]:
        raise ValueError("the runner-up must be one of the registered arms")
    if path.exists():
        raise SystemExit(
            f"{path} already exists; a registration is written once, before "
            "any candidate runs. Rewriting it after seeing results is the same "
            "act as moving SPRT bounds."
        )
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(
        {"schema": "rarog-holdout-registration-v1", **doc}, indent=2,
        sort_keys=True) + "\n", encoding="utf-8", newline="\n")


def load_spent(path: Path) -> dict:
    if not path.is_file():
        return {"schema": "rarog-spent-cohorts-v1", "spent": {}}
    return json.loads(path.read_text(encoding="utf-8"))


def check_not_spent(ledger: dict, cohort: str, purpose: str) -> None:
    """A cohort that produced a verdict may VETO, but may not SELECT again.

    The asymmetry is not fussiness. "This candidate discards a won position" is
    a safety property and does not get less true from reuse; "this candidate
    converts 74%" is an estimate and does, because the candidate was chosen
    partly on this data's noise.
    """
    if purpose == "veto":
        return
    if purpose != "selection":
        raise ValueError(f"purpose must be 'selection' or 'veto', got {purpose!r}")
    record = ledger.get("spent", {}).get(cohort)
    if record:
        raise SystemExit(
            f"cohort {cohort[:16]} is SPENT for selection: it produced a "
            f"verdict on {record.get('date', '?')} for {record.get('step', '?')}. "
            "It remains valid as a veto. Select new candidates on fresh "
            "positions."
        )


def spend(path: Path, cohort: str, step: str, date: str) -> None:
    ledger = load_spent(path)
    ledger.setdefault("spent", {})[cohort] = {"step": step, "date": date}
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(ledger, indent=2, sort_keys=True) + "\n",
                    encoding="utf-8", newline="\n")


def main() -> int:
    ap = argparse.ArgumentParser(
        description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter
    )
    sub = ap.add_subparsers(dest="command", required=True)

    s = sub.add_parser("split", help="show the dev/held-out split of a report")
    s.add_argument("--report", required=True, type=Path)
    s.add_argument("--family", required=True)
    s.add_argument("--holdout-fraction", type=float, default=0.5)
    s.add_argument("--seed", type=int, default=0x4E9A2)

    c = sub.add_parser("compare", help="paired McNemar between two reports")
    c.add_argument("--a", required=True, type=Path)
    c.add_argument("--b", required=True, type=Path)
    c.add_argument("--family", required=True)
    c.add_argument("--half", choices=("development", "holdout", "all"),
                   default="holdout")
    c.add_argument("--holdout-fraction", type=float, default=0.5)
    c.add_argument("--seed", type=int, default=0x4E9A2)

    args = ap.parse_args()

    def load(path: Path, family: str):
        doc = json.loads(path.read_text(encoding="utf-8"))
        if doc.get("schema") != "rarog-endgame-truth-v2":
            raise SystemExit(f"{path}: not a rarog-endgame-truth-v2 report")
        entry = doc["families"][family]
        if "positions" not in entry:
            raise SystemExit(
                f"{path}: {family} has no per-position records; re-run with "
                "--per-position, since a paired test needs the pairing"
            )
        return doc, entry

    if args.command == "split":
        _, entry = load(args.report, args.family)
        fens = [r["fen"] for r in entry["positions"]]
        dev, held = split_cohort(fens, args.holdout_fraction, args.seed)
        print(f"{args.family}: {len(dev)} development, {len(held)} held out "
              f"of {len(fens)}")
        return 0

    doc_a, ea = load(args.a, args.family)
    doc_b, eb = load(args.b, args.family)
    if ea["cohort_sha256"] != eb["cohort_sha256"]:
        raise SystemExit(
            "the two reports measured different position sets; a paired test "
            "over unpaired data is not a test (RAR-E14 defect B)"
        )
    fens = [r["fen"] for r in ea["positions"]]
    dev, held = split_cohort(fens, args.holdout_fraction, args.seed)
    keep = {"development": dev, "holdout": held,
            "all": list(range(len(fens)))}[args.half]
    conv_a = [ea["positions"][i]["outcome"] == "mated" for i in keep]
    conv_b = [eb["positions"][i]["outcome"] == "mated" for i in keep]
    result = mcnemar(conv_a, conv_b)
    print(f"family {args.family}, half {args.half}, n={result['n']}")
    print(f"  both {result['both']}  a-only {result['a_only']}  "
          f"b-only {result['b_only']}  neither {result['neither']}")
    if result["z"] is None:
        print(f"  INDETERMINATE: {result['indeterminate']}")
    else:
        print(f"  McNemar z = {result['z']:+.4f} (positive favours A)")
    return 0


if __name__ == "__main__":
    sys.exit(main())
