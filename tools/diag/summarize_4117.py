"""Validate and tabulate the registered 4.11.7 per-position reports."""
import hashlib
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
OUT = ROOT / "tools/results/budget-transfer-20260905"
BUDGETS = (60000, 200000, 600000)


def main():
    manifest = json.loads((OUT / "manifest.json").read_text())
    assert manifest["status"] == "complete", "study incomplete"
    assert len(manifest["runs"]) == 6
    reports = {}
    success = {}
    for run in manifest["runs"]:
        key = (run["arm"], run["budget"])
        assert key not in reports and run["returncode"] == 0
        path = OUT / key[0] / f"truth-{key[1]}.json"
        assert hashlib.sha256(path.read_bytes()).hexdigest() == run["report_sha256"]
        report = json.loads(path.read_text())
        assert report["nodes_per_move"] == key[1]
        assert report["positions_per_family"] == 100
        assert report["max_plies"] == 100 and report["hash_mb"] == 16
        assert report["persistent_tt_per_game"] and report["workers"] == 30
        assert report["seed"] == 6200600
        reports[key] = report
        success[key] = {}
        for family, entry in report["families"].items():
            records = entry["positions"]
            assert len(records) == 100
            assert len({p["fen"] for p in records}) == 100
            wins = {p["fen"]: p for p in records if p["theory_wdl"] == 2}
            converted = {fen for fen, p in wins.items() if p["outcome"] == "mated"}
            assert len(wins) == entry["theoretically_won"]
            assert len(converted) == entry["converted"]
            success[key][family] = converted
    baseline = reports[("head", 60000)]
    for report in reports.values():
        assert report["cohort"] == baseline["cohort"]
        assert list(report["families"]) == list(baseline["families"])
        for family, entry in report["families"].items():
            identities = lambda e: [(p["index"], p["fen"], p["theory_wdl"], p["theory_dtz"])
                                    for p in e["positions"]]
            assert identities(entry) == identities(baseline["families"][family])

    rows = []
    for family, entry in baseline["families"].items():
        row = {"family": family, "won": entry["theoretically_won"], "budgets": {}}
        for budget in BUDGETS:
            head = success[("head", budget)][family]
            reference = success[("reference", budget)][family]
            old = success[("head", 60000)][family]
            row["budgets"][str(budget)] = {
                "head": len(head), "reference": len(reference),
                "head_only": len(head - reference), "reference_only": len(reference - head),
                "head_gained_vs_60k": sorted(head - old),
                "head_lost_vs_60k": sorted(old - head),
            }
            previous = BUDGETS[max(0, BUDGETS.index(budget) - 1)]
            row["budgets"][str(budget)]["changes_from_previous_budget"] = {
                arm: {
                    "previous_budget": previous,
                    "gained": sorted(success[(arm, budget)][family] - success[(arm, previous)][family]),
                    "lost": sorted(success[(arm, previous)][family] - success[(arm, budget)][family]),
                }
                for arm in ("head", "reference")
            }
        rows.append(row)
    (OUT / "comparison.json").write_text(json.dumps(rows, indent=2) + "\n", encoding="utf-8")
    print("| Family | Won | R / SF 60k | R / SF 200k | R / SF 600k |")
    print("|---|---:|---:|---:|---:|")
    for row in rows:
        cells = [f"{row['budgets'][str(b)]['head']} / {row['budgets'][str(b)]['reference']}" for b in BUDGETS]
        print(f"| {row['family']} | {row['won']} | " + " | ".join(cells) + " |")
    for budget in BUDGETS:
        totals = {k: sum(r["budgets"][str(budget)][k] for r in rows)
                  for k in ("head", "reference", "head_only", "reference_only")}
        totals["won"] = sum(r["won"] for r in rows)
        print(budget, totals)


if __name__ == "__main__":
    main()
