# Haunted Forest — parametric ship model generator (v0 greybox).
#
# Living spec. Run against live Blender via the MCP bridge:
#   exec(open('/home/eric/proj/haunted-forest/modeling/ship.py').read())
#
# Cargo hold = a SEMI-MONOCOQUE cylinder. Each 20 t container is CLAMPED to a
# pair of fore/aft rails (stringers) shear-tied to the inner-hull skin at every
# frame. A box sheds only its OWN axial load into the skin locally, so no box
# carries the column (no bottom-box crush, no progressive collapse) and no rail
# carries the column (no whole-column buckling -- the old model's error). The
# skin carries the ACCUMULATING axial load as membrane compression, the way a
# rocket airframe already does, tapering thin->thick fore->aft. A mutually-braced
# frame cage (radial struts + hoops at the core and in the double-hull gap +
# meridional diagonals in the sector-boundary planes) stabilises the stringers
# and reacts lateral + midcourse-flip loads as axial spar forces. Gravity is
# along Z and REVERSES at midcourse; a clamp holds both signs, so the flip just
# changes the sign of loads the members already carry. All geometry is DERIVED
# from container size + clearances + structurally-sized members (structural_sizing).
#
# 1 km x 40 m (12.5:1). The fuel bands are 315 m of full-section tank,
# 1.41e6 m3 of IS metallic-hydrogen cells at 0.7 g/cm3 (~990 kt), which
# leaves 365 m of hold — shorter than the old 600 m but fatter, ten tracks,
# more tonnes. Habitat sits midships between the fuel bands. Hotel plant is a 70 m tandem mirror aft. No tail
# ICF: the torch is two IS black-box nacelles on 45° A-frames that pierce
# the hull and land on a torque box (belt rings + longerons). Hotel waste
# leaves through the aft hull skin. Nothing on the bow but Whipple. Debris
# suite is two packages on the nacelle shoulders, not the nose.

import time

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
C_DIAG = (0.95, 0.55, 0.78)
C_SKIN = (0.28, 0.44, 0.62)
C_CARGO = (0.92, 0.74, 0.20)
C_HOT = (0.95, 0.35, 0.15)
C_DOOR = (0.15, 0.85, 0.35)
C_LOAD = (0.15, 0.72, 0.88)
C_PLASMA = (0.96, 0.28, 0.72)
C_CHAMBER = (0.40, 0.42, 0.48)
C_COIL = (0.28, 0.54, 0.86)
C_CHOKE = (0.20, 0.85, 0.92)
C_PLUG = (0.96, 0.80, 0.25)
C_CONV = (0.55, 0.85, 0.55)
C_FUEL = (0.36, 0.62, 0.92)
C_METAL = (0.22, 0.58, 0.88)
C_ENG = (0.88, 0.56, 0.30)
C_RAD = (0.93, 0.42, 0.18)
C_RAD_ZONES = (
    (0.95, 0.38, 0.14), (0.92, 0.50, 0.16), (0.88, 0.36, 0.22), (0.96, 0.44, 0.10),
    (0.90, 0.32, 0.18), (0.94, 0.48, 0.20), (0.86, 0.40, 0.14), (0.98, 0.42, 0.24),
)
C_DRIVE = (0.72, 0.76, 0.84)
C_NAC = (0.18, 0.18, 0.20)
C_NAC_RAD = (0.85, 0.45, 0.20)
C_BELT = (0.72, 0.68, 0.58)
C_BOX = (0.50, 0.48, 0.42)
C_FEL = (0.95, 0.90, 0.35)
C_FEL_MACH = (0.75, 0.70, 0.28)
C_RADAR = (0.45, 0.72, 0.88)
C_SHUT = (0.22, 0.24, 0.26)
C_HAB = (0.90, 0.28, 0.38)


def arange(lo, hi, step):
    out, x = [], lo
    while x <= hi + 1e-9:
        out.append(round(x, 4)); x += step
    return out


