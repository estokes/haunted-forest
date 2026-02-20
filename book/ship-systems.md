# Ship Systems — Engineering Reference

## Hull and Structure

The ship is a cylinder roughly 1 km long and 300 m in radius, constructed
from carbon nanotube composites and graphene sheets — materials with
tensile strengths of 50-100 GPa at densities of ~1,400 kg/m³.

For perspective: the structural stress at the base of a 1 km column under
1G acceleration, at the ship's average density, is about 7 kPa. CNT
handles 50 GPa. The structure could withstand thousands of G before
failing. Continuous 1G is nothing.

The hull is a 1 mm CNT pressure shell — about 3,500 tonnes for the entire
ship. This is thinner than a soda can relative to its size, yet stronger
than any steel structure ever built.

Most of the interior volume is **unpressurized**: cargo bays, hangars,
engineering spaces, fuel tankage. Pressurized habitable space for the crew
of ~10 is a tiny fraction of the total — a few hundred cubic meters of
quarters, a bridge, a workshop. The ship is essentially a massive
automated cargo and exploration vessel with a small human crew making
decisions.


## Ship Architecture

### Cross-Section

The ship is built in concentric layers, designed to protect the crew and
sensitive cargo from radiation regardless of orientation:

```
     Outer hull (CNT, 1mm)
     H₂ ice coating (10-30cm, entire surface)
     ─────────────────────────────────────
     Unpressurized bays (cargo, hangars)
     LH₂ reaction mass tanks (distributed)
     ─────────────────────────────────────
     Crew / sensitive cargo core (pressurized)
     ─────────────────────────────────────
     LH₂ reaction mass tanks (distributed)
     Unpressurized bays (cargo, hangars)
     ─────────────────────────────────────
     H₂ ice coating
     Outer hull (CNT)
```

The LH₂ reaction mass tanks are the primary radiation shield — hydrogen
is the best radiation shielding material per unit mass. The tanks are
distributed around the crew compartment on all sides, not concentrated at
one end, because the ship must flip at the midpoint of every interstellar
transit. The ice coating handles ISM ablation. Together they protect the
crew regardless of ship orientation.

### Longitudinal Layout

```
[Stardrive nacelles] ← aft during acceleration
    [Radiator endcap (shadow side)]
    [Cargo bays / hangars]
    [LH₂ tanks]
    [Crew compartment / sensitive cargo]
    [LH₂ tanks]
    [Cargo bays / hangars]
    [Ship reactor / engineering]
    [Radiator endcap (shadow side during decel)]
[Conventional engine nozzles] → fore during acceleration
```

The stardrive modules are housed in nacelles offset from the main hull,
connected by structural booms. This isolates stardrive waste heat and
radiation from the ship proper. Each nacelle contains its own thermal
management (see Thermal Management below).

During cruise acceleration, the stardrive is aft. The ISM hits the fore
endcap. At the midpoint, the ship flips: the stardrive now faces the ISM.
The scoop collects incoming hydrogen while the engine exhaust blasts
forward through the scoop field — the exhaust velocity far exceeds the
ISM approach speed, so the scoop doesn't recollect it.


## Ablative Shield and Radiation Protection

### The Problem

At 0.5c, each ISM hydrogen atom carries ~117 MeV of kinetic energy —
comparable to a cosmic ray proton. The raw particle flux on the forward
face is about 2.8 kW/m². Unshielded, this would cook the crew in hours.

### The Solution (three layers)

1. **Magnetic deflection**: The bussard scoop's magnetic field extends
   thousands of kilometers ahead of the ship, deflecting ~95% of ionized
   ISM particles before they reach the hull. Most of the ISM is ionized.

2. **H₂ ice coating**: Solid hydrogen ice, reinforced with CNT mesh,
   coats the entire hull. Not water ice — there's no oxygen in
   interstellar space. Deep space is ~3K, well below hydrogen's 14K
   freezing point. The ice handles the ~5% of ISM particles that get
   through the magnetic field (~140 W/m² on the exposed endcap), ablating
   and carrying energy away. During cruise, the ramscoop collects more
   hydrogen than the shield loses to ablation — the coating is maintained
   indefinitely.

3. **Distributed LH₂ tanks**: 100,000 tonnes of liquid hydrogen reaction
   mass in tanks surrounding the crew compartment on all sides. This is
   the primary radiation barrier. Hydrogen's low atomic mass makes it the
   best neutron moderator and cosmic ray shield per unit mass.

### Ice Shield Details

