# Courier — Design Brief

2026-10-10. **Proposal**, not canon. If accepted it replaces the 1 km
freighter in [Ship Systems](ship-systems.md). Numbers are first-order,
from the sizing in this brief; treat engine masses as ±2×.

## Why

The freighter rested on **catalyzed p-p fusion**, which is probably
impossible. The Sun's core makes ~276 W/m³; the first step of the chain
is a weak-force event that takes a proton billions of years.

This design uses **antimatter**, and puts nothing impossible aboard:

- **Making antimatter** is an engineering problem — stellar-scale
  collectors and very efficient pair production. A superintelligence can
  plausibly solve it. It happens at IS facilities, not on the ship.
- **Metastable metallic hydrogen** (and its antimatter twin) is
  theoretically plausible. It makes both fuels dense, storable, and
  superconducting, so they can be levitated and pumped by magnetic fields.
- **Annihilation, pions, gammas, magnetic nozzles** are textbook physics.

The price: no engine that can carry a 1 Mt freighter survives its own
gamma rays. The ship has to be small.

## The Engine — Beamed Core

A proton meeting an antiproton makes ~5 pions. About **62%** of the energy
goes to charged pions; about **36%** to neutral pions, which become gamma
rays averaging ~200 MeV, in every direction.

A magnetic nozzle turns the charged pions into an exhaust. Keane & Zhang
(2012, Geant4 simulation) get the pions out at ~0.69c. Counted over all
the propellant burned — matter and antimatter, including the energy lost
to gammas — that is an **effective exhaust of ~0.41c** and **~1.4 N of
thrust per GW of annihilation**. Antimatter is half the propellant.

### The gamma problem

Gammas are neutral (no field steers them) and penetrating (stopping
them takes 15–20 radiation lengths: ~7 cm of tungsten). There are three
places they hurt:

1. **The nozzle's own magnets.** Superconducting coils can take almost
   no heat, so each one hides in the shadow of a tungsten tile facing the
   reaction. A tile can shed ~10 MW/m² (radiating at ~2,900 K through
   fins), which sets how close to the reaction anything may sit:
   r ≥ √(G / 4π·10 MW/m²), with G the gamma power.
2. **The rest of the ship.** It sits at the far end of a boom, behind a
   shield plate. The plate's heat scales with the ship's angular size seen
   from the engine.
3. **Using the energy.** We do not. Absorbed gamma energy becomes heat at
   the absorber's temperature, and ~3,000 K cannot make a 0.4c exhaust.
   The gamma third leaves the ship. That is the honest cost.

### Why the engine is big

Thrust grows with power (∝ G). Tile area grows with the nozzle's radius
(∝ √G). Magnet mass grows with the field volume (∝ G^1.5). So small
engines are mass-inefficient, very large ones are magnet-bound, and the
sweet spot is in between:

| Gamma power | Nozzle radius | Tiles | Magnets | Engine | Thrust | Thrust/engine mass |
|---|---|---|---|---|---|---|
| 0.1 TW | 28 m | 120 t | ~0 | 160 t | 380 N | 2.4 mN/kg |
| 1 TW | 89 m | 380 t | 15 t | 510 t | 3.8 kN | 7.4 mN/kg |
| **10 TW** | **280 m** | **1.2 kt** | **0.47 kt** | **2.2 kt** | **38 kN** | **17.5 mN/kg** |
| 30 TW | 490 m | 2.1 kt | 2.5 kt | 5.9 kt | 114 kN | 19 mN/kg |
| 100 TW | 890 m | 3.8 kt | 15 kt | 24 kt | 380 kN | 16 mN/kg |

Assumptions: two coils, 0.25 m wide with cryostat; 0.15 T in the reaction
zone (pions at ~350 MeV/c bend inside their ~18 m decay length); magnets
at 10× the stored-energy (virial) floor on near-ideal CNT; +30% for
structure and feeds.

### Ship size follows

With the engine ~40% of the ship, flying to the midpoint and braking the
rest of the way (burn the whole crossing):

| Gamma power | Ship | Accel | 4.4 ly | 10 ly (peak) | Antimatter | Payload |
|---|---|---|---|---|---|---|
| 0.1 TW | 390 t | 0.10 mG | ~410 yr | ~630 yr (0.03c) | 28 t | ~140 t |
| 1 TW | 1.3 kt | 0.30 mG | ~240 yr | ~360 yr (0.06c) | 150 t | ~330 t |
| **10 TW** | **5.4 kt** | **0.71 mG** | **~150 yr** | **~230 yr (0.086c)** | **~930 t** | **~850 t** |
| 30 TW | 15 kt | 0.80 mG | ~150 yr | ~220 yr (0.09c) | 2.6 kt | ~2.1 kt |

Above ~10 TW the crossing barely gets faster; the ship just gets bigger.
Small couriers are possible but slow: a 400 t ship takes six centuries to
go 10 ly.

## Design Point

**~10 TW of gammas (~28 TW of annihilation).** Departure mass ~5,400 t.

```
  ← direction of travel (accelerating)

  [payload | stores | dust tank | W plate]════════ 500 m boom ════════( engine )
   ~10 m wide                                                   annihilation point
                                                                inside a ~560 m ring
                                                                nozzle; tiles face in
```

