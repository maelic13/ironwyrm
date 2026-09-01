#!/usr/bin/env python3
"""Check GUIDE.md's status board mechanically.

GUIDE is the file that says what to do next, so a wrong checkbox sends the next
session at finished work or hides unfinished work. Three failures are possible
and none is visible by reading:

1. **Sub-item indentation.** A sub-item under a `- ` parent must be indented
   **4** spaces. The parent's content column is 2, and an indented code block
   starts 4 columns past that -- so **6 spaces renders as a code block**, not as
   a nested list. That is exactly what happened when the checkboxes were first
   added, and it looks fine in a diff.

2. **A hanging parent.** If every sub-step of a step is ticked, the parent must
   be ticked too. Leaving it open makes finished work look outstanding.

3. **A missing phase.** GUIDE is the maintainer's week-to-week status board and
   must list EVERY phase, not only the one being worked on. Phases 6-9 were
   dropped during a shortening pass on 2026-08-30 and nobody caught it by
   reading; the maintainer did, weeks later.

The child pattern is checked against the format GUIDE actually uses --
`- [ ] **4.9a.1** ...`, bold and optionally letter-suffixed. The first version
of this checker required a bare `4.9.1`, matched no line in the file, and so
passed vacuously for every edit it existed to guard. The step count in the
success line is there to make that failure mode visible.

Usage:
  python tools/diag/check_guide.py

Exit status is 0 when clean, 1 otherwise, so it can gate a commit.
"""

import pathlib
import re
import sys

ROOT = pathlib.Path(__file__).resolve().parents[2]
GUIDE = ROOT / "GUIDE.md"

# The step number must be the WHOLE bold run. Writing `**4.9 NEXT**` puts the
# marker inside the bold, the number then fails to match, and the step drops
# out of the count silently -- which is how the count went 100 -> 101 when one
# such marker was moved out. Markers go after the bold: `**4.9** NEXT - ...`.
PARENT = re.compile(r"^- \[([ x])\] \*\*(\d+\.\d+[a-z]?)\*\*")
CHILD = re.compile(r"^( *)- \[([ x])\] \*\*(\d+\.\d+[a-z]?\.\d+)\*\*")
STRAY = re.compile(r"^ *- \[[ x]\] \*\*\d+\.\d+[a-z]?(\.\d+)? [^*]")
PHASE = re.compile(r"^## Phase (\d+)")
REQUIRED_PHASES = {4, 5, 6, 7, 8, 9}


def main():
    sys.stdout.reconfigure(encoding="utf-8")
    lines = GUIDE.read_text(encoding="utf-8").splitlines()
    problems = []
    parent = None
    kids = []
    phases = set()
    steps = 0

    def close():
        if parent is not None and kids and all(kids) and not parent[1]:
            problems.append(
                "hanging parent: %s has every sub-step ticked but is not ticked"
                % parent[0]
            )

    for n, line in enumerate(lines, 1):
        ph = PHASE.match(line)
        if ph:
            phases.add(int(ph.group(1)))
        m = PARENT.match(line)
        if m:
            close()
            parent = (m.group(2), m.group(1) == "x")
            kids = []
            steps += 1
            continue
        if STRAY.match(line):
            problems.append(
                "GUIDE.md:%d: text inside the step number's bold run; the "
                "number must be the whole bold (write `**4.9** NEXT - ...`), "
                "or the step drops out of the count silently" % n
            )
        k = CHILD.match(line)
        if k:
            steps += 1
            indent = len(k.group(1))
            if indent != 4:
                problems.append(
                    "GUIDE.md:%d: sub-item %s indented %d spaces, must be 4 "
                    "(6 renders as an indented code block)"
                    % (n, k.group(3), indent)
                )
            if parent is not None:
                kids.append(k.group(2) == "x")
    close()

    missing = sorted(REQUIRED_PHASES - phases)
    if missing:
        problems.append(
            "missing phase heading(s): %s -- GUIDE lists every phase, not only "
            "the active one" % ", ".join("Phase %d" % p for p in missing)
        )
    if steps == 0:
        problems.append(
            "no step bullets matched: the GUIDE format changed and this "
            "checker is now passing vacuously"
        )

    if problems:
        for p in problems:
            sys.stdout.write("  %s\n" % p)
        sys.stdout.write("FAIL: %d problem(s) in GUIDE.md\n" % len(problems))
        return 1
    sys.stdout.write(
        "GUIDE.md consistent: %d steps, phases %s\n"
        % (steps, ", ".join(str(p) for p in sorted(phases)))
    )
    return 0


if __name__ == "__main__":
    sys.exit(main())
