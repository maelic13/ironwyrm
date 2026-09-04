"""Unit tests for the endgame truth harness's termination rule (PLAN 4.10.1).

These are the tests RAR-E14 says were missing. The defect they cover is not a
crash: the old harness ended a game the moment the strong side's piece count
dropped, which reads as a plausible "material lost" outcome and silently scores
correct pawn technique as a failure. Nothing about the output looked wrong.

So each test below asserts a BEHAVIOUR, and the schema tests assert that the
guards FAIL on a known-bad input rather than merely pass on a good one -- which
is 4.10.4's rule applied to the change that motivated it.

Run:
  python -m unittest discover -s tools/diag -p "test_*.py"
"""

import json
import sys
import tempfile
import unittest
from pathlib import Path

import chess

sys.path.insert(0, str(Path(__file__).resolve().parent))
import endgame_floors
import endgame_truth


class StubEngine:
    """Plays a scripted move list, then the first legal move forever.

    `play_and_grade` only needs `.play(board, limit, game=...)`. Scripting the
    moves is what makes the termination rule testable without an engine binary,
    a tablebase or a node budget.
    """

    def __init__(self, moves):
        self.moves = list(moves)
        self.calls = 0

    def play(self, board, limit, game=None):
        self.calls += 1
        if self.moves:
            move = chess.Move.from_uci(self.moves.pop(0))
        else:
            move = next(iter(board.legal_moves))

        class Result:
            pass

        result = Result()
        result.move = move
        return result


class StubTablebase:
    """WDL/DTZ from a caller-supplied table, defaulting to a clean White win."""

    def __init__(self, wdl=2, dtz=20):
        self.wdl = wdl
        self.dtz = dtz

    def probe_wdl(self, board):
        # play_and_grade converts to White's point of view by the side to move.
        return self.wdl if board.turn == chess.WHITE else -self.wdl

    def probe_dtz(self, board):
        return self.dtz


class TerminationRuleTests(unittest.TestCase):
    def test_pawn_sacrifice_does_not_end_the_game(self):
        """The defect itself: KPP-K, White gives one pawn to promote the other.

        Under the old rule this returned `material_lost` at ply 1 with the win
        still intact. It must now play on and record the shed ply instead.
        """
        # White Kd6, pawns c7 and d5; Black Kc8 to move and able to take on c7.
        # Losing the c-pawn leaves K+P vs K, which is NOT insufficient material
        # and is often still won -- exactly the case the old rule aborted.
        board = chess.Board("2k5/2P5/3K4/3P4/8/8/8/8 b - - 0 1")
        self.assertEqual(chess.popcount(board.occupied_co[chess.WHITE]), 3)

        played = endgame_truth.play_and_grade(
            StubEngine(["c8c7"]), StubTablebase(), board,
            nodes=1, max_plies=12, game_token=object(),
        )
        self.assertNotEqual(played["outcome"], "material_lost")
        self.assertIsNotNone(played["shed_material_ply"])
        self.assertGreater(played["plies"], played["shed_material_ply"])

    def test_material_lost_is_no_longer_a_possible_outcome(self):
        """The whole point of 4.10.1: nothing terminates on material any more."""
        source = Path(endgame_truth.__file__).read_text(encoding="utf-8")
        self.assertNotIn('outcome = "material_lost"', source)

    def test_shed_ply_records_the_first_drop_only(self):
        # The capture happens on ply 0, so the drop is first observed by the
        # ply-1 check. A later second capture must not overwrite it.
        board = chess.Board("2k5/2P5/3K4/3P4/8/8/8/8 b - - 0 1")
        played = endgame_truth.play_and_grade(
            StubEngine(["c8c7"]), StubTablebase(), board,
            nodes=1, max_plies=20, game_token=object(),
        )
        self.assertEqual(played["shed_material_ply"], 1)

    def test_bare_king_shed_is_caught_by_insufficient_material_first(self):
        """The isolation proof, asserted rather than assumed.

        In every bare-king family a strong-side loss reaches a position the
        insufficient-material test terminates on. If that ordering is ever
        changed, this fails.
        """
        for fen, name in [
            ("4k3/8/8/8/8/8/8/4K1N1 b - - 0 1", "KN-K after a KNN-K shed"),
            ("4k3/8/8/8/8/8/8/4K1B1 b - - 0 1", "KB-K after a KBN-K shed"),
        ]:
            with self.subTest(name):
                board = chess.Board(fen)
                self.assertTrue(board.is_insufficient_material())
                played = endgame_truth.play_and_grade(
                    StubEngine([]), StubTablebase(wdl=0), board,
                    nodes=1, max_plies=8, game_token=object(),
                )
                self.assertEqual(played["outcome"], "insufficient_material")

    def test_report_records_the_new_diagnostic(self):
        board = chess.Board("2k5/2P5/3K4/3P4/8/8/8/8 b - - 0 1")
        played = endgame_truth.play_and_grade(
            StubEngine(["c8c7"]), StubTablebase(), board,
            nodes=1, max_plies=12, game_token=object(),
        )
        self.assertIn("shed_material_ply", played)
        self.assertIn("first_discard_ply", played)


class SchemaGuardTests(unittest.TestCase):
    """4.10.4: a guard is not verified until it FAILS on a known-bad input."""

    def _write(self, directory, name, doc):
        path = Path(directory) / name
        path.write_text(json.dumps(doc), encoding="utf-8")
        return path

    def test_floors_reject_a_v1_truth_report(self):
        with tempfile.TemporaryDirectory() as d:
            bad = self._write(d, "truth.json", {
                "schema": "rarog-endgame-truth-v1", "families": {}})
            with self.assertRaises(SystemExit) as caught:
                endgame_floors.load_report(bad)
            self.assertIn("v1", str(caught.exception))

    def test_floors_accept_a_v2_truth_report(self):
        with tempfile.TemporaryDirectory() as d:
            good = self._write(d, "truth.json", {
                "schema": "rarog-endgame-truth-v2", "families": {}})
            self.assertEqual(endgame_floors.load_report(good)["families"], {})

    def test_committed_floors_are_rejected_until_re_derived(self):
        """The committed floors predate 4.10.1 and must fail closed (4.11.2)."""
        doc = json.loads(
            endgame_floors.DEFAULT_FLOORS.read_text(encoding="utf-8"))
        self.assertNotEqual(doc.get("truth_schema"), endgame_floors.TRUTH_SCHEMA)

    def test_truth_runner_emits_v2(self):
        source = Path(endgame_truth.__file__).read_text(encoding="utf-8")
        self.assertIn('"schema": "rarog-endgame-truth-v2"', source)


if __name__ == "__main__":
    unittest.main()
