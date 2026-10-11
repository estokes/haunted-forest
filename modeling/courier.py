# Courier mass budget and trip time (book/courier.md). The engine's magnet
# mass, gamma shadow, plate load, thrust and hits come from a hf-nozzle
# report; the rest of the ship is the brief's. Run it to print the brief's
# tables.
#
# Trips are brachistochrones at full power: burn toward the target, flip
# midway in mass (equal rapidity each half), arrive at rest. All propellant
# is annihilated (matter and anti-dust 1:1), so mass flow is P/c^2 and the
# effective exhaust is thrust / mass flow. Relativistic in rapidity.

import math
import sys

import nozzle

C = 2.998e8
LY = 9.4607e15
YR = 3.156e7

GAMMA_SHARE = 0.36             # of annihilation power, as gammas
FRAME_T_PER_M = 700.0 / 535    # engine frame and feeds, per metre of mirror ring radius
TILE_T_PER_GW = 74.0           # tiles, coil fins, helium loop per GW of conductor shadow
HIT_T_PER_PPM = 40.0           # cooling for charged power landing on anything, per ppm of P_ANN
PLATE_M = 0.10                 # plate tungsten, 30 radiation lengths: ~0.07% of each shower leaks
PLATE_COOL_T_PER_GW = 20.0     # plate helium loop and fins per GW of gammas on it
STOP_M = 7.0                   # dust behind each plate never burned: stops the leak to below cosmic rays
W_T_M3, DUST_T_M3 = 19.3, 0.6
# jackets: cooled skins on the tank sections and the far ends, which take the
# muon-decay electrons that reach the pods (hf-nozzle hits_along_ppm)
REST_T = {"power": 100.0, "structure": 500.0, "jackets": 500.0}


def engine_t(r):
    """Engine mass, t: magnets, the tiles and cooling their shadow needs, frame,
    and cooling for charged hits."""
    c = r["coil_ppm"]
    hits = r["plate_ppm"] + r["side_ppm"] + c["ring"] + c["sweeper"] + c["spoke"]
    frame = FRAME_T_PER_M * r["spec"]["rings"][0]["r"]
    return r["magnet_t"] + TILE_T_PER_GW * r["shadow_gw"] + frame + HIT_T_PER_PPM * hits


def p_ann(r):
    return r["spec"]["gamma_tw"] * 1e12 / GAMMA_SHARE


def plates_t(r):
    """Shield plates and the stops of dust behind them, t."""
    pods = r["spec"]["ship"]["pods"]
    area = pods["n"] * math.pi * pods["r"] ** 2
    return area * (PLATE_M * W_T_M3 + STOP_M * DUST_T_M3) + PLATE_COOL_T_PER_GW * r["plate_gw"]


def ship_t(r):
    """Everything but the engine and payload, t: plates, stops and the rest."""
    return plates_t(r) + sum(REST_T.values())


def dry_t(r, payload_t=850.0):
    return engine_t(r) + ship_t(r) + payload_t


def trip(ve_c, dry_kg, dist_ly, p_w, n=4000):
    """(years, propellant kg, peak v/c) for a brachistochrone of dist_ly at
    annihilation power p_w."""
    mdot = p_w / C ** 2

    def fly(fuel):
        m0 = dry_kg + fuel
        m_mid = math.sqrt(m0 * dry_kg)
        x = t = 0.0
        dm = fuel / n
        for k in range(n):
            m = m0 - (k + 0.5) * dm
            phi = ve_c * (math.log(m0 / m) if m > m_mid else math.log(m / dry_kg))
            dtau = dm / mdot
            x += C * math.sinh(phi) * dtau
            t += math.cosh(phi) * dtau
        return x, t, math.tanh(ve_c * math.log(m0 / m_mid))

    lo, hi = 1.0, 1e12
    for _ in range(100):
        mid = math.sqrt(lo * hi)
        if fly(mid)[0] < dist_ly * LY:
            lo = mid
        else:
            hi = mid
    x, t, peak = fly(hi)
    return t / YR, hi, peak


