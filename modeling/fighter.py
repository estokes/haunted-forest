# The fighter: its outer lines and what is packed inside them
# (book/small-craft.md). A greybox to get the shape and the layout right,
# and the bottom-up mass that the down-and-back mission (crates/hf-fighter)
# turns on.
#
# Headless:  blender -b --factory-startup --python modeling/fighter.py -- [--scale S] [--gauge MM] [--high] [--sweep] [--save FILE] [--render DIR]
# Live:      exec(open('/home/eric/proj/haunted-forest/modeling/fighter.py').read())
#
# X runs aft from the nose, Y to starboard, Z up; metres. The airframe
# (fuselage stations, wing, fin, conformal tanks) scales with SCALE; the
# equipment keeps its size and moves with the stations. The fuselage is
# lofted through superelliptic sections; the wing is a cropped delta with a
# biconvex section, one piece through the fuselage. Tank regions are shapes
# of their own: what each holds is its volume clipped to the skin, less
# what equipment it overlaps, times a fill factor for frames and plumbing.
# Every equipment block is checked against the skin. There is no canopy:
# the pilot rides a liquid-filled capsule in the spine, near the centre of
# gravity, and sees through the sensors. The structure is sized from the
# loads by modeling/fighter_structure.py.

import math
import sys

import os

import bmesh
import bpy

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import fighter_equipment  # noqa: E402
import fighter_structure  # noqa: E402

# Equipment masses, kg, central or (--high) what today's practice adds back.
HIGH = "--high" in sys.argv
_EQ = {n: v[1 if HIGH else 0] for n, v in fighter_equipment.items().items()}
fighter_structure.GEAR_FRACTION = _EQ["landing_gear"] / fighter_equipment.MAX_LANDING_KG
from mathutils import Vector
from mathutils.bvhtree import BVHTree

SCALE = 1.0

# The fuselage's stations: (x, half-width, height of the body above the
# axis, depth below it, height of the dorsal spine above the body, the
# spine's half-width). Aft of the radome the section is chined: an elliptic
# top and a flatter belly meet at a sharp edge at the half-width; the spine
# rides the top over the capsule and the laser. Between stations every
# parameter runs smoothly (monotone cubic), so the stations are a shape,
# not a polyline.
FUSELAGE = [
    (0.00, 0.03, 0.03, 0.03, 0.00, 0.00),
    (0.40, 0.15, 0.15, 0.13, 0.00, 0.00),
    (1.00, 0.29, 0.29, 0.26, 0.00, 0.00),
    (1.80, 0.43, 0.42, 0.38, 0.00, 0.00),
    (2.60, 0.56, 0.51, 0.45, 0.00, 0.00),
    (3.40, 0.82, 0.58, 0.50, 0.00, 0.00),
    (4.60, 1.12, 0.68, 0.60, 0.06, 0.45),
    (5.60, 1.40, 0.76, 0.70, 0.04, 0.45),
    (6.80, 1.62, 0.82, 0.74, 0.10, 0.55),
    (8.00, 1.52, 0.84, 0.74, 0.32, 0.75),
    (9.50, 1.28, 0.84, 0.72, 0.32, 0.75),
    (11.00, 1.02, 0.80, 0.66, 0.30, 0.65),
    (12.50, 0.76, 0.70, 0.60, 0.26, 0.55),
    (13.60, 0.60, 0.56, 0.56, 0.10, 0.40),
    (14.20, 0.55, 0.53, 0.53, 0.00, 0.00),
]
# Superellipse exponents of the top and the belly along x: a round radome,
# then the chine.
SECTION = [(0.0, 2.0, 2.0), (2.2, 2.0, 2.0), (4.2, 2.4, 4.0)]

# The wing's sections: (y, thickness/chord, fullness). Fullness 1 is
# biconvex; lower is fuller, deep over more of the chord. The inner wing is
# a little fuller and thicker, 5.5-6%, to hold a missile bay each side;
# outboard it is a 5% biconvex.
WING = {"apex": 5.0, "root": 8.6, "semispan": 4.6, "sweep_le": 58.0, "tc": 0.06, "z": -0.15,
        "sections": [(0.0, 0.055, 0.5), (2.8, 0.060, 0.5), (3.4, 0.050, 1.0), (4.6, 0.050, 1.0)]}
FIN = {"root_x": 9.6, "root": 4.2, "height": 2.6, "sweep_le": 55.0, "tc": 0.05, "z": 0.6}
# Conformal tanks on the back, each side of the dorsal spine, or None:
# (x0, x1, y, half-width, height)
CFT = None

