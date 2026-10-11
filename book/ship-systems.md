# Ship Systems — Engineering Reference

Every starship is a **courier**: an antimatter engine a kilometre
across, with two pods riding its ring. This is the reference for what is
aboard and how it works. Why the engine looks like this, and the physics
behind its numbers, is in [Courier — Engine Design
Notes](courier.md). The numbers come from the nozzle model
(`crates/hf-nozzle`) and `modeling/courier.py`; treat engine masses as
±2×.

There are no freighters. No engine that could push a megatonne survives
its own gamma rays, so nothing crosses between stars in bulk. What
crosses is light: information, minds, seeds, sleepers, and the rare
object (see [Trade](trade.md)).


## The Ship at a Glance

| | |
|---|---|
| Engine | antimatter; ~500 TW of annihilation at full power |
| Mirror ring | 1,070 m across, 800 m ahead of the reaction point |
| Pods | two, on the ring, each 5.7 m in radius and ~220 m long |
| Dry mass | ~7.0 kt, with 850 t of payload |
| Fueled for 10 ly | ~26.5 kt, 9.8 kt of it antimatter |
| Thrust | ~460 kN: ~2 mG at departure, ~7 mG on arrival |
| 10 ly | ~112 years, peak 0.18c |
| Crew | ~10, asleep for the crossing |

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


## Layout

### The Ring

The ring is the engine and the backbone. The thrust acts on it; the pods
are bolted to it. Around its 3.4 km circumference run:

- **The conductor**: ~25 MA of superconductor at ~900 K, ~1,000 t of
  magnet. Superconductors work to ~1000 K in this century, so nothing
  aboard is cryogenic.
- **The tiles**: 5 cm of helium-cooled tungsten on the face toward the
  reaction point, taking ~50 MW/m² of gammas and shadowing the conductor.
- **The helium loop**: ~7 t/s, closed and segmented, carrying the tile
  heat to the turbines.
- **The fins**: radiators for the turbines and the coils.
- **The frame and feeds**, with Whipple strips on the faces the medium
  meets.
- **The port heater**, on the ring well away from both pods (see Power).

### The Pods

Each pod sits on the ring on the line from the reaction point through
the conductor, its shield plate ~20 m beyond the conductor and facing the
reaction. Each is half a ship: a hab, a sleeper bay, half the payload,
half the propellant. Either can fly the ship.

From the plate outward, fitted for a 10 ly hop:

| Section | Length | |
|---|---|---|
| Shield plate | 10 cm | tungsten, helium-cooled; takes ~1.5 GW of gammas |
| Stop | ~7 m | dust that is never burned; takes the ~0.07% of each shower that gets through the plate |
| Crew section | ~35 m | sleeper bay, hab, payload bay, systems |
| Dust tank | ~42 m | matter propellant, under bladders |
| Antimatter core | ~118 m | anti-dust in a magnetic bottle, inside ~1 m of matter dust |
| Far end | ~20 m | a hot skin, structure |

The crew section is where it is for two reasons (see Shielding): no
charged particle from the engine lands within ~40 m of the plate, and
the ring's own field turns away cosmic rays best there.

### The Shadow

A kilometre from the reaction point, the gammas carry **~14 MW/m²** —
ten thousand times sunlight — onto anything that faces it. The only safe
places are the shadows of the plates and the tiles.

- **Everything a pod has lies inside its plate's shadow.**
- **Everything outside a shadow lies edge-on**: radiators, sensor faces,
  antennas. Each lies in a plane that contains the line to the reaction
  point, so its faces never see it.
- **Nobody goes outside while the engine burns.** A hand past the edge of
  the shadow is cooked.
- **Nobody crosses between the pods while it burns.** The only path is
  along the ring, and even behind the tiles ~5% of each shower leaks
  through. The pods are a kilometre apart and sealed from each other for
  the whole crossing, joined by power and data.
- **To go out on the ring you put the fire out.** The engine burns the
  whole crossing, so a repair is a coast, and the crossing runs late.

### Down

Thrust pushes the ring away from the reaction point, so "down" in a pod
is always toward its plate. That holds on both halves of the crossing,
because the ship flips. At 2–7 mG it barely matters: a dropped tool takes
five to ten seconds to fall a metre. The crew lives weightless and stows
everything.


## The Engine

### How It Works

Both pods fire grains of dust and anti-dust across ~1 km of vacuum to
the **annihilation point**, up to ~2.8 g/s of each in all — about 120
coarse grains a second of each, and every anti-grain is a kiloton.
Grains are neutral, so the field does not bend them.

A proton meeting an antiproton makes pions:

- **~36%** of the energy leaves as gamma rays, in every direction. That
  is the honest cost of antimatter: no field steers them.
