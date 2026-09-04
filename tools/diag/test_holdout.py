"""Tests for the held-out selection contract (PLAN 4.10.7).

Every guard here is exercised on a known-BAD input as well as a good one, per
PLAN rule 15 and the lesson of 4.10.4, where an anchor that only ever saw good
inputs passed under the vector it existed to catch.

Run:
  python -m unittest discover -s tools/diag -p "test_*.py"
"""

import json
import sys
import tempfile
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import holdout


FENS = [f"8/8/8/8/8/8/{i}k5/K7 w - - 0 1" for i in range(1, 61)]


class SplitTests(unittest.TestCase):
    def test_the_split_is_a_partition(self):
        dev, held = holdout.split_cohort(FENS)
        self.assertEqual(sorted(dev + held), list(range(len(FENS))))
        self.assertFalse(set(dev) & set(held))

    def test_the_split_is_deterministic(self):
        self.assertEqual(holdout.split_cohort(FENS), holdout.split_cohort(FENS))

    def test_the_split_follows_the_position_not_its_index(self):
        """Reordering or extending the cohort must not move a position.

        Assignment by index would let a regeneration in a different order
        silently reshuffle which half decides -- the same class of defect as
        seeding a family by its list index (RAR-E14 defect B).
        """
        dev, held = holdout.split_cohort(FENS)
        held_fens = {FENS[i] for i in held}
        shuffled = list(reversed(FENS)) + ["8/8/8/8/8/8/8/K6k w - - 0 1"]
        _, held2 = holdout.split_cohort(shuffled)
        self.assertEqual({shuffled[i] for i in held2} & set(FENS), held_fens)

    def test_the_fraction_is_respected_approximately(self):
        _, held = holdout.split_cohort(FENS, holdout_fraction=0.25)
        self.assertLess(abs(len(held) / len(FENS) - 0.25), 0.15)

    def test_a_degenerate_fraction_is_refused(self):
        for bad in (0.0, 1.0, -0.5, 1.5):
            with self.subTest(bad), self.assertRaises(ValueError):
                holdout.split_cohort(FENS, holdout_fraction=bad)


class McNemarTests(unittest.TestCase):
    def test_agreement_carries_no_information(self):
        """200 positions both arms convert say nothing about which is better."""
        a = [True] * 30
        result = holdout.mcnemar(a, list(a))
        self.assertEqual(result["discordant"], 0)
        self.assertIsNone(result["z"])

    def test_a_clear_advantage_to_a_is_positive(self):
        a = [True] * 20 + [False] * 5
        b = [False] * 20 + [False] * 5
        self.assertGreater(holdout.mcnemar(a, b)["z"], 3.0)

    def test_the_sign_reverses_with_the_arms(self):
        a = [True] * 20 + [False] * 5
        b = [False] * 20 + [False] * 5
        self.assertAlmostEqual(holdout.mcnemar(a, b)["z"],
                               -holdout.mcnemar(b, a)["z"])

    def test_thin_discordance_is_indeterminate_not_a_number(self):
        a = [True] * 3 + [True] * 40
        b = [False] * 3 + [True] * 40
        result = holdout.mcnemar(a, b)
        self.assertIsNone(result["z"])
        self.assertIn("indeterminate", result)

    def test_unpaired_inputs_are_refused(self):
        with self.assertRaises(ValueError):
            holdout.mcnemar([True, False], [True])


class SeparationTests(unittest.TestCase):
    def test_a_plateau_is_reported_as_a_plateau(self):
        """Best of N without separation is not a winner (BAS-E41)."""
        arms = {
            "leader": [True] * 21 + [False] * 9,
            "second": [True] * 20 + [False] * 10,
            "third": [True] * 19 + [False] * 11,
        }
        out = holdout.separation(arms, "leader")
        self.assertTrue(out["is_plateau"])
        self.assertEqual(out["separated_from"], [])

    def test_a_real_separation_is_reported(self):
        arms = {
            "leader": [True] * 25 + [False] * 5,
            "weak": [False] * 25 + [False] * 5,
        }
        out = holdout.separation(arms, "leader")
        self.assertFalse(out["is_plateau"])
        self.assertEqual(out["separated_from"], ["weak"])


class RegistrationTests(unittest.TestCase):
    def _doc(self, **over):
        doc = {
            "cohort_sha256": "aa" * 32,
            "deciding_half": "holdout",
            "arms": ["cand-a", "cand-b"],
            "runner_up": "cand-b",
            "policy": "accept the leader if held-out McNemar z >= 2",
        }
        doc.update(over)
        return doc

    def test_a_valid_registration_is_written(self):
        with tempfile.TemporaryDirectory() as d:
            p = Path(d) / "reg.json"
            holdout.register(p, self._doc())
            self.assertEqual(
                json.loads(p.read_text())["schema"],
                "rarog-holdout-registration-v1")

    def test_it_refuses_to_be_rewritten(self):
        with tempfile.TemporaryDirectory() as d:
            p = Path(d) / "reg.json"
            holdout.register(p, self._doc())
            with self.assertRaises(SystemExit):
                holdout.register(p, self._doc(deciding_half="development"))

    def test_a_missing_runner_up_is_refused(self):
        with tempfile.TemporaryDirectory() as d:
            with self.assertRaises(ValueError) as caught:
                holdout.register(Path(d) / "r.json", self._doc(runner_up=None))
            self.assertIn("runner-up", str(caught.exception))

    def test_a_runner_up_outside_the_arms_is_refused(self):
        with tempfile.TemporaryDirectory() as d:
            with self.assertRaises(ValueError):
                holdout.register(Path(d) / "r.json", self._doc(runner_up="ghost"))

    def test_a_nonsense_deciding_half_is_refused(self):
        with tempfile.TemporaryDirectory() as d:
            with self.assertRaises(ValueError):
                holdout.register(Path(d) / "r.json", self._doc(deciding_half="both"))

    def test_missing_fields_are_refused(self):
        with tempfile.TemporaryDirectory() as d:
            doc = self._doc()
            del doc["policy"]
            with self.assertRaises(ValueError):
                holdout.register(Path(d) / "r.json", doc)


class SpentCohortTests(unittest.TestCase):
    COHORT = "cc" * 32

    def _ledger(self):
        return {"schema": "rarog-spent-cohorts-v1",
                "spent": {self.COHORT: {"step": "4.12.9", "date": "2026-09-04"}}}

    def test_a_fresh_cohort_may_select(self):
        holdout.check_not_spent({"spent": {}}, self.COHORT, "selection")

    def test_a_spent_cohort_may_not_select(self):
        with self.assertRaises(SystemExit) as caught:
            holdout.check_not_spent(self._ledger(), self.COHORT, "selection")
        self.assertIn("SPENT", str(caught.exception))
        self.assertIn("4.12.9", str(caught.exception))

    def test_a_spent_cohort_may_still_veto(self):
        """A safety property does not get less true from reuse."""
        holdout.check_not_spent(self._ledger(), self.COHORT, "veto")

    def test_an_unknown_purpose_is_refused(self):
        with self.assertRaises(ValueError):
            holdout.check_not_spent({"spent": {}}, self.COHORT, "whatever")

    def test_spending_round_trips(self):
        with tempfile.TemporaryDirectory() as d:
            p = Path(d) / "spent.json"
            holdout.spend(p, self.COHORT, "4.12.9", "2026-09-04")
            with self.assertRaises(SystemExit):
                holdout.check_not_spent(holdout.load_spent(p),
                                        self.COHORT, "selection")


if __name__ == "__main__":
    unittest.main()
