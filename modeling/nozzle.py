# Beamed-core magnetic nozzle: coaxial current loops around an annihilation
# point at the origin. Charged pions from p-pbar annihilation are tracked
# through the field with their decays (pi -> mu nu, mu -> e nu nu). The ship
# is on +z; exhaust should leave along -z. Neutral pions (gammas) carry no
# net momentum and are not tracked.
#
# Scores: thrust per annihilation power (as an effective exhaust over all
# propellant, v/c), charged power reaching the coils or the ship's plate,
# and the loops' stored energy, hence magnet mass.

import math
import sys

import numpy as np

C = 2.998e8
MU0 = 4e-7 * math.pi
MEV_C = 1e6 * 1.602176634e-19 / C          # kg m/s per MeV/c
Q = 1.602176634e-19
M_PI, M_MU, M_E = 139.57, 105.66, 0.511    # MeV
TAU_PI, TAU_MU = 26.03e-9, 2.197e-6
ANN_MEV = 1876.5
P_STAR_PI = 29.79                          # muon momentum in the pion frame, MeV/c
RHO_CNT, SIG_ALLOW = 1400.0, 25e9


def ellipke(m):
    """Complete elliptic integrals K(m), E(m) by the arithmetic-geometric mean."""
    a = np.ones_like(m)
    b = np.sqrt(1.0 - m)
    c2 = m.copy()
    s = 0.5 * c2
    p2 = 0.5
    for _ in range(12):
        c = 0.5 * (a - b)
        a, b = 0.5 * (a + b), np.sqrt(a * b)
        p2 *= 2.0
        s += p2 * c * c
    K = math.pi / (2.0 * a)
    return K, K * (1.0 - s)


def loop_field(R, z0, I, rho, z):
    """(B_rho, B_z) of a circular loop of radius R at height z0 carrying I."""
    zeta = z - z0
    s2 = R * R + rho * rho + zeta * zeta
    alpha2 = np.maximum(s2 - 2 * R * rho, 1e-12)
    beta2 = s2 + 2 * R * rho
    beta = np.sqrt(beta2)
    m = 1.0 - alpha2 / beta2
    K, E = ellipke(np.clip(m, 0.0, 1.0 - 1e-15))
    c = MU0 * I / math.pi
    bz = c / (2 * alpha2 * beta) * ((R * R - rho * rho - zeta * zeta) * E + alpha2 * K)
    safe = np.where(rho > 1e-9, rho, 1.0)
    br = np.where(rho > 1e-9,
                  c * zeta / (2 * alpha2 * beta * safe) * ((R * R + rho * rho + zeta * zeta) * E - alpha2 * K),
                  0.0)
    return br, bz


def field(loops, x, sweeper=None):
    """Loop fields plus an optional sweeper: a uniform transverse field b_x
    in the slab z1 < z < z2 within r_max of the axis, between the mirror and
    the ship, that bends forward leakage off the axis."""
    rho = np.hypot(x[:, 0], x[:, 1])
    br = np.zeros(len(x)); bz = np.zeros(len(x))
    for R, z0, I in loops:
        a, b = loop_field(R, z0, I, rho, x[:, 2])
        br += a; bz += b
    safe = np.where(rho > 1e-9, rho, 1.0)
    cx = np.where(rho > 1e-9, x[:, 0] / safe, 0.0)
    cy = np.where(rho > 1e-9, x[:, 1] / safe, 0.0)
    b = np.stack([br * cx, br * cy, bz], axis=1)
    if sweeper is not None:
        z1, z2, r_max, bx = sweeper
        inside = (x[:, 2] > z1) & (x[:, 2] < z2) & (rho < r_max)
        b[inside, 0] += bx
    return b


def stored_energy(loops, wire_r=0.1):
    """Self and mutual inductance of coaxial loops, W = 1/2 sum M_ij I_i I_j."""
    W = 0.0
    for i, (Ri, zi, Ii) in enumerate(loops):
        for j, (Rj, zj, Ij) in enumerate(loops):
            if i == j:
                L = MU0 * Ri * (math.log(8 * Ri / wire_r) - 2)
            else:
                k2 = 4 * Ri * Rj / ((Ri + Rj) ** 2 + (zi - zj) ** 2)
                K, E = ellipke(np.array([k2]))
                k = math.sqrt(k2)
                L = MU0 * math.sqrt(Ri * Rj) * ((2 / k - k) * K[0] - 2 / k * E[0])
            W += 0.5 * L * Ii * Ij
    return W


def scale_to_origin_field(loops, b0):
    bz = field(loops, np.zeros((1, 3)))[0, 2]
    return [(R, z0, I * b0 / bz) for R, z0, I in loops]


def isotropic(n, rng):
    u = rng.uniform(-1, 1, n)
    phi = rng.uniform(0, 2 * math.pi, n)
    s = np.sqrt(1 - u * u)
    return np.stack([s * np.cos(phi), s * np.sin(phi), u], axis=1)


