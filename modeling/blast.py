# One ICF blast in the cruiser's ring (book/icf-drive.md, The Blast): the
# fraction of its momentum turned astern by the single-particle tracker,
# the thin-shell model and the hybrid model, at three burn-point stand-offs.
#
#   python3 blast.py          single particles and thin shell (a minute)
#   python3 blast.py hybrid   adds the hybrid runs (~25 minutes each)

import json
import math
import subprocess
import sys

import nozzle

RING_R, RING_MA = 100.0, 25.0
STANDOFFS = (60, 100, 150)


def binary(mode, spec):
    out = subprocess.run([nozzle.BIN, mode, "-"], input=json.dumps(spec), capture_output=True, text=True, check=True)
    return json.loads(out.stdout)


def particles(z):
    mu0 = 4e-7 * math.pi
    b0 = mu0 * RING_MA * 1e6 * RING_R ** 2 / (2 * (RING_R ** 2 + z ** 2) ** 1.5)
    return nozzle.evaluate({
        "b0": b0, "gamma_tw": 0.2, "rings": [{"r": RING_R, "z": z}], "com": [0, 0, z],
        "ship": {"pods": {"n": 3, "r": 8, "length": 60, "gap": 10, "phase_deg": 0}},
        "run": {"n_ann": 100000, "emission": {"ions": [
            {"m": 938.272, "charge": 1, "t": 14.7, "per_event": 1},
            {"m": 3727.38, "charge": 2, "t": 3.6, "per_event": 1}]}}})


def ring(z):
    return [{"r": RING_R, "z": z, "ma": RING_MA}]


if __name__ == "__main__":
    print("stand-off  particles  thin shell" + ("  hybrid  fwd mass  fwd energy" if "hybrid" in sys.argv else ""))
    for z in STANDOFFS:
        p = particles(z)["collimation"]
        s = binary("shell", {"rings": ring(z), "nodes": 121, "t_end": 3e-3})["collimation"]
        line = f"{z:6} m  {p:9.3f}  {s:10.3f}"
        if "hybrid" in sys.argv:
            h = binary("hybrid", {"rings": ring(z), "particles": 100000, "t_end": 1.2e-3})
            line += f"  {h['collimation']:6.3f}  {h['forward_mass']:8.3f}  {h['forward_energy']:10.3f}"
        print(line, flush=True)