# Equipment: name -> (kind, size, centre, mass kg, colour). Sizes are fixed;
# x of the centre scales with the airframe. "box" size is (dx, dy, dz);
# "cyl_x" and "cyl_z" are (diameter, length) along X or Z.
EQUIPMENT = {
    "radar":           ("cyl_x", (0.45, 0.45), (2.00, 0.0, 0.02), _EQ["radar"], (0.6, 0.6, 0.9)),
    "avionics":        ("box", (0.50, 0.50, 0.38), (2.90, 0.0, 0.02), _EQ["avionics"], (0.5, 0.5, 0.8)),
    "nose_gear":       ("box", (1.00, 0.40, 0.36), (3.70, 0.0, -0.20), 0, (0.3, 0.3, 0.3)),
    # Pilot, ~0.24 m3 of tissue-density breathing liquid, the shell, the
    # oxygenator and scrubber, the BCI: an escape capsule in its own right.
    # It is boarded through a dorsal hatch between the conformal tanks, by
    # a built-in ladder, and ejects upward through it. Drained, its liquid
    # waits in the reservoir.
    "capsule":         ("box", (2.00, 0.60, 0.50), (8.60, 0.0, 0.72), _EQ["capsule"], (0.95, 0.85, 0.35)),
    "capsule_reservoir": ("box", (0.70, 0.60, 0.60), (5.45, 0.0, 0.35), 0, (0.95, 0.75, 0.45)),
    # The fan's shaft runs skin to skin; its doors are the skin.
    "lift_fan":        ("cyl_z", (1.55, 1.42), (6.60, 0.0, 0.06), _EQ["lift_fan"], (0.2, 0.8, 0.9)),
    # Fan, compressor, casings, hot section, nozzle, and motors for ~45 MW,
    # what the fan and compressor want at Mach 2.
    "engine":          ("cyl_x", (1.00, 4.10), (10.65, 0.0, -0.05), _EQ["engine"] + _EQ["engine_motors"], (0.9, 0.45, 0.2)),
    "swivel_nozzle":   ("cyl_x", (0.90, 1.70), (13.55, 0.0, -0.05), _EQ["swivel_nozzle"], (0.95, 0.3, 0.15)),
    "turbopump":       ("cyl_x", (0.45, 1.40), (10.20, 0.68, 0.22), _EQ["turbopump"], (0.85, 0.2, 0.6)),
    "power_electronics": ("box", (1.20, 0.45, 0.40), (10.20, -0.68, 0.22), _EQ["power_electronics"], (0.9, 0.9, 0.2)),
    "fel":             ("box", (3.00, 0.55, 0.40), (11.20, 0.0, 0.62), _EQ["laser"], (0.2, 0.9, 0.3)),
    "fel_director":    ("cyl_z", (0.60, 0.40), (4.90, 0.0, 0.70), _EQ["laser_turret"], (0.1, 1.0, 0.4)),
    # Stowed beside the engine's belly; the legs swing out to a wide track.
    "main_gear_l":     ("box", (1.30, 0.40, 0.32), (10.30, -0.66, -0.40), 0, (0.3, 0.3, 0.3)),
    "main_gear_r":     ("box", (1.30, 0.40, 0.32), (10.30, 0.66, -0.40), 0, (0.3, 0.3, 0.3)),
    # Two dust missiles side by side in each wing root, ~2.4 m x 0.18 m and
    # ~90 kg each; doors on the belly, sealed for entry like the Shuttle's
    # gear doors. Mass: doors, ejectors, hot seals.
    "bay_l":           ("box", (2.60, 0.50, 0.24), (11.10, -1.85, -0.15), _EQ["bays"] / 2, (0.95, 0.55, 0.1)),
    "bay_r":           ("box", (2.60, 0.50, 0.24), (11.10, 1.85, -0.15), _EQ["bays"] / 2, (0.95, 0.55, 0.1)),
}
# Equipment meant to stand proud of the skin: the laser's turret and the
# nozzle's exit.
EXTERNAL = ("fel_director", "swivel_nozzle")
# Inlet ducts: semicircular capture each side, then a round duct of the
# same area to the engine face, passing the lift fan. (x, y, z) waypoints.
INLET = {"area": 0.30, "face_x": 8.60, "mouth_x": 4.90, "fair_x": 8.60, "fairing_r": 0.42}
DUCT_PATH = [(4.80, 1.24, -0.05), (5.80, 1.26, -0.05), (6.60, 1.20, -0.05), (7.60, 0.90, -0.05), (8.60, 0.25, -0.05)]

# Tank regions: name -> box (x0, x1, y0, y1, z0, z1), or "wing" for the wing
# box outboard of the fuselage between these chord fractions.
TANKS = {
    "forward":   ("box", (3.10, 5.85, -1.3, 1.3, -0.7, 0.95)),
    "saddle":    ("box", (8.65, 12.6, -1.4, 1.4, -0.75, 0.95)),
    "wing":      ("wing", (0.15, 0.75)),
}
TANK_FILL = 0.75  # of a tank region's free volume: frames, baffles, feeds, and the systems that thread between

# Equipment the list does not place, kg, taken at the centre of gravity.
# Flush hardpoints: no pylons in the flow on entry.
FIXED = {n: _EQ[n] for n in ("inlets_and_ducts", "propellant_system", "flight_controls_and_rcs",
                              "thermal", "electrical")}
