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
nozzle is modelled in `crates/hf-nozzle` (driven from `modeling/nozzle.py`
and `modeling/courier.py`), which tracks pions through the real coil
fields, with their decays to muons and electrons. Counted over all the
propellant burned — matter and antimatter, including the energy lost to
gammas and neutrinos — it gives an **effective exhaust of ~0.28c** and
**~0.93 N of thrust per GW of annihilation**. Antimatter is half the
propellant.

Where the energy goes: ~36% leaves as gammas, ~42% as neutrinos (pions
decay to muons within ~18 m, muons to electrons over ~2 km, and their
neutrinos keep whatever direction the parent had), and what is left
leaves as the charged exhaust.

### The nozzle

- **One mirror ring**, ~535 m in radius, set **800 m from the reaction
  point** toward the direction of travel and carrying ~25 MA. It makes
  only **~5 mT** at the reaction point: nothing solid sits within 800 m,
  so the pions and muons have room to swing wide while the field turns
  them. Magnets ~1.0 kt.
- **Most charged particles reflect** off the ring's field and leave
  behind the ship. About **0.9%** of the annihilation power leaks forward
  through the ring. Nothing is there for it to hit.
- **Nothing may sit on the axis.** The field is symmetric about the axis,
  so a particle born on it keeps zero angular momentum about it, and its
  orbit keeps coming back to it: the nozzle refocuses the reaction point
  onto the axis again and again. A plate only 1 m in radius on the axis, a
  kilometre out, catches ~0.5% of the annihilation power (~2.5 TW). A
  shield plate there cannot be cooled and a deflector in front of it only
  moves the hits to the ship's sides. So the ship is not on the axis.
- **The ring is a shelter.** The same conservation law forbids particles
  from the reaction from reaching anywhere the ring's magnetic flux is
  large. Even the hardest pions (~930 MeV/c) cannot come within
  **~330–380 m** of the conductor. That is why the coils see no charged
  hits, and it is where the ship rides (see Design Point).
- **Steering** is done by moving the reaction point, not the magnet (see
  Maneuvering).

### The gamma problem

Gammas are neutral (no field steers them) and penetrating (a 200 MeV
shower needs 15–20 radiation lengths of tungsten, 5–7 cm). There are
three places they hurt:

1. **The nozzle's own magnet.** The ring's conductor sits in the shadow of
   a tungsten tile facing the reaction. How much gamma flux a tile can take
   sets how close to the reaction anything may sit:
   r ≥ √(G / 4π·q), with G the gamma power and q the tile limit. The tiles
   are **helium-cooled**: gammas deposit heat through the tile's depth, not
   at its face, so channels a few millimetres apart hold the tungsten
   within tens of kelvin of the coolant, and **q ≈ 50 MW/m²**. The helium
   carries the heat to the ship's power plant. With 1000 K superconductors
   the coils run warm; the tiles need only 15 radiation lengths (~5 cm),
   and the ~5% of each shower that leaks through is radiated by the coils
   at ~900 K.
2. **The rest of the ship.** It rides the ring, ~1 km from the reaction,
   behind shield plates. Their heat scales with the ship's cross-section
   seen from the reaction point.
3. **Using the energy.** We do not. Absorbed gamma energy becomes heat at
   the absorber's temperature, and ~3,000 K cannot make a 0.4c exhaust.
   The gamma third leaves the ship. That is the honest cost.

### Why the engine is big

Thrust grows with power (∝ G). The closest any tiled structure may sit
grows as √G, so the nozzle grows with it, and so do the tiles and the
magnet. Scaling the modelled nozzle and picking the best field at each
size (`modeling/courier.py`), at q = 50 MW/m²:

| Gamma power | Ring radius | Field | Magnet | Gamma on conductor | Engine | Thrust | Thrust/engine mass | 10 ly |
|---|---|---|---|---|---|---|---|---|
| 30 TW | 218 m | 12 mT | 0.37 kt | 10 GW | 1.5 kt | 86 kN | 57 mN/kg | ~151 yr |
| 100 TW | 399 m | 6.7 mT | 0.72 kt | 20 GW | 2.7 kt | 270 kN | 100 mN/kg | ~113 yr |
| **180 TW** | **535 m** | **5.0 mT** | **1.0 kt** | **27 GW** | **3.7 kt** | **460 kN** | **125 mN/kg** | **~103 yr** |
| 300 TW | 691 m | 3.9 mT | 1.3 kt | 35 GW | 4.8 kt | 740 kN | 152 mN/kg | ~96 yr |
| 500 TW | 892 m | 3.0 mT | 1.8 kt | 46 GW | 6.4 kt | 1.2 MN | 183 mN/kg | ~92 yr |

Assumptions: magnet at 10× the stored-energy floor on near-ideal CNT;
conductor cross-section from that mass; tiles, coil fins and the helium
loop at ~74 t per GW of gamma the conductor shadows (twice its bare
shadow, for tile overhang and frame); frame and feeds ~700 t per 535 m of
ring radius.

Bigger engines still gain, but slowly: 500 TW cuts a 10 ly crossing from
~103 to ~92 years and needs ~22 kt of antimatter instead of ~9. The tile
limit is the master knob: passive tiles at ~10 MW/m² give an engine about
a quarter as good.

## Design Point

**~180 TW of gammas (~500 TW of annihilation), 850 t of payload.** Dry
mass ~5.4 kt; ~23 kt at departure fueled for 10 ly.

**The ship rides the ring.** Two pods sit on the mirror ring at opposite
sides, each on the line from the reaction point through the conductor,
its shield plate ~20 m beyond the conductor and facing the reaction. They
are inside the ring's magnetic shelter: in a model run of a million
annihilations, 2 of 600,000 charged tracks reached a pod (~0.2 ppm,
~100 MW, both pods together). The axis is empty. There is no boom: the
thrust acts on the ring, and the pods are bolted to it.

```
            ↑ direction of travel (accelerating)

        pod ▮                              ▮ pod       plates face the
           ▮                                ▮          reaction point
     ring ●━━━━━━━━━━━━ 1,070 m ━━━━━━━━━━━━━● ring
            ╲                              ╱
               ╲                        ╱       pellet streams
                  ╲         800 m    ╱          from the pods
                     ╲      ┊     ╱
                        ╲   ┊  ╱
                           ✦        annihilation point
                           ↓        exhaust
```

| Part | Mass | Notes |
|---|---|---|
| Engine | ~3,700 t | mirror ring (~535 m radius, ~25 MA, ~900 K), ~1,000 t of magnet; ~1,100 t of 5 cm helium-cooled tungsten tiles; ~200 t of coil fins; ~700 t of helium loop and radiators; ~700 t of frame and feeds |
| Shield plates | ~250 t | two, 7 cm tungsten, 5.7 m radius, helium-cooled; ~1.5 GW of gammas each |
| Power plant | ~100 t | the turbines on the tile loop, and the ~40 t port heater |
| Structure, tanks, systems, hab | ~500 t | split between the pods |
| **Payload** | **~850 t** | split between the pods |
| Matter dust | ~9.0 kt (10 ly) | ~63 m of each pod behind its plate: the second shield layer and the annihilation partner |
| Antimatter | ~9.0 kt (10 ly) | levitated in a magnetic store behind each dust tank |

Each pod is ~5.7 m in radius and ~150 m long. Both pods fire pellets of
dust and anti-dust across ~1 km at the reaction point, up to ~2.8 g/s of
each in all. Pellets are neutral, so the field does not bend them. Both
pods draw evenly, which keeps the centre of mass on the axis.

Thrust ~460 kN; ~2 mG at departure, ~9 mG on arrival.

**Payload is nearly free.** The engine and its propellant are most of the
ship, so carrying more costs little time:

| Payload | 10 ly | 30 ly |
|---|---|---|
| 100 t | ~98 yr | ~217 yr |
| **850 t** | **~103 yr** | **~225 yr** |
| 2,000 t | ~110 yr | ~237 yr |
| 5,000 t | ~125 yr | ~263 yr |

