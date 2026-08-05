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
C_ENG = (0.88, 0.56, 0.30)


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
    # REACTOR: one D-3He tandem mirror on the spine (closed-cycle power plant).
    # The 354 m burning column runs down the axis; the main-drive deuterium fuel
    # farm fills the annulus around it (feeds the reactor's own D need too, and
    # neutron-shields everything outboard). Terminates fore at a fuel-shielded
    # engineering deck carrying the manual fuel shutoff.
    "reactor": {
        "aft_choke_z": -420.0, "burn_len": 354.0,
        "plasma_r": 3.0, "chamber_r": 5.0,
        "coil_r": 7.0, "coil_minor": 0.9, "coil_pitch": 15.0,
        "choke_r": 8.5, "choke_minor": 1.8,
        "plug_len": 12.0, "plug_r": 3.0, "plug_coil_r": 5.0, "plug_coil_minor": 1.1,
        "conv_len": 18.0, "conv_r": 12.0,
        "fuel_r_in": 9.0, "fuel_r_out": 24.0, "eng_deck_len": 10.0,
    },
}


def _beam_mass(side, length, count, rho):
    return side * side * length * count * rho


def structural_sizing(spec):
    g = 9.81
    m = spec["container_mass_t"] * 1000.0
    E = spec["material_E_Pa"]; rho = spec["material_density"]
    sf = spec["safety_factor"]; sig = spec["material_strength_Pa"] / sf
    cw, cl = spec["container"]["W"], spec["clearance"]
    zp = spec["container"]["L"] + 0.5
    R = spec["radius_outer"] - spec["barrel_thickness"]   # inner-hull skin radius
    core = spec["core_radius"]; ns = spec["n_sectors"]; bay = spec["cargo_bay_len"]
    F_axial = m * spec["max_accel_G"] * g
    F_lat = m * spec["lateral_G"] * g

    # STRINGER (rail): sized by LOCAL column buckling of ONE box's axial load.
    # Shear-tied to the skin each frame so it never accumulates the column. The
    # CAGE (mid-bay frame rings + meridional boundary diagonals) braces it at
    # L_eff = zp/2 -> the design value; L_eff = zp is the un-braced reference.
    def rail_for(leff):
        return max(0.08, (12 * F_axial * sf * leff ** 2 / (math.pi ** 2 * E)) ** 0.25)
    a_rail = rail_for(zp / 2)                        # braced: the design
    a_rail_unbraced = rail_for(zp)                  # reference (no cage)

    # FRAME (radial strut / hoop / diagonal): lateral 2 G + stabilisation. The
    # triangulated cage carries lateral as axial spar force, so this is modest --
    # a small bending beam over a radial cell, floored to stay stiffer than the
    # stringer.
    M_lat = (2 * F_lat) * (cw + 1.0) / 4.0
    a_strut = max((6 * M_lat / sig) ** (1 / 3.0), 1.2 * a_rail)
    a_arc = a_rail
    a_conn = max(0.06, 0.8 * a_rail)

    # DERIVED radial packing (mirrors build_cargo) -> cargo count per bay.
    rail_off = a_arc / 2 + cl + a_rail / 2
    ring_gap = 2 * rail_off + a_rail
    ring_pitch = cw + ring_gap
    rings = arange(core + 1.5, R - 0.4, ring_pitch)
    n_lines = 0
    for i in range(len(rings) - 1):
        ri, ro = rings[i], rings[i + 1]; rc = (ri + ro) / 2
        margin = (cw / 2 + a_strut / 2 + 2 * cl) / rc
        span = TAU / ns - 2 * margin
        n_lines += max(1, int(span * rc / (cw + cl))) * ns
    n_z = int(bay / zp)
    n_boxes = n_lines * n_z
    P_bay = n_boxes * F_axial                       # total axial into the skin

    # SKIN (inner hull): axial-compression shell buckling sets it. sigma_cr =
    # gamma*0.6*E*t/R, sigma_app*sf = P_bay*sf/(2*pi*R*t). R cancels ->
    # t = sqrt(P_bay*sf / (2*pi*gamma*0.6*E)); gamma = 0.5 knockdown.
    gamma = 0.5
    t_skin = math.sqrt(P_bay * sf / (2 * math.pi * gamma * 0.6 * E))

    # masses per bay (t) for the braced-cage geometry actually drawn: arcs +
    # connectors at zp/2 (mid-bay bracing), radial struts + hoops at zp, one
    # meridional diagonal per boundary strut-bay, skin on a linear taper (0.5).
    n_arc_st = 2 * n_z + 1
    n_str_st = n_z + 1
    hyp = math.hypot(R - core, zp)
    m_rail = _beam_mass(a_rail, bay, 2 * n_lines, rho)
    m_strut = _beam_mass(a_strut, R - core, ns * n_str_st, rho)
    m_hoop = _beam_mass(a_strut, TAU * R, n_str_st, rho)
    m_diag = _beam_mass(a_strut, hyp, 2 * ns * (n_str_st - 1), rho)  # X cross-brace
    m_arc = _beam_mass(a_arc, n_arc_st * TAU * sum(rings), 1, rho)
    m_conn = _beam_mass(a_conn, rail_off, 2 * n_lines * n_arc_st, rho)
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
        "tracks_per_sector": len(rings) - 1,
        "rail_lines_per_bay": n_lines,
        "boxes_per_bay": n_boxes,
        "capacity_both_bays": n_boxes * 2,
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
    c_sup, c_arc, c_rail, c_cargo, c_hot, c_door, c_skin = colls
    inner = spec["radius_outer"] - spec["barrel_thickness"]
    core = spec["core_radius"]
    ns = spec["n_sectors"]
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
    # LOADING END: the door sits forward (+Z) of the racks and opens into a clear
    # staging segment one door-height deep. The loader brings boxes in/out there
    # and indexes them aft onto the rails. Cargo + frame stop at cz1; the segment
    # [cz1, z1] is the empty staging concourse.
    door_h = spec["container"]["L"] + 2.0
    cz1 = z1 - door_h                                 # fore end of cargo / staging boundary
    z_lo = z0 + 5.0                                   # aft pad
    nz = max(1, round((cz1 - z_lo) / zp))             # layers that fill [z_lo, cz1] exactly
    bp = (cz1 - z_lo) / nz                            # actual bay pitch (~zp, no slack)
    zsts = [z_lo + bp * (k + 0.5) for k in range(nz)]        # container centres
    frame_z = [z_lo + bp * k for k in range(nz + 1)]        # struts + diagonals
    arc_z = [z_lo + bp * k / 2 for k in range(2 * nz + 1)]  # arcs + ties @ mid-bay brace
    zc_door = (cz1 + z1) / 2

    # RADIAL SUPPORTS carry the load, bonded to the hull. Struts sit ONLY at the
    # frame stations (= container fore/aft faces), so a box slides out between
    # them. No end-cap struts — those landed in the empty bay mouths fore/aft of
    # the cargo and braced nothing.
    strut_zs = list(frame_z)
    for s in range(ns):
        a = TAU * s / ns
        # struts bond DIRECTLY to both the core and the inner hull (no chords)
        for z in strut_zs:
            _beam(f"strut_{s}_{int(z)}", P(core, a, z), P(r_out, a, z), t_strut, c_sup, C_STRUT)
        # CAGE: full X cross-brace in every bay of the sector-boundary plane, so
        # EVERY strut node is triangulated at BOTH the core and hull ends. A
        # single zig-zag diagonal leaves alternate nodes braced at only one end,
        # forcing them to resist racking in bending. The X also suits the
        # midcourse gravity REVERSAL: whichever diagonal is in tension carries
        # the shear either way. It lives in the boundary plane, so it never
        # crosses a container lane.
        for k in range(len(strut_zs) - 1):
            za, zb = strut_zs[k], strut_zs[k + 1]
            _beam(f"diagA_{s}_{int(za)}", P(core, a, za), P(r_out, a, zb), t_strut, c_sup, C_DIAG)
            _beam(f"diagB_{s}_{int(za)}", P(r_out, a, za), P(core, a, zb), t_strut, c_sup, C_DIAG)
    # INNER-HULL frames: hoops at r=inner that the struts land on, plus the
    # structural SKIN at r=inner that actually carries the accumulating axial
    # column as membrane compression. The ~2 m out to the outer hull
    # (radius_outer) is the water-Whipple barrel — the double hull.
    for z in strut_zs + [z1]:                          # + fore end-ring at the staging mouth
        _arc(f"innerhull_{int(z)}", inner, 0.0, TAU, z, t_strut, c_sup, C_STRUT, segs=48)
    _cyl_surface("inner_hull_skin", inner, z0, z1, c_skin, C_SKIN, verts=96)

    n = mass = mom_x = mom_y = 0
    per_track = []
    for s in range(ns):                               # cargo now fills ALL sectors
        f = 1.0
        a0, a1 = TAU * s / ns, TAU * (s + 1) / ns
        for rr in rings:
            for zf in arc_z:
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
                    _beam(f"rail_{s}_{i}_{j}_{int(rr)}", P(rr, th, z0 + 4), P(rr, th, cz1),
                          t_rail, c_rail, C_RAIL)
                    # connectors bond each rail to its arc at every frame; seat
                    # them into the arc (past its centre) so they positively meet
                    ext = arc_r + math.copysign(t_arc / 2, arc_r - rr)
                    for zf in arc_z:
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

    # a container mid-load, sitting in the staging concourse just inside the door
    _box("loading_container", (cw, cw, spec["container"]["L"]),
         (0, -(inner - 6.0), zc_door), colls[3], (0.20, 0.95, 0.35))

    # DOOR forward of the racks, opening into the staging segment; the loader
    # works this concourse (radial + tangential) then indexes boxes aft.
    _box("bay_door", (6, 1, door_h), (0, -(inner - 0.3), zc_door), c_door, C_DOOR)
    _box("loader", (3.2, 9, 3.2), (0, -(inner - 5.5), zc_door), c_door, C_LOAD)
    tps = sum(t["per_sector"] for t in per_track)
    return {"containers_shown": n, "tracks": len(per_track), "per_track": per_track,
            "ring_pitch_m": round(ring_pitch, 2), "per_sector_layer": tps,
            "full_bay_capacity_est": tps * ns * int(spec["cargo_bay_len"] / zp)}