# The design manoeuvre's mass, and the heaviest the aircraft lands at.
DESIGN_KG = fighter_structure.DESIGN_KG
LANDING_KG = 21000.0

C_SKIN = (0.55, 0.62, 0.70)
C_TANK = (0.25, 0.45, 0.95)
C_CFT = (0.30, 0.55, 1.00)
C_DUCT = (0.75, 0.75, 0.75)


def args():
    a = sys.argv[sys.argv.index("--") + 1:] if "--" in sys.argv else []
    out = {"scale": SCALE, "save": None, "render": None}
    for i, k in enumerate(a):
        if k == "--gauge":
            fighter_structure.MIN_GAUGE = float(a[i + 1]) * 1e-3
            fighter_structure.BELLY_GAUGE = max(fighter_structure.BELLY_GAUGE, fighter_structure.MIN_GAUGE)
        if k == "--scale":
            out["scale"] = float(a[i + 1])
        elif k == "--save":
            out["save"] = a[i + 1]
        elif k == "--render":
            out["render"] = a[i + 1]
    return out


def fresh_scene():
    if bpy.data.objects:
        bpy.data.batch_remove(tuple(bpy.data.objects))
    for c in list(bpy.data.collections):
        bpy.data.collections.remove(c)
    for block in (bpy.data.meshes, bpy.data.curves, bpy.data.cameras, bpy.data.lights):
        orphans = [b for b in block if b.users == 0]
        if orphans:
            bpy.data.batch_remove(orphans)


def collection(name):
    c = bpy.data.collections.new(name)
    bpy.context.scene.collection.children.link(c)
    return c


def mesh_object(name, verts, faces, coll, color, alpha=1.0):
    m = bpy.data.meshes.new(name)
    m.from_pydata(verts, [], faces)
    m.update()
    o = bpy.data.objects.new(name, m)
    coll.objects.link(o)
    o.color = (*color, alpha)
    return o


def pchip(xs, ys, x):
    """Monotone cubic interpolation (Fritsch-Carlson) of ys over xs at x."""
    n = len(xs)
    if x <= xs[0]:
        return ys[0]
    if x >= xs[-1]:
        return ys[-1]
    h = [xs[i + 1] - xs[i] for i in range(n - 1)]
    d = [(ys[i + 1] - ys[i]) / h[i] for i in range(n - 1)]
    m = [d[0]] + [0.0] * (n - 2) + [d[-1]]
    for i in range(1, n - 1):
        if d[i - 1] * d[i] > 0:
            w1, w2 = 2 * h[i] + h[i - 1], h[i] + 2 * h[i - 1]
            m[i] = (w1 + w2) / (w1 / d[i - 1] + w2 / d[i])
    i = max(j for j in range(n - 1) if xs[j] <= x)
    t = (x - xs[i]) / h[i]
    h00, h10 = 2 * t ** 3 - 3 * t * t + 1, t ** 3 - 2 * t * t + t
    h01, h11 = -2 * t ** 3 + 3 * t * t, t ** 3 - t * t
    return h00 * ys[i] + h10 * h[i] * m[i] + h01 * ys[i + 1] + h11 * h[i] * m[i + 1]


def station(x, s=1.0):
    """The fuselage's (hw, top, bot, spine height, spine half-width, top
    exponent, belly exponent) at x."""
    xs = [st[0] * s for st in FUSELAGE]
    shape = tuple(pchip(xs, [st[k] * s for st in FUSELAGE], x) for k in range(1, 6))
    xe = [st[0] * s for st in SECTION]
    return shape + tuple(pchip(xe, [st[k] for st in SECTION], x) for k in (1, 2))


def fuselage_section(hw, top, bot, spine_h, spine_w, n_top, n_belly, k=72):
    """The section, a ring of (y, z) from the starboard chine over the top
    and back under the belly."""
    pts, n = [], k // 2
    for i in range(n):
        u = math.cos(math.pi * i / n)
        z = top * (1 - abs(u) ** n_top) ** (1 / n_top)
        if spine_w > 0:
            z += spine_h * max(0.0, 1 - (hw * u / spine_w) ** 2) ** 1.5
        pts.append((hw * u, z))
    for i in range(n):
        u = -math.cos(math.pi * i / n)
        pts.append((hw * u, -bot * (1 - abs(u) ** n_belly) ** (1 / n_belly)))
    return pts


def fuselage_stations(s, count=90):
    """x of each lofted ring, close together at the nose and tail."""
    end = FUSELAGE[-1][0] * s
    return [end * (1 - math.cos(math.pi * i / (count - 1))) / 2 for i in range(count)]


