# The fighter's engine in 3D, from hf-engine's design (modeling/turbo.py
# reads it): the air path's walls, cut away over a quadrant to show the
# inside; every blade row; each rotor's superconducting motor in the hub,
# sized by its mass; the fan's pitch trunnions; the combustor's liner, the
# mixer's lobes, the reheat's flameholder and spray bars, the nozzle; and
# the turbopump beside the core, its exhaust piped to the combustor.
#
# Headless:  blender -b --factory-startup --python modeling/engine.py -- [--render DIR] [--save FILE]

import math
import os
import sys

import bpy
from mathutils import Vector

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import fighter  # noqa: E402
import turbo  # noqa: E402

# Machines and their stores are dense: superconducting motor and generator,
# kg/m3, and the motor's bore over its outside.
MACHINE_DENSITY = 6000.0
MOTOR_BORE = 0.6
# The quadrant cut away, radians, and the lathe's divisions per turn.
CUT = (math.radians(95), math.radians(195))
TURN = 96
# Mixer lobes; reheat spray bars.
LOBES = 16
SPRAY_BARS = 12

C_CASE = (0.62, 0.64, 0.68)
C_FAN = (0.25, 0.30, 0.36)
C_COMP = (0.45, 0.50, 0.58)
C_STATOR = (0.70, 0.66, 0.55)
C_MOTOR = (0.80, 0.45, 0.15)
C_HOT = (0.85, 0.30, 0.20)
C_TP = (0.30, 0.65, 0.45)


def shown(t):
    """Whether angle `t` is outside the cut-away quadrant."""
    t = math.atan2(math.sin(t), math.cos(t))
    return not (CUT[0] < t < CUT[1])


def lathe(name, profile, coll, color, cut=True, k=TURN):
    """A surface of revolution through (x, r), over the whole turn or all
    of it but the cut-away quadrant."""
    if cut:
        span = 2 * math.pi - (CUT[1] - CUT[0])
        angles = [CUT[1] + span * i / k for i in range(k + 1)]
    else:
        angles = [2 * math.pi * i / k for i in range(k)]
    verts = [(x, r * math.cos(t), r * math.sin(t)) for t in angles for x, r in profile]
    n, faces = len(profile), []
    rings = len(angles) if cut else len(angles) + 1
    for j in range(rings - 1):
        a0, a1 = (j % len(angles)) * n, ((j + 1) % len(angles)) * n
        for i in range(n - 1):
            faces.append((a0 + i, a0 + i + 1, a1 + i + 1, a1 + i))
    return fighter.mesh_object(name, verts, faces, coll, color)


def loft(name, rings, coll, color):
    """A closed solid through equal-length closed rings."""
    n = len(rings[0])
    verts = [p for ring in rings for p in ring]
    faces = []
    for j in range(len(rings) - 1):
        for i in range(n):
            a, b = j * n + i, j * n + (i + 1) % n
            faces.append((a, b, b + n, a + n))
    faces.append(tuple(reversed(range(n))))
    last = (len(rings) - 1) * n
    faces.append(tuple(range(last, last + n)))
    return fighter.mesh_object(name, verts, faces, coll, color)


def ring(name, x0, x1, r_in, r_out, coll, color, cut=True):
    """A thick annulus: a tube from r_in to r_out over x0..x1."""
    return lathe(name, [(x0, r_in), (x0, r_out), (x1, r_out), (x1, r_in), (x0, r_in)], coll, color, cut)


def radial(name, x, r0, r1, t, d, coll, color):
    """A cylinder of diameter d along the radius at angle t, from r0 to r1."""
    o = fighter.cylinder(name, "z", d, r1 - r0, (0, 0, 0), coll, color, k=12)
    r = (r0 + r1) / 2
    o.location = (x, r * math.cos(t), r * math.sin(t))
    o.rotation_euler = (t - math.pi / 2, 0, 0)
    return o


def rows(air, coll):
    for i, row in enumerate(air["rows"]):
        rotor = row["spin"] != 0
        color = C_STATOR if not rotor else C_FAN if row["component"] == "Fan" else C_COMP
        for k in range(row["blades"]):
            theta = 2 * math.pi * k / row["blades"]
            loft(f"row{i}_{k}", turbo.blade(row, theta), coll, color)


def hub_at(inner, x):
    for (x0, r0), (x1, r1) in zip(inner, inner[1:]):
        if x0 <= x <= x1:
            return r0 + (r1 - r0) * (x - x0) / (x1 - x0) if x1 > x0 else min(r0, r1)
    return inner[-1][1]