- **Coverage**: Entire hull — both endcaps (30 cm) and barrel (10 cm)
- **Mass**: ~31,000 tonnes
- **Endcap erosion at 0.5c**: ~85 kg/s (after magnetic deflection)
- **Ramscoop collection at 0.5c**: ~155 kg/s
- **Net**: collection exceeds erosion — shield is continuously replenished


## Mass Budget

### Empty Mass (~150,000 tonnes)

The ship with nothing in it — hull, structure, permanently installed
systems:

| Component | Mass |
|-----------|------|
| Hull (CNT, 1mm) | 3,500 t |
| Internal structure (decks, bulkheads, framing) | 20,000 t |
| Ship reactor + direct conversion (~500 GW fusion) | 25,000 t |
| Thermal management (tin loops, endcap radiators) | 8,000 t |
| Drive reactors + conventional engines (~25 TW) | 20,000 t |
| Stardrive nacelles (black-boxed, incl. structure) | 50,000 t |
| Weapons systems | 10,000 t |
| Sensors, communications, computing | 5,000 t |
| Life support (sized for ~10 crew) | 500 t |
| Hangar structures and launch systems | 3,000 t |
| Antimatter containment systems | 5,000 t |
| Miscellaneous (wiring, plumbing, fittings) | 3,000 t |

### Operating Mass (~310,000 tonnes)

Ready to fly — fueled, shielded, crewed, but no cargo:

| Component | Mass |
|-----------|------|
| Empty mass | 150,000 t |
| H₂ ice shield (full hull) | 31,000 t |
| Reaction mass (LH₂, distributed tanks) | 100,000 t |
| Antimatter | 100 t |
| D-³He reactor fuel | 2,000 t |
| Atmosphere (crew areas only) | 500 t |
| Water and consumables | 100 t |
| Small craft (shuttles, fighters, transports) | 5,000 t |
| Crew (~10 people) | ~1 t |

### Loaded Mass (up to ~500,000 tonnes)

Operating mass plus cargo. **Cargo capacity: ~190,000 tonnes.**

Cargo includes: cold sleep pods with passengers, black-boxed technology,
trade goods, raw materials, colonization equipment — whatever the mission
requires. Actual loaded mass varies:

- Trade run: ~400,000 t
- Military patrol: ~300,000 t
- Colony expedition: ~500,000 t (maximum)


## Power: The Fusion Reactor

The ship's power plant is a D-³He (deuterium-helium-3) fusion reactor
producing ~500 GW of fusion power. This is what we simulate in-game.

D-³He fusion is nearly aneutronic — the primary reaction products are
charged particles (14.7 MeV protons + 3.6 MeV alphas). ~90% of fusion
energy emerges as charged particles; the remaining ~10% is neutron
energy from unavoidable D-D side reactions.

The charged particle fraction is directly converted to electricity via
magnetic decelerators — no thermal cycle, no working fluid, no turbines.
At ~70% conversion efficiency, the reactor delivers **~315 GW
electrical** from ~450 GW of charged particle energy. The ~50 GW neutron
fraction and ~135 GW of direct conversion losses are rejected as waste
heat (see Thermal Management below).

The reactor powers:
- Life support and environmental systems
- Fabrication and repair equipment
- Weapons systems
- Sensors and communications
- Computing (AI systems)
- Antimatter magnetic containment (critical — if this fails, bad things happen)

The reactor is **not** involved in propulsion. The conventional engines
have their own dedicated drive reactors (~25 TW total). The stardrive is
an entirely separate black-boxed system.

315 GW is enormously overprovisioned for 10 crew — modern human
civilization uses ~18 TW for 8 billion people. The surplus covers weapons,
fabrication, and the ever-present need for antimatter containment power.


## Thermal Management

### The Problem

500 GW of fusion power produces ~185 GW of waste heat that must be
rejected to space. There is no convection, no conduction to the
environment — radiation is the only option.

### Why Not a Thermal Cycle?

A conventional power plant would run the thermal energy through a Brayton
or Rankine cycle to extract electricity, then radiate the low-temperature
waste heat. D-³He doesn't need this — 90% of fusion energy is already in
charged particles handled by direct conversion. The remaining 10%
(~50 GW) is neutron energy deposited in the reactor blanket.

To extract work from that 50 GW thermally, you'd need a heat engine
operating between the blanket temperature and the radiator temperature.
A Brayton cycle between 2500 K and 2000 K achieves only ~20% efficiency
— about 10 GW electrical. Not worth the mass and complexity when direct
conversion already provides 315 GW.

A heat pump to boost low-temperature waste heat to high-temperature
radiators is even worse: the Carnot COP from 800 K to 2000 K is 1.67,
and real systems achieve maybe 60% of Carnot. You'd spend nearly as much
electrical power pumping as heat you're trying to reject.

