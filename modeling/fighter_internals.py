# The fighter's insides: modeling/fighter.py's airframe, its equipment
# blocks replaced where they matter by the parts themselves: the engine as
# hf-engine designs it (modeling/engine.py), cut away, and its turbopump
# beside it; the lift fan's duct and counter-rotating rotors; the four
# missiles in their bays; the main gear's wheels, stowed flat; and the main
# runs between them: the power bus from the generator through the power
# electronics to the motors, slurry from the tanks to the turbopump, the
# turbopump's exhaust to the combustor.
#
# It checks what fits: every pair of parts whose surfaces cross, and every
# part's share outside the skin.
#
# Headless:  blender -b --factory-startup --python modeling/fighter_internals.py -- [--render DIR] [--save FILE]

import math
import os
import sys

import bmesh
import bpy
from mathutils import Vector
from mathutils.bvhtree import BVHTree

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import engine  # noqa: E402
import fighter  # noqa: E402
import turbo  # noqa: E402

# Lift fan: rotors, blades per rotor, hub over tip, blade chord over span.
LIFT_ROTORS = 2
LIFT_BLADES = (24, 22)
LIFT_HUB = 0.40
# Missiles: diameter, length, nose length; two side by side in each bay.
MISSILE = (0.18, 2.40, 0.45)
# Main gear: tyre diameter and width, for 21 t on unprepared ground, stowed
# lying flat in its bay.
TYRE = (0.70, 0.24)
# Routes: tube diameters, m.
BUS, FEED, HOT = 0.06, 0.05, 0.27

# What each part joins, so that its touching them is not a clash: the
# runs their ends, the bus's branches each other, the ducts the fan face
# and each other where they merge, the legs their tyres.
BUS_RUNS = ("bus_generator", "bus_engine", "bus_lift_fan", "bus_fel")
JOINS = {"bus_generator": ("turbopump", "power_electronics", *BUS_RUNS), "bus_engine": ("power_electronics", "engine", *BUS_RUNS),
         "bus_lift_fan": ("power_electronics", "lift_fan", *BUS_RUNS), "bus_fel": ("power_electronics", "fel", *BUS_RUNS),
         "feed": ("turbopump",), "tp_exhaust": ("turbopump", "engine"), "duct_-1": ("engine", "duct_1"),
         "duct_1": ("engine",), "leg_l": ("tyre_l",), "leg_r": ("tyre_r",)}
# Parts meant to meet the skin: the lift fan's faces are its doors, the
# ducts open at their captures.
SKIN_PARTS = ("lift_fan", "duct_-1", "duct_1")

C_ROUTE = {"bus": (0.95, 0.85, 0.20), "feed": (0.30, 0.55, 0.95), "hot": (0.95, 0.35, 0.20)}
C_MISSILE = (0.85, 0.85, 0.80)
C_TYRE = (0.12, 0.12, 0.12)


def tree_of(o):
    """A BVH of any object's evaluated surface, in world coordinates."""
    dg = bpy.context.evaluated_depsgraph_get()
    ev = o.evaluated_get(dg)
    mesh = ev.to_mesh()
    bm = bmesh.new()
    bm.from_mesh(mesh)
    bm.transform(o.matrix_world)
    t = BVHTree.FromBMesh(bm)
    verts = [v.co.copy() for v in bm.verts]
    bm.free()
    ev.to_mesh_clear()
    return t, verts


def tube(name, points, d, coll, color):
    curve = bpy.data.curves.new(name, "CURVE")
    curve.dimensions = "3D"
    curve.bevel_depth = d / 2
    sp = curve.splines.new("POLY")
    sp.points.add(len(points) - 1)
    for p, q in zip(sp.points, points):
        p.co = (*q, 1)
    o = bpy.data.objects.new(name, curve)
    coll.objects.link(o)
    o.color = (*color, 1)
    return o


def centre(name):
    kind, size, c, *_ = fighter.EQUIPMENT[name]
    return Vector(c), kind, size


def place_engine(coll, data):
    """The engine cut away at its place, and its turbopump with the
    turbine's end at the combustor; their equipment blocks hidden."""
    air, tp = data["air"], data["turbopump"]
    b = engine.Builder(coll, cut=True)
    b.engine(air, nozzle=False)
    engine.move(b.objects, (fighter.ENGINE_FACE_X, 0, -0.05))
    c, _, (d, length) = centre("turbopump")
    objs, _ = b.turbopump(tp)
    engine.move(objs, (c.x + length / 2, c.y, c.z))
    for name in ("engine", "turbopump"):
        bpy.data.objects[name].hide_render = True