SPEC = {
    "length": 1000.0, "radius_outer": 40.0, "barrel_thickness": 2.0,
    "core_radius": 4.0, "nose_cap_len": 105.0, "tail_len": 16.0,
    "container": {"L": 12.0, "W": 2.5, "H": 2.5}, "container_mass_t": 20.0,
    "clearance": 0.15,
    "n_sectors": 8,
    "max_accel_G": 9.0, "lateral_G": 2.0,
    "nacelle_mass_t": 25_000.0,
    "material_E_Pa": 500.0e9,
    "material_strength_Pa": 50.0e9,
    "material_density": 1400.0,
    "safety_factor": 2.0,
    "cargo_bay_len": 185.0,
    "cargo_bays": (180.0, 185.0),
    "metal_density": 700.0,
    "zones": [
        ("nose_whipple",   50.0,  (0.35, 0.55, 0.85)),
        ("cargo_fore",    180.0,  (0.90, 0.65, 0.15)),
        ("fuel_fore",     160.0,  (0.20, 0.70, 0.90)),
        ("crew_core",      40.0,  (0.90, 0.25, 0.35)),
        ("fuel_aft",      155.0,  (0.20, 0.70, 0.90)),
        ("cargo_aft",     185.0,  (0.90, 0.65, 0.15)),
        ("reactor",       140.0,  (0.70, 0.30, 0.90)),
        ("radiator_aft",   90.0,  (0.90, 0.40, 0.20)),
    ],
    # Hotel plant: 70 m x 3 m D-3He tandem mirror. Sits entirely in the
    # reactor zone. Throttle is burning length (coil groups). No drive-fuel
    # farm — the 2 kt D-3He load is two tanks in the annulus.
    "reactor": {
        "burn_len": 70.0,
        "plasma_r": 3.0, "chamber_r": 4.5,
        "coil_r": 6.2, "coil_minor": 0.7, "coil_pitch": 7.0,
        "choke_r": 7.5, "choke_minor": 1.4,
        "plug_len": 8.0, "plug_r": 2.8, "plug_coil_r": 4.5, "plug_coil_minor": 0.9,
        "conv_len": 12.0, "conv_r": 11.0,
        "eng_deck_len": 10.0, "shield_len": 4.0,
        "dhe3_tank_r": 6.0, "dhe3_tank_len": 55.0, "dhe3_tank_off": 22.0,
    },
    # Aft hull skin, eight azimuthal gores. Covers the reactor zone and the
    # tail barrel — not the tanks, not the hold. No wake structure.
    "radiator": {
        "n_zones": 8, "standoff": 0.35, "gap_frac": 0.05,
    },
    # IS black boxes on 45° A-frames that pierce the hull and land on a
    # torque box (two CNT belt rings + longerons). Thrust is axial in the
    # struts. The box fires either way; nozzles at both ends, no slew.
    "nacelle": {
        "count": 2, "pod_r": 11.0, "pod_len": 58.0, "stand_off": 40.0,
        "nozzle_len": 14.0, "node_r": 8.0, "node_minor": 1.6,
        "strut_t": 3.6, "splay": 0.32, "ring_minor": 2.3, "belt_minor": 1.6,
        "longeron_t": 2.2, "box_r": 9.0,
        "rad_n": 4, "rad_span": 10.0, "rad_len": 22.0, "rad_thick": 0.35,
    },
    # Debris suite on the nacelle shoulders, not the nose. Two 10 m AESAs
    # and two FEL directors looking +Z. Machines live in the torque box.
    # Shutters stowed (open) in the cruise pose.
    "sensors": {
        "aesa_m": 10.0, "aesa_thick": 0.55,
        "fel_r": 1.15, "fel_len": 8.0,
        "thz_m": 3.2, "thz_thick": 0.4,
        "shutter_thick": 0.28,
        "machine_r": 3.2, "machine_len": 28.0,
    },
}


def _beam_mass(side, length, count, rho):
    return side * side * length * count * rho


def _box_wall_area(a, b, tw):
    return 2.0 * (a + b) * tw


def drive_mount_mass(spec):
    """Tube-and-belt mass for the 45° A-frames and torque box. Greybox
    members are drawn solid for visibility; this is the sized structure."""
    nc = spec["nacelle"]
    rho = spec["material_density"]
    E = spec["material_E_Pa"]
    sf = spec["safety_factor"]
    R = spec["radius_outer"]
    inner = R - spec["barrel_thickness"]
    axis_r = R + nc["stand_off"] + nc["pod_r"]
    dz = axis_r - inner
    L_strut = math.hypot(dz, dz) * 1.06
    F = spec.get("nacelle_mass_t", 25_000.0) * spec["max_accel_G"] * 9.81 * 1000.0
    P = F / math.sqrt(2.0)
    I = sf * P * L_strut ** 2 / (math.pi ** 2 * E)
    r_od = nc["strut_t"] / 2.0
    t_wall = I / (math.pi * r_od ** 3)
    A_strut = 2.0 * math.pi * r_od * t_wall

    def kg(area, length, n=1.0):
        return area * length * n * rho / 1e3

    rings = (kg(_box_wall_area(3.0, 1.5, 0.15), TAU * inner, 2)
             + kg(_box_wall_area(2.2, 1.2, 0.12), TAU * (R + 0.4), 2))
    longerons = kg(_box_wall_area(2.2, 2.2, 0.10), 2.0 * dz, 4)
    box = kg(TAU * nc["box_r"] * 0.04, 2.0 * dz)
    spokes = kg(_box_wall_area(1.5, 1.5, 0.08), inner - nc["box_r"], 8)
    hardpoints = kg(_box_wall_area(1.8, 1.8, 0.08), 2.0 * inner * nc["splay"], 8)
    nodes = kg(_box_wall_area(0.9, 0.9, 0.12), TAU * nc["node_r"], 2)
    struts = kg(A_strut, L_strut, 8)
    parts = {
        "struts": round(struts),
        "belt_rings": round(rings),
        "longerons": round(longerons),
        "box_skin": round(box),
        "spokes": round(spokes),
        "hardpoints": round(hardpoints),
        "nodes": round(nodes),
    }
    sub = sum(parts.values())
    parts["fittings"] = round(0.2 * sub)
    parts["total"] = sub + parts["fittings"]
    parts["strut_wall_m"] = round(t_wall, 3)
    return parts


def track_layout(spec, a_rail, a_arc, a_strut):
    inner = spec["radius_outer"] - spec["barrel_thickness"]
    core = spec["core_radius"]
    ns = spec["n_sectors"]
    cw, cl = spec["container"]["W"], spec["clearance"]
    rail_off = a_arc / 2 + cl + a_rail / 2
    ring_gap = 2 * rail_off + a_rail
    ring_pitch = cw + ring_gap
    rings = arange(core + 1.0, inner - 0.25, ring_pitch)
    n_lines = 0
    per_track = []
    for i in range(len(rings) - 1):
        ri, ro = rings[i], rings[i + 1]
        rc = (ri + ro) / 2
        margin = (cw / 2 + a_strut / 2 + 2 * cl) / rc
        span = TAU / ns - 2 * margin
        per = max(1, int(span * rc / (cw + cl)))
        n_lines += per * ns
        per_track.append({"r_mid": round(rc, 1), "per_sector": per,
                          "r_in": round(ri, 2), "r_out": round(ro, 2)})
    return {
        "rings": rings, "rail_off": rail_off, "ring_gap": ring_gap,
        "ring_pitch": ring_pitch, "n_lines": n_lines, "per_track": per_track,
    }


