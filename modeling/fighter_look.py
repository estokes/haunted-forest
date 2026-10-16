# The fighter as it would look: modeling/fighter.py's outer lines, smoothed
# and skinned, with what an eye reads an aircraft by: inlet cones and dark
# throats, the laser's turret, sensor windows where a canopy would be,
# panel lines for doors and control surfaces, the swivel nozzle, wingtip
# thrusters; a dark high-emissivity coating; sky, sun and cameras.
#
# Headless:  blender -b --factory-startup --python modeling/fighter_look.py -- [--stretch K] [--render DIR] [--save FILE] [--samples N]

import math
import os
import sys

import bpy
from mathutils import Vector

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import fighter  # noqa: E402

COATING = (0.040, 0.042, 0.046)


def material(name, color, metallic=0.3, roughness=0.5, emission=None):
    m = bpy.data.materials.new(name)
    m.use_nodes = True
    b = m.node_tree.nodes["Principled BSDF"]
    b.inputs["Base Color"].default_value = (*color, 1)
    b.inputs["Metallic"].default_value = metallic
    b.inputs["Roughness"].default_value = roughness
    if emission:
        b.inputs["Emission Color"].default_value = (*emission[0], 1)
        b.inputs["Emission Strength"].default_value = emission[1]
    m.diffuse_color = (*color, 1)
    return m


def coating():
    """The carbide skin: near-black, a little metallic, its roughness
    broken up so the light moves across it."""
    m = material("coating", COATING, metallic=0.12, roughness=0.5)
    nt = m.node_tree
    b = nt.nodes["Principled BSDF"]
    noise = nt.nodes.new("ShaderNodeTexNoise")
    noise.inputs["Scale"].default_value = 3.0
    ramp = nt.nodes.new("ShaderNodeMapRange")
    ramp.inputs["To Min"].default_value = 0.42
    ramp.inputs["To Max"].default_value = 0.62
    nt.links.new(noise.outputs["Fac"], ramp.inputs["Value"])
    nt.links.new(ramp.outputs["Result"], b.inputs["Roughness"])
    return m


def link(obj, mat, coll):
    if obj.name not in coll.objects:
        coll.objects.link(obj)
    obj.data.materials.clear()
    obj.data.materials.append(mat)
    return obj


def smooth(obj, angle=35.0):
    for p in obj.data.polygons:
        p.use_smooth = True
    mod = obj.modifiers.new("edges", "EDGE_SPLIT")
    mod.split_angle = math.radians(angle)


def snap(tree, x, y, from_top):
    """The skin's point above or below (x, y), and its normal."""
    z0, d = (6.0, Vector((0, 0, -1))) if from_top else (-6.0, Vector((0, 0, 1)))
    hit = tree.ray_cast(Vector((x, y, z0)), d)
    return (hit[0], hit[1]) if hit[0] is not None else (None, None)


def line(name, pts, coll, mat, width=0.006):
    """A panel line through `pts`, lifted off the skin a hair."""
    pts = [p for p in pts if p is not None]
    if len(pts) < 2:
        return None
    cu = bpy.data.curves.new(name, "CURVE")
    cu.dimensions = "3D"
    cu.bevel_depth = width
    sp = cu.splines.new("POLY")
    sp.points.add(len(pts) - 1)
    for p, q in zip(sp.points, pts):
        p.co = (*q, 1)
    o = bpy.data.objects.new(name, cu)
    coll.objects.link(o)
    o.data.materials.append(mat)
    return o


def skin_line(tree, name, xy, from_top, coll, mat, steps=12):
    """A panel line on the skin through the (x, y) corners, densified."""
    pts = []
    for (x0, y0), (x1, y1) in zip(xy, xy[1:]):
        for i in range(steps):
            t = i / steps
            p, n = snap(tree, x0 + t * (x1 - x0), y0 + t * (y1 - y0), from_top)
            if p is not None:
                pts.append(p + n * 0.003)
    p, n = snap(tree, *xy[-1], from_top)
    if p is not None:
        pts.append(p + n * 0.003)
    return line(name, pts, coll, mat)


def rect(x0, x1, y0, y1):
    return [(x0, y0), (x1, y0), (x1, y1), (x0, y1), (x0, y0)]


