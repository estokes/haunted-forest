# Ship Systems — Engineering Reference

## Hull and Structure

The ship is a slender cylinder — **1 km** long and **40 m** in radius
(an 80 m diameter, a 12.5:1 needle). Facing the interstellar medium
head-on at a fraction of light speed, frontal area still matters, so
this is only a third fatter than a 30 m hull, not a sausage. It is
constructed from carbon nanotube composites and graphene sheets —
materials with tensile strengths of 50-100 GPa at densities of
~1,400 kg/m³.

The fuel takes **340 m** of full-section tank — two bands around the
midships habitat — and leaves **366 m** of hold. It is not liquid
hydrogen. It is **metallic hydrogen shot** from Interstellar Systems —
superconducting balls in three sizes that pour like a liquid and pack
eight and a half times as much hydrogen into a cubic metre: the bands
hold **~910,000 t** (see The Fuel). As LH₂ the same bands would hold
110,000 t, and that does not reach a star. The bays are fatter than
they were at 30 m, ten tracks instead of seven, and they carry more.
The face grew by 1.8×. Dust and FEL time scale with that.

The load-bearing structure is the **inner hull**, an **isogrid**: a thin
carbon skin with a triangular grid of ribs grown into its outer face, in
the gap under the Whipple. The bore stays smooth for the silo pistons and
the cargo rails. Thrust enters it at the pylon anchor frames (see The
Pylons) and runs forward as compression, ~43 GN at the root, tapering to
nothing at the nose. The isogrid carries that at ~0.12 m equivalent
thickness, ~22,000 t in all. A plain shell would need 0.30 m and
~57,000 t, because a shell is sized by buckling and runs at ~1% of the
material's strength; worse, a dent or a patch lowers its buckling load
sharply, and shell buckling is sudden. The grid fails locally: a cut rib
sheds its load to its neighbours in three directions, and the same grid
carries the twist when one engine cuts out. With the hold racks and the
silo bulkheads, the load-bearing structure is ~37,000 t, about 3% of a
fueled ship. The drive's 9 G redline is crew and nozzles; the frame is
built for the thrust, not the other way round.

Most of the interior volume is **unpressurized**: cargo bays, hangars,
engineering spaces, fuel tankage. Pressurized habitable space for the crew
of ~10 is a tiny fraction of the total — a few hundred cubic metres of
quarters, a bridge, a workshop. That volume holds about a tonne of air.
Reserve tanks and the life-support plant are separate (see Mass Budget).
The ship is essentially a massive automated cargo and exploration vessel
with a small human crew making decisions.


## Ship Architecture

### Cross-Section

The ship is built in concentric layers, designed to protect the crew and
sensitive cargo from radiation coming from any direction:

```
     Carbon-nanosheet Whipple bumpers (outer, multi-layer, dry)
     Vacuum gap: isogrid ribs, crawlspace
     Inner hull (isogrid skin, structural)
     ─────────────────────────────────────
     Unpressurized bays (cargo, hangars)
     Fuel bands (metallic-hydrogen shot, piston silos)
     ─────────────────────────────────────
     Crew / sensitive cargo core (pressurized)
     ─────────────────────────────────────
     Fuel bands (metallic-hydrogen shot, piston silos)
     Unpressurized bays (cargo, hangars)
     ─────────────────────────────────────
     Inner hull / gap / carbon Whipple bumpers
```

The barrel hull is a **dry Whipple shield**: spaced carbon-nanosheet
bumper layers over a vacuum gap. The carbon shrugs off the interstellar
gas and shatters micrometeoroids into clouds that spread across the gap
before they reach the inner hull. It is not a radiation shield, and it
does not try to be: almost nothing aboard needs one. The crew hab has
its own water jacket, and each sleeper rides in a shielded box (see
Passengers as Freight). The ship is **directionally specialized**: it
never flips (the drive reverses instead — see The Stardrive), so each
end does one job. The **nose** is a heavier, raked dry Whipple; it
always faces the ISM and is the single most-hit surface on the ship,
so it gets the best armor the design has, not a dual-role compromise. The **tail** is a pure radiator (see Thermal
Management) — it never meets the ISM, so it carries no shield duty at all.

### Longitudinal Layout

```
                  ↑ direction of travel — always

     [ Dry Whipple nose                                  ]
     [ Cargo bay / hangars                               ]
     [ Fuel bands (metallic hydrogen)                    ]
     [ Crew / sensitive cargo                            ]
     [ Fuel bands (metallic hydrogen)                    ]
     [ Cargo bay / hangars                               ]
     [ Ship reactor / engineering                        ]
     [ Aft radiating hull (8 zoned gores)                ]  ← always in the wake

  Stardrive nacelles ride guyed pylons at the hold/reactor bulkhead:
  one radial post each to a keystone on the axis, two cables each to
  hull frames fore and aft. The boxes fire either way by internal
  reconfiguration — nothing on the pylon moves. The stand-off keeps
  the plumes off the radiating skin.
```

The ship is **directionally specialized and never flips**: the nose
always meets the ISM, the tail always radiates into the wake, and the
hotel reactor sits aft beside the radiators. The stardrive modules —
IS black boxes — ride in nacelles so their waste heat and radiation
stay off the hull. The pylons do not slew. Each IS box fires
fore or aft by reconfiguring internally on command — hours, not
seconds; IS does not publish the procedure. A fight is a one-way
burn. Midcourse you have years.

### The Pylons — Posts and Cables

Each pod sits in a **cage** — three rings and four longerons around
the IS shell, ship hardware. The cage hangs on a **guyed pylon**:

- **One radial post** from the cage to a **keystone** on the ship's
  axis, at the bulkhead between the aft hold and the reactor. The two
  nacelles' posts meet head-on there, so their pushes cancel inside
  the ship. The post is in compression whichever way the boxes fire.
- **Two cables.** One runs from the cage aft-inward to a hull frame
  ~54 m behind the post, one forward-inward to a frame ~54 m ahead of
  it. Forward thrust loads the aft cable; reverse loads the fore cable.
  A cable only ever pulls.
- Each cable's line is aimed through the pod's thrust axis at the
  post, so the three forces meet at a point and the pod asks nothing
  of the cage in bending.
- At each anchor frame a **spoke** through the axis ties the two
  nacelles' cables together, so the hull ring never sees a radial
  point load. The forward spoke runs in a sector-boundary plane of the
  hold; the aft spokes stop at a hub ring around the reactor coils.
- Light splayed **stays** take lateral load.

Carbon nanotube is strongest in tension and heaviest in compression,
where buckling grows with length squared. The pylon puts the big loads
in cables and keeps the one compression member short and fat. At the
46 GN rating (23 GN per nacelle), with 10% cable pretension to keep
the idle cable taut:

| Member | Load | Size |
|--------|------|------|
| Cable (one loaded at a time) | ~29 GN tension | ~1.2 m, 84 m long |
| Post | ~15 GN compression, always | ~6 m tube, 80 m to the axis |
| Anchor spokes | ~15 GN tension | |

No member changes sign when the boxes reverse. Posts, cables, stays,
spokes, hub, keystone, cages, and fittings come to **~3,400 t**. Thrust
enters the hull skin at the two anchor frames. The post also carries
the shot lines and the FEL power out to the pods.

There is no second engine at the tail and nothing on
the bow but armor. To decelerate, the boxes are told to reverse and
the torch takes the speed off the way it put it on. The ship never
turns over. Nothing on the pylon moves.

Because the *ship* never turns over, **thrust-gravity reverses between
the burns**. On the departure burn the drive pushes the ship forward, so
"down" is toward the **tail**; when the boxes reverse for the braking
burn, "down" is toward the **nose**. Between the burns is the cruise:
decades of free fall. **The floor becomes the ceiling for the last
leg of the trip.** The crew rides interstellar
cruise in cold sleep, so no one walks the decks during the swap, and the
pods, tankage, and fluid systems are built to work under thrust from
either direction. But anyone awake — an in-flight emergency, a
mid-cruise wake-up, an AI captain who never slept — finds the ship's
usable "down" set by which burn the voyage last made, with a space
laid out for the outbound floor now hanging overhead on the way home.


## Cargo — The Hold