def structural_sizing(spec):
    g = 9.81
    m = spec["container_mass_t"] * 1000.0
    E = spec["material_E_Pa"]; rho = spec["material_density"]
    sf = spec["safety_factor"]; sig = spec["material_strength_Pa"] / sf
    cw, cl = spec["container"]["W"], spec["clearance"]
    zp = spec["container"]["L"] + 0.5
    R = spec["radius_outer"] - spec["barrel_thickness"]
    core = spec["core_radius"]; ns = spec["n_sectors"]; bay = spec["cargo_bay_len"]
    F_axial = m * spec["max_accel_G"] * g
    F_lat = m * spec["lateral_G"] * g

    def rail_for(leff):
        return max(0.08, (12 * F_axial * sf * leff ** 2 / (math.pi ** 2 * E)) ** 0.25)
    a_rail = rail_for(zp / 2)
    a_rail_unbraced = rail_for(zp)

    M_lat = (2 * F_lat) * (cw + 1.0) / 4.0
    a_strut = max((6 * M_lat / sig) ** (1 / 3.0), 1.2 * a_rail)
    a_arc = a_rail
    a_conn = max(0.06, 0.8 * a_rail)

    lay = track_layout(spec, a_rail, a_arc, a_strut)
    n_lines = lay["n_lines"]
    n_z = int(bay / zp)
    n_boxes = n_lines * n_z
    P_bay = n_boxes * F_axial

    gamma = 0.5
    t_skin = math.sqrt(P_bay * sf / (2 * math.pi * gamma * 0.6 * E))

    n_arc_st = 2 * n_z + 1
    n_str_st = n_z + 1
    hyp = math.hypot(R - core, zp)
    m_rail = _beam_mass(a_rail, bay, 2 * n_lines, rho)
    m_strut = _beam_mass(a_strut, R - core, ns * n_str_st, rho)
    m_hoop = _beam_mass(a_strut, TAU * R, n_str_st, rho)
    m_diag = _beam_mass(a_strut, hyp, 2 * ns * (n_str_st - 1), rho)
    m_arc = _beam_mass(a_arc, n_arc_st * TAU * sum(lay["rings"]), 1, rho)
    m_conn = _beam_mass(a_conn, lay["rail_off"], 2 * n_lines * n_arc_st, rho)
    scaf = (m_rail + m_strut + m_hoop + m_diag + m_arc + m_conn) / 1e3
    m_skin = 2 * math.pi * R * t_skin * bay * rho * 0.5 / 1e3

    return {
        "F_container_axial_MN": round(F_axial / 1e6, 2),
        "F_container_lateral_MN": round(F_lat / 1e6, 2),
        "z_pitch": zp,
        "rail_side_m": round(a_rail, 3),
        "rail_side_unbraced_m": round(a_rail_unbraced, 3),
        "arc_side_m": round(a_arc, 3),
        "strut_side_m": round(a_strut, 3),
        "conn_side_m": round(a_conn, 3),
        "skin_t_m": round(t_skin, 3),
        "tracks_per_sector": len(lay["rings"]) - 1,
        "rail_lines_per_bay": n_lines,
        "boxes_per_bay": n_boxes,
        "capacity_both_bays": n_lines * sum(
            int(b / zp) for b in spec.get("cargo_bays", (spec["cargo_bay_len"], spec["cargo_bay_len"]))),
        "capacity_t": n_lines * sum(
            int(b / zp) for b in spec.get("cargo_bays", (spec["cargo_bay_len"], spec["cargo_bay_len"])))
        * spec["container_mass_t"],
        "P_bay_GN": round(P_bay / 1e9, 2),
        "mass_t": {
            "rails": round(m_rail / 1e3),
            "frames_struts_hoops_diagonals": round((m_strut + m_hoop + m_diag) / 1e3),
            "arcs_connectors": round((m_arc + m_conn) / 1e3),
            "scaffold": round(scaf),
            "skin_tapered": round(m_skin),
            "structure_total": round(m_skin + scaf),
        },
    }


def _fresh_scene():
    if bpy.data.objects:
        bpy.data.batch_remove(tuple(bpy.data.objects))
    for coll in list(bpy.data.collections):
        bpy.data.collections.remove(coll)
    for block in (bpy.data.meshes, bpy.data.curves, bpy.data.cameras, bpy.data.lights):
        orphans = [b for b in block if b.users == 0]
        if orphans:
            bpy.data.batch_remove(orphans)


def _collection(name):
    c = bpy.data.collections.new(name)
    bpy.context.scene.collection.children.link(c)
    return c


def _add(obj, coll, color, alpha=1.0):
    coll.objects.link(obj)
    obj.color = (color[0], color[1], color[2], alpha)
    return obj


_BOX_FACES = (
    (0, 1, 3, 2), (4, 6, 7, 5), (0, 4, 5, 1),
    (2, 3, 7, 6), (0, 2, 6, 4), (1, 5, 7, 3),
)


