# The fighter: its outer lines and what is packed inside them
# (book/small-craft.md). A greybox to get the shape and the layout right,
# and the bottom-up mass that the down-and-back mission (crates/hf-fighter)
# turns on.
#
# Headless:  blender -b --factory-startup --python modeling/fighter.py -- [--scale S] [--stretch K] [--gauge MM] [--high] [--sweep] [--save FILE] [--render DIR]
# Live:      exec(open('/home/eric/proj/haunted-forest/modeling/fighter.py').read())
#
# X runs aft from the nose, Y to starboard, Z up; metres. The airframe
# (fuselage stations, wing, fin, conformal tanks) scales with SCALE; the
# equipment keeps its size and moves with the stations. The fuselage is
# lofted through smooth stations; the wing is a low cropped delta, one
# piece through the fuselage. Tank regions are shapes of their own: what
# each holds is its volume clipped to the skin, less what equipment it
# overlaps, times a fill factor for frames and plumbing. Every equipment
# block is checked against the skin. There is no canopy: the pilot rides
# a liquid-filled capsule under a hump where a cockpit would be, and sees
# through the sensors. The structure is sized from the loads by
# modeling/fighter_structure.py; the Mach-1 wave drag, which
# crates/hf-fighter takes as aero::WAVE, from the area distribution.

import copy
import math
import os
import sys

import bmesh
import bpy

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import fighter_equipment  # noqa: E402
import fighter_structure  # noqa: E402
import turbo  # noqa: E402

# Equipment masses, kg, central or (--high) what today's practice adds back.
HIGH = "--high" in sys.argv
_EQ = {n: v[1 if HIGH else 0] for n, v in fighter_equipment.items().items()}
fighter_structure.GEAR_FRACTION = _EQ["landing_gear"] / fighter_equipment.MAX_LANDING_KG
from mathutils import Vector
from mathutils.bvhtree import BVHTree

SCALE = 1.0

# The fuselage's stations: (x, half-width, height of the body above the
# axis, depth below it, height of the dorsal spine above the body, the
# spine's half-width). Round at the radome; aft of it the section is
# chined, an elliptic top and a flatter belly meeting at a sharp edge at
# the half-width, the belly flat where the low wing meets it. The spine
# rides the top: a hump over the capsule, then over the laser and tanks. Between stations every
# parameter runs smoothly (monotone cubic), so the stations are a shape,
# not a polyline.
FUSELAGE = [
    (0.00, 0.03, 0.03, 0.03, 0.00, 0.00),
    (0.56, 0.15, 0.15, 0.14, 0.00, 0.00),
    (1.41, 0.29, 0.29, 0.27, 0.00, 0.00),
    (2.54, 0.42, 0.42, 0.39, 0.00, 0.00),
    (3.38, 0.50, 0.50, 0.46, 0.00, 0.00),
    (4.79, 0.72, 0.64, 0.58, 0.20, 0.45),
    (6.48, 0.96, 0.72, 0.66, 0.24, 0.45),
    (7.89, 1.22, 0.76, 0.70, 0.14, 0.45),
    (9.58, 1.58, 0.82, 0.74, 0.06, 0.45),
    (11.27, 1.52, 0.84, 0.74, 0.22, 0.75),
    (13.38, 1.28, 0.84, 0.72, 0.24, 0.75),
    (15.49, 1.02, 0.80, 0.66, 0.22, 0.65),
    (17.61, 0.76, 0.70, 0.60, 0.16, 0.55),
    (19.15, 0.60, 0.56, 0.56, 0.04, 0.40),
    (20.00, 0.48, 0.43, 0.53, 0.00, 0.00),
]
# Superellipse exponents of the top and the belly along x: a round radome,
# then the chine.
SECTION = [(0.00, 2.00, 2.00), (4.23, 2.00, 2.00), (6.48, 2.20, 3.00), (9.58, 2.40, 4.00)]

# The wing's sections: (y, thickness/chord, fullness). Fullness 1 is
# biconvex; lower is fuller, deep over more of the chord. The thickest
# point is at `peak` of the chord: forward of half, so that the straight
# trailing edge does not stack the whole span's thickness at the tail. The
# inner wing is a little thicker, ~4-4.4%, to hold a missile bay each
# side; outboard it is 3.6%. The wing is low, its underside the belly's
# plane. Its leading edge is slatted, two segments a side, flush when
# retracted; `tc` is the box's for the structure. Its centre of lift,
# subsonic, is `ac` of the mean chord back from the leading edge there, as
# cfd/vlm.py finds it for the wing and body: a slender delta's lies well
# aft of a straight wing's quarter chord.
WING = {"apex": 5.76, "root": 11.83, "semispan": 4.09, "sweep_le": 68.0, "tc": 0.040, "z": -0.48, "peak": 0.4,
        "ac": 0.344,
        "sections": [(0.00, 0.0400, 1.0), (2.49, 0.0437, 1.0), (3.02, 0.0363, 1.0), (4.09, 0.0363, 1.0)]}