def motors(air, inner, coll, dx=0.002):
    """Each rotor's motor, a ring in the hub from its rotor aft, under the
    stator that follows, tapering with the hub 15 mm inside it, as long as
    its mass needs; returns the overlaps between neighbours, m."""
    rotors = [r for r in air["rows"] if r["spin"] != 0]
    spans = []
    for row, kg in zip(rotors, air["motors"]):
        x, vol, outline = row["x_le"], kg / MACHINE_DENSITY, []
        while vol > 0:
            r = hub_at(inner, x) - 0.015
            outline.append((x, r))
            vol -= math.pi * r * r * (1 - MOTOR_BORE ** 2) * dx
            x += dx
        outline.append((x, hub_at(inner, x) - 0.015))
        bore = [(xx, MOTOR_BORE * r) for xx, r in reversed(outline)]
        lathe(f"motor_{len(spans)}", outline + bore + outline[:1], coll, C_MOTOR)
        spans.append((row["x_le"], x))
    return [max(0.0, a[1] - b[0]) for a, b in zip(spans, spans[1:])]


def trunnions(air, coll):
    """The variable-pitch fan blades' spindles and their bearing rings."""
    for i, row in enumerate(r for r in air["rows"] if r["component"] == "Fan" and r["spin"]):
        x = (row["x_le"] + row["x_te"]) / 2
        r_hub = min(row["hub"])
        d = 0.5 * row["chord"]
        for k in range(row["blades"]):
            t = 2 * math.pi * k / row["blades"]
            if not shown(t):
                continue
            radial(f"trunnion_{i}_{k}", x, r_hub - 0.03, r_hub, t, d, coll, C_MOTOR)


def combustor(air, coll):
    c = air["combustor"]
    h = c["r_out"] - c["r_in"]
    liner = [(c["x0"], c["r_in"] + 0.15 * h), (c["x0"] + 0.1 * h, c["r_in"] + 0.15 * h),
             (c["x1"], c["r_in"] + 0.1 * h)]
    lathe("liner_inner", liner, coll, C_HOT)
    lathe("liner_outer", [(x, c["r_in"] + c["r_out"] - r) for x, r in liner], coll, C_HOT)
    lathe("dome", [(c["x0"] + 0.1 * h, c["r_in"] + 0.15 * h), (c["x0"], c["r_in"] + 0.5 * h),
                   (c["x0"] + 0.1 * h, c["r_out"] - 0.15 * h)], coll, C_HOT)


def mixer(air, coll):
    """A lobed mixer: the splitter between core and bypass, waving between
    them over its length into LOBES lobes."""
    m, c = air["mixer"], air["combustor"]
    x0 = c["x1"]
    amp = 0.45 * min(m["r_lobes"] - m["r_cone"], m["r_out"] - m["r_lobes"])
    n_x, n_t = 12, LOBES * 12
    verts, faces = [], []
    for i in range(n_x + 1):
        u = i / n_x
        x = x0 + (m["x"] - x0) * u
        base = c["r_out"] + (m["r_lobes"] - c["r_out"]) * u
        for j in range(n_t):
            t = 2 * math.pi * j / n_t
            r = base + amp * u * u * math.sin(LOBES * t)
            verts.append((x, r * math.cos(t), r * math.sin(t)))
    for i in range(n_x):
        for j in range(n_t):
            a, b = i * n_t + j, i * n_t + (j + 1) % n_t
            if shown(2 * math.pi * (j + 0.5) / n_t):
                faces.append((a, b, b + n_t, a + n_t))
    fighter.mesh_object("mixer", verts, faces, coll, C_HOT)


def reheat(air, coll):
    m, h = air["mixer"], air["reheat"]
    x = m["x"] + 0.3 * (h["x1"] - h["x0"])
    r = h["r_out"]
    for name, rr in (("flameholder_inner", 0.45 * r), ("flameholder_outer", 0.75 * r)):
        lathe(name, [(x, rr - 0.02), (x + 0.04, rr), (x, rr + 0.02)], coll, C_HOT)
    for k in range(SPRAY_BARS):
        t = 2 * math.pi * k / SPRAY_BARS
        if shown(t):
            radial(f"spray_bar_{k}", x - 0.12, 0.15 * r, 0.95 * r, t, 0.015, coll, C_HOT)


