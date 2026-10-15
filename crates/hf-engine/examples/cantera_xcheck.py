"""Cross-check hf-engine equilibrium against Cantera, gas and graphite.

    cargo run -q --release -p hf-engine --example xcheck > points.txt
    python cantera_xcheck.py nasa_gas.yaml points.txt

nasa_gas.yaml is Cantera's data/nasa_gas.yaml (NASA seven-coefficient fits);
graphite is Cantera's own graphite.yaml.
"""
import cantera as ct, sys

allsp = ct.Species.list_from_file(sys.argv[1])
gas = ct.Solution(thermo='ideal-gas',
                  species=[s for s in allsp if set(s.composition) <= {'H', 'N', 'C'}])
graphite = ct.Solution('graphite.yaml')


def equilibrium(X, T, p):
    """Gas and graphite at T, p from moles X; returns (kg, J, kg soot)."""
    gas.TPX = T, p, X
    gas.equilibrate('TP')
    mix = ct.Mixture([(gas, sum(X.values())), (graphite, 0.)])
    mix.T, mix.P = T, p
    mix.equilibrate('TP')
    mg = mix.phase_moles(0) * gas.mean_molecular_weight / 1e3
    mc = mix.phase_moles(1) * graphite.mean_molecular_weight / 1e3
    return mg + mc, mg * gas.enthalpy_mass + mc * graphite.enthalpy_mass, mc


def moles(bH, bN, bC):
    X = {'H': bH - 4 * bC + 1e-12}
    if bN > 0:
        X['N2'] = bN / 2
    if bC > 0:
        X['CH4'] = bC
    return X


for line in open(sys.argv[2]):
    f = line.split()
    if f[0] == 'tp':
        T, p, bH, bC, soot = map(float, f[1:])
        m, _, mc = equilibrium(moles(bH, 0., bC), T, p)
        print(f"TP methane {T:.0f} K {p/1e5:.0f} bar: soot rust {soot:.4f} cantera {mc/m:.4f}")
        continue
    name, (x, p, bH, bN, bC, h, T, xH, soot, Te, dh) = f[0], map(float, f[1:])
    X = moles(bH, bN, bC)
    lo, hi = 1000., 8000.
    for _ in range(50):
        mid = (lo + hi) / 2
        m, H, mc = equilibrium(X, mid, p)
        lo, hi = (mid, hi) if H / m < h else (lo, mid)
    m, H, mc = equilibrium(X, lo, p)
    print(f"{name:7} 1:{x:<5} {p/1e6:3.0f} MPa: T rust {T:6.0f} cantera {lo:6.0f} | "
          f"xH {xH:.4f} {gas['H'].X[0]:.4f} | soot {soot:.4f} {mc/m:.4f}")
