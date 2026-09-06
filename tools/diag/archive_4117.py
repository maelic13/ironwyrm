"""Archive raw measurement bytes without Git newline conversion."""
import hashlib
import json
from pathlib import Path
import zipfile

ROOT = Path(__file__).resolve().parents[2]
OUT = ROOT / "tools/results/budget-transfer-20260905"
DEST = ROOT / "analysis/artifacts/budget-transfer-20260905.zip"


def main():
    manifest = json.loads((OUT / "manifest.json").read_text())
    assert manifest["status"] == "complete"
    files = {p.relative_to(ROOT).as_posix(): p for p in OUT.rglob("*") if p.is_file()}
    for arm in ("head", "reference"):
        path = ROOT / f"tools/results/truth-v2-{arm}/endgame-truth.json"
        files[path.relative_to(ROOT).as_posix()] = path
    for name, expected in manifest["harness"].items():
        path = ROOT / "tools/diag" / name
        assert hashlib.sha256(path.read_bytes()).hexdigest() == expected
        files[f"frozen_harness/{name}"] = path
    for name in ("test-debug.txt", "test-release.txt", "test-tooling.txt", "clippy.txt", "incomplete-guard.txt", "comparison-guards.txt"):
        files[f"validation/{name}"] = Path("D:/chess/results/rarog-4117-build") / name
    fingerprints = list(Path("D:/chess/results/rarog-4117-build/release/.fingerprint").glob("rarog-*/bin-rarog.json"))
    assert len(fingerprints) == 1
    assert json.loads(fingerprints[0].read_text())["features"] == "[]"
    files["validation/production-build-fingerprint.json"] = fingerprints[0]
    DEST.parent.mkdir(parents=True, exist_ok=True)
    with zipfile.ZipFile(DEST, "w", compression=zipfile.ZIP_DEFLATED, compresslevel=9) as bundle:
        for name, path in sorted(files.items()):
            bundle.write(path, name)
    with zipfile.ZipFile(DEST) as bundle:
        assert bundle.testzip() is None
        assert set(bundle.namelist()) == set(files)
        for name, path in files.items():
            assert bundle.read(name) == path.read_bytes(), name
    print(f"Verified {len(files)} files; {DEST.stat().st_size} bytes")
    print("SHA256", hashlib.sha256(DEST.read_bytes()).hexdigest())


if __name__ == "__main__":
    main()
