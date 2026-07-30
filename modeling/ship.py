# Haunted Forest — parametric ship model generator (v0 greybox).
#
# Living spec. Run against live Blender via the MCP bridge:
#   exec(open('/home/eric/proj/haunted-forest/modeling/ship.py').read())
#
# Cargo hold = an OPEN polar "hard-disk" beam-frame, bonded to the hull (same
# CNT material). All geometry is DERIVED from the container size + clearances +
# structurally-sized member thicknesses (see structural_sizing).
#
# Gravity is along Z and reverses at midcourse. KEY STRUCTURAL FINDING: if every
# container were carried independently by its rails, each rail would carry a full
# fore/aft column (~24 boxes) and buckling would force ~0.36 m solid rails ->
# ~34,000 t of scaffold per bay, heavier than the whole hull. So containers
# STACK-REST against the bay-end bulkheads (which bond into the hull's
# longitudinal frame); the rails are LATERAL restraint + flip retention only, so
# they size down to ~0.1 m and the scaffold is a few kt.

import bpy
import math
import mathutils

TAU = 2 * math.pi
C_CHORD = (0.46, 0.50, 0.60)
C_STRUT = (0.54, 0.58, 0.66)
C_ARC = (0.62, 0.72, 0.55)
C_RAIL = (0.80, 0.62, 0.85)
C_CONN = (0.85, 0.80, 0.55)
C_CORE = (0.34, 0.36, 0.42)
C_CARGO = (0.92, 0.74, 0.20)
C_HOT = (0.95, 0.35, 0.15)
C_DOOR = (0.15, 0.85, 0.35)
C_LOAD = (0.15, 0.72, 0.88)


def arange(lo, hi, step):
    out, x = [], lo
    while x <= hi + 1e-9:
        out.append(round(x, 4)); x += step
    return out


SPEC = {
    "length": 1000.0, "radius_outer": 30.0, "barrel_thickness": 2.0,
    "core_radius": 4.0, "nose_cap_len": 70.0, "tail_len": 30.0,
    "container": {"L": 12.0, "W": 2.5, "H": 2.5}, "container_mass_t": 20.0,
    "clearance": 0.15,
    "n_sectors": 8,
    # --- structural inputs ---
    "max_accel_G": 9.0, "lateral_G": 2.0,
    "material_E_Pa": 500.0e9,        # CNT-composite modulus (assumed)
    "material_strength_Pa": 50.0e9,  # bible: 50-100 GPa tensile
    "material_density": 1400.0,
    "safety_factor": 2.0,
    "cargo_bay_len": 300.0,          # for the full-column sizing case
    "sector_fill": {0: 0.9, 1: 0.0, 2: 0.0, 3: 0.0, 4: 0.0, 5: 0.0, 6: 0.0, 7: 0.75},
    "zones": [
        ("nose_whipple",  40.0,  (0.35, 0.55, 0.85)),
        ("cargo_fore",   300.0,  (0.90, 0.65, 0.15)),
        ("lh2_fore",      90.0,  (0.20, 0.75, 0.90)),
        ("crew_core",     40.0,  (0.90, 0.25, 0.35)),
        ("lh2_aft",       90.0,  (0.20, 0.75, 0.90)),
        ("cargo_aft",    300.0,  (0.90, 0.65, 0.15)),
        ("reactor",       90.0,  (0.70, 0.30, 0.90)),
        ("radiator_tail", 50.0,  (0.90, 0.40, 0.20)),
    ],
}


