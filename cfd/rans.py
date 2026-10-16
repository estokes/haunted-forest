# Viscous CFD of a part of the fighter (cfd/export.py --parts), with SU2's
# RANS solver resolved to the wall: a boundary-layer mesh of prisms
# extruded from the part's surface, `--layers` of them from a first cell
# of `--first` m growing by `--growth`, tetrahedra beyond to a far-field
# box five lengths out, fine for half a metre round the part where its
# vortices are. Its runs are fully turbulent: Spalart-Allmaras, negative
# and with the rotation correction for vortex-dominated flow, or
# Menter's SST, to bracket the turbulence model; or inviscid, Euler on the
# same mesh, so that the two differ only by viscosity. Coefficients on the
# wing's area and mean chord, moments about x = 12 m, as cfd/su2.py has
# them, so the two compare directly.
#
# ~/.local/opt/cfd-venv/bin/python cfd/rans.py --mesh IN.stl OUT.su2 [--size 0.06] [--layers 40] [--first 3e-6] [--growth 1.25] [--surfdist 0.001]
# python cfd/rans.py MESH.su2 MACH ALTITUDE ALPHA [ALPHA ...] [--turb SA|SST|EULER] [--iter N] [--threads N]

import math
import os
import subprocess
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import su2  # noqa: E402

# How far round the part, m, the cells stay at the surface's size.
NEAR = 0.5


def opt(a, name, default):
    return a[a.index(name) + 1] if name in a else default


def mesh(stl, out, size, layers, first, growth, surfdist):
    import gmsh
    import mesh as surface_mesh

    clean = os.path.splitext(out)[0] + "-surface.stl"
    surface_mesh.remesh(stl, size, clean, surfdist)
    gmsh.initialize()
    gmsh.option.setNumber("General.Terminal", 1)
    # The first cell is microns thick in a box tens of metres across: the
    # default tolerance, relative to the box, would merge its nodes.
    gmsh.option.setNumber("Geometry.Tolerance", 1e-13)
    gmsh.merge(clean)
    gmsh.model.mesh.removeDuplicateNodes()
    gmsh.model.mesh.classifySurfaces(math.radians(40), True, False, math.pi)
    walls = gmsh.model.getEntities(2)
    heights, h = [], 0.0
    for k in range(layers):
        h += first * growth ** k
        heights.append(h)
    gmsh.option.setNumber("Geometry.ExtrudeReturnLateralEntities", 0)
    ext = gmsh.model.geo.extrudeBoundaryLayer(walls, [1] * layers, heights, True)
    top = [t for d, t in ext if d == 2]
    layer_vols = [t for d, t in ext if d == 3]
    gmsh.model.geo.synchronize()
    boxes = [gmsh.model.getBoundingBox(d, t) for d, t in walls]
    x0, x1 = min(b[0] for b in boxes), max(b[3] for b in boxes)
    half, cx = 5 * (x1 - x0), (x0 + x1) / 2
    g = gmsh.model.geo
    corners = [g.addPoint(cx + sx * half, sy * half, sz * half) for sx in (-1, 1) for sy in (-1, 1) for sz in (-1, 1)]
    lines, faces = {}, []
    for f in [(0, 1, 3, 2), (4, 6, 7, 5), (0, 4, 5, 1), (2, 3, 7, 6), (0, 2, 6, 4), (1, 5, 7, 3)]:
        edges = []
        for a, b in zip(f, f[1:] + f[:1]):
            key = (min(a, b), max(a, b))
            if key not in lines:
                lines[key] = g.addLine(corners[key[0]], corners[key[1]])
            edges.append(lines[key] if a < b else -lines[key])
        faces.append(g.addPlaneSurface([g.addCurveLoop(edges)]))
    vol = g.addVolume([g.addSurfaceLoop(faces), g.addSurfaceLoop(top)])
    g.synchronize()
    gmsh.model.addPhysicalGroup(2, [t for _, t in walls], name="wall")
    gmsh.model.addPhysicalGroup(2, faces, name="farfield")
    gmsh.model.addPhysicalGroup(3, layer_vols + [vol], name="fluid")
    f = gmsh.model.mesh.field
    d = f.add("Distance")
    f.setNumbers(d, "SurfacesList", [t for _, t in walls])
    t = f.add("Threshold")
    f.setNumber(t, "InField", d)
    f.setNumber(t, "SizeMin", size)
    f.setNumber(t, "SizeMax", half / 8)
    f.setNumber(t, "DistMin", NEAR)
    f.setNumber(t, "DistMax", half)
    f.setAsBackgroundMesh(t)
    for k, v in (("MeshSizeExtendFromBoundary", 0), ("MeshSizeFromPoints", 0), ("MeshSizeFromCurvature", 0),
                 ("Algorithm3D", 1), ("SaveAll", 0)):
        gmsh.option.setNumber(f"Mesh.{k}", v)
    gmsh.option.setNumber("General.NumThreads", os.cpu_count())
    gmsh.model.mesh.generate(3)
    types, tags, _ = gmsh.model.mesh.getElements(3)
    cells = {gmsh.model.mesh.getElementProperties(ty)[0]: len(tg) for ty, tg in zip(types, tags)}
    print("RANS mesh", cells, "nodes", len(gmsh.model.mesh.getNodes()[0]), "layers to", round(h, 4), "m", flush=True)
    gmsh.write(out)
    gmsh.finalize()


