#!/usr/bin/env python3
"""Parallel front end for extract.py with the identical dataset contract."""

from __future__ import annotations

import io
import json
import os
import random
import sys
from collections import Counter
from multiprocessing import Pool
from pathlib import Path

import chess.pgn

import extract as seq


def next_game_offset(path: Path, nominal: int, size: int) -> int:
    if nominal <= 0:
        return 0
    if nominal >= size:
        return size
    with path.open("rb") as stream:
        stream.seek(max(0, nominal - 1))
        carry = b""
        base = stream.tell()
        while True:
            chunk = stream.read(1 << 16)
            if not chunk:
                return size
            data = carry + chunk
            found = data.find(b"\n[Event ")
            if found >= 0:
                return base - len(carry) + found + 1
            carry = data[-8:]
            base += len(chunk)


def worker(task):
    path_text, start, end, opts = task
    path = Path(path_text)
    with path.open("rb") as stream:
        stream.seek(start)
        text = stream.read(end - start).decode("utf-8", errors="replace")
    pgn = io.StringIO(text)
    output = []
    stats = Counter()
    while True:
        try:
            game = chess.pgn.read_game(pgn)
        except Exception:
            stats["parse_errors"] += 1
            continue
        if game is None:
            break
        stats["recorded_games"] += 1
        opening = seq.start_key(game)
        split = seq.split_for(game, opts["validation_pct"], opts["test_pct"])
        rows, rejected = seq.process_game(
            game, opts["skip_start"], opts["skip_end"],
            opts["max_per_phase_per_game"], opts["max_per_game"],
            opts["quiet_filter"], random.Random(opts["seed"] ^ seq.start_digest(game)),
        )
        stats["quiet_rejected"] += rejected
        if not rows:
            stats["skipped"] += 1
            output.append((opening, split, []))
            continue
        stats["raw"] += len(rows)
        result = seq.RESULT_MAP[game.headers["Result"]]
        labelled = []
        for fen, bucket, cp in rows:
            target = result
            if split == "train" and opts["blend"] < 1.0:
                if cp is None:
                    stats["missing_evals"] += 1
                else:
                    target = opts["blend"] * result + (1.0 - opts["blend"]) * seq.sigmoid_cp(cp)
            labelled.append((fen, target, bucket, cp))
        output.append((opening, split, labelled))
    return output, dict(stats)