def _box_mesh(name, dx, dy, dz):
    m = bpy.data.meshes.new(name)
    hx, hy, hz = dx / 2, dy / 2, dz / 2
    vs = [(x, y, z) for x in (-hx, hx) for y in (-hy, hy) for z in (-hz, hz)]
    m.from_pydata(vs, [], _BOX_FACES); m.update()
    return m


class Batch:
    """World-space boxes glued into one mesh. Cargo used to be ~12k objects."""

    def __init__(self):
        self.vs = []
        self.faces = []

    def add_box(self, dims, loc, rotz=0.0):
        hx, hy, hz = dims[0] / 2, dims[1] / 2, dims[2] / 2
        lx, ly, lz = loc
        if rotz:
            c, s = math.cos(rotz), math.sin(rotz)
            def xform(x, y, z):
                return (lx + c * x - s * y, ly + s * x + c * y, lz + z)
        else:
            def xform(x, y, z):
                return (lx + x, ly + y, lz + z)
        base = len(self.vs)
        for x in (-hx, hx):
            for y in (-hy, hy):
                for z in (-hz, hz):
                    self.vs.append(xform(x, y, z))
        self.faces.extend((base + a, base + b, base + c, base + d) for a, b, c, d in _BOX_FACES)

    def add_beam(self, p0, p1, t):
        p0v, p1v = mathutils.Vector(p0), mathutils.Vector(p1)
        d = p1v - p0v
        if d.length < 1e-6:
            return
        mid = (p0v + p1v) * 0.5
        rot = d.to_track_quat('Z', 'Y').to_matrix()
        hx = hy = t * 0.5
        hz = d.length * 0.5
        base = len(self.vs)
        for x in (-hx, hx):
            for y in (-hy, hy):
                for z in (-hz, hz):
                    self.vs.append(tuple(mid + rot @ mathutils.Vector((x, y, z))))
        self.faces.extend((base + a, base + b, base + c, base + d) for a, b, c, d in _BOX_FACES)

    def add_arc(self, r, a0, a1, z, t, segs=3):
        for i in range(segs):
            self.add_beam(P(r, a0 + (a1 - a0) * i / segs, z),
                          P(r, a0 + (a1 - a0) * (i + 1) / segs, z), t)

    def emit(self, name, coll, color):
        if not self.vs:
            return None
        mesh = bpy.data.meshes.new(name)
        mesh.from_pydata(self.vs, [], self.faces)
        mesh.update()
        return _add(bpy.data.objects.new(name, mesh), coll, color)


def _cyl_surface(name, radius, z0, z1, coll, color, verts=72, caps=False, alpha=1.0):
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
    return _add(bpy.data.objects.new(name, mesh), coll, color, alpha)


def _cone(name, radius, z_base, z_apex, coll, color, verts=64):
    mesh = bpy.data.meshes.new(name)
    vs = [(radius * math.cos(TAU * i / verts), radius * math.sin(TAU * i / verts), z_base)
          for i in range(verts)]
    apex = len(vs); vs.append((0, 0, z_apex))
    faces = [(i, (i + 1) % verts, apex) for i in range(verts)]
    mesh.from_pydata(vs, [], faces); mesh.update()
    return _add(bpy.data.objects.new(name, mesh), coll, color)


def _torus(name, R, r, z, coll, color, nu=28, nv=12):
    mesh = bpy.data.meshes.new(name)
    vs, faces = [], []
    for i in range(nu):
        a = TAU * i / nu; ca, sa = math.cos(a), math.sin(a)
        for j in range(nv):
            b = TAU * j / nv; rr = R + r * math.cos(b)
            vs.append((rr * ca, rr * sa, z + r * math.sin(b)))
    for i in range(nu):
        i2 = (i + 1) % nu
        for j in range(nv):
            j2 = (j + 1) % nv
            faces.append((i * nv + j, i2 * nv + j, i2 * nv + j2, i * nv + j2))
    mesh.from_pydata(vs, [], faces); mesh.update()
    return _add(bpy.data.objects.new(name, mesh), coll, color)


def _cyl_panel(name, r, z0, z1, a0, a1, coll, color, segs=14):
    mesh = bpy.data.meshes.new(name)
    vs, faces = [], []
    n = segs + 1
    for z in (z0, z1):
        for i in range(n):
            a = a0 + (a1 - a0) * i / segs
            vs.append((r * math.cos(a), r * math.sin(a), z))
    for i in range(segs):
        faces.append((i, i + 1, n + i + 1, n + i))
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


def zone_map(spec):
    L = spec["length"]
    z_top = L / 2
    out = {}
    for name, length, color in spec["zones"]:
        z0 = z_top - length
        out[name] = (z0, z_top, color, length)
        z_top = z0
    return out


def fuel_volume_m3(spec):
    inner = spec["radius_outer"] - spec["barrel_thickness"]
    core = spec["core_radius"]
    area = math.pi * (inner ** 2 - core ** 2)
    length = sum(z[1] for z in spec["zones"] if z[0].startswith("fuel_"))
    return area * length, length


