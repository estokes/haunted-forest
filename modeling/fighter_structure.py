# The fighter's structure sized from its loads (book/small-craft.md). Pure
# Python, so it runs inside Blender (modeling/fighter.py) or without it.
#
# Near-ideal CNT, knocked down for heat, damage and fatigue. The design
# manoeuvre is N_LIMIT g at DESIGN_KG (n W held constant, so a heavier
# aircraft pulls less), times 1.5 for ultimate. The wing box's covers carry
# bending in compression, its webs shear; lift spreads over the exposed
# wing as its chord does. The carry-through crosses the fuselage around the
# engine. The fuselage is a thin shell on stringers carrying the bending of
# its own and its equipment's inertia against the lift. Wherever the loads
# ask for less than minimum gauge, minimum gauge it is: on this aircraft
# that is often what decides the mass. The low-aspect-ratio delta is taken
# to be flutter-free on that stiffness, an assumption to revisit.

import math

CNT = {
    "density": 1400.0,          # kg/m3
    "tension": 30e9,            # Pa, bulk unidirectional
    "compression": 8e9,         # tubes buckle before they break
    "shear": 5e9,               # +-45 webs
    "modulus": 1.0e12,
    "knockdown": 0.67,          # heat, damage tolerance, fatigue
}
N_LIMIT = 20.0
DESIGN_KG = 15000.0
ULTIMATE = 1.5
MIN_GAUGE = 0.6e-3              # m, any skin or web
BELLY_GAUGE = 1.0e-3            # m, the windward skin: gravel off dirt strips, debris
CORE = 1.3                      # sandwich core and stringers, on skins and covers
CUTOUT = 1.3                    # fuselage doors, fan shaft, inlets, gear bays
FRAMES = 0.25                   # of the fuselage shell
CARRY_THROUGH_DETOUR = 1.5      # the carry-through splits round the engine
WING_LIFT_SHARE = 0.65          # the rest is the wide body's
COATING_KG_M2 = 1.0             # carbide skin, everywhere: hot oxygen on every surface in entry
BELLY_INSULATION = (15.0, 8.0)  # m2 over belly tanks, kg/m2 (5 cm)
OTHER_INSULATION = (20.0, 3.0)  # m2 of hot skin over equipment, kg/m2 (2 cm)
GEAR_FRACTION = 0.035           # of the heaviest landing mass, on dirt
MOUNTS = 0.03                   # equipment mounts at ultimate, of the equipment


def sigma(kind):
    return CNT[kind] * CNT["knockdown"]


def wing(w, y_in, load):
    """Both wing panels outboard of y_in and the carry-through between them.
    `w` is the wing: apex, root chord, semispan, sweep (deg), t/c. `load`
    is the ultimate lift on the wing, N. Returns kg and the root bending
    moment, N m, and where the panels hit minimum gauge."""
    rho = CNT["density"]
    tan = math.tan(math.radians(w["sweep_le"]))
    chord = lambda y: max(0.3, w["root"] - y * tan)
    b2 = w["semispan"]
    n = 60
    ys = [y_in + (b2 - y_in) * (i + 0.5) / n for i in range(n)]
    dy = (b2 - y_in) / n
    area = sum(chord(y) for y in ys) * dy
    lift = [load / 2 * chord(y) / area for y in ys]  # N per m, one side
    mass, gauge_from = 0.0, None
    root_m = 0.0
    for i, y in enumerate(ys):
        v = sum(lift[j] * dy for j in range(i, n))
        m = sum(lift[j] * dy * (ys[j] - y) for j in range(i, n))
        if i == 0:
            root_m = m + v * dy / 2
        c = chord(y)
        depth = 0.85 * w["tc"] * c
        lever = 0.9 * depth
        box = 0.6 * c
        t_cover = max(MIN_GAUGE, m / (lever * box * sigma("compression")))
        if t_cover == MIN_GAUGE and gauge_from is None:
            gauge_from = y
        t_web = max(3 * MIN_GAUGE, v / (depth * sigma("shear")))
        covers = 2 * box * t_cover * CORE
        webs = depth * t_web
        ribs = 2.0 * 0.7 * c * depth * MIN_GAUGE  # one every 0.5 m
        edges = 2 * 0.4 * c * MIN_GAUGE * CORE
        mass += rho * (covers + webs + ribs + edges) * dy
    mass *= 2
    # Carry-through: the root moment across the fuselage.
    c0 = chord(y_in)
    depth0 = 0.85 * w["tc"] * c0
    t0 = max(MIN_GAUGE, root_m / (0.9 * depth0 * 0.6 * c0 * sigma("compression")))
    carry = rho * 2 * 0.6 * c0 * t0 * CORE * 2 * y_in * CARRY_THROUGH_DETOUR
    return {"panels": mass, "carry_through": carry, "root_moment": root_m,
            "root_cover_mm": t0 * 1e3, "gauge_from_y": gauge_from}