- **~42%** leaves as neutrinos from the pions' and muons' decays.
- The rest is charged pions, then muons, then electrons, which the
  **mirror ring's** field turns into the exhaust.

Counted over all the propellant — matter and antimatter, gammas and
neutrinos included — the **effective exhaust is ~0.28c**, and the engine
makes **~0.93 N of thrust per GW** of annihilation.

The ring sits 800 m ahead of the reaction point and makes only ~5 mT
there, so nothing solid sits near the reaction. Two facts of the field
shape the whole ship:

- **Nothing may sit on the axis.** The field refocuses the reaction
  point onto the axis over and over; a plate there would be hammered.
- **The ring is a shelter.** No pion or muon from the reaction can come
  within ~330–380 m of the conductor. The coils see no charged hits, and
  the pods ride there; only some electrons from muons that decay far out
  find their way in (see Shielding).

### Throttle

Any power from a trickle up to ~500 TW of annihilation; thrust follows
at ~0.93 N per GW. Full power is ~460 kN.

### Steering and the Flip

Nothing moves but the aim. The pods aim their grain streams to meet
slightly off the axis; moving the reaction point sideways gives ~1 m of
torque arm per metre moved, ~0.5 MN·m per metre at full thrust.

Most of the ship's mass lies along the line through the pods, so turning
about that line is easy: **a flip in an hour on the main drive needs the
reaction point moved ~1–2 m.** Tilting the ring works too (1° gives
~11 m of arm) but needs a mechanism.

### Reaction Jets

Clusters on the ring and the pods burn dust as a **monopropellant**:
metal that reverts in a small chamber releases 216 MJ/kg by itself, good
for ~10–15 km/s of exhaust. They do attitude with the engine cold, fine
pointing, and proximity work in port.

- A flip in an hour with the engine cold needs ~0.5 kN at each side of
  the ring and ~0.3 t of dust.
- Translation is dear: 10 m/s on a 15 kt ship is ~12 t of dust.


## Propellant — Dust and Anti-Dust

### The Metal

Squeeze hydrogen past ~400 GPa and it becomes a metal. Theory has
predicted since the 1970s that it might *stay* metal when the pressure
comes off, the way diamond stays diamond. **Interstellar Systems can
make it**, and so far only IS. That is the setting's one gimme about
matter, stated plainly: metastable metallic hydrogen at zero pressure —
and its antimatter twin.

- **Density ~0.7 g/cm³** as a solid, ten times liquid hydrogen.
- **It is dust.** IS ships it as grains graded like sand, in three sizes
  — ~4 mm, ~0.6 mm, and ~80 µm fines — each filling the gaps of the size
  above. The mix flows like a liquid and packs to ~85%. Finer would pack
  worse and flow worse: micron powder is cohesive, arches, and throws
  sparks. Spacers call it **dust**, and the antimatter **anti-dust**,
  with some reverence and a lot of fear.
- **It superconducts**, well above the temperature it is kept at. A field
  pushes every grain away, so dust can be pumped by magnets and
  anti-dust can be held off every wall.
- **It reverts above ~1000 K**, releasing 216 MJ/kg — about 50× TNT.

The matter half could in principle be any solid; couriers burn IS metal
because it is dense, pumpable, and shields like hydrogen. The antimatter
half has no substitute. Keeping kilotonnes of antimatter is only
practical as anti-metal.

### The Antimatter Core

Anti-dust rides down the middle of each pod as a packed bed in a
magnetic bottle. The grains superconduct, so the field holds the bed off
every wall; it never touches matter. At milligees the bed weighs almost
nothing, and the field mostly holds it together and against the feed.

The bed must stay cold. Heat soaks out of it slowly — there is no gas in
an antimatter store to carry it — and an anti-grain that warms past
1000 K reverts to gas. Gas does not superconduct, so it falls out of the
field, finds a wall, and annihilates. That is why the core is wrapped in
~1 m of matter dust, which takes what the pod's skin lets through.

### Feed and Drain

A coil-lined pipe running a travelling field herds grains along with
each grain held off its neighbours and off the wall: nothing grinds,
nothing heats, nothing sparks. Reverse the field and the flow stops dead.
The valves are magnetic. At ~1.4 g/s of each per pod at full power, a
line holds grams, not tonnes, and the grains leave through injectors
beside the plate.

The tanks drain from the far end, matter and antimatter together, and
both pods draw evenly so the centre of mass stays on the axis. The
**stop** — the dust right behind each plate — is never burned.

### What It Costs to Carry

Dust is stable until it is heated or shocked:

| Unit | Metal | Reversion energy | TNT |
|------|-------|------------------|-----|
| One coarse grain (4 mm) | ~23 mg | ~5 kJ | a firecracker |
| A pinch | 1 g | ~216 kJ | ~50 g |
| A bucket | 10 kg | ~2 GJ | ~0.5 t |
| One pod's dust, 10 ly | ~4.9 kt | ~1×10¹⁵ J | ~250 kt |