def lift_fan(coll):
    """The lift fan's duct, hub and two counter-rotating rotors, about its
    vertical axis; built along X and turned up."""
    c, _, (d, h) = centre("lift_fan")
    r_tip = d / 2 - 0.03
    r_hub = LIFT_HUB * r_tip
    b = engine.Builder(coll, cut=False)
    b.lathe("lift_duct", [(-h / 2, r_tip + 0.03), (h / 2, r_tip + 0.03)], engine.C_CASE)
    b.lathe("lift_hub", [(-h / 2, 0), (-0.3 * h, r_hub), (0.3 * h, r_hub), (h / 2, 0)], engine.C_MOTOR)
    chord = 0.6 * (r_tip - r_hub)
    for k in range(LIFT_ROTORS):
        x = -0.15 * h + 0.3 * h * k
        spin = 1 if k == 0 else -1
        row = {"spin": spin, "x_le": x - chord / 2, "x_te": x + chord / 2, "hub": (r_hub, r_hub), "tip": (r_tip, r_tip),
               "chord": chord, "sections": [{"r": r_hub, "metal_in": 35, "metal_out": 10, "tc": 0.08},
                                            {"r": r_tip, "metal_in": 62, "metal_out": 52, "tc": 0.03}]}
        for i in range(LIFT_BLADES[k]):
            b.loft(f"lift_blade_{k}_{i}", turbo.blade(row, 2 * math.pi * i / LIFT_BLADES[k]), engine.C_FAN)
    for o in b.objects:
        o.rotation_euler = (0, -math.pi / 2, 0)
        o.location = c
    bpy.data.objects["lift_fan"].hide_render = True
    return b.objects


def missiles(coll):
    """Two missiles in each bay, side by side, tails aft."""
    out = []
    d, length, nose = MISSILE
    r = d / 2
    profile = [(0, 0)] + [(nose * (1 - math.cos(math.pi / 2 * i / 8)), r * math.sin(math.pi / 2 * i / 8))
                          for i in range(1, 9)] + [(length, r), (length, 0)]
    for side in ("l", "r"):
        c, _, (dx, dy, dz) = centre(f"bay_{side}")
        for k in (-1, 1):
            y = c.y + k * dy / 4
            o = fighter.lathe(f"missile_{side}{k}", [(c.x - length / 2 + x, rr) for x, rr in profile], coll, C_MISSILE,
                              centre=(y, c.z))
            out.append(o)
        bpy.data.objects[f"bay_{side}"].hide_render = True
    return out


def gear(coll):
    """Each main gear's tyre, stowed lying flat, and its leg forward of it."""
    out = []
    for side in ("l", "r"):
        c, _, (dx, dy, dz) = centre(f"main_gear_{side}")
        x = c.x + dx / 2 - TYRE[0] / 2
        out.append(fighter.cylinder(f"tyre_{side}", "z", TYRE[0], TYRE[1], (x, c.y, c.z), coll, C_TYRE))
        out.append(fighter.cylinder(f"leg_{side}", "x", 0.12, dx - TYRE[0], (c.x - TYRE[0] / 2, c.y, c.z), coll,
                                    engine.C_CASE, k=12))
        bpy.data.objects[f"main_gear_{side}"].hide_render = True
    return out


def routes(coll, data):
    """The power bus, the slurry feed and the turbopump's exhaust."""
    air = data["air"]
    face = fighter.ENGINE_FACE_X
    tp, _, (tp_d, tp_len) = centre("turbopump")
    pe, _, _ = centre("power_electronics")
    fan, _, _ = centre("lift_fan")
    fel, _, _ = centre("fel")
    gen_x = tp.x - tp_len / 2 + 0.6
    over = air["rows"][0]["tip"][0] + 0.10
    side = -0.85
    out = [
        tube("bus_generator", [(gen_x, tp.y, tp.z + tp_d / 2), (gen_x, tp.y * 0.6, over), (gen_x, -0.4, over),
                               (pe.x + 0.6, pe.y, pe.z)], BUS, coll, C_ROUTE["bus"]),
        tube("bus_engine", [(pe.x + 0.6, pe.y, pe.z - 0.1), (face + 0.1, -0.45, -0.05)], BUS, coll, C_ROUTE["bus"]),
        tube("bus_lift_fan", [(pe.x - 0.6, pe.y, pe.z), (pe.x - 0.8, side, 0.45), (fan.x + 0.9, side, 0.45),
                              (fan.x + 0.78, side * 0.7, fan.z)], BUS, coll, C_ROUTE["bus"]),
        tube("bus_fel", [(pe.x - 0.6, pe.y, pe.z + 0.1), (fel.x + 1.6, -0.3, fel.z - 0.1)], BUS, coll, C_ROUTE["bus"]),
        tube("feed", [(13.6, 0.65, -0.45), (tp.x - tp_len / 2 - 0.15, tp.y, tp.z - 0.1), (tp.x - tp_len / 2, tp.y, tp.z)],
             FEED, coll, C_ROUTE["feed"]),
    ]
    c = air["combustor"]
    exhaust = (tp.x + tp_len / 2, tp.y, tp.z)
    out.append(tube("tp_exhaust", [exhaust, (exhaust[0] + 0.1, tp.y * 0.5, tp.z * 0.5),
                                   (face + c["x0"] + 0.05, 0.0, -0.05 + c["r_out"])], HOT, coll, C_ROUTE["hot"]))
    return out


