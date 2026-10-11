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
gamma rays. The ship is tens of kilotonnes, and most of it is engine.

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
   tungsten tile facing the reaction. How much gamma flux a tile can take
   sets how close to the reaction anything may sit:
   r ≥ √(G / 4π·q), with G the gamma power and q the tile limit. The tiles
   are **helium-cooled**: gammas deposit heat through the tile's depth, not
   at its face, so channels a few millimetres apart hold the tungsten
   within tens of kelvin of the coolant, and **q ≈ 50 MW/m²**. The helium
   carries the heat to the ship's power plant. With 1000 K superconductors
   the coils run warm; the tiles need only 15 radiation lengths (~5 cm),
   and the ~5% of each shower that leaks through is radiated by the coils
   at ~900 K.
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
sweet spot is in between. At q = 50 MW/m²:

| Gamma power | Nozzle radius | Tiles | Magnets | Cooling | Engine | Thrust | Thrust/engine mass |
|---|---|---|---|---|---|---|---|
| 1 TW | 40 m | 130 t | ~0 | 85 t | 0.28 kt | 3.8 kN | 14 mN/kg |
| 10 TW | 126 m | 400 t | 42 t | 270 t | 0.92 kt | 38 kN | 41 mN/kg |
| 100 TW | 400 m | 1.3 kt | 1.3 kt | 0.85 kt | 4.5 kt | 380 kN | 85 mN/kg |
| **180 TW** | **535 m** | **1.7 kt** | **3.2 kt** | **1.1 kt** | **7.8 kt** | **684 kN** | **87 mN/kg** |
| 300 TW | 690 m | 2.2 kt | 6.9 kt | 1.5 kt | 13.7 kt | 1.1 MN | 83 mN/kg |

Assumptions: two coils 0.25 m wide; 0.15 T in the reaction zone (pions at
~350 MeV/c bend inside their ~18 m decay length); magnets at 10× the
stored-energy (virial) floor on near-ideal CNT; +30% for frame and feeds;
coil fins for the tile leak; cooling = helium loop plus radiators.

The tile limit is the master knob. Passive tiles radiating at ~2,900 K
take ~10 MW/m², and the best engine then manages ~22 mN/kg — a 10 ly
crossing of ~200 years. Helium cooling quadruples the specific thrust.

## Design Point

**~180 TW of gammas (~500 TW of annihilation), 850 t of payload.** Dry
mass ~9.9 kt; ~27 kt at departure fueled for 10 ly.

```
  ← direction of travel (accelerating)

  [payload | anti store | dust tank | W plate]═══════ 1 km boom ═══════( engine )
   16 m wide                                                     annihilation point
                                                                 inside a ~1 km ring
                                                                 nozzle; tiles face in
```

| Part | Mass | Notes |
|---|---|---|
| Engine | ~7,850 t | two superconducting rings, ~535 m radius, at ~900 K; ~1,700 t of 5 cm helium-cooled tungsten tiles; ~3,200 t of magnet; ~1,500 t of frame and feeds; ~300 t of coil fins; ~1,100 t of helium loop and radiators |
| Boom | ~200 t | 1 km, guyed at its quarter points; 684 kN in compression |
| Shield plate | ~360 t | 7 cm tungsten, 8 m radius, helium-cooled; takes ~2.9 GW; its own fins at the payload end |
| Power plant | ~100 t | the turbines on the tile loop, and the ~40 t port heater |
| Structure, tanks, systems, hab | ~500 t | |
| **Payload** | **~850 t** | |
| Matter dust tank | ~8.6 kt of dust (10 ly) | ~72 m long at 8 m radius, behind the plate: the second shield layer and the annihilation partner |
| Antimatter store | ~8.6 kt of anti-dust (10 ly) | levitated in a magnetic store behind the tank; fed down the boom at up to ~2.8 g/s |

Thrust ~684 kN; ~2.6 mG at departure, rising as propellant burns.

**Payload is nearly free.** The engine and its propellant are most of the
ship, so carrying more costs little time:

| Payload | 10 ly | 30 ly |
|---|---|---|
| 100 t | ~96 yr | ~196 yr |
| **850 t** | **~98 yr** | **~201 yr** |
| 2,000 t | ~103 yr | ~209 yr |
| 5,000 t | ~113 yr | ~226 yr |

## Flight

The engine burns the **whole crossing**: accelerate to the midpoint, flip
the ship, brake the rest. Propellant is loaded for the hop:

| Hop | Time | Peak | Departure | Antimatter |
|---|---|---|---|---|
| α Cen (4.4 ly) | ~60 yr | ~0.15c | ~20 kt | ~5.3 kt |
| 10 ly | ~98 yr | ~0.21c | ~27 kt | ~8.6 kt |
| 20 ly | ~153 yr | ~0.27c | ~37 kt | ~13.5 kt |
| 30 ly | ~201 yr | ~0.31c | ~45 kt | ~17.7 kt |

That is about the old freighter's crossing times, honestly. Coasting at a
lower peak saves propellant and costs time. At the peaks above, time
dilation is a few percent: a 10 ly crossing runs about a year shorter
aboard than at home. The times are non-relativistic; at 0.3c they are a
few percent off.

**Flipping is back.** The ship rotates at midcourse. The engine leads
into the medium on the braking half, so its rings carry their own Whipple
strips, and the tungsten plate is good armor for the other end.

**The medium gets harder.** At 0.21c an ISM proton arrives with ~21 MeV
(4.7 MeV at 0.1c) and a dust grain with ~4× the energy; at 0.31c, ~49 MeV
and ~10×. Debris defense must be redone for these speeds.

## Heat

Almost all the waste leaves for free: ~180 TW of gammas and the pion and
muon exhaust fly away, and annihilation happens in vacuum with no chamber
walls. Only what the hardware intercepts has to be handled:

| Source | Heat | Temperature | Handled by |
|---|---|---|---|
| Engine tiles | ~84 GW | ~2,900 K | helium loop (~11 t/s) to turbines: ~25 GW of electricity, ~59 GW rejected through ~110,000 m² of fins at ~1,500 K |
| Leak through the tiles into the coils | ~4 GW | ~900 K | ~60,000 m² of coil fins |
| Shield plate | ~2.9 GW | ~2,900 K | its own helium loop and ~5,600 m² of fins at the payload end |
| Port heater (engine cold) | ~70 MW of waste | ~1,000–1,500 K | the plant fins |
| Hab, avionics, stores | MW | ~300 K | small radiators |

**Radiators must be edge-on to the engine.** The plate's shadow is only as
wide as the plate, 8 m in radius. A kilometre from the engine, any surface
outside it faces ~14 MW/m² of gammas. So every radiator, at both ends of
the boom, is a fin in a plane that contains the line to the reaction
point; its faces never see it. Everything else in the payload stack stays
inside the 8 m shadow.

**The dust tank must stay cold.** Metal reverts near 1000 K. The plate
takes the gammas; the tank behind it is insulated from it.

## Power

The tile loop gives **~25 GW of electricity whenever the engine burns** —
which is the whole crossing. Hotel, sensors, and FELs all fit with room to
spare. With 1000 K superconductors nothing aboard needs cryogenic
cooling: nozzle coils, the antimatter store's levitation, and the dust
pumps all run warm.

**In port the same turbines run on a second heat source:** a small
**annihilation heater** beside them at the engine end. Anti-dust and dust
meet inside a tungsten core ~0.9 m in radius, thick enough to stop the
gammas (~20 radiation lengths) and the pions (~11 cm). About 84% of the
energy stays as heat; the rest leaves as neutrinos from the stopped pions
and muons. It is the one place on the ship where annihilation is fully
contained, and it can be, because it is small.

| | Value |
|---|---|
| Thermal power | ~100 MW (surface at ~10 MW/m²) |
| Electricity | ~30 MW |
| Core | ~40 t of tungsten |
| Antimatter | ~20 g a year |

It feeds from the same anti-dust line as the engine. Pion capture on
tungsten makes neutrons and activates the core over time, which is
another reason it lives at the far end of the boom. The 100 GW reactor sim
no longer powers the ship.

