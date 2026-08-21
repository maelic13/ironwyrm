#!/usr/bin/env python3
"""Check GUIDE.md's status checkboxes mechanically.

GUIDE is the file that says what to do next, so a wrong checkbox sends the next
session at finished work or hides unfinished work. Two failures are possible and
neither is visible by reading:

1. **Sub-item indentation.** A sub-item under a `- ` parent must be indented
   **4** spaces. The parent's content column is 2, and an indented code block
   starts 4 columns past that -- so **6 spaces renders as a code block**, not as
   a nested list. That is exactly what happened when the checkboxes were first
   added, and it looks fine in a diff.

2. **A hanging parent.** If every sub-step of a step is ticked, the parent must
   be ticked too. Leaving it open makes finished work look outstanding.

Usage:
  python tools/diag/check_guide.py

Exit status is 0 when clean, 1 otherwise, so it can gate a commit.
"""

import pathlib
import re
import sys

ROOT = pathlib.Path(__file__).resolve().parents[2]
GUIDE = ROOT / "GUIDE.md"

PARENT = re.compile(r"^- \[([ x])\] \*\*(\d+\.\d+[a-z]?)")
CHILD = re.compile(r"^( *)- \[([ x])\] (\d+\.\d+\.\d+)")


def main():
    sys.stdout.reconfigure(encoding="utf-8")
    lines = GUIDE.read_text(encoding="utf-8").splitlines()
    problems = []
    parent = None
    kids = []

    def close():
        if parent is not None and kids and all(kids) and not parent[1]:
            problems.append(
                "hanging parent: %s has every sub-step ticked but is not ticked"
                % parent[0]
            )

    for n, line in enumerate(lines, 1):
        m = PARENT.match(line)
        if m:
            close()
            parent = (m.group(2), m.group(1) == "x")
            kids = []
            continue
        k = CHILD.match(line)
        if k:
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

    if problems:
        for p in problems:
            sys.stdout.write("  %s\n" % p)
        sys.stdout.write("FAIL: %d problem(s) in GUIDE.md\n" % len(problems))
        return 1
    sys.stdout.write("GUIDE.md checkboxes consistent\n")
    return 0


if __name__ == "__main__":
    sys.exit(main())
