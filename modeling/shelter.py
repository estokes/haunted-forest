# How much of the cosmic-ray dose the courier's ring field turns away from
# its pods (book/ship-systems.md). hf-nozzle's shelter mode gives the open
# fraction of arrival directions by proton rigidity at points along a pod;
# this folds it with the unmodulated interstellar spectrum (between stars
# there is no stellar wind to filter it), weighted by stopping power in
# water, for protons and for nuclei with A/Z = 2 (helium and heavier).

import json
import math
import subprocess
import sys

import nozzle

M_P = 0.938272  # GeV
GROUPS = {"protons": (1, 1), "A/Z=2 nuclei": (2, 4)}


def j_lis(p_gv):
    """Interstellar intensity per GeV of kinetic energy per nucleon, as a
    function of rigidity per nucleon, arbitrary units (Burger & Usoskin)."""
    return p_gv ** -2.78 / (1 + 0.4866 * p_gv ** -2.51)


def stopping(z, beta):
    """Bethe stopping power in water, MeV cm²/g."""
    bg2 = beta * beta / (1 - beta * beta)
    return 0.307 * 0.555 * z * z / (beta * beta) * (math.log(2 * 0.511 * bg2 / 75e-6) - beta * beta)


def open_at(table, r):
    """Open fraction at rigidity r from the (rigidity, open) table, log-linear,
    held flat past the ends."""
    if r <= table[0][0]:
        return table[0][1]
    for (r0, f0), (r1, f1) in zip(table, table[1:]):
        if r <= r1:
            return f0 + (f1 - f0) * math.log(r / r0) / math.log(r1 / r0)
    return table[-1][1]


def dose_open(table, z, a, t_min=0.1, t_max=500.0, n=400):
    """Dose-weighted open fraction for nuclei (z, a), over kinetic energy per
    nucleon t (GeV) on a log grid; same per-nucleon spectrum shape. Below
    t_min they stop in the pod's skin and stores (~8 g/cm² at 0.1 GeV)."""
    num = den = 0.0
    for k in range(n):
        t = t_min * (t_max / t_min) ** ((k + 0.5) / n)
        p_n = math.sqrt(t * (t + 2 * M_P))  # GeV/c per nucleon
        r = a / z * p_n
        beta = p_n / (t + M_P)
        w = j_lis(p_n) * stopping(z, beta) * t  # per log t
        num += w * open_at(table, r)
        den += w
    return num / den


def run(spec):
    out = subprocess.run([nozzle.BIN, "shelter", "-"], input=json.dumps(spec),
                         capture_output=True, text=True, check=True)
    return json.loads(out.stdout)


if __name__ == "__main__":
    r = json.load(open(sys.argv[1])) if len(sys.argv) > 1 else run({"n": 2000})
    gv = r["spec"]["rigidity_gv"]
    print("from plate   " + "  ".join(f"{g}" for g in GROUPS) + "   (fraction of dose that still arrives)")
    for along, row in zip(r["spec"]["along"], r["open"]):
        table = list(zip(gv, row))
        print(f"{along:6.0f} m    " + "  ".join(f"{dose_open(table, z, a):8.2f}    " for z, a in GROUPS.values()))