The answer is simpler: design the reactor to produce waste heat at
radiator temperature from the start.

### Reactor Blanket: Liquid Tin at 1800-2000 K

The reactor's first wall and neutron blanket use high-temperature
ceramics (SiC composites) cooled by liquid tin (Sn). Tin is liquid from
505 K to 2875 K — an enormous operating range with comfortable margins
at the design point of ~1800 K.

The tin loop absorbs neutron energy in the blanket and direct conversion
losses in the collector structures, carries the combined ~185 GW to the
endcap radiator panels, and returns. No thermal cycle extracts work from
this loop — the heat is dumped directly.

### Endcap Radiators

The ship radiates waste heat from its endcaps — flat surfaces of
283,000 m² each. At 1800 K, one endcap radiates:

> P = ε × σ × A × T⁴ = 0.9 × 5.67×10⁻⁸ × 283,000 × 1800⁴ ≈ **152 GW**

At 2000 K: **231 GW** per endcap. One endcap handles the full 185 GW
waste heat load with margin. Both endcaps together (in-system) provide
~460 GW of rejection capacity.

**Shadow placement**: During acceleration, the aft endcap faces empty
space and radiates freely. The fore endcap faces the ISM and is coated
in H₂ ice for ablative shielding. At the midpoint flip, roles swap (see
Flip Thermal Sequence below).

During in-system operations (no ISM flux), both endcaps radiate
simultaneously — matching the higher power demands of combat,
fabrication, and weapons.

No deployable mechanisms. No moving parts.

### Flip Thermal Sequence

The midpoint flip swaps which endcap radiates and which carries ice.
The outgoing radiator is at ~1800 K; H₂ ice freezes at 14 K. Bridging
that gap requires a multi-day sequenced procedure:

**Phase 1 — Radiative cooldown (~3-4 days before flip)**

The reactor throttles to minimum power (a few GW — enough for
antimatter containment). Tin coolant is drained from the aft radiator
endcap. With no heat input, the endcap cools radiatively into the
shadow of the ship. Radiative cooling follows T⁴ — the first 1700 K
sheds in hours, the last hundred takes days:

| Target temp | Time from 1800 K |
|-------------|------------------|
| 500 K | ~2 hours |
| 100 K | ~10 hours |
| 50 K | ~3.5 days |

During this phase, the ship's thermal mass absorbs the reactor's
minimal waste heat (a few hundred MW — negligible temperature rise
across 300,000 tonnes of ship). The stardrive runs independently.

**Phase 2 — Physical flip (minutes)**

RCS thrusters or reaction wheels rotate the ship 180°.

**Phase 3 — New radiator online (~1 hour after flip)**

The old fore endcap (now aft, still coated in H₂ ice) becomes the new
radiator. Warm tin coolant is routed through it, sublimating ~7,300
tonnes of ice. At ~449 kJ/kg sublimation energy, this takes ~3.3 TJ —
about 5 minutes at 10 GW of reactor waste heat. The hydrogen vents to
space. Once clear, the endcap is the active radiator and the reactor
ramps back to operating power.

**Phase 4 — Ice buildup (~1-2 days after flip)**

The old aft endcap (now fore, at ~50 K) is actively cooled to 14 K
with LH₂ spray from the ramscoop — about 15 minutes. Ice deposition
begins immediately: ~7,300 tonnes of H₂ ice for 30 cm of coverage,
fed from scoop surplus at the ship's current velocity. At practical
deposition rates, full coverage takes 1-2 days.

**Vulnerable window**: Between the flip and full ice coverage, the
forward endcap has no ablative shield. The scoop's magnetic field still
deflects ~95% of ISM radiation, and the distributed LH₂ tanks protect
the crew from the remainder. Radiation exposure is elevated but
survivable. An attack during this window would hit the ship at its most
vulnerable — reactor just coming back online, one face unshielded.

**Total flip sequence: ~5-6 days.** This is a planned, predictable
event at the midpoint of every interstellar transit. The ship AI begins
the procedure autonomously; the crew remains in cold sleep throughout.

### Stardrive Thermal Management

The stardrive nacelles manage their own waste heat independently of the
ship reactor:

- **Drive plume**: The vast majority of stardrive waste heat is dumped
  directly into the exhaust stream. The reaction mass is the coolant —
  heated and expelled.
- **Antimatter factory**: Integrated into the nacelles, powered by
  siphoned stardrive output. Thermodynamic losses go into the exhaust.