def disc(name, centre, normal, radius, coll, mat, k=32):
    n = Vector(normal).normalized()
    a = n.orthogonal().normalized()
    b = n.cross(a)
    c = Vector(centre)
    verts = [c + (a * math.cos(2 * math.pi * i / k) + b * math.sin(2 * math.pi * i / k)) * radius
             for i in range(k)]
    return link(fighter.mesh_object(name, verts, [tuple(range(k))], coll, (0, 0, 0)), mat, coll)


def build(scale=1.0):
    fighter.build(scale)
    for c in ("FIGHTER_equipment", "FIGHTER_tanks", "FIGHTER_ducts"):
        for o in bpy.data.collections[c].objects:
            o.hide_render = True
            o.hide_set(True)
    look = fighter.collection("FIGHTER_look")
    skin_m = coating()
    black = material("inlet", (0.004, 0.004, 0.005), metallic=0.0, roughness=1.0)
    black.node_tree.nodes["Principled BSDF"].inputs["Specular IOR Level"].default_value = 0.0
    lines_m = material("panel", (0.012, 0.012, 0.014), metallic=0.2, roughness=0.6)
    metal = material("nozzle", (0.16, 0.14, 0.12), metallic=0.85, roughness=0.32)
    glass = material("window", (0.01, 0.015, 0.03), metallic=0.0, roughness=0.05)
    cone_m = material("cone", (0.07, 0.07, 0.075), metallic=0.6, roughness=0.3)

    oml = bpy.data.objects["airframe"]
    oml.data.materials.clear()
    oml.data.materials.append(skin_m)
    oml.data.materials.append(black)
    # The inlet mouths: faces of the skin facing forward at the mouth plane.
    mouth = fighter.INLET["mouth_x"] * scale
    for p in oml.data.polygons:
        if abs(p.center.x - mouth) < 0.01 and p.normal.x < -0.9:
            p.material_index = 1
    smooth(oml)
    oml.color = (*COATING, 1)
    tree = fighter.bvh(oml)

    # Inlet cones, the Mirage's souris: half-cones against the flank, at
    # the centre of the half-round capture.
    y, z = fighter.hw_at(mouth, scale), fighter.duct_point(mouth, scale)[1]
    for side in (-1, 1):
        rings = []
        for i in range(8):
            u = i / 7
            x = mouth - 0.62 + 0.62 * u
            r = 0.005 + 0.26 * u
            ring = [(y - 0.05 + r * math.cos(t), z + r * math.sin(t)) for t in [math.pi * (j / 24 - 0.5) for j in range(25)]]
            ring += [(y - 0.05, z + r * (1 - 2 * j / 8)) for j in range(1, 8)]
            rings.append([(x, side * a, b) for a, b in (ring if side > 0 else reversed(ring))])
        cone = fighter.ring_loft(f"cone_{side}", rings, look, (0, 0, 0))
        link(cone, cone_m, look)
        smooth(cone, 60)

    # The laser's turret on the forward spine, its window facing up and ahead.
    tx = fighter.EQUIPMENT["fel_director"][2][0] * scale
    top, n = snap(tree, tx, 0.0, True)
    bpy.ops.mesh.primitive_uv_sphere_add(radius=0.30, location=top - Vector((0, 0, 0.12)), segments=48, ring_count=24)
    dome = bpy.context.active_object
    dome.name = "turret"
    dome.scale = (1.0, 1.0, 0.75)
    for c in dome.users_collection:
        c.objects.unlink(dome)
    link(dome, skin_m, look)
    smooth(dome, 80)
    disc("turret_window", top + Vector((-0.09, 0, 0.07)), (-0.6, 0, 1), 0.13, look, glass)

    # Sensor windows where a canopy would be: two each side of the nose, a
    # chin pair, and two on the spine looking up and back.
    end = fighter.FUSELAGE[-1][0] * scale
    for x, y, top_side in [(2.2, 0.35, True), (2.2, -0.35, True), (3.4, 0.55, True), (3.4, -0.55, True),
                           (2.8, 0.25, False), (2.8, -0.25, False), (12.0, 0.25, True), (12.0, -0.25, True)]:
        p, nn = snap(tree, x * end / 14.2, y, top_side)
        if p is not None:
            disc(f"eye_{x}_{y}", p + nn * 0.004, nn, 0.07, look, glass)

    # Panel lines: lift-fan doors above and below, the capsule hatch, bays,
    # gear doors, elevons, rudder.
    fx = fighter.EQUIPMENT["lift_fan"][2][0] * scale
    for from_top in (True, False):
        tag = "top" if from_top else "bottom"
        skin_line(tree, f"fan_doors_{tag}", rect(fx - 0.82, fx + 0.82, -0.82, 0.82), from_top, look, lines_m)
        skin_line(tree, f"fan_split_{tag}", [(fx - 0.82, 0.0), (fx + 0.82, 0.0)], from_top, look, lines_m)
    cx = fighter.EQUIPMENT["capsule"][2][0] * scale
    skin_line(tree, "hatch", rect(cx - 0.72, cx + 0.72, -0.30, 0.30), True, look, lines_m)
    for side in (-1, 1):
        bx = fighter.EQUIPMENT["bay_r"][2][0] * scale
        skin_line(tree, f"bay_{side}", rect(bx - 1.32, bx + 1.32, side * 1.58, side * 2.12), False, look, lines_m)
        skin_line(tree, f"bay_split_{side}", [(bx - 1.32, side * 1.85), (bx + 1.32, side * 1.85)], False, look, lines_m)
        gx = fighter.EQUIPMENT["main_gear_r"][2][0] * scale
        skin_line(tree, f"gear_{side}", rect(gx - 0.68, gx + 0.68, side * 0.44, side * 0.90), False, look, lines_m)
    nx = fighter.EQUIPMENT["nose_gear"][2][0] * scale
    skin_line(tree, "nose_gear", rect(nx - 0.55, nx + 0.55, -0.22, 0.22), False, look, lines_m)

    w = fighter.WING
    tan = math.tan(math.radians(w["sweep_le"]))
    le = lambda y: (w["apex"] + abs(y) * tan) * scale
    chord = lambda y: max(0.3, w["root"] - abs(y) * tan) * scale
    for side in (-1, 1):
        for from_top in (True, False):
            tag = f"{side}_{'top' if from_top else 'bottom'}"
            hinge = [(le(y) + 0.78 * chord(y), side * y) for y in [1.9 + 0.2 * i for i in range(13)]]
            skin_line(tree, f"elevon_hinge_{tag}", hinge, from_top, look, lines_m, steps=3)
            for y in (1.9, 3.1, 4.3):
                skin_line(tree, f"elevon_edge_{tag}_{y}", [(le(y) + 0.78 * chord(y), side * y),
                                                          (le(y) + chord(y) - 0.01, side * y)],
                          from_top, look, lines_m, steps=4)

    f = fighter.FIN
    ftan = math.tan(math.radians(f["sweep_le"]))
    pts = []
    for i in range(10):
        h = f["z"] + 0.15 + (f["height"] - 0.35) * i / 9
        x_le = f["root_x"] + (h - f["z"]) * ftan
        c = max(0.5, f["root"] - (h - f["z"]) * ftan)
        pts.append(((x_le + 0.68 * c) * scale, h * scale))
    for side in (-1, 1):
        hit = [tree.ray_cast(Vector((x, side * 2.0, z)), Vector((0, -side, 0))) for x, z in pts]
        line(f"rudder_{side}", [h[0] + h[1] * 0.003 for h in hit if h[0] is not None], look, lines_m)

    # The swivel nozzle: the three-bearing duct's segments and the petals,
    # dark metal, its lip turning in to a black throat well inside.
    rings = []
    for dx, r in [(-0.45, 0.46), (-0.15, 0.46), (0.15, 0.455), (0.42, 0.45), (0.75, 0.42), (0.85, 0.415),
                  (0.85, 0.39), (0.65, 0.37), (0.10, 0.30)]:
        rings.append([(end + dx * scale, r * math.cos(t), -0.05 + r * math.sin(t))
                      for t in [2 * math.pi * j / 48 for j in range(48)]])
    noz = fighter.ring_loft("nozzle", rings, look, (0, 0, 0))
    link(noz, metal, look)
    noz.data.materials.append(black)
    for p in noz.data.polygons:
        if math.hypot(p.center.y, p.center.z + 0.05) < 0.40:
            p.material_index = 1
    smooth(noz, 50)
    for i, x in enumerate((end - 0.15 * scale, end + 0.15 * scale, end + 0.42 * scale)):
        tilt = 0.0 if i == 2 else (0.18 if i == 0 else -0.18)
        r = 0.465
        line(f"bearing_{i}", [(x + tilt * math.sin(t) * r, r * math.cos(t), -0.05 + r * math.sin(t))
                              for t in [2 * math.pi * j / 48 for j in range(49)]], look, lines_m, 0.008)
    for j in range(16):
        t = 2 * math.pi * j / 16
        line(f"petal_{j}", [(end + 0.42 * scale, 0.453 * math.cos(t), -0.05 + 0.453 * math.sin(t)),
                            (end + 0.85 * scale, 0.418 * math.cos(t), -0.05 + 0.418 * math.sin(t))], look, lines_m, 0.005)

    # Wingtip thrusters: a pod on each tip with nozzles out, up and down.
    tip_x = le(w["semispan"]) + 0.15 * scale
    for side in (-1, 1):
        y = side * w["semispan"] * scale
        rings = [[(tip_x + dx, y + r * math.cos(t), w["z"] * scale + r * math.sin(t))
                  for t in [2 * math.pi * j / 24 for j in range(24)]]
                 for dx, r in [(-0.05, 0.01), (0.10, 0.06), (0.55, 0.06), (0.65, 0.03)]]
        pod = fighter.ring_loft(f"rcs_{side}", rings, look, (0, 0, 0))
        link(pod, skin_m, look)
        smooth(pod, 60)
        for d in ((0, side, 0), (0, 0, 1), (0, 0, -1)):
            disc(f"rcs_port_{side}_{d}", Vector((tip_x + 0.33, y, w["z"] * scale)) + Vector(d) * 0.061, d, 0.022, look, black)

    oml.hide_set(False)
    oml.hide_render = False
    return look


