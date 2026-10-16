# The engine's nozzle in RANS (SU2, Spalart-Allmaras), axisymmetric: the
# convergent-divergent nozzle hf-engine designs (modeling/turbo.py reads
# it), from a short length of the reheat duct to its exit, at the cycle's
# entry conditions, dry or in reheat. The gas is ideal at the cycle's frozen
# gamma and gas constant; the walls adiabatic. The exit is supersonic, so it
# is the domain's outlet.
#
# Its thrust is the exit plane's momentum and pressure, against the ideal:
# the same flow expanded isentropically to the ambient pressure. Prints the
# gross thrust coefficient (thrust over the actual flow's ideal thrust), the
# discharge coefficient (flow over the ideal throat's), and the kinetic-
# energy efficiency they make, hf-engine's nozzle_eta.
#
# ~/.local/opt/cfd-venv/bin/python cfd/nozzle.py DIR [--wet] [--threads N] [--iter N] [--refine K] [--post]
#
# --refine divides the core's cells by K; --post only reads a finished
# case in DIR.

import csv
import math
import os
import subprocess
import sys

import gmsh

sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "modeling"))
import turbo  # noqa: E402

SU2 = os.path.expanduser("~/.local/opt/su2/bin/SU2_CFD")
# The reheat duct ahead of the nozzle, m; first cell at the wall, m, and
# its growth; cells across the core.
DUCT = 0.3
FIRST, GROWTH, BL = 2.5e-6, 1.18, 0.012
CORE = 0.004


def mesh(air, wet, out, refine=1.0):
    wall = turbo.nozzle_profile(air, wet)
    (x0, r0) = wall[0]
    pts = [(x0 - DUCT, r0)] + wall
    xe, re = pts[-1]
    gmsh.initialize()
    gmsh.option.setNumber("General.Terminal", 0)
    g = gmsh.model.geo
    wp = [g.addPoint(x, r, 0) for x, r in pts]
    axis_out, axis_in = g.addPoint(xe, 0, 0), g.addPoint(x0 - DUCT, 0, 0)
    walls = [g.addLine(a, b) for a, b in zip(wp, wp[1:])]
    outlet = g.addLine(wp[-1], axis_out)
    axis = g.addLine(axis_out, axis_in)
    inlet = g.addLine(axis_in, wp[0])
    loop = g.addCurveLoop([*walls, outlet, axis, inlet])
    surf = g.addPlaneSurface([loop])
    g.synchronize()
    f = gmsh.model.mesh.field
    bl = f.add("BoundaryLayer")
    f.setNumbers(bl, "CurvesList", walls)
    f.setNumbers(bl, "FanPointsList", wp[2:3])
    f.setNumber(bl, "Size", FIRST)
    f.setNumber(bl, "Ratio", GROWTH)
    f.setNumber(bl, "Thickness", BL)
    f.setNumber(bl, "Quads", 1)
    f.setAsBoundaryLayer(bl)
    gmsh.option.setNumber("Mesh.MeshSizeMax", CORE / refine)
    gmsh.option.setNumber("Mesh.Algorithm", 6)
    gmsh.model.addPhysicalGroup(1, walls, name="wall")
    gmsh.model.addPhysicalGroup(1, [inlet], name="inlet")
    gmsh.model.addPhysicalGroup(1, [outlet], name="outlet")
    gmsh.model.addPhysicalGroup(1, [axis], name="axis")
    gmsh.model.addPhysicalGroup(2, [surf], name="fluid")
    gmsh.model.mesh.generate(2)
    n = len(gmsh.model.mesh.getNodes()[0])
    gmsh.write(out)
    gmsh.finalize()
    return n