This is what the ship is *for*. Strip away the drive, the shield, the reactor,
and their only job is to carry the hold between stars. Two cargo bays — one
forward of the crew core, one aft — run about **180 m** and **185 m**. The
fuel bands take 315 m next to them (see Hull and Structure). The bays
are full-radius, and every tonne in them costs speed.

### The Standard Container

Cargo is fully containerized in a single standard box: **12 × 2.5 × 2.5 m**,
about 75 m³, ~20 t loaded — a deliberate descendant of the twentieth-century
intermodal container, because a standard that already worked for four hundred
years isn't worth reinventing. One size means one set of rails, one handling
system, one lock. The rack holds **9,568 containers** (~191,000 t) if every
slot is filled; the loadmaster leaves empty slots for trim.

### The Rack — a Hard Disk Stood on Its Axis

Gravity aboard runs *along* the long axis (the drive thrusts that way) and
**reverses for the braking burn** — see Longitudinal Layout. That single fact shapes
the whole rack. The cargo rails run fore-and-aft, parallel to gravity; each
container clamps to an **inner and an outer rail** and rides the gap between
them.

Those rails hang on a polar skeleton that, seen down the ship's axis, looks
like a hard disk: **radial support trusses** split each bay into eight
**sectors** (the spokes), **concentric arc rings** mark the boundary of each
**track**, and the rails run between the rings. Ten tracks nest from the core
out to the inner hull; because the outer tracks have more circumference they
hold more boxes — one or two per sector at the cramped inner rings, nine at the
outermost. The trusses bond directly to the central core and to the inner hull,
grown from the same carbon-nanotube stock and fused to both. It's all open
framework: a container slides straight along its rails and out through the open
rings, so no box ever has to be dug out from behind another.

### What Actually Holds It Up

Under a 9 g combat burn a 20 t container weighs **1.77 MN** — and there
are thousands. If each box hung from a rail that ran the whole bay,
every rail would carry a whole fore-and-aft column, buckling would
force them absurdly thick, and the rack would out-mass the hull. If
the boxes simply stacked onto the bay-end bulkhead, the bottom box
would have to be a structural column: twenty-four neighbours, 9 g,
and a 2.5 m face. The containers are cargo, not spars.

So the hold is a **semi-monocoque cylinder**. Each box clamps to a
pair of fore/aft rails (stringers) that are shear-tied to the
inner-hull skin at every frame. A box sheds only its *own* axial load
into the skin locally — no box carries the column, no rail carries
the column. The skin carries the accumulating axial load as membrane
compression, tapering thin→thick fore→aft, the way a rocket airframe
already does. A mutually-braced frame cage (radial struts, hoops, and
meridional X-braces in the sector-boundary planes) keeps the
stringers from wracking and takes the lateral and braking-flip
loads as axial spar forces. Gravity reverses for the braking burn; a clamp
holds both signs, so the flip just changes the sign of loads the
members already carry. The **inner-hull skin, not the bulkheads and
not the rails, is the primary structure of the hold.**

### Loading and Trim

Containers enter through a few small **bay doors** — the hull is never opened
along its length — and an internal handler runs them along the rails to any
slot. Where a box goes is a **center-of-gravity** decision: mass can be placed
at any sector (port/starboard) and any station (fore/aft), and the loadmaster
trims the ship by choosing. A lopsided load pulls the CG off the thrust axis
and the ship handles badly under burn — and the trim that balances the
*outbound* leg is wrong for the *return*, because down has flipped. Load heavy
and careless before a hard maneuver and the cargo itself can shift; a sudden CG
lurch mid-burn is how ships are lost.

### The Cargo Manager and Its Boxes

Individually the containers are nearly dumb. Each has a small embedded
controller — about as clever as a hobbyist single-board computer — enough to
run a rail motor, pulse a **cold-gas thruster** for fine repositioning in
free-fall, report its telemetry, and hold power to whatever's inside. The
intelligence lives ship-side: one of the ship's **sub-threshold AIs is
dedicated full-time to the hold**, a cargo manager that commands the whole swarm
at once. None of a container's motion is quick, but interstellar cruise is
*years* of weightless coast, and given that much time the manager can quietly
**reorganize the entire hold** — rebalancing trim, consolidating a delivery,
clearing a path to a box the crew wants. A ship can arrive with its cargo
arranged differently than it left, and no human touched it.

The rails carry **power and data** to every slot, so the manager keeps a live
inventory — every container's identity, telemetry, and internal state — and the
crew can address any box, or the manager itself, from the bridge. A container
that goes dark, or a manager that starts moving boxes for reasons of its own, is
an event.

### Passengers as Freight

Some containers carry **people**. Biological passengers cross in cold sleep, AI
persons in hardened storage (see [Trade — Passengers and Persons](trade.md));
either way they ride in the hold, as cargo, in a powered container. For the
cold sleepers the rail power is **not optional** — it runs their cryo life
support for the entire voyage. A freighter is therefore also a sleeper ship,
and the same rail that flags a misplaced pallet is keeping a hundred people
alive in the dark. Losing power to a passenger container isn't a manifest
error; it's a body count. The hold is monitored the way a hospital ward is
monitored, and for the same reason.

A sleeper box carries **one** sleeper. The pod sits in the middle of the
box inside ~0.75 m of water and polyethylene — ~15 t, most of the 20 t
the box is rated for. Spacers call them **lead boxes**. They are not lead:
lead turns a cosmic ray into a shower of secondaries and makes the dose
worse. Hydrogen stops them best.

It is still not enough. Deep space delivers on the order of a sievert a
year; the box halves it. A century and a half in the hold is tens of
sieverts, and a frozen body repairs nothing. The box only buys time.
**Revival** is what saves a sleeper — a long, expensive repair of a
lifetime of radiation damage — and not everyone comes back. A sleeper
costs a whole slot and a revival bill. Colonies are founded with genomes
and artificial wombs instead; a genome kept as data takes no dose.


## Speed and the Forward Environment

There is no speed wall. The torch is a rocket, and the ship flies at
whatever speed its fuel buys:

> \(\Delta v = v_e \ln(m_0 / m_f)\), with \(v_e \approx 0.12c\)

Catalysis fuses all of the hydrogen and the box puts nearly all of the
p-p mass defect (~0.007 \(mc^2\)) into the exhaust. A crossing is one
burn up, a coast, and one burn down, so the tank pays for cruise speed
twice. Going to **0.07c and stopping** takes a mass ratio of **3.2**;
0.10c takes 5.3, more than the bands hold with cargo aboard. That is why the ship is
mostly fuel when it leaves, and why every tonne of cargo is a speed
decision.

Time dilation is not in this bargain. \(\gamma(0.10c) \approx 1.005\).
Reaching 0.5c would cost \((\gamma - 1)Mc^2\): 65 years at full nozzle
power for a 470 kt hull, before you ask where the fuel fits. A century
in flight is a century at home. The crew buys time with **cold sleep**,
not with \(\gamma\). An AI captain who stays awake pays the century in
lived years. See [Narrative](narrative.md) and [History](history.md).

### Speed Is Bought With Cargo

Fuel lives only in the bands: ~910 kt of shot, loaded at port. Keep
**25 kt** in reserve for the arrival system, and the cargo load sets
the cruise:

| Cruise | Cargo | Fuel | Departs | Arrives | α Cen (4.4 ly) | 10 ly | 30 ly |
|--------|-------|------|---------|---------|----------------|-------|-------|
| 0.05c | 191 kt | 528 kt (bands part full) | 889 kt | 386 kt | ~90 yr | ~202 yr | ~602 yr |
| 0.06c | 191 kt | 689 kt (bands part full) | 1.05 Mt | 386 kt | ~75 yr | ~169 yr | ~502 yr |
| **~0.07c — freighter** | **191 kt** | **908 kt (bands full)** | **1.27 Mt** | **386 kt** | **~65 yr** | **~143 yr** | **~424 yr** |
| 0.08c | 120 kt | 908 kt | 1.20 Mt | 315 kt | ~58 yr | ~128 yr | ~378 yr |
| 0.09c | 58 kt | 908 kt | 1.14 Mt | 252 kt | ~52 yr | ~114 yr | ~336 yr |
| **~0.10c — empty hold** | **0** | **908 kt** | **1.08 Mt** | **195 kt** | **~46 yr** | **~101 yr** | **~296 yr** |