def main() -> int:
    command = seq.parser()
    command.description = __doc__
    command.add_argument("--jobs", type=int, default=0)
    command.add_argument("--audit-only", action="store_true")
    args = command.parse_args()
    seq.validate_args(args, command)
    if args.preflight_games:
        command.error("use extract.py for the bounded pilot preflight")
    paths = seq.iter_pgn_paths(args.pgn)
    jobs = args.jobs or max(1, (os.cpu_count() or 2) - 2)
    if jobs < 1:
        command.error("--jobs must be positive")
    counts = seq.split_counts(args.target_train, args.validation_pct, args.test_pct)
    quotas, reservoirs = seq.make_reservoirs(counts, args.phase_weights, args.seed)
    opts = {name: getattr(args, name) for name in (
        "validation_pct", "test_pct", "skip_start", "skip_end",
        "max_per_phase_per_game", "max_per_game", "quiet_filter", "seed", "blend")}
    tasks = []
    for path in paths:
        size = path.stat().st_size
        parts = min(jobs, max(1, size // (16 << 20)))
        nominal = [size * index // parts for index in range(parts + 1)]
        offsets = [next_game_offset(path, value, size) for value in nominal]
        offsets[0], offsets[-1] = 0, size
        tasks.extend((str(path), offsets[index], offsets[index + 1], opts)
                     for index in range(parts) if offsets[index + 1] > offsets[index])
    if not tasks:
        raise SystemExit("no non-empty PGN ranges")

    print(f"Parallel extraction: {len(paths)} PGN(s), {len(tasks)} ranges, {jobs} workers")
    seen_positions: set[str] = set()
    seen_starts: set[str] = set()
    totals = Counter()
    unique = {split: [0] * len(seq.PHASE_BUCKETS) for split in seq.SPLITS}
    with Pool(min(jobs, len(tasks))) as pool:
        for task_output, stats in pool.imap(worker, tasks):
            totals.update(stats)
            for opening, split, rows in task_output:
                if opening in seen_starts:
                    continue
                seen_starts.add(opening)
                totals["games"] += 1
                totals[f"games_{split}"] += 1
                for fen, target, bucket, cp in rows:
                    key = seq.fen_key(fen)
                    if key in seen_positions:
                        continue
                    seen_positions.add(key)
                    unique[split][bucket] += 1
                    reservoirs[split][bucket].offer((fen, target, cp))

    print(f"Independent starts={totals['games']:,} recorded_games={totals['recorded_games']:,} "
          f"paired_replays={totals['recorded_games']-totals['games']:,} skipped={totals['skipped']:,} "
          f"parse_errors={totals['parse_errors']:,} raw={totals['raw']:,} "
          f"unique={len(seen_positions):,} quiet_rejected={totals['quiet_rejected']:,}")
    if args.blend < 1.0:
        print(f"Missing training evals={totals['missing_evals']:,} (pure-WDL fallback)")
    short = []
    for split in seq.SPLITS:
        for phase, name in enumerate(seq.BUCKET_NAMES):
            have = len(reservoirs[split][phase].items)
            want = quotas[split][phase]
            print(f"  {split:10}/{name:13}: {have:,}/{want:,} eligible={reservoirs[split][phase].seen:,}")
            if have < want:
                short.append((split, name))
    if short:
        print("ERROR: exact quotas not met; existing outputs unchanged.", file=sys.stderr)
        return 2
    if args.audit_only:
        print("Audit complete; no CSVs written.")
        return 0

    out_dir = Path(args.out_dir).resolve() if args.out_dir else paths[0].parent
    names = {"train": args.train, "validation": args.validation, "test": args.test}
    targets = {split: out_dir / names[split] for split in seq.SPLITS}
    manifest_path = out_dir / "manifest.json"
    for target in (*targets.values(), manifest_path):
        if target.exists():
            raise FileExistsError(f"refusing to overwrite frozen dataset artifact: {target}")
    out_dir.mkdir(parents=True, exist_ok=True)
    staged = []
    hashes = {}
    shuffle = random.Random(args.seed)
    for split in seq.SPLITS:
        rows = [(fen, target) for phase in reservoirs[split]
                for fen, target, _cp in phase.items]
        shuffle.shuffle(rows)
        temporary = seq.stage_rows(targets[split], rows)
        staged.append((temporary, targets[split]))
        hashes[split] = seq.sha256_file(temporary)
    manifest = {
        "schema": "rarog-hce-wdl-v2",
        "inputs": [{"path": str(path), "bytes": path.stat().st_size,
                    "sha256": seq.sha256_file(path)} for path in paths],
        "seed": args.seed,
        "independent_starts": totals["games"],
        "recorded_games": totals["recorded_games"],
        "paired_replays_discarded": totals["recorded_games"] - totals["games"],
        "skipped_games": totals["skipped"],
        "parse_errors": totals["parse_errors"],
        "games_by_split": {split: totals[f"games_{split}"] for split in seq.SPLITS},
        "rows": counts,
        "phase_quotas": {split: dict(zip(seq.BUCKET_NAMES, quotas[split]))
                         for split in seq.SPLITS},
        "output_sha256": hashes,
        "dedup_fields": 5,
        "filters": {"quiet": args.quiet_filter, "skip_start": args.skip_start,
                    "skip_end": args.skip_end,
                    "max_per_phase_per_game": args.max_per_phase_per_game,
                    "max_per_game": args.max_per_game},
        "label": "white-perspective self-play WDL",
        "train_blend": args.blend,
        "parallel_ranges": len(tasks),
    }
    manifest_tmp = manifest_path.with_name(manifest_path.name + ".tmp")
    manifest_tmp.write_text(json.dumps(manifest, indent=2) + "\n", encoding="utf-8", newline="\n")
    for temporary, target in staged:
        os.replace(temporary, target)
    os.replace(manifest_tmp, manifest_path)
    print(f"Published {sum(counts.values()):,} rows under {out_dir}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
