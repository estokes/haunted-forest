# The fighter's engine in 3D, from hf-engine's design (modeling/turbo.py
# reads it): the air path's walls, cut away over a quadrant to show the
# inside if asked; every blade row; each rotor's superconducting motor in
# the hub; the fan's pitch trunnions; the combustor's liner, the mixer's
# lobes, the reheat's flameholder and spray bars, the nozzle; and the
# turbopump, its turbine's end at the combustor. modeling/fighter_internals.py
# places it in the airframe.
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


class Builder:
    """Puts the engine's parts into `coll`, cut away or whole."""

    def __init__(self, coll, cut):
        self.coll, self.cut, self.objects = coll, cut, []

    def keep(self, o):
        self.objects.append(o)
        return o

    def shown(self, t):
        """Whether angle `t` is outside the cut-away quadrant."""
        t = math.atan2(math.sin(t), math.cos(t))
        return not (self.cut and CUT[0] < t < CUT[1])

    def lathe(self, name, profile, color):
        arc = (CUT[1], 2 * math.pi - (CUT[1] - CUT[0])) if self.cut else None
        return self.keep(fighter.lathe(name, profile, self.coll, color, arc=arc, k=TURN))

    def ring(self, name, x0, x1, r_in, r_out, color):
        """A thick annulus: a tube from r_in to r_out over x0..x1."""
        return self.lathe(name, [(x0, r_in), (x0, r_out), (x1, r_out), (x1, r_in), (x0, r_in)], color)

    def cylinder(self, name, axis, d, length, c, color, k=32):
        return self.keep(fighter.cylinder(name, axis, d, length, c, self.coll, color, k=k))

    def radial(self, name, x, r0, r1, t, d, color):
        """A cylinder of diameter d along the radius at angle t, from r0 to r1."""
        o = self.cylinder(name, "z", d, r1 - r0, (0, 0, 0), color, k=12)
        r = (r0 + r1) / 2
        o.location = (x, r * math.cos(t), r * math.sin(t))
        o.rotation_euler = (t - math.pi / 2, 0, 0)
        return o

    def loft(self, name, rings, color):
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
        return self.keep(fighter.mesh_object(name, verts, faces, self.coll, color))

    def walls(self, air):
        walls = turbo.walls(air)
        for name, wall in zip(("hub", "core_wall", "bypass_wall", "casing"), walls):
            self.lathe(name, wall, C_CASE)
        self.lathe("casing_skin", [(x, r + 0.012) for x, r in walls[3]], C_CASE)

    def rows(self, air):
        for i, row in enumerate(air["rows"]):
            color = C_STATOR if not row["spin"] else C_FAN if row["component"] == "Fan" else C_COMP
            for k in range(row["blades"]):
                self.loft(f"row{i}_{k}", turbo.blade(row, 2 * math.pi * k / row["blades"]), color)

    def motors(self, air):
        """Each rotor's motor, a ring in the hub, as hf-engine places it."""
        for i, m in enumerate(air["motors"]):
            self.ring(f"motor_{i}", m["x0"], m["x1"], m["r_in"], m["r_out"], C_MOTOR)

    def trunnions(self, air):
        """The variable-pitch fan blades' spindles."""
        for i, row in enumerate(r for r in air["rows"] if r["component"] == "Fan" and r["spin"]):
            x, r_hub = (row["x_le"] + row["x_te"]) / 2, min(row["hub"])
            for k in range(row["blades"]):
                t = 2 * math.pi * k / row["blades"]
                if self.shown(t):
                    self.radial(f"trunnion_{i}_{k}", x, r_hub - 0.03, r_hub, t, 0.5 * row["chord"], C_MOTOR)

    def combustor(self, air):
        c = air["combustor"]
        h = c["r_out"] - c["r_in"]
        liner = [(c["x0"], c["r_in"] + 0.15 * h), (c["x0"] + 0.1 * h, c["r_in"] + 0.15 * h),
                 (c["x1"], c["r_in"] + 0.1 * h)]
        self.lathe("liner_inner", liner, C_HOT)
        self.lathe("liner_outer", [(x, c["r_in"] + c["r_out"] - r) for x, r in liner], C_HOT)
        self.lathe("dome", [(c["x0"] + 0.1 * h, c["r_in"] + 0.15 * h), (c["x0"], c["r_in"] + 0.5 * h),
                            (c["x0"] + 0.1 * h, c["r_out"] - 0.15 * h)], C_HOT)

    def mixer(self, air):
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
                if self.shown(2 * math.pi * (j + 0.5) / n_t):
                    faces.append((a, b, b + n_t, a + n_t))
        self.keep(fighter.mesh_object("mixer", verts, faces, self.coll, C_HOT))

    def reheat(self, air):
        m, h = air["mixer"], air["reheat"]
        x = m["x"] + 0.3 * (h["x1"] - h["x0"])
        r = h["r_out"]
        for name, rr in (("flameholder_inner", 0.45 * r), ("flameholder_outer", 0.75 * r)):
            self.lathe(name, [(x, rr - 0.02), (x + 0.04, rr), (x, rr + 0.02)], C_HOT)
        for k in range(SPRAY_BARS):
            t = 2 * math.pi * k / SPRAY_BARS
            if self.shown(t):
                self.radial(f"spray_bar_{k}", x - 0.12, 0.15 * r, 0.95 * r, t, 0.015, C_HOT)

    def nozzle(self, air):
        """The convergent-divergent nozzle, at its dry setting."""
        self.lathe("nozzle", turbo.nozzle_profile(air), C_CASE)

    def turbopump(self, tp):
        """The turbopump along X, its turbine's exhaust end at the origin.
        Returns its objects and length."""
        length = turbo.turbopump_size(tp)[1]
        x, objs = -length, []
        for name, r, part in turbo.turbopump_parts(tp):
            color = C_TP if name in ("slurry_pump", "generator") else C_HOT
            objs.append(self.cylinder(f"tp_{name}", "x", 2 * r, part, Vector((x + part / 2, 0, 0)), color))
            x += part + turbo.TP_GAP
        return objs, length

    def engine(self, air, nozzle=True):
        """The air path's parts; `nozzle` if the engine ends in its own."""
        self.walls(air)
        self.rows(air)
        self.motors(air)
        self.trunnions(air)
        self.combustor(air)
        self.mixer(air)
        self.reheat(air)
        if nozzle:
            self.nozzle(air)


def move(objs, offset):
    for o in objs:
        o.location = Vector(o.location) + Vector(offset)


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
    b = Builder(fighter.collection("engine"), cut=True)
    b.engine(air)
    objs, tp_length = b.turbopump(tp)
    move(objs, (air["combustor"]["x0"], 0, -(air["bypass"]["r_out"] + 0.3)))
    length = turbo.nozzle_profile(air)[-1][0]
    print(f"ENGINE length {length:.2f} m from the fan face, casing r {air['rows'][0]['tip'][0]:.3f} m, "
          f"{len(air['rows'])} rows, {sum(r['blades'] for r in air['rows'])} blades; turbopump {tp_length:.2f} m")
    if opt("--render"):
        os.makedirs(opt("--render"), exist_ok=True)
        frame_and_render(opt("--render"), length)
    if opt("--save"):
        bpy.ops.wm.save_as_mainfile(filepath=opt("--save"))


if __name__ == "__main__":
    main()