Times include both burns. A full hold cruises at **~0.07c**; that is a
freighter. Past that the bands are full and the only way faster is to
carry less: 0.08c is 120 kt of cargo, and an empty hold reaches
**0.10c** — couriers, warships, and anyone being chased.

### The Local Bubble — the Map Is Inside It

Inhabited space is a 30-light-year sphere. The Local Bubble is on the order of
a thousand light-years across. **Every voyage the player can fly is in thin
air:** ~0.1 atoms/cm³, not the galactic-disk average of ~1. Spiral-arm densities
(5–50 cm⁻³) are not on this map. Route knowledge is local clouds and filaments
— a factor of a few, not three orders of magnitude.

Density no longer touches speed or fuel. Nothing aboard collects the
medium. Density sets **how hard the dust hits and how much FEL time you
spend.** A clean lane is kind to the hull. A dirty one is a gauntlet.

### The Burn Profile

At the 3.2 PW nozzle rating and \(v_e = 0.12c\) the torch eats ~5 kg/s
and pushes ~180 MN. On a 1.27 Mt freighter that is **0.014 G**, rising
as the bands empty: **3.6 years** to 0.07c, covering 0.12 ly. Braking
is lighter work on a lighter ship: **2.0 years**, 0.08 ly. An empty
hold takes 4.0 years up and 1.7 down. Everything between is a coast.

Overdraw does not help here; 2× is good for minutes. Neither does a rich
mixture: every kilogram thrown below full burn is cruise speed lost.
Leaving Sol is years of a feather on the floor.

### The Forward Environment at Cruise

At 0.10c each ISM proton carries \((\gamma-1)mc^2 \approx 4.7\,\mathrm{MeV}\)
(use the relativistic figure; \(\tfrac12 mv^2\) is only 1% low here). Incident
power on the nose, disk-average:

> \(n\,c\,(mc^2)\,\gamma(\gamma-1)\beta \approx 23\,\mathrm{W/m}^2\)

About 116 kW over the ~5,000 m² nose. In the Bubble, 2.3 W/m² — a rounding
error next to sunlight. The hull does not have a heat problem. Unshielded, the
*dose* is still promptly lethal (MeV protons stop in the first millimetre of
tissue at tens of gray per second), so the Whipple is the radiation
shield that matters for anything alive behind the nose. Nothing magnetic
stands in front of the ship, and nothing needs to: 4.7 MeV protons stop
in the first bumper. The
figures here are for 0.10c, the fastest anything flies (an empty
hold); a 0.07c freighter sees half the energy per particle and per
grain.

Aberration and blueshift are mild: \(D = \sqrt{(1+\beta)/(1-\beta)} \approx 1.11\),
round-trip \(D^2 \approx 1.22\). Threats are still biased forward. They are not
crammed into an 8° cone. The ranging suite stays in the band it was built for.

The reaction budget, for a grain first pinged at light-distance \(T\), is
\(T\cdot(1-\beta)/(\beta(1+\beta))\) — **8.2 seconds per light-second** of
ping range. Causality is no longer the bottleneck. **Ten milliseconds of
fire-control** is: you need the grain at ≥333 km when you shoot, so a
radar detect at ≥407 km. 150,000 km of think-time is a pebble problem.
Sand is a THz-and-cooker problem. See Active Debris Defense.

### The Master Tension

The ship does not fly at the speed a captain wants. It flies at the
speed its loadout bought: every slot of cargo is a slot of fuel it is
not. A full hold is 0.07c. Emptying it buys 0.10c and a 10 ly crossing
forty years shorter. The lane matters for dust
and FEL time, not speed. Sleepers do not care how long the crossing is.
The cargo, the insurance clock, and anyone awake do.


## Hull Armor and Radiation Protection

### The Problem

At 0.10c cruise, each ISM hydrogen atom carries \((\gamma-1)mc^2 \approx
4.7\,\mathrm{MeV}\). The raw particle flux on the forward face is about
23 W/m² at disk-average density, ~2 W/m² in the Local Bubble — thermally
nothing. Unshielded, the *dose* is still promptly lethal: MeV protons stop
in the first millimetre of tissue. The ISM is also ~1% dust, and a grain
arrives as a hypervelocity impactor
(see Active Debris Defense). Galactic cosmic rays (GeV, roughly isotropic
at this \(\beta\)) are the penetrating radiation problem; the ram is not.

### The Solution

1. **Dry carbon-Whipple hull**: The outer hull is not a coating but a
   **Whipple shield** — spaced carbon-nanosheet bumper layers over a
   vacuum gap — doing two jobs at once:
   - *Gas erosion*: carbon is bound ~700× more tightly than the hydrogen
     ice it replaces (covalent ~7 eV/atom vs van der Waals ~0.01 eV), so
     the relativistic ISM gas barely touches it. A carbon hull outlasts
     the ship by orders of magnitude against the gas — no ablation, no
     cryo, no in-flight replenishment.
   - *Micrometeoroids*: at these speeds a grain vaporizes on the first
     bumper whatever is behind it. The layers and the spacing do the
     work: each bumper spreads the cloud further, and the gap lets it fan
     out before the next. Water behind the bumpers added mass, not much
     protection, and froze, boiled, and needed bladders. It is gone.

2. **Local radiation shielding**: cosmic rays are the penetrating
   problem, and only a few small spaces need protecting. The crew hab
   carries a **1 m water jacket** (~1,900 t), which doubles as a
   life-support reserve. Sleepers ride one to a shielded box. Nothing
   else aboard minds the dose.

### The Whipple Hull

- **Barrel**: spaced carbon-nanosheet bumpers over a dry gap, with the
  isogrid ribs of the inner hull standing in it.
- **Nose**: a heavier, raked dry Whipple, more layers on the cone.
  Because the ship never flips (see The Stardrive), the nose is *always*
  the leading face and the single most-hit surface on the ship, so it
  gets the best armor the design has rather than the compromise a
  dual-role endcap would force. It also sits inside the active-defense
  cone, which is trained forward, so it gets the most help shedding the
  medium grains.
- **Tail**: a pure radiator (see Thermal Management). It never meets the
  ISM, so it carries no shield duty — no bumper compromise, just a
  radiator built to radiate.
- **Erosion**: negligible against the ISM *gas*. Practical lifetime is set
  by the rare large grain active defense misses and slow dust
  micro-cratering — patched at drydock, not self-healed. A raked, conical
  leading cap spreads the flux and glances off dust (sloped-armor logic),
  buying more margin still.
- **Inspection and repair**: the gap under the bumpers is a crawlspace.
  Crew suit up and **crawl the hull** — internal EVA, in vacuum, among
  the isogrid ribs — to inspect bumper layers, splice a cut rib, and
  patch battle damage or micrometeoroid punctures from the inside.
- **No in-flight replenishment**: carbon is patched at service intervals.


## Active Debris Defense

### The Threat the Passive Shield Misses

The carbon-Whipple hull handles the *interstellar medium* —
individual atoms and ions, ~4.7 MeV apiece at 0.10c (see Hull Armor), and the hull's
bumper layers shatter the smallest grains. But a **grain** with real mass is another
matter. The ISM is ~1% dust by mass: silicate and carbonaceous
motes from sub-micron up through the rare sand- and pebble-sized debris of
shattered worlds. Above a few tens of micrometres no passive hull can absorb a concentrated impactor.
At 0.10c the kinetic energy is KE ≈ (γ−1)mc² with γ ≈ 1.005
(masses below are 1.6 g/cm³ cubes of the stated size — order-of-magnitude):

| Grain | Mass | KE at 0.10c | Equivalent | Handled by |
|-------|------|-------------|------------|------------|
| 1 µm mote | ~1.6 pg | ~0.7 J | a flicked pebble | Whipple hull |
| 10 µm dust | ~1.6 ng | ~0.7 kJ | a thrown rock | Whipple hull |
| 100 µm fine sand | ~1.6 µg | ~0.7 MJ | a hand grenade | Whipple hull, marginally / **active defense** |
| 1 mm coarse sand | ~1.6 mg | ~0.7 GJ | ~170 kg TNT | **active defense** / **avoidance** |
| 1 cm pebble | ~1.6 g | ~0.7 TJ | ~170 t TNT | **avoid at all costs** |