def pion_momenta(n, rng, p0=117.0, pmax=928.0):
    """|p| for charged pions from p-pbar at rest: p^2 exp(-p/p0), mean ~350 MeV/c."""
    out = np.empty(0)
    while len(out) < n:
        p = rng.gamma(3.0, p0, 2 * n)
        out = np.concatenate([out, p[p < pmax]])
    return out[:n]


def boost(p4_rest, beta_vec):
    """Lorentz-boost rest-frame 4-momenta (E, px, py, pz) by velocity beta_vec."""
    b2 = np.sum(beta_vec * beta_vec, axis=1)
    g = 1.0 / np.sqrt(1.0 - b2)
    bp = np.sum(beta_vec * p4_rest[:, 1:], axis=1)
    e = g * (p4_rest[:, 0] + bp)
    coef = np.where(b2 > 0, (g - 1) * bp / np.where(b2 > 0, b2, 1.0), 0.0) + g * p4_rest[:, 0]
    p = p4_rest[:, 1:] + coef[:, None] * beta_vec
    return np.column_stack([e, p])


def two_body(parent_e, parent_p, m_d, p_star, rng):
    """Daughter of mass m_d with rest-frame momentum p_star, isotropic, boosted."""
    n = len(parent_e)
    d = isotropic(n, rng) * p_star
    e_d = math.sqrt(p_star ** 2 + m_d ** 2)
    rest = np.column_stack([np.full(n, e_d), d])
    return boost(rest, parent_p / parent_e[:, None])


def michel(n, rng):
    """Electron energy fraction x = 2E/m_mu from the Michel spectrum x^2(3-2x)."""
    out = np.empty(0)
    while len(out) < n:
        x = rng.uniform(0, 1, 3 * n)
        keep = rng.uniform(0, 1, 3 * n) < x * x * (3 - 2 * x)
        out = np.concatenate([out, x[keep]])
    return out[:n]


def simulate(loops, n_ann=600, seed=1, ship_z=1000.0, plate_r=8.0, hit_r=2.0,
             r_dom=8000.0, max_steps=60000, sweeper=None, cone=None):
    """cone=c_min samples pion directions only with cos(theta) > c_min toward
    the ship and weights tallies by that solid-angle fraction: resolves the
    forward leak; thrust from such a run is meaningless."""
    rng = np.random.default_rng(seed)
    n = 3 * n_ann
    p_mag = pion_momenta(n, rng)
    dirs = isotropic(n, rng)
    weight = 1.0
    if cone is not None:
        u = rng.uniform(cone, 1.0, n)
        dirs = np.column_stack([np.sqrt(1 - u * u) * dirs[:, 0] / np.maximum(np.hypot(dirs[:, 0], dirs[:, 1]), 1e-12),
                                np.sqrt(1 - u * u) * dirs[:, 1] / np.maximum(np.hypot(dirs[:, 0], dirs[:, 1]), 1e-12),
                                u])
        weight = (1.0 - cone) / 2.0
    pvec = dirs * p_mag[:, None]                              # MeV/c
    mass = np.full(n, M_PI)
    charge = rng.choice([-1.0, 1.0], n)
    tau = np.full(n, TAU_PI)
    life = -tau * np.log(rng.uniform(1e-12, 1, n))
    x = np.zeros((n, 3))
    alive = np.ones(n, bool)
    tally = dict(escape_pz=0.0, escape_e=0.0, nu_pz=0.0, nu_e=0.0,
                 coil_e=0.0, plate_e=0.0, fwd_e=0.0, trapped_e=0.0)
    plate_by = {M_PI: 0.0, M_MU: 0.0, M_E: 0.0}
    coils = [(R, z0) for R, z0, _ in loops]

    for _ in range(max_steps):
        idx = np.nonzero(alive)[0]
        if len(idx) == 0:
            break
        p = pvec[idx]; m = mass[idx]
        e = np.sqrt(np.sum(p * p, axis=1) + m * m)
        b = field(loops, x[idx], sweeper)
        bmag = np.linalg.norm(b, axis=1) + 1e-30
        omega = Q * bmag / (e * MEV_C / C)                    # rad/s, relativistic
        r = np.linalg.norm(x[idx], axis=1)
        ds = np.clip(0.01 * r + 1.0, 1.0, 40.0)
        dt = np.minimum(0.15 / omega, ds / C)

        # Boris rotation in SI momentum units
        psi = p * MEV_C
        t = (charge[idx] * Q * dt / (2.0 * e * MEV_C / C))[:, None] * b
        pprime = psi + np.cross(psi, t)
        s = 2 * t / (1 + np.sum(t * t, axis=1))[:, None]
        psi = psi + np.cross(pprime, s)
        p = psi / MEV_C
        v = p / e[:, None] * C
        x[idx] += v * dt[:, None]
        pvec[idx] = p
        life[idx] -= dt * m / e

        xi = x[idx]
        rho = np.hypot(xi[:, 0], xi[:, 1])
        done = np.zeros(len(idx), bool)

        hit = np.zeros(len(idx), bool)
        for R, z0 in coils:
            hit |= np.hypot(rho - R, xi[:, 2] - z0) < hit_r
        tally["coil_e"] += np.sum(e[hit]); done |= hit

        plate = (~done) & (xi[:, 2] >= ship_z) & (rho <= plate_r)
        tally["plate_e"] += np.sum(e[plate]); done |= plate
        for mm in plate_by:
            plate_by[mm] += np.sum(e[plate & (m == mm)])

        out = (~done) & (np.linalg.norm(xi, axis=1) > r_dom)
        tally["escape_pz"] += np.sum(p[out, 2]); tally["escape_e"] += np.sum(e[out])
        tally["fwd_e"] += np.sum(e[out & (p[:, 2] > 0)])
        done |= out
        alive[idx[done]] = False

        dec = (~done) & (life[idx] <= 0)
        if np.any(dec):
            d = idx[dec]
            pe = np.sqrt(np.sum(pvec[d] ** 2, axis=1) + mass[d] ** 2)
            pions = mass[d] == M_PI
            muons = mass[d] == M_MU
            if np.any(pions):
                k = d[pions]
                mu = two_body(pe[pions], pvec[k], M_MU, P_STAR_PI, rng)
                nu = np.column_stack([pe[pions], pvec[k]]) - mu
                tally["nu_pz"] += np.sum(nu[:, 3]); tally["nu_e"] += np.sum(nu[:, 0])
                pvec[k] = mu[:, 1:]; mass[k] = M_MU; tau[k] = TAU_MU
                life[k] = -TAU_MU * np.log(rng.uniform(1e-12, 1, len(k)))
            if np.any(muons):
                k = d[muons]
                xe = michel(len(k), rng)
                ee = np.maximum(xe * M_MU / 2, M_E * 1.0001)
                pe_star = np.sqrt(ee ** 2 - M_E ** 2)
                rest = np.column_stack([ee, isotropic(len(k), rng) * pe_star[:, None]])
                el = boost(rest, pvec[k] / pe[muons][:, None])
                nu = np.column_stack([pe[muons], pvec[k]]) - el
                tally["nu_pz"] += np.sum(nu[:, 3]); tally["nu_e"] += np.sum(nu[:, 0])
                pvec[k] = el[:, 1:]; mass[k] = M_E; tau[k] = np.inf; life[k] = np.inf
    left = np.nonzero(alive)[0]
    if len(left):
        tally["trapped_e"] = float(np.sum(np.sqrt(np.sum(pvec[left] ** 2, axis=1) + mass[left] ** 2)))

    e_ann = n_ann * ANN_MEV / weight
    thrust_frac = -(tally["escape_pz"] + tally["nu_pz"]) / e_ann
    return dict(
        ve_eff=thrust_frac,
        n_per_gw=thrust_frac / C * 1e9,
        coil=tally["coil_e"] / e_ann,
        plate=tally["plate_e"] / e_ann,
        forward=tally["fwd_e"] / e_ann,
        neutrino=tally["nu_e"] / e_ann,
        trapped=tally["trapped_e"] / e_ann,
        plate_pi=plate_by[M_PI] / e_ann, plate_mu=plate_by[M_MU] / e_ann, plate_e=plate_by[M_E] / e_ann,
    )