## Flight

The engine burns the **whole crossing**: accelerate to the midpoint, flip
the ship, brake the rest. Propellant is loaded for the hop:

| Hop | Time | Peak | Departure | Antimatter |
|---|---|---|---|---|
| α Cen (4.4 ly) | ~60 yr | ~0.15c | ~16 kt | ~5.3 kt |
| 10 ly | ~103 yr | ~0.20c | ~23 kt | ~9.0 kt |
| 20 ly | ~167 yr | ~0.25c | ~35 kt | ~14.5 kt |
| 30 ly | ~225 yr | ~0.28c | ~44 kt | ~19.5 kt |

That is about the old freighter's crossing times, honestly. Coasting at a
lower peak saves propellant and costs time. At the peaks above, time
dilation is a few percent: a 10 ly crossing runs about a year shorter
aboard than at home. The trip model works in rapidity, so the times
include relativity.

**Flipping is back.** The ship rotates at midcourse. Accelerating, the
ring and the pods' aft ends lead into the medium; braking, the reaction
point leads and the tile faces and tungsten plates take what gets past
it. The ring carries its own Whipple strips.

**The medium gets harder.** See Debris Defense.

## Heat

Almost all the waste leaves for free: ~180 TW of gammas and the pion and
muon exhaust fly away, and annihilation happens in vacuum with no chamber
walls. Only what the hardware intercepts has to be handled:

| Source | Heat | Temperature | Handled by |
|---|---|---|---|
| Engine tiles | ~54 GW | ~2,900 K | helium loop (~7 t/s) to turbines: ~16 GW of electricity, ~38 GW rejected through ~70,000 m² of fins at ~1,500 K |
| Leak through the tiles into the coils | ~3 GW | ~900 K | ~40,000 m² of coil fins |
| Shield plates | ~3.0 GW of gammas | ~2,900 K | a helium loop and ~3,000 m² of fins per pod |
| Port heater (engine cold) | ~70 MW of waste | ~1,000–1,500 K | the plant fins |
| Hab, avionics, stores | MW | ~300 K | small radiators |

**Radiators must be edge-on to the engine.** A plate's shadow is only as
wide as the plate, 5.7 m in radius. A kilometre from the reaction, any
surface outside it faces ~15 MW/m² of gammas. So every radiator, on the
ring and on the pods, is a fin in a plane that contains the line to the
reaction point; its faces never see it. Everything else in a pod stays
inside its plate's shadow.

**The dust tanks must stay cold.** Metal reverts near 1000 K. The plates
take the gammas; the tanks behind them are insulated from them.

## Power

The tile loop gives **~16 GW of electricity whenever the engine burns** —
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

It feeds from the pods' anti-dust lines. Pion capture on tungsten makes
neutrons and activates the core over time, so it sits on the ring well
away from both pods.

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
  ~500 TW of annihilation; thrust follows at ~0.93 N per GW.
- **Attitude under burn.** The pods aim their pellet streams to meet
  slightly off the axis. Moving the reaction point sideways gives ~1 m of
  torque arm per metre moved: ~0.5 MN·m per metre at full thrust. Nothing
  moves but the aim. Most of the ship's mass lies along the line through
  the pods, so turning about that line is easy: a **flip in an hour on the
  main drive needs the reaction point moved ~1–2 m.** Tilting the ring
  works too (1° gives ~11 m of arm) but needs a mechanism.
- **Reaction jets.** Clusters on the ring and the pods, burning dust as a
  monopropellant: metal that reverts in a small chamber releases 216 MJ/kg
  by itself, good for ~10–15 km/s of exhaust. They do attitude with the
  engine cold, fine pointing, and proximity work in port.
  - A flip in an hour with the engine cold needs ~0.5 kN at each side of
    the ring and ~0.3 t of dust.
  - Translation is dear: 10 m/s on a 15 kt ship is ~12 t of dust.