def build_cargo(z0, z1, colls, spec, sz):
    c_sup, c_arc, c_rail, c_cargo, c_hot, c_door, c_skin = colls
    inner = spec["radius_outer"] - spec["barrel_thickness"]
    core = spec["core_radius"]
    ns = spec["n_sectors"]
    cw = spec["container"]["W"]; cl = spec["clearance"]
    t_rail, t_arc, t_conn, t_strut = sz["rail_side_m"], sz["arc_side_m"], sz["conn_side_m"], sz["strut_side_m"]
    zp = sz["z_pitch"]
    r_out = inner
    lay = track_layout(spec, t_rail, t_arc, t_strut)
    rings, rail_off, ring_gap = lay["rings"], lay["rail_off"], lay["ring_gap"]
    door_h = spec["container"]["L"] + 2.0
    cz1 = z1 - door_h
    z_lo = z0 + 5.0
    nz = max(1, round((cz1 - z_lo) / zp))
    bp = (cz1 - z_lo) / nz
    zsts = [z_lo + bp * (k + 0.5) for k in range(nz)]
    frame_z = [z_lo + bp * k for k in range(nz + 1)]
    arc_z = [z_lo + bp * k / 2 for k in range(2 * nz + 1)]
    zc_door = (cz1 + z1) / 2

    strut_zs = list(frame_z)
    cage, diags, arcs, conns, rails, boxes, hot = (Batch() for _ in range(7))
    for s in range(ns):
        a = TAU * s / ns
        for z in strut_zs:
            cage.add_beam(P(core, a, z), P(r_out, a, z), t_strut)
        for k in range(len(strut_zs) - 1):
            za, zb = strut_zs[k], strut_zs[k + 1]
            diags.add_beam(P(core, a, za), P(r_out, a, zb), t_strut)
            diags.add_beam(P(r_out, a, za), P(core, a, zb), t_strut)
    for z in strut_zs + [z1]:
        cage.add_arc(inner, 0.0, TAU, z, t_strut, segs=32)
    _cyl_surface("inner_hull_skin", inner, z0, z1, c_skin, C_SKIN, verts=96)

    n = mass = mom_x = mom_y = 0
    per_track = []
    for s in range(ns):
        a0, a1 = TAU * s / ns, TAU * (s + 1) / ns
        for rr in rings:
            for zf in arc_z:
                arcs.add_arc(rr, a0, a1, zf, t_arc, segs=8)
        for i in range(len(rings) - 1):
            ri, ro = rings[i], rings[i + 1]
            rc = (ri + ro) / 2
            margin = (cw / 2 + t_strut / 2 + 2 * cl) / rc
            lo, hi = a0 + margin, a1 - margin
            ncont = max(1, int((hi - lo) * rc / (cw + cl)))
            if s == 0:
                per_track.append({"r_mid": round(rc, 1), "per_sector": ncont})
            r_inner_rail, r_outer_rail = ri + rail_off, ro - rail_off
            for j in range(ncont):
                th = lo + (j + 0.5) * (hi - lo) / ncont
                for (rr, arc_r) in ((r_inner_rail, ri), (r_outer_rail, ro)):
                    rails.add_beam(P(rr, th, z0 + 4), P(rr, th, cz1), t_rail)
                    ext = arc_r + math.copysign(t_arc / 2, arc_r - rr)
                    for zf in arc_z:
                        conns.add_beam(P(rr, th, zf), P(ext, th, zf), t_conn)
                for k in range(nz):
                    is_hot = (s == 0 and i == 2 and j == 0)
                    dest = hot if is_hot else boxes
                    dest.add_box((ro - ri - ring_gap, cw, spec["container"]["L"]),
                                 P(rc, th, zsts[k]), rotz=th)
                    mm = spec["container_mass_t"] * (1.9 if is_hot else 1.0)
                    x, y, _ = P(rc, th, 0)
                    n += 1; mass += mm; mom_x += mm * x; mom_y += mm * y

    cage.emit("cargo_cage", c_sup, C_STRUT)
    diags.emit("cargo_diags", c_sup, C_DIAG)
    arcs.emit("cargo_arcs", c_arc, C_ARC)
    conns.emit("cargo_conns", c_arc, C_CONN)
    rails.emit("cargo_rails", c_rail, C_RAIL)
    boxes.emit("cargo_boxes", c_cargo, C_CARGO)
    hot.emit("cargo_heavy", c_hot, C_HOT)
    _box("loading_container", (cw, cw, spec["container"]["L"]),
         (0, -(inner - 6.0), zc_door), colls[3], (0.20, 0.95, 0.35))
    _box("bay_door", (6, 1, door_h), (0, -(inner - 0.3), zc_door), c_door, C_DOOR)
    _box("loader", (3.2, 9, 3.2), (0, -(inner - 5.5), zc_door), c_door, C_LOAD)
    tps = sum(t["per_sector"] for t in per_track)
    return {"containers_shown": n, "tracks": len(per_track), "per_track": per_track,
            "ring_pitch_m": round(lay["ring_pitch"], 2), "per_sector_layer": tps,
            "full_bay_capacity_est": tps * ns * int(spec["cargo_bay_len"] / zp)}


def build_tanks(coll, spec, zones):
    inner = spec["radius_outer"] - spec["barrel_thickness"]
    core = spec["core_radius"]
    for name in ("fuel_fore", "fuel_aft"):
        z0, z1, _, _ = zones[name]
        _cyl_surface(f"{name}_inner", core + 0.4, z0, z1, coll, C_METAL, verts=48, alpha=0.45)
        _cyl_surface(f"{name}_outer", inner - 0.3, z0, z1, coll, C_METAL, verts=64, alpha=0.45)
    vol, length = fuel_volume_m3(spec)
    return {"volume_m3": round(vol), "mass_kt": round(vol * spec["metal_density"] / 1e6, 1),
            "tank_len_m": length}