## Maneuvering

There is **one drive**. If the ship moves, the main engine moves it.

- **Throttle.** The engine runs at any power from a trickle up to
  ~500 TW of annihilation; thrust follows at ~1.4 N per GW.
- **Attitude.** Steering the nozzle field slightly off axis vectors the
  thrust; with the mass spread along a kilometre of boom, a small angle is
  a large torque. The midcourse flip uses the same trick.
- **In-system**, at full thrust, burning the whole way, between a ship
  that has arrived (~15 kt, ~4.6 mG) and one fueled to leave (~27 kt,
  ~2.6 mG):

| Trip | Time | Δv | Antimatter |
|---|---|---|---|
| Earth–Moon distance | 2–3 days | 6–8 km/s | ~0.5–0.7 t |
| 1 AU | 42–56 days | 120–165 km/s | ~10–14 t |
| 5 AU | 94–126 days | 275–370 km/s | ~22–30 t |

- **Ports.** A courier never docks. It parks several light-seconds out,
  with its exhaust pointed away from anything that matters, and shuttles
  carry payload in.
- **Combat.** Everything happens at milligees. Fights are slow and
  positional: who saw whom first, where the exhaust points, and how much
  propellant each side can spend.

## Shielding and Sleepers

- **Engine gammas:** the plate and the dust tank behind it shadow the
  whole payload stack.
- **Cosmic rays:** isotropic, so a sleeper needs shielding all round —
  ~700 g/cm², ~45 t of water or dust per pod. With payload this cheap, a
  courier can carry tens of fully shielded sleepers: 2,000 t of payload
  costs five years on a 10 ly crossing.
- Most of what crosses is still data, genomes, wombs, AI persons, and
  unique objects.

## The Bomb

A 10 ly load of ~8.6 kt of antimatter is ~1.5×10²⁴ J, **~370 Tt** —
several Chicxulubs; a 30 ly load is ~760 Tt. One 4 mm anti-grain is
~1 kt. A courier that loses its store takes everything within several
light-seconds with it. Ports keep couriers far out; transfers ride
shuttles. IS ships enough antimatter on one courier to end a world.

## What It Changes

- **No bulk interstellar trade.** Systems are self-sufficient; what crosses
  is information, seeds, minds, a few sleepers, and the rare object.
- **Colonies are founded from genomes and wombs**; humans in the flesh
  cross rarely, in tens per ship.
- **IS** is the antimatter monopoly, with stellar collectors at its
  facilities, and the only maker of the metal.
- **Crossings are a century or two**, close to the old canon; history and
  the IS service interval need only light re-timing.

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
3. **Tile cooling.** 50 MW/m² of volumetric gamma heating carried off by
   ~11 t/s of helium for centuries. It sets the nozzle size and most of
   the engine mass.
4. **Debris at 0.2–0.3c.** Grain energies up to ~10× the freighter's.
5. **Crew.** An AI captain and sleepers, or a waking human crew.
6. **Range.** Tankage for 30 ly hops (~45 kt departing).
7. **Thrust vectoring.** How far the nozzle field can steer the exhaust
   off axis, which sets the flip time and the attitude authority.

## Assumptions

| | Value |
|---|---|
| Gamma share of annihilation energy | 0.36 |
| Effective exhaust over total propellant | 0.41c |
| Thrust per annihilation power | 1.4 N/GW |
| Tile limit | 50 MW/m², helium-cooled |
| Tile thickness | 5 cm tungsten, 15 radiation lengths (1.0 t/m²); ~5% leaks |
| Superconductor operating limit | ~1000 K; coils run at ~900 K |
| Nozzle field in the reaction zone | 0.15 T |
| Magnet mass | 10× virial floor, CNT at 25 GPa allowable |
| Radiator fins | ~5 kg/m², edge-on to the reaction |
| Payload | 850 t |

Sources: Keane & Zhang, "Beamed Core Antimatter Propulsion: Engine Design
and Optimization", JBIS 2012 (arXiv 1205.2281).