def structural_sizing(spec):
    g = 9.81
    m = spec["container_mass_t"] * 1000.0
    E = spec["material_E_Pa"]; sig = spec["material_strength_Pa"] / spec["safety_factor"]
    zp = spec["container"]["L"] + 0.5
    F_axial = m * spec["max_accel_G"] * g
    F_lat = m * spec["lateral_G"] * g
    # rail: lateral restraint, simply-supported beam over one bay (zp) span
    M_rail = F_lat * zp / 4.0
    a_rail = (6 * M_rail / sig) ** (1 / 3.0)
    # the REJECTED alternative: independent-lock rail carries a full column in
    # compression, braced every zp -> Euler buckling sets the size
    H = int(spec["cargo_bay_len"] / zp)
    P_col = H * F_axial * spec["safety_factor"]
    I_req = P_col * zp ** 2 / (math.pi ** 2 * E)
    a_lock = (12 * I_req) ** 0.25
    return {
        "F_container_axial_MN": round(F_axial / 1e6, 2),
        "F_container_lateral_MN": round(F_lat / 1e6, 2),
        "z_pitch": zp,
        "rail_side_m": round(max(0.1, a_rail), 3),
        "arc_side_m": round(max(0.1, a_rail) * 1.15, 3),
        "strut_side_m": round(max(0.1, a_rail) * 2.2, 3),
        "conn_side_m": round(max(0.08, a_rail * 0.9), 3),
        "REJECTED_independent_lock_rail_m": round(a_lock, 3),
        "column_load_MN": round(H * F_axial / 1e6, 1),
    }


def _fresh_scene():
    for obj in list(bpy.data.objects):
        bpy.data.objects.remove(obj, do_unlink=True)
    for coll in list(bpy.data.collections):
        bpy.data.collections.remove(coll)
    for block in (bpy.data.meshes, bpy.data.curves, bpy.data.cameras):
        for b in list(block):
            if b.users == 0:
                block.remove(b)


def _collection(name):
    c = bpy.data.collections.new(name)
    bpy.context.scene.collection.children.link(c)
    return c


def _add(obj, coll, color):
    coll.objects.link(obj)
    obj.color = (color[0], color[1], color[2], 1.0)
    return obj


def _box_mesh(name, dx, dy, dz):
    m = bpy.data.meshes.new(name)
    hx, hy, hz = dx / 2, dy / 2, dz / 2
    vs = [(x, y, z) for x in (-hx, hx) for y in (-hy, hy) for z in (-hz, hz)]
    f = [(0, 1, 3, 2), (4, 6, 7, 5), (0, 4, 5, 1), (2, 3, 7, 6), (0, 2, 6, 4), (1, 5, 7, 3)]
    m.from_pydata(vs, [], f); m.update()
    return m


def _cyl_surface(name, radius, z0, z1, coll, color, verts=72, caps=False):
    mesh = bpy.data.meshes.new(name)
    vs, faces = [], []
    for z in (z0, z1):
        for i in range(verts):
            a = TAU * i / verts
            vs.append((radius * math.cos(a), radius * math.sin(a), z))
    for i in range(verts):
        b = (i + 1) % verts
        faces.append((i, b, verts + b, verts + i))
    if caps:
        c0 = len(vs); vs.append((0, 0, z0)); c1 = len(vs); vs.append((0, 0, z1))
        for i in range(verts):
            b = (i + 1) % verts
            faces.append((c0, b, i)); faces.append((c1, verts + i, verts + b))
    mesh.from_pydata(vs, [], faces); mesh.update()
    return _add(bpy.data.objects.new(name, mesh), coll, color)


def _cone(name, radius, z_base, z_apex, coll, color, verts=64):
    mesh = bpy.data.meshes.new(name)
    vs = [(radius * math.cos(TAU * i / verts), radius * math.sin(TAU * i / verts), z_base)
          for i in range(verts)]
    apex = len(vs); vs.append((0, 0, z_apex))
    faces = [(i, (i + 1) % verts, apex) for i in range(verts)]
    mesh.from_pydata(vs, [], faces); mesh.update()
    return _add(bpy.data.objects.new(name, mesh), coll, color)


def P(r, th, z):
    return (r * math.cos(th), r * math.sin(th), z)


def _beam(name, p0, p1, t, coll, color):
    p0v, p1v = mathutils.Vector(p0), mathutils.Vector(p1)
    d = p1v - p0v
    if d.length < 1e-6:
        return None
    o = bpy.data.objects.new(name, _box_mesh(name, t, t, d.length))
    o.location = (p0v + p1v) / 2
    o.rotation_euler = d.to_track_quat('Z', 'Y').to_euler()
    return _add(o, coll, color)


