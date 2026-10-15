# The fighter: its outer lines and what is packed inside them
# (book/small-craft.md). A greybox to get the shape and the layout right,
# and the bottom-up mass that the down-and-back mission (crates/hf-fighter)
# turns on.
#
# Headless:  blender -b --factory-startup --python modeling/fighter.py -- [--scale S] [--save FILE] [--render DIR]
# Live:      exec(open('/home/eric/proj/haunted-forest/modeling/fighter.py').read())
#
# X runs aft from the nose, Y to starboard, Z up; metres. The airframe
# (fuselage stations, wing, fin, conformal tanks) scales with SCALE; the
# equipment keeps its size and moves with the stations. The fuselage is
# lofted through superelliptic sections; the wing is a cropped delta with a
# biconvex section, one piece through the fuselage. Tank regions are shapes
# of their own: what each holds is its volume clipped to the skin, less
# what equipment it overlaps, times a fill factor for frames and plumbing.
# Every equipment block is checked against the skin.

import math
import sys

import bmesh
import bpy
from mathutils import Vector
from mathutils.bvhtree import BVHTree

SCALE = 1.0

# (x, half-width, height above the axis, depth below it)
FUSELAGE = [
    (0.00, 0.03, 0.03, 0.03),
    (0.60, 0.22, 0.20, 0.20),
    (1.50, 0.40, 0.36, 0.36),
    (2.60, 0.56, 0.52, 0.48),
    (3.60, 0.70, 0.98, 0.58),
    (4.60, 0.95, 1.05, 0.66),
    (5.60, 1.35, 0.95, 0.72),
    (6.80, 1.60, 0.88, 0.74),
    (8.00, 1.50, 0.95, 0.74),
    (9.50, 1.25, 0.95, 0.72),
    (11.00, 1.00, 0.85, 0.66),
    (12.50, 0.74, 0.66, 0.60),
    (13.60, 0.58, 0.56, 0.56),
    (14.20, 0.55, 0.53, 0.53),
]
SECTION_EXP = 2.6  # superellipse exponent: 2 is an ellipse, higher is boxier

WING = {"apex": 5.0, "root": 8.6, "semispan": 4.6, "sweep_le": 58.0, "tc": 0.05, "z": -0.15}
FIN = {"root_x": 9.6, "root": 4.2, "height": 2.6, "sweep_le": 55.0, "tc": 0.05, "z": 0.6}
# Conformal tanks on the back, each side of the dorsal spine: (x0, x1, y, half-width, height)
CFT = {"x0": 5.9, "x1": 13.2, "y": 0.82, "hw": 0.62, "h": 0.74, "z": 0.60}

# Machines at 100 kW/kg: a 50 MW turbopump (generator ~500 kg), a 30 MW
# lift-fan motor (~300 kg), and their electronics.
# Equipment: name -> (kind, size, centre, mass kg, colour). Sizes are fixed;
# x of the centre scales with the airframe. "box" size is (dx, dy, dz);
# "cyl_x" and "cyl_z" are (diameter, length) along X or Z.
EQUIPMENT = {
    "radar":           ("cyl_x", (0.60, 0.80), (1.10, 0.0, 0.0), 150, (0.6, 0.6, 0.9)),
    "avionics":        ("box", (0.80, 0.70, 0.60), (2.10, 0.0, -0.02), 300, (0.5, 0.5, 0.8)),
    "nose_gear":       ("box", (1.00, 0.40, 0.45), (3.00, 0.0, -0.30), 0, (0.3, 0.3, 0.3)),
    "cockpit":         ("box", (2.00, 0.90, 1.35), (3.85, 0.0, 0.35), 400, (0.95, 0.85, 0.35)),
    "lift_fan":        ("cyl_z", (1.55, 1.55), (6.60, 0.0, 0.05), 700, (0.2, 0.8, 0.9)),
    "engine":          ("cyl_x", (1.00, 4.10), (10.65, 0.0, -0.05), 1250, (0.9, 0.45, 0.2)),
    "swivel_nozzle":   ("cyl_x", (0.90, 1.70), (13.55, 0.0, -0.05), 250, (0.95, 0.3, 0.15)),
    "turbopump":       ("cyl_x", (0.45, 1.40), (10.20, 0.62, -0.25), 650, (0.85, 0.2, 0.6)),
    "power_electronics": ("box", (1.20, 0.45, 0.40), (10.20, -0.62, -0.25), 500, (0.9, 0.9, 0.2)),
    "fel":             ("box", (3.60, 0.55, 0.45), (9.70, 0.0, 0.70), 1000, (0.2, 0.9, 0.3)),
    "fel_director":    ("cyl_z", (0.60, 0.45), (7.70, 0.0, 1.05), 200, (0.1, 1.0, 0.4)),
    "main_gear_l":     ("box", (1.40, 0.55, 0.55), (10.00, -1.15, -0.35), 0, (0.3, 0.3, 0.3)),
    "main_gear_r":     ("box", (1.40, 0.55, 0.55), (10.00, 1.15, -0.35), 0, (0.3, 0.3, 0.3)),
}
# Inlet ducts: semicircular capture each side, then a round duct of the
# same area to the engine face, passing the lift fan. (x, y, z) waypoints.
INLET = {"area": 0.30, "face_x": 8.60}
DUCT_PATH = [(4.80, 1.30, -0.05), (5.80, 1.30, -0.05), (6.60, 1.22, -0.05), (7.60, 0.90, -0.05), (8.60, 0.25, -0.05)]