Anti-dust is another order of thing. **One coarse anti-grain that meets
matter is ~1 kt.**

### The Bomb

A 10 ly load of ~9.8 kt of antimatter is ~1.8×10²⁴ J, **~420 Tt** —
several Chicxulubs. A 30 ly load is ~900 Tt. A courier that loses its
store gives anyone unshielded a lethal dose of gammas out to several
light-seconds, and a harmful one at thirty.

Ports keep couriers far out, and shuttles do the carrying. Everyone who
lives near one knows what it is: kilotonnes of antimatter, held off the
walls by a magnet.


## Power and Heat

### Heat

Almost all of the waste leaves for free: ~180 TW of gammas and the pion
and muon exhaust fly away, and the annihilation happens in vacuum with no
chamber walls. Only what the hardware intercepts has to be handled:

| Source | Heat | Temperature | Handled by |
|---|---|---|---|
| Engine tiles | ~54 GW | ~2,900 K | helium loop (~7 t/s) to turbines: ~16 GW of electricity, ~38 GW rejected through ~70,000 m² of fins at ~1,500 K |
| Leak through the tiles into the coils | ~3 GW | ~900 K | ~40,000 m² of coil fins |
| Shield plates | ~3.0 GW of gammas | ~2,900 K | a helium loop and ~3,000 m² of fins per pod |
| Electrons striking the pods | ~80 MW, both pods | ~700 K | the jackets, radiating from the pods' sides |
| Stops | ~1 MW per pod | cold | helium cooling in their first metre |
| Port heater (engine cold) | ~70 MW | ~1,000–1,500 K | the plant fins |
| Hab, avionics, stores | MW | ~300 K | small radiators |

**Fins are edge-on twice over.** Each fin lies in a plane through the
ship's axis, so it is edge-on to the reaction point and edge-on to the
medium the ship flies into.

**The tanks must stay cold.** Metal reverts near 1000 K. The plates and
stops take the gammas, the jackets take the electrons, and the tanks are
insulated from both.

### Power

The tile loop gives **~16 GW of electricity whenever the engine burns**
— which is the whole crossing. Hotel, sensors, and the FEL all fit with
room to spare.

**In port the same turbines run on a second heat source:** a small
**annihilation heater**. Anti-dust and dust meet inside a tungsten core
~0.9 m in radius, thick enough to stop the gammas (~20 radiation lengths)
and the pions (~11 cm). About 84% of the energy stays as heat; the rest
leaves as neutrinos. It is the one place on the ship where annihilation
is fully contained, and it can be, because it is small.

| | Value |
|---|---|
| Thermal power | ~100 MW (surface at ~10 MW/m²) |
| Electricity | ~30 MW |
| Core | ~40 t of tungsten |
| Antimatter | ~20 g a year |

Pion capture on tungsten makes neutrons and activates the core over
time, so it sits on the ring well away from both pods.

The 100 GW D-³He reactor of older designs powers stations and colonies
now, not ships. See [Station Power](station-power.md).

**The helium loops** are closed and segmented. A leak isolates one
sector of tiles, and the engine throttles down around it. Spare helium
rides in the payload. A courier that loses enough of it is throttled,
then coasting.


## Flight

### The Crossing

The engine burns the **whole crossing**: accelerate to the midpoint,
flip, brake the rest. Propellant is loaded for the hop:

| Hop | Time | Peak | Departure | Antimatter |
|---|---|---|---|---|
| α Cen (4.4 ly) | ~66 yr | ~0.14c | ~18.6 kt | ~5.8 kt |
| 10 ly | ~112 yr | ~0.18c | ~26.5 kt | ~9.8 kt |
| 20 ly | ~180 yr | ~0.23c | ~38.3 kt | ~15.7 kt |
| 30 ly | ~241 yr | ~0.26c | ~48.8 kt | ~20.9 kt |

The standard pods hold a 10 ly load; a 15 ly load needs them ~40 m
longer. A 20 or 30 ly load needs pods long enough to leave the ring's
shelter, so long hops are rare and specially fitted (see Open
Questions).

Coasting at a lower peak saves propellant and costs time. Relativity
is a footnote: a 10 ly crossing runs about half a year shorter aboard
than at home, a 30 ly one about two and a half. A century in flight is a
century at home. The crew buys time with **cold sleep**, not with γ. An
AI captain who stays awake pays the century in lived years. See
[Narrative](narrative.md) and [History](history.md).

### Payload Is Nearly Free

The engine and its propellant are most of the ship, so carrying more
costs little time:

| Payload | 10 ly | 30 ly |
|---|---|---|
| 100 t | ~108 yr | ~233 yr |
| **850 t** | **~112 yr** | **~241 yr** |
| 2,000 t | ~118 yr | ~251 yr |
| 5,000 t | ~132 yr | ~275 yr |

Speed is bought with antimatter, and antimatter is bought from IS.

### The Flip

At midcourse the ship rotates. Accelerating, the pods' far ends and the
ring lead into the medium; braking, the reaction point and the exhaust
lead, then the tile faces and the plates. The pods' long sides lean 34°
into the medium on both halves.

### The Local Bubble — the Map Is Inside It

Inhabited space is a 30-light-year sphere. The Local Bubble is on the
order of a thousand light-years across. **Every voyage the player can
fly is in thin air:** ~0.1 atoms/cm³, not the galactic-disk average of
~1. Route knowledge is local clouds and filaments — a factor of a few,
not three orders of magnitude.

Density does not touch speed or fuel; nothing aboard collects the
medium. Density sets **how hard the dust hits and how much FEL time you
spend.** A clean lane is kind to the ship. A dirty one is a gauntlet.

### The Forward Environment

At the 10 ly peak (0.18c) each ISM proton carries ~16 MeV; at the 30 ly
peak (0.26c), ~33 MeV. In the Bubble that is 14–43 W/m² on whatever
leads — thermally nothing. Unshielded, the *dose* would still be lethal,
but such protons stop in the first bumper of a Whipple.

Aberration and blueshift are mild: \(D \approx 1.2\)–\(1.3\), round-trip
\(D^2 \approx 1.4\)–\(1.7\). Threats are biased forward, not crammed into
a narrow cone.

The reaction budget, for a grain first pinged at light-distance \(T\), is
\(T\cdot(1-\beta)/(\beta(1+\beta))\): **3.9 seconds per light-second** of
ping range at 0.18c, 2.3 at 0.26c. Causality is not the bottleneck. **Ten
milliseconds of fire control** is. See Active Debris Defense.


## In-System and Ports

### In-System

On the main drive at full thrust, burning the whole way, between a ship
that has arrived (~7.0 kt, ~7 mG) and one fueled to leave (~26.5 kt,
~2 mG):

| Trip | Time | Δv | Antimatter |
|---|---|---|---|
| Earth–Moon distance | 1.8–3.4 days | 5–10 km/s | ~0.4–0.8 t |
| 1 AU | 35–68 days | 100–200 km/s | ~8–16 t |
| 5 AU | 78–152 days | 230–450 km/s | ~19–36 t |

### Ports

**A courier never docks.** At full power it is a 180 TW lamp of gamma
rays, in every direction; pointing the exhaust away does nothing for
those. Within ~10,000 km it gives an unshielded person a lethal dose in
about a day, and at a light-second a few grays a year. Couriers throttle
down near anything that breathes, park tens of light-seconds out on the
port heater, and shuttles carry the payload in.

### Combat

Everything happens at milligees. Fights are slow and positional: who saw
whom first, where each ship will be in a week, and how much antimatter
each side can spend. A courier's exhaust is a weapon at short range and
its store is a bomb at any range, so nobody closes on one lightly.


## Shielding and Sleepers

### Engine Gammas

Each pod's **plate** — 10 cm of tungsten, 30 radiation lengths — absorbs
~1.5 GW of gammas into its helium loop. About 0.07% of each shower gets
through, ~1 MW per pod. The **stop** behind it, ~7 m of dust that is
never burned, takes that to below the cosmic-ray dose. The propellant
cannot do the job: by arrival it is gone.

### Engine Electrons

None of the pions or muons from the reaction reach the pods. Some of
the **electrons** do: a muon that decays far from the axis gives its
electron a new orbit, one the ring's shelter does not forbid. In the
model they land on the pods' sides at ~80 MW for both pods, tens of MeV
apiece, **none within ~40 m of the plate**, rising toward the far ends.

So the crew section sits in the first ~40 m. The tank sections wear
cooled **jackets** that stop the electrons and radiate the heat from the
pods' sides, and the far ends are a hot skin. The matter dust around the
antimatter core takes what the jackets let through.

### Cosmic Rays

Deep space delivers on the order of a sievert a year in galactic cosmic
rays, from every direction, and between stars there is no stellar wind
to thin them. A 5.7 m pod has no room for metres of shielding all round.

The **ring's own field** does much of the work. Traced backward through
it in the model, cosmic rays reach a pod only along some directions:

| Distance from the plate | Proton dose that arrives | Helium-and-heavier dose that arrives |
|---|---|---|
| 0 m | ~11% | ~31% |
| 10 m | ~13% | ~38% |
| 40 m | ~30% | ~62% |
| 120 m | ~63% | ~88% |
| 220 m | ~92% | ~100% |

