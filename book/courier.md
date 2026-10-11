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
(2012, Geant4) showed the idea with compact ~10 T coils. This design's
nozzle is modelled in `modeling/nozzle.py`, which tracks pions through the
real loop fields, with their decays to muons and electrons. Counted over
all the propellant burned — matter and antimatter, including the energy
lost to gammas and neutrinos — it gives an **effective exhaust of ~0.30c**
and **~1.0 N of thrust per GW of annihilation**. Antimatter is half the
propellant.

Where the energy goes: ~36% leaves as gammas, ~43% as neutrinos (pions
decay to muons within ~18 m, muons to electrons over ~2 km, and their
neutrinos keep whatever direction the parent had), and what is left
leaves as the charged exhaust.

### The nozzle

- **One mirror ring**, ~535 m in radius, set ~430 m toward the ship from
  the reaction point and carrying ~18 MA. It makes only **~10 mT** at the
  reaction point: nothing solid sits within half a kilometre, so the pions
  and muons have room to swing wide while the field turns them. A stronger
  field barely helps and the magnets' stored energy grows with its square.
- **Most charged particles reflect** off the ring's field and leave along
  −z. About **0.8%** of the annihilation power leaks forward through the
  mirror, along the axis, straight at the ship.
- **A sweeper** stops that. A ~20 mT transverse field over ~440 m between
  the ring and the plate — a particle accelerator's trick — bends the leak
  off the axis. Without it the plate takes ~2% of the annihilation power
  (~10 TW); with it, ~3 parts per million (~1.5 GW). Deflecting the leak
  pushes the ship sideways at ~3% of the thrust; the nozzle tilts ~1.5° to
  cancel it.
- **The coils see no charged particles.** Field lines from the reaction
  point never reach them.
- Magnets ~0.5 kt for the ring and ~0.1 kt for the sweeper.

### The gamma problem

Gammas are neutral (no field steers them) and penetrating (a 200 MeV
shower needs 15–20 radiation lengths of tungsten, 5–7 cm). There are
three places they hurt:

1. **The nozzle's own magnets.** Each conductor — the mirror ring and the
   sweeper — sits in the shadow of a tungsten tile facing the reaction. How much gamma flux a tile can take
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
| 10 TW | 126 m | 400 t | ~10 t | 270 t | 0.87 kt | 28 kN | 32 mN/kg |
| 100 TW | 400 m | 1.3 kt | 0.26 kt | 0.85 kt | 3.1 kt | 280 kN | 91 mN/kg |
| **180 TW** | **535 m** | **1.7 kt** | **0.62 kt** | **1.1 kt** | **4.5 kt** | **500 kN** | **112 mN/kg** |
| 500 TW | 890 m | 2.8 kt | 2.9 kt | 1.9 kt | 9.9 kt | 1.4 MN | 141 mN/kg |
| 1,000 TW | 1,260 m | 4.0 kt | 8.2 kt | 2.7 kt | 19 kt | 2.8 MN | 145 mN/kg |

Assumptions: tiles for the ring and the sweeper conductors (0.5 m of
shadowed width in all); magnets scaled from the modelled ring and sweeper
(10× the stored-energy floor on near-ideal CNT); +30% for frame and feeds;
coil fins for the tile leak; cooling = helium loop plus radiators.

Bigger engines past ~180 TW gain little: 500 TW cuts a 10 ly crossing from
~103 to ~91 years and more than doubles the antimatter. The tile limit is
the master knob: passive tiles at ~10 MW/m² give an engine about a quarter
as good.

## Design Point

**~180 TW of gammas (~500 TW of annihilation), 850 t of payload.** Dry
mass ~6.5 kt; ~25 kt at departure fueled for 10 ly.

```
  ← direction of travel (accelerating)

  [payload | anti store | dust tank | W plate]═════ 1 km boom ═════╪══════( engine )
   16 m wide                                                  sweeper   annihilation point;
                                                                        mirror ring 535 m
                                                                        radius, 430 m toward
                                                                        the ship
```