Nothing millimetre-class may strike the hull at cruise. Fine sand is a
grenade the Whipple can be asked to eat once; coarse sand is a tactical
munition; a pebble is a warhead. The ship cannot tank those — it must never
let them arrive intact.

### The Layered Bow

No single mechanism spans twelve orders of magnitude, so defense is binned by
energy:

1. **Whipple hull** (dry carbon bumpers): shatters and absorbs the small grains
   (≲ a few µm).
2. **Active detection**: finds the medium grains the hull can't take, far
   enough out to act.
3. **Active interdiction**: vaporizes or deflects them at standoff.
4. **Navigational avoidance**: the rare large bodies that can't be
   interdicted are detected at long range and routed around. You do not vaporize
   a pebble carrying kilotons.

### The Fire-Control Clock

Track, aim, and fire is **10 ms**. That is a machine number, not a
human one. Light still has to fly.

At 0.10c the grain closes at \(3.0 \times 10^7\,\mathrm{m/s}\). After
the computer is done, the FEL pulse still takes \(R/c\) to arrive. You
need the grain far enough out that it does not beat its own death:

> \(R_\mathrm{fire} / v \ge 10\,\mathrm{ms} + R_\mathrm{fire} / c\)

> \(R_\mathrm{fire} \ge v \cdot 10\,\mathrm{ms} / (1 - \beta) = 333\,\mathrm{km}\)

A radar ping is worse. The echo takes \(2R/(c+v)\); the grain is at
\(R \cdot (c-v)/(c+v) = 0.818\,R\) when you first hear it. That
remaining range must still clear 333 km:

> \(R_\mathrm{detect} \ge 407\,\mathrm{km}\)

That is the **floor** — last-ditch, one target, no side-kick, no
second shot. Everything the suite is for lives *outside* that number.
Doppler is mild (\(D \approx 1.11\), round-trip \(\approx 1.22\)).
Aberration is mild. The suite stays in-band.

150,000 km of detect-and-think was a 0.5c number. At 0.10c it is 4 s
of luxury, and you will not get it on a 100 µm grain with a search
radar. The floor is four hundred kilometres. The design goal is
thousands, on the grains that can actually return a photon.

### The Radar — and the Cooker

A 100 µm grain is a Rayleigh speck at centimetre waves. Cross-section
falls as \(a^6/\lambda^4\). The radar equation then goes as \(1/R^4\).
A 10 m dish at 100 GHz, looking for that speck at 150,000 km, wants
more watts than the ship has ever heard of. That search is dead. Do
not write it down as a requirement.

What works:

- **Get out of Rayleigh.** THz / sub-mm (\(\lambda \sim 0.3\,\mathrm{mm}\))
  puts a 100 µm grain near \(ka \sim 1\). RCS climbs toward geometric
  (\(\sim 10^{-8}\,\mathrm{m}^2\)). Same 10 m aperture, 400 km, is
  **megawatts**, not hundreds of gigawatts. This is the sand band.
- **Search for the bigger ones farther out.** A millimetre grain is a
  different problem. Long-wave / mm radar flags 1 mm–pebble at
  thousands to tens of thousands of kilometres — avoid vs shoot, with
  time to think. You do not THz-search the whole sky for motes.
- **Stare down a cone, not a sphere.** Threats are biased forward.
  Power and aperture go on two forward-looking faces on the
  *nacelles*, not a rotating air-search and not the nose.
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
walks in front of the array. The "radar range" is both a detection
envelope and a place you do not stand.

Passive IR/optical is the other one-way channel. Grains that cross the
cooker's flood flash *before* they arrive.
Often the first you know of a 100 µm grain is that it got hot, not
that it echoed.

Design ranges (0.10c, 10 ms fire-control, order-of-magnitude):

| Job | Band | Grain | Detect (honest) | Notes |
|-----|------|-------|-----------------|-------|
| Course | long-wave | ≳ 1 m rock | 10⁴–10⁵ km | the only thing microwave sees at a light-second |
| Pebble search | cm / mm radar | 1 cm | **2,000–15,000 km** | see below |
| Pebble, emitting | passive IR / cooker | 1 cm | **3×10⁴–1.5×10⁵ km** | one-way; needs it warm |
| Shoot / walk | mm | ~1 mm | 2,000–20,000 km | plenty of clock |
| Shoot | THz / sub-mm | ~100 µm | 500–2,000 km | search cone only |
| Last ditch | any cue + LIDAR | ~100 µm | ≥ 407 km | floor |

### How Far Is a Centimetre?

A 1 cm pebble is not Rayleigh at centimetre waves. \(ka \sim 1\). RCS
is about geometric: \(\sigma \sim \pi a^2 \sim 8 \times 10^{-5}\,\mathrm{m}^2\)
— ten thousand times a THz 100 µm mote, not ten million. The radar
equation is still \(1/R^4\).

Worked monostatic, 10 m aperture, \(\lambda = 3\,\mathrm{cm}\),
\(P_t = 1\,\mathrm{MW}\), \(T_\mathrm{sys} = 50\,\mathrm{K}\),
10 kHz noise bandwidth, SNR 13 dB: **~600 km**. That is already
past the fire-control floor (20 ms of close) and not a dodge.

Scale the things we actually have:

| Setup | Detect ~ |
|-------|----------|
| 1 MW, 10 m, short pulse | 600 km |
| 10 MW, 15 m, 1 kHz effective | ~3,000 km (100 ms of close) |
| 100 MW, 20 m, 0.1 s CPI | ~10,000–15,000 km (0.3–0.5 s) |
| 1 GW optical LIDAR, 1 m class, cued | ~5×10⁴–10⁵ km |
| Passive IR, pebble sunlit or cooker-warm, 10 m stare | ~10⁵ km, if it emits |

A light-second (300,000 km) on a **centimetre** with microwave search
is a lie. \(R \propto (P A^2 \sigma)^{1/4}\). To go from 600 km to
300,000 km is a factor of 500 in range, \(500^4 \approx 6 \times 10^{10}\)
in the numerator. That is a **rock**, tens of metres, not a pebble.
The long-wave array's job at a light-second is "is there an asteroid
in this cone," not "is there gravel."

What you *do* get at 3,000–15,000 km is time to **shoot**, not time
to **turn**. Lateral miss of 30 m at 0.1 G wants \(t \approx \sqrt{2s/a}
\approx 8\,\mathrm{s}\) → detect at ~240,000 km. Microwave will not
give you that on a centimetre. Passive IR or a cued optical might.
If they do not, the pebble is an interdiction problem with a longer
fuse: vaporize at 10,000 km and the cloud has a third of a second
to spread (kilometres, if the vapor is doing km/s). Vaporize at
600 km and you have 20 ms of expansion — a ship-width of plasma,
which is why the table said do not take that shot unless you have to.

So: yes, long-wave sees a centimetre **quite a ways** compared to
sand. Thousands of kilometres, not four hundred. Not a light-second.
Light-seconds are for things you steer around. Centimetres you
detect-and-shoot if the IR suite has not already given you a lane.

The undetectable close-in fraction is now the last few hundred
kilometres, not tens of thousands. Passive stare and a standing
kill-zone cover some of it. The rest is an accepted Whipple hit rate.

### Where the Suite Sits

Not on the nose. The nose is a stack of Whipple bumpers, and
structural CNT is a conductor. You cannot look through it. The
rake that glances dust points the cone's face mostly *out* (~69° off
the velocity vector), so a conformal AESA on the bumper cannot stare
dead ahead anyway. Flattening the tip would fix the angle and put a
10–20 m disk on the single most-hit surface on the ship. One array.
One hit.

The suite is **two packages, one on each nacelle shoulder**, ship
hardware on the pod shell, not inside the IS box:

- A **10 m AESA** looking +Z (cm / mm pebble search). Two independent
  10 m faces at 10 MW are ~2,400 km each on a centimetre.
- A **THz head** on the same pallet (sand cone).
- A **metre-class FEL director** (interdiction and cooker). The
  machine — a GW-class linac — rides the pod cage beside it. Only
  power crosses the post.