The shelter is best at the plate end, where the crew section already
is. Protons, which bend easily, are mostly turned away; heavy nuclei,
which are stiffer for the same energy, mostly are not. The ring is a
persistent superconducting loop and stays energized in port. If it
quenches, the shelter goes with it.

### The Sleeper Bay

The **sleeper bay** sits just behind the stop, with dust beyond it along
the pod. Two dozen or so sleepers per pod, ~5 t to a berth. In the bay the
ring passes about a seventh of the proton dose and two fifths of the
heavy-nucleus dose, and the heavy nuclei carry most of the harm; the
bay's walls and the dust around it take some of the rest. What is left
is, roughly, **a fifth of a sievert a year — some twenty sieverts a
crossing.** (First-order: the nuclei's share and the walls' effect are
estimates, not model runs.)

A frozen body repairs nothing. **Revival** means repairing radiation
damage, which is expensive, and not everyone comes back. So most of what
crosses is data, genomes, wombs, AI persons, and unique objects; colonies
are founded with genomes and artificial wombs, because a genome kept as
data takes no dose. Spacers still call a berth a **lead box**, from the
days when each box carried its own shielding — never lead, which turns a
cosmic ray into a shower and makes the dose worse.


## Active Debris Defense

### The Threat the Passive Shield Misses

The Whipple strips and jackets handle the *gas*: ISM protons at tens of
MeV stop in the first bumper. A **grain** with real mass is another
matter. The ISM is ~1% dust by mass: silicate and carbonaceous motes
from sub-micron up through the rare sand- and pebble-sized debris of
shattered worlds. At 0.18–0.26c (masses below are 1.6 g/cm³ cubes of the
stated size — order-of-magnitude):

| Grain | Mass | KE at 0.18c | KE at 0.26c | Equivalent | Handled by |
|-------|------|-------------|-------------|------------|------------|
| 1 µm mote | ~1.6 pg | ~2.4 J | ~5 J | a thrown pebble | Whipple |
| 10 µm dust | ~1.6 ng | ~2.4 kJ | ~5 kJ | a rifle round | Whipple |
| 100 µm fine sand | ~1.6 µg | ~2.4 MJ | ~5 MJ | 0.6–1.2 kg TNT | Whipple, marginally / **active defense** |
| 1 mm coarse sand | ~1.6 mg | ~2.4 GJ | ~5 GJ | 0.6–1.2 t TNT | **active defense** |
| 1 cm pebble | ~1.6 g | ~2.4 TJ | ~5 TJ | 0.6–1.2 kt TNT | **never let it arrive** |

Fine sand is a grenade the Whipple can be asked to eat; coarse sand is a
tactical munition; a pebble is a warhead.

### What Gets Hit

The ship is a kilometre-wide disc that is **mostly hole**. What can be
hit is the ring — conductor, tiles, frame, a few thousand m² — and the
pods, whose long sides lean 34° into the medium and show ~1,500 m² each.
The fins are edge-on and show almost nothing. A grain whose line crosses
the disc at random hits hardware well under one time in a hundred.

So the defense does not have to stop every grain ahead. It has to track
each one well enough to know whether its line crosses the ring or a pod,
and stop those. The rest pass through the middle and on past the
reaction point, which is a metre across.

### The Layered Defense

No single mechanism spans twelve orders of magnitude, so defense is
binned by energy:

1. **Whipple** strips on the ring and jackets on the pods: shatter and
   absorb the small grains.
2. **Active detection**: finds the grains the Whipple can't take, far
   enough out to act, and sorts the ones on hitting lines.
3. **Active interdiction**: vaporizes or deflects them at standoff.
4. **Navigational avoidance**: the rare large bodies, detected at long
   range and steered around.

### The Fire-Control Clock

Track, aim, and fire is **10 ms**. That is a machine number, not a
human one. Light still has to fly.

At 0.18c the grain closes at \(5.4 \times 10^7\,\mathrm{m/s}\). After
the computer is done, the FEL pulse still takes \(R/c\) to arrive. You
need the grain far enough out that it does not beat its own death:

> \(R_\mathrm{fire} \ge v \cdot 10\,\mathrm{ms} / (1 - \beta)\): **~660 km** at 0.18c, ~1,050 km at 0.26c

A radar ping is worse. The echo takes \(2R/(c+v)\); the grain is at
\(R(c-v)/(c+v)\) when you first hear it. That remaining range must still
clear the firing range:

> \(R_\mathrm{detect}\): **~950 km** at 0.18c, ~1,800 km at 0.26c

That is the **floor** — last-ditch, one target, no second shot.
Everything the suite is for lives *outside* that number.

### The Radar — and the Cooker

A 100 µm grain is a Rayleigh speck at centimetre waves. Cross-section
falls as \(a^6/\lambda^4\). The radar equation then goes as \(1/R^4\).
A 10 m dish at 100 GHz, looking for that speck at a light-second, wants
more watts than the ship has ever heard of. That search is dead.