# Tank regions: name -> box (x0, x1, y0, y1, z0, z1), or "wing" for the wing
# box outboard of the fuselage between these chord fractions.
TANKS = {
    "forward":   ("box", (4.85, 5.85, -1.3, 1.3, -0.7, 0.95)),
    "saddle":    ("box", (8.65, 12.6, -1.4, 1.4, -0.75, 0.95)),
    "wing":      ("wing", (0.15, 0.75)),
}
TANK_FILL = 0.75  # of a tank region's free volume: frames, baffles, feeds, and the systems that thread between

# Mass, kg, of what the equipment list does not hold.
STRUCTURE_KG_M2 = 16.0     # CNT airframe per m2 of wetted skin: ~half a Mirage 2000's ~30
HOT_SKIN_KG_M2 = 6.0       # carbide skin and insulation, per m2 of windward and leading-edge skin
TANK_KG_M3 = 50.0          # internal tank walls, liners, insulation, per m3 held
CFT_KG_M3 = 80.0           # conformal tank structure and insulation, per m3 held
GEAR_FRACTION = 0.045      # rough-field gear, of maximum takeoff mass
MTOW = 16500.0
FIXED = {"pilot": 100, "inlets_and_ducts": 350, "propellant_system": 250,
         "flight_controls_and_rcs": 250, "thermal": 200, "electrical": 150}

C_SKIN = (0.55, 0.62, 0.70)
C_TANK = (0.25, 0.45, 0.95)
C_CFT = (0.30, 0.55, 1.00)
C_DUCT = (0.75, 0.75, 0.75)


def args():
    a = sys.argv[sys.argv.index("--") + 1:] if "--" in sys.argv else []
    out = {"scale": SCALE, "save": None, "render": None}
    for i, k in enumerate(a):
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


def superellipse(hw, top, bot, n, k):
    pts = []
    for i in range(k):
        t = 2 * math.pi * i / k
        c, s = math.cos(t), math.sin(t)
        y = hw * math.copysign(abs(c) ** (2 / n), c)
        z = (top if s >= 0 else bot) * math.copysign(abs(s) ** (2 / n), s)
        pts.append((y, z))
    return pts


def loft(name, stations, coll, color, k=48, alpha=1.0, offset=(0, 0, 0)):
    """Closed mesh through superelliptic sections (x, hw, top, bot)."""
    verts, faces = [], []
    for x, hw, top, bot in stations:
        for y, z in superellipse(hw, top, bot, SECTION_EXP, k):
            verts.append((x + offset[0], y + offset[1], z + offset[2]))
    for j in range(len(stations) - 1):
        for i in range(k):
            a, b = j * k + i, j * k + (i + 1) % k
            faces.append((a, b, b + k, a + k))
    faces.append(tuple(reversed(range(k))))
    last = (len(stations) - 1) * k
    faces.append(tuple(range(last, last + k)))
    return mesh_object(name, verts, faces, coll, color, alpha)


def biconvex(chord, tc, n=24):
    """Upper surface of a biconvex section, (x, z) from leading to trailing edge."""
    return [(chord * i / n, 2 * tc * chord * (i / n) * (1 - i / n)) for i in range(n + 1)]