- **Nacelle radiators**: Residual heat that can't be dumped into the
  plume is radiated from nacelle-mounted panels, part of the black-boxed
  stardrive package.


## Conventional Propulsion

For in-system operations — maneuvering, combat, docking, and boosting to
stardrive catch speed — the ship uses antimatter-heated hydrogen.

A small quantity of antimatter (~50-100 tonnes) annihilates with an equal
mass of matter, releasing energy that heats the LH₂ reaction mass to
extreme temperatures. The superheated hydrogen is expelled through
magnetic nozzles at ~13,000 km/s (0.044c).

### Performance

| Loaded mass | Available Δv | 0.1G endurance |
|-------------|-------------|----------------|
| 500,000 t (max cargo) | ~2,900 km/s | ~30 days |
| 400,000 t | ~3,750 km/s | ~24 days |
| 300,000 t (light) | ~5,270 km/s | ~18 days |

Maximum acceleration: 0.1G. The ship feels heavy — maneuvers take
minutes to hours. In-system transits take days. This is deliberate: the
ship is a kilometer long and masses hundreds of thousands of tonnes. It
handles like a supertanker, not a fighter.

### Port Call Δv Budget

Every port call costs Δv:

| Phase | Δv cost |
|-------|---------|
| Final deceleration (stardrive hands off at ~900 km/s) | 900 km/s |
| In-system maneuver and docking | 200 km/s |
| Departure maneuver | 100 km/s |
| Boost back to stardrive catch speed (~900 km/s) | 900 km/s |
| Reserve | 200 km/s |
| **Total per port call** | **~2,300 km/s** |

At maximum cargo (500 kt, 2,900 km/s available), this leaves only 600
km/s of margin — enough for a clean port call, but combat or unexpected
maneuvering eats into it fast. At lighter loads the margin is comfortable.

**The trade tension**: Arrive loaded with cargo, sell it, depart light.
You have plenty of Δv to leave. But if you want to depart loaded — having
bought cargo to fill the hold — you either need to refuel at the station
or trust that your margin is enough. Fuel isn't free, and not every port
has antimatter for sale.


## The Stardrive

The stardrive is a black-boxed module built by Interstellar Systems. Its
internal mechanisms are proprietary, with multiple layers of tamper
protection preventing reverse engineering.

What is known: the stardrive implements a Ram-Augmented Interstellar
Rocket (RAIR). It uses a bussard ramscoop to collect interstellar hydrogen
and a proprietary process to fuse it — including plain protium via the
proton-proton chain, which conventional physics considers far too slow for
engineering use. The stardrive builders solved this. How they solved it is
the most closely guarded secret in human civilization.

### Why This Matters

Normal D-D or D-³He fusion can only burn deuterium or helium-3 — trace
elements in the interstellar medium (deuterium is ~20 parts per million of
ISM hydrogen). The stardrive can fuse **all** collected hydrogen, turning
the entire ISM into both fuel and reaction mass. This is the difference
between a ship limited to a few thousand km/s of Δv and one that can reach
99% of lightspeed.

### Acceleration Profile

The stardrive's thrust depends on how fast the ship is moving — faster
means more hydrogen collected per second. It catches weakly at ~0.003c
(900 km/s) and builds from there:

| Ship velocity | ISM collection | Accel (500 kt) | Accel (250 kt) |
|---------------|---------------|-----------------|-----------------|
| 0.003c (900 km/s) | 0.9 kg/s | 0.007G | 0.013G |
| 0.005c (1,500 km/s) | 1.5 kg/s | 0.011G | 0.022G |
| 0.01c (3,000 km/s) | 3.1 kg/s | 0.022G | 0.044G |
| 0.03c (9,000 km/s) | 9.3 kg/s | 0.067G | 0.13G |
| 0.1c (30,000 km/s) | 31 kg/s | 0.22G | 0.44G |
| 0.3c (90,000 km/s) | 93 kg/s | 0.67G | 1.3G |
| 0.5c (150,000 km/s) | 155 kg/s | 1.1G | 2.2G |
| 0.9c (270,000 km/s) | 279 kg/s | 2.0G | 4.0G |

The snowball effect is dramatic: sluggish at first, then surging. The
critical phase is the conventional boost from 0 to 900 km/s, where the
stardrive barely catches. Every km/s of conventional Δv spent on in-system
maneuvering is fuel you might need for that transition.

### Flip-and-Burn

At the midpoint of an interstellar transit, the ship flips to decelerate.
The stardrive — now facing the direction of travel — continues to collect
ISM hydrogen with its scoop field. The engine exhaust exits forward
through the scoop field; because the exhaust velocity far exceeds the ISM
approach speed, the scoop doesn't recollect it. The ship decelerates on
free ISM hydrogen all the way down to ~900 km/s, where the stardrive
hands off to conventional engines for final approach.

