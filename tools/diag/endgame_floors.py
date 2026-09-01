#!/usr/bin/env python3
"""Aggregate endgame floors: the ratchet half of the 4.9a.3 contract.

The hard vetoes live in `tests/endgames.rs` and are absolute -- a won position
may not score as drawn, a drawn position may not be claimed as forced mate.
This file owns the other half, which is deliberately NOT absolute: family
conversion rates, win-preserving rates and DTZ progress are statistics, and the
audit's policy is explicit that they are aggregate floors rather than
per-position vetoes. A candidate is allowed to move an individual position; it
is not allowed to move the family average down.

Two rules the audit set, implemented here:

  Floors are compared with a NOISE ALLOWANCE. At 100 positions per family the
  binomial standard error on a conversion rate is about 3.5 points, so a bare
  "must not decrease" test would fail on resampling alone. `--tolerance`
  defaults to 5 points, roughly 1.4 standard errors.

  Floors RATCHET. `--update` rewrites the floors from a run that passed, so an
  accepted improvement raises the bar. It refuses to lower any floor unless
  `--allow-lower` is passed, because "never relax a correctness test in the
  implementation commit" is the rule that this switch exists to make awkward.

Usage:

  # check a candidate against the committed floors
  python tools/diag/endgame_floors.py --report <endgame-truth.json>

  # ratchet the floors up after an accepted improvement (own commit)
  python tools/diag/endgame_floors.py --report <endgame-truth.json> --update
"""

from __future__ import annotations

import argparse
import json
import sys
from pathlib import Path

DEFAULT_FLOORS = Path(__file__).with_name("endgame_floors.json")

# Metrics compared per family. Each is a rate in [0,1] where higher is better.
METRICS = ("conversion_rate", "win_preserving_rate", "dtz_progress_rate")


def load_report(path: Path) -> dict:
    data = json.loads(path.read_text(encoding="utf-8"))
    if data.get("schema") != "rarog-endgame-truth-v1":
        raise SystemExit(f"{path}: not a rarog-endgame-truth-v1 report")
    return data


def rates(report: dict) -> dict[str, dict[str, float]]:
    out: dict[str, dict[str, float]] = {}
    for name, entry in report["families"].items():
        vals = {}
        for metric in METRICS:
            value = entry.get(metric)
            if value is not None:
                vals[metric] = float(value)
        if vals:
            out[name] = vals
    return out


def main() -> int:
    ap = argparse.ArgumentParser(
        description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter
    )
    ap.add_argument("--report", required=True, type=Path)
    ap.add_argument("--floors", type=Path, default=DEFAULT_FLOORS)
    ap.add_argument("--tolerance", type=float, default=0.05,
                    help="noise allowance in rate units (default 0.05 = 5 points)")
    ap.add_argument("--update", action="store_true",
                    help="rewrite the floors from this report (ratchet up)")
    ap.add_argument("--allow-lower", action="store_true",
                    help="permit --update to LOWER a floor; requires its own "
                         "commit and a recorded reason")
    args = ap.parse_args()

    report = load_report(args.report)
    current = rates(report)

    if args.update and not args.floors.is_file():
        args.floors.write_text(
            json.dumps({"schema": "rarog-endgame-floors-v1", "families": current},
                       indent=2, sort_keys=True) + "\n",
            encoding="utf-8", newline="\n")
        print(f"created {args.floors} from {args.report}")
        return 0

    if not args.floors.is_file():
        raise SystemExit(f"no floors file at {args.floors}; create it with --update")

    floors = json.loads(args.floors.read_text(encoding="utf-8"))["families"]

    failures, missing, improved = [], [], []
    for family, want in sorted(floors.items()):
        if family not in current:
            missing.append(family)
            continue
        for metric, floor in want.items():
            got = current[family].get(metric)
            if got is None:
                missing.append(f"{family}.{metric}")
                continue
            if got < floor - args.tolerance:
                failures.append((family, metric, floor, got))
            elif got > floor + args.tolerance:
                improved.append((family, metric, floor, got))

    print(f"floors   : {args.floors}")
    print(f"report   : {args.report}")
    print(f"tolerance: {args.tolerance:.3f}")
    print()
    if improved:
        print("Improved beyond tolerance (candidates to ratchet with --update):")
        for f, m, fl, got in improved:
            print(f"  {f:<10} {m:<20} {fl:.4f} -> {got:.4f}  (+{got - fl:.4f})")
        print()
    if missing:
        print("MISSING from the report (a floor with no measurement is not a pass):")
        for item in missing:
            print(f"  {item}")
        print()
    if failures:
        print("BELOW FLOOR:")
        for f, m, fl, got in failures:
            print(f"  {f:<10} {m:<20} floor {fl:.4f}, got {got:.4f}  ({got - fl:+.4f})")

    if args.update:
        if failures and not args.allow_lower:
            print("\nREFUSED: this report is below a floor, so --update would "
                  "LOWER it. Pass --allow-lower only with a recorded reason, in "
                  "its own commit.")
            return 1
        merged = {}
        for family, vals in current.items():
            prev = floors.get(family, {})
            merged[family] = {
                m: (v if args.allow_lower else max(v, prev.get(m, v)))
                for m, v in vals.items()
            }
        args.floors.write_text(
            json.dumps({"schema": "rarog-endgame-floors-v1", "families": merged},
                       indent=2, sort_keys=True) + "\n",
            encoding="utf-8", newline="\n")
        print(f"\nfloors updated from {args.report}")
        return 0

    if failures or missing:
        print(f"\nFAIL: {len(failures)} below floor, {len(missing)} missing")
        return 1
    print("PASS: no family below its floor")
    return 0


if __name__ == "__main__":
    sys.exit(main())