| Part | Mass | Notes |
|---|---|---|
| Engine | ~4,500 t | mirror ring (~535 m radius, ~18 MA, ~900 K) and sweeper, ~620 t of magnet; ~1,700 t of 5 cm helium-cooled tungsten tiles; ~700 t of frame and feeds; ~300 t of coil fins; ~1,100 t of helium loop and radiators |
| Boom | ~200 t | 1 km, guyed at its quarter points; 500 kN in compression |
| Shield plate | ~360 t | 7 cm tungsten, 8 m radius, helium-cooled; takes ~2.9 GW of gammas and ~1.5 GW of swept leak; its own fins at the payload end |
| Power plant | ~100 t | the turbines on the tile loop, and the ~40 t port heater |
| Structure, tanks, systems, hab | ~500 t | |
| **Payload** | **~850 t** | |
| Matter dust tank | ~9.1 kt of dust (10 ly) | ~76 m long at 8 m radius, behind the plate: the second shield layer and the annihilation partner |
| Antimatter store | ~9.1 kt of anti-dust (10 ly) | levitated in a magnetic store behind the tank; fed down the boom at up to ~2.8 g/s |

Thrust ~500 kN; ~2 mG at departure, rising as propellant burns.

**Payload is nearly free.** The engine and its propellant are most of the
ship, so carrying more costs little time:

| Payload | 10 ly | 30 ly |
|---|---|---|
| 100 t | ~99 yr | ~214 yr |
| **850 t** | **~103 yr** | **~222 yr** |
| 2,000 t | ~109 yr | ~232 yr |
| 5,000 t | ~123 yr | ~256 yr |

## Flight

The engine burns the **whole crossing**: accelerate to the midpoint, flip
the ship, brake the rest. Propellant is loaded for the hop:

| Hop | Time | Peak | Departure | Antimatter |
|---|---|---|---|---|
| α Cen (4.4 ly) | ~61 yr | ~0.15c | ~17 kt | ~5.4 kt |
| 10 ly | ~103 yr | ~0.20c | ~25 kt | ~9.1 kt |
| 20 ly | ~166 yr | ~0.26c | ~36 kt | ~14.6 kt |
| 30 ly | ~222 yr | ~0.29c | ~45 kt | ~19.4 kt |

That is about the old freighter's crossing times, honestly. Coasting at a
lower peak saves propellant and costs time. At the peaks above, time
dilation is a few percent: a 10 ly crossing runs about a year shorter
aboard than at home. The times are non-relativistic; at 0.3c they are a
few percent off.

**Flipping is back.** The ship rotates at midcourse. The engine leads
into the medium on the braking half, so its ring carries its own Whipple
strips, and the tungsten plate is good armor for the other end.

**The medium gets harder.** See Debris Defense.

## Heat

Almost all the waste leaves for free: ~180 TW of gammas and the pion and
muon exhaust fly away, and annihilation happens in vacuum with no chamber
walls. Only what the hardware intercepts has to be handled:

| Source | Heat | Temperature | Handled by |
|---|---|---|---|
| Engine tiles | ~84 GW | ~2,900 K | helium loop (~11 t/s) to turbines: ~25 GW of electricity, ~59 GW rejected through ~110,000 m² of fins at ~1,500 K |
| Leak through the tiles into the coils | ~4 GW | ~900 K | ~60,000 m² of coil fins |
| Shield plate | ~4.4 GW (gammas plus swept leak) | ~2,900 K | its own helium loop and ~8,500 m² of fins at the payload end |
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
another reason it lives at the far end of the boom.

The 100 GW D-³He mirror reactor no longer powers the ship. It powers
**stations and colonies**: only so many people want to live next to
kilotonnes of antimatter.

**The helium loops** are closed and segmented, so a leak isolates one
sector of tiles and the engine throttles down around it. Spare helium
rides in the payload stack. A courier that loses enough of it is
throttled, then coasting.

## Maneuvering

**One main drive**, plus **dust reaction jets** for everything small.

- **Throttle.** The main engine runs at any power from a trickle up to
  ~500 TW of annihilation; thrust follows at ~1.0 N per GW.
- **Attitude under burn.** Steering the nozzle field slightly off axis
  vectors the thrust; with the mass spread along a kilometre of boom, a
  small angle is a large torque.
- **Reaction jets.** Clusters at both ends of the boom, burning dust as a
  monopropellant: metal that reverts in a small chamber releases 216 MJ/kg
  by itself, good for ~10–15 km/s of exhaust. They do attitude with the
  engine cold, fine pointing, proximity work in port, and a backup flip.
  - A flip in an hour needs ~8.5 kN at each end of the boom and ~5 t of
    dust; in six hours, a few hundred newtons and under a tonne.
  - Translation is dear: 10 m/s on a 15 kt ship is ~12 t of dust.
- **In-system**, on the main drive at full thrust, burning the whole way,
  between a ship that has arrived (~7 kt, ~7 mG) and one fueled to
  leave (~25 kt, ~2 mG):