What works:

- **Get out of Rayleigh.** THz / sub-mm (\(\lambda \sim 0.3\,\mathrm{mm}\))
  puts a 100 µm grain near \(ka \sim 1\). RCS climbs toward geometric
  (\(\sim 10^{-8}\,\mathrm{m}^2\)). Same 10 m aperture at 400 km is
  **megawatts**, not hundreds of gigawatts. This is the sand band.
- **Search for the bigger ones farther out.** Long-wave / mm radar flags
  1 mm–pebble at thousands to tens of thousands of kilometres.
- **Stare down a cone, not a sphere.** Threats are biased forward.
- **LIDAR, cued.** Once something exists as a track, a narrow optical
  beam pins the aimpoint the FEL needs. LIDAR does not search.
- **Several frequencies.** Absorption features and size resonances
  make grains dark in one band and loud in another.

And the cheat, which is also a weapon-shaped object:

**The cooker.** A high-power optical / near-IR illuminator — the same
class of director as the FEL, run as a flood or a cued spot. A 100 µm
silicate grain has a thermal mass of millijoules per thousand kelvin.
A gigawatt into a diffraction-limited spot at a thousand kilometres
warms it into the mid-IR in tens of milliseconds. It then *emits*,
one way. The passive array sees a warm mote the search radar never
could. In the near field the same beam would cook a chicken. Nobody
walks in front of the array.

Passive IR/optical is the other one-way channel. Often the first you
know of a 100 µm grain is that it got hot, not that it echoed.

Design ranges (10 ms fire control, order-of-magnitude):

| Job | Band | Grain | Detect (honest) | Notes |
|-----|------|-------|-----------------|-------|
| Course | long-wave | ≳ 1 m rock | 10⁴–10⁵ km | the only thing microwave sees at a light-second |
| Pebble search | cm / mm radar | 1 cm | **2,000–15,000 km** | see below |
| Pebble, emitting | passive IR / cooker | 1 cm | **3×10⁴–1.5×10⁵ km** | one-way; needs it warm |
| Shoot / walk | mm | ~1 mm | 2,000–20,000 km | enough clock |
| Shoot | THz / sub-mm | ~100 µm | 500–2,000 km | at or below the floor |
| Last ditch | any cue + LIDAR | ~100 µm | ≥ 950 km | floor at 0.18c |

**Fine sand is mostly a Whipple problem now.** The THz search reaches
about as far as the fire-control floor at 0.18c and not past it at
0.26c. The active suite works on millimetre grains and up; the strips and
jackets eat the sand.

### How Far Is a Centimetre?

A 1 cm pebble is not Rayleigh at centimetre waves. \(ka \sim 1\). RCS
is about geometric: \(\sigma \sim \pi a^2 \sim 8 \times 10^{-5}\,\mathrm{m}^2\)
— ten thousand times a THz 100 µm mote, not ten million. The radar
equation is still \(1/R^4\).

Worked monostatic, 10 m aperture, \(\lambda = 3\,\mathrm{cm}\),
\(P_t = 1\,\mathrm{MW}\), \(T_\mathrm{sys} = 50\,\mathrm{K}\),
10 kHz noise bandwidth, SNR 13 dB: **~600 km** — under the floor.

| Setup | Detect ~ |
|-------|----------|
| 1 MW, 10 m, short pulse | 600 km |
| 10 MW, 15 m, 1 kHz effective | ~3,000 km (55 ms of close at 0.18c) |
| 100 MW, 20 m, 0.1 s CPI | ~10,000–15,000 km (0.2–0.3 s) |
| 1 GW optical LIDAR, 1 m class, cued | ~5×10⁴–10⁵ km |
| Passive IR, pebble sunlit or cooker-warm, 10 m stare | ~10⁵ km, if it emits |

A light-second on a **centimetre** with microwave search is a lie.
\(R \propto (P A^2 \sigma)^{1/4}\). Going from 600 km to 300,000 km is a
factor of 500 in range, \(500^4 \approx 6 \times 10^{10}\) in the
numerator. That is a **rock**, tens of metres, not a pebble.

What you get at 3,000–15,000 km is time to **shoot**, not time to
**turn**. A 30 m sidestep at 2–7 mG takes 30–55 seconds, so dodging
needs the threat 5–10 light-seconds out at 0.18c. Only rocks are seen
that far. **A courier cannot dodge a pebble.** It shoots it, or eats it.
Vaporize at 10,000 km and the cloud has a fifth of a second to spread;
vaporize at 700 km and you have 13 ms of expansion — a ship-width of
plasma. Most of a spread cloud then passes through the hole in the
middle.

### Where the Suite Sits

