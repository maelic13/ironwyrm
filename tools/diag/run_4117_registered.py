"""Execute the frozen 4.11.7 study; fail closed on a changed 60k baseline.

This is the reproduction recipe for the dated measurement, not a general CLI.
Run from the repository root. Refuses an existing output directory.
"""
from __future__ import annotations

import hashlib
import json
from pathlib import Path
import shutil
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[2]
OUT = ROOT / "tools/results/budget-transfer-20260905"
ARCHIVE = Path("D:/chess/results/budget-transfer-20260905")


def digest(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main() -> None:
    OUT.mkdir(parents=True, exist_ok=False)
    ARCHIVE.mkdir(parents=True, exist_ok=False)
    baseline = json.loads((ROOT / "tools/results/truth-v2-head/endgame-truth.json").read_text())
    families = ",".join(baseline["families"])
    manifest = {
        "source": subprocess.check_output(["git", "rev-parse", "HEAD"], cwd=ROOT, text=True).strip(),
        "rustc": subprocess.check_output(["rustc", "-Vv"], text=True),
        "build": "RUSTFLAGS='-C target-cpu=native --cfg rarog_pext' cargo build --release --locked --no-default-features --target-dir D:/chess/results/rarog-4117-build -j4",
        "bench13": {"nodes": 6901489, "ebf": 2.458},
        "harness": {name: digest(ROOT / "tools/diag" / name) for name in
                    ("endgame_truth.py", "endgame_budget_bracket.py", Path(__file__).name)},
        "binaries": {}, "runs": [], "status": "running",
    }
    manifest_path = OUT / "manifest.json"

    def save() -> None:
        manifest_path.write_text(json.dumps(manifest, indent=2) + "\n", encoding="utf-8")

    engines = {
        "head": Path("D:/chess/results/rarog-4117-build/release/rarog.exe"),
        "reference": Path("D:/chess/engines/stockfish-windows-x86-64-bmi2.exe"),
    }
    for arm, source in engines.items():
        target = ARCHIVE / f"{arm}.exe"
        shutil.copy2(source, target)
        assert digest(source) == digest(target)
        engines[arm] = target
        manifest["binaries"][arm] = {"source": str(source), "archive": str(target), "sha256": digest(target)}
    shutil.copy2("D:/chess/results/rarog-4117-build/bench13.txt", OUT / "bench13.txt")
    save()
    try:
        for budget in (60000, 200000, 600000):
            for arm, engine in engines.items():
                assert digest(engine) == manifest["binaries"][arm]["sha256"]
                cmd = [sys.executable, str(ROOT / "tools/diag/endgame_budget_bracket.py"),
                       "--engine", str(engine), "--syzygy", "D:/chess/tablebases/syzygy3456",
                       "--families", families, "--positions", "100", "--max-plies", "100",
                       "--seed", "6200600", "--workers", "30", "--budgets", str(budget),
                       "--out-dir", str(OUT / arm)]
                print(f"START {arm} {budget}", flush=True)
                with (OUT / f"{arm}-{budget}.log").open("w", encoding="utf-8") as log:
                    result = subprocess.run(cmd, cwd=ROOT, stdout=log, stderr=subprocess.STDOUT)
                manifest["runs"].append({"arm": arm, "budget": budget, "command": cmd,
                                         "returncode": result.returncode})
                save()
                result.check_returncode()
                report_path = OUT / arm / f"truth-{budget}.json"
                report = json.loads(report_path.read_text())
                assert report["cohort"] == baseline["cohort"], "cohort changed"
                assert report["nodes_per_move"] == budget
                if budget == 60000:
                    old = json.loads((ROOT / f"tools/results/truth-v2-{arm}/endgame-truth.json").read_text())
                    assert report["families"] == old["families"], f"{arm} 60k results changed; investigate"
                manifest["runs"][-1]["report_sha256"] = digest(report_path)
                save()
                print(f"PASS {arm} {budget}: cohort/provenance verified", flush=True)
        manifest["status"] = "complete"
        save()
    except BaseException as error:
        manifest["status"] = "stopped"
        manifest["error"] = str(error)
        save()
        raise


if __name__ == "__main__":
    main()