| Trip | Time | Δv | Antimatter |
|---|---|---|---|
| Earth–Moon distance | 2–3 days | 6–10 km/s | ~0.4–0.8 t |
| 1 AU | 34–63 days | 110–210 km/s | ~8–15 t |
| 5 AU | 75–140 days | 245–460 km/s | ~18–34 t |

- **Ports.** A courier never docks. It parks several light-seconds out,
  with its exhaust pointed away from anything that matters, and shuttles
  carry payload in.
- **Combat.** Everything happens at milligees. Fights are slow and
  positional: who saw whom first, where the exhaust points, and how much
  propellant each side can spend.

## Debris Defense

The freighter's debris section was built for 0.1c (and an earlier draft
for 0.5c). The courier peaks at 0.2–0.3c:

| | 0.21c | 0.31c |
|---|---|---|
| ISM proton | ~21 MeV | ~49 MeV |
| 100 µm grain | ~3 MJ | ~7.5 MJ |
| 1 mm grain | ~0.8 t TNT | ~1.8 t TNT |
| 1 cm pebble | ~0.8 kt TNT | ~1.8 kt TNT |
| Fire-control floor (10 ms) | ~800 km | ~1,350 km |
| Radar detect floor | ~1,200 km | ~2,600 km |

**The engine is the target, not the payload.** The payload stack shows the
medium ~200 m². The mirror ring and the sweeper — conductors, tiles, and
Whipple strips, the ring a kilometre across — show ~5,000 m², the old
freighter's whole face, and
they are exposed on both halves of the crossing. Interdiction has to cover
a kilometre-wide annulus, not a nose.

What helps: the FEL bus has ~25 GW to draw on while the engine burns, and
the ship's own sensor baseline is a kilometre. **The anti-social laser
AI is back**, with more to shoot at and more power to shoot with. To be
worked out properly.

## Shielding and Sleepers

- **Engine gammas:** the plate and the dust tank behind it shadow the
  whole payload stack.
- **Cosmic rays:** isotropic, so a sleeper needs shielding all round —
  ~700 g/cm², ~45 t of water or dust per pod. With payload this cheap, a
  courier can carry tens of fully shielded sleepers: 2,000 t of payload
  costs six years on a 10 ly crossing.
- Most of what crosses is still data, genomes, wombs, AI persons, and
  unique objects.

## The Bomb

A 10 ly load of ~9.1 kt of antimatter is ~1.6×10²⁴ J, **~390 Tt** —
several Chicxulubs; a 30 ly load is ~840 Tt. One 4 mm anti-grain is
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
IS box as a fusion torch; the 100 GW plant as ship power (it moves to
stations and colonies); "the ship never flips."

## Open Questions

1. **Nozzle refinements.** The model (`modeling/nozzle.py`) uses a fitted
   pion momentum spectrum, ignores electron radiation, and resolves the
   swept leak only to ~3 ppm. Next: finer leak statistics, a real
   sweeper coil instead of an ideal field slab, and thrust vectoring.
2. **Debris at 0.2–0.3c**, with the engine's ring as the main target.
3. **Tile cooling.** 50 MW/m² of volumetric gamma heating carried off by
   ~11 t/s of helium in closed, segmented loops for centuries.
4. **Crew.** An AI captain and sleepers, or a waking human crew.
5. **Range.** Tankage for 30 ly hops (~45 kt departing).
6. **Thrust vectoring.** How far tilting the mirror ring steers the
   exhaust off axis, which sets attitude authority under burn and trims
   the sweeper's side push. Now modelable.

## Assumptions

| | Value |
|---|---|
| Gamma share of annihilation energy | 0.36 |
| Effective exhaust over total propellant | 0.30c (modelled) |
| Thrust per annihilation power | 1.0 N/GW (modelled) |
| Neutrino share of annihilation energy | ~0.43 (modelled) |
| Tile limit | 50 MW/m², helium-cooled |
| Tile thickness | 5 cm tungsten, 15 radiation lengths (1.0 t/m²); ~5% leaks |
| Superconductor operating limit | ~1000 K; coils run at ~900 K |
| Nozzle | one mirror ring, ~535 m radius, ~430 m toward the ship, ~10 mT at the reaction point; ~20 mT sweeper over ~440 m |
| Magnet mass | 10× virial floor, CNT at 25 GPa allowable |
| Radiator fins | ~5 kg/m², edge-on to the reaction |
| Payload | 850 t |

Sources: Keane & Zhang, "Beamed Core Antimatter Propulsion: Engine Design
and Optimization", JBIS 2012 (arXiv 1205.2281).