def surface(name, sections, coll, color, vertical=False, alpha=1.0):
    """Closed mesh through airfoil sections: (x_le, span, chord, tc, base)."""
    verts, faces = [], []
    n = 24
    for x_le, s, chord, tc, base in sections:
        up = biconvex(chord, tc, n)
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

    fus = loft("fuselage", [(x * s, hw * s, t * s, b * s) for x, hw, t, b in FUSELAGE], skin, C_SKIN)
    w = WING
    tan = math.tan(math.radians(w["sweep_le"]))
    tip_le = (w["apex"] + w["semispan"] * tan) * s
    tip_chord = max(0.3, w["root"] - w["semispan"] * tan) * s
    wing = surface("wing", [
        (tip_le, -w["semispan"] * s, tip_chord, w["tc"], (0, 0, w["z"] * s)),
        (w["apex"] * s, 0.0, w["root"] * s, w["tc"], (0, 0, w["z"] * s)),
        (tip_le, w["semispan"] * s, tip_chord, w["tc"], (0, 0, w["z"] * s)),
    ], skin, C_SKIN)
    f = FIN
    ftan = math.tan(math.radians(f["sweep_le"]))
    fin = surface("fin", [
        (f["root_x"] * s, f["z"] * s, f["root"] * s, f["tc"], (0, 0, 0)),
        ((f["root_x"] + f["height"] * ftan) * s, (f["z"] + f["height"]) * s,
         max(0.5, f["root"] - f["height"] * ftan) * s, f["tc"], (0, 0, 0)),
    ], skin, C_SKIN, vertical=True)
    oml = union("airframe", [fus, wing, fin], skin, C_SKIN, 0.25)
    for o in (fus, wing, fin):
        o.hide_set(True)
        o.hide_render = True
    oml_v, oml_a = volume(oml)
    tree = bvh(oml)

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
            f0, f1 = spec
            v, m = 0.0, 0.0
            y_in = 1.35 * s
            steps = 20
            for i in range(steps):
                y = y_in + (w["semispan"] * s - y_in) * (i + 0.5) / steps
                le = (w["apex"] + abs(y / s) * tan) * s
                chord = (w["root"] - abs(y / s) * tan) * s
                if chord <= 0.3 * s:
                    continue
                # Biconvex: mean thickness between f0 and f1 of chord.
                thick = sum(4 * w["tc"] * chord * u * (1 - u) for u in [f0 + (f1 - f0) * (j + 0.5) / 10 for j in range(10)]) / 10
                dv = thick * chord * (f1 - f0) * (w["semispan"] * s - y_in) / steps * 2
                v += dv
                m += dv * (le + chord * (f0 + f1) / 2)
            cx = m / v
        tank[name] = {"volume": v * TANK_FILL, "x": cx}

    # Mass and balance.
    windward = 2 * 41.0 * s * s * 0.5 + 0.0  # lower half of the planform
    gear_kg = GEAR_FRACTION * MTOW
    tanks_v = sum(t["volume"] for t in tank.values())
    items = {n: (e["mass"], e["x"]) for n, e in eq.items() if e["mass"]}
    items["structure"] = (STRUCTURE_KG_M2 * oml_a, oml_v and centroid_x(oml))
    items["hot_skin"] = (HOT_SKIN_KG_M2 * (planform(s) + 6.0 * s), centroid_x(oml) - 0.5 * s)
    items["landing_gear"] = (gear_kg, 9.0 * s)
    items["internal_tanks"] = (TANK_KG_M3 * tanks_v, weighted_x(tank))
    for n, m in FIXED.items():
        items[n] = (m, 9.0 * s)
    cft_kg = CFT_KG_M3 * cft_v * TANK_FILL
    empty = sum(m for m, _ in items.values())
    cg_empty = sum(m * x for m, x in items.values()) / empty
    # Full: the tanks hold dust shot with ammonia in its gaps, 0.70 t/m3.
    cft_x = sum(centroid_x(o) for o in cfts) / 2
    prop = 696.0 * (tanks_v + cft_v * TANK_FILL)
    prop_x = (696.0 * tanks_v * weighted_x(tank) + 696.0 * cft_v * TANK_FILL * cft_x) / prop
    full = empty + cft_kg + prop
    cg_full = (empty * cg_empty + cft_kg * cft_x + prop * prop_x) / full
    fan_x = EQUIPMENT["lift_fan"][2][0] * s
    nozzle_x = EQUIPMENT["swivel_nozzle"][2][0] * s
    split = lambda cg: (cg - fan_x) / (nozzle_x - cg)
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
        "outside": {n: f'{100 * e["outside"]:.0f}%' for n, e in eq.items() if e["outside"] > 0.02},
        "masses": {n: round(m) for n, (m, _) in sorted(items.items(), key=lambda kv: -kv[1][0])},
        "empty_with_pilot": empty,
        "cft_dry": cft_kg,
        "cg_empty": cg_empty,
        "cg_full": cg_full,
        "full_mass": full,
        "fan_x": fan_x,
        "nozzle_x": nozzle_x,
        "nozzle_per_fan_thrust": (round(split(cg_empty), 2), round(split(cg_full), 2)),
    }


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
