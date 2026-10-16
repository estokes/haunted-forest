# The fighter's FEL in the atmosphere: delivered power vs range
# (book/small-craft.md, "FEL Ranges").  Run it to print the tables.
#
# The beam is 2 MW from the two 0.5 m directors (8 MW electrical, ~25%
# wall plug), diffraction limited (first dark at 1.22 * lambda * R / D),
# and tunable near-IR -> UV (0.28 - 1.5 um).  The targeting computer
# picks the wavelength that maximises intensity at the aim point, which
# is T/lambda^2, not T: when the spot is smaller than the target,
# maximising transmission alone spreads the power over a bigger spot.
#
# The atmosphere is US76 density with Rayleigh scattering (0.013 /km at
# 0.55 um, lambda^-4), aerosols in visibility classes (clear ~20 km,
# hazy ~5 km, dusty ~1.5 km; exponential decay with altitude,
# lambda^-alpha), H2O/CO2 bands at 0.94/1.13/1.38/1.43/1.55 um (50% RH
# base, RH multiplier, ~1.7 km scale height), the O2 A-band, and an
# ozone profile peaking at 22 km.  Paths are straight lines; tau = J *
# path length, J being the path-shape integral, so the range scan is
# analytic.  Ground shots land as ellipses stretched by 1/sin(elevation).
#
# fel_extras.py covers the low-altitude and above-atmosphere shots,
# hazy air-to-air, the vacuum envelope, and the spot-vs-target crossover.

import math

P_BEAM = 2.0e6            # W, beam power
D = 0.5                   # m, director aperture
FLOOR = 1.22              # Airy: first-dark radius = 1.22 * lam * R / D
LAM_MIN, LAM_MAX, LAM_STEP = 0.28, 1.50, 0.01   # um
RH = 1.0                  # 1.0 = 50% RH at the surface

# ---------------- standard atmosphere (US76, 0-20 km) ----------------
def rho_ratio(z):
    """air density / sea-level density at altitude z (m)"""
    z = max(z, 0.0)
    if z <= 11000.0:
        p = (1.0 - 2.25577e-5*z)**5.2561
        T = 1.0 - 6.5e-3*z/288.15
    else:
        p = (1.0 - 2.25577e-5*11000.0)**5.2561 * math.exp(-1.5777e-4*(z-11000.0))
        T = 216.65/288.15
    return p/T

B_RAY_550 = 0.013         # km^-1, Rayleigh, clean dry air, sea level, 0.55 um

def o3_beta(lam, z):
    """ozone extinction km^-1; profile peaked ~22 km, sigma from 2 anchors"""
    n = 6.4e20 * math.exp(-((z-22000.0)/7000.0)**2)      # m^-3
    sig = 1.5e-21*math.exp(-((lam-0.275)/0.008)**2)      # Hartley edge
    sig += 1.5e-27*math.exp(-((lam-0.60)/0.15)**2)       # Chappuis
    return n*sig/1000.0                                  # m^-1 -> km^-1

# absorption bands: (center um, sigma um, ground beta km^-1 @50%RH, profile)
BANDS = [
    (0.940, 0.025, 0.030, 'h2o'),
    (1.130, 0.030, 0.020, 'h2o'),
    (1.380, 0.045, 0.150, 'h2o'),
    (1.430, 0.030, 0.150, 'h2o'),
    (1.550, 0.060, 0.060, 'h2o'),
    (0.760, 0.008, 0.0020, 'o2'),
]

CONDS = {
    'clear': dict(aer=0.18, alpha=1.6),   # ~20 km visibility at the surface
    'hazy':  dict(aer=0.77, alpha=1.3),   # ~5 km visibility
    'dusty': dict(aer=2.58, alpha=1.0),   # ~1.5 km visibility
}

def beta(lam, z, cond):
    """total extinction km^-1 at wavelength lam (um), altitude z (m)"""
    rho = rho_ratio(z)
    b = B_RAY_550*(0.55/lam)**4*rho
    b += cond['aer']*math.exp(-z/1500.0)*(0.55/lam)**cond['alpha']
    for c, w, b0, kind in BANDS:
        f = RH*math.exp(-z/1700.0) if kind == 'h2o' else rho
        b += b0*f*math.exp(-0.5*((lam-c)/w)**2)
    b += o3_beta(lam, z)
    return b

# ---------------- paths ----------------
def path_J(lam, case, H_km, cond, n=300):
    """J = integral of beta over the path shape (dimensionless u).
    tau(R) = J * S, S = path length (km).  Independent of range."""
    if case == 'a2a':
        return beta(lam, H_km*1000.0, cond)
    H = H_km*1000.0
    s = 0.0
    for i in range(n):
        u = (i + 0.5)/n
        s += beta(lam, H*(1.0-u), cond)
    return s/n

def path_len(case, R_km, H_km):
    return R_km if case == 'a2a' else math.hypot(R_km, H_km)

