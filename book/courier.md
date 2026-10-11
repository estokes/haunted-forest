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

The setting's rule for materials: **if physics allows it, six centuries
of superintelligent engineering has built it.** So the ship assumes carbon
nanotube composites near their ideal strength and **superconductors that
work up to ~1000 K**. Neither is forbidden; both are among the first
things such a civilization would crack.

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

Gammas are neutral (no field steers them) and penetrating (a 200 MeV
shower needs 15–20 radiation lengths of tungsten, 5–7 cm). There are
three places they hurt:

1. **The nozzle's own magnets.** Each coil sits in the shadow of a
   tungsten tile facing the reaction. A tile can shed ~10 MW/m² (radiating
   at ~2,900 K through fins), which sets how close to the reaction
   anything may sit: r ≥ √(G / 4π·10 MW/m²), with G the gamma power. With
   superconductors good to ~1000 K the coils run warm, so the tiles only
   need 15 radiation lengths (~5 cm); the ~5% of the shower that leaks
   through is radiated by the coils themselves at ~900 K.
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
| 0.1 TW | 28 m | 90 t | ~0 | 120 t | 380 N | 3.2 mN/kg |
| 1 TW | 89 m | 280 t | 15 t | 400 t | 3.8 kN | 9.5 mN/kg |
| **10 TW** | **280 m** | **0.9 kt** | **0.47 kt** | **1.8 kt** | **38 kN** | **21 mN/kg** |
| 30 TW | 490 m | 1.6 kt | 2.4 kt | 5.3 kt | 114 kN | 22 mN/kg |
| 100 TW | 890 m | 2.8 kt | 15 kt | 23 kt | 380 kN | 16 mN/kg |

Assumptions: two coils 0.25 m wide; 0.15 T in the reaction zone (pions at
~350 MeV/c bend inside their ~18 m decay length); magnets at 10× the
stored-energy (virial) floor on near-ideal CNT; +30% for structure and
feeds; coil fins for the tile leak.

### Ship size follows

With the engine 40% of a ship fueled for 10 ly, burning the whole
crossing:

| Gamma power | Ship | Accel | 4.4 ly | 10 ly (peak) | Antimatter | Payload |
|---|---|---|---|---|---|---|
| 0.1 TW | 300 t | 0.13 mG | ~340 yr | ~520 yr (0.04c) | 26 t | ~100 t |
| 1 TW | 1.0 kt | 0.39 mG | ~190 yr | ~290 yr (0.07c) | 140 t | ~210 t |
| **10 TW** | **4.5 kt** | **0.86 mG** | **~120 yr** | **~190 yr (0.11c)** | **~920 t** | **~430 t** |
| 30 TW | 13 kt | 0.88 mG | ~120 yr | ~185 yr (0.11c) | 2.7 kt | ~1.2 kt |

Above ~10 TW the crossing stops getting faster; the ship just gets bigger.
Small couriers are possible but slow: a 300 t ship takes five centuries to
go 10 ly. Payload is nearly free at the design point: cutting it from
850 t to 100 t saves only ~10% of the crossing time, because the engine
and propellant are most of the ship.

## Design Point

**~10 TW of gammas (~28 TW of annihilation), 850 t of payload.** Dry mass
~3,330 t; ~5,350 t at departure fueled for 10 ly.

```
  ← direction of travel (accelerating)

  [payload | stores | dust tank | W plate]════════ 500 m boom ════════( engine )
   ~10 m wide                                                   annihilation point
                                                                inside a ~560 m ring
                                                                nozzle; tiles face in
```

| Part | Mass | Notes |
|---|---|---|
| Engine | ~1,810 t | two superconducting rings, ~280 m radius, running at ~900 K; ~900 t of 5 cm tungsten tiles shading them; ~470 t of magnet; ~33 t of coil fins; feeds and frame |
| Boom | ~30 t | 500 m, in compression at 38 kN — light |
| Shield plate | ~100 t | 7 cm tungsten, 5 m radius, facing the engine; takes ~250 MW and radiates it at ~2,900 K |
| Power plant | ~140 t | heat engine on the tiles, its radiator fins, and the ~40 t port heater |
| Structure, systems, hab | ~400 t | |
| **Payload** | **~850 t** | |
| Matter dust tank | ~1,010 t of dust | behind the plate: ~22 m of dust, ~1,300 g/cm², the second shield layer and the annihilation partner |
| Antimatter store | ~1,010 t of anti-dust | levitated in a magnetic store behind the tank; fed down the boom at ~0.15 g/s |

Thrust ~38 kN; ~0.7 mG at departure, rising as propellant burns.

## Flight

The engine burns the **whole crossing**: accelerate to the midpoint, flip
the ship, brake the rest. Propellant is loaded for the hop:

| Hop | Time | Peak | Departure | Antimatter |
|---|---|---|---|---|
| α Cen (4.4 ly) | ~130 yr | ~0.067c | ~4.6 kt | ~640 t |
| 10 ly | ~210 yr | ~0.097c | ~5.3 kt | ~1,010 t |
| 20 ly | ~310 yr | ~0.13c | ~6.3 kt | ~1,500 t |
| 30 ly | ~390 yr | ~0.16c | ~7.1 kt | ~1,900 t |

That is ~1.5× the freighter's crossings. Coasting at a lower peak saves
propellant and costs time.

**Flipping is back.** The ship rotates at midcourse. The engine leads
into the medium on the braking half, so its rings carry their own Whipple
strips (~200 t, inside the engine line), and the tungsten plate is good
armor for the other end.

## Heat

