# What cfd/su2.py's cases say, beside a mesh: per Mach number the lift,
# drag and pitching moment at each angle of attack, and from them the lift
# slope, the neutral point, the zero-lift drag as a drag area, D/q m2, and
# the drag due to lift, K in CD = CD0 + K (CL - CL0)^2, by least squares.
#
# python cfd/polar.py RUN_DIR

import csv
import math
import os
import re
import sys

import numpy as np

from su2 import MOMENT_X, REF_AREA, REF_LENGTH


def last(case):
    """The case's final coefficients, None unless its second-order run
    finished."""
    try:
        with open(os.path.join(case, "log1.txt")) as f:
            if "Exit Success" not in f.read():
                return None
    except FileNotFoundError:
        return None
    with open(os.path.join(case, "history.csv")) as f:
        rows = list(csv.reader(f))
    head = [h.strip().strip('"') for h in rows[0]]
    return dict(zip(head, [float(v) for v in rows[-1]]))


def main():
    run = sys.argv[1]
    cases = {}
    for d in os.listdir(run):
        m = re.fullmatch(r"m([\d.]+)_z(\d+)_a([\d.]+)", d)
        if m and (r := last(os.path.join(run, d))):
            cases.setdefault((float(m[1]), int(m[2])), []).append((float(m[3]), r))
    for (mach, z), pts in sorted(cases.items()):
        pts.sort()
        print(f"Mach {mach:.2f} at {z} m")
        for a, r in pts:
            print(f"  alpha {a:4.1f}: CL {r['CL']:7.4f}  CD {r['CD']:.5f}  Cm {r['CMy']:+.4f}  L/D {r['CL'] / r['CD']:5.2f}")
        small = [(a, r) for a, r in pts if a <= 5]
        if len(small) < 2:
            continue
        (a0, r0), (a1, r1) = small[0], small[-1]
        cla = (r1["CL"] - r0["CL"]) / math.radians(a1 - a0)
        x_np = MOMENT_X - (r1["CMy"] - r0["CMy"]) / (r1["CL"] - r0["CL"]) * REF_LENGTH
        line = f"  CL_alpha {cla:.3f}/rad, neutral point {x_np:.2f} m"
        low = [(a, r) for a, r in pts if a <= 12]
        if len(low) >= 3:
            cl = np.array([r["CL"] for _, r in low])
            cd = np.array([r["CD"] for _, r in low])
            k, b, c = np.polyfit(cl, cd, 2)
            cl0 = -b / (2 * k)
            cd0 = c - b * b / (4 * k)
            line += (f", CD0 {cd0:.5f} ({cd0 * REF_AREA:.3f} m2 D/q) at CL {cl0:+.3f}, "
                     f"K {k:.3f} (1/CL_alpha {1 / cla:.3f})")
        print(line)


if __name__ == "__main__":
    main()