- A **shutter** that parks over the face when that box fires
  forward. Cruise is look. Reverse is lids.

From the node at r ≈ 91 m, a line to a grain thousands of kilometres
dead ahead clears the 40 m hull by ~50 m. The fire-control floor is
407 km; on-axis is not blind where it matters. Inside a few hundred
metres of the nose the hull occults — that volume is already
Whipple's. A 182 m baseline is a ranging gift a bow dish never had.

A nose hit does not take the suite. A nacelle hit takes one engine
and one eye; the other side still stares down the pipe. The nose
stays a dumb, wet, raked Whipple.

### Interdiction — Spread It, Don't Match It

The interdictor is a **free-electron laser** — tunable to grain
and bow plasma, two independently-steered directors on the nacelles.
**It cannot cancel the grain's kinetic energy.** 0.7 MJ forward is
still 0.7 MJ
after vaporization. It can spread that energy into a cloud the Whipple
will eat, or, with time, ablate one face and walk the grain off the
hull.

Vaporizing the grain is cheap. A 1.6 mg silicate (the 1 mm line) wants
tens of joules of heat, not 0.7 GJ. Coupling through the bow plasma is
the real tax — \(10^{-3}\) to \(10^{-4}\) is a sour day — so you budget
**tens to hundreds of kilojoules on the aperture** for a millimetre,
millijoules-to-kilojoules for 100 µm. The 10 ms fire window then asks
for **megawatts to tens of megawatts** on a single easy target.

You still build a **gigawatt-class** machine, because you never get
one easy target. Dirty air is a salvo: many grains, poor coupling,
walk-not-just-pop. Average draw **~1 GW** off
the 63 GW bus. Peak **10–30 GW for 10 ms** from capacitors (100–300 MJ
a shot). Charge time at 1 GW is a fraction of a second. The plant can
feed that. The tin loop sees the generation losses; the beam itself
leaves the ship.

A 1 cm pebble is still avoidance. You *can* vaporize 1.6 g for
kilojoules-to-megajoules. You then have 0.7 TJ of plasma that has not
had time to spread. Do not take that shot unless the alternative is
worse.