# The slats: from `from` to `to` of the semispan, `chord` of the local
# chord, drooped `deflection` degrees about their swept hinge, normal to it
# (0 retracted; set for CFD by cfd/export.py --slats).
SLAT = {"from": 0.32, "to": 0.95, "chord": 0.15, "deflection": 0.0}
FIN = {"root_x": 13.52, "root": 4.2, "height": 2.6, "sweep_le": 55.0, "tc": 0.05, "z": 0.6}
# Conformal tanks on the back, each side of the dorsal spine, or None:
# (x0, x1, y, half-width, height)
CFT = None

# The swivel nozzle: diameter, length, centre. The engine, as hf-engine
# designs it (modeling/turbo.py reads the design), from the fan face to
# the reheat's end, where the swivel nozzle takes its exit: its casing,
# with a skin, as a solid of revolution.
SWIVEL = (0.90, 1.70, 19.08)
_ENGINE = turbo.load()
_AIR = _ENGINE["air"]
TURBOPUMP_SIZE = turbo.turbopump_size(_ENGINE["turbopump"])
ENGINE_PROFILE = ([(-0.05, 0.0)] + [(x, r + 0.012) for x, r in turbo.walls(_AIR)[3] if x <= _AIR["reheat"]["x1"] + 1e-9]
                  + [(_AIR["reheat"]["x1"], 0.0)])
ENGINE_FACE_X = SWIVEL[2] - SWIVEL[1] / 2 - _AIR["reheat"]["x1"]

# Equipment: name -> (kind, size, centre, mass kg, colour). Sizes are fixed;
# x of the centre scales with the airframe. "box" size is (dx, dy, dz);
# "cyl_x" and "cyl_z" are (diameter, length) along X or Z; "lathe" is a
# solid of revolution about X, its (x, r) profile from the centre.
EQUIPMENT = {
    "radar":           ("cyl_x", (0.45, 0.45), (2.04, 0.0, 0.0), _EQ["radar"], (0.6, 0.6, 0.9)),
    "avionics":        ("box", (0.50, 0.50, 0.38), (2.89, 0.0, 0.0), _EQ["avionics"], (0.5, 0.5, 0.8)),
    "nose_gear":       ("box", (1.00, 0.40, 0.36), (4.93, 0.0, -0.25), 0, (0.3, 0.3, 0.3)),
    # Pilot, ~0.24 m3 of tissue-density breathing liquid, the shell, the
    # oxygenator and scrubber, the BCI: an escape capsule in its own right,
    # where a cockpit would be. It is boarded through a dorsal hatch, by a
    # built-in ladder, and ejects upward through it. Drained, its liquid
    # waits in the reservoir under it.
    "capsule":         ("box", (2.00, 0.60, 0.50), (6.48, 0.0, 0.42), _EQ["capsule"], (0.95, 0.85, 0.35)),
    "capsule_reservoir": ("box", (0.70, 0.60, 0.50), (6.97, 0.0, -0.25), 0, (0.95, 0.75, 0.45)),
    # The fan's shaft runs skin to skin; its doors are the skin.
    "lift_fan":        ("cyl_z", (1.55, 1.42), (8.30, 0.0, 0.06), _EQ["lift_fan"], (0.2, 0.8, 0.9)),
    # Fan, compressor, casings, hot section, and motors for what the fan
    # and compressor want at Mach 2.
    "engine":          ("lathe", ENGINE_PROFILE, (ENGINE_FACE_X, 0.0, -0.05), _EQ["engine"] + _EQ["engine_motors"], (0.9, 0.45, 0.2)),
    "swivel_nozzle":   ("cyl_x", SWIVEL[:2], (SWIVEL[2], 0.0, -0.05), _EQ["swivel_nozzle"], (0.95, 0.3, 0.15)),
    # Beside the engine, its turbine's end at the combustor, so that its
    # 4,000 K exhaust has no length of pipe to cross; the power
    # electronics opposite, forward, where cable is all they need.
    "turbopump":       ("cyl_x", TURBOPUMP_SIZE, (ENGINE_FACE_X + _AIR["combustor"]["x0"] - TURBOPUMP_SIZE[1] / 2, 0.64, 0.18),
                        _EQ["turbopump"], (0.85, 0.2, 0.6)),
    "power_electronics": ("box", (1.60, 0.38, 0.36), (14.37, -0.70, 0.32), _EQ["power_electronics"], (0.9, 0.9, 0.2)),
    # The laser along the spine over the engine face, its turret at the
    # front, just aft of the lift fan.
    "fel":             ("box", (3.00, 0.55, 0.40), (13.24, 0.0, 0.66), _EQ["laser"], (0.2, 0.9, 0.3)),
    "fel_director":    ("cyl_z", (0.60, 0.40), (10.92, 0.0, 0.98), _EQ["laser_turret"], (0.1, 1.0, 0.4)),
    # Stowed in the wing roots, inboard of the bays, as the Mirage 2000's
    # are: the tyre lying flat aft, its leg forward, swinging down to a
    # wide track behind the centre of gravity.
    "main_gear_l":     ("box", (1.30, 0.72, 0.26), (13.10, -1.25, -0.48), 0, (0.3, 0.3, 0.3)),
    "main_gear_r":     ("box", (1.30, 0.72, 0.26), (13.10, 1.25, -0.48), 0, (0.3, 0.3, 0.3)),
    # Two dust missiles side by side in each wing root, ~2.4 m x 0.18 m and
    # ~90 kg each; doors on the belly, sealed for entry like the Shuttle's
    # gear doors. Mass: doors, ejectors, hot seals.
    "bay_l":           ("box", (2.60, 0.50, 0.24), (13.42, -1.85, -0.48), _EQ["bays"] / 2, (0.95, 0.55, 0.1)),
    "bay_r":           ("box", (2.60, 0.50, 0.24), (13.42, 1.85, -0.48), _EQ["bays"] / 2, (0.95, 0.55, 0.1)),
    # The second director, stowed behind belly doors for entry and let down
    # below the belly to strike the ground.
    "laser_turret_ventral": ("cyl_z", (0.60, 0.40), (11.40, 0.0, -0.50), _EQ["laser_turret_ventral"], (0.1, 1.0, 0.4)),
    # Countermeasures. Space decoys under dorsal doors in the middle bay,
    # the jammers below them dropping through the belly, the towed decoys
    # paying out of the fin root above the plume, the dispensers flanking
    # the engine, the jammer's antennas in a fin-tip pod.
    "space_decoys":    ("box", (1.00, 0.60, 0.60), (9.75, 0.0, 0.25), _EQ["space_decoys"], (1.0, 0.3, 0.3)),
    "jammer_pods":     ("box", (1.10, 0.55, 0.30), (9.95, 0.0, -0.45), _EQ["jammer_pods"], (1.0, 0.5, 0.3)),
    "towed_decoys":    ("box", (0.60, 0.25, 0.20), (16.60, 0.0, 0.80), _EQ["towed_decoys"], (1.0, 0.5, 0.5)),
    "chaff_flares_l":  ("box", (0.50, 0.25, 0.20), (16.60, -0.62, -0.40), _EQ["chaff_flares"] / 2, (1.0, 0.7, 0.3)),
    "chaff_flares_r":  ("box", (0.50, 0.25, 0.20), (16.60, 0.62, -0.40), _EQ["chaff_flares"] / 2, (1.0, 0.7, 0.3)),
    "ew_pod":          ("cyl_x", (0.20, 1.10), (17.50, 0.0, 3.22), _EQ["ew"], (1.0, 0.2, 0.2)),
}
# Equipment meant to stand proud of the skin: the laser's turret, the
# nozzle's exit, the fin-tip pod.
EXTERNAL = ("fel_director", "swivel_nozzle", "ew_pod")
# Sensor windows, wide-angle imaging and IR cameras that give the pilot
# the view all round and warn of missiles: (x as a share of the length,
# angle round the body from straight up toward starboard, degrees, and
# how far forward of the skin's normal it looks). Each sees
# SENSOR_HALF_ANGLE either side of where it looks; the nose's windows are
# faceted, angled forward, as the F-35's are, the nose being too slender
# for a flush window to see ahead. The tail is watched from the fin-tip
# pod's aft end.
SENSORS = [(0.16, 45, 35), (0.16, -45, 35), (0.16, 135, 35), (0.16, -135, 35), (0.55, 0, 0), (0.55, 180, 0),
           (0.80, 80, 0), (0.80, -80, 0), (0.92, 135, 0), (0.92, -135, 0)]