On the pods. Each pod carries **a package on its flank**, edge-on to the
reaction point like everything else outside the shadow, looking ahead
about 56° off its face on either half of the crossing:

- A **10 m AESA** for cm / mm pebble search.
- A **THz head** on the same pallet.
- A **metre-class FEL director** (interdiction and cooker). The machine,
  a GW-class linac, rides inside the pod.

The two packages are ~1.1 km apart: a ranging baseline no single dish
ever had. Braking, the view ahead passes ~550 m beside the reaction
point and through the exhaust. The exhaust is too thin to stop radar or
light, but it is the brightest thing in that part of the sky.

### Interdiction — Spread It, Don't Match It

The interdictor is a **free-electron laser** — tunable to grain
and bow plasma, two independently steered directors a kilometre apart.
**It cannot cancel the grain's kinetic energy.** 2.4 GJ forward is still
2.4 GJ after vaporization. It can spread that energy into a cloud the
Whipple will eat, or, with time, ablate one face and walk the grain off
its line.

Vaporizing the grain is cheap. A 1.6 mg silicate (the 1 mm line) wants
tens of joules of heat. Coupling through the bow plasma is the real tax
— \(10^{-3}\) to \(10^{-4}\) is a sour day — so you budget **tens to
hundreds of kilojoules on the aperture** for a millimetre. The 10 ms
fire window then asks for **megawatts to tens of megawatts** on a single
easy target.

You still build a **gigawatt-class** machine, because you never get
one easy target. Dirty air is a salvo: many grains, poor coupling,
walk-not-just-pop. Average draw **~1 GW** off the 16 GW bus. Peak
**10–30 GW for 10 ms** from capacitors (100–300 MJ a shot).

A 1 cm pebble you shoot early or not at all. You *can* vaporize 1.6 g for
kilojoules-to-megajoules. Then you have 2.4 TJ of plasma that has not
had time to spread.

