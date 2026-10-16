# The FEL's edge cases (book/small-craft.md, "FEL Ranges"): the
# low-altitude attack, the strike from above the atmosphere, hazy
# air-to-air, the vacuum envelope, and where the first-dark spot stops
# covering a target.  Runs on fel_range.py.

import math
import fel_range as F

CONDS = F.CONDS
A_FIGHTER, A_GROUND, A_SENSOR = 15.0, 5.0, 0.25

def env(case, H_km, A_t, cond, Rs, tag):
    print(f"\n{tag} ({case}, H={H_km:.0f} km, {cond}):")
    print("R(km)   best l(um)   T(%)   I(MW/m2)   P_on_target(MW)")
    for R in Rs:
        lam, T, P_on, I, d, el = F.optimize(case, R, H_km, cond, A_t)
        print(f"{R:4.0f}     {lam:5.2f}     {T*100:7.2f}   {I/1e6:8.3f}   {P_on/1e6:8.3f}")

# low-altitude ground attack (H=3 km): most of the path in the boundary layer
env('ground', 3.0, A_GROUND, 'clear', (10, 25, 50, 75, 100), 'LOW ALTITUDE attack')

# strike from above the atmosphere (H=30 km)
env('ground', 30.0, A_GROUND, 'clear', (100, 200, 300, 500, 750, 1000), 'ABOVE-ATMOSPHERE strike')

# hazy air-to-air at 10 km (aerosols thin up high)
env('a2a', 10.0, A_FIGHTER, 'hazy', (100, 200, 300, 500), 'HAZY air-to-air')

# vacuum A2A reference (no atmosphere at all)
print("\nVACUUM A2A reference (no atmosphere), best lambda:")
print("R(km)   best l(um)   spot dia(m)  I(MW/m2)   P_on_target(MW)")
for R in (100, 200, 300, 500, 750, 1000):
    best = None
    lam = F.LAM_MIN
    while lam <= F.LAM_MAX + 1e-9:
        # vacuum: J = 0 -> T = 1
        T, I, P_on, A, d, el = F.delivered(lam, 'a2a', R, 10.0, CONDS['clear'],
                                           0.0, A_FIGHTER)
        if best is None or I > best[3]:
            best = (lam, T, P_on, I, d, el)
        lam += F.LAM_STEP
    print(f"{R:4.0f}     {best[0]:5.2f}     {best[4]:8.3f}   {best[3]/1e6:8.3f}   {best[2]/1e6:8.3f}")

# spot vs fighter crossover: first-dark spot area = 15 m2 (fighter),
# 0.43 m2 (the 2.4 m x 0.18 m missile)
print("\nRange where the first-dark spot equals the target's cross-section:")
print("lambda(um)   fighter 15 m2 (km)   missile 0.43 m2 (km)")
for lam in (0.40, 0.50, 0.70, 1.00, 1.22, 1.50):
    row = []
    for A_t in (15.0, 0.43):
        r1 = math.sqrt(A_t/math.pi)
        row.append(f"{r1/(F.FLOOR*lam*1e-6/F.D)/1000.0:8.0f}")
    print(f"{lam:5.2f}      {row[0]}   {row[1]}")