def ring_loft(name, rings, coll, color, alpha=1.0):
    """Closed mesh through rings of 3D points, all of one length."""
    k = len(rings[0])
    verts = [p for ring in rings for p in ring]
    faces = []
    for j in range(len(rings) - 1):
        for i in range(k):
            a, b = j * k + i, j * k + (i + 1) % k
            faces.append((a, b, b + k, a + k))
    faces.append(tuple(reversed(range(k))))
    last = (len(rings) - 1) * k
    faces.append(tuple(range(last, last + k)))
    return mesh_object(name, verts, faces, coll, color, alpha)


def duct_point(x, s):
    """The inlet duct's centreline (y, z) at x, starboard."""
    xs = [p[0] * s for p in DUCT_PATH]
    return pchip(xs, [p[1] for p in DUCT_PATH], x), pchip(xs, [p[2] for p in DUCT_PATH], x)


def superellipse(hw, top, bot, n, k):
    pts = []
    for i in range(k):
        t = 2 * math.pi * i / k
        c, s = math.cos(t), math.sin(t)
        y = hw * math.copysign(abs(c) ** (2 / n), c)
        z = (top if s >= 0 else bot) * math.copysign(abs(s) ** (2 / n), s)
        pts.append((y, z))
    return pts


def loft(name, stations, coll, color, k=48, alpha=1.0, offset=(0, 0, 0), n=2.5):
    """Closed mesh through superelliptic sections (x, hw, top, bot)."""
    verts, faces = [], []
    for x, hw, top, bot in stations:
        for y, z in superellipse(hw, top, bot, n, k):
            verts.append((x + offset[0], y + offset[1], z + offset[2]))
    for j in range(len(stations) - 1):
        for i in range(k):
            a, b = j * k + i, j * k + (i + 1) % k
            faces.append((a, b, b + k, a + k))
    faces.append(tuple(reversed(range(k))))
    last = (len(stations) - 1) * k
    faces.append(tuple(range(last, last + k)))
    return mesh_object(name, verts, faces, coll, color, alpha)


def half_thickness(chord, tc, fullness, u):
    """Half the section's thickness at chord fraction u: biconvex at
    fullness 1, fuller below."""
    return 0.5 * tc * chord * (4 * u * (1 - u)) ** fullness


def section(chord, tc, fullness, n=24):
    """Upper surface, (x, z) from leading to trailing edge."""
    return [(chord * i / n, half_thickness(chord, tc, fullness, i / n)) for i in range(n + 1)]


def surface(name, sections, coll, color, vertical=False, alpha=1.0):
    """Closed mesh through airfoil sections: (x_le, span, chord, tc, base,
    fullness)."""
    verts, faces = [], []
    n = 24
    for x_le, s, chord, tc, base, fullness in sections:
        up = section(chord, tc, fullness, n)
        ring = [(x_le + x, z) for x, z in up] + [(x_le + x, -z) for x, z in reversed(up[1:-1])]
        for x, z in ring:
            verts.append((x, z + base[1], s) if vertical else (x, s, z + base[2]))
    k = 2 * n
    for j in range(len(sections) - 1):
        for i in range(k):
            a, b = j * k + i, j * k + (i + 1) % k
            faces.append((a, b, b + k, a + k))
    faces.append(tuple(reversed(range(k))))
    last = (len(sections) - 1) * k
    faces.append(tuple(range(last, last + k)))
    return mesh_object(name, verts, faces, coll, color, alpha)


def box(name, x0, x1, y0, y1, z0, z1, coll, color, alpha=1.0):
    v = [(x, y, z) for x in (x0, x1) for y in (y0, y1) for z in (z0, z1)]
    f = [(0, 1, 3, 2), (4, 6, 7, 5), (0, 4, 5, 1), (2, 3, 7, 6), (0, 2, 6, 4), (1, 5, 7, 3)]
    return mesh_object(name, v, f, coll, color, alpha)


def cylinder(name, axis, d, length, c, coll, color, k=32):
    verts = []
    for end in (-0.5, 0.5):
        for i in range(k):
            t = 2 * math.pi * i / k
            a, b = d / 2 * math.cos(t), d / 2 * math.sin(t)
            if axis == "x":
                verts.append((c[0] + end * length, c[1] + a, c[2] + b))
            else:
                verts.append((c[0] + a, c[1] + b, c[2] + end * length))
    faces = [(i, (i + 1) % k, k + (i + 1) % k, k + i) for i in range(k)]
    faces += [tuple(reversed(range(k))), tuple(range(k, 2 * k))]
    return mesh_object(name, verts, faces, coll, color)


def volume(o):
    bm = bmesh.new()
    bm.from_mesh(o.data)
    bm.transform(o.matrix_world)
    v = abs(bm.calc_volume(signed=True))
    a = sum(f.calc_area() for f in bm.faces)
    bm.free()
    return v, a