The flip itself is a ~5-6 day thermal management procedure — the
radiator endcap must cool to cryogenic temperatures before ice can form,
and the ice-covered endcap must be cleared for radiator duty. See the
Flip Thermal Sequence in Thermal Management for details.

### Maintenance

The stardrive requires servicing by Interstellar Systems every ~60
light-years of travel. The inspection and maintenance process is
opaque — IS tells you what needs doing and what it costs, never the
details of what they found. The cost varies unpredictably.

Skipping maintenance is possible but not advisable. Stardrives that
exceed their service interval have been known to fail in "energetic"
ways — a drive capable of pushing 500,000 tonnes to 0.99c does not
fail quietly. No captain pushes the interval twice.

IS maintains service facilities in a handful of systems across
inhabited space. This constrains route planning — your circuit must
pass within ~30 light-years of a facility, because you need to get
there and back before the interval expires. See
[Trade](trade.md#interstellar-systems-and-the-shape-of-civilization)
for the strategic implications.


## Cruise Operations

During interstellar cruise at ~0.5c, the ramscoop collects far more
hydrogen than the stardrive consumes for thrust. The surplus is diverted
to replenish ship stores:

- **Reaction mass (LH₂)**: Tanks refill in ~2-3 months. Ships arrive at
  every destination with full tanks.
- **H₂ ice shield**: Continuously replenished. Collection rate exceeds
  ablation rate — the shield is maintained indefinitely.
- **D-³He reactor fuel**: The scoop collects deuterium freely (~20 ppm
  of ISM hydrogen — far more than the reactor consumes). Helium-3 is
  scarcer: the scoop's magnetic field is optimized for protons, and He²⁺
  has half the charge-to-mass ratio, yielding only **~25 tonnes of ³He
  per year** at cruise speed. The reactor at full power burns ~27 t/year
  of ³He — so cruise collection barely covers full-power consumption.
  During cruise the reactor runs at reduced power (crew in cold sleep),
  accumulating a modest ³He surplus. This surplus is spent down during
  in-system operations. ³He is a port commodity — sourced primarily from
  gas giant atmosphere mining.
- **Antimatter**: An onboard particle accelerator, powered by the
  stardrive's multi-petawatt energy output, produces ~30-100 tonnes of
  antimatter per year. Over a multi-year transit, the antimatter supply
  is substantially replenished. (For reference, CERN produces nanograms
  per year today. The stardrive has ~10¹⁵ times more power available.)

The crew is in cold sleep during transit. Ship AI handles navigation,
shield maintenance, and resource management autonomously.


## The Bussard Collector

The ramscoop is a magnetic field extending ~14,000 km from the ship,
generated by superconducting coils in the stardrive nacelles. It ionizes
and funnels interstellar hydrogen into the stardrive's reaction chamber.

### Collection Rate

**ṁ = π × r² × ρ × v**

Where:
- r = scoop radius (~14,000 km = 1.4 × 10⁷ m)
- ρ = ISM density (~1 atom/cm³ = 1.67 × 10⁻²¹ kg/m³, galactic disk average)
- v = ship velocity

At 0.5c: π × (1.4×10⁷)² × 1.67×10⁻²¹ × 1.5×10⁸ ≈ 155 kg/s

ISM density varies by region. In the Local Bubble (near Sol): ~0.1
atoms/cm³. In denser regions near spiral arms: 5-50 atoms/cm³. This
affects stardrive performance — some routes are faster than others.

The scoop also serves double duty as the first layer of radiation
protection, deflecting ~95% of incoming ionized particles before they
reach the hull.


## Small Craft

The ship carries a complement of smaller vessels in unpressurized hangar
bays (~5,000 tonnes total):

- **Shuttles**: Surface-to-orbit capable, ~200-500 t each
- **Fighters**: In-system combat, autonomous or remote-piloted, ~50-100 t each
- **Transfer craft**: In-system cargo/passenger movement, ~500-2,000 t each

Details TBD.


## Open Questions

Future sections to be developed:

- **Weapons systems**: What does combat look like at these energy scales?
- **Defensive systems**: Beyond the ice shield and magnetic field
- **Sensors and communications**: Relativistic effects on sensing
- **Life support**: Cold sleep technology, crew rotation
- **Fabrication**: What can and can't be manufactured aboard ship
- **AI systems**: Ship automation, decision-making, the philosophical questions from NARRATIVE.md
