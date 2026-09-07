"""One-off transcript capture around the existing bench parser, no new parser."""
import hashlib
import json
from pathlib import Path
import re
import sys

sys.path.insert(0, str(Path('tools/diag').resolve()))
import bench_counters

out = Path('analysis/artifacts/see-repair-20260906')
exe = Path('target/rarog-see-repair.exe').resolve()
identity = json.loads((out / 'identity.json').read_text())
assert hashlib.sha256(exe.read_bytes()).hexdigest().upper() == identity['production_sha256']
real_popen = bench_counters.subprocess.Popen
processes = []
lines = []

class Tee:
    def __init__(self, stream):
        self.stream = stream
    def readline(self):
        line = self.stream.readline()
        lines.append(line)
        return line

def recorded_popen(*args, **kwargs):
    proc = real_popen(*args, **kwargs)
    proc.stdout = Tee(proc.stdout)
    processes.append(proc)
    return proc

bench_counters.subprocess.Popen = recorded_popen
try:
    nodes, counters, dumps, sequence = bench_counters.run_bench(exe, 13, 1, [])
finally:
    bench_counters.subprocess.Popen = real_popen
    (out / 'bench13-uci.log').write_text(''.join(lines), encoding='utf-8')
assert len(processes) == 1 and processes[0].returncode == 0
assert nodes > 0 and not counters
ebf = re.findall(r'^Geomean EBF\s*:\s*(\d+\.\d+)$', ''.join(lines), re.M)
assert len(ebf) == 1, ebf
result = dict(nodes=nodes, ebf=ebf[0], engine_exit=processes[0].returncode,
              driver='tools/diag/bench_counters.py::run_bench', production_sha256=identity['production_sha256'])
(out / 'bench-result.json').write_text(json.dumps(result, indent=2) + '\n')
print(json.dumps(result))