def magnet_t(loops):
    return 10 * stored_energy(loops) * RHO_CNT / SIG_ALLOW / 1e3


def design(R=535.0, b0=0.01, ship_z=1000.0):
    """The brief's nozzle: one mirror ring 0.8 R toward the ship, ~10 mT at
    the reaction point, and a 20 mT sweeper between the ring and the plate."""
    loops = scale_to_origin_field([(R, 0.8 * R, 1.0)], b0)
    sweeper = (0.8 * R + 50.0, ship_z - 10.0, 500.0, 0.02)
    return loops, sweeper


def report(name, loops, sweeper, res):
    print(f"{name:28} v_e {res['ve_eff']:.3f}c  {res['n_per_gw']:.2f} N/GW  "
          f"plate {res['plate']*1e6:7.1f} ppm  fwd {res['forward']*100:4.2f}%  "
          f"coil {res['coil']*100:.3f}%  nu {res['neutrino']*100:4.1f}%  "
          f"magnet {magnet_t(loops):6.0f} t", flush=True)


if __name__ == "__main__":
    n_ann = int(sys.argv[1]) if len(sys.argv) > 1 else 1500
    R = 535.0
    loops, sweeper = design(R)
    report("design", loops, sweeper, simulate(loops, n_ann=n_ann, sweeper=sweeper, seed=3))
    report("design, no sweeper", loops, None, simulate(loops, n_ann=n_ann, seed=3))
    leak = simulate(loops, n_ann=4 * n_ann, sweeper=sweeper, cone=0.6, seed=5)
    print(f"design forward-cone run: plate {leak['plate']*1e6:.1f} ppm", flush=True)
    for a in (0.8, 1.0, 1.5):
        for b0 in (0.005, 0.01, 0.02):
            lp = scale_to_origin_field([(R, a * R, 1.0)], b0)
            sw = (a * R + 50.0, 990.0, 500.0, 0.02)
            report(f"mirror {a:.1f}R, {b0*1e3:.0f} mT", lp, sw,
                   simulate(lp, n_ann=n_ann // 3, sweeper=sw, seed=2))