Almost all the waste leaves for free: ~10 TW of gammas and the pion and
muon exhaust fly away, and annihilation happens in vacuum with no chamber
walls. Only what the hardware intercepts has to be radiated:

| Source | Heat | Temperature | Rejected by |
|---|---|---|---|
| Engine tiles | ~8.9 GW | ~2,900 K | tile fins, edge-on to the reaction |
| Leak through the tiles into the coils | ~0.44 GW | ~900 K | ~7,000 m² of coil fins, edge-on |
| Shield plate | ~0.25 GW | ~2,900 K | its face and fins |
| Power plant | ~1.2 GW of waste for ~0.5 GW of electricity | ~1,000–1,500 K | 5,000–25,000 m² of fins |
| Port heater (engine cold) | ~70 MW of waste | ~1,000–1,500 K | the same plant fins |
| Hab, avionics, stores | MW | ~300 K | small radiators |

**Radiators must be edge-on to the engine.** The plate's shadow is only as
wide as the plate, 5 m in radius. At 500 m from the engine, any surface
outside it faces ~3.2 MW/m² of gammas. So every radiator, at both ends of
the boom, is a fin in a plane that contains the line to the reaction
point; its faces never see it.

**The dust tank must stay cold.** Metal reverts near 1000 K. The plate
takes the gammas and radiates them at ~2,900 K; the tank behind it is
insulated from it.

## Power

The tile heat engine gives **gigawatts of electricity whenever the engine
burns** — which is the whole crossing. Hotel, sensors, and an FEL all fit.
With 1000 K superconductors nothing aboard needs cryogenic cooling:
nozzle coils, the antimatter store's levitation, and the dust pumps all
run warm.

**In port the same heat engine runs on a second heat source:** a small
**annihilation heater** beside it at the engine end. Anti-dust and dust
meet inside a tungsten core ~0.9 m in radius, thick enough to stop the
gammas (~20 radiation lengths) and the pions (~11 cm). About 84% of the
energy stays as heat; the rest leaves as neutrinos from the stopped pions
and muons. It is the one place on the ship where annihilation is fully
contained, and it can be, because it is small.

| | Value |
|---|---|
| Thermal power | ~100 MW (surface at the same ~10 MW/m² limit) |
| Electricity | ~30 MW |
| Core | ~40 t of tungsten |
| Antimatter | ~20 g a year |

It feeds from the same anti-dust line as the engine and rejects its waste
through the plant's radiators. Pion capture on tungsten makes neutrons and
activates the core over time, which is another reason it lives at the far
end of the boom. The 100 GW reactor sim no longer powers the ship.

## Maneuvering

There is **one drive**. If the ship moves, the main engine moves it.

- **Throttle.** The engine runs at any power from a trickle up to ~28 TW
  of annihilation; thrust follows at ~1.4 N per GW.
- **Attitude.** Steering the nozzle field slightly off axis vectors the
  thrust; with the mass spread along a 500 m boom, a small angle is a
  large torque. The midcourse flip uses the same trick.
- **In-system**, at full thrust (~0.7–0.8 mG), burning the whole way:

| Trip | Time | Δv | Antimatter |
|---|---|---|---|
| Earth–Moon distance | ~5 days | ~3.5 km/s | ~70 kg |
| 1 AU | ~100 days | ~70 km/s | ~1.4 t |
| 5 AU | ~230 days | ~150 km/s | ~3 t |

- **Ports.** A courier never docks. It parks a light-second or more out,
  with its exhaust pointed away from anything that matters, and shuttles
  carry payload in.
- **Combat.** Everything happens at milligees. Fights are slow and
  positional: who saw whom first, where the exhaust points, and how much
  propellant each side can spend.

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

A 10 ly load of ~1,010 t of antimatter is ~1.8×10²³ J, **~43 Tt** —
Chicxulub-class; a 30 ly load is ~80 Tt. One 4 mm anti-grain is ~1 kt. A
courier that loses its store is gone, and so is anything within about a
light-second. Ports keep couriers far out; transfers ride shuttles.

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
any thruster but the main drive; "the ship never flips."

## Open Questions

1. **The reactor sim.** Port power is the annihilation heater; the 100 GW
   D-³He mirror sim no longer powers the ship. Keep it as a station and
   colony system, or retire it.
2. **Nozzle design.** Keane & Zhang use ~10 T compact coils; this brief
   assumes large 0.15 T rings so the coils can sit far from the
   reaction. Needs a real field model.
3. **Tile flux limit** (10 MW/m² assumed). It sets the nozzle size and
   most of the engine mass.
4. **Crew.** An AI captain and sleepers, or a waking human crew in a
   payload of hundreds of tonnes.
5. **Range.** Propellant and tankage for 30 ly hops versus payload.
6. **Thrust vectoring.** How far the nozzle field can steer the exhaust
   off axis, which sets the flip time and the attitude authority.

## Assumptions

| | Value |
|---|---|
| Gamma share of annihilation energy | 0.36 |
| Effective exhaust over total propellant | 0.41c |
| Thrust per annihilation power | 1.4 N/GW |
| Tile flux limit | 10 MW/m² |
| Tile thickness | 5 cm tungsten, 15 radiation lengths (1.0 t/m²); ~5% leaks |
| Superconductor operating limit | ~1000 K; coils run at ~900 K |
| Nozzle field in the reaction zone | 0.15 T |
| Magnet mass | 10× virial floor, CNT at 25 GPa allowable |
| Radiator fins | ~5 kg/m², edge-on to the reaction |
| Payload | 850 t |

Sources: Keane & Zhang, "Beamed Core Antimatter Propulsion: Engine Design
and Optimization", JBIS 2012 (arXiv 1205.2281).