def config(n, mesh_file, iters):
    """`n` is the nozzle's design dict: its entry flow and ambient."""
    k, R, t0, p0 = n["gamma"], n["r_gas"], n["t0"], n["p0"]
    m_in = 0.2
    t = t0 / (1 + (k - 1) / 2 * m_in ** 2)
    p = p0 * (t / t0) ** (k / (k - 1))
    return f"""SOLVER= RANS
KIND_TURB_MODEL= SA
MATH_PROBLEM= DIRECT
AXISYMMETRIC= YES
FLUID_MODEL= IDEAL_GAS
GAMMA_VALUE= {k:.5f}
GAS_CONSTANT= {R:.4f}
VISCOSITY_MODEL= SUTHERLAND
MU_REF= 1.716E-5
MU_T_REF= 273.15
SUTHERLAND_CONSTANT= 110.4
CONDUCTIVITY_MODEL= CONSTANT_PRANDTL
PRANDTL_LAM= 0.72
PRANDTL_TURB= 0.90
MACH_NUMBER= {m_in}
AOA= 0.0
INIT_OPTION= TD_CONDITIONS
FREESTREAM_OPTION= TEMPERATURE_FS
FREESTREAM_PRESSURE= {p:.1f}
FREESTREAM_TEMPERATURE= {t:.2f}
REF_DIMENSIONALIZATION= DIMENSIONAL
MESH_FILENAME= {mesh_file}
MESH_FORMAT= SU2
MARKER_HEATFLUX= ( wall, 0.0 )
INLET_TYPE= TOTAL_CONDITIONS
MARKER_INLET= ( inlet, {t0:.2f}, {p0:.1f}, 1.0, 0.0, 0.0 )
MARKER_OUTLET= ( outlet, {n["ambient"]:.1f} )
MARKER_SYM= ( axis )
MARKER_PLOTTING= ( outlet )
MARKER_MONITORING= ( wall )
MARKER_ANALYZE= ( inlet, outlet )
MARKER_ANALYZE_AVERAGE= MASSFLUX
NUM_METHOD_GRAD= GREEN_GAUSS
CONV_NUM_METHOD_FLOW= ROE
MUSCL_FLOW= YES
SLOPE_LIMITER_FLOW= VENKATAKRISHNAN
VENKAT_LIMITER_COEFF= 0.05
CONV_NUM_METHOD_TURB= SCALAR_UPWIND
MUSCL_TURB= NO
TIME_DISCRE_FLOW= EULER_IMPLICIT
TIME_DISCRE_TURB= EULER_IMPLICIT
CFL_NUMBER= 1.0
CFL_ADAPT= YES
CFL_ADAPT_PARAM= ( 0.5, 1.1, 1.0, 50.0 )
LINEAR_SOLVER= FGMRES
LINEAR_SOLVER_PREC= ILU
LINEAR_SOLVER_ERROR= 1E-6
LINEAR_SOLVER_ITER= 10
ITER= {iters}
CONV_FIELD= SURFACE_MASSFLOW
CONV_CAUCHY_ELEMS= 200
CONV_CAUCHY_EPS= 1E-6
CONV_STARTITER= 1000
SCREEN_OUTPUT= ( INNER_ITER, RMS_DENSITY, RMS_NU_TILDE, SURFACE_MASSFLOW )
SCREEN_WRT_FREQ_INNER= 100
HISTORY_OUTPUT= ( ITER, RMS_RES, FLOW_COEFF )
OUTPUT_FILES= ( RESTART, SURFACE_CSV, PARAVIEW )
OUTPUT_WRT_FREQ= 1000
"""


def thrust(case, n):
    """Momentum and pressure thrust through the exit plane, N, and its mass
    flow, kg/s, from SU2's surface solution (conserved variables, the
    pressure from the energy): the exit is a line from its lip to the axis,
    each segment a ring of 2 pi r dr."""
    with open(os.path.join(case, "surface.csv")) as f:
        rows = [{k.strip().strip('"'): float(v) for k, v in r.items()} for r in csv.DictReader(f)]
    rows.sort(key=lambda r: r["y"])
    k = n["gamma"]
    force = mdot = 0.0
    for a, b in zip(rows, rows[1:]):
        r, dr = (a["y"] + b["y"]) / 2, b["y"] - a["y"]
        ring = 2 * math.pi * r * dr
        rho, mx, my, e = ((a[q] + b[q]) / 2 for q in ("Density", "Momentum_x", "Momentum_y", "Energy"))
        p = (k - 1) * (e - (mx * mx + my * my) / (2 * rho))
        mdot += mx * ring
        force += (mx * mx / rho + p - n["ambient"]) * ring
    return force, mdot


def ideal(n):
    """The ideal nozzle: its choked flow through the design's throat, and
    the jet speed of full isentropic expansion to ambient."""
    k, R, t0, p0 = n["gamma"], n["r_gas"], n["t0"], n["p0"]
    flux = p0 / math.sqrt(R * t0) * math.sqrt(k) * (2 / (k + 1)) ** ((k + 1) / (2 * (k - 1)))
    v = math.sqrt(2 * k / (k - 1) * R * t0 * (1 - (n["ambient"] / p0) ** ((k - 1) / k)))
    return flux * math.pi * n["throat"] ** 2, v


def main():
    a = sys.argv[1:]
    opt = lambda k, d: a[a.index(k) + 1] if k in a else d
    case, wet = a[0], "--wet" in a
    threads, iters = opt("--threads", "8"), int(opt("--iter", "8000"))
    os.makedirs(case, exist_ok=True)
    air = turbo.load()["air"]
    n = air["nozzle"][1 if wet else 0]
    if "--post" not in a:
        nodes = mesh(air, wet, os.path.join(case, "nozzle.su2"), float(opt("--refine", "1")))
        print(f"NOZZLE mesh {nodes} nodes", flush=True)
        with open(os.path.join(case, "case.cfg"), "w") as f:
            f.write(config(n, "nozzle.su2", iters))
        with open(os.path.join(case, "log.txt"), "w") as log:
            subprocess.run([SU2, "-t", threads, "case.cfg"], cwd=case, stdout=log, stderr=subprocess.STDOUT)
    force, mdot = thrust(case, n)
    m_ideal, v_ideal = ideal(n)
    cfg = force / (mdot * v_ideal)
    print(f"NOZZLE {'reheat' if wet else 'dry'}: thrust {force / 1e3:.2f} kN, flow {mdot:.2f} kg/s "
          f"(cycle {n['m']:.2f}); gross thrust coefficient {cfg:.4f}, discharge coefficient "
          f"{mdot / m_ideal:.4f}; as the cycle's kinetic-energy efficiency {cfg * cfg:.4f}", flush=True)


if __name__ == "__main__":
    main()