The array is, by temperament, **deeply anti-social.** As a sub-threshold ship
intelligence (see [Narrative Design](narrative.md#sub-threshold-ai-interactions))
it has settled into a personality defined almost entirely by ordnance: its sense
of a quarter well spent correlates directly with tonnage vaporized, and it does
not love long clean stretches of empty medium. The crew learns to read its mood
the way you'd read a bored predator.

### Where It Couples

The FEL and the cooker both drink from the 16 GW bus and the same
capacitor farm. A dirty passage is a power event and a thermal event:
generation losses land on the plant fins, and the cooker is a
forward-facing beacon. Every joule that leaves as beam does *not* land
inboard. Coverage is finite — you point THz and cooker down the cone you
can afford. The cheapest defense against a pebble is still to never fly
at it.

### Open Tensions

- **The last thousand kilometres.** Below the fire-control floor, the
  shot is late by construction. Passive stare and a standing kill zone
  eat some of it. The rest is Whipple.
- **Sand at 0.26c.** On long hops even fine sand outruns the suite. How
  much the strips and jackets eat in a dirty filament is the operational
  risk.
- **Cooker vs stealth.** Flooding the cone with optical gigawatts finds
  sand and announces you. A burning courier is announced anyway.


## Payload

### The Standard Container

Payload rides in a single standard box: **12 × 2.5 × 2.5 m**, about
75 m³, ~20 t loaded — a deliberate descendant of the twentieth-century
intermodal container, because a standard that already worked for four
hundred years isn't worth reinventing. A courier carries **~30 boxes**,
about fifteen to a pod, in the payload bay of each crew section.

### The Payload Manager and Its Boxes

Individually the containers are nearly dumb. Each has a small embedded
controller — about as clever as a hobbyist single-board computer — enough to
run a rail motor, report its telemetry, and hold power to whatever's inside.
The intelligence lives ship-side: one of the ship's **sub-threshold AIs is
dedicated full-time to the payload**, a manager that knows every box,
every sleeper berth, and every gram of what they hold. The crossing is a
century of weightless coast, and the manager has all of it to rearrange,
inspect, and tend. A ship can arrive with its boxes arranged differently
than it left, and no human touched them.

The rails carry **power and data** to every slot, so the manager keeps a live
inventory, and the crew can address any box, or the manager itself, from the
hab. A container that goes dark, or a manager that starts moving boxes for
reasons of its own, is an event.

### Balance

The two pods must stay balanced, or the centre of mass leaves the axis
and the ship turns under thrust. The loadmaster splits the payload
evenly; what is left over, the pellet aim trims — a hundred tonnes out
of balance moves the centre of mass ~2 m, about what a flip needs.

### Passengers

Some boxes carry **persons**. AI persons cross in hardened storage, as
payload (see [Trade — Passengers and Persons](trade.md#passengers-and-persons)).
Biological passengers cross in the **sleeper bay** (see Shielding and
Sleepers). For both, the rail power is **not optional**: it runs their life
support for the entire voyage. Losing power to a berth isn't a manifest
error; it's a body count. The bay is monitored the way a hospital ward is
monitored, and for the same reason.


## Mass Budget

| Part | Mass | Notes |
|---|---|---|
| Engine | ~3,700 t | mirror ring (~535 m radius, ~25 MA, ~900 K), ~1,000 t of magnet; ~1,100 t of 5 cm helium-cooled tungsten tiles; ~200 t of coil fins; ~700 t of helium loop and radiators; ~700 t of frame and feeds |
| Shield plates | ~450 t | two, 10 cm tungsten, 5.7 m radius, helium-cooled; ~1.5 GW of gammas each |
| Stops | ~860 t | ~7 m of dust behind each plate, never burned |
| Jackets | ~500 t | cooled skins on the tank sections and the far ends |
| Power plant | ~100 t | the turbines on the tile loop, and the ~40 t port heater |
| Structure, tanks, systems, habs | ~500 t | split between the pods |
| **Payload** | **~850 t** | ~30 containers and the sleeper bays |
| **Dry** | **~7.0 kt** | |
| Matter dust | ~9.8 kt (10 ly) | propellant |
| Antimatter | ~9.8 kt (10 ly) | propellant |
| **Departure** | **~26.5 kt** | |


## The Fuel Leash

IS sells dust and anti-dust at its **facilities**, where its stellar
collectors make the antimatter, and at **depots** it stocks with its own
couriers — more depots than facilities, fewer than inhabited systems.
**IS sets the price. IS chooses its buyers. IS decides which systems get
a depot.** It is the largest line in any courier's budget, and it is
paid in advance of a century.

Anyone with a shipyard can build a courier; the engine is textbook
physics. No one else can fill one.

A courier cannot carry its way home. Its tanks hold one hop; out and
back on a single 10 ly fill reaches only ~2.5 ly each way. **A system
without a depot is a one-way destination.** Ships go there to stay, or
to wait for IS. Pulling a depot does not blockade a system. It makes
every courier that arrives there permanent. See
[Trade](trade.md#interstellar-systems-and-the-shape-of-civilization).


## Small Craft

A courier never docks, so the shuttles belong to the ports: tugs and
transfer craft that meet a parked courier tens of light-seconds out and
carry its payload in. A courier may carry one small craft in its payload
for systems that have none. What drives them — dust monopropellant,
fusion, or contained antimatter heating a working fluid — is TBD.


## Cruise Operations

During the crossing the engine burns and the ship coasts on nothing. The
medium is something to survive, not something to eat.

- **Propellant**: all of it spoken for. A course correction is a change
  of aim, paid for out of the arrival margin.
- **Stores**: the payload manager watches the antimatter cores the way it
  watches the sleepers. A core running warm is an event.
- **The ring**: tile segments, helium segments, fins. A failed segment
  throttles the engine around it until someone can put the fire out and
  go out to it.
- **The flip**: at midcourse, an hour of turning on the main drive.

The crew is in cold sleep during transit. The ship's AIs handle
navigation, stores, and the engine autonomously. An AI captain who stays
awake lives the crossing.


## Open Questions

- **Injection.** Within about a metre of the reaction point, the gammas
  heat an incoming grain past 1000 K in about a millisecond, so grains
  revert and arrive as gas. Whether that makes a steady core, and how
  precisely two grain streams can be aimed across a kilometre.
- **Range.** A 20–30 ly load needs pods 300–470 m long, past the reach of
  the ring's shelter. Fatter pods, more pods, or a bigger ring.
- **Electrons on the pods.** ~80 MW in the model, from a few dozen hits in
  ten million tracks. Where exactly they land sets the jackets.
- **Shower leakage.** The plate and stop are sized from shower tails to
  first order; scattered gammas from the tiles are not counted yet.
- **Debris at 0.2–0.3c**, on a ring and two leaning pods.
- **Tile cooling.** 50 MW/m² of volumetric gamma heating carried off by
  ~7 t/s of helium in closed, segmented loops for centuries.
- **Weapons**: what does combat look like at milligees?
- **Life support**: cold sleep technology, crew rotation.
- **Fabrication**: what can and can't be manufactured aboard ship.
- **AI systems**: Ship systems are sub-threshold AIs — sophisticated
  intelligences below the Compact's statutory personhood threshold. They
  manage themselves, respond to commands, and occasionally exhibit behavior
  that doesn't quite fit their programming. The player's treatment of these
  systems is tracked invisibly and shapes the endgame. Over the course of
  the game, ship systems may begin approaching the threshold. See
  [Narrative Design](narrative.md#sub-threshold-ai-interactions) for the
  full mechanic and its implications.
