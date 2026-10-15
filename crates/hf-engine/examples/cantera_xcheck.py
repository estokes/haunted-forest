"""Cross-check hf-engine equilibrium against Cantera.

    cargo run -q --release -p hf-engine --example xcheck > points.txt
    python cantera_xcheck.py nasa_gas.yaml points.txt

nasa_gas.yaml is Cantera's data/nasa_gas.yaml (NASA seven-coefficient fits).
"""
import cantera as ct, sys
allsp = ct.Species.list_from_file(sys.argv[1])
keep = [s for s in allsp if set(s.composition) <= {'H','N'}]
gas = ct.Solution(thermo='ideal-gas', species=keep)
print(len(keep), 'H/N species from nasa_gas.yaml:', ' '.join(s.name for s in keep))
for line in open(sys.argv[2]):
    x,p,bH,bN,h,T,xH,Te,dh = map(float,line.split())
    # composition with the same elements: H atoms + N2
    gas.TPX = T, p, {'H': bH, 'N2': bN/2} if bN>0 else {'H':1}
    gas.equilibrate('TP')
    gas.HP = h, p
    gas.equilibrate('HP')
    T_ct, xH_ct = gas.T, gas['H'].X[0]
    s0, h0 = gas.s, gas.h
    gas.SP = s0, 1.5e6
    gas.equilibrate('SP')
    print(f"1:{x:<5} {p/1e6:4.0f} MPa  T rust {T:6.0f} cantera {T_ct:6.0f} | xH {xH:.4f} {xH_ct:.4f} | expanded T {Te:6.0f} {gas.T:6.0f} | dh {dh/1e6:6.2f} {(h0-gas.h)/1e6:6.2f} MJ/kg")
