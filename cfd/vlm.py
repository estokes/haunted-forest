# The fighter's lifting surfaces by vortex lattice (AeroSandbox): the
# wing, carried through the body, and the canards, from the planform
# cfd/export.py writes. Attached flow only: no vortex lift, no
# compressibility. Prints lift slope, neutral point, induced drag, and
# what the canards add and take: their control power and the downwash
# they throw on the wing.
#
# ~/.local/opt/cfd-venv/bin/python cfd/vlm.py PLANFORM.json

import json
import math
import sys

import aerosandbox as asb


def surfaces(p, canard_twist=0.0, canards=True):
    w = p["wing"]
    tan = math.tan(math.radians(w["sweep_le"]))
    tip = w["root"] - w["semispan"] * tan
    foil = asb.Airfoil("naca0004")
    wing = asb.Wing(name="wing", symmetric=True, xsecs=[
        asb.WingXSec(xyz_le=[w["apex"], 0, w["z"]], chord=w["root"], airfoil=foil),
        asb.WingXSec(xyz_le=[w["apex"] + w["semispan"] * tan, w["semispan"], w["z"]], chord=tip, airfoil=foil),
    ])
    out = [wing]
    c = p["canard"]
    if canards and c:
        ctan = math.tan(math.radians(c["sweep"]))
        out.append(asb.Wing(name="canard", symmetric=True, xsecs=[
            asb.WingXSec(xyz_le=[c["x"], c["root_y"], c["z"]], chord=c["root"], twist=canard_twist, airfoil=foil),
            asb.WingXSec(xyz_le=[c["x"] + c["span"] * ctan, c["root_y"] + c["span"], c["z"]],
                         chord=c["root"] * c["taper"], twist=canard_twist, airfoil=foil),
        ]))
    return out


def solve(p, alpha, cg, **kw):
    w = p["wing"]
    plane = asb.Airplane(name="fighter", xyz_ref=[cg, 0, 0], wings=surfaces(p, **kw),
                         s_ref=55.37, c_ref=p["mac"], b_ref=2 * w["semispan"])
    vlm = asb.VortexLatticeMethod(airplane=plane, op_point=asb.OperatingPoint(velocity=100, alpha=alpha),
                                  spanwise_resolution=16, chordwise_resolution=16, verbose=False)
    return vlm.run()


def main():
    p = json.load(open(sys.argv[1]))
    cg = sum(p["cg"]) / 2
    for label, kw in (("wing and body", {"canards": False}), ("with canards", {})):
        a0, a1 = solve(p, 0, cg, **kw), solve(p, 4, cg, **kw)
        cla = (a1["CL"] - a0["CL"]) / math.radians(4)
        cma = (a1["Cm"] - a0["Cm"]) / math.radians(4)
        x_np = cg - cma / cla * p["mac"]
        k = a1["CD"] / a1["CL"] ** 2
        print(f"{label:14s}: CL_alpha {cla:.3f}/rad, neutral point {x_np:.2f} m "
              f"(cg {cg:.2f}, margin {100 * (x_np - cg) / p['mac']:+.1f}% of the mean chord), "
              f"induced CD/CL^2 {k:.3f} (1/(pi A) {1 / (math.pi * (2 * p['wing']['semispan']) ** 2 / 55.37):.3f})")
    if p["canard"]:
        base, up = solve(p, 4, cg), solve(p, 4, cg, canard_twist=5.0)
        print(f"canards +5°: dCL {up['CL'] - base['CL']:+.4f}, dCm {up['Cm'] - base['Cm']:+.4f} "
              f"(per degree: {(up['Cm'] - base['Cm']) / 5:+.5f})")
        bare = solve(p, 4, cg, canards=False)
        print(f"at 4°: CL with canards {base['CL']:.4f}, without {bare['CL']:.4f}; "
              f"the canards' downwash and their own lift net {base['CL'] - bare['CL']:+.4f}")


if __name__ == "__main__":
    main()