SENSOR_HALF_ANGLE = 65.0
# Inlet ducts: semicircular capture each side, then a round duct of the
# same area to the engine face, passing the lift fan. (x, y, z) waypoints.
# The capture is a half-circle, its area the duct's, its flat side a
# splitter plate standing `gap` off the body's flank: the boundary layer
# the forebody has grown by the mouth, ~0.37 x / Re^0.2, passes between
# them instead of into the engine. The trunk round the capture is the
# capture and a thin lip, so that the mouth adds no blunt face to the area
# distribution; a wedge from the body carries it, starting a little
# behind the lip and spilling the diverted air above and below.
# Strakes on the intake trunks, as on the Mirage 2000-5: small swept
# plates whose vortices keep the flow over the wing root and fin in order
# at high angles of attack. (x behind the mouth, length, span, height
# above the duct's axis.)
STRAKE = {"from": 1.0, "length": 0.9, "span": 0.25, "z": 0.22}
# Canards, or None: all-moving, close-coupled as on the Rafale, on the
# intake trunks just behind their lips, ahead of the wing's root and above
# it; their vortices do the strakes' work, so they replace them. (x of the
# root's leading edge, root chord, exposed semispan from `root_y`,
# leading-edge sweep, taper, thickness/chord, height.)
CANARD = {"x": 7.10, "root": 2.30, "span": 1.95, "root_y": 1.20, "sweep": 50.0, "taper": 0.35,
          "tc": 0.04, "z": 0.22}
INLET = {"area": 0.30, "gap": 0.07, "diverter": (0.30, 1.20), "face_x": ENGINE_FACE_X, "mouth_x": 6.90, "fair_x": 11.00, "lip": 0.025}
# Behind their S-bend the two ducts run on side by side, a bifurcated duct,
# to the fan face.
DUCT_PATH = [(6.76, 1.24, -0.05), (8.17, 1.26, -0.05), (9.30, 1.20, -0.05), (10.70, 0.90, -0.05), (12.11, 0.40, -0.05),
             (12.95, 0.20, -0.05), (ENGINE_FACE_X, 0.20, -0.05)]