def score(r, dist_ly=10.0, payload_t=850.0):
    """(years, dry kt, antimatter kt, peak v/c) for a nozzle report."""
    dry = dry_t(r, payload_t)
    yrs, fuel, peak = trip(r["ve_eff"], dry * 1e3, dist_ly, p_ann(r))
    return yrs, dry / 1e3, fuel / 2e6, peak


def in_system(r, dist_m, mass_kg):
    """(days, delta-v m/s, antimatter kg) for a burn-the-whole-way trip,
    ignoring the propellant's mass change."""
    thrust = r["ve_eff"] * p_ann(r) / C
    a = thrust / mass_kg
    t = 2 * math.sqrt(dist_m / a)
    return t / 86400, a * t, p_ann(r) / C ** 2 * t / 2


def scaled_spec(gamma_tw, b0):
    """The default nozzle with its geometry scaled to gamma_tw's tile radius."""
    s = math.sqrt(gamma_tw / 180.0)
    return {"gamma_tw": gamma_tw, "b0": b0, "rings": [{"r": 535 * s, "z": 800 * s}], "com": [0, 0, 800 * s]}


if __name__ == "__main__":
    n = int(sys.argv[1]) if len(sys.argv) > 1 else 200000
    r = nozzle.evaluate({"run": {"n_ann": n}})
    yrs, dry, am, peak = score(r)
    thrust = r["ve_eff"] * p_ann(r) / C
    print(nozzle.line("design", r))
    print(f"engine {engine_t(r):.0f} t (magnets {r['magnet_t']:.0f} t, shadow {r['shadow_gw']:.0f} GW), "
          f"plates and stops {plates_t(r):.0f} t, dry {dry*1e3:.0f} t, thrust {thrust/1e3:.0f} kN, "
          f"{thrust/(engine_t(r)*1e3)*1e3:.0f} mN/kg engine")

    print("\npayload    10 ly     30 ly")
    for pay in (100, 850, 2000, 5000):
        print(f"{pay:5} t  {score(r, 10, pay)[0]:6.0f} yr {score(r, 30, pay)[0]:6.0f} yr")

    print("\nhop      time    peak   departure  antimatter")
    for d in (4.4, 10, 20, 30):
        y, dry, am, pk = score(r, d)
        print(f"{d:4} ly {y:6.0f} yr {pk:5.2f}c {dry + 2*am:7.1f} kt {am:7.1f} kt "
              f"({am*1e6*2*C**2/4.184e15/1e6:.0f} Tt)")

    full = (score(r, 10)[1] + 2 * score(r, 10)[2]) * 1e6
    print(f"\nin-system, arrived ({dry_t(r):.0f} t) to fueled for 10 ly ({full/1e3:.0f} t):")
    for name, d in (("Earth-Moon", 3.84e8), ("1 AU", 1.496e11), ("5 AU", 7.48e11)):
        lo, hi = in_system(r, d, dry_t(r) * 1e3), in_system(r, d, full)
        print(f"{name:11} {lo[0]:5.1f}-{hi[0]:5.1f} days  {lo[1]/1e3:4.0f}-{hi[1]/1e3:4.0f} km/s  "
              f"{lo[2]:7.2f}-{hi[2]:7.2f} kg antimatter")

    print("\ngamma TW  ring R  best b0  magnets  shadow  engine   thrust   mN/kg  10 ly")
    for g in (30, 100, 180, 300, 500):
        best = None
        for b0 in (0.0035, 0.005, 0.0065):
            rr = nozzle.evaluate(dict(scaled_spec(g, b0 * math.sqrt(180 / g)), run={"n_ann": 10000}))
            y = score(rr)[0]
            if best is None or y < best[0]:
                best = (y, rr)
        y, rr = best
        th = rr["ve_eff"] * p_ann(rr) / C
        print(f"{g:8} {rr['spec']['rings'][0]['r']:6.0f} {rr['spec']['b0']*1e3:6.1f} mT {rr['magnet_t']:6.0f} t "
              f"{rr['shadow_gw']:4.0f} GW {engine_t(rr):6.0f} t {th/1e3:6.0f} kN {th/engine_t(rr):6.0f} {y:5.0f} yr")
