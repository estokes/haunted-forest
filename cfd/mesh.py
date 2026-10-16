# The air round the fighter, for SU2: cfd/export.py's surface remeshed to
# even triangles of `--body` metres, its edges sharper than 30° kept (the
# booleans that made it leave slivers no mesher can use), then split into
# patches at those edges; inside a far-field box `--far` body lengths out,
# tetrahedra growing away from the body, finer near its edges.
# Boundaries: wall, inflow (the inlets' capture faces), exhaust (the base
# at the tail), farfield.
#
# ~/.local/opt/cfd-venv/bin/python cfd/mesh.py IN.stl OUT.su2 [--body 0.1] [--edge 0.05] [--far 5] [--near 0.2] [--surfdist 0.005]
#
# --near is how far, m, the body's cell size holds before the cells grow;
# --surfdist how far, m, the remeshed surface may stand off the original.

import math
import os
import sys

import gmsh
import pymeshlab


def opt(name, default):
    a = sys.argv
    return float(a[a.index(name) + 1]) if name in a else default


def remesh(stl, size, out, surfdist):
    """Even triangles of `size`, finer where the surface curves, sharp
    edges kept, within `surfdist` of the surface."""
    ms = pymeshlab.MeshSet()
    ms.load_new_mesh(stl)
    ms.meshing_merge_close_vertices(threshold=pymeshlab.PureValue(1e-5))
    ms.meshing_remove_duplicate_faces()
    ms.meshing_remove_null_faces()
    ms.meshing_isotropic_explicit_remeshing(iterations=12, targetlen=pymeshlab.PureValue(size), featuredeg=30, adaptive=True,
                                            checksurfdist=True, maxsurfdist=pymeshlab.PureValue(surfdist))
    ms.save_current_mesh(out, binary=False)
    print("REMESH faces", ms.current_mesh().face_number())


def main():
    stl, out = sys.argv[1], sys.argv[2]
    body, edge, far, near = opt("--body", 0.1), opt("--edge", 0.05), opt("--far", 5.0), opt("--near", 0.2)
    clean = os.path.splitext(out)[0] + "-surface.stl"
    remesh(stl, body, clean, opt("--surfdist", 0.005))
    gmsh.initialize()
    gmsh.option.setNumber("General.Terminal", 1)
    gmsh.merge(clean)
    gmsh.model.mesh.removeDuplicateNodes()
    gmsh.model.mesh.classifySurfaces(math.radians(30), True, False, math.radians(180))
    surfaces = [t for _, t in gmsh.model.getEntities(2)]

    xs = [gmsh.model.getBoundingBox(2, t) for t in surfaces]
    x_min, x_max = min(b[0] for b in xs), max(b[3] for b in xs)
    length = x_max - x_min
    mouth = min(b[0] for b in xs if b[3] - b[0] < 0.02 and b[0] > x_min + 0.2 * length)

    def flat_at(x, b):
        return b[3] - b[0] < 0.02 and abs(b[0] - x) < 0.02

    groups = {"inflow": [], "exhaust": [], "wall": []}
    for t, b in zip(surfaces, xs):
        key = "inflow" if flat_at(mouth, b) else "exhaust" if flat_at(x_max, b) else "wall"
        groups[key].append(t)

    # The far field: a box `far` lengths out, the body centred in it.
    g = gmsh.model.geo
    cx = (x_min + x_max) / 2
    half = far * length
    corners = [g.addPoint(cx + sx * half, sy * half, sz * half)
               for sx in (-1, 1) for sy in (-1, 1) for sz in (-1, 1)]
    faces = [(0, 1, 3, 2), (4, 6, 7, 5), (0, 4, 5, 1), (2, 3, 7, 6), (0, 2, 6, 4), (1, 5, 7, 3)]
    lines, loops = {}, []
    for f in faces:
        edges = []
        for a, b in zip(f, f[1:] + f[:1]):
            key = (min(a, b), max(a, b))
            if key not in lines:
                lines[key] = g.addLine(corners[key[0]], corners[key[1]])
            edges.append(lines[key] if a < b else -lines[key])
        loops.append(g.addPlaneSurface([g.addCurveLoop(edges)]))
    g.synchronize()
    volume = gmsh.model.addDiscreteEntity(3, -1, loops + surfaces)

    for name, tags in groups.items():
        gmsh.model.addPhysicalGroup(2, tags, name=name)
    gmsh.model.addPhysicalGroup(2, loops, name="farfield")
    gmsh.model.addPhysicalGroup(3, [volume], name="fluid")

    # Sizes: fine on the body and finer along its edges, growing outward.
    curves = [t for _, t in gmsh.model.getBoundary([(2, t) for t in surfaces], oriented=False)]
    f = gmsh.model.mesh.field
    d_body = f.add("Distance")
    f.setNumbers(d_body, "SurfacesList", surfaces)
    t_body = f.add("Threshold")
    f.setNumber(t_body, "InField", d_body)
    f.setNumber(t_body, "SizeMin", body)
    f.setNumber(t_body, "SizeMax", half / 8)
    f.setNumber(t_body, "DistMin", near)
    f.setNumber(t_body, "DistMax", half)
    d_edge = f.add("Distance")
    f.setNumbers(d_edge, "CurvesList", sorted(set(abs(c) for c in curves)))
    f.setNumber(d_edge, "Sampling", 200)
    t_edge = f.add("Threshold")
    f.setNumber(t_edge, "InField", d_edge)
    f.setNumber(t_edge, "SizeMin", edge)
    f.setNumber(t_edge, "SizeMax", half / 8)
    f.setNumber(t_edge, "DistMin", 0.05)
    f.setNumber(t_edge, "DistMax", 3.0)
    smallest = f.add("Min")
    f.setNumbers(smallest, "FieldsList", [t_body, t_edge])
    f.setAsBackgroundMesh(smallest)
    gmsh.option.setNumber("Mesh.MeshSizeExtendFromBoundary", 0)
    gmsh.option.setNumber("Mesh.MeshSizeFromPoints", 0)
    gmsh.option.setNumber("Mesh.MeshSizeFromCurvature", 0)
    gmsh.option.setNumber("Mesh.Algorithm", 6)
    gmsh.option.setNumber("Mesh.Algorithm3D", 10)
    gmsh.option.setNumber("General.NumThreads", 14)

    gmsh.model.mesh.generate(3)
    gmsh.model.mesh.optimize("Netgen")
    print("MESH", {k: len(v) for k, v in groups.items()}, "nodes", len(gmsh.model.mesh.getNodes()[0]))
    gmsh.option.setNumber("Mesh.SaveAll", 0)
    gmsh.write(out)
    gmsh.finalize()


if __name__ == "__main__":
    main()