The array is, by temperament, **deeply anti-social.** As a sub-threshold ship
intelligence (see [Narrative Design](narrative.md#sub-threshold-ai-interactions))
it has settled into a personality defined almost entirely by ordnance: its sense
of a quarter well spent correlates directly with tonnage vaporized, and it does
not love long clean stretches of empty medium. The crew learns to read its mood
the way you'd read a bored predator.

### Where It Couples

The FEL and the cooker both drink from the 63 GW bus and the same
capacitor farm. A dirty passage is a power event and a thermal event:
generation losses land on the tin loop, and the cooker is a
forward-facing beacon as loud as a lit radiator zone. Every joule that
leaves as beam
does *not* land inboard. Coverage is finite — you point THz and cooker
down the cone you can afford. The cheapest defense against a pebble is
still to never fly at it.

### Open Tensions

- **The last 400 km.** Below the fire-control floor, the shot is late
  by construction. Passive stare and a standing kill-zone eat some of
  it. The rest is Whipple. How high that rate runs in a dirty filament
  is the operational risk.
- **The pebble you can't dodge.** At 0.1 G and a few thousand km/s of
  in-system Δv, a body spotted light-minutes out is a navigation
  problem, not a last-second one.
- **Cooker vs stealth.** Flooding the bow with optical gigawatts finds
  sand and announces you. Zoning the aft radiators does not hide a
  cooker.


## Mass Budget

### Empty Mass (~161,000 tonnes)

The ship with nothing in it — hull, structure, permanently installed
systems:

| Component | Mass |
|-----------|------|
| Hull armor (dry carbon-nanosheet Whipple, heavier on the nose) | 30,000 t |
| Inner hull (isogrid, ~0.12 m equivalent at the root) | 22,000 t |
| Hold racks (rails, frames, cage — both bays) | 10,000 t |
| Fuel silo piston bulkheads (compartments every 20 m) | 5,000 t |
| Ship reactor + direct conversion (~100 GW fusion) | 8,000 t |
| Thermal management (tin loops, headers) | 8,000 t |
| Stardrive modules (IS boxes, feeds) | 52,000 t |
| Drive pylons (posts, cables, cages, spokes) | 3,400 t |
| Weapons systems | 10,000 t |
| Sensors, communications, computing | 5,000 t |
| Life support (sized for ~10 crew) | 500 t |
| Hangar structures and launch systems | 3,000 t |
| Miscellaneous (wiring, plumbing, fittings) | 3,000 t |

The armor and structure lines come out of `modeling/ship.py`, sized for
the 46 GN mount rating.

### Operating Mass (~1,080,000 tonnes)

Ready to fly — bands full, crewed, hold empty:

| Component | Mass |
|-----------|------|
| Empty mass | 161,000 t |
| Crew hab water jacket (1 m) | 1,900 t |
| Metallic-hydrogen shot, fuel bands (full) | ~908,000 t |
| D-³He reactor fuel | 2,000 t |
| Atmosphere (hab air + reserves) | 20 t |
| Water and consumables | 100 t |
| Small craft (shuttles, fighters, transports) | 5,000 t |
| Crew (~10 people) | ~1 t |

### Loaded Mass (~0.9–1.3 Mt departing, ~0.20–0.39 Mt arriving)

Operating mass plus the hold: **9,568 containers, ~191,000 tonnes** of
cargo at most. The cargo load is the cruise speed — see Speed Is Bought
With Cargo.

| Loadout | Departs | Arrives | Cruise |
|---------|---------|---------|--------|
| Freighter, full hold | ~1.27 Mt | ~386 kt | ~0.07c |
| Fast freighter, 120 kt cargo | ~1.20 Mt | ~315 kt | 0.08c |
| Courier or warship, empty hold | ~1.08 Mt | ~195 kt | ~0.10c |
| Colony expedition, full hold, slow | ~0.89 Mt | ~386 kt | 0.05c |
| Out-and-back, no fuel at the far end | ~1.27 Mt | ~386 kt | ~0.036c each way |

The drive mounts are rated for **46 GN** — 9 G at 520 kt. A fueled ship
is limited by mass, not crew: **~3.7 G** at a freighter's departure, the
full 9 G only once the bands are low.

Fuel is one account, in the bands. A hull breach no longer spills cold
hydrogen. It opens a silo compartment, which is worse (see The Fuel).


## Power: The Fusion Reactor

The ship's power plant is a D-³He (deuterium-helium-3) fusion reactor
rated at **~100 GW** fusion. It is not the drive. The torch handles
propulsion and dumps almost all of its waste into the plume. This
plant is hotel, fabrication, sensors, and the FEL/weapons bus.

D-³He fusion is nearly aneutronic — the primary reaction products are
charged particles (14.7 MeV protons + 3.6 MeV alphas). ~90% of fusion
energy emerges as charged particles; the remaining ~10% is neutron
energy from unavoidable D-D side reactions.

The charged particle fraction is directly converted to electricity via
magnetic decelerators — no thermal cycle, no working fluid, no turbines.
At ~70% conversion efficiency, the reactor delivers **~63 GW
electrical** from ~90 GW of charged particle energy. The ~10 GW neutron
fraction and ~27 GW of direct conversion losses are rejected as waste
heat (**~37 GW** at rated; see Thermal Management).

That is enough. Hotel, plugs, tin pumps, and compute are a few GW.
The FEL is GW-class at 0.10c — a 1-second, 1 GW pulse
is a gigajoule, and a sand grain is ~0.7 GJ. Weapons charge
capacitors off the same bus. Fabrication can draw hard for a while.
A 500 GW plant was a leftover from when someone thought the drive
needed a hotel that large. It did not. Keep a capacitor farm if you
want a bright, short shout. Do not haul a power station to feed it.

The **light show is the point, in combat.** Even 37 GW of waste, if
you light every zone, is a beacon. Run cool and dark at cruise. Light
panels when you need the watts. Shut the ones that face a contact.

³He at rated is ~5.4 t/year. The 2,000 t tank is centuries at
redline, forever at cruise. Nothing aboard collects ³He; it is bought
at ports. The tank is insurance, not the weekly shop.


## Confinement and Throttle

### The Tandem Mirror

The reactor confines its plasma in a **tandem magnetic mirror**: a straight
magnetic bottle roughly **70 m** long sitting aft by the radiators, with the
field intensified at each end into a magnetic "choke" that reflects escaping
particles back toward the middle. The burning plasma is a thin column about
**3 m** in radius — some **2,000 m³** at full length — held at **9 T** in the
central cell, rising to ~27 T at the end chokes (mirror ratio ≈ 3).

D-³He forces this choice. Because the fuel is nearly aneutronic, its fusion
power density is low and bremsstrahlung is always trying to outrun it, so the
plasma must run at high pressure relative to its confining field — high **beta**
(β, the ratio of plasma pressure to magnetic pressure). The mirror is a
naturally high-β machine; at the 100 GW operating point it runs **β ≈ 0.57**,
against a soft limit of 0.75 and a hard (disruption) limit of 0.90. A tokamak,
capped near β ≈ 0.04, would be strangled by bremsstrahlung on this fuel.

A simple mirror leaks: particles moving too nearly along the axis fall into the
**loss cone** and stream out the ends. The tandem design plugs those ends
electrostatically — small, dense **plug cells** at each extremity, sustained by
continuous beam and microwave heating, raise an electric potential that repels
central-cell ions back inward. That plug drive is a permanent power overhead,
and it is what sets the reactor's low-power floor (below).

The leak is not wasted. Particles that do escape the chokes stream directly into
the endcap **direct converters** — this is the "magnetic decelerator" described
above. The confinement geometry and the power-conversion geometry are the same
hardware.

### Throttling: The Length Knob

A burning D-³He plasma is **bistable** — ignited-hot or cold-dead, with nothing
stable between. Try to throttle it down the obvious way, by cutting fuel and
thinning the plasma, and self-heating falls faster than losses: the plasma
cools, reactivity collapses, and it quenches. You cannot idle a fusion fire by
starving it.

The mirror throttles a different way — it changes **how much** plasma is
burning, not how hard. Energizing more or fewer of the central-cell coils
lengthens or shortens the burning column while leaving its density and
temperature untouched. Every meter that is lit is fully, happily ignited; the
reactor simply lights more or fewer meters.

> P_fusion ≈ 100 GW × (L_active / 70 m)

Because density and temperature never move, the plasma never approaches the
collapse cliff — there is just less of it. This is the throttle: **power
proportional to burning length, at constant plasma condition.** Fuel feed tracks
the length; the field is held.

The central-cell **field strength** is a second, slower knob. It does not set
power — it sets *margin*. Raising the field lowers beta (more distance to the
disruption limit) and improves confinement (energy confinement time scales as
the square of the field), at the cost of coil stress that rises as the square of
the field. Operators trim it for the regime, not for throttling.

### Operating Envelope

| Regime | Fusion power | Burning length | Character |
|--------|--------------|----------------|-----------|
| Redline | 100 GW | ~70 m (full) | Combat / peak FEL. β riding toward 0.75; every radiator zone lit, or fewer zones running hotter. |
| Self-sustaining | 10–100 GW (10–100%) | ~7–70 m | Everyday range. Ignited at constant density; overhead turns marginal below ~10 GW. |
| Aux-driven / lean | 3–10 GW | at the ~7 m geometric floor | Deep-cruise minimum. Density dropped, aux heating holds temperature. Delicate. |
| Shutdown | < ~3 GW | cold | Overhead exceeds output — cold-and-dark, cold-start to relight. |
| Overdraw | 100–140 GW | ~70 m (full) | Surge past rated: density pushed up, β climbing. Thermally loud. You are a lighthouse. |

The **top** is a hard wall with two independent guards: the plasma's own
stability (beta past 0.90 → flute/interchange disruption) and thermal rejection
(the radiators saturate; see Thermal Management). The **bottom** is soft and
economic rather than physical: the burning column shrinks but the plug drive,
cryogenics, and coolant pumps do not, so at a few percent power the fixed
overhead is a large fraction of output, and below ~3 GW it exceeds output
entirely. There is no point idling there — the reactor is shut down and later
cold-started. (There is also a hard geometric floor: the central cell cannot be
shorter than the plug and transition regions.)

Cruising lean is what makes a voyage possible. At a few percent power the reactor
sips a small fraction of its rated fuel, stretching the **2,000 t** D-³He load
across interstellar transit times. At the 100 GW redline the same tank is
**centuries** (~5.4 t/year of ³He).

The **risk lives in the transitions, not the setpoints.** Changing the burning
length re-energizes superconducting coil groups, moving large amounts of stored
magnetic energy — done too fast, that stresses the coils and can quench them.
And ramping *up* can out-run the thermal loop, which lags 10–30 s behind a power
change: push power faster than the tin and radiators can follow and the plasma
overheats before cooling catches up. A good engineer throttles gently.

### Reactor Failure Modes

- **Beta-limit disruption** — pressure driven past β ≈ 0.90 (too much fuel, or
  field lost) trips a flute/interchange instability; the plasma loses
  confinement and dumps.
- **Plug collapse** — loss of electrostatic-plug drive (battle damage, heater
  failure) drops the end barrier; central-cell ions pour out the loss cone, and
  the plasma drains, cools, and quenches. The most likely way to lose the
  reactor on a long cruise, when the plugs run for months.
- **Coil quench** — ramping coils too fast or pushing the field too high quenches
  a superconducting magnet; the stored field energy dumps as a bang and
  confinement is lost.
- **Thermal cascade** — a power ramp that outpaces the radiators saturates the
  tin loop and forces an automatic throttle or SCRAM (see Thermal Management).
- **Control drift** — the mirror leaks continuously and must be actively held;
  lose the (physical, destructible) control computers and nothing trims fuel,
  field, and plugs against the loss cone, so the reactor wanders off its
  operating point.


## Thermal Management

### The Problem

100 GW of fusion produces **~37 GW** of waste heat that must be rejected
to space. Radiation is the only option. The torch is not in this
budget — its waste leaves in the plume.

### Why Not a Thermal Cycle?

D-³He does not need one. 90% of the yield is already charged particles
in the direct converters. The leftover ~10 GW of neutron heat, run
through a Brayton between 2500 K and 2000 K, is ~20% — 2 GW. Not worth
the mass next to 63 GW already in hand.

A heat pump from a cold radiator up to a hot one spends roughly as
much work as heat moved. Design the loop to reject at radiator
temperature from the start. Tin's liquid range (505–2875 K) is why
that is easy.

### Reactor Blanket: Liquid Tin

SiC first wall and blanket, liquid tin coolant. The loop is **short**:
reactor, converters, and radiators all live aft. No kilometer of 1800 K
pipe through the hab. Pump power is a fraction of a GW at this scale.

The loop does not have a single design temperature. Cruise sits near
1300 K. Rated load is hotter. A zoned-down combat posture, half the
area shut, runs hotter still. Tin does not care.

### Aft Radiators — The Hull, Zoned

There is no wake disk. A petal fan behind the ship sits in the
torch plume and dies, and a 300 m cantilever at 9 g is a combat
target we never paid for. The **aft hull is the radiator.**

Only the engineering end radiates: the reactor barrel, the tail
barrel, and the tail face — about **63,000 m²** (ε ≈ 0.9). Nose,
holds, fuel bands, and habitat stay dry Whipple. You do not run
1800 K skin over metal — it reverts near 1000 K — or over a sleeper
container. The barrel gap on those stations is vacuum and MLI next
to the hot gores, as it is everywhere else.

Eight azimuthal gores, eight valves, matching the plant sim. Shut
the ones that face a contact. The remaining skin runs hotter, or
you throttle.

| Load | All zones lit | Character |
|------|----------------|-----------|
| Cruise waste ~8–12 GW | ~1260–1390 K | Glow in the wake. The nose is still cold. |
| Rated 37 GW | ~1850 K | A lantern from astern. Graphite/C-C skin. |
| Overdraw ~50 GW | ~2000 K | You are a lighthouse aft. |
| Rated, half the zones shut | ~2200 K | Soft limit. Throttle or open the dark gores. |

That is hotter than a 500,000 m² fan would have been. The fan
would also have been in the exhaust. T⁴ buys the watts back; tin
boils at 2875 K. Cruise is no longer dim — not from behind. From
ahead the ship is still a cold Whipple needle. Directional
specialization again: the face that never sees the ISM is allowed
to shine.

The torch plume is a separate, much louder announcement. Zoning
the hotel plant does not hide a 3 G burn. It hides a ship that is
*not* burning, or is burning away from you. The pylons stand the
nozzles ~50 m off the skin so a collimated plume misses the gores.
The pods sit at the hold/reactor bulkhead, so the aft plume runs the
whole radiating barrel; at combat divergence the gores nearest the
plumes valve off because the plume is already heating them.

No deployables. Zoning is valves. The radiating skin is the hull.
Its mass is already on the hull line; the thermal budget is the
tin loop and the headers.

### Midcourse Turnaround

The ship never flips, so there is no flip thermal sequence — the old
multi-day cool-the-radiator, rebuild-the-shield ordeal is simply gone. The
ship coasts. When it is time to brake, the boxes reconfigure to fire
forward — hours — and the torch burns the speed off (see The Stardrive). The tail
radiates throughout; the nose shields throughout. Nothing structural
on the ship itself moves, and there is no vulnerable window — the
armor is permanent, not a coating that has to be regrown.

The one thing the crew would feel, if any were awake, is that
thrust-gravity reverses: "down" points sternward on the accelerating half
and bow-ward on the braking burn, with decades of weightless coast
between. The floor becomes the ceiling. See Longitudinal Layout.

### Stardrive Thermal Management

The stardrive nacelles manage their own waste heat independently of the
ship reactor:

- **Drive plume**: The vast majority of stardrive waste heat is dumped
  directly into the exhaust stream. The reaction mass is the coolant —
  heated and expelled.
- **Nacelle radiators**: Residual heat that can't be dumped into the
  plume is radiated from nacelle-mounted panels, part of the black-boxed
  stardrive package.


## Propulsion

One vendor, two products. **The stardrive** is an Interstellar Systems
black box: feed it metal, a fusion torch comes out. Catalyzed p-p — the
secret — is inside. There is no ram, no pellet, no onboard igniter
farm. **The fuel** is IS's too: metastable metallic hydrogen shot that
nobody else can make. See The Fuel.

Nothing aboard collects anything. In-system and interstellar are the
same engine on a different budget, and the budget is the rocket
equation.

### Two Knobs, and Combat Lives Between Them

**Exhaust velocity (Isp).** Fuel and remass are the same fluid. Fuse all
of it and \(v_e\) is ~0.12c (lean, long, gentle). Fuse a little and throw
the rest unburned (rich, hard, short). Crossings run lean. A fight runs
rich.

**Power (overdraw).** The nozzles are rated for **3.2 PW** continuously.
You can run them hotter at rising risk: a **soft redline at 2× (6.4 PW)**,
sustainable for minutes, and a **hard limit at 3.5× (11 PW)**, good for
seconds. Past that the exhaust plasma overwhelms the nozzle field and
burns through the throat, or the coils quench. Overdraw buys thrust
*without* spending your range — you pay in nozzle life instead.

Acceleration is **a = 2·P / (m·v_e)**. At 3.2 PW, on a freighter that
has just braked into a system (~386 kt) with its 25 kt reserve:

| Mode | Exhaust vel | Accel | Reserve lasts | Δv from reserve |
|------|------------:|------:|--------------:|----------------:|
| Lean (full burn) | 36,000 km/s | 0.05 G | ~59 days | ~2,400 km/s |
| Cruise | 16,900 km/s | 0.1 G | ~13 days | ~1,100 km/s |
| Hard maneuver | 1,690 km/s | 1 G | ~3 hours | ~110 km/s |
| Combat | 560 km/s | 3 G | ~20 min | ~38 km/s |
| Violent | 340 km/s | 5 G | ~7 min | ~23 km/s |

Read the last column and wince: hard acceleration is **Δv-catastrophic**.
Empty the reserve at 5 G and you've changed velocity by 23 km/s; spend
it lean and the same mass buys 2,400. And the reserve is all there is.
At departure a fight spends cruise speed. On arrival it spends the
margin you meant to dock on. More reserve is less cruise speed.

### The Combat Redline

Maximum acceleration is **~9 G**, reached with overdraw and a rich mixture
together — and it is a *convergent* redline. Four independent systems hit
their limit at the same number:

- **Crew** — in immersion couches with pharmacological support (the same
  tech as the cold-sleep pods), the crew take ~9 G briefly, ~5 G sustained.
  Bare, it's ~5 G.
- **Nozzle** — the 3.5× overdraw burn-through / quench limit.
- **Reaction mass** — the reserve empties in minutes at these rates.
- **Feed** — the shot lines are sized for it, ~96 t/s (see Fuel
  Handling).

A fueled ship never gets there: the mounts' 46 GN is ~3.7 G on a
departing freighter. 9 G is an arrival-weight number.

Below 9 G you have margin on all four; at 9 G you have margin on none, and
battle damage lowers whichever gives first. Combat is a three-resource
economy — **nozzle life, Δv-efficiency, reaction mass** — traded live. The
ship handles like a supertanker that can throw one hard punch: it lunges at
several G, but each lunge spends stamina it needs to get home. The agile
work belongs to the fighters (100 t, ~5,000× lighter), which pull tens of G
on the same drive tech. The capital ship punches; the fighters knife-fight.

### Port Call — the Tank Is the Voyage

The bands hold one crossing. You cannot top them up from a comet or a
gas giant: hydrogen is everywhere, metal is not. Refuelling is an IS
transaction at an IS depot — an IS tanker tops up the bands, and you
pay what IS asks (see The Fuel).

A clean port call — approach, dock, take on shot, depart — barely touches
the reserve. A fight can empty it.

**The trade tension** is what goes in each slot, and whether the next
system stocks metal. Arrive at a depot and you fill. Arrive somewhere
without one and you had better have carried your way home — out and
back on one fill is ~0.036c with a full hold, a slow boat both ways.
Deuterium and ³He are reactor stories. Metal is the drive story.


## The Stardrive

The stardrive is a black-boxed module built by Interstellar Systems.
Its internals are proprietary, with multiple layers of tamper
protection. IS sells the box, and IS sells what goes in it.

What is known: you connect a shot line. The box produces
a fusion torch. The process fuses plain protium via the proton-proton
chain, which conventional physics considers far too slow for
engineering use. How they solved the rate is the most closely guarded
secret in human civilization. How they keep the fuel metallic is the
second.

That is the whole machine. It is not a ramjet. Insert metal, extract
thrust.

### Why This Matters

Normal D-D or D-³He fusion can only burn deuterium or helium-3 —
trace isotopes. The torch fuses **protium**, so fuel and remass are one
fluid and the whole p-p yield goes into the exhaust: \(v_e \approx
0.12c\) at full burn. That is what makes the stars reachable at all.

What makes them reachable **with cargo** is density. The 3.2 mass
ratio a full-hold freighter needs for 0.07c-and-stop is ~880 kt of
hydrogen. As LH₂ that is twelve million cubic metres, two and a half
times the whole hull. As shot it fits in the bands.

The secret is the *rate* and the *storage*. Catalysis does not increase
the yield; the energy per nucleon is still ~6.7 MeV. There is no wall
and no free lunch: \(\Delta v = v_e \ln(m_0/m_f)\), and the logarithm
is the whole economy.

### Deceleration — Reverse and Burn

Months before the braking burn, the boxes are told to reverse —
internally, on IS's schedule, hours — and the torch takes the speed off
the way it put it on. The ship is lighter by then, so the burn is
shorter: ~2.0 years for a freighter from 0.07c. The ship never turns
over. The nose stays a shield; the tail stays a radiator. The pylons
do not move.

A box that will not reverse is the old nightmare: half the brake. The
burn doubles and has to start earlier; find out late and you overshoot
the star. Two boxes that will not reverse and you cannot stop at all.
There is no sail. See Maintenance.

### Maintenance

The **box** requires servicing by Interstellar Systems every ~60
light-years of travel. The inspection is
opaque — IS tells you what needs doing and what it costs, never the
details of what they found. The cost varies unpredictably.

Skipping maintenance is possible but not advisable. Stardrives that
exceed their service interval have been known to fail in "energetic"
ways — a drive that throws a million tonnes at seven percent of \(c\)
does not fail quietly. No captain pushes the interval twice. Sixty light-years is
six to nine centuries of flight; the interval is a fluence and wear
budget, not a calendar.

IS maintains service facilities in a handful of systems across
inhabited space. This constrains route planning — your circuit must
pass within ~30 light-years of a facility, because you need to get
there and back before the interval expires. See
[Trade](trade.md#interstellar-systems-and-the-shape-of-civilization)
for the strategic implications.


## Cruise Operations

During interstellar cruise the ship coasts. Nothing is collected; the
medium is something to survive, not something to eat.

- **Fuel**: the bands, all of it spoken for — the braking burn and the
  reserve. Nothing aboard makes more. A course correction is paid
  for out of the arrival margin.
- **Silos**: the cargo manager watches every compartment the way it
  watches the sleepers. A compartment running warm is an event (see The
  Fuel).
- **Hull armor**: needs no in-flight replenishment — the carbon Whipple
  hull doesn't ablate.
- **D-³He reactor fuel**: no skim. The 2,000 t tank is centuries at
  redline, so a lean cruise barely touches it. **³He is a port
  commodity** — gas-giant atmosphere mining. Deuterium is bought the
  same way.

The crew is in cold sleep during transit. Ship AI handles navigation,
silo monitoring, and resource management autonomously. An AI captain
who stays awake lives the crossing.


## The Fuel — Metallic Hydrogen

Squeeze hydrogen past ~400 GPa and it becomes a metal. Theory has
predicted since the 1970s that it might *stay* metal when the pressure
comes off, the way diamond stays diamond. Nobody in our century showed
it; the best experiments watched it revert as the load came off.
**Interstellar Systems can.** That is the setting's second gimme, stated
plainly: metastable metallic hydrogen at zero pressure, and only IS knows
how to make it. Once made it stays metal on its own, the way diamond
does. The secret is the making, not the keeping.

### What It Is

- **Density ~0.7 g/cm³** as a solid, ten times liquid hydrogen.
- **It is shot.** IS ships it as superconducting balls in three sizes:
  ~25 mm, ~3.5 mm, and ~0.5 mm fines, each filling the gaps of the size
  above. The mix flows like a liquid and packs to ~85% under
  compression. The smallest grain is still far bigger than any gap in
  the tank, so it cannot leak. The bands hold **~910,000 t**. Spacers call it **dust**, with some
  reverence and a lot of fear.
- **It is still hydrogen.** The box fuses it like any other protium. It
  shields like hydrogen, packed eight and a half times as dense.
- **It superconducts.** Metallic hydrogen was predicted in 1968 to be a
  high-temperature superconductor. IS metal is, well above the
  temperature it is stored at. That is what makes it pumpable (see Fuel
  Handling).
- **Only IS makes it.** It arrives by IS tanker and goes into the bands
  through sealed fill ports. IS meters every gram, not because the
  metal needs watching but because IS wants to know where its metal
  goes.

### What It Costs to Carry

Metal that reverts to molecular gas releases **216 MJ/kg** — about 50×
TNT. Above ~1000 K the metastability fails. That is small next to fusion
(0.03% of the p-p yield), and it does not matter in the drive. It
matters in a breach.

| Unit | Metal | Reversion energy | TNT |
|------|-------|------------------|-----|
| One fine grain | ~46 µg | ~10 J | a snap |
| One middle ball | ~16 mg | ~3 kJ | a firecracker |
| One coarse ball | ~6 g | ~1.2 MJ | a hand grenade |
| One silo compartment | ~53 kt | ~1.1×10¹⁶ J | ~2.7 Mt |
| Full bands | ~910 kt | ~2×10¹⁷ J | ~47 Mt |

### Piston Silos — the Fuel Never Falls

Each band is a stack of compartments, 20 m long, separated by **piston
bulkheads** that ride the core and the hull skin like a trash
compactor's ram. The gap between piston and wall is tighter than the
fines. A band drains one
compartment at a time, from the end nearest the feed, and the next
bulkhead follows the charge down. Every compartment is either full and
locked or being emptied under a piston. The shot is always packed. It
never has room to move.

That is the point. A loose charge in a partly empty tank, with the boxes
reversed, falls the length of the tank: twenty metres at 4 G is a
53 kt slug hitting a bulkhead at ~40 m/s, with every ball grinding
against its neighbours on the way. The pistons make that impossible by
construction.

The bulkheads are also firebreaks. A compartment going up whole is not
survivable and no bulkhead pretends otherwise. They are a wall for a
small reversion to stop at before it finds the rest. The nightmare is
**sympathetic reversion**, one ball's heat tripping its neighbours'.
Everything hot lives aft, behind the hold, away from the bands. In
combat a hit on the bands is the worst hit there is. The bands do not
jettison.

This is not antimatter. Metal is stable until it is heated or shocked,
it pours and stores, a grain that fails is a firecracker and not a
crater, and the gimme stays in IS's factories.

### Fuel Handling — Pumping Dust

You cannot revert the metal to gas and pipe it. Reversion is 216 MJ/kg
of heat inside the ship: a gigawatt at lean flow, terawatts in a fight.
So the metal reaches the box as metal, and the box reverts and fuses it,
where 216 MJ/kg is noise.

Each ball superconducts, so a magnetic field pushes it away. A
coil-lined pipe running a travelling field herds the shot along with
the balls held off each other and off the wall: nothing grinds, nothing
heats. Reverse the field and the flow stops dead. The valves are
magnetic.

The feed has to carry what the torch can drink:

| Burn | Flow (both boxes) |
|------|-------------------|
| Lean crossing burn | ~5 kg/s |
| 0.1 G, arriving | ~28 kg/s |
| 1 G, arriving / departing | ~2.8 / ~29 t/s |
| 3 G, arriving | ~25 t/s |
| 9 G (46 GN at 11 PW) | ~96 t/s |

Nine G sizes the line: ~1.7 m bore per nacelle at ~60 m/s. The route is
~500 m — from the draining compartment inward to the core spine, aft
along the core to the keystone, out through each post, into the box.

The cost is that a charged line is a fuse. Each nacelle's line holds
~400 t of moving shot, ~20 kt of TNT, threaded through its pylon. So the
lines run dry between burns: the shot is drawn back into the silo, and
the lines are **charged** before a burn — the moment everyone aboard
dislikes. A reversion in a line runs both ways until a magnetic gate
stops it. A hit on a post takes that nacelle off the line. A leak is
worse than a hole: spilled shot rolls everywhere, and every ball is a
firecracker that has to be found with a magnet.

### The Fuel Leash

IS sells metal at its service facilities and at **depots** it stocks
with its own tankers — more depots than service facilities, fewer than
inhabited systems. IS sets the price, and it is the largest line in any
freight budget. IS decides which systems get a depot.

A system without one is a slow-boat destination: a ship going there has
to carry the fuel home with it, and out-and-back on one fill is
~0.036c with a full hold, half a freighter's speed. Pulling a depot does not
blockade a system. It makes every trip there twice as long. See
[Trade](trade.md#interstellar-systems-and-the-shape-of-civilization).


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
- **Active combat defense**: Active Debris Defense (above) handles natural
  hazards; defending against a hostile actor's weapons, saturation, and
  countermeasures is unspecified — see Weapons systems
- **Sensors and communications**: Light-time and faint targets; relativistic
  corrections are mild at 0.10c
- **Life support**: Cold sleep technology, crew rotation
- **Fabrication**: What can and can't be manufactured aboard ship
- **AI systems**: Ship systems are sub-threshold AIs — sophisticated
  intelligences below the Compact's statutory personhood threshold. They
  manage themselves, respond to commands, and occasionally exhibit behavior
  that doesn't quite fit their programming. The player's treatment of these
  systems is tracked invisibly and shapes the endgame. Over the course of
  the game, ship systems may begin approaching the threshold. See
  [Narrative Design](narrative.md#sub-threshold-ai-interactions) for the
  full mechanic and its implications.
