# Courier — Engine Design Notes

Why the courier looks the way it does, and where its numbers come from.
The ship itself — layout, systems, flight — is described in [Ship
Systems](ship-systems.md). The numbers here are first-order; treat engine
masses as ±2×.

The models:

- `crates/hf-nozzle` tracks pions through the real coil fields, with
  their decays to muons and electrons, and reports thrust, leaks, hits on
  the ship, and magnet mass. Its `shelter` mode traces cosmic rays
  backward through the ring's field.
- `modeling/nozzle.py` drives it; `modeling/courier.py` turns its reports
  into masses and trip times and prints the tables in the bible;
  `modeling/shelter.py` folds the shelter runs into doses.

## Why Antimatter

The setting once rested on **catalyzed p-p fusion**, which is probably
impossible. The Sun's core makes ~276 W/m³; the first step of the chain
is a weak-force event that takes a proton billions of years.

Antimatter puts nothing impossible aboard:

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

The price: no engine that can carry a megatonne survives its own gamma
rays. The ship is kilotonnes, and most of it is engine.

## The Engine — Beamed Core

A proton meeting an antiproton makes ~5 pions. About **62%** of the energy
goes to charged pions; about **36%** to neutral pions, which become gamma
rays averaging ~200 MeV, in every direction.

A magnetic nozzle turns the charged pions into an exhaust. Keane & Zhang
(2012, Geant4) showed the idea with compact ~10 T coils. Counted over all
the propellant burned — matter and antimatter, including the energy lost
to gammas and neutrinos — this design's nozzle gives an **effective
exhaust of ~0.28c** and **~0.93 N of thrust per GW of annihilation**.
Antimatter is half the propellant.

Where the energy goes: ~36% leaves as gammas, ~42% as neutrinos (pions
decay to muons within ~18 m, muons to electrons over ~2 km, and their
neutrinos keep whatever direction the parent had), and what is left
leaves as the charged exhaust.

### The Nozzle

- **One mirror ring**, ~535 m in radius, set **800 m from the reaction
  point** toward the direction of travel and carrying ~25 MA. It makes
  only **~5 mT** at the reaction point: nothing solid sits within 800 m,
  so the pions and muons have room to swing wide while the field turns
  them. Magnets ~1.0 kt.
- **Most charged particles reflect** off the ring's field and leave
  behind the ship. About **0.9%** of the annihilation power leaks forward
  through the ring. Nothing is there for it to hit.
- **An ideal sweeper is a trap.** An earlier model put a uniform field
  slab beyond the ring to turn the forward leak aside, and got 0.30c. The
  slab was quietly acting as a second mirror. A real sweeper coil's stray
  field reaches back to the reaction point and pushes the ship sideways
  by ~18%. Real coils give ~0.28c. A second ring on the exhaust side
  traps the exhaust instead of straightening it.

### Nothing on the Axis

The field is symmetric about the axis, so a particle born on it keeps
zero angular momentum about it, and its orbit keeps coming back to it:
the nozzle refocuses the reaction point onto the axis again and again. A
plate only 1 m in radius on the axis, a kilometre out, catches ~0.5% of
the annihilation power (~2.5 TW). It cannot be cooled, and a deflector in
front of it only moves the hits to the ship's sides. So the ship is not
on the axis.

### The Ring Is a Shelter

The same conservation law forbids particles from the reaction from
reaching anywhere the ring's magnetic flux is large. Even the hardest
pions (~930 MeV/c) cannot come within **~330–380 m** of the conductor.
That is why the coils see no charged hits, and it is where the ship
rides: two pods on the ring, each on the line from the reaction point
through the conductor, plates ~20 m beyond it.

The law binds what is born at the reaction point. A muon that decays far
from the axis gives its electron a fresh orbit, and some of those reach
the pods: ~0.35 ppm of the annihilation power (~175 MW) on 285 m pods in
a run of three million annihilations, all electrons, all on the pods'
sides, none within ~40 m of the plates, rising toward the far ends. Pions
and muons: none.

### Steering

Moving the reaction point sideways moves the line of thrust: ~1 m of
torque arm per metre of offset, from aiming the pellet streams. The noise
floor of the model is ~1 m of arm.

### The Gamma Problem

Gammas are neutral (no field steers them) and penetrating (a 200 MeV
shower needs 15–20 radiation lengths of tungsten, 5–7 cm). There are
three places they hurt:

1. **The nozzle's own magnet.** The ring's conductor sits in the shadow of
   a tungsten tile facing the reaction. How much gamma flux a tile can take
   sets how close to the reaction anything may sit:
   r ≥ √(G / 4π·q), with G the gamma power and q the tile limit. The tiles
   are **helium-cooled**: gammas deposit heat through the tile's depth, not
   at its face, so channels a few millimetres apart hold the tungsten
   within tens of kelvin of the coolant, and **q ≈ 50 MW/m²**. With 1000 K
   superconductors the coils run warm; the tiles need only 15 radiation
   lengths (~5 cm), and the ~5% of each shower that leaks through is
   radiated by the coils at ~900 K.
2. **The pods.** Past the shower maximum (~3.7 radiation lengths in
   tungsten for 200 MeV) the energy that leaks falls by e every ~3.6
   radiation lengths: ~1% through 20, ~0.07% through 30. So the plates are
   10 cm (30 radiation lengths). Behind a 5.7 m plate the last ~1 MW needs
   ~7 m of dust (~28 e-folds at ~15 g/cm² each for MeV photons in
   hydrogen) to fall below the cosmic-ray dose. In the standard fit the
   plate is 12.25 m in radius, to shadow the dust ball, and takes ~7 GW;
   the ball's ~10 m of dust is far more than the gamma stop needs. The
   propellant cannot be the stop: by arrival it is gone.
3. **Using the energy.** We do not. Absorbed gamma energy becomes heat at
   the absorber's temperature, and ~3,000 K cannot make a 0.3c exhaust.
   The gamma third leaves the ship. That is the honest cost.

### Why the Engine Is Big

Thrust grows with power (∝ G). The closest any tiled structure may sit
grows as √G, so the nozzle grows with it, and so do the tiles and the
magnet. Scaling the modelled nozzle and picking the best field at each
size (`modeling/courier.py`), at q = 50 MW/m², with the rest of the ship
held at the standard fit and the same tank:

| Gamma power | Ring radius | Field | Magnet | Gamma on conductor | Engine | Thrust | Thrust/engine mass | 10 ly |
|---|---|---|---|---|---|---|---|---|
| 30 TW | 218 m | 16 mT | 0.62 kt | 14 GW | 1.9 kt | 92 kN | 48 mN/kg | ~280 yr |
| 100 TW | 399 m | 8.7 mT | 1.2 kt | 26 GW | 3.6 kt | 290 kN | 79 mN/kg | ~180 yr |
| **180 TW** | **535 m** | **5.0 mT** | **1.0 kt** | **27 GW** | **3.7 kt** | **460 kN** | **125 mN/kg** | **~151 yr** |
| 300 TW | 691 m | 5.0 mT | 2.2 kt | 46 GW | 6.5 kt | 790 kN | 121 mN/kg | ~129 yr |
| 500 TW | 892 m | 3.0 mT | 1.8 kt | 46 GW | 6.4 kt | 1.2 MN | 183 mN/kg | ~121 yr |

Assumptions: magnet at 10× the stored-energy floor on near-ideal CNT;
conductor cross-section from that mass; tiles, coil fins and the helium
loop at ~74 t per GW of gamma the conductor shadows (twice its bare
shadow, for tile overhang and frame); frame and feeds ~700 t per 535 m of
ring radius. Each row is a 10,000-annihilation run, so the field column
is noisy.

The dust balls made the ship heavier, so a stronger ring now pays a
little: at 180 TW, 6.5 mT gives ~0.30c and saves ~2 years on 10 ly for
~0.7 kt more magnet. The design point stays at 5 mT; the optimum is flat.

Bigger engines still gain: 500 TW cuts a 10 ly crossing from ~151 to
~121 years on the same tank. The tile limit is the master knob: passive
tiles at ~10 MW/m² give an engine about a quarter as good. The design
point is **~180 TW of gammas (~500 TW of annihilation)**.

## Trips: Burn, Coast, Brake

The engine runs at fixed power, so its thrust is fixed. Propellant beyond
what a trip needs only makes the ship heavier and slower to accelerate.
`modeling/courier.py` finds, for each trip, the load that makes it
fastest: burn at full power, coast, flip, brake at full power, with
equal rapidity each way. Burning the whole way is never quite best:

| Hop | Burning the whole way | Burn, coast, brake |
|---|---|---|
| 10 ly | ~155 yr on 13.6 kt of antimatter | ~151 yr on 10.5 kt |
| 30 ly | ~315 yr on 27.5 kt | ~308 yr on 13.5 kt (one tank) |

So range is not capped by the tanks. The tank, 13.5 kt of each
propellant, is sized to go 10 ly and back without refuelling (~380
years at ~0.067c); a longer hop just coasts longer.