def build_crew(coll, spec, zones):
    z0, z1, _, _ = zones["crew_core"]
    _cyl_surface("hab_pressure", 6.5, z0 + 2.0, z1 - 2.0, coll, C_HAB, verts=36, caps=True)
    _cyl_surface("bridge_blister", 4.0, z1 - 8.0, z1 + 4.0, coll, C_HAB, verts=24, caps=True)
    return {"hab_len_m": round((z1 - z0) - 4.0, 1)}


def build_reactor(coll, fuel_coll, spec, zones):
    rc = spec["reactor"]
    z0, z1, _, zlen = zones["reactor"]
    # Stack, aft -> fore, packed against the aft end of the zone so the
    # engineering deck sits at the forward (crew-side) bulkhead.
    z = z0 + 1.0
    z_aft_conv0 = z
    z_aft_conv1 = z + rc["conv_len"]; z = z_aft_conv1
    z_aft_plug0 = z
    z_aft_plug1 = z + rc["plug_len"]; z = z_aft_plug1
    z_aft_choke = z + 2.5; z = z_aft_choke + 2.5
    z_plasma0 = z
    z_plasma1 = z + rc["burn_len"]; z = z_plasma1
    z_fore_choke = z + 2.5; z = z_fore_choke + 2.5
    z_fore_plug0 = z
    z_fore_plug1 = z + rc["plug_len"]; z = z_fore_plug1
    z_fore_conv0 = z
    z_fore_conv1 = z + rc["conv_len"]; z = z_fore_conv1
    z_shield0 = z
    z_shield1 = z + rc["shield_len"]; z = z_shield1
    z_eng0 = z
    z_eng1 = z + rc["eng_deck_len"]
    if z_eng1 > z1 - 0.5:
        raise RuntimeError(f"reactor stack {z_eng1 - z0:.1f} m exceeds zone {zlen:.1f} m")

    _cyl_surface("reactor_plasma", rc["plasma_r"], z_plasma0, z_plasma1, coll, C_PLASMA, verts=32)
    _cyl_surface("reactor_chamber", rc["chamber_r"], z_plasma0, z_plasma1, coll, C_CHAMBER, verts=48)
    for zc in arange(z_plasma0 + rc["coil_pitch"], z_plasma1 - rc["coil_pitch"] + 1e-6, rc["coil_pitch"]):
        _torus(f"cc_coil_{int(zc)}", rc["coil_r"], rc["coil_minor"], zc, coll, C_COIL)
    for zc in (z_aft_choke, z_fore_choke):
        for dz in (-2.0, 2.0):
            _torus(f"choke_{int(zc)}_{int(dz)}", rc["choke_r"], rc["choke_minor"], zc + dz, coll, C_CHOKE)
    for tag, zp0, zp1 in (("aft", z_aft_plug0, z_aft_plug1), ("fore", z_fore_plug0, z_fore_plug1)):
        _cyl_surface(f"plug_bore_{tag}", rc["plug_r"], zp0, zp1, coll, C_CHAMBER, verts=24)
        for frac in (0.35, 0.65):
            zp = zp0 + (zp1 - zp0) * frac
            _torus(f"plugcoil_{tag}_{int(zp)}", rc["plug_coil_r"], rc["plug_coil_minor"], zp, coll, C_PLUG)
    _cone("converter_aft", rc["conv_r"], z_aft_conv0, z_aft_conv1, coll, C_CONV)
    _cone("converter_fore", rc["conv_r"], z_fore_conv1, z_fore_conv0, coll, C_CONV)
    _cyl_surface("reactor_fuel_shield", rc["conv_r"], z_shield0, z_shield1, fuel_coll, C_FUEL, verts=32, caps=True)
    _cyl_surface("eng_deck", rc["conv_r"] * 0.8, z_eng0, z_eng1, coll, C_ENG, verts=32, caps=True)

    # 2 kt D-3He — two tanks in the annulus, not a spine farm.
    for s, tag in ((0.0, "port"), (math.pi, "stbd")):
        x, y, _ = P(rc["dhe3_tank_off"], s, 0)
        zc = (z_plasma0 + z_plasma1) / 2
        half = rc["dhe3_tank_len"] / 2
        tank = _cyl_surface(f"dhe3_{tag}", rc["dhe3_tank_r"], zc - half, zc + half,
                            fuel_coll, C_FUEL, verts=24, caps=True)
        tank.location = (x, y, 0.0)

    return {"burn_len_m": rc["burn_len"],
            "column_z": [round(z_plasma0, 1), round(z_plasma1, 1)],
            "stack_len_m": round(z_eng1 - z_aft_conv0, 1),
            "eng_deck_z": round((z_eng0 + z_eng1) / 2, 1),
            "objects": len(coll.objects) + len(fuel_coll.objects)}


def build_radiator(coll, spec, zones):
    r = spec["radiator"]
    R = spec["radius_outer"]
    z_aft, _, _, _ = zones["radiator_aft"]
    _, z_fore, _, _ = zones["reactor"]
    n = r["n_zones"]
    sector = TAU / n
    gap = sector * r["gap_frac"]
    live = sector - gap
    rr = R + r["standoff"]
    for i in range(n):
        a0 = i * sector + gap / 2
        a1 = a0 + live
        col = C_RAD_ZONES[i % len(C_RAD_ZONES)]
        _cyl_panel(f"rad_gore_{i}", rr, z_aft, z_fore, a0, a1, coll, col, segs=16)
        amid = 0.5 * (a0 + a1)
        zc = 0.5 * (z_aft + z_fore)
        _beam(f"tin_riser_{i}", P(6.0, amid, zc), P(R, amid, zc), 0.9, coll, C_RAD)
    barrel = TAU * R * (z_fore - z_aft)
    tail = math.pi * R * R
    return {"barrel_area_m2": round(barrel), "tail_area_m2": round(tail),
            "total_area_m2": round(barrel + tail), "n_zones": n,
            "length_m": round(z_fore - z_aft)}