def union(name, objs, coll, color, alpha):
    base = objs[0].copy()
    base.data = objs[0].data.copy()
    base.name = name
    coll.objects.link(base)
    for o in objs[1:]:
        mod = base.modifiers.new("u", "BOOLEAN")
        mod.operation = "UNION"
        mod.solver = "EXACT"
        mod.object = o
        dg = bpy.context.evaluated_depsgraph_get()
        ev = base.evaluated_get(dg)
        m = bpy.data.meshes.new_from_object(ev)
        base.modifiers.clear()
        base.data = m
    base.color = (*color, alpha)
    return base


def bvh(o):
    bm = bmesh.new()
    bm.from_mesh(o.data)
    bm.transform(o.matrix_world)
    t = BVHTree.FromBMesh(bm)
    bm.free()
    return t


def inside(tree, p):
    """Ray parity: is p inside the closed mesh?"""
    n, origin, d = 0, Vector(p), Vector((0.0, 0.37, 0.93)).normalized()
    while True:
        hit = tree.ray_cast(origin, d)
        if hit[0] is None:
            return n % 2 == 1
        n += 1
        origin = hit[0] + d * 1e-5


def sample_volume(tree, x0, x1, y0, y1, z0, z1, step, keep=lambda p: True):
    """Volume of the part of a box inside the mesh, and its centroid."""
    v, cx = 0.0, 0.0
    cell = step ** 3
    x = x0 + step / 2
    while x < x1:
        y = y0 + step / 2
        while y < y1:
            z = z0 + step / 2
            while z < z1:
                p = (x, y, z)
                if inside(tree, p) and keep(p):
                    v += cell
                    cx += x * cell
                z += step
            y += step
        x += step
    return v, (cx / v if v else (x0 + x1) / 2)


