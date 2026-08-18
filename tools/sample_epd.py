"""Create an exact deterministic uniform reservoir sample of an EPD book.

The SPSA harness starts a fresh fastchess process for every point. Giving each
process the full multi-million-line book makes it repeatedly index far more
openings than a tune can consume. This one-time sampler keeps the source
distribution without taking the first N positions or loading the source book
into memory. Output replacement is atomic.
"""

from __future__ import annotations

import argparse
import os
from pathlib import Path
import random


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--input", required=True, type=Path)
    parser.add_argument("--output", required=True, type=Path)
    parser.add_argument("--count", required=True, type=int)
    parser.add_argument("--seed", default=20260802, type=int)
    args = parser.parse_args()

    if args.count <= 0:
        raise SystemExit("--count must be positive")
    if not args.input.is_file():
        raise SystemExit(f"Input EPD does not exist: {args.input}")

    rng = random.Random(args.seed)
    reservoir: list[bytes] = []
    seen = 0
    with args.input.open("rb") as source:
        for seen, line in enumerate(source, start=1):
            if seen <= args.count:
                reservoir.append(line)
                continue
            selected = rng.randrange(seen)
            if selected < args.count:
                reservoir[selected] = line

    if seen < args.count:
        raise SystemExit(
            f"Input contains only {seen} positions; requested {args.count}"
        )

    # Avoid preserving reservoir-slot order, which retains some source-order
    # structure even though membership is uniform.
    rng.shuffle(reservoir)
    args.output.parent.mkdir(parents=True, exist_ok=True)
    temporary = args.output.with_name(args.output.name + ".tmp")
    try:
        with temporary.open("wb") as destination:
            destination.writelines(reservoir)
        os.replace(temporary, args.output)
    finally:
        temporary.unlink(missing_ok=True)

    print(
        f"Sampled {args.count} of {seen} EPD positions "
        f"(seed {args.seed}) -> {args.output}"
    )


if __name__ == "__main__":
    main()
