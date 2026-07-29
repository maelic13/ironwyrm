#!/usr/bin/env python3
"""
extract.py  —  PGN → FEN;target dataset for Rarog Texel tuning (Phase 3.4 + 6.2.0)

Usage:
    python tools/texel/extract.py <selfplay.pgn> [options]

Options:
    --out-dir DIR       Output directory (default: same directory as PGN)
    --train  FILENAME   Training set filename  (default: train.csv)
    --holdout FILENAME  Holdout set filename   (default: holdout.csv)
    --holdout-pct N     Percent of games → holdout (default: 5)
    --max-per-game N    Max qualifying plies sampled per game (default: 12)
    --skip-start N      Plies to skip at game start  (default: 16, = 8 full moves)
    --skip-end   N      Plies to skip at game end    (default: 6)
    --seed N            Random seed (default: 42)
    --min-train N       Warn if fewer than N training positions (default: 1500000)
    --balance-phase R   If >0, downsample over-represented phase buckets in TRAIN so
                        none exceeds R x the smallest (e.g. 2.0). Lossy; off by default.
                        Always prints the train/holdout phase mix regardless.
    --no-quiet-filter   Disable the true-quiet filter (Phase 6.2.0). By default a
                        sampled position is dropped when the side to move has a
                        winning capture available (victim > attacker, or victim
                        undefended) — a cheap SEE>0 proxy. The old filter only
                        rejected positions where the PLAYED move was a capture.
    --blend LAMBDA      Blended labels (Phase 6.2.0): target = LAMBDA*result +
                        (1-LAMBDA)*sigmoid(own_search_cp/400), using the engine
                        eval parsed from the fastchess PGN comment of the move
                        played from the position ({+0.25/12 0.013s}, mover POV).
                        Default 1.0 = pure WDL. Applies to TRAIN only; the
                        holdout always keeps the pure game result so different
                        LAMBDA fits stay comparable on one fixed holdout.

Output format (FEN;target):
    rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq - 0 1;0.5

    target is from White's perspective in [0,1].

Requires:
    pip install chess
"""

import argparse
import os
import random
import re
import sys

try:
    import chess
    import chess.pgn
except ImportError:
    print("ERROR: python-chess not installed. Run: pip install chess", file=sys.stderr)
    sys.exit(1)


RESULT_MAP = {
    "1-0":     1.0,
    "0-1":     0.0,
    "1/2-1/2": 0.5,
}

# Game-phase weights, matching the engine's PHASE_W (knight/bishop 1, rook 2,
# queen 4; max 24 = full non-pawn material). Buckets match the tuner's
# bucket_of(): opening >= 16, middlegame 6..16, endgame < 6.
PHASE_W = {chess.KNIGHT: 1, chess.BISHOP: 1, chess.ROOK: 2, chess.QUEEN: 4}

# Rough piece values for the winning-capture (SEE>0 proxy) filter.
PIECE_VAL = {
    chess.PAWN: 1, chess.KNIGHT: 3, chess.BISHOP: 3,
    chess.ROOK: 5, chess.QUEEN: 9, chess.KING: 20,
}

# fastchess move comment: "{+0.25/12 0.013s}" or mate "{+M5/12 0.002s}",
# score in pawns from the MOVER's point of view.
COMMENT_CP = re.compile(r"^([+-]?)(M?)(\d+(?:\.\d+)?)/")

CP_CLAMP = 2000


def game_phase(board: "chess.Board") -> int:
    return sum(
        PHASE_W[pt] * len(board.pieces(pt, c))
        for pt in PHASE_W
        for c in (chess.WHITE, chess.BLACK)
    )


def phase_bucket(phase: int) -> int:
    return 0 if phase >= 16 else (1 if phase >= 6 else 2)


PHASE_NAMES = ("opening", "middlegame", "endgame")


def fen_key(fen: str) -> str:
    """Return the first 4 FEN fields as a deduplication key (position, side, castling, ep)."""
    return " ".join(fen.split()[:4])


def has_winning_capture(board: "chess.Board") -> bool:
    """Cheap SEE>0 proxy: a capture of a higher-valued piece, or of an
    undefended piece, is available to the side to move. Positions where this
    holds are tactically hot — a searching/label source resolves the tactic,
    a static eval cannot, so they poison the fit (Phase 6.2.0 true-quiet
    filter; the Ethereal/Zurichess 'quiet-labeled' requirement)."""
    for mv in board.generate_legal_captures():
        victim = board.piece_type_at(mv.to_square)
        if victim is None:  # en passant
            victim = chess.PAWN
        attacker = board.piece_type_at(mv.from_square)
        if PIECE_VAL[victim] > PIECE_VAL[attacker]:
            return True
        if not board.is_attacked_by(not board.turn, mv.to_square):
            return True
    return False


