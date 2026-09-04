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

There is a fourth thing that is not a failure but is just as invisible: WHICH
LEAF IS NEXT. The board is 146 lines with 42 ticked, so reading the next few
open items off it by eye is exactly the sort of manual step this file exists to
replace. `--next N` prints them, generated from the board rather than copied
into it, so the queue cannot drift from the checkboxes the way a hand-written
list would.

An ACTIONABLE leaf is an unticked item that nobody else discharges: a sub-step,
or a step that has no sub-steps. A parent with children is a heading, not work.

Usage:
  python tools/diag/check_guide.py
  python tools/diag/check_guide.py --next 8

Exit status is 0 when clean, 1 otherwise, so it can gate a commit. `--next`
does not change it.
"""

import argparse
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
PLAN = ROOT / "PLAN.md"
# Every GUIDE step number must appear somewhere in PLAN. GUIDE and PLAN are
# required to change in the same commit, and three times in one session a
# scripted PLAN edit matched no anchor, reported success, and was committed with
# a GUIDE that had changed -- leaving the two disagreeing with nothing to catch
# it. This is the cheap half of that check: not that the prose agrees, but that
# PLAN has heard of every step GUIDE lists.
STEP_IN_PLAN = re.compile(r"(?<![\d.])%s(?![\d])")


def actionable(lines):
    """The unticked leaves, in board order, with their trailing text.

    Returns (number, text) pairs. A step with sub-steps is a heading and is
    skipped: ticking it is 4.10.10's hanging-parent rule, not work.
    """
    items = []
    for line in lines:
        m = PARENT.match(line)
        if m:
            items.append((0, m.group(2), m.group(1) == "x",
                          line[m.end():].strip(" -—")))
            continue
        k = CHILD.match(line)
        if k:
            items.append((len(k.group(1)), k.group(3), k.group(2) == "x",
                          line[k.end():].strip(" -—")))
    out = []
    for i, (indent, number, ticked, text) in enumerate(items):
        if indent == 0:
            has_children = i + 1 < len(items) and items[i + 1][0] > 0
            if has_children:
                continue
        if not ticked:
            out.append((number, text))
    return out


def main():
    ap = argparse.ArgumentParser(description="Check GUIDE.md's status board.")
    ap.add_argument("--next", type=int, default=0, metavar="N",
                    help="also print the next N actionable leaves, in board "
                         "order, generated from the checkboxes")
    args = ap.parse_args()

    sys.stdout.reconfigure(encoding="utf-8")
    lines = GUIDE.read_text(encoding="utf-8").splitlines()
    problems = []
    parent = None
    kids = []
    phases = set()
    steps = 0
    step_numbers = []

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
            step_numbers.append(m.group(2))
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
            step_numbers.append(k.group(3))
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

    plan_text = PLAN.read_text(encoding="utf-8") if PLAN.is_file() else ""
    if not plan_text:
        problems.append("PLAN.md missing; GUIDE and PLAN must change together")
    else:
        absent = [s for s in step_numbers
                  if not re.search(STEP_IN_PLAN.pattern % re.escape(s), plan_text)]
        if absent:
            problems.append(
                "step(s) in GUIDE that PLAN never mentions: %s -- GUIDE and "
                "PLAN change in the same commit" % ", ".join(absent)
            )

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
    if args.next:
        queue = actionable(lines)
        sys.stdout.write(
            "\n%d actionable leaves open. Next %d, in board order:\n"
            % (len(queue), min(args.next, len(queue)))
        )
        for number, text in queue[:args.next]:
            sys.stdout.write("  %-9s %s\n" % (number, text[:78]))
    return 0


if __name__ == "__main__":
    sys.exit(main())