| Part | Mass | Notes |
|---|---|---|
| Engine | ~2,200 t | two superconducting rings, ~280 m radius; ~1,200 t of tungsten tiles shading them, shedding ~9 GW; ~470 t of magnet; feeds and frame |
| Boom | ~30 t | 500 m, in compression at 38 kN — light |
| Shield plate | ~100 t | 7 cm tungsten, 5 m radius, facing the engine; takes ~250 MW and radiates it at ~2,900 K |
| Matter dust tank | ~930 t of dust | behind the plate: ~20 m of dust, ~1,200 g/cm², the second shield layer and the annihilation partner |
| Antimatter store | ~930 t of anti-dust | levitated in a magnetic store behind the tank; fed down the boom at ~0.15 g/s |
| Structure, power, systems, hab | ~440 t | |
| **Payload** | **~850 t** | |

Thrust ~38 kN; ~0.7 mG at departure, rising as propellant burns.

## Flight

The engine burns the **whole crossing**: accelerate to the midpoint, flip
the ship, brake the rest. At 0.7 mG:

| Hop | Time | Peak |
|---|---|---|
| α Cen (4.4 ly) | ~150 yr | ~0.057c |
| 10 ly | ~230 yr | ~0.086c |

That is ~1.6× the freighter's crossings. Coasting at a lower peak saves
propellant and costs time. Longer hops need more propellant: 30 ly at full
burn peaks near 0.15c and takes the propellant from about a third of the
ship to about half, eating payload.

**Flipping is back.** The ship rotates at midcourse. The engine leads
into the medium on the braking half, so its rings carry their own Whipple
strips (~200 t, inside the engine line), and the tungsten plate is good
armor for the other end.

## Power

The tiles shed ~9 GW at ~2,900 K. A heat engine running between them and
~1,000 K radiators gives **gigawatts of electricity whenever the engine
burns** — which is the whole crossing. Hotel, sensors, and an FEL all fit.

In port, with the engine cold, the ship needs another source. **Open:**
the existing 100 GW D-³He reactor sim (~8 kt) does not fit a 5 kt ship.
Options: a much smaller plant, stored energy, or keep the mirror reactor
as a station and colony system.

## Maneuvering and Combat

- **Main drive:** ~0.7 mG. Not a combat drive.
- **Thruster:** a dust rocket with no antimatter. Metal that reverts in a
  chamber releases 216 MJ/kg by itself — a monopropellant good for
  ~10–15 km/s of exhaust, capped by chamber temperature. At ~10 GW it
  pushes ~1.5 MN, ~0.03 G on 5 kt. Each 500 t of dust is ~1 km/s of Δv.
- Fights are slow, positional, and decided by Δv bookkeeping.

## Shielding and Sleepers

- **Engine gammas:** the plate and the dust tank behind it shadow the
  whole payload stack.
- **Cosmic rays:** isotropic, so a sleeper needs shielding all round —
  ~700 g/cm², ~45 t of water or dust per pod. A courier carries **a
  handful** of fully shielded sleepers, or tens in lighter boxes that
  leave the revival bill high.
- Most of what crosses is data, genomes, wombs, AI persons, and unique
  objects.

## The Bomb

~930 t of antimatter is ~1.7×10²³ J, **~40 Tt** — Chicxulub-class. One
4 mm anti-grain is ~1 kt. A courier that loses its store is gone, and so
is anything within about a light-second. Ports keep couriers far out;
transfers ride shuttles.

## What It Changes

- **No bulk interstellar trade.** Systems are self-sufficient; what crosses
  is information, seeds, minds, and the rare object.
- **Colonies are founded from genomes and wombs**; humans in the flesh
  cross rarely.
- **IS** is the antimatter monopoly, with stellar collectors at its
  facilities, and the only maker of the metal.
- **Crossings are centuries.** History, trade, and the IS service
  interval need re-timing.

## What It Retires

The 1 km hull, the hold, rack, and cargo manager; the guyed pylons; the
fuel bands and bladder silos; the vault as designed; catalyzed p-p and the
IS box as a fusion torch; the 100 GW plant as ship power (see Power);
"the ship never flips."

## Open Questions

1. **Port power** and the fate of the reactor sim.
2. **Nozzle design.** Keane & Zhang use ~10 T compact coils; this brief
   assumes large 0.15 T rings so the coils can sit far from the
   reaction. Needs a real field model.
3. **Tile flux limit** (10 MW/m² assumed). It sets the nozzle size and
   most of the engine mass.
4. **Crew.** An AI captain and sleepers, or a waking human crew in a
   payload of hundreds of tonnes.
5. **Range.** Propellant fraction for 30 ly hops versus payload.

## Assumptions

| | Value |
|---|---|
| Gamma share of annihilation energy | 0.36 |
| Effective exhaust over total propellant | 0.41c |
| Thrust per annihilation power | 1.4 N/GW |
| Tile flux limit | 10 MW/m² |
| Tile thickness | 7 cm tungsten (1.35 t/m²) |
| Nozzle field in the reaction zone | 0.15 T |
| Magnet mass | 10× virial floor, CNT at 25 GPa allowable |
| Engine share of ship | 40% |

Sources: Keane & Zhang, "Beamed Core Antimatter Propulsion: Engine Design
and Optimization", JBIS 2012 (arXiv 1205.2281).