def comment_cp_white(comment: str, white_to_move: bool):
    """Parse the engine eval from a fastchess move comment into White-POV
    centipawns (clamped), or None when absent/unparseable."""
    m = COMMENT_CP.match(comment.strip())
    if not m:
        return None
    sign = -1.0 if m.group(1) == "-" else 1.0
    if m.group(2):  # mate score, e.g. +M5
        cp = sign * CP_CLAMP
    else:
        cp = sign * float(m.group(3)) * 100.0
    cp = max(-CP_CLAMP, min(CP_CLAMP, cp))
    return cp if white_to_move else -cp


def sigmoid_cp(cp: float) -> float:
    return 1.0 / (1.0 + 10.0 ** (-cp / 400.0))


def process_game(game, skip_start: int, skip_end: int, max_per_game: int,
                 quiet_filter: bool, rng: random.Random):
    """
    Extract qualifying (fen, phase_bucket, cp_white_or_None) triples from one
    game. Returns (triples, n_quiet_rejected).
    """
    result_str = game.headers.get("Result", "*")
    if result_str not in RESULT_MAP:
        return [], 0

    board = game.board()
    moves_with_comments = [(node.move, node.comment) for node in game.mainline()]
    n = len(moves_with_comments)

    candidates = []
    for ply_idx, (move, comment) in enumerate(moves_with_comments):
        if ply_idx < skip_start or ply_idx >= n - skip_end or board.is_check() \
                or board.is_capture(move) or move.promotion is not None:
            board.push(move)
            continue
        cp = comment_cp_white(comment or "", board.turn == chess.WHITE)
        candidates.append((board.fen(), phase_bucket(game_phase(board)), cp))
        board.push(move)

    # Sample BEFORE the (comparatively expensive) quiet check so the per-game
    # cost stays bounded; oversample 2x to compensate for quiet-filter drops,
    # then trim back to max_per_game.
    sample_n = max_per_game * 2 if quiet_filter else max_per_game
    if len(candidates) > sample_n:
        candidates = rng.sample(candidates, sample_n)

    quiet_rejected = 0
    if quiet_filter:
        kept = []
        for fen, bucket, cp in candidates:
            if has_winning_capture(chess.Board(fen)):
                quiet_rejected += 1
            else:
                kept.append((fen, bucket, cp))
        candidates = kept[:max_per_game]

    return candidates, quiet_rejected


def fmt_target(t: float) -> str:
    if t == 1.0:
        return "1"
    if t == 0.0:
        return "0"
    if t == 0.5:
        return "0.5"
    return f"{t:.4f}"


