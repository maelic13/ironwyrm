"""Structural checks for independent exchange truth, including its limits."""
import unittest
from unittest.mock import patch

import chess
import see_contract_oracle as oracle


class SeeOracleTests(unittest.TestCase):
    def test_committed_fixture_and_wrong_arithmetic_negative_control(self):
        self.assertEqual(oracle.OUTPUT.read_text(encoding="utf-8"), oracle.render())
        wrong = list(oracle.CASES[0])
        wrong[-1] += 1
        with patch.object(oracle, "CASES", [tuple(wrong)]):
            with self.assertRaises(AssertionError):
                oracle.render()

    def test_color_mirrors_and_restoration(self):
        for name, _, fen, uci, expected in oracle.CASES:
            with self.subTest(name=name):
                board = chess.Board(fen).mirror()
                mv = chess.Move.from_uci(uci)
                mirrored = chess.Move(chess.square_mirror(mv.from_square),
                                      chess.square_mirror(mv.to_square), mv.promotion)
                before = board.fen()
                self.assertEqual(oracle.exchange(board, mirrored), expected)
                self.assertEqual(board.fen(), before)
                self.assertEqual(board.move_stack, [])

    def test_created_and_released_pins_are_real(self):
        created = chess.Board("2k5/2n5/2B5/3p4/8/8/8/2R1K3 w - - 0 1")
        self.assertFalse(created.is_pinned(chess.BLACK, chess.C7))
        created.push_uci("c6d5")
        self.assertTrue(created.is_pinned(chess.BLACK, chess.C7))
        self.assertNotIn(chess.Move.from_uci("c7d5"), created.legal_moves)
        released = chess.Board("k7/1r1p4/2B5/8/8/8/8/4K3 w - - 0 1")
        self.assertTrue(released.is_pinned(chess.BLACK, chess.B7))
        released.push_uci("c6d7")
        self.assertFalse(released.is_pinned(chess.BLACK, chess.B7))
        self.assertIn(chess.Move.from_uci("b7d7"), released.legal_moves)

    def test_all_promotion_replies_and_best_choice(self):
        board = chess.Board("7k/8/8/8/8/7K/pR6/1r6 w - - 0 1")
        board.push_uci("b2b1")
        replies = [m for m in board.legal_moves if m.to_square == chess.B1 and board.is_capture(m)]
        self.assertEqual({m.promotion for m in replies},
                         {chess.QUEEN, chess.ROOK, chess.BISHOP, chess.KNIGHT})
        self.assertEqual(oracle.recaptures(board, chess.B1), 1300)

    def test_illegal_initial_move_is_rejected(self):
        with self.assertRaises(ValueError):
            oracle.exchange(chess.Board(), chess.Move.from_uci("e2e5"))


if __name__ == "__main__":
    unittest.main()