# Tank regions: name -> box (x0, x1, y0, y1, z0, z1), or "wing" for the wing
# box outboard of the fuselage between these chord fractions.
TANKS = {
    "forward":   ("box", (3.27, 8.24, -1.3, 1.3, -0.7, 0.95)),
    "middle":    ("box", (9.13, 12.18, -1.4, 1.4, -0.75, 1.10)),
    "saddle":    ("box", (12.18, 17.75, -1.4, 1.4, -0.75, 1.10)),
    "wing":      ("wing", (0.15, 0.75)),
}
TANK_FILL = 0.75  # of a tank region's free volume: frames, baffles, feeds, and the systems that thread between

# Equipment the list does not place, kg, taken at the centre of gravity.
# Flush hardpoints: no pylons in the flow on entry.
FIXED = {n: _EQ[n] for n in ("inlets_and_ducts", "propellant_system", "flight_controls_and_rcs",
                              "thermal", "electrical", "sensor_windows", "slats_and_strakes")}
# The design manoeuvre's mass, and the heaviest the aircraft lands at.
DESIGN_KG = fighter_structure.DESIGN_KG
LANDING_KG = 21000.0

C_SKIN = (0.55, 0.62, 0.70)
C_TANK = (0.25, 0.45, 0.95)
C_CFT = (0.30, 0.55, 1.00)
C_DUCT = (0.75, 0.75, 0.75)


# The fuselage as drawn, before any stretch.
_DRAWN = copy.deepcopy((FUSELAGE, SECTION, EQUIPMENT, DUCT_PATH, INLET, TANKS, WING, FIN))
CG_DRAWN = 11.56  # full, as drawn


def stretch(k):
    """Lengthen the fuselage by k: its stations, what it carries and its
    tanks spread along x; sections, equipment, wing and fin keep their
    size. The wing and its bays move with the centre of gravity, so that
    the trim is unchanged; the fin moves with the tail."""
    fus, sec, eq, duct, inlet, tanks, wing, fin = copy.deepcopy(_DRAWN)
    shift = CG_DRAWN * (k - 1)
    FUSELAGE[:] = [(x * k, *r) for x, *r in fus]
    SECTION[:] = [(x * k, *r) for x, *r in sec]
    EQUIPMENT.clear()
    for name, (kind, size, cen, mass, color) in eq.items():
        x = cen[0] + shift if name.startswith("bay") else cen[0] * k
        EQUIPMENT[name] = (kind, size, (x, cen[1], cen[2]), mass, color)
    DUCT_PATH[:] = [(x * k, y, z) for x, y, z in duct]
    INLET.update({key: inlet[key] * k for key in ("face_x", "mouth_x", "fair_x")})
    for name, (kind, spec) in tanks.items():
        TANKS[name] = (kind, (spec[0] * k, spec[1] * k, *spec[2:]) if kind == "box" else spec)
    WING["apex"] = wing["apex"] + shift
    FIN["root_x"] = fin["root_x"] * k


def args():
    a = sys.argv[sys.argv.index("--") + 1:] if "--" in sys.argv else []
    out = {"scale": SCALE, "save": None, "render": None}
    for i, k in enumerate(a):
        if k == "--stretch":
            stretch(float(a[i + 1]))
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


def half_thickness(chord, tc, fullness, u, peak=0.5):
    """Half the section's thickness at chord fraction u: biconvex at
    fullness 1, fuller below; thickest at `peak` of the chord."""
    v = u ** (math.log(0.5) / math.log(peak))
    return 0.5 * tc * chord * (4 * v * (1 - v)) ** fullness


def section(chord, tc, fullness, peak, n=24):
    """Upper surface, (x, z) from leading to trailing edge."""
    return [(chord * i / n, half_thickness(chord, tc, fullness, i / n, peak)) for i in range(n + 1)]


def droop(ring, chord, fraction, angle):
    """`ring`, (x, z) from the leading edge, its front `fraction` of the
    chord turned nose down `angle` radians about the hinge there."""
    h = fraction * chord
    c, s = math.cos(angle), math.sin(angle)
    return [(x, z) if x >= h else (h + (x - h) * c - z * s, z * c + (x - h) * s) for x, z in ring]


def surface(name, sections, coll, color, vertical=False, alpha=1.0, peak=0.5, slat=None):
    """Closed mesh through airfoil sections: (x_le, span, chord, tc, base,
    fullness). `slat(span)`, if given, is the (chord fraction, radians) of
    the leading edge's droop there."""
    verts, faces = [], []
    n = 96
    for x_le, s, chord, tc, base, fullness in sections:
        up = section(chord, tc, fullness, peak, n)
        ring = [(x, z) for x, z in up] + [(x, -z) for x, z in reversed(up[1:-1])]
        if slat:
            ring = droop(ring, chord, *slat(s))
        ring = [(x_le + x, z) for x, z in ring]
        if vertical:
            ring.reverse()
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