def _box(name, dims, loc, coll, color, rotz=0.0):
    o = bpy.data.objects.new(name, _box_mesh(name, *dims))
    o.location = loc
    if rotz:
        o.rotation_euler = (0, 0, rotz)
    return _add(o, coll, color)


def _arc(name, r, a0, a1, z, t, coll, color, segs=3):
    for i in range(segs):
        _beam(f"{name}_{i}", P(r, a0 + (a1 - a0) * i / segs, z),
              P(r, a0 + (a1 - a0) * (i + 1) / segs, z), t, coll, color)


def build_cargo(z0, z1, colls, spec, sz):
    c_sup, c_arc, c_rail, c_cargo, c_hot, c_door = colls
    inner = spec["radius_outer"] - spec["barrel_thickness"]
    core = spec["core_radius"]
    ns = spec["n_sectors"]; fill = spec["sector_fill"]
    cw = spec["container"]["W"]; cl = spec["clearance"]
    t_rail, t_arc, t_conn, t_strut = sz["rail_side_m"], sz["arc_side_m"], sz["conn_side_m"], sz["strut_side_m"]
    zp = sz["z_pitch"]
    r_out = inner                                    # struts bond to the inner hull
    # DERIVED radial layout: container band (cw) + a ring gap that exactly fits
    # [arc + 2 rails + clearances] between tracks.
    rail_off = t_arc / 2 + cl + t_rail / 2            # rail offset from its arc
    ring_gap = 2 * rail_off + t_rail                  # radial room a ring needs
    ring_pitch = cw + ring_gap
    rings = arange(core + 1.5, inner - 0.4, ring_pitch)   # out to just inside the inner hull
    nz = max(1, int((z1 - z0 - 8) / zp))
    zsts = [z0 + 5 + (k + 0.5) * zp for k in range(nz)]
    frame_z = [z0 + 5 + k * zp for k in range(nz + 1)]
    zc = (z0 + z1) / 2

    # RADIAL SUPPORTS carry the load, bonded to the hull. No outer chord (the
    # hull IS the outer boundary); inner chord bonds to the core, struts bond
    # to the hull at their outer end.
    strut_zs = [z0 + 3] + frame_z + [z1 - 3]
    for s in range(ns):
        a = TAU * s / ns
        # struts bond DIRECTLY to both the core and the inner hull (no chords)
        for z in strut_zs:
            _beam(f"strut_{s}_{int(z)}", P(core, a, z), P(r_out, a, z), t_strut, c_sup, C_STRUT)
    # INNER-HULL frames: hoops at r=inner that the struts land on. The ~2 m out
    # to the outer hull (radius_outer) is the water-Whipple barrel — the double
    # hull, not yet modelled.
    for z in strut_zs:
        _arc(f"innerhull_{int(z)}", inner, 0.0, TAU, z, t_strut, c_sup, C_STRUT, segs=48)

    n = mass = mom_x = mom_y = 0
    per_track = []
    for s in range(ns):
        f = fill.get(s, 0.0)
        if f <= 0:
            continue
        a0, a1 = TAU * s / ns, TAU * (s + 1) / ns
        for rr in rings:
            for zf in frame_z:
                _arc(f"arc_{s}_{int(rr)}_{int(zf)}", rr, a0, a1, zf, t_arc, c_arc, C_ARC, segs=10)
        for i in range(len(rings) - 1):
            ri, ro = rings[i], rings[i + 1]
            rc = (ri + ro) / 2
            # keep containers clear of the boundary struts (an aisle each edge)
            margin = (cw / 2 + t_strut / 2 + 2 * cl) / rc
            lo, hi = a0 + margin, a1 - margin
            ncont = max(1, int((hi - lo) * rc / (cw + cl)))
            if s == 0:
                per_track.append({"r_mid": round(rc, 1), "per_sector": ncont})
            r_inner_rail, r_outer_rail = ri + rail_off, ro - rail_off
            for j in range(ncont):
                th = lo + (j + 0.5) * (hi - lo) / ncont
                for (rr, arc_r) in ((r_inner_rail, ri), (r_outer_rail, ro)):
                    _beam(f"rail_{s}_{i}_{j}_{int(rr)}", P(rr, th, z0 + 4), P(rr, th, z1 - 4),
                          t_rail, c_rail, C_RAIL)
                    # connectors bond each rail to its arc at every frame; seat
                    # them into the arc (past its centre) so they positively meet
                    ext = arc_r + math.copysign(t_arc / 2, arc_r - rr)
                    for zf in frame_z:
                        _beam(f"conn_{s}_{i}_{j}_{int(rr)}_{int(zf)}", P(rr, th, zf),
                              P(ext, th, zf), t_conn, c_arc, C_CONN)
                for k in range(max(1, round(nz * f))):
                    hot = (s == 0 and i == 2 and j == 0)
                    _box(f"c_{s}_{i}_{j}_{k}", (ro - ri - ring_gap, cw, spec["container"]["L"]),
                         P(rc, th, zsts[k]), c_hot if hot else c_cargo,
                         C_HOT if hot else C_CARGO, rotz=th)
                    mm = spec["container_mass_t"] * (1.9 if hot else 1.0)
                    x, y, _ = P(rc, th, 0)
                    n += 1; mass += mm; mom_x += mm * x; mom_y += mm * y

    if len(rings) > 4:
        ri, ro = rings[2], rings[3]; rc = (ri + ro) / 2
        _box("EXTRACT_container", (ro - ri - ring_gap, cw, spec["container"]["L"]),
             P(rc, TAU * 0.5 / ns, z1 + 6), colls[3], (0.20, 0.95, 0.35), rotz=TAU * 0.5 / ns)

    _box("bay_door", (13, 1, 14), (0, -(inner - 0.3), zc), c_door, C_DOOR)
    _box("loader", (3.2, 9, 3.2), (0, -(inner - 5.5), zc), c_door, C_LOAD)
    tps = sum(t["per_sector"] for t in per_track)
    return {"containers_shown": n, "tracks": len(per_track), "per_track": per_track,
            "ring_pitch_m": round(ring_pitch, 2), "per_sector_layer": tps,
            "full_bay_capacity_est": tps * ns * int(spec["cargo_bay_len"] / zp)}