def stage(samples):
    sc = bpy.context.scene
    sc.render.engine = "CYCLES"
    sc.cycles.samples = samples
    sc.cycles.use_denoising = True
    sc.render.resolution_x, sc.render.resolution_y = 1920, 1080
    sc.view_settings.view_transform = "AgX"
    world = bpy.data.worlds.new("sky")
    sc.world = world
    world.use_nodes = True
    nt = world.node_tree
    sky = nt.nodes.new("ShaderNodeTexSky")
    sky.sky_type = "NISHITA" if "NISHITA" in [i.identifier for i in sky.bl_rna.properties["sky_type"].enum_items] else sky.sky_type
    sky.sun_elevation = math.radians(28)
    sky.sun_rotation = math.radians(140)
    # The sky's own sun lights the aircraft; scaled down, it leaves a dark
    # coating dark, with a real highlight.
    sky.sun_disc = True
    nt.links.new(sky.outputs["Color"], nt.nodes["Background"].inputs["Color"])
    nt.nodes["Background"].inputs["Strength"].default_value = 0.06
    sc.view_settings.exposure = 0.3


def shoot(directory):
    sc = bpy.context.scene
    cam_data = bpy.data.cameras.new("look_cam")
    cam = bpy.data.objects.new("look_cam", cam_data)
    sc.collection.objects.link(cam)
    sc.camera = cam
    k = fighter.FUSELAGE[-1][0] / 14.2
    target = Vector((7.6 * k, 0.0, 0.0))
    shots = {
        "front_three_quarter": ((-7.0, -11.0, 5.5), 50),
        "rear_three_quarter": ((27.0, 13.0, 6.0), 50),
        "side": ((7.6, -26.0, 0.6), 50),
        "above": ((7.6, -4.0, 26.0), 50),
        "below": ((-3.0, -17.0, -9.0), 50),
    }
    for name, (loc, lens) in shots.items():
        loc = target + (Vector(loc) - Vector((7.6, 0, 0))) * k
        cam.location = loc
        cam_data.lens = lens
        cam.rotation_euler = (target - Vector(loc)).to_track_quat("-Z", "Y").to_euler()
        sc.render.filepath = os.path.join(directory, f"look_{name}.png")
        bpy.ops.render.render(write_still=True)


def main():
    a = sys.argv[sys.argv.index("--") + 1:] if "--" in sys.argv else []
    opt = lambda k, d=None: a[a.index(k) + 1] if k in a else d
    if opt("--stretch"):
        fighter.stretch(float(opt("--stretch")))
    build()
    stage(int(opt("--samples", "48")))
    if opt("--save"):
        bpy.ops.wm.save_as_mainfile(filepath=opt("--save"))
    if opt("--render"):
        shoot(opt("--render"))


if __name__ == "__main__":
    main()