def lathe(name, profile, coll, color, arc=None, k=48, centre=(0.0, 0.0)):
    """A surface of revolution about X through (x, r), over the whole turn
    or over `arc`, (start, sweep) radians; a profile that starts and ends
    on the axis makes a closed solid."""
    if arc:
        angles = [arc[0] + arc[1] * i / k for i in range(k + 1)]
    else:
        angles = [2 * math.pi * i / k for i in range(k)]
    cy, cz = centre
    verts = [(x, cy + r * math.cos(t), cz + r * math.sin(t)) for t in angles for x, r in profile]
    n, faces = len(profile), []
    for j in range(len(angles) - (1 if arc else 0)):
        a0, a1 = j * n, ((j + 1) % len(angles)) * n
        for i in range(n - 1):
            faces.append((a0 + i, a0 + i + 1, a1 + i + 1, a1 + i))
    return mesh_object(name, verts, faces, coll, color)


def radius_at(profile, x):
    """A lathe profile's radius at x, 0 outside it."""
    for (x0, r0), (x1, r1) in zip(profile, profile[1:]):
        if x0 <= x <= x1 and x1 > x0:
            return r0 + (r1 - r0) * (x - x0) / (x1 - x0)
    return 0.0


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
    # Half-cone inlets on the flanks: a half-round trunk off the body's
    # flank, the capture and its lip, its flat side the splitter plate,
    # following the duct in as it turns to the engine, so that the body
    # takes it in. A little behind the lip the diverter joins it to the
    # body through the gap, a wedge in side view: the lip standing free is
    # one piece, the trunk and its neck another, overlapping.
    nacelles = []
    r = inlet_radius(s) + INLET["lip"] * s
    x0, x1 = INLET["mouth_x"] * s, INLET["fair_x"] * s
    d0, d1 = (d * s for d in INLET["diverter"])
    plate = lambda x: splitter_y(s) + duct_point(x, s)[0] - duct_point(x0, s)[0]
    half = [math.pi * (j / 24 - 0.5) for j in range(25)]
    span = lambda a, b, n: [a + (b - a) * j / n for j in range(1, n)]

    def lip_ring(x):
        y, z, q = plate(x) + 0.004 * s, duct_point(x, s)[1], r - 0.004 * s
        return [(y + q * math.cos(t), z + q * math.sin(t)) for t in half] + \
               [(y, z + q * (1 - 2 * j / 8)) for j in range(1, 8)]

    def trunk_ring(x):
        y, z = plate(x), duct_point(x, s)[1]
        h = 0.9 * r * min(1.0, 0.15 + 0.85 * (x - x0 - d0) / d1)
        inner = min(hw_at(x, s), y) - 0.2 * s
        return [(y + r * math.cos(t), z + r * math.sin(t)) for t in half] + \
               [(y, v) for v in span(z + r, z + h, 3)] + [(y, z + h)] + \
               [(u, z + h) for u in span(y, inner, 4)] + [(inner, z + h)] + \
               [(inner, v) for v in span(z + h, z - h, 4)] + [(inner, z - h)] + \
               [(u, z - h) for u in span(inner, y, 4)] + [(y, z - h)] + \
               [(y, v) for v in span(z - h, z - r, 3)]

    for side in (-1, 1):
        for name, xa, xb, ring in (("lip", x0, x0 + d0 + 0.1 * s, lip_ring), ("trunk", x0 + d0, x1, trunk_ring)):
            rings = [[(x, side * a, b) for a, b in ring(x)] for x in [xa + (xb - xa) * i / 23 for i in range(24)]]
            if side < 0:
                rings = [list(reversed(q)) for q in rings]
            nacelles.append(ring_loft(f"inlet_{name}_{side}", rings, skin, C_SKIN))
        if CANARD:
            continue
        st = STRAKE
        sx0 = x0 + st["from"] * s
        srings = []
        for i in range(8):
            x = sx0 + st["length"] * s * i / 7
            zs = duct_point(x, s)[1] + st["z"] * s
            root_y = plate(x) + math.sqrt(max(r * r - (st["z"] * s) ** 2, 0.0)) - 0.05 * s
            tip = root_y + 0.05 * s + max(0.01, st["span"] * s * i / 7)
            h = 0.01 * s
            srings.append([(x, side * a, b) for a, b in ((root_y, zs - h), (tip, zs - h), (tip, zs + h), (root_y, zs + h))])
        if side < 0:
            srings = [list(reversed(q)) for q in srings]
        nacelles.append(ring_loft(f"strake_{side}", srings, skin, C_SKIN))
    w = WING
    tan = math.tan(math.radians(w["sweep_le"]))
    chord_at = lambda y: max(0.3, w["root"] - abs(y) * tan) * s
    le_at = lambda y: (w["apex"] + abs(y) * tan) * s
    half = [(y * s, tc, p) for y, tc, p in w["sections"]]
    slat = None
    if SLAT["deflection"]:
        sl, semi = SLAT, w["semispan"] * s
        y0, y1 = sl["from"] * semi, sl["to"] * semi
        between = lambda y, k: next(a[k] + (b[k] - a[k]) * (y - a[0]) / (b[0] - a[0])
                                    for a, b in zip(half, half[1:]) if a[0] <= y <= b[0])
        ends = (y0 - 0.02 * semi, y0, y1, y1 + 0.02 * semi)
        half = sorted(half + [(y, between(y, 1), between(y, 2)) for y in ends])
        hinge = math.atan((1 - sl["chord"]) * tan)
        angle = math.atan(math.tan(math.radians(sl["deflection"])) * math.cos(hinge))
        slat = lambda y: (sl["chord"], angle if y0 - 1e-6 <= abs(y) <= y1 + 1e-6 else 0.0)
    spanwise = [(-y, tc, p) for y, tc, p in reversed(half[1:])] + half
    wing = surface("wing", [(le_at(y / s), y, chord_at(y / s), tc, (0, 0, w["z"] * s), p)
                            for y, tc, p in spanwise], skin, C_SKIN, peak=w["peak"], slat=slat)
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

    # Canards: separate surfaces, so that the wing-body's centre of lift
    # stays the wing's; their own lift is the aero model's to place.
    canards, canard = [], None
    if CANARD:
        c = CANARD
        ctan = math.tan(math.radians(c["sweep"]))
        tip = c["root"] * c["taper"]
        y0, y1 = c["root_y"] * s, (c["root_y"] + c["span"]) * s
        root_s = (c["x"] * s, y0, c["root"] * s, c["tc"], (0, 0, c["z"] * s), 1.0)
        tip_s = ((c["x"] + c["span"] * ctan) * s, y1, tip * s, c["tc"], (0, 0, c["z"] * s), 1.0)
        for side in (-1, 1):
            secs = [root_s, tip_s] if side > 0 else [(tip_s[0], -y1, *tip_s[2:]), (root_s[0], -y0, *root_s[2:])]
            canards.append(surface(f"canard_{side}", secs, skin, C_SKIN))
        area_c = 2 * (c["root"] + tip) / 2 * c["span"] * s * s
        ymac_c = c["span"] / 3 * (1 + 2 * c["taper"]) / (1 + c["taper"])
        mac_c = 2 / 3 * c["root"] * (1 + c["taper"] + c["taper"] ** 2) / (1 + c["taper"])
        canard = {"area": area_c, "aspect": (2 * c["span"] * s) ** 2 / area_c, "sweep": c["sweep"],
                  "x_ac": (c["x"] + ymac_c * ctan + 0.25 * mac_c) * s}
    canard_a = sum(volume(o)[1] for o in canards)

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
        elif kind == "lathe":
            prof = [(cen[0] + x, r) for x, r in size]
            o = lathe(name, prof, gear, color, centre=cen[1:])
            grid = [(x, cen[1] + f * r * math.cos(t), cen[2] + f * r * math.sin(t))
                    for x, r in prof for f in (0, 0.5, 1) for t in [i * math.pi / 4 for i in range(8)]]
            vol = sum(math.pi * (x1 - x0) * (r0 * r0 + r0 * r1 + r1 * r1) / 3
                      for (x0, r0), (x1, r1) in zip(prof, prof[1:]))
            cen = (sum(math.pi * (x1 - x0) * (r0 * r0 + r1 * r1) / 2 * (x0 + x1) / 2
                       for (x0, r0), (x1, r1) in zip(prof, prof[1:])) / vol, *cen[1:])
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
            elif kind == "lathe":
                r = radius_at(size, x - cx)
                if (y - cen[1]) ** 2 + (z - cen[2]) ** 2 <= r * r:
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
    if canard:
        lay.append((_EQ["canards"], canard["x_ac"]))
        eq_mass += _EQ["canards"]
    fuel_x = (tanks_v * weighted_x(tank) + ext_v * cft_x) / (tanks_v + ext_v)
    bays = [e for n, e in eq.items() if n.startswith("bay")]
    bay_v = sum(e["volume"] for e in bays)
    fin_area = (f["root"] + max(0.5, f["root"] - f["height"] * ftan)) / 2 * f["height"] * s * s

    def structure_for(empty_guess):
        geo = {
            "design_mass": DESIGN_KG,
            "landing_mass": LANDING_KG,
            "wing": {k: (v * s if k in ("apex", "root", "semispan") else v) for k, v in w.items()},
            "y_in": hw_at((w["apex"] + w["root"] / 2) * s, s),
            "sections": [(x, fuselage_section(*station(x, s))) for x in fuselage_stations(s, 60)],
            "fuselage_masses": lay + [(max(0.0, DESIGN_KG - empty_guess), fuel_x)],
            "lift_x": None,
            "fin_area": fin_area,
            "internal_tank_surface": sum(6 * max(t["volume"], 0.01) ** (2 / 3) for t in tank.values()),
            "cft_surface": cft_a,
            "wetted": oml_a + cft_a + canard_a,
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
    if canard:
        items["canards"] = (_EQ["canards"], canard["x_ac"])
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
    ac_sub = le_at(y_mac / s) + w["ac"] * mac
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
    nozzle = eq["swivel_nozzle"]["obj"]
    length = max((nozzle.matrix_world @ Vector(c)).x for c in nozzle.bound_box)

    # What the pilot and the directors can see past the airframe.
    blockers = [tree, bvh(nozzle), bvh(eq["ew_pod"]["obj"]), *[bvh(o) for o in canards]]
    eyes = []
    for f, a, tilt in SENSORS:
        p, n = on_skin(tree, f * FUSELAGE[-1][0] * s, a)
        t = math.radians(tilt)
        eyes.append((p, (n * math.cos(t) + Vector((-1.0, 0.0, 0.0)) * math.sin(t)).normalized()))
    pod = EQUIPMENT["ew_pod"]
    eyes.append((Vector((pod[2][0] * s + pod[1][1] / 2 + 0.01, 0.0, pod[2][2])), Vector((1.0, 0.0, 0.0))))
    dirs = sphere(2000)
    seen, gaps = coverage(blockers, eyes, SENSOR_HALF_ANGLE, dirs)
    gap_dirs = [(round(math.degrees(math.atan2(d.y, d.x))), round(math.degrees(math.asin(d.z)))) for d in gaps[:8]]
    regard = {}
    for name, up in (("fel_director", 1.0), ("laser_turret_ventral", -1.0)):
        cx, cy, cz = EQUIPMENT[name][2]
        base, _ = on_skin(tree, cx * s, 0.0 if up > 0 else 180.0)
        point = base + Vector((0.0, 0.0, 0.25 * up))
        regard[name] = tuple(round(v, 2) for v in field_of_regard(blockers, point, Vector((0, 0, up)), dirs))
    dist = area_distribution([fus, *nacelles, wing, fin, *canards, nozzle, eq["fel_director"]["obj"]], length)
    dq, dq_sh, v_eff = wave_drag(dist, 2 * INLET["area"], INLET["mouth_x"] * s, length)
    return {
        "scale": s,
        "length": FUSELAGE[-1][0] * s,
        "span": 2 * w["semispan"] * s,
        "wing_area": planform(s),
        "airframe_volume": oml_v,
        "wetted_area": oml_a + cft_a + canard_a,
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
        "elevon_x": le_at(y_mac / s) + 0.89 * mac,
        "canard": canard,
        "ac_subsonic": ac_sub,
        "cp_super_hyper": cp_high,
        "trim (cg, subsonic margin %MAC, high-speed margin %MAC, nozzle/fan)": margins,
        "sensor_coverage": (round(seen, 3), gap_dirs),
        "director_regard (upper, lower hemisphere)": regard,
        "area_peak": max(dist, key=lambda d: d[1]),
        "wave_drag (D/q Mach 1 m2, Sears-Haack, E_WD)": (round(float(dq), 2), round(float(dq_sh), 2), round(float(dq / dq_sh), 2)),
        "area_distribution": dist,
    }


def area_distribution(objs, x_end, dx=0.05):
    """Cross-sectional area of the union of `objs` at stations dx apart:
    [(x, m2)]. Each station cuts the union's triangles with its plane and
    sums the cut polygon's area, each segment oriented by its triangle's
    outward normal, so the area is exact for the closed mesh."""
    import numpy as np
    whole = union("area_body", objs, bpy.context.scene.collection, C_SKIN, 0.0)
    bm = bmesh.new()
    bm.from_mesh(whole.data)
    bm.transform(whole.matrix_world)
    bmesh.ops.triangulate(bm, faces=bm.faces[:])
    bm.verts.ensure_lookup_table()
    v = np.array([vert.co[:] for vert in bm.verts])
    tri = np.array([[vert.index for vert in f.verts] for f in bm.faces])
    bm.free()
    bpy.data.objects.remove(whole)
    a, b, c = v[tri[:, 0]], v[tri[:, 1]], v[tri[:, 2]]
    n = np.cross(b - a, c - a)
    out = []
    for i in range(int(x_end / dx) + 2):
        x = i * dx + 1e-7
        da, db, dc = a[:, 0] - x, b[:, 0] - x, c[:, 0] - x
        hit = (np.minimum(np.minimum(da, db), dc) < 0) & (np.maximum(np.maximum(da, db), dc) > 0)
        if not hit.any():
            out.append((i * dx, 0.0))
            continue
        pts = []
        for (p, dp), (q, dq) in (((a, da), (b, db)), ((b, db), (c, dc)), ((c, dc), (a, da))):
            cross = hit & ((dp < 0) != (dq < 0))
            t = np.where(cross, dp / np.where(cross, dp - dq, 1.0), 0.0)
            pts.append((cross, p + (q - p) * t[:, None]))
        first = np.where(pts[0][0][:, None], pts[0][1], pts[1][1])
        second = np.where(pts[2][0][:, None], pts[2][1], pts[1][1])
        p0, p1, nn = first[hit][:, 1:], second[hit][:, 1:], n[hit]
        seg = p1 - p0
        flip = seg[:, 0] * nn[:, 2] - seg[:, 1] * nn[:, 1] < 0
        p0, p1 = np.where(flip[:, None], p1, p0), np.where(flip[:, None], p0, p1)
        out.append((i * dx, abs(0.5 * np.sum(p0[:, 0] * p1[:, 1] - p0[:, 1] * p1[:, 0]))))
    return out


def wave_drag(dist, capture, mouth_x, length, terms=24, smooth=0.15):
    """Mach-1 wave drag, D/q in m2, of the area distribution less the inlet
    capture aft of the mouth (the stream tube is not the body), by von
    Karman's slender-body series; and of the Sears-Haack body of the same
    length and volume. The distribution is smoothed over `smooth` m first:
    the series charges every kink, and a faceted mesh is all kinks."""
    xs = [x for x, _ in dist if x <= length]
    dx = xs[1] - xs[0]
    raw = [max(0.0, s - (capture if x > mouth_x + dx / 2 else 0.0)) for x, s in dist[:len(xs)]]
    taps = range(-int(3 * smooth / dx), int(3 * smooth / dx) + 1)
    w = [math.exp(-0.5 * (j * dx / smooth) ** 2) for j in taps]
    a = [sum(wj * raw[min(max(i + j, 0), len(raw) - 1)] for wj, j in zip(w, taps)) / sum(w)
         for i in range(len(raw))]
    vol = sum((a[i] + a[i + 1]) / 2 * (xs[i + 1] - xs[i]) for i in range(len(xs) - 1))
    slope = [(a[min(i + 1, len(a) - 1)] - a[max(i - 1, 0)]) / (xs[min(i + 1, len(a) - 1)] - xs[max(i - 1, 0)])
             for i in range(len(a))]
    k = 2000
    th = [math.pi * j / k for j in range(k + 1)]
    f = []
    for t in th:
        x = length / 2 * (1 - math.cos(t))
        i = min(int(x / (xs[1] - xs[0])), len(xs) - 2)
        u = (x - xs[i]) / (xs[i + 1] - xs[i])
        f.append(slope[i] * (1 - u) + slope[i + 1] * u)
    dq = 0.0
    for n in range(1, terms + 1):
        b = 2 / math.pi * sum((f[j] * math.sin(n * th[j]) + f[j + 1] * math.sin(n * th[j + 1])) / 2 * (th[1] - th[0])
                              for j in range(k))
        dq += math.pi / 4 * n * b * b
    return dq, 128 * vol * vol / (math.pi * length ** 4), vol


def splitter_y(s):
    """The splitter plate's distance from the centreline."""
    return hw_at(INLET["mouth_x"] * s, s) + INLET["gap"] * s


def inlet_radius(s):
    """The half-circle capture's radius."""
    return math.sqrt(2 * INLET["area"] / math.pi) * s


def sphere(n):
    """n directions spread evenly over the sphere."""
    out, g = [], math.pi * (3 - math.sqrt(5))
    for i in range(n):
        z = 1 - 2 * (i + 0.5) / n
        r = math.sqrt(1 - z * z)
        out.append(Vector((r * math.cos(g * i), r * math.sin(g * i), z)))
    return out


def on_skin(tree, x, angle):
    """The skin's point and outward normal at station x, `angle` degrees
    round from straight up toward starboard, cast from the axis."""
    a = math.radians(angle)
    d = Vector((0.0, math.sin(a), math.cos(a)))
    origin = Vector((x, 0.0, 0.0))
    hit, last = tree.ray_cast(origin, d), None
    while hit[0] is not None:
        last = hit
        hit = tree.ray_cast(hit[0] + d * 1e-4, d)
    n = last[1] if last[1].dot(d) > 0 else -last[1]
    return last[0], n


def visible(trees, point, d):
    """Is direction d open to the sky from point, past everything in trees?"""
    return all(t.ray_cast(point + d * 0.03, d)[0] is None for t in trees)


def coverage(trees, eyes, half_angle, dirs):
    """Share of `dirs` that some eye (point, normal) sees within
    half_angle of its normal, unblocked; and the directions none sees."""
    c = math.cos(math.radians(half_angle))
    seen, gaps = 0, []
    for d in dirs:
        if any(n.dot(d) >= c and visible(trees, p, d) for p, n in eyes):
            seen += 1
        else:
            gaps.append(d)
    return seen / len(dirs), gaps


def field_of_regard(trees, point, up, dirs, below=10.0):
    """The share of the upper and of the lower hemisphere a director at
    `point` can reach: anything within `below` degrees past the plane its
    mount faces, unblocked."""
    lim = -math.sin(math.radians(below))
    reach = [d for d in dirs if d.dot(up) >= lim and visible(trees, point, d)]
    upper = sum(1 for d in reach if d.z > 0) / sum(1 for d in dirs if d.z > 0)
    lower = sum(1 for d in reach if d.z < 0) / sum(1 for d in dirs if d.z < 0)
    return upper, lower


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


def frame_and_render(directory, length):
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
    mid, wide = length / 2, length * 1.12
    views = {
        "top": ((mid, 0.0, 30.0), (0.0, 0.0, 0.0), wide),
        "side": ((mid, -30.0, 0.5), (math.pi / 2, 0.0, 0.0), wide),
        "front": ((-30.0, 0.0, 0.5), (math.pi / 2, 0.0, -math.pi / 2), 11.0),
    }
    for name, (loc, rot, ortho) in views.items():
        cam.location = loc
        cam.rotation_euler = rot
        cam_data.ortho_scale = ortho
        scene.render.filepath = f"{directory}/fighter_{name}.png"
        bpy.ops.render.render(write_still=True)
    cam_data.type = "PERSP"
    cam.location = (mid - 13.0, -19.0, 12.0)
    direction = Vector((mid, 0.0, 0.0)) - cam.location
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
        if k != "area_distribution":
            print(f"{k}: {v}")
    if opts["save"]:
        bpy.ops.wm.save_as_mainfile(filepath=opts["save"])
    if opts["render"]:
        frame_and_render(opts["render"], result["length"])


if __name__ == "__main__":
    main()
