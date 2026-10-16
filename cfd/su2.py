# Euler CFD of the fighter with SU2: one case per (Mach, altitude, angle of
# attack), each in its own directory beside the mesh. The inlets swallow
# MFR of the air their capture meets, held at the static pressure that
# passes it (behind a normal shock, supersonic: pitot inlets), the base
# exhausts a jet at T0_JET;
# forces are the walls' alone, as the drag polar books them (the engine's
# net thrust is the engine's), on the wing's area and mean chord, moments
# about MOMENT_X. Prints CL, CD, Cm per case.
#
# python cfd/su2.py MESH.su2 MACH ALTITUDE ALPHA [ALPHA ...] [--iter N] [--threads N]

import csv
import math
import os
import subprocess
import sys

SU2 = os.path.expanduser("~/.local/opt/su2/bin/SU2_CFD")
REF_AREA, REF_LENGTH, MOMENT_X = 55.37, 8.03, 12.0
MFR, T0_JET, P0_JET = 0.85, 850.0, 1.4


def isa(z):
    """Temperature, K, and pressure, Pa, of the standard atmosphere."""
    if z < 11000:
        t = 288.15 - 0.0065 * z
        return t, 101325 * (t / 288.15) ** 5.2559
    return 216.65, 22632 * math.exp(-(z - 11000) / 6341.6)


def flux(m):
    """Mass flux over the critical, at Mach `m`, isentropic."""
    return m * (1.2 / (1 + 0.2 * m * m)) ** 3


def face_pressure(mach, p0):
    """Static pressure at the inlets' faces that passes MFR of the
    capture, its total pressure `p0` less a normal shock's loss."""
    p0_face = p0
    if mach > 1:
        m2 = mach * mach
        p0_face *= (2.4 * m2 / (0.4 * m2 + 2)) ** 3.5 * (2.4 / (2.8 * m2 - 0.4)) ** 2.5
    want = MFR * flux(mach) * p0 / p0_face
    lo, hi = 0.0, 1.0
    for _ in range(60):
        mid = (lo + hi) / 2
        lo, hi = (mid, hi) if flux(mid) < want else (lo, mid)
    return p0_face / (1 + 0.2 * lo * lo) ** 3.5


def config(mesh, mach, z, alpha, iters, second_order, restart):
    t, p = isa(z)
    p0 = p * (1 + 0.2 * mach * mach) ** 3.5
    return f"""SOLVER= EULER
MATH_PROBLEM= DIRECT
MACH_NUMBER= {mach}
AOA= {alpha}
SIDESLIP_ANGLE= 0.0
FREESTREAM_OPTION= TEMPERATURE_FS
FREESTREAM_PRESSURE= {p:.2f}
FREESTREAM_TEMPERATURE= {t:.3f}
REF_DIMENSIONALIZATION= DIMENSIONAL
REF_ORIGIN_MOMENT_X= {MOMENT_X}
REF_ORIGIN_MOMENT_Y= 0.0
REF_ORIGIN_MOMENT_Z= 0.0
REF_LENGTH= {REF_LENGTH}
REF_AREA= {REF_AREA}
MESH_FILENAME= {mesh}
MESH_FORMAT= SU2
MARKER_EULER= ( wall )
MARKER_FAR= ( farfield )
MARKER_OUTLET= ( inflow, {face_pressure(mach, p0):.1f} )
INLET_TYPE= TOTAL_CONDITIONS
MARKER_INLET= ( exhaust, {T0_JET}, {P0_JET * p0:.1f}, 1.0, 0.0, 0.0 )
MARKER_MONITORING= ( wall )
MARKER_PLOTTING= ( wall )
NUM_METHOD_GRAD= WEIGHTED_LEAST_SQUARES
CONV_NUM_METHOD_FLOW= ROE
MUSCL_FLOW= {"YES" if second_order else "NO"}
SLOPE_LIMITER_FLOW= VENKATAKRISHNAN
VENKAT_LIMITER_COEFF= 0.03
TIME_DISCRE_FLOW= EULER_IMPLICIT
CFL_NUMBER= 1.0
CFL_ADAPT= YES
CFL_ADAPT_PARAM= ( 0.5, 1.05, 0.5, {5.0 if second_order else 10.0} )
LINEAR_SOLVER= FGMRES
LINEAR_SOLVER_PREC= ILU
LINEAR_SOLVER_ERROR= 1E-4
LINEAR_SOLVER_ITER= 10
MGLEVEL= 0
ITER= {iters}
CONV_FIELD= DRAG
CONV_CAUCHY_ELEMS= 150
CONV_CAUCHY_EPS= 1E-5
CONV_STARTITER= 300
SCREEN_OUTPUT= ( INNER_ITER, WALL_TIME, RMS_DENSITY, LIFT, DRAG )
SCREEN_WRT_FREQ_INNER= 50
HISTORY_OUTPUT= ( ITER, RMS_RES, AERO_COEFF )
OUTPUT_FILES= ( RESTART, SURFACE_PARAVIEW )
OUTPUT_WRT_FREQ= 500
RESTART_SOL= {"YES" if restart else "NO"}
SOLUTION_FILENAME= restart.dat
RESTART_FILENAME= restart.dat
"""


def run(mesh, mach, z, alpha, iters, threads, first=400):
    """First order for `first` iterations, then second order from there."""
    case = os.path.join(os.path.dirname(os.path.abspath(mesh)), f"m{mach:.2f}_z{z:.0f}_a{alpha:.1f}")
    os.makedirs(case, exist_ok=True)
    for stage, (n, second, restart) in enumerate(((first, False, False), (iters, True, True))):
        if n <= 0:
            continue
        with open(os.path.join(case, "case.cfg"), "w") as f:
            f.write(config(os.path.abspath(mesh), mach, z, alpha, n, second, restart))
        with open(os.path.join(case, f"log{stage}.txt"), "w") as log:
            subprocess.run([SU2, "-t", str(threads), "case.cfg"], cwd=case, stdout=log, stderr=subprocess.STDOUT)
    with open(os.path.join(case, "history.csv")) as f:
        rows = list(csv.reader(f))
    head = [h.strip().strip('"') for h in rows[0]]
    if rows[-1][0].strip().startswith('"'):
        print(f"Mach {mach:.2f} at {z:5.0f} m, alpha {alpha:4.1f}: diverged, see {case}", flush=True)
        return None
    last = dict(zip(head, [float(v) for v in rows[-1]]))
    print(f"Mach {mach:.2f} at {z:5.0f} m, alpha {alpha:4.1f}: CL {last['CL']:.4f} CD {last['CD']:.5f} "
          f"CMy {last['CMy']:.4f} after {int(last['Inner_Iter'])} iterations, rms rho {last['rms[Rho]']:.2f}",
          flush=True)
    return last


def main():
    a = sys.argv[1:]
    opt = lambda k, d: a[a.index(k) + 1] if k in a else d
    iters, threads, first = int(opt("--iter", 3000)), int(opt("--threads", 14)), int(opt("--first", 400))
    pos = [x for i, x in enumerate(a) if not x.startswith("--") and (i == 0 or not a[i - 1].startswith("--"))]
    mesh, mach, z, alphas = pos[0], float(pos[1]), float(pos[2]), [float(x) for x in pos[3:]]
    for alpha in alphas:
        run(mesh, mach, z, alpha, iters, threads, first)


if __name__ == "__main__":
    main()
