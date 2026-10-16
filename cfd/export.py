# The fighter's outer surface for CFD (cfd/mesh.py meshes the air round it):
# modeling/fighter.py's airframe and canards made one closed surface, and
# written as ASCII STL in three solids: the walls; the inlets' capture
# faces, where the engine takes its air; and the base at the tail, where
# it exhausts. Beside it, OUT.json: the lifting surfaces' planforms and the
# centre of gravity, for cfd/vlm.py.
#
# Headless:  blender -b --factory-startup --python cfd/export.py -- OUT.stl

import json
import os
import sys

import bmesh
import bpy

sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "modeling"))
import fighter  # noqa: E402


def faces_by_group(obj):
    """Triangles of `obj`, world coordinates, grouped by what the flow
    sees there."""
    bm = bmesh.new()
    bm.from_mesh(obj.data)
    bm.transform(obj.matrix_world)
    bmesh.ops.triangulate(bm, faces=bm.faces[:])
    mouth = fighter.INLET["mouth_x"]
    tail = max(v.co.x for v in bm.verts)
    groups = {"wall": [], "inflow": [], "exhaust": []}
    for f in bm.faces:
        c, n = f.calc_center_median(), f.normal
        if abs(c.x - mouth) < 0.01 and n.x < -0.9:
            key = "inflow"
        elif c.x > tail - 0.01 and n.x > 0.9:
            key = "exhaust"
        else:
            key = "wall"
        groups[key].append((tuple(n), [tuple(v.co) for v in f.verts]))
    bm.free()
    return groups


def write_stl(path, groups):
    with open(path, "w") as out:
        for name, tris in groups.items():
            out.write(f"solid {name}\n")
            for n, vs in tris:
                out.write(f"facet normal {n[0]:.6e} {n[1]:.6e} {n[2]:.6e}\n outer loop\n")
                for v in vs:
                    out.write(f"  vertex {v[0]:.6e} {v[1]:.6e} {v[2]:.6e}\n")
                out.write(" endloop\nendfacet\n")
            out.write(f"endsolid {name}\n")


def main():
    path = sys.argv[sys.argv.index("--") + 1]
    r = fighter.build(1.0)
    with open(os.path.splitext(path)[0] + ".json", "w") as f:
        json.dump({"wing": fighter.WING, "canard": fighter.CANARD, "fin": fighter.FIN,
                   "cg": (r["cg_empty"], r["cg_full"]), "mac": r["mac"], "ac_subsonic": r["ac_subsonic"]}, f)
    parts = [bpy.data.objects["airframe"]] + [o for o in bpy.data.objects if o.name.startswith("canard_")]
    whole = fighter.union("cfd_body", parts, bpy.context.scene.collection, fighter.C_SKIN, 1.0)
    bm = bmesh.new()
    bm.from_mesh(whole.data)
    print("CFD body: volume", round(fighter.volume(whole)[0], 2), "m3, non-manifold edges",
          sum(1 for e in bm.edges if not e.is_manifold))
    bm.free()
    groups = faces_by_group(whole)
    print("CFD faces:", {k: len(v) for k, v in groups.items()})
    write_stl(path, groups)


if __name__ == "__main__":
    main()