- **In-system**, on the main drive at full thrust, burning the whole way,
  between a ship that has arrived (~5.4 kt, ~9 mG) and one fueled to
  leave (~23 kt, ~2 mG):

| Trip | Time | Δv | Antimatter |
|---|---|---|---|
| Earth–Moon distance | 1.5–3 days | 6–11 km/s | ~0.4–0.8 t |
| 1 AU | 31–64 days | 110–230 km/s | ~7–15 t |
| 5 AU | 68–140 days | 240–510 km/s | ~16–34 t |

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

**The engine is the target, not the payload.** The pods show the medium
~200 m² together. The mirror ring — conductor, tiles, and Whipple strips,
a kilometre across — shows a few thousand m², most of the old freighter's
face, and it is exposed on both halves of the crossing. Interdiction has
to cover a kilometre-wide annulus, not a nose.

What helps: the FEL bus has ~16 GW to draw on while the engine burns, and
the ship's own sensor baseline is a kilometre. **The anti-social laser
AI is back**, with more to shoot at and more power to shoot with. To be
worked out properly.

## Shielding and Sleepers

- **Engine gammas:** each pod's plate and the dust tank behind it shadow
  the rest of the pod.
- **Engine pions, muons and electrons:** none reach the pods; they ride
  inside the ring's magnetic shelter.
- **Cosmic rays:** isotropic, so a sleeper needs shielding all round —
  ~700 g/cm², ~45 t of water or dust per pod. With payload this cheap, a
  courier can carry tens of fully shielded sleepers: 2,000 t of payload
  costs seven years on a 10 ly crossing.
- Most of what crosses is still data, genomes, wombs, AI persons, and
  unique objects.

## The Bomb

A 10 ly load of ~9.0 kt of antimatter is ~1.6×10²⁴ J, **~390 Tt** —
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

1. **Nozzle refinements.** The model uses a fitted pion momentum
   spectrum, ignores electron radiation, and treats the conductor as a
   thin wire. Next: the tile and conductor as real solids, and the pods'
   own small fields (stores, hab) in the shelter.
2. **Injection.** Within about a metre of the reaction point, the gammas
   heat an incoming pellet past 1000 K in about a millisecond, so pellets
   revert and arrive as gas. Whether that makes a steady core, and how
   precisely two pellet streams can be aimed across a kilometre.
3. **Debris at 0.2–0.3c**, with the ring as the main target.
4. **Tile cooling.** 50 MW/m² of volumetric gamma heating carried off by
   ~7 t/s of helium in closed, segmented loops for centuries.
5. **Crew.** An AI captain and sleepers, or a waking human crew.
6. **Range.** Tankage for 30 ly hops (~44 kt departing).
7. **Cosmic rays.** The ring's field (a magnetic moment of ~2×10¹³ A·m²)
   may also shelter the pods from much of the cosmic-ray flux while it is
   energized. Modelable with the same code, starting particles far away.

## Assumptions

| | Value |
|---|---|
| Gamma share of annihilation energy | 0.36 |
| Effective exhaust over total propellant | 0.28c (modelled) |
| Thrust per annihilation power | 0.93 N/GW (modelled) |
| Neutrino share of annihilation energy | ~0.42 (modelled) |
| Tile limit | 50 MW/m², helium-cooled |
| Tile thickness | 5 cm tungsten, 15 radiation lengths (1.0 t/m²); ~5% leaks |
| Superconductor operating limit | ~1000 K; coils run at ~900 K |
| Nozzle | one mirror ring, ~535 m radius, 800 m toward the direction of travel, ~5 mT at the reaction point |
| Ship | two pods on the ring, 5.7 m radius, ~150 m long, plates ~980 m from the reaction |
| Magnet mass | 10× virial floor, CNT at 25 GPa allowable |
| Radiator fins | ~5 kg/m², edge-on to the reaction |
| Payload | 850 t |

Sources: Keane & Zhang, "Beamed Core Antimatter Propulsion: Engine Design
and Optimization", JBIS 2012 (arXiv 1205.2281).