def trans(lam, case, R_km, H_km, cond, J, cloud_tau=0.0):
    return math.exp(-(J*path_len(case, R_km, H_km) + cloud_tau))

def spot(case, R_km, lam, H_km):
    """spot area (m^2), first-dark diameter (m), ground elongation factor"""
    r1 = FLOOR*lam*1e-6*R_km*1000.0/D
    A = math.pi*r1*r1
    elong = 1.0
    if case == 'ground':
        elong = path_len('ground', R_km, H_km)/H_km   # 1/sin(elev)
        A *= elong
    return A, 2.0*r1, elong

def delivered(lam, case, R_km, H_km, cond, J, A_t, cloud_tau=0.0):
    T = trans(lam, case, R_km, H_km, cond, J, cloud_tau)
    A, d, el = spot(case, R_km, lam, H_km)
    P_in = P_BEAM*T
    I = P_in/A
    P_on = min(P_in, I*A_t)
    return T, I, P_on, A, d, el

_JCACHE = {}
def Js(case, H_km, cond):
    key = (case, H_km, cond)
    if key not in _JCACHE:
        Js_ = {}
        lam = LAM_MIN
        while lam <= LAM_MAX + 1e-9:
            Js_[round(lam, 4)] = path_J(lam, case, H_km, CONDS[cond])
            lam += LAM_STEP
        _JCACHE[key] = Js_
    return _JCACHE[key]

def optimize(case, R_km, H_km, cond, A_t, cloud_tau=0.0):
    """computer picks lambda maximising intensity at target (damage rate)"""
    Jt = Js(case, H_km, cond)
    best = None
    lam = LAM_MIN
    while lam <= LAM_MAX + 1e-9:
        T, I, P_on, A, d, el = delivered(lam, case, R_km, H_km, CONDS[cond],
                                        Jt[round(lam, 4)], A_t, cloud_tau)
        if best is None or I > best[3]:
            best = (lam, T, P_on, I, d, el)
        lam += LAM_STEP
    return best