def build(scale):
    fresh_scene()
    bpy.context.scene.unit_settings.system = "METRIC"
    s = scale
    skin = collection("FIGHTER_skin")
    gear = collection("FIGHTER_equipment")
    tanks = collection("FIGHTER_tanks")
    ducts = collection("FIGHTER_ducts")

    rings = []
    for x in fuselage_stations(s):
        rings.append([(x, y, z) for y, z in fuselage_section(*station(x, s))])
    fus = ring_loft("fuselage", rings, skin, C_SKIN)
    # Half-cone inlets on the flanks: a fairing round each duct from its
    # mouth, sinking into the body as the duct turns in to the engine.
    nacelles = []
    for side in (-1, 1):
        nrings = []
        for i in range(24):
            x = (INLET["mouth_x"] + (INLET["fair_x"] - INLET["mouth_x"]) * i / 23) * s
            y, z = duct_point(x, s)
            r = INLET["fairing_r"] * s
            nrings.append([(x, side * y + r * math.cos(t), z + r * math.sin(t))
                           for t in [2 * math.pi * j / 36 for j in range(36)]])
        nacelles.append(ring_loft(f"inlet_{side}", nrings, skin, C_SKIN))
    w = WING
    tan = math.tan(math.radians(w["sweep_le"]))
    chord_at = lambda y: max(0.3, w["root"] - abs(y) * tan) * s
    le_at = lambda y: (w["apex"] + abs(y) * tan) * s
    half = [(y * s, tc, p) for y, tc, p in w["sections"]]
    spanwise = [(-y, tc, p) for y, tc, p in reversed(half[1:])] + half
    wing = surface("wing", [(le_at(y / s), y, chord_at(y / s), tc, (0, 0, w["z"] * s), p)
                            for y, tc, p in spanwise], skin, C_SKIN)
    f = FIN
    ftan = math.tan(math.radians(f["sweep_le"]))
    fin = surface("fin", [
        (f["root_x"] * s, f["z"] * s, f["root"] * s, f["tc"], (0, 0, 0), 1.0),
        ((f["root_x"] + f["height"] * ftan) * s, (f["z"] + f["height"]) * s,
         max(0.5, f["root"] - f["height"] * ftan) * s, f["tc"], (0, 0, 0), 1.0),
    ], skin, C_SKIN, vertical=True)
    oml = union("airframe", [fus, *nacelles, wing, fin], skin, C_SKIN, 0.25)
    for o in (fus, *nacelles, wing, fin):
        o.hide_set(True)
        o.hide_render = True
    oml_v, oml_a = volume(oml)
    tree = bvh(oml)

    cfts = []
    if CFT:
        c = CFT
        cft_st = []
        for i in range(9):
            u = i / 8
            x = (c["x0"] + u * (c["x1"] - c["x0"])) * s
            taper = math.sin(math.pi * u) ** 0.5 if 0 < u < 1 else 0.05
            cft_st.append((x, c["hw"] * s * taper, c["h"] * s * taper, c["h"] * s * 0.35 * taper))
        cfts = [loft(f"cft_{side}", cft_st, tanks, C_CFT, alpha=0.8, offset=(0, side * c["y"] * s, c["z"] * s))
                for side in (-1, 1)]
    cft_v = sum(volume(o)[0] for o in cfts)
    cft_a = sum(volume(o)[1] for o in cfts)

    # Equipment.
    eq = {}

    def outside_share(points):
        return sum(1 for p in points if not inside(tree, p)) / len(points)

    for name, (kind, size, cen, mass, color) in EQUIPMENT.items():
        cen = (cen[0] * s, cen[1], cen[2])
        if kind == "box":
            dx, dy, dz = size
            o = box(name, cen[0] - dx / 2, cen[0] + dx / 2, cen[1] - dy / 2, cen[1] + dy / 2,
                    cen[2] - dz / 2, cen[2] + dz / 2, gear, color)
            grid = [(cen[0] + (i / 4 - 0.5) * dx, cen[1] + (j / 4 - 0.5) * dy, cen[2] + (k / 4 - 0.5) * dz)
                    for i in range(5) for j in range(5) for k in range(5)]
            vol = dx * dy * dz
        else:
            d, L = size
            o = cylinder(name, kind[-1], d, L, cen, gear, color)
            pts = [(e, r * math.cos(t), r * math.sin(t)) for e in (-0.5, -0.25, 0, 0.25, 0.5)
                   for r in (0, d / 4, d / 2) for t in [i * math.pi / 4 for i in range(8)]]
            if kind == "cyl_x":
                grid = [(cen[0] + e * L, cen[1] + a, cen[2] + b) for e, a, b in pts]
            else:
                grid = [(cen[0] + a, cen[1] + b, cen[2] + e * L) for e, a, b in pts]
            vol = math.pi * d * d / 4 * L
        eq[name] = {"volume": vol, "mass": mass, "x": cen[0], "outside": outside_share(grid), "obj": o}

    # Inlet ducts, as tubes along the path.
    r_duct = math.sqrt(INLET["area"] / math.pi)
    duct_v = 0.0
    for side in (-1, 1):
        pts = [(x * s, side * y, z) for x, y, z in DUCT_PATH]
        curve = bpy.data.curves.new(f"duct_{side}", "CURVE")
        curve.dimensions = "3D"
        curve.bevel_depth = r_duct
        sp = curve.splines.new("POLY")
        sp.points.add(len(pts) - 1)
        for p, q in zip(sp.points, pts):
            p.co = (*q, 1)
        o = bpy.data.objects.new(f"duct_{side}", curve)
        ducts.objects.link(o)
        o.color = (*C_DUCT, 1)
        length = sum((Vector(pts[i + 1]) - Vector(pts[i])).length for i in range(len(pts) - 1))
        duct_v += INLET["area"] * length

    # What the equipment and ducts take out of a tank region.
    def occupied(p):
        x, y, z = p
        for name, (kind, size, cen, *_r) in EQUIPMENT.items():
            cx = cen[0] * s
            if kind == "box":
                dx, dy, dz = size
                if abs(x - cx) <= dx / 2 and abs(y - cen[1]) <= dy / 2 and abs(z - cen[2]) <= dz / 2:
                    return True
            elif kind == "cyl_x":
                d, L = size
                if abs(x - cx) <= L / 2 and (y - cen[1]) ** 2 + (z - cen[2]) ** 2 <= d * d / 4:
                    return True
            else:
                d, L = size
                if abs(z - cen[2]) <= L / 2 and (x - cx) ** 2 + (y - cen[1]) ** 2 <= d * d / 4:
                    return True
        for side in (-1, 1):
            pts = [Vector((px * s, side * py, pz)) for px, py, pz in DUCT_PATH]
            q = Vector(p)
            for a, b in zip(pts, pts[1:]):
                ab = b - a
                t = max(0.0, min(1.0, (q - a).dot(ab) / ab.length_squared))
                if (a + ab * t - q).length <= r_duct + 0.03:
                    return True
        return False

    tank = {}
    for name, (kind, spec) in TANKS.items():
        if kind == "box":
            x0, x1, y0, y1, z0, z1 = spec
            region = (x0 * s, x1 * s, y0 * s, y1 * s, z0 * s, z1 * s)
            v, cx = sample_volume(tree, *region, 0.1, keep=lambda p: not occupied(p))
            box(f"tank_{name}", *region, tanks, C_TANK, alpha=0.35)
        else:
            # The wing box outboard of the fuselage, inside the skin, less
            # the bays and whatever else is in it.
            f0, f1 = spec
            y_in = 1.35 * s

            def in_box(p):
                y = abs(p[1])
                if y < y_in or occupied(p):
                    return False
                u = (p[0] - le_at(y / s)) / chord_at(y / s)
                return f0 <= u <= f1
            zc = w["z"] * s
            v, cx = sample_volume(tree, w["apex"] * s, (w["apex"] + w["root"]) * s, -w["semispan"] * s,
                                  w["semispan"] * s, zc - 0.35 * s, zc + 0.35 * s, 0.08, keep=in_box)
        tank[name] = {"volume": v * TANK_FILL, "x": cx}

    # Mass and balance.
    tanks_v = sum(t["volume"] for t in tank.values())
    eq_mass = sum(e["mass"] for e in eq.values()) + sum(FIXED.values())
    cft_x = sum(centroid_x(o) for o in cfts) / 2 if cfts else 0.0
    ext_v = cft_v * TANK_FILL
    # At the design manoeuvre the fuselage carries its equipment and the
    # propellant that brings the aircraft to the design mass, in the tanks.
    lay = [(e["mass"], e["x"]) for e in eq.values() if e["mass"]]
    lay += [(m, centroid_x(oml)) for m in FIXED.values()]
    fuel_x = (tanks_v * weighted_x(tank) + ext_v * cft_x) / (tanks_v + ext_v)
    bays = [e for n, e in eq.items() if n.startswith("bay")]
    bay_v = sum(e["volume"] for e in bays)
    fin_area = (f["root"] + max(0.5, f["root"] - f["height"] * ftan)) / 2 * f["height"] * s * s

    def structure_for(empty_guess):
        geo = {
            "design_mass": DESIGN_KG,
            "landing_mass": LANDING_KG,
            "wing": {k: (v * s if k in ("apex", "root", "semispan") else v) for k, v in w.items()},
            "y_in": hw_at(9.0 * s, s),
            "sections": [(x, fuselage_section(*station(x, s))) for x in fuselage_stations(s, 60)],
            "fuselage_masses": lay + [(max(0.0, DESIGN_KG - empty_guess), fuel_x)],
            "lift_x": None,
            "fin_area": fin_area,
            "internal_tank_surface": sum(6 * max(t["volume"], 0.01) ** (2 / 3) for t in tank.values()),
            "cft_surface": cft_a,
            "wetted": oml_a + cft_a,
            "equipment_mass": eq_mass,
        }
        total = sum(m for m, _ in geo["fuselage_masses"])
        geo["lift_x"] = sum(m * x for m, x in geo["fuselage_masses"]) / total
        return fighter_structure.structure(geo)

    empty = eq_mass + 3000.0
    for _ in range(6):
        parts, diag = structure_for(empty)
        empty = eq_mass + sum(parts.values())
    struct_x = centroid_x(oml)
    items = {n: (e["mass"], e["x"]) for n, e in eq.items() if e["mass"]}
    for n, m in FIXED.items():
        items[n] = (m, struct_x)
    for n, m in parts.items():
        x = {"cft_walls": cft_x, "internal_tank_walls": weighted_x(tank)}.get(n, struct_x)
        items[n] = (m, x)
    cft_kg = parts["cft_walls"]
    empty = sum(m for m, _ in items.values()) - cft_kg
    cg_empty = (sum(m * x for m, x in items.values()) - cft_kg * cft_x) / empty
    # Full: the tanks hold dust shot with ammonia in its gaps, 0.70 t/m3.
    prop = 696.0 * (tanks_v + ext_v)
    prop_x = (696.0 * tanks_v * weighted_x(tank) + 696.0 * ext_v * cft_x) / prop
    full = empty + cft_kg + prop
    cg_full = (empty * cg_empty + cft_kg * cft_x + prop * prop_x) / full
    fan_x = EQUIPMENT["lift_fan"][2][0] * s
    nozzle_x = EQUIPMENT["swivel_nozzle"][2][0] * s
    split = lambda cg: (cg - fan_x) / (nozzle_x - cg)

    # Trim. Subsonic, the wing's aerodynamic centre: a quarter of its mean
    # chord. Supersonic and hypersonic, near the centroid of the whole
    # planform, body and all (linear theory on a delta; Newtonian flow on a
    # flat underside). The centre of gravity full, empty, and with one tank
    # drained at a time.
    lam = max(0.3, w["root"] - w["semispan"] * tan) / w["root"]
    mac = 2 / 3 * w["root"] * s * (1 + lam + lam * lam) / (1 + lam)
    y_mac = w["semispan"] * s / 3 * (1 + 2 * lam) / (1 + lam)
    ac_sub = le_at(y_mac / s) + 0.25 * mac
    area, mx = 0.0, 0.0
    step = 0.05
    x = step / 2
    while x < (FUSELAGE[-1][0] + 0.5) * s:
        y = -w["semispan"] * s + step / 2
        while y < w["semispan"] * s:
            if tree.ray_cast(Vector((x, y, 10.0)), Vector((0, 0, -1)))[0] is not None:
                area += step * step
                mx += x * step * step
            y += step
        x += step
    cp_high = mx / area

    def cg_with(drained):
        m = empty * cg_empty + cft_kg * cft_x
        tot = empty + cft_kg
        for n, t in tank.items():
            if n not in drained:
                m += 696.0 * t["volume"] * t["x"]
                tot += 696.0 * t["volume"]
        return m / tot
    margins = {}
    for label, drained in [("full", ()), ("forward drained", ("forward",)), ("saddle drained", ("saddle",)),
                           ("wing drained", ("wing",)), ("empty", tuple(tank))]:
        cg = cg_with(drained)
        margins[label] = (round(cg, 2), round(100 * (ac_sub - cg) / mac), round(100 * (cp_high - cg) / mac),
                          round(split(cg), 2))
    return {
        "scale": s,
        "length": FUSELAGE[-1][0] * s,
        "span": 2 * w["semispan"] * s,
        "wing_area": planform(s),
        "airframe_volume": oml_v,
        "wetted_area": oml_a + cft_a,
        "equipment_volume": sum(e["volume"] for e in eq.values()),
        "duct_volume": duct_v,
        "tanks": {n: (round(t["volume"], 2), round(t["x"], 2)) for n, t in tank.items()},
        "internal_tank_volume": tanks_v,
        "cft_volume": cft_v * TANK_FILL,
        "bay_volume": bay_v,
        "outside": {n: f'{100 * e["outside"]:.0f}%' for n, e in eq.items()
                    if e["outside"] > 0.02 and n not in EXTERNAL},
        "masses": {n: round(m) for n, (m, _) in sorted(items.items(), key=lambda kv: -kv[1][0])},
        "empty_with_pilot": empty,
        "cft_dry": cft_kg,
        "structure": {k: round(v, 2) if isinstance(v, float) else v for k, v in diag.items()},
        "cg_empty": cg_empty,
        "cg_full": cg_full,
        "full_mass": full,
        "fan_x": fan_x,
        "nozzle_x": nozzle_x,
        "nozzle_per_fan_thrust": (round(split(cg_empty), 2), round(split(cg_full), 2)),
        "planform_m2": area,
        "mac": mac,
        "ac_subsonic": ac_sub,
        "cp_super_hyper": cp_high,
        "trim (cg, subsonic margin %MAC, high-speed margin %MAC, nozzle/fan)": margins,
    }