def turbopump(tp, air, coll):
    """The turbopump under the core, on the combustor's axis: slurry pump,
    reverter, the turbine's six stages, the generator; its exhaust piped up
    to the combustor."""
    c = air["combustor"]
    axis_z = -(air["bypass"]["r_out"] + 0.25)
    r_t = tp["mean_radius"] + tp["blades"][-1] / 2 + 0.02
    gen_r = 0.16
    gen_len = tp["generator"] / MACHINE_DENSITY / (math.pi * gen_r ** 2)
    rev_r = (tp["reverter"] / (2 * math.pi)) ** (1 / 3)
    x = c["x0"] - 0.6
    parts = [("slurry_pump", 0.07, 0.12, C_TP), ("reverter", rev_r, 2 * rev_r, C_HOT)]
    stage_len = 0.025
    parts += [(f"turbine_{i}", tp["mean_radius"] + b / 2, stage_len, C_HOT) for i, b in enumerate(tp["blades"])]
    parts += [("generator", gen_r, gen_len, C_TP)]
    out = {}
    for name, r, length, color in parts:
        fighter.cylinder(f"tp_{name}", "x", 2 * r, length, Vector((x + length / 2, 0, axis_z)), coll, color, k=32)
        out[name] = (x, x + length)
        x += length + 0.01
    exhaust_x = out[f"turbine_{len(tp['blades']) - 1}"][1]
    pipe = Vector((exhaust_x, 0, (axis_z + -c["r_out"]) / 2))
    fighter.cylinder("tp_exhaust", "z", 0.06, abs(axis_z + c["r_out"]), pipe, coll, C_HOT, k=16)
    return out, r_t


def frame_and_render(directory, length):
    scene = bpy.context.scene
    scene.render.engine = "BLENDER_WORKBENCH"
    scene.display.shading.color_type = "OBJECT"
    scene.display.shading.show_cavity = True
    scene.render.resolution_x, scene.render.resolution_y = 1800, 1000
    cam_data = bpy.data.cameras.new("cam")
    cam = bpy.data.objects.new("cam", cam_data)
    scene.collection.objects.link(cam)
    scene.camera = cam
    target = Vector((length / 2, 0, 0))
    views = {
        "cutaway": Vector((length * 0.42, -3.9, 2.9)),
        "front": Vector((-1.6, -1.0, 0.8)),
        "side": Vector((length / 2, -4.5, 0.0)),
    }
    for name, loc in views.items():
        cam.location = loc
        look = (target if name != "front" else Vector((0.2, 0, 0))) - loc
        cam.rotation_euler = look.to_track_quat("-Z", "Y").to_euler()
        cam_data.lens = 40
        scene.render.filepath = f"{directory}/engine_{name}.png"
        bpy.ops.render.render(write_still=True)


def main():
    argv = sys.argv[sys.argv.index("--") + 1:] if "--" in sys.argv else []
    opt = lambda k: argv[argv.index(k) + 1] if k in argv else None
    data = turbo.load()
    air, tp = data["air"], data["turbopump"]
    fighter.fresh_scene()
    coll = fighter.collection("engine")
    names = ("hub", "core_wall", "bypass_wall", "casing")
    walls = turbo.walls(air)
    for name, wall in zip(names, walls):
        lathe(name, wall, coll, C_CASE)
    lathe("casing_skin", [(x, r + 0.012) for x, r in walls[3]], coll, C_CASE)
    rows(air, coll)
    overlaps = motors(air, walls[0], coll)
    trunnions(air, coll)
    combustor(air, coll)
    mixer(air, coll)
    reheat(air, coll)
    tp_parts, _ = turbopump(tp, air, coll)
    length = turbo.nozzle_profile(air)[-1][0]
    blades = sum(r["blades"] for r in air["rows"])
    print(f"ENGINE length {length:.2f} m from the fan face, casing r {air['rows'][0]['tip'][0]:.3f} m, "
          f"{len(air['rows'])} rows, {blades} blades")
    print("ENGINE motor overlaps along the hub, m:", [round(o, 3) for o in overlaps])
    print("ENGINE turbopump parts, x:", {k: (round(a, 2), round(b, 2)) for k, (a, b) in tp_parts.items()})
    if opt("--render"):
        os.makedirs(opt("--render"), exist_ok=True)
        frame_and_render(opt("--render"), length)
    if opt("--save"):
        bpy.ops.wm.save_as_mainfile(filepath=opt("--save"))


if __name__ == "__main__":
    main()