def config(mesh_file, mach, z, alpha, iters, turb, second_order, restart):
    t, p = su2.isa(z)
    model = {"SA": "SOLVER= RANS\nKIND_TURB_MODEL= SA\nSA_OPTIONS= NEGATIVE, WITHFT2, ROTATION",
             "SST": "SOLVER= RANS\nKIND_TURB_MODEL= SST\nSST_OPTIONS= V2003m",
             "EULER": "SOLVER= EULER"}[turb]
    wall = "MARKER_EULER= ( wall )" if turb == "EULER" else "MARKER_HEATFLUX= ( wall, 0.0 )"
    return f"""{model}
MATH_PROBLEM= DIRECT
INIT_OPTION= TD_CONDITIONS
FREESTREAM_OPTION= TEMPERATURE_FS
MACH_NUMBER= {mach}
AOA= {alpha}
SIDESLIP_ANGLE= 0.0
FREESTREAM_PRESSURE= {p:.2f}
FREESTREAM_TEMPERATURE= {t:.3f}
REF_DIMENSIONALIZATION= DIMENSIONAL
FLUID_MODEL= STANDARD_AIR
VISCOSITY_MODEL= SUTHERLAND
CONDUCTIVITY_MODEL= CONSTANT_PRANDTL
REF_ORIGIN_MOMENT_X= {su2.MOMENT_X}
REF_ORIGIN_MOMENT_Y= 0.0
REF_ORIGIN_MOMENT_Z= 0.0
REF_LENGTH= {su2.REF_LENGTH}
REF_AREA= {su2.REF_AREA}
MESH_FILENAME= {mesh_file}
MESH_FORMAT= SU2
{wall}
MARKER_FAR= ( farfield )
MARKER_MONITORING= ( wall )
MARKER_PLOTTING= ( wall )
NUM_METHOD_GRAD= GREEN_GAUSS
CONV_NUM_METHOD_FLOW= ROE
MUSCL_FLOW= {"YES" if second_order else "NO"}
SLOPE_LIMITER_FLOW= VENKATAKRISHNAN
VENKAT_LIMITER_COEFF= 0.05
CONV_NUM_METHOD_TURB= SCALAR_UPWIND
MUSCL_TURB= NO
TIME_DISCRE_FLOW= EULER_IMPLICIT
TIME_DISCRE_TURB= EULER_IMPLICIT
CFL_NUMBER= 1.0
CFL_ADAPT= YES
CFL_ADAPT_PARAM= ( 0.5, 1.05, 0.5, {20.0 if second_order else 40.0} )
LINEAR_SOLVER= FGMRES
LINEAR_SOLVER_PREC= ILU
LINEAR_SOLVER_ERROR= 1E-4
LINEAR_SOLVER_ITER= 15
MGLEVEL= 0
ITER= {iters}
CONV_FIELD= ( LIFT, DRAG )
CONV_CAUCHY_ELEMS= 200
CONV_CAUCHY_EPS= 1E-5
CONV_STARTITER= 500
SCREEN_OUTPUT= ( INNER_ITER, WALL_TIME, RMS_DENSITY, LIFT, DRAG )
SCREEN_WRT_FREQ_INNER= 50
HISTORY_OUTPUT= ( ITER, RMS_RES, AERO_COEFF )
OUTPUT_FILES= ( RESTART, SURFACE_PARAVIEW )
OUTPUT_WRT_FREQ= 500
RESTART_SOL= {"YES" if restart else "NO"}
SOLUTION_FILENAME= restart.dat
RESTART_FILENAME= restart.dat
"""


def run(mesh_file, mach, z, alpha, iters, threads, turb, first=600):
    """First order for `first` iterations, then second order from there,
    in a case directory beside the mesh named for its turbulence model."""
    case = os.path.join(os.path.dirname(os.path.abspath(mesh_file)),
                        f"{turb.lower()}_m{mach:.2f}_z{z:.0f}_a{alpha:.1f}")
    os.makedirs(case, exist_ok=True)
    for stage, (n, second, restart) in enumerate(((first, False, False), (iters, True, True))):
        with open(os.path.join(case, "case.cfg"), "w") as f:
            f.write(config(os.path.abspath(mesh_file), mach, z, alpha, n, turb, second, restart))
        with open(os.path.join(case, f"log{stage}.txt"), "w") as log:
            subprocess.run([su2.SU2, "-t", str(threads), "case.cfg"], cwd=case, stdout=log, stderr=subprocess.STDOUT)
    print(f"RANS {turb} Mach {mach:.2f} at {z:5.0f} m, alpha {alpha:4.1f}: {case}", flush=True)


def main():
    a = sys.argv[1:]
    if a[0] == "--mesh":
        mesh(a[1], a[2], float(opt(a, "--size", 0.06)), int(opt(a, "--layers", 40)),
             float(opt(a, "--first", 3e-6)), float(opt(a, "--growth", 1.25)), float(opt(a, "--surfdist", 0.001)))
        return
    named = {"--turb", "--iter", "--threads"}
    pos = [x for i, x in enumerate(a) if not x.startswith("--") and (i == 0 or a[i - 1] not in named)]
    mesh_file, mach, z = pos[0], float(pos[1]), float(pos[2])
    for alpha in (float(x) for x in pos[3:]):
        run(mesh_file, mach, z, alpha, int(opt(a, "--iter", 6000)), int(opt(a, "--threads", 14)), opt(a, "--turb", "SA"))


if __name__ == "__main__":
    main()