def build_nacelles(coll, spec, zones):
    nc = spec["nacelle"]
    R = spec["radius_outer"]
    inner = R - spec["barrel_thickness"]
    z_reac0, z_reac1, _, _ = zones["reactor"]
    zc = (z_reac0 + z_reac1) / 2
    axis_r = R + nc["stand_off"] + nc["pod_r"]
    dz = axis_r - inner
    z_fore, z_aft = zc + dz, zc - dz
    half = nc["pod_len"] / 2
    splay = nc["splay"]

    _cyl_surface("pylon_box", nc["box_r"], z_aft, z_fore, coll, C_BOX, verts=28)
    for tag, z in (("fore", z_fore), ("aft", z_aft)):
        _torus(f"belt_inner_{tag}", inner, nc["ring_minor"], z, coll, C_BELT, nu=40, nv=12)
        _torus(f"belt_outer_{tag}", R + 0.4, nc["belt_minor"], z, coll, C_BELT, nu=40, nv=12)
    for k in range(4):
        a = TAU * k / 4
        _beam(f"longeron_{k}", P(inner, a, z_aft), P(inner, a, z_fore), nc["longeron_t"], coll, C_BELT)
        _beam(f"box_spoke_fore_{k}", P(nc["box_r"], a, z_fore), P(inner, a, z_fore),
              nc["longeron_t"] * 0.7, coll, C_BOX)
        _beam(f"box_spoke_aft_{k}", P(nc["box_r"], a, z_aft), P(inner, a, z_aft),
              nc["longeron_t"] * 0.7, coll, C_BOX)

    for s in range(nc["count"]):
        th = TAU * s / nc["count"]
        px, py = P(axis_r, th, 0)[:2]
        node_pt = P(axis_r, th, zc)
        lands = []
        for sz in (1.0, -1.0):
            for sy in (1.0, -1.0):
                land = P(inner, th + sy * splay, zc + sz * dz)
                lands.append(land)
                _beam(f"strut_{s}_{int(sz)}_{int(sy)}", node_pt, land, nc["strut_t"], coll, C_STRUT)
        for a, b in ((0, 1), (2, 3), (0, 2), (1, 3)):
            _beam(f"hardpoint_{s}_{a}{b}", lands[a], lands[b], nc["longeron_t"] * 0.8, coll, C_BELT)
        node = _torus(f"node_{s}", nc["node_r"], nc["node_minor"], 0.0, coll, C_DRIVE, nu=20, nv=10)
        node.location = (px, py, zc)
        node.rotation_euler = (0.0, math.pi / 2, th)
        pod = _cyl_surface(f"nacelle_{s}", nc["pod_r"], -half, half, coll, C_NAC,
                           verts=28, caps=True)
        pod.location = (px, py, zc)
        noz_aft = _cone(f"nac_noz_aft_{s}", nc["pod_r"] * 0.95, -half - nc["nozzle_len"],
                        -half, coll, C_DRIVE)
        noz_aft.location = (px, py, zc)
        noz_fore = _cone(f"nac_noz_fore_{s}", nc["pod_r"] * 0.95, half + nc["nozzle_len"],
                         half, coll, C_DRIVE)
        noz_fore.location = (px, py, zc)
        for k in range(nc["rad_n"]):
            a = th + TAU * (k + 0.5) / nc["rad_n"]
            _box(f"nac_rad_{s}_{k}",
                 (nc["rad_span"], nc["rad_thick"], nc["rad_len"]),
                 (px + 0.65 * nc["pod_r"] * math.cos(a),
                  py + 0.65 * nc["pod_r"] * math.sin(a), zc),
                 coll, C_NAC_RAD, rotz=a)
    return {"nacelles": nc["count"], "z_center": round(zc, 1),
            "axis_r": round(axis_r, 1), "ring_sep_m": round(2 * dz, 1),
            "strut_len_m": round(math.hypot(dz, dz), 1)}


def build_sensors(coll, spec, zones):
    s = spec["sensors"]
    nc = spec["nacelle"]
    R = spec["radius_outer"]
    z_reac0, z_reac1, _, _ = zones["reactor"]
    zc = (z_reac0 + z_reac1) / 2
    axis_r = R + nc["stand_off"] + nc["pod_r"]
    half = nc["pod_len"] / 2
    aesa = s["aesa_m"]
    off_y = nc["pod_r"] + aesa / 2 + 0.8
    z_face = zc + 10.0

    for side, th in enumerate((0.0, math.pi)):
        px, py = P(axis_r, th, 0)[:2]
        # Both packages sit on world +Y so the faces look +Z and clear the bells.
        _box(f"aesa_{side}", (aesa, aesa, s["aesa_thick"]),
             (px, py + off_y, z_face), coll, C_RADAR)
        _box(f"thz_{side}", (s["thz_m"], s["thz_m"], s["thz_thick"]),
             (px, py + off_y, z_face + aesa / 2 + s["thz_m"] / 2 + 0.4),
             coll, C_RADAR)
        fel = _cyl_surface(f"fel_dir_{side}", s["fel_r"], 0.0, s["fel_len"],
                           coll, C_FEL, verts=16, caps=True)
        fel.location = (px, py + off_y + aesa / 2 + s["fel_r"] + 0.6,
                        z_face + 2.0)
        # Visor parked along the pod, not covering the face — cruise is look.
        _box(f"shutter_{side}", (aesa + 0.4, s["shutter_thick"], aesa + 0.4),
             (px, py + off_y + aesa / 2 + 0.5, zc - 4.0), coll, C_SHUT)

    # FEL machines in the torque box. The A-frame tubes are the beam path.
    for side, th in enumerate((0.0, math.pi)):
        px, py = P(axis_r, th, 0)[:2]
        mach = _cyl_surface(f"fel_machine_{side}", s["machine_r"],
                            -s["machine_len"] / 2, s["machine_len"] / 2,
                            coll, C_FEL_MACH, verts=20, caps=True)
        mach.location = (math.copysign(spec["core_radius"] + s["machine_r"] + 1.2, px),
                         0.0, zc)
        _beam(f"fel_feed_{side}",
              (math.copysign(nc["box_r"], px), 0.0, zc),
              (px, py + off_y, z_face),
              0.7, coll, C_FEL)
    return {"aesa": 2, "fel_directors": 2, "fel_machines": 2, "aperture_m": aesa}