def section_props(pts):
    """Perimeter, and the second moment about the horizontal axis per metre
    of wall thickness, of a section given as a ring of (y, z); and the
    farthest fibre from its centroid."""
    k = len(pts)
    zc = sum(z for _, z in pts) / k
    per, i_t = 0.0, 0.0
    for (y0, z0), (y1, z1) in zip(pts, pts[1:] + pts[:1]):
        ds = math.hypot(y1 - y0, z1 - z0)
        zm = (z0 + z1) / 2 - zc
        per += ds
        i_t += zm * zm * ds
    return per, i_t, max(abs(z - zc) for _, z in pts)


def fuselage(sections, masses, lift_x, n_g):
    """Shell and frames. `sections` are (x, ring of (y, z)) along the
    fuselage; `masses` are (kg, x) of everything the fuselage carries; the
    lift, n_g times their weight, acts at `lift_x`; the bending is what
    inertia and lift leave."""
    rho = CNT["density"]
    g = 9.80665
    total = sum(m for m, _ in masses)
    x_end = sections[-1][0]
    n = 80
    xs = [x_end * (i + 0.5) / n for i in range(n)]
    dx = x_end / n

    def moment(x):
        """Bending at x from everything forward of it."""
        mom = sum(m * n_g * g * (x - xm) for m, xm in masses if xm < x)
        if lift_x < x:
            mom -= total * n_g * g * (x - lift_x)
        return abs(mom)

    def nearest(x):
        return min(sections, key=lambda sec: abs(sec[0] - x))[1]

    shell, governed = 0.0, 0
    for x in xs:
        per, i_t, zmax = section_props(nearest(x))
        need = moment(x) * zmax / (i_t * sigma("compression")) if i_t > 0 else 0.0
        t = max(MIN_GAUGE * 0.6 + BELLY_GAUGE * 0.4, need)
        governed += need > MIN_GAUGE
        shell += rho * per * t * CORE * CUTOUT * dx
    return {"shell": shell, "frames": FRAMES * shell, "loads_govern_share": governed / n,
            "peak_moment": max(moment(x) for x in xs)}


def fin(area):
    return CNT["density"] * 2 * area * MIN_GAUGE * CORE * 1.5


def tank_walls(surface):
    """Liners or conformal walls of `surface` m2: minimum gauge sandwich,
    with baffles and fittings half again. Twenty g of dust shot asks for
    microns."""
    return CNT["density"] * surface * MIN_GAUGE * CORE * 1.5


def structure(geo):
    """Every structural mass, kg, for the geometry in `geo`."""
    n_g = N_LIMIT * DESIGN_KG / geo["design_mass"] * ULTIMATE
    lift = geo["design_mass"] * 9.80665 * n_g
    w = wing(geo["wing"], geo["y_in"], lift * WING_LIFT_SHARE)
    f = fuselage(geo["sections"], geo["fuselage_masses"], geo["lift_x"], n_g)
    out = {
        "wing_panels": w["panels"],
        "carry_through": w["carry_through"],
        "fuselage_shell": f["shell"],
        "fuselage_frames": f["frames"],
        "fin": fin(geo["fin_area"]),
        "internal_tank_walls": tank_walls(geo["internal_tank_surface"]),
        "cft_walls": tank_walls(geo["cft_surface"]),
        "coating": COATING_KG_M2 * geo["wetted"],
        "insulation": BELLY_INSULATION[0] * BELLY_INSULATION[1] + OTHER_INSULATION[0] * OTHER_INSULATION[1],
        "landing_gear": GEAR_FRACTION * geo["landing_mass"],
        "mounts": MOUNTS * geo["equipment_mass"],
    }
    diag = {
        "ultimate_g": n_g,
        "wing_root_moment_MNm": w["root_moment"] / 1e6,
        "wing_root_cover_mm": w["root_cover_mm"],
        "wing_min_gauge_from_y": w["gauge_from_y"],
        "fuselage_peak_moment_MNm": f["peak_moment"] / 1e6,
        "fuselage_load_governed_share": f["loads_govern_share"],
    }
    return out, diag
