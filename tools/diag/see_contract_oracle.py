"""Independent legal same-square exchange oracle for PLAN 4.11b.4.

Each side may decline further exchange, even in check: this is a material
subgame, not tactical search. Enumerate ALL legal recaptures (including every
promotion), never a single LVA reply. No Rarog code or binary supplies truth.
"""

import argparse
from pathlib import Path

import chess

ROOT = Path(__file__).resolve().parents[2]
OUTPUT = ROOT / "tests/data/see-contract-v1.tsv"
VALUES = {chess.PAWN: 100, chess.KNIGHT: 320, chess.BISHOP: 330,
          chess.ROOK: 500, chess.QUEEN: 900, chess.KING: 32000}
# name, disposition, FEN, initial move, hand-derived material result
CASES = [
    ("free-pawn", "exchange", "4k3/8/8/3p4/8/8/3R4/4K3 w - - 0 1", "d2d5", 100),
    ("defended-pawn", "exchange", "4k3/8/2p5/3p4/8/8/3R4/4K3 w - - 0 1", "d2d5", -400),
    ("king-after-pawn", "debt", "7k/8/2p5/3pK3/8/8/3R4/8 w - - 0 1", "d2d5", -300),
    ("legal-king-recapture", "exchange", "3qk3/8/8/8/8/8/3Q4/3K4 w - - 0 1", "d2d8", 0),
    ("defended-king-destination", "exchange", "3qk3/8/8/6B1/8/8/3Q4/3K4 w - - 0 1", "d2d8", 900),
    ("pinned-pawn", "exchange", "3k4/3p4/2p5/8/4B3/8/8/3RK3 w - - 0 1", "e4c6", 100),
    ("pin-created", "debt", "2k5/2n5/2B5/3p4/8/8/8/2R1K3 w - - 0 1", "c6d5", 100),
    ("pin-released", "exchange", "k7/1r1p4/2B5/8/8/8/8/4K3 w - - 0 1", "c6d7", -230),
    ("xray", "exchange", "3rk3/8/8/3r4/8/8/3R4/3RK3 w - - 0 1", "d2d5", 500),
    ("ep-opens-rook", "exchange", "7k/8/8/3pP3/8/8/K7/3r4 w - d6 0 1", "e5d6", 0),
    ("quiet-hanging", "policy-quiet", "7k/8/8/2p5/8/8/3R4/7K w - - 0 1", "d2d4", -500),
    ("quiet-promotion-hanging", "policy-promotion", "7r/P7/7k/8/8/8/8/K7 w - - 0 1", "a7a8q", -100),
    ("quiet-underpromotion", "policy-promotion", "4k3/P7/8/8/8/8/8/4K3 w - - 0 1", "a7a8n", 220),
    ("capture-promotion", "exchange", "1r2k3/P7/8/8/8/8/8/4K3 w - - 0 1", "a7b8q", 1300),
    ("capture-underpromotion", "exchange", "1r2k3/P7/8/8/8/8/8/4K3 w - - 0 1", "a7b8n", 720),
    ("promotion-recapture", "debt", "7k/8/8/8/8/7K/pR6/1r6 w - - 0 1", "b2b1", -800),
    ("castle-king", "policy-castle", "4k3/8/8/8/8/8/8/R3K2R w KQ - 0 1", "e1g1", 0),
    ("castle-queen", "policy-castle", "4k3/8/8/8/8/8/8/R3K2R w KQ - 0 1", "e1c1", 0),
]


def gain(board, move):
    victim = chess.PAWN if board.is_en_passant(move) else board.piece_type_at(move.to_square)
    return VALUES.get(victim, 0) + (VALUES[move.promotion] - 100 if move.promotion else 0)


def recaptures(board, target):
    best = 0
    for move in list(board.legal_moves):
        if move.to_square != target or not board.is_capture(move):
            continue
        immediate = gain(board, move)
        board.push(move)
        value = immediate - recaptures(board, target)
        board.pop()
        best = max(best, value)
    return best


def exchange(board, move):
    if move not in board.legal_moves:
        raise ValueError(f"illegal initial move {move}")
    immediate = gain(board, move)
    board.push(move)
    result = immediate - recaptures(board, move.to_square)
    board.pop()
    return result


def render():
    lines = ["# see-contract-v1: name|disposition|FEN|move|legal-tree value|immediate gain"]
    for name, disposition, fen, uci, expected in CASES:
        board = chess.Board(fen)
        assert board.is_valid(), name
        move = chess.Move.from_uci(uci)
        value = exchange(board, move)
        assert value == expected, (name, value, expected)
        assert board.fen(en_passant="fen") == fen, name
        lines.append(f"{name}|{disposition}|{fen}|{uci}|{value}|{gain(board, move)}")
    return "\n".join(lines) + "\n"


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--write", action="store_true")
    args = parser.parse_args()
    result = render()
    if args.write:
        OUTPUT.write_text(result, encoding="utf-8")
    else:
        assert OUTPUT.read_text(encoding="utf-8") == result, "oracle fixture drift"
    print(f"see-contract-v1: {len(CASES)} independent fixtures PASS; python-chess {chess.__version__}")


if __name__ == "__main__":
    main()