## Cosmic Rays and the Ring

The ring is a magnetic moment of ~2×10¹³ A·m², and the pods sit in its
strongest field, beside the conductor. `hf-nozzle shelter` traces protons
backward (charge reversed) from points along a pod: a direction is open
if its track escapes to free space. By Liouville's theorem the intensity
along every open direction is the free-space intensity, so the open
fraction is the fraction of the isotropic flux that arrives. The tracer
reproduces Størmer's cutoffs for a dipole.

`modeling/shelter.py` folds the open fractions with the unmodulated
interstellar spectrum, weighted by stopping power in water, above
100 MeV per nucleon (slower particles stop in the pod's skin). Nuclei with
A/Z = 2 have twice a proton's rigidity at the same energy per nucleon:

| Distance from the plate | Protons | A/Z = 2 nuclei |
|---|---|---|
| 0 m | 0.11 | 0.31 |
| 10 m | 0.13 | 0.38 |
| 20 m | 0.17 | 0.44 |
| 40 m | 0.30 | 0.62 |
| 80 m | 0.42 | 0.77 |
| 120 m | 0.63 | 0.88 |
| 220 m | 0.92 | 1.00 |

(Fraction of the free-space dose that arrives, from the field alone.)

The dust does the rest, and this part is not modelled. The deep,
high-energy part of the cosmic-ray dose falls by e for roughly every
100 g/cm² of hydrogen. That is scaled from how the dose falls through
Earth's atmosphere (about tenfold per 380 g/cm² of air) by hydrogen's
shorter nuclear interaction length (52 against 90 g/cm²); it could be
70–150. A bay 1.5 m in radius in 4.2 m of dust, with the field, comes to
~3 µSv/h. The standard fit puts a 57 m³ spherical bay inside ~9.9 m of
dust (~590 g/cm²) for ~0.1 µSv/h; the premium fit uses ~12 m for
~0.03 µSv/h. A sphere is the cheapest shape for a thick shield, ~25%
lighter than a cylinder of the same volume.

## Assumptions

| | Value |
|---|---|
| Gamma share of annihilation energy | 0.36 |
| Effective exhaust over total propellant | 0.28c (modelled) |
| Thrust per annihilation power | 0.93 N/GW (modelled) |
| Neutrino share of annihilation energy | ~0.42 (modelled) |
| Tile limit | 50 MW/m², helium-cooled |
| Tile thickness | 5 cm tungsten, 15 radiation lengths (1.0 t/m²); ~5% leaks |
| Plate | 10 cm tungsten, 30 radiation lengths, 12.25 m radius; ~0.07% leaks |
| Dust ball | ~9.9 m of dust around a 57 m³ spherical bay, never burned; ~0.1 µSv/h |
| Superconductor operating limit | ~1000 K; coils run at ~900 K |
| Nozzle | one mirror ring, ~535 m radius, 800 m toward the direction of travel, ~5 mT at the reaction point |
| Ship | two pods on the ring, a 24.5 m dust ball then 5.7 m radius, ~285 m long, plates ~980 m from the reaction |
| Tank | 13.5 kt of each propellant: 10 ly and back on one fill |
| Magnet mass | 10× virial floor, CNT at 25 GPa allowable |
| Radiator fins | ~5 kg/m², edge-on to the reaction |
| Packed dust | 0.6 t/m³, matter and antimatter alike |
| Payload | 850 t |

## Open Questions

1. **Nozzle refinements.** The model uses a fitted pion momentum
   spectrum, ignores electron radiation, and treats the conductor as a
   thin wire. Next: the tile and conductor as real solids, and the pods'
   own small fields (stores, hab) in the shelter.
2. **Injection.** Within about a metre of the reaction point, the gammas
   heat an incoming pellet past 1000 K in about a millisecond, so pellets
   revert and arrive as gas. Whether that makes a steady core, and how
   precisely two pellet streams can be aimed across a kilometre.
3. **Electron hits.** A few dozen hits in ten million tracks. A biased
   run (pions sampled toward the pods) would map them properly.
4. **Shower transport.** Plate leakage, the dust balls, and gammas
   scattered from the tiles toward the pods, done properly.
5. **Cosmic-ray transport.** The dust ball's shielding, done with a real
   transport code instead of scaled atmosphere curves.

Sources: Keane & Zhang, "Beamed Core Antimatter Propulsion: Engine Design
and Optimization", JBIS 2012 (arXiv 1205.2281).
