# The engine's geometry from hf-engine's design, without Blender: for the
# 3D model (modeling/engine.py) and for the blade-passage CFD meshes.
# Regenerate the design with
#   hf-engine --flowpath --json modeling/data/flowpath.json
#
# Engine coordinates: x along the axis from the fan face, aft; radius r.
# Blade sections are circular-arc camber lines (metal angle linear in arc
# length) with biconvex thickness, stacked on their centroids along a
# radial line through the row's mid-chord.

import json
import math
import os

DATA = os.path.join(os.path.dirname(os.path.abspath(__file__)), "data", "flowpath.json")
# Thickness at the leading and trailing edges, over chord; rotor tip
# clearance, m; how far a blade's root sinks into its hub, m.
EDGE = 0.01
CLEARANCE = 0.002
ROOT = 0.004
# Axial length over which the bypass duct turns in behind the splitter, m.
TRANSITION = 0.3


def load(path=DATA):
    with open(path) as f:
        return json.load(f)


def at_radius(sections, r):
    """A section's metal angles and thickness at radius r, linear between
    the design's hub, mean and tip."""
    keys = ("metal_in", "metal_out", "tc")
    if r <= sections[0]["r"]:
        return {k: sections[0][k] for k in keys}
    for a, b in zip(sections, sections[1:]):
        if r <= b["r"]:
            t = (r - a["r"]) / (b["r"] - a["r"])
            return {k: a[k] + (b[k] - a[k]) * t for k in keys}
    return {k: sections[-1][k] for k in keys}


def profile(chord, metal_in, metal_out, tc, side, n=40):
    """Closed section, (a, b): a axial, b tangential arc length, from the
    leading edge along the suction side and back. `side` is the sense, +1
    or -1, in which the flow runs tangentially through the row."""
    k1, k2 = math.radians(metal_in), math.radians(metal_out)
    ds = chord / n
    a = b = 0.0
    line = []
    for i in range(n + 1):
        u = i / n
        k = k1 + (k2 - k1) * u
        line.append((a, b, k, u))
        a += math.cos(k) * ds
        b += side * math.sin(k) * ds
    up, down = [], []
    for a, b, k, u in line:
        h = 0.5 * chord * max(tc * 4 * u * (1 - u), EDGE)
        na, nb = -side * math.sin(k), math.cos(k)
        up.append((a + h * na, b + h * nb))
        down.append((a - h * na, b - h * nb))
    return up + down[::-1]


def centroid(loop):
    a = sum(p[0] for p in loop) / len(loop)
    b = sum(p[1] for p in loop) / len(loop)
    return a, b


def blade(row, theta, stations=7):
    """One blade of `row` at angle `theta`: a list of closed sections, each
    a list of (x, y, z), hub to tip."""
    side = 1 if row["spin"] == 0 else -row["spin"]
    x_mid = (row["x_le"] + row["x_te"]) / 2
    hub = (row["hub"][0] + row["hub"][1]) / 2 - ROOT
    tip = (row["tip"][0] + row["tip"][1]) / 2 - (CLEARANCE if row["spin"] else -ROOT)
    out = []
    for i in range(stations):
        r = hub + (tip - hub) * i / (stations - 1)
        s = at_radius(row["sections"], r)
        loop = profile(row["chord"], s["metal_in"], s["metal_out"], s["tc"], side)
        ca, cb = centroid(loop)
        ring = []
        for a, b in loop:
            t = theta + (b - cb) / r
            ring.append((x_mid + a - ca, r * math.cos(t), r * math.sin(t)))
        out.append(ring)
    return out


def nozzle_profile(air, wet=False):
    """The convergent-divergent nozzle's wall, (x, r), from the reheat duct's
    end: convergent at 30 degrees, divergent at 12."""
    n = air["nozzle"][1 if wet else 0]
    r0, x0 = air["reheat"]["r_out"], air["reheat"]["x1"]
    x_t = x0 + (r0 - n["throat"]) / math.tan(math.radians(30))
    x_e = x_t + (n["exit"] - n["throat"]) / math.tan(math.radians(12))
    return [(x0, r0), (x_t, n["throat"]), (x_e, n["exit"])]


def walls(air):
    """The air path's walls as (x, r) polylines: the core's inner wall (the
    hub, spinner to tail cone), the core's outer wall (splitter to mixer),
    the bypass duct's inner wall, and the outer wall (fan casing, bypass
    duct, reheat duct, nozzle)."""
    rows = air["rows"]
    fan = [r for r in rows if r["component"] == "Fan"]
    comp = [r for r in rows if r["component"] == "Compressor"]
    c, b, m, h = air["combustor"], air["bypass"], air["mixer"], air["reheat"]
    sx, sr = air["splitter"]

    hub0 = fan[0]["hub"][0]
    inner = [(-1.6 * hub0, 0.0), (-0.8 * hub0, 0.75 * hub0), (-0.2 * hub0, 0.97 * hub0)]
    for r in fan:
        inner += [(r["x_le"], r["hub"][0]), (r["x_te"], r["hub"][1])]
    for r in comp:
        inner += [(r["x_le"], r["hub"][0]), (r["x_te"], r["hub"][1])]
    inner += [(c["x0"], c["r_in"]), (c["x1"], c["r_in"]), (m["x"], m["r_cone"]),
              (m["x"] + 2.5 * m["r_cone"], 0.0)]

    core = [(sx, sr)]
    for r in comp:
        core += [(r["x_le"], r["tip"][0]), (r["x_te"], r["tip"][1])]
    core += [(c["x0"], c["r_out"]), (c["x1"], c["r_out"]), (m["x"], m["r_lobes"])]

    outer = [(-0.05, fan[0]["tip"][0])]
    for r in fan:
        outer += [(r["x_le"], r["tip"][0]), (r["x_te"], r["tip"][1])]
    outer += [(b["x0"] + TRANSITION, b["r_out"]), (b["x1"], b["r_out"]), (m["x"], m["r_out"]),
              (h["x1"], h["r_out"])]
    outer += nozzle_profile(air)[1:]
    bypass = [(sx, sr), (b["x0"] + TRANSITION, b["r_in"]), (b["x1"], b["r_in"]), (m["x"], m["r_lobes"])]
    return inner, core, bypass, outer