# ---------------- tables ----------------
def main():
    print("="*78)
    print(f"FEL: P = {P_BEAM/1e6:.0f} MW, D = {D} m, tunable {LAM_MIN}-{LAM_MAX} um, "
          f"RH x{RH} (50% RH base)")
    print("="*78)

    print("\n--- A. Beam divergence, diffraction limited (first dark) ---")
    print("lambda(um)  theta=1.22l/D (urad)  l/D (urad)  spot dia @100km (m)  @500km (m)")
    for lam in (0.30, 0.40, 0.50, 0.60, 0.70, 0.85, 1.00, 1.10, 1.30, 1.40, 1.50):
        th = FLOOR*lam/D          # lam in um, D in m -> umrad
        ld = lam/D                # umrad
        d100 = 2*FLOOR*lam*1e-6*1e5/D
        d500 = 2*FLOOR*lam*1e-6*5e5/D
        print(f"{lam:5.2f}      {th:6.2f}            {ld:6.2f}      {d100:8.3f}      {d500:8.2f}")

    print("\n--- B. Transmission of a 100 km path, clear air, 50% RH ---")
    for case, H, label in (('a2a', 10.0, 'air-to-air, both at 10 km (horizontal)'),
                           ('ground', 10.0, 'air-to-ground, 100 km out from 10 km')):
        print(f"\n{label}:")
        print("lambda(um)   T(%)")
        for lam in (0.30, 0.40, 0.50, 0.60, 0.76, 1.00, 1.06, 1.38, 1.43, 1.55):
            J = path_J(lam, case, H, CONDS['clear'])
            T = trans(lam, case, 100.0, H, CONDS['clear'], J)*100
            print(f"{lam:5.2f}      {T:8.2f}")

    def envelope(case, H_km, A_t, cond='clear', cloud_tau=0.0,
                 Rs=(10, 25, 50, 75, 100, 150, 200, 300, 500, 750, 1000)):
        print(f"\n{case}, H={H_km:.0f} km, target {A_t:g} m2, {cond}:")
        print("R(km)   best l(um)   T(%)   spot dia(m)  I(MW/m2)   P_on_target(MW)")
        for R in Rs:
            lam, T, P_on, I, d, el = optimize(case, R, H_km, cond, A_t, cloud_tau)
            print(f"{R:4.0f}     {lam:5.2f}     {T*100:7.2f}   {d:8.3f}   {I/1e6:8.3f}   {P_on/1e6:8.3f}")

    A_FIGHTER, A_GROUND, A_SENSOR = 15.0, 5.0, 0.25
    envelope('a2a', 10.0, A_FIGHTER, Rs=(25, 50, 75, 100, 150, 200, 300, 500, 750, 1000))
    envelope('ground', 10.0, A_GROUND, Rs=(10, 25, 50, 75, 100, 150, 200, 300, 500))

    print("\n--- C. Ground attack, 100 km and 25 km, weather sensitivity ---")
    for R in (100.0, 25.0):
        print(f"\nR = {R:.0f} km (target 5 m2):")
        print("condition   best l(um)   T(%)   I(MW/m2)   P_on_target(MW)")
        for cond in ('clear', 'hazy', 'dusty'):
            lam, T, P_on, I, d, el = optimize('ground', R, 10.0, cond, A_GROUND)
            print(f"{cond:6s}       {lam:5.2f}     {T*100:7.2f}   {I/1e6:8.3f}   {P_on/1e6:8.3f}")
    print("\ncloud on path (1 km of cloud, tau=5 or 10), ground 100 km:")
    print("cloud tau   best l(um)   T(%)   I(MW/m2)")
    for ct in (0.0, 5.0, 10.0):
        lam, T, P_on, I, d, el = optimize('ground', 100.0, 10.0, 'clear', A_GROUND, ct)
        print(f"{ct:8.0f}     {lam:5.2f}     {T*100:7.2f}   {I/1e6:8.3f}")

    print("\n--- D. Mission ranges, clear air, 50% RH (optimised wavelength) ---")
    def range_for(thr, case, H_km, A_t, kind='I', Rmax=1500.0, step=1.0):
        # largest R where envelope meets threshold; None if never met,
        # -Rmax if it still holds at the scan cap
        hits = []
        R = step
        while R <= Rmax:
            lam, T, P_on, I, d, el = optimize(case, R, H_km, 'clear', A_t)
            val = I if kind == 'I' else P_on
            if val >= thr:
                hits.append(R)
            R += step
        if not hits:
            return None
        return -Rmax if hits[-1] >= Rmax else hits[-1]

    def fmt_range(r):
        return ">%.0f" % -r if r is not None and r < 0 else ("%d" % r if r else "--")

    print("threshold (optimised lambda, clear):")
    print("  mission effect              A2A @10km     ground @10km")
    rows = [
        ("blinding (>=1 kW on 0.25 m2 sensor)", 'P', A_SENSOR, 1e3, 4000.0),
        ("optics destroyed (>=10 kW on sensor)", 'P', A_SENSOR, 1e4, 2000.0),
        ("quick kill, 5 s (I >= 20 MW/m2)",     'I', None, 20e6, 500.0),
        ("kill in 30 s (I >= 3.4 MW/m2)",       'I', None, 3.4e6, 500.0),
        ("1 cm armour in 5 s (I >= 200 MW/m2)", 'I', None, 200e6, 500.0),
    ]
    for label, kind, A_t, thr, Rmax in rows:
        A = A_t if A_t else (A_FIGHTER if kind == 'I' else A_SENSOR)
        ra = range_for(thr, 'a2a', 10.0, A, kind, Rmax)
        rg = range_for(thr, 'ground', 10.0, A, kind, Rmax)
        print(f"  {label:36s}  {fmt_range(ra):>10s} km   {fmt_range(rg):>8s} km")

    print("\n  (same, dusty air, ground only):")
    for label, kind, A_t, thr, _Rmax in rows[:3]:
        A = A_t if A_t else A_GROUND
        hits = []
        R = 1.0
        while R <= 300.0:
            lam, T, P_on, I, d, el = optimize('ground', R, 10.0, 'dusty', A)
            val = I if kind == 'I' else P_on
            if val >= thr:
                hits.append(R)
            R += 1.0
        print(f"  {label:36s}  {'--':>8s}   {(hits[-1] if hits else 0):8.0f} km")

    print("\n--- E. What the computer actually picks, margins ---")
    for case, H, A_t, Rs in (('a2a', 10.0, A_FIGHTER, (100.0, 300.0)),
                             ('ground', 10.0, A_GROUND, (50.0, 100.0))):
        print(f"\n{case}, top-3 wavelengths:")
        for R in Rs:
            res = []
            Jt = Js(case, H, 'clear')
            lam = LAM_MIN
            while lam <= LAM_MAX + 1e-9:
                T, I, P_on, A, d, el = delivered(lam, case, R, H, CONDS['clear'],
                                                Jt[round(lam, 4)], A_t)
                res.append((I, lam, T))
                lam += LAM_STEP
            res.sort(reverse=True)
            top = ",  ".join(f"{l:.2f} um: {I/1e6:.2f} MW/m2 (T {T*100:.1f}%)"
                             for I, l, T in res[:3])
            print(f"  R={R:4.0f} km  {top}")

    print("\n--- F. Vacuum reference (no atmosphere), I at target ---")
    print("R(km)    I @1.06um (MW/m2)   I @0.5um (MW/m2)")
    for R in (100.0, 300.0, 1000.0):
        A106 = math.pi*(FLOOR*1.06e-6*R*1000.0/D)**2
        A05 = math.pi*(FLOOR*0.5e-6*R*1000.0/D)**2
        print(f"{R:5.0f}    {P_BEAM/A106/1e6:14.2f}   {P_BEAM/A05/1e6:14.2f}")

if __name__ == '__main__':
    main()