def build_reactor(coll, fuel_coll, spec):
    rc = spec["reactor"]
    z_aft = rc["aft_choke_z"]
    z_fore = z_aft + rc["burn_len"]                  # fore choke, at the engineering deck
    chr_ = rc["chamber_r"]
    # burning plasma column + vacuum vessel / first wall
    _cyl_surface("reactor_plasma", rc["plasma_r"], z_aft, z_fore, coll, C_PLASMA, verts=32)
    _cyl_surface("reactor_chamber", chr_, z_aft, z_fore, coll, C_CHAMBER, verts=48)
    # central-cell coils along the burning length (energize more/fewer -> throttle)
    for z in arange(z_aft + rc["coil_pitch"], z_fore - rc["coil_pitch"] + 1e-6, rc["coil_pitch"]):
        _torus(f"cc_coil_{int(z)}", rc["coil_r"], rc["coil_minor"], z, coll, C_COIL)
    # end chokes (high-field mirror coils, ~27 T): a pair at each end
    for zc in (z_fore, z_aft):
        for dz in (-2.5, 2.5):
            _torus(f"choke_{int(zc)}_{int(zc + dz)}", rc["choke_r"], rc["choke_minor"], zc + dz, coll, C_CHOKE)
    # plug cells + direct-converter expanders beyond each choke; leaked plasma
    # streams into the converters (confinement geometry == conversion geometry)
    for sgn, zc in ((1.0, z_fore), (-1.0, z_aft)):
        zp0 = zc + sgn * 3.0
        zp1 = zp0 + sgn * rc["plug_len"]
        _cyl_surface(f"plug_bore_{int(zc)}", rc["plug_r"], min(zp0, zp1), max(zp0, zp1), coll, C_CHAMBER, verts=24)
        for frac in (0.35, 0.65):
            zp = zp0 + sgn * rc["plug_len"] * frac
            _torus(f"plugcoil_{int(zc)}_{int(zp)}", rc["plug_coil_r"], rc["plug_coil_minor"], zp, coll, C_PLUG)
        _cone(f"converter_{int(zc)}", rc["conv_r"], zp1 + sgn * rc["conv_len"], zp1, coll, C_CONV)
    # main-drive DEUTERIUM fuel farm: annular tankage wrapping the column -- also
    # the reactor's own D supply and a neutron shield for everything outboard
    _cyl_surface("drive_fuel_inner", rc["fuel_r_in"], z_aft, z_fore, fuel_coll, C_FUEL, verts=48)
    _cyl_surface("drive_fuel_outer", rc["fuel_r_out"], z_aft, z_fore, fuel_coll, C_FUEL, verts=64)
    # engineering deck forward of the fore converter, behind a fuel-tank shield;
    # the manual fuel shutoff lives here, shielded from the reactor by its fuel
    z_shield = z_fore + 3.0 + rc["plug_len"] + rc["conv_len"]
    _cyl_surface("reactor_fuel_shield", rc["conv_r"], z_shield, z_shield + 4.0, fuel_coll, C_FUEL, verts=32, caps=True)
    z_ed = z_shield + 4.0
    _cyl_surface("eng_deck", rc["conv_r"] * 0.8, z_ed, z_ed + rc["eng_deck_len"], coll, C_ENG, verts=32, caps=True)
    return {"burn_len_m": rc["burn_len"], "column_z": [round(z_aft, 1), round(z_fore, 1)],
            "eng_deck_z": round(z_ed + rc["eng_deck_len"] / 2, 1),
            "fuel_annulus_r": [rc["fuel_r_in"], rc["fuel_r_out"]],
            "objects": len(coll.objects) + len(fuel_coll.objects)}


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
             _collection("SHIP_cargo_heavy"), _collection("SHIP_cargo_door"),
             _collection("SHIP_inner_hull"))
    _cyl_surface("Core", SPEC["core_radius"], -L / 2, L / 2, struct, C_CORE, caps=True)
    z_top = L / 2; zone_z = {}
    for name, length, color in SPEC["zones"]:
        z0 = z_top - length
        _cyl_surface(f"zone_{name}", R, z0, z_top, hull, color); zone_z[name] = (z0, z_top); z_top = z0
    _cone("Nose_cap", R, L / 2, L / 2 + SPEC["nose_cap_len"], struct, (0.40, 0.60, 0.90))
    _cyl_surface("Tail_cap", R, -L / 2 - SPEC["tail_len"], -L / 2, struct, (0.90, 0.45, 0.25), caps=True)
    z0f, _ = zone_z["cargo_fore"]
    sec = build_cargo(z0f + 15, z0f + 95, colls, SPEC, sz)
    rsec = build_reactor(_collection("SHIP_reactor"), _collection("SHIP_reactor_fuel"), SPEC)
    _set_viewport_object_color()
    try:
        bpy.ops.ed.undo_push(message="haunted-forest: build ship")
    except Exception:
        pass
    return {"radius_m": R, "length_m": L, "objects": len(bpy.data.objects),
            "sizing": sz, "section": sec, "reactor": rsec}


result = build()