def build_hangar(coll, spec, zones):
    R = spec["radius_outer"]
    z0, z1, _, _ = zones["cargo_aft"]
    zc = (z0 + z1) / 2
    _box("hangar_door", (18.0, 1.2, 28.0), (0.0, -(R - 0.4), zc), coll, C_DOOR)
    return {"z": round(zc, 1)}


def _set_viewport():
    try:
        for area in bpy.context.screen.areas:
            if area.type == 'VIEW_3D':
                for sp in area.spaces:
                    if sp.type == 'VIEW_3D':
                        sp.shading.type = 'SOLID'
                        sp.shading.color_type = 'OBJECT'
                        sp.shading.show_object_outline = True
                        sp.clip_end = 12000.0
    except Exception:
        pass


def _frame_camera(spec):
    L = spec["length"]
    cam_data = bpy.data.cameras.new("Camera")
    cam_data.clip_end = 12000.0
    cam_data.lens = 35.0
    cam = bpy.data.objects.new("Camera", cam_data)
    bpy.context.scene.collection.objects.link(cam)
    cam.location = (380.0, -1600.0, 60.0)
    direction = mathutils.Vector((0.0, 0.0, 0.0)) - cam.location
    cam.rotation_euler = direction.to_track_quat('-Z', 'Y').to_euler()
    bpy.context.scene.camera = cam
    sun_data = bpy.data.lights.new("Sun", 'SUN')
    sun_data.energy = 4.0
    sun = bpy.data.objects.new("Sun", sun_data)
    sun.rotation_euler = (math.radians(50), 0.0, math.radians(30))
    bpy.context.scene.collection.objects.link(sun)


def build():
    t0 = time.perf_counter()
    _fresh_scene()
    bpy.context.scene.unit_settings.system = "METRIC"
    R = SPEC["radius_outer"]; L = SPEC["length"]
    sz = structural_sizing(SPEC)
    zones = zone_map(SPEC)
    hull = _collection("SHIP_hull_zones"); struct = _collection("SHIP_structure")
    colls = (_collection("SHIP_cargo_support"), _collection("SHIP_cargo_arc"),
             _collection("SHIP_cargo_rail"), _collection("SHIP_cargo"),
             _collection("SHIP_cargo_heavy"), _collection("SHIP_cargo_door"),
             _collection("SHIP_inner_hull"))
    _cyl_surface("Core", SPEC["core_radius"], -L / 2, L / 2, struct, C_CORE, caps=True)
    for name, (z0, z1, color, _) in zones.items():
        _cyl_surface(f"zone_{name}", R, z0, z1, hull, color)
    _cone("Nose_cap", R, L / 2, L / 2 + SPEC["nose_cap_len"], struct, (0.40, 0.60, 0.90))
    _cyl_surface("Tail_cap", R, -L / 2 - SPEC["tail_len"], -L / 2, struct, (0.90, 0.45, 0.25), caps=True)

    z0f, z1f, _, _ = zones["cargo_fore"]
    # 80 m sample of the forward bay — a full 300 m hold is tens of thousands
    # of objects and does not change the argument.
    sec = build_cargo(z0f + 15, z0f + 95, colls, SPEC, sz)
    tanksec = build_tanks(_collection("SHIP_fuel"), SPEC, zones)
    crewsec = build_crew(_collection("SHIP_crew"), SPEC, zones)
    rsec = build_reactor(_collection("SHIP_reactor"), _collection("SHIP_reactor_fuel"), SPEC, zones)
    radsec = build_radiator(_collection("SHIP_radiator"), SPEC, zones)
    nacsec = build_nacelles(_collection("SHIP_nacelle"), SPEC, zones)
    sensec = build_sensors(_collection("SHIP_sensors"), SPEC, zones)
    hansec = build_hangar(_collection("SHIP_hangar"), SPEC, zones)
    _frame_camera(SPEC)
    _set_viewport()
    try:
        bpy.ops.ed.undo_push(message="haunted-forest: build ship")
    except Exception:
        pass
    return {
        "radius_m": R, "length_m": L, "objects": len(bpy.data.objects),
        "build_s": round(time.perf_counter() - t0, 2),
        "sizing": sz, "section": sec, "fuel": tanksec, "crew": crewsec,
        "reactor": rsec, "radiator": radsec, "nacelles": nacsec,
        "mount_mass_t": drive_mount_mass(SPEC),
        "sensors": sensec, "hangar": hansec,
    }


result = build()