def _set_viewport_object_color():
    try:
        for area in bpy.context.screen.areas:
            if area.type == 'VIEW_3D':
                for sp in area.spaces:
                    if sp.type == 'VIEW_3D':
                        sp.shading.type = 'SOLID'; sp.shading.color_type = 'OBJECT'
                        sp.shading.show_object_outline = True
    except Exception:
        pass


def build():
    _fresh_scene()
    bpy.context.scene.unit_settings.system = "METRIC"
    R = SPEC["radius_outer"]; L = SPEC["length"]
    sz = structural_sizing(SPEC)
    hull = _collection("SHIP_hull_zones"); struct = _collection("SHIP_structure")
    colls = (_collection("SHIP_cargo_support"), _collection("SHIP_cargo_arc"),
             _collection("SHIP_cargo_rail"), _collection("SHIP_cargo"),
             _collection("SHIP_cargo_heavy"), _collection("SHIP_cargo_door"))
    _cyl_surface("Core", SPEC["core_radius"], -L / 2, L / 2, struct, C_CORE, caps=True)
    z_top = L / 2; zone_z = {}
    for name, length, color in SPEC["zones"]:
        z0 = z_top - length
        _cyl_surface(f"zone_{name}", R, z0, z_top, hull, color); zone_z[name] = (z0, z_top); z_top = z0
    _cone("Nose_cap", R, L / 2, L / 2 + SPEC["nose_cap_len"], struct, (0.40, 0.60, 0.90))
    _cyl_surface("Tail_cap", R, -L / 2 - SPEC["tail_len"], -L / 2, struct, (0.90, 0.45, 0.25), caps=True)
    z0f, _ = zone_z["cargo_fore"]
    sec = build_cargo(z0f + 15, z0f + 95, colls, SPEC, sz)
    _set_viewport_object_color()
    try:
        bpy.ops.ed.undo_push(message="haunted-forest: build ship")
    except Exception:
        pass
    return {"radius_m": R, "length_m": L, "objects": len(bpy.data.objects),
            "sizing": sz, "section": sec}


result = build()
