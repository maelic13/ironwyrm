# Board-implementation speed comparison via perft (movegen + make/unmake only,
# no search, no eval). All four engines bulk-count at depth 1 (verified in
# source), so nodes/sec is directly comparable. Timing is external
# (write-of-go -> sentinel line), identical for every engine; the ~ms pipe
# latency is noise against multi-second runs. Node counts are checked against
# the known perft values, so a mismatch = movegen bug, not a timing artifact.
import subprocess, sys, time

POSITIONS = [
    # (label, fen or None for startpos, depth, expected nodes)
    ("P1 startpos (opening)", None, 6, 119_060_324),
    ("P2 kiwipete (tactical mg)", "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1", 5, 193_690_690),
    ("P3 rook endgame (EP pins)", "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1", 7, 178_633_661),
    ("P4 promo storm", "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1", 6, 706_045_033),
    ("P5 promo+checks", "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8", 5, 89_941_194),
    ("P6 quiet middlegame", "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10", 5, 164_075_551),
]

ENGINES = [
    # (name, exe, go_format, sentinel_prefix, node_parser)
    ("Rarog p103-gate", r"D:\code\rarog\tools\test_engines\rarog-p103head-a-pext-pgo.exe",
     "go perft {d}", "Nodes searched:", lambda l: int(l.split(":")[1].strip())),
    ("Basilisk 1.9.0", r"D:\chess\engines\basilisk-v1.9.0-windows-x86_64-pext-pgo.exe",
     "go perft {d}", "Nodes searched:", lambda l: int(l.split(":")[1].strip())),
    ("Reckless (local)", r"D:\code\Reckless\target\release\reckless.exe",
     "simpleperft {d}", "total:", lambda l: int(l.split(":")[1].strip())),
    ("Stockfish bmi2", r"D:\chess\engines\stockfish-windows-x86-64-bmi2.exe",
     "go perft {d}", "Nodes searched:", lambda l: int(l.split(":")[1].strip())),
]

ROUNDS = 3

def start(exe):
    p = subprocess.Popen([exe], stdin=subprocess.PIPE, stdout=subprocess.PIPE,
                         stderr=subprocess.STDOUT, text=True, bufsize=1)
    return p

def send(p, line):
    p.stdin.write(line + "\n")
    p.stdin.flush()

def wait_for(p, prefix, timeout=600):
    t_end = time.time() + timeout
    while time.time() < t_end:
        line = p.stdout.readline()
        if not line:
            raise RuntimeError("engine died")
        if line.strip().startswith(prefix):
            return line.strip()
    raise RuntimeError("timeout")

def get_id(p):
    send(p, "uci")
    name = "?"
    while True:
        line = p.stdout.readline().strip()
        if line.startswith("id name"):
            name = line[8:]
        if line == "uciok":
            return name

procs = {}
ids = {}
for name, exe, *_ in ENGINES:
    p = start(exe)
    procs[name] = p
    ids[name] = get_id(p)
    print(f"engine: {name:18} -> {ids[name]}", flush=True)

# results[engine][pos_label] = list of (elapsed, nodes)
results = {name: {lbl: [] for lbl, *_ in POSITIONS} for name, *_ in ENGINES}
errors = []

for rnd in range(ROUNDS):
    print(f"--- round {rnd+1}/{ROUNDS} ---", flush=True)
    for name, exe, gofmt, sentinel, parse in ENGINES:
        p = procs[name]
        for lbl, fen, depth, expected in POSITIONS:
            send(p, "position startpos" if fen is None else f"position fen {fen}")
            t0 = time.perf_counter()
            send(p, gofmt.format(d=depth))
            line = wait_for(p, sentinel)
            t1 = time.perf_counter()
            nodes = parse(line)
            if nodes != expected:
                errors.append(f"{name} {lbl}: got {nodes}, expected {expected}")
            results[name][lbl].append((t1 - t0, nodes))
        print(f"  {name} done", flush=True)

for name in procs:
    try:
        send(procs[name], "quit")
    except Exception:
        pass

print()
if errors:
    print("NODE-COUNT MISMATCHES (comparison invalid for these cells):")
    for e in errors:
        print(" ", e)
else:
    print("All node counts match the reference values for all engines.  OK")

print()
hdr = f"{'Position':28}" + "".join(f"{name:>20}" for name, *_ in ENGINES)
print(hdr)
print("-" * len(hdr))
totals = {name: 0.0 for name, *_ in ENGINES}
total_nodes = 0
for lbl, fen, depth, expected in POSITIONS:
    row = f"{lbl:28}"
    total_nodes += expected
    for name, *_ in ENGINES:
        best = min(t for t, n in results[name][lbl])
        totals[name] += best
        mnps = expected / best / 1e6
        row += f"{mnps:>17.1f} Mn"
    print(row)
print("-" * len(hdr))
row = f"{'SUITE (weighted Mnps)':28}"
for name, *_ in ENGINES:
    row += f"{total_nodes / totals[name] / 1e6:>17.1f} Mn"
print(row)
row = f"{'suite wall time (best, s)':28}"
for name, *_ in ENGINES:
    row += f"{totals[name]:>18.2f} s"
print(row)