def check(parts, skin):
    """Pairs of parts whose surfaces cross, and parts' shares outside the
    skin."""
    trees = {o.name: tree_of(o) for o in parts}
    skin_tree, _ = tree_of(skin)
    names = sorted(trees)
    clashes = []
    for i, a in enumerate(names):
        for b in names[i + 1:]:
            joined = b in JOINS.get(a, ()) or a in JOINS.get(b, ())
            if not joined and trees[a][0].overlap(trees[b][0]):
                clashes.append((a, b))
    outside = {}
    for name, (_, verts) in trees.items():
        share = sum(1 for v in verts[:: max(1, len(verts) // 200)] if not fighter.inside(skin_tree, v))
        share /= len(verts[:: max(1, len(verts) // 200)])
        if share > 0.005 and name not in SKIN_PARTS:
            outside[name] = round(100 * share)
    return clashes, outside


def render(directory, length):
    scene = bpy.context.scene
    scene.render.engine = "BLENDER_WORKBENCH"
    scene.display.shading.color_type = "OBJECT"
    scene.display.shading.show_cavity = True
    scene.render.resolution_x, scene.render.resolution_y = 2000, 1000
    for o in bpy.data.objects:
        if o.name.startswith("tank_"):
            o.hide_render = True
    bpy.data.objects["airframe"].color = (*fighter.C_SKIN, 0.15)
    scene.display.shading.show_xray = False
    cam_data = bpy.data.cameras.new("cam")
    cam = bpy.data.objects.new("cam", cam_data)
    scene.collection.objects.link(cam)
    scene.camera = cam
    mid = length / 2 + 1.5
    views = {
        "internals_side": ((mid, -30.0, 0.2), (math.pi / 2, 0, 0), "ORTHO", length * 0.9),
        "internals_top": ((mid, 0.0, 30.0), (0, 0, 0), "ORTHO", length * 0.9),
        "internals_iso": ((mid - 7.0, -9.0, 6.5), None, "PERSP", 0),
    }
    for name, (loc, rot, kind, scale) in views.items():
        cam_data.type = kind
        cam.location = loc
        if rot:
            cam.rotation_euler = rot
            cam_data.ortho_scale = scale
        else:
            cam.rotation_euler = (Vector((mid, 0, 0)) - Vector(loc)).to_track_quat("-Z", "Y").to_euler()
            cam_data.lens = 30
        scene.render.filepath = f"{directory}/{name}.png"
        bpy.ops.render.render(write_still=True)


def main():
    argv = sys.argv[sys.argv.index("--") + 1:] if "--" in sys.argv else []
    opt = lambda k: argv[argv.index(k) + 1] if k in argv else None
    result = fighter.build(1.0)
    data = turbo.load()
    coll = fighter.collection("FIGHTER_internals")
    place_engine(coll, data)
    detail = lift_fan(coll) + missiles(coll) + gear(coll) + routes(coll, data)
    blocks = [bpy.data.objects[n] for n in fighter.EQUIPMENT if n not in fighter.EXTERNAL]
    ducts = [o for o in bpy.data.objects if o.name.startswith("duct_")]
    hidden_lift = {o.name for o in detail if o.name.startswith("lift_")}
    parts = [o for o in blocks + ducts if o.name not in ("lift_fan", "bay_l", "bay_r", "main_gear_l", "main_gear_r")]
    parts += [o for o in detail if o.name not in hidden_lift] + [bpy.data.objects["lift_fan"]]
    clashes, outside = check(parts, bpy.data.objects["airframe"])
    print("INTERNALS clashes:", clashes or "none")
    print("INTERNALS outside the skin, %:", outside or "none")
    if opt("--render"):
        os.makedirs(opt("--render"), exist_ok=True)
        render(opt("--render"), result["length"])
    if opt("--save"):
        bpy.ops.wm.save_as_mainfile(filepath=opt("--save"))


if __name__ == "__main__":
    main()
