# Courier mass budget and trip time (book/ship-systems.md, book/courier.md).
# The engine's magnet mass, gamma shadow, plate load, thrust and hits come
# from a hf-nozzle report; the rest of the ship is set here. Run it to print
# the bible's tables.
#
# Trips burn at full power, coast, and brake with equal rapidity each way,
# arriving at rest. All propellant is annihilated (matter and anti-dust 1:1),
# so mass flow is P/c^2 and the effective exhaust is thrust / mass flow. At
# fixed power the thrust is fixed, so carrying more propellant than the
# optimum only slows the ship: each trip loads the propellant that makes it
# fastest, up to the tank. Relativistic in rapidity.

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
BAY_M3 = 57.0                  # crew and sleepers, per pod, in a sphere at the centre of the dust ball
W_T_M3, DUST_T_M3 = 19.3, 0.6
TANK_KG = 27.1e6               # propellant the pods hold, both species: 10 ly out and back on one fill
# The dust ball behind each plate is never burned. Its radius is hf-nozzle's
# pod block: ~9.9 m of dust around the bay. 4.2 m gives ~3 uSv/h with the
# ring's shelter; each further ~1.7 m (100 g/cm2 of hydrogen) cuts the dose
# by e, so ~9.9 m gives ~0.1 uSv/h. It is also the plate's gamma stop.
REST_T = {"power": 100.0, "structure": 500.0}
# Jackets: cooled skins on the pods past the first CLEAN_M, where the
# muon-decay electrons land (hf-nozzle hits_along_ppm).
JACKET_T_PER_M2, CLEAN_M = 0.039, 40.0


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
    """Shield plates and the dust balls behind them, t."""
    pods = r["spec"]["ship"]["pods"]
    big = pods["block"]["r"]
    bay = (3 * BAY_M3 / (4 * math.pi)) ** (1 / 3)
    ball = 4 / 3 * math.pi * (big ** 3 - bay ** 3) * DUST_T_M3
    plate = math.pi * big ** 2 * PLATE_M * W_T_M3
    return pods["n"] * (ball + plate) + PLATE_COOL_T_PER_GW * r["plate_gw"]


def jackets_t(r):
    pods = r["spec"]["ship"]["pods"]
    return JACKET_T_PER_M2 * pods["n"] * 2 * math.pi * pods["r"] * (pods["length"] - CLEAN_M)


def ship_t(r):
    """Everything but the engine and payload, t: plates, dust balls, jackets and the rest."""
    return plates_t(r) + jackets_t(r) + sum(REST_T.values())


def dry_t(r, payload_t=850.0):
    return engine_t(r) + ship_t(r) + payload_t


def burn(ve_c, dry_kg, fuel, p_w, n=2000):
    """(metres, seconds, peak v/c) to burn fuel kg up to speed and back down
    to rest, without coasting."""
    mdot = p_w / C ** 2
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


def trip(ve_c, dry_kg, dist_ly, p_w, tank_kg=math.inf):
    """(years, propellant kg, peak v/c) for the fastest crossing of dist_ly
    on at most tank_kg of propellant."""
    d = dist_ly * LY
    lo, hi = 1.0, 1e12
    for _ in range(100):
        mid = math.sqrt(lo * hi)
        lo, hi = (mid, hi) if burn(ve_c, dry_kg, mid, p_w)[0] < d else (lo, mid)

    def years(fuel):
        x, t, peak = burn(ve_c, dry_kg, fuel, p_w)
        return (t + max(d - x, 0.0) / (C * peak)) / YR

    a, b = math.log(lo * 1e-3), math.log(min(lo, tank_kg))
    g = (math.sqrt(5) - 1) / 2
    for _ in range(60):
        c1, c2 = b - g * (b - a), a + g * (b - a)
        a, b = (a, c2) if years(math.exp(c1)) < years(math.exp(c2)) else (c1, b)
    fuel = math.exp(0.5 * (a + b))
    return years(fuel), fuel, burn(ve_c, dry_kg, fuel, p_w)[2]


def round_trip(ve_c, dry_kg, dist_ly, p_w, tank_kg=TANK_KG, n=40):
    """(years out, years back, cruise v/c) for dist_ly and back on one fill,
    split between the legs for the shortest total."""
    best = None
    for k in range(1, n):
        back = trip(ve_c, dry_kg, dist_ly, p_w, tank_kg * k / n)
        out = trip(ve_c, dry_kg + back[1], dist_ly, p_w, tank_kg - back[1])
        if best is None or out[0] + back[0] < best[0] + best[1]:
            best = (out[0], back[0], out[2])
    return best


def score(r, dist_ly=10.0, payload_t=850.0):
    """(years, dry kt, antimatter kt, peak v/c) for a nozzle report."""
    dry = dry_t(r, payload_t)
    yrs, fuel, peak = trip(r["ve_eff"], dry * 1e3, dist_ly, p_ann(r), TANK_KG)
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
          f"plates and dust balls {plates_t(r):.0f} t, plate gammas {r['plate_gw']:.1f} GW, jackets {jackets_t(r):.0f} t, dry {dry*1e3:.0f} t, thrust {thrust/1e3:.0f} kN, "
          f"{thrust/(engine_t(r)*1e3)*1e3:.0f} mN/kg engine")

    print("\npayload    10 ly     30 ly")
    for pay in (100, 850, 2000, 5000):
        print(f"{pay:5} t  {score(r, 10, pay)[0]:6.0f} yr {score(r, 30, pay)[0]:6.0f} yr")

    print("\nhop      time    peak   departure  antimatter")
    for d in (4.4, 10, 20, 30, 50):
        y, dry, am, pk = score(r, d)
        print(f"{d:4} ly {y:6.0f} yr {pk:5.2f}c {dry + 2*am:7.1f} kt {am:7.1f} kt "
              f"({am*1e6*2*C**2/4.184e15/1e6:.0f} Tt)")

    print("\nout and back on one fill   out     back    cruise")
    for d in (5, 10, 20):
        out, back, v = round_trip(r["ve_eff"], dry_t(r) * 1e3, d, p_ann(r))
        print(f"{d:4} ly                {out:6.0f} yr {back:6.0f} yr {v:5.3f}c")

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