def main():
    parser = argparse.ArgumentParser(description=__doc__,
                                     formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument("pgn", help="Input PGN file path")
    parser.add_argument("--out-dir",      default="",          metavar="DIR")
    parser.add_argument("--train",        default="train.csv",    metavar="FILENAME")
    parser.add_argument("--holdout",      default="holdout.csv",  metavar="FILENAME")
    parser.add_argument("--holdout-pct",  default=5,   type=int, metavar="N")
    parser.add_argument("--max-per-game", default=12,  type=int, metavar="N")
    parser.add_argument("--skip-start",   default=16,  type=int, metavar="N",
                        help="Plies to skip at game start (default 16 = 8 full moves)")
    parser.add_argument("--skip-end",     default=6,   type=int, metavar="N",
                        help="Plies to skip at game end (default 6)")
    parser.add_argument("--seed",         default=42,  type=int, metavar="N")
    parser.add_argument("--min-train",    default=1_500_000, type=int, metavar="N")
    parser.add_argument("--balance-phase", default=0.0, type=float, metavar="R",
                        help="If >0, downsample over-represented phase buckets in TRAIN so "
                             "none exceeds R x the smallest bucket (e.g. 2.0). Lossy; off by "
                             "default. Holdout is never rebalanced.")
    parser.add_argument("--no-quiet-filter", dest="quiet_filter", action="store_false",
                        help="Disable the winning-capture-available quiet filter (6.2.0).")
    parser.add_argument("--blend", default=1.0, type=float, metavar="LAMBDA",
                        help="TRAIN target = LAMBDA*result + (1-LAMBDA)*sigmoid(own cp). "
                             "1.0 (default) = pure WDL. Holdout always stays pure WDL.")
    args = parser.parse_args()

    if not os.path.isfile(args.pgn):
        print(f"ERROR: PGN file not found: {args.pgn}", file=sys.stderr)
        sys.exit(1)
    if not (0.0 <= args.blend <= 1.0):
        print("ERROR: --blend must be in [0,1].", file=sys.stderr)
        sys.exit(1)

    out_dir = args.out_dir if args.out_dir else os.path.dirname(os.path.abspath(args.pgn))
    os.makedirs(out_dir, exist_ok=True)

    train_path   = os.path.join(out_dir, args.train)
    holdout_path = os.path.join(out_dir, args.holdout)

    rng = random.Random(args.seed)
    holdout_threshold = args.holdout_pct / 100.0

    seen: set[str] = set()
    train_positions   = []   # (fen, target, bucket)
    holdout_positions = []

    games_total    = 0
    games_skipped  = 0
    raw_candidates = 0
    quiet_rejected = 0
    missing_evals  = 0

    print(f"Reading PGN: {args.pgn}")
    print(f"  skip_start={args.skip_start} plies, skip_end={args.skip_end} plies, "
          f"max_per_game={args.max_per_game}, holdout={args.holdout_pct}%, "
          f"quiet_filter={'on' if args.quiet_filter else 'OFF'}, blend={args.blend}")

    with open(args.pgn, encoding="utf-8", errors="replace") as pgn_file:
        while True:
            try:
                game = chess.pgn.read_game(pgn_file)
            except Exception as exc:
                print(f"  WARNING: parse error, skipping game: {exc}", file=sys.stderr)
                games_skipped += 1
                continue

            if game is None:
                break

            games_total += 1
            if games_total % 10_000 == 0:
                print(f"  {games_total:,} games processed, "
                      f"train={len(train_positions):,}, holdout={len(holdout_positions):,}, "
                      f"unique positions so far={len(seen):,}")

            result_str = game.headers.get("Result", "*")
            triples, rejected = process_game(game, args.skip_start, args.skip_end,
                                             args.max_per_game, args.quiet_filter, rng)
            quiet_rejected += rejected
            if not triples:
                games_skipped += 1
                continue

            raw_candidates += len(triples)
            result = RESULT_MAP[result_str]

            # Split by game (not by position) to avoid train/holdout leakage
            is_holdout = rng.random() < holdout_threshold

            for fen, bucket, cp in triples:
                key = fen_key(fen)
                if key in seen:
                    continue
                seen.add(key)
                if is_holdout:
                    # Holdout keeps the pure game result: one fixed comparison
                    # set across different --blend fits.
                    holdout_positions.append((fen, result, bucket))
                elif args.blend < 1.0 and cp is not None:
                    target = args.blend * result + (1.0 - args.blend) * sigmoid_cp(cp)
                    train_positions.append((fen, target, bucket))
                else:
                    if args.blend < 1.0 and cp is None:
                        missing_evals += 1
                    train_positions.append((fen, result, bucket))

    def phase_counts(positions):
        c = [0, 0, 0]
        for _, _, b in positions:
            c[b] += 1
        return c

    def fmt_phase(positions):
        c = phase_counts(positions)
        tot = max(sum(c), 1)
        return ", ".join(f"{PHASE_NAMES[i]} {c[i]:,} ({100*c[i]/tot:.1f}%)" for i in range(3))

    # Optional phase rebalancing of TRAIN (holdout left untouched so it stays a
    # faithful sample of the played distribution).
    if args.balance_phase > 0:
        counts = phase_counts(train_positions)
        present = [n for n in counts if n > 0]
        if present:
            cap = int(args.balance_phase * min(present))
            by_bucket = ([], [], [])
            for item in train_positions:
                by_bucket[item[2]].append(item)
            balanced = []
            for b in range(3):
                bucket_items = by_bucket[b]
                if len(bucket_items) > cap:
                    bucket_items = rng.sample(bucket_items, cap)
                balanced.extend(bucket_items)
            rng.shuffle(balanced)
            print(f"\nPhase balance (cap = {args.balance_phase} x smallest = {cap:,}):")
            print(f"  before: {fmt_phase(train_positions)}")
            train_positions = balanced
            print(f"  after : {fmt_phase(train_positions)}")

    print(f"\nSummary:")
    print(f"  Games read       : {games_total:,}")
    print(f"  Games skipped    : {games_skipped:,}")
    print(f"  Raw candidates   : {raw_candidates:,}")
    print(f"  Quiet-rejected   : {quiet_rejected:,}")
    if args.blend < 1.0:
        print(f"  Missing evals    : {missing_evals:,} (fell back to pure WDL)")
    print(f"  Unique positions : {len(seen):,}")
    print(f"  Train positions  : {len(train_positions):,}")
    print(f"  Holdout positions: {len(holdout_positions):,}")
    print(f"  Train phase mix  : {fmt_phase(train_positions)}")
    print(f"  Holdout phase mix: {fmt_phase(holdout_positions)}")

    print(f"\nWriting {train_path} ...")
    with open(train_path, "w", encoding="utf-8") as f:
        for fen, target, _ in train_positions:
            f.write(f"{fen};{fmt_target(target)}\n")

    print(f"Writing {holdout_path} ...")
    with open(holdout_path, "w", encoding="utf-8") as f:
        for fen, target, _ in holdout_positions:
            f.write(f"{fen};{fmt_target(target)}\n")

    print(f"\nDone.")
    if len(train_positions) < args.min_train:
        print(f"\nWARNING: only {len(train_positions):,} training positions "
              f"(target >= {args.min_train:,}).")
        print("  Generate more games with datagen.ps1 (try more -Rounds or different -Nodes).")
        sys.exit(2)
    else:
        print(f"Target met: {len(train_positions):,} >= {args.min_train:,} training positions.")


if __name__ == "__main__":
    main()
