from math import pi, sqrt, exp, log
c=2.998e8; mu0=4e-7*pi; e=1.602e-19; mp=1.673e-27; k=1.381e-23
v=0.1*c; g=9.81
eps_c2=0.0071*c**2          # p-p full burn, J/kg
M_ship=4.7e8                # trade-run loaded mass, kg
rho_cond=8000; J_lo,J_hi=1e9,1e10     # HTS engineering current density, A/m^2
sr_lo,sr_hi=3.5e6,3.5e7               # structure specific strength sigma/rho, J/kg
Brho=mp*v/e/sqrt(1-(v/c)**2)          # proton rigidity, T*m
print(f"proton rigidity at 0.1c: {Brho:.3f} T*m, ram KE {0.5*mp*v*v/e/1e6:.2f} MeV")

# bible bow coils: 43 m loop at 20 T -> Stormer radius (region where field steers protons at all)
a,B0=43.0,20.0
r_st=sqrt(a**3*B0/(2*Brho))
for n in (0.1e6,):
    rho=n*mp; Bc=v*sqrt(2*mu0*rho)
    r_mp=a*(B0/Bc)**(1/3)
    print(f"bow coil: pressure standoff {r_mp/1e3:.1f} km, Stormer {r_st/1e3:.2f} km, "
          f"capture at Stormer {pi*r_st**2*rho*v*3.156e7:.2f} kg/yr")

def row(mdot,n,T):
    rho=n*mp; A=mdot/(rho*v); R=sqrt(A/pi)
    di=sqrt(mp/(mu0*n*e*e))
    Bc=v*sqrt(2*mu0*rho); Phi=A*Bc
    sig=sqrt(k*T/mp)/v
    Rt=2.15*sig*R; Bt=Bc*(R/Rt)**2      # 90% mirror transmission
    hoop=lambda J: 4*rho_cond*Phi/(mu0*J)
    I=2*R*Bc/mu0; Lam=log(8*R/0.5)-1; T_h=mu0*I*I/(4*pi)*Lam
    struct=lambda sr: 2*pi*R*T_h/sr
    return dict(R=R/1e3,R_di=R/di,Bc=Bc*1e9,Phi=Phi,Rt=Rt/1e3,Bt=Bt,
                cond_lo=hoop(J_lo)/1e6,cond_hi=hoop(J_hi)/1e6,
                st_lo=struct(sr_lo)/1e6,st_hi=struct(sr_hi)/1e6,I=I/1e6)

print("\nmdot  gas        R_mouth  R/d_i  B_c   flux     throat(90%) B_t,max  I_hoop  hoop conductor kt  hoop structure kt")
print("kg/s             km              nT    Wb       km          T        MA      J=1e9  J=1e10      fiber  CNT")
for gas,n,T in (("bible 0.1",0.1e6,7000),("warm ions",0.07e6,7000),("hot bubble",0.005e6,1e6)):
    for mdot in (0.3,1,3.1,5):
        r=row(mdot,n,T)
        print(f"{mdot:4}  {gas:10} {r['R']:8.0f} {r['R_di']:6.1f} {r['Bc']:5.0f} {r['Phi']:8.2e} {r['Rt']:8.1f}  {r['Bt']:9.3g} {r['I']:7.1f}  {r['cond_lo']:7.0f} {r['cond_hi']:6.0f}  {r['st_lo']:9.0f} {r['st_hi']:6.0f}")

print("\npass-through ramjet at 0.1c (ideal, full burn):")
dv=sqrt(v*v+2*eps_c2)-v
for mdot in (0.3,1,3.1,5):
    F=mdot*dv; P=mdot*eps_c2
    print(f"  {mdot} kg/s: thrust {F/1e6:.0f} MN, fusion {P/1e15:.2f} PW, accel {F/M_ship/g*1000:.1f} mG")
print("option 1, stop-and-convert at 0.1c: ram power per kg/s %.2f PW"%(0.5*v*v/1e15))
for mdot in (0.3,1,3.1):
    Pr=0.5*mdot*v*v
    print(f"  {mdot} kg/s: ram {Pr/1e12:.0f} TW, waste @97% {0.03*Pr/1e9:.0f} GW, @99% {0.01*Pr/1e9:.0f} GW")
print("stopping energy / fusion yield at 0.1c: %.2f"%(0.5*v*v/eps_c2))
for b in (0.3,0.5,0.75):
    gm=1/sqrt(1-b*b); print(f"  at {b}c: (gamma-1)c^2 / eps c^2 = {(gm-1)/0.0071:.0f}; break-even converter eff {1-0.0071/(gm-1):.3f}")

# photoionization of neutrals ahead of a 14,000 km mouth (lower bound: unit probability over 1e8 m)
sig_pi=6.3e-22; L=1e8; R=1.4e7
flux=v/(sig_pi*L); P=flux*13.6*e*pi*R*R
print(f"\nphotoionizing a 14,000 km column 100,000 km ahead: >= {P:.1e} W")

print("\ntime to speed at nozzle power, 470 kt (relativistic KE / P):")
for b in (0.1,0.3,0.5,0.75,0.87):
    gm=1/sqrt(1-b*b); KE=(gm-1)*M_ship*c*c
    print(f"  {b}c gamma {gm:.3f}: 3.2 PW {KE/3.2e15/3.156e7:7.1f} yr, 11 PW {KE/11e15/3.156e7:6.1f} yr; "
          f"power for 1 g at that speed {M_ship*g*b*c/1e15:.0f} PW")