def hw_at(x, s):
    """Fuselage half-width at station x."""
    return station(x, s)[0]


def planform(s):
    w = WING
    tan = math.tan(math.radians(w["sweep_le"]))
    tip = max(0.3, w["root"] - w["semispan"] * tan)
    return (w["root"] + tip) * w["semispan"] * s * s


def centroid_x(o):
    bm = bmesh.new()
    bm.from_mesh(o.data)
    num = sum(f.calc_area() * f.calc_center_median().x for f in bm.faces)
    den = sum(f.calc_area() for f in bm.faces)
    bm.free()
    return num / den


def weighted_x(tank):
    v = sum(t["volume"] for t in tank.values())
    return sum(t["volume"] * t["x"] for t in tank.values()) / v


def frame_and_render(directory):
    scene = bpy.context.scene
    scene.render.engine = "BLENDER_WORKBENCH"
    scene.display.shading.color_type = "OBJECT"
    scene.display.shading.show_xray = True
    scene.display.shading.xray_alpha = 0.7
    scene.render.resolution_x, scene.render.resolution_y = 1600, 900
    cam_data = bpy.data.cameras.new("cam")
    cam_data.type = "ORTHO"
    cam = bpy.data.objects.new("cam", cam_data)
    scene.collection.objects.link(cam)
    scene.camera = cam
    views = {
        "top": ((7.2, 0.0, 30.0), (0.0, 0.0, 0.0), 16.5),
        "side": ((7.2, -30.0, 0.5), (math.pi / 2, 0.0, 0.0), 16.5),
        "front": ((-30.0, 0.0, 0.5), (math.pi / 2, 0.0, -math.pi / 2), 11.0),
    }
    for name, (loc, rot, ortho) in views.items():
        cam.location = loc
        cam.rotation_euler = rot
        cam_data.ortho_scale = ortho
        scene.render.filepath = f"{directory}/fighter_{name}.png"
        bpy.ops.render.render(write_still=True)
    cam_data.type = "PERSP"
    cam.location = (-6.0, -14.0, 9.0)
    direction = Vector((7.0, 0.0, 0.0)) - cam.location
    cam.rotation_euler = direction.to_track_quat("-Z", "Y").to_euler()
    scene.render.filepath = f"{directory}/fighter_iso.png"
    bpy.ops.render.render(write_still=True)


def main():
    opts = args()
    if "--sweep" in sys.argv:
        for sc in (0.85, 0.9, 0.95, 1.0, 1.1):
            r = build(sc)
            print(f"scale {sc}: {r['length']:.1f} m, wing {r['wing_area']:.0f} m2, wetted {r['wetted_area']:.0f} m2, "
                  f"tanks {r['internal_tank_volume']:.1f} + {r['cft_volume']:.1f} m3, empty {r['empty_with_pilot']:.0f} "
                  f"+ {r['cft_dry']:.0f} kg, outside {r['outside']}")
    result = build(opts["scale"])
    for k, v in result.items():
        print(f"{k}: {v}")
    if opts["save"]:
        bpy.ops.wm.save_as_mainfile(filepath=opts["save"])
    if opts["render"]:
        frame_and_render(opts["render"])


if __name__ == "__main__":
    main()
