# Ship Systems — Engineering Reference

## Hull and Structure

The ship is a slender cylinder — **1 km** long and **40 m** in radius
(an 80 m diameter, a 12.5:1 needle). Facing the interstellar medium
head-on at a fraction of light speed, frontal area still matters, so
this is only a third fatter than a 30 m hull, not a sausage. It is
constructed from carbon nanotube composites and graphene sheets —
materials with tensile strengths of 50-100 GPa at densities of
~1,400 kg/m³.

A hundred thousand tonnes of LH₂ is 1.4 million cubic metres. At 40 m
radius that is **315 m** of full-section tank — two bands around the
midships habitat — and leaves **365 m** of hold. The bays are shorter
than they were at 30 m; they are also fatter, ten tracks instead of
seven, and they carry more. The face grew by 1.8×. Dust and FEL time
scale with that.

For perspective: the structural stress at the base of a 1 km column
under 1G, at the ship's average density (~100 kg/m³ at operating
mass), is about 1.0 MPa. CNT handles 50 GPa. The structure could
withstand tens of thousands of G before failing; the drive's 9G
redline is nothing to the frame — the limits are the crew and the
nozzles, not the structure.

The load-bearing structure — a carbon-nanosheet inner hull plus the ship's
longitudinal frame — is a small fraction of the mass. The heavy part of the
hull is the multi-layer, water-stuffed Whipple armor wrapped around it (see
Hull Armor and Radiation Protection); the inner shell mainly holds pressure
and carries thrust.

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
     Carbon-nanosheet Whipple bumpers (outer, multi-layer)
     Water stuffing (barrel: radiation shield + debris absorber)
     Inner hull (carbon nanosheet, structural)
     ─────────────────────────────────────
     Unpressurized bays (cargo, hangars)
     LH₂ reaction mass tanks (distributed)
     ─────────────────────────────────────
     Crew / sensitive cargo core (pressurized)
     ─────────────────────────────────────
     LH₂ reaction mass tanks (distributed)
     Unpressurized bays (cargo, hangars)
     ─────────────────────────────────────
     Inner hull / water stuffing / carbon Whipple bumpers
```

The barrel hull is a **stuffed Whipple shield**: spaced carbon-nanosheet
bumper layers with water in the gaps. The carbon shrugs off the
interstellar gas and shatters micrometeoroids; the water disperses the
debris cloud and is the primary radiation shield for the crew's flanks.
Water and hydrogen are the best radiation shielding per unit mass, and the
water — unlike the reaction mass — is never burned, so the shield doesn't
thin as you spend fuel. The LH₂ tanks are distributed around the crew on
all sides — cosmic rays are isotropic, so the crew needs shielding from
every direction — adding to the water barrel and covering fore and aft. The
ship is **directionally specialized**: it never flips (the drive reverses
instead — see The Stardrive), so each end does one job. The **nose** is a
thick water-stuffed Whipple, the same construction as the barrel but
heavier and raked; it always faces the ISM and is the single most-hit
surface on the ship, so it gets the best armor the design has, not a
dual-role compromise. The **tail** is a pure radiator (see Thermal
Management) — it never meets the ISM, so it carries no shield duty at all.

### Longitudinal Layout

```
                  ↑ direction of travel — always

     [ Water-Whipple nose + collector coils + bow suite ]
     [ Cargo bay / hangars                               ]
     [ LH₂ tanks                                         ]
     [ Crew / sensitive cargo                            ]
     [ LH₂ tanks                                         ]
     [ Cargo bay / hangars                               ]
     [ Ship reactor / engineering                        ]
     [ Aft radiating hull (8 zoned gores)                ]  ← always in the wake

  Stardrive nacelles sit on 45° A-frames that pierce the hull and
  land on a torque box (two CNT belt rings + longerons). The boxes
  fire either way by internal reconfiguration — nothing on the
  pylon moves. The stand-off keeps the plumes off the radiating skin.
```

The ship is **directionally specialized and never flips**: the nose
always meets the ISM, the tail always radiates into the wake, and the
hotel reactor sits aft beside the radiators. The stardrive modules —
IS black boxes — ride in nacelles so their waste heat and radiation
stay off the hull. The pylons do not slew. Each IS box fires
fore or aft by reconfiguring internally on command — hours, not
seconds; IS does not publish the procedure. A fight is a one-way
burn. Midcourse you have years. The pylons are not cantilevers. Four 45° struts per nacelle pierce
the barrel and land on a torque box: two heavy CNT belt rings
through the engineering hull, longerons between them, a box around
the core. Thrust is axial in the struts — one pair in compression,
one in tension — and the rings spread it as hoop so the skin never
sees a point load. A 25 kt pod at 9 g is 2.2 GN; a radial boom
would have to take that as a 40 m bending moment. The A-frame
refuses the moment. Sized as tubes — Euler owns the 75 m
compression legs, ~22 cm wall in a 3.6 m strut — the mounts are
**~6,000 t**. There is no second engine at the tail. The **collectors** are
a separate system, not IS's: superconducting coils on the bow, always
facing the ISM. To decelerate at midcourse the scoop field reshapes
from funnel to parachute and the ship bleeds speed on magnetic drag;
once well below the 0.12c wall the boxes are told to reverse and the
torch finishes the descent on tanked hydrogen. The ship never turns
over. Nothing on the pylon moves.

Because the *ship* never turns over, **thrust-gravity reverses at
midcourse**. On the outbound half the drive pushes the ship forward, so
"down" is toward the **tail**; when the scoop becomes a parachute,
"down" is toward the **nose**, and it stays there through the reverse
burn. Cruise itself is milligees in the Bubble — almost free-fall. The
only true weightless moment is the handoff: sail idle, boxes
reconfiguring, torch not yet lit. **The floor becomes the ceiling
halfway through the trip.** The crew rides interstellar
cruise in cold sleep, so no one walks the decks during the swap, and the
pods, tankage, and fluid systems are built to work under thrust from
either direction. But anyone awake — an in-flight emergency, a
mid-cruise wake-up, an AI captain who never slept — finds the ship's
usable "down" set by which half of the voyage they're in, with a space
laid out for the outbound floor now hanging overhead on the way home.


## Cargo — The Hold

This is what the ship is *for*. Strip away the drive, the shield, the reactor,
and their only job is to carry the hold between stars. Two cargo bays — one
forward of the crew core, one aft — run about **180 m** and **185 m**. The
hydrogen takes 315 m next to them (LH₂ is fluffy; see Hull and Structure).
The bays are full-radius: every tonne of cargo the rack can take, it takes.

### The Standard Container

Cargo is fully containerized in a single standard box: **12 × 2.5 × 2.5 m**,
about 75 m³, ~20 t loaded — a deliberate descendant of the twentieth-century
intermodal container, because a standard that already worked for four hundred
years isn't worth reinventing. One size means one set of rails, one handling
system, one lock. The rack holds **10,304 containers** (~206,000 t) if every
slot is filled; the loadmaster leaves empty slots for trim.

### The Rack — a Hard Disk Stood on Its Axis

Gravity aboard runs *along* the long axis (the drive thrusts that way) and
**reverses at midcourse** — see Longitudinal Layout. That single fact shapes
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
stringers from wracking and takes the lateral and midcourse-flip
loads as axial spar forces. Gravity reverses at midcourse; a clamp
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


## Speed and the Forward Environment

The ship **cruises at 0.10c** (30,000 km/s). The stardrive's hard limit is
**0.12c**. That is not an engineering redline. It is a conservation-law wall.

The drive fuses the hydrogen it scoops. The p-p mass defect is ~0.007 \(mc^2\),
which buys an exhaust velocity \(v_e \approx 0.12c\) and no more. Catalysis —
Interstellar Systems' secret — makes the reaction *fast enough to run*. It does
not pay more energy per nucleon. Net thrust on a ram that swallows what it
fuses is \(\dot{m}(v_e - v)\). As \(v\) approaches \(v_e\), thrust dies. **0.12c
is the asymptote.** Captains hold **0.10c** so there is still something to push
with: midcourse corrections, a density clump, the first bite of turnaround.

Time dilation is not in this bargain. \(\gamma(0.10c) \approx 1.005\). A century
in flight is a century at home. The crew buys time with **cold sleep**, not
with \(\gamma\). An AI captain who stays awake pays the century in lived years.
See [Narrative](narrative.md) and [History](history.md).

### Why a Tenth Is Worth Having

On tankage alone the torch is a few thousand km/s of \(\Delta v\) —
enough for a port, not for a star. Ten light-years as a pure rocket
is a millennium. The collector is what makes a 30-light-year
civilization possible: it puts the ISM in the tanks so the torch can
keep burning. A tenth of \(c\) is as fast as that loop goes.

Flight time in the Local Bubble, accel and decel included (see below):

| Hop | Distance | Galaxy / sleeper-skip |
|-----|----------|------------------------|
| α Cen | 4.4 ly | ~70 yr |
| Typical trade leg | 10 ly | ~125 yr |
| Far edge of inhabited space | 30 ly | ~330 yr |

The minimum interstellar crossing is a human career. A campaign of ports is
centuries of galactic time. Sleepers skip it. Anyone who stays up lives it.

### The Local Bubble — the Map Is Inside It

Inhabited space is a 30-light-year sphere. The Local Bubble is on the order of
a thousand light-years across. **Every voyage the player can fly is in thin
air:** ~0.1 atoms/cm³, not the galactic-disk average of ~1. Spiral-arm densities
(5–50 cm⁻³) are not on this map. Route knowledge is local clouds and filaments
— a factor of a few, not three orders of magnitude.

Density does not change the speed limit. \(v_e\) is set by the mass defect, not
by how much hydrogen is out there. Density changes **how fast you reach cruise,
how hard the dust hits, and how much ³He you skim.** A fat filament is a gust:
you punch up to 0.10c faster, and you eat more sand. A clean lane is light air.

### The Acceleration Hump

Collection \(\dot{m} = \pi r^2 \rho v\) grows with speed, but net thrust is
\(\dot{m}(v_e - v)\). Acceleration is a **hump**, not a snowball: it peaks near
\(v_e/2 = 0.06c\) and falls to zero at the wall.

At 500,000 t, disk-average ISM (1 atom/cm³) — a dense local cloud, not the
Bubble:

| Ship velocity | Collection | Accel (500 kt) | Accel (250 kt) |
|---------------|------------|----------------|----------------|
| 0.003c (900 km/s) | 0.93 kg/s | 0.0066 G | 0.013 G |
| 0.03c | 9.3 kg/s | 0.051 G | 0.10 G |
| 0.06c — peak | 19 kg/s | 0.068 G | 0.14 G |
| 0.08c | 25 kg/s | 0.060 G | 0.12 G |
| **0.10c — cruise** | **31 kg/s** | **0.038 G** | **0.076 G** |
| 0.11c | 34 kg/s | 0.021 G | 0.042 G |
| 0.12c — wall | 37 kg/s | 0 | 0 |

In the Local Bubble, divide collection and accel by ten. At 900 km/s
scoop-fed thrust is 0.00066 G on a loaded hull. Peak is 0.007 G.
Cruise thrust is 0.004 G — station-keeping against a clump, not a
surge. Leaving Sol is a long climb in light air.

Time 0.003c → 0.10c at 500 kt, scoop-fed: **~2.3 years** and **~0.09 ly**
in a dense cloud; **~23 years** and **~0.9 ly** in the Bubble. The rest
of any long hop is cruise. Below a few thousand km/s the scoop's income
is a trickle. You climb that first bit on **tanked** LH₂ with the scoop
off or idling. Every kilogram burned in-system is a kilogram you might
want for that climb. A ship that fights itself dry and then departs
into the Bubble is a rocket with a long walk to income.

### The Forward Environment at Cruise

At 0.10c each ISM proton carries \((\gamma-1)mc^2 \approx 4.7\,\mathrm{MeV}\)
(use the relativistic figure; \(\tfrac12 mv^2\) is only 1% low here). Incident
power on the nose, disk-average:

> \(n\,c\,(mc^2)\,\gamma(\gamma-1)\beta \approx 23\,\mathrm{W/m}^2\)

About 116 kW over the ~5,000 m² nose. In the Bubble, 2.3 W/m² — a rounding
error next to sunlight. The hull does not have a heat problem. Unshielded, the
*dose* is still promptly lethal (MeV protons stop in the first millimetre of
tissue at tens of gray per second), so the Whipple water is the radiation
shield that matters. The scoop's job at this speed is **fuel and pre-filter**,
not keeping the crew alive; 4.7 MeV protons do not punch a water-stuffed metre.

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

The ship does not fly at the speed a captain wants. It flies at **0.10c**
because that is the last tenth the ram can still push, and it flies the lane
the navigator will sign because density is accel and dust, not a higher wall.
A clean lane is slow to cruise and kind to the hull. A dirty one is a gust
that pays in ³He and in FEL time. Sleepers do not care how long the crossing
is. The cargo, the insurance clock, and anyone awake do.


## Hull Armor and Radiation Protection

### The Problem

At 0.10c cruise, each ISM hydrogen atom carries \((\gamma-1)mc^2 \approx
4.7\,\mathrm{MeV}\). The raw particle flux on the forward face is about
23 W/m² at disk-average density, ~2 W/m² in the Local Bubble — thermally
nothing. Unshielded, the *dose* is still promptly lethal: MeV protons stop
in the first millimetre of tissue. The ISM is also ~1% dust, and a neutral
grain the magnetic field can't touch arrives as a hypervelocity impactor
(see Active Debris Defense). Galactic cosmic rays (GeV, roughly isotropic
at this \(\beta\)) are the penetrating radiation problem; the ram is not.

### The Solution (three layers)

1. **Magnetic deflection**: The bussard scoop's magnetic field extends
   thousands of kilometers ahead of the ship, funneling ionized ISM into
   the collector and glancing the rest aside — and most
   photoelectrically-charged dust with it — before they reach the hull.
   At 0.10c this is a fuel system and a sputtering pre-filter, not the
   crew's radiation shield; 4.7 MeV protons stop in the first millimetre
   of the Whipple.

2. **Stuffed carbon-Whipple hull**: The outer hull is not a coating but a
   **Whipple shield** — spaced carbon-nanosheet bumper layers with water in
   the gaps — doing two jobs at once:
   - *Gas erosion*: carbon is bound ~700× more tightly than the hydrogen
     ice it replaces (covalent ~7 eV/atom vs van der Waals ~0.01 eV), so
     the relativistic ISM gas barely touches it. Where the old ice eroded
     at ~85 kg/s and had to be continuously regrown, a carbon hull outlasts
     the ship by orders of magnitude against the gas — no ablation, no
     cryo, no scoop-fed replenishment.
   - *Micrometeoroids*: the outer bumper shatters and vaporizes a
     penetrating grain into a spreading debris cloud; the standoff gap lets
     it fan out; the water disperses it as a diffuse impulse instead of a
     hole-punch. Gas bladders in the water manage the pressure pulse and
     water's 9% freeze-expansion.

3. **Water + LH₂ radiation shield**: The barrel's water stuffing is the
   primary radiation barrier for the crew's flanks; the distributed LH₂
   reaction-mass tanks add more and cover fore and aft. Water and hydrogen
   are the best radiation shielding per unit mass. And because the water is
   never burned as propellant, the shield doesn't thin out as you spend
   fuel — a robustness the old LH₂-only barrier never had.

### The Whipple Hull

- **Barrel**: spaced carbon-nanosheet bumpers, water-stuffed, ~1 m thick.
  Always cool (grazing incidence), so the water self-arranges
  frozen-outer / liquid-inner with no active cryo.
- **Nose**: full water-stuffed Whipple — the same construction as the
  barrel, built thicker and raked. Because the ship never flips (see The
  Stardrive), the nose is *always* the leading face and the single most-hit
  surface on the ship, so it gets the best armor the design has rather than
  the dry compromise a dual-role endcap would force. It also sits inside the
  active-defense cone, which is trained forward, so it gets the most help
  shedding the medium grains.
- **Tail**: a pure radiator (see Thermal Management). It never meets the
  ISM, so it carries no shield duty — no bumper compromise, no
  drained-coolant dance, just a radiator built to radiate. The LH₂ tanks
  behind the crew carry the fore/aft radiation shielding.
- **Erosion**: negligible against the ISM *gas*. Practical lifetime is set
  by the rare large grain active defense misses and slow dust
  micro-cratering — patched at drydock, not self-healed. A raked, conical
  leading cap spreads the flux and glances off dust (sloped-armor logic),
  buying more margin still.
- **Inspection and repair**: the barrel gaps are a meter of water a person
  can enter. Crew suit up and **dive the hull** — internal, wet EVA — to
  inspect bumper layers and patch battle damage or micrometeoroid
  punctures from the inside, the water pumped down locally to work a
  section dry.
- **No scoop dependence**: the hull needs no in-flight replenishment. Water
  is topped up at any icy body; carbon is patched at service intervals.
  Lose the scoop and the *shield is fine* — only the fuel supply suffers.


## Active Debris Defense

### The Threat the Passive Shield Misses

The magnetic scoop and the carbon-Whipple hull handle the *interstellar medium* —
individual atoms and ions, ~4.7 MeV apiece at 0.10c (see Hull Armor), and the hull's
bumper layers shatter the smallest grains. But a **grain** with real mass is another
matter. The ISM is ~1% dust by mass: silicate and carbonaceous
motes from sub-micron up through the rare sand- and pebble-sized debris of
shattered worlds. A neutral grain isn't deflected by the scoop's field, and above
a few tens of micrometres no passive hull can absorb a concentrated impactor.
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

1. **Magnetic deflection** (scoop, existing): sweeps aside ionized matter *and*
   charged dust — most ISM grains are photoelectrically charged — thousands of
   km out.
2. **Whipple hull** (carbon + water, existing): shatters and absorbs the small
   *neutral* grains (≲ a few µm) that slip through.
3. **Active detection** (new): finds the medium grains the first two layers miss,
   far enough out to act.
4. **Active interdiction** (new): vaporizes or deflects them at standoff.
5. **Navigational avoidance** (new): the rare large bodies that can't be
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
  Power and aperture go on the bow, not a rotating air-search.
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

Passive IR/optical is the other one-way channel. Grains that hit the
scoop's bow plasma and the cooker's flood flash *before* they arrive.
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

### Interdiction — Spread It, Don't Match It

The interdictor is a **free-electron laser array** — tunable to grain
and bow plasma, several independently-steered directors. **It cannot
cancel the grain's kinetic energy.** 0.7 MJ forward is still 0.7 MJ
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
walk-not-just-pop, ionize-for-the-scoop. Average draw **~1 GW** off
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

### Ionize and Scoop — the Threat as Fuel

There's a better outcome than spreading a grain onto the hull, and it falls out of
a system the ship already carries. The scoop funnels **charged** matter into the tanks — that is its day
job. So if the FEL doesn't merely vaporize a grain but *ionizes the
plume*, the resulting plasma is charged, and the same field that
deflects ISM ions bends the debris plasma off the hull and **into the
tanks.** The impactor isn't neutralized, it's *eaten* — redirected
into the LH₂ the torch will drink. This is deflection in the truest sense: the
grain never reaches the hull, and its mass is on the books.

The physics constrains it. A solid grain, however charged, has far too little
charge-to-mass ratio for the field to bend — its gyroradius dwarfs the scoop. You
must vaporize it to a plasma first, so the field acts on individual ions (high
q/m) rather than a massive solid. And heavy debris ions (silicon, iron) carry
lower q/m than the hydrogen the scoop is tuned for, so they funnel less cleanly: a
fraction makes the tanks, the rest still ends up as a diffuse ablation load on
the hull. As torch-fuel it's second-rate — the box fuses hydrogen, not
metal — but any mass is remass, and the hydrogen/helium fraction is
real fuel. So the
interdiction hierarchy is: **ionize-and-scoop where the geometry allows (threat →
fuel), vaporize-and-spread as the fallback (threat → ablation), avoid what's too
large for either.**

This sharpens the route tension: a dirty region is more dangerous *and* more
fuel-rich. More grains means more impacts to interdict and more power on the laser
bus — but also more mass funneled into the tanks. Flying the dusty lane is a
gamble that pays in propellant, if the FEL keeps up.

### Where It Couples

The FEL and the cooker both drink from the 63 GW bus and the same
capacitor farm. A dirty passage is a power event and a thermal event:
generation losses land on the tin loop, and the cooker is a bow-facing
beacon as loud as a lit radiator zone. Every joule that leaves as beam
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

### Empty Mass (~165,000 tonnes)

The ship with nothing in it — hull, structure, permanently installed
systems:

| Component | Mass |
|-----------|------|
| Hull (multi-layer carbon-nanosheet Whipple armor) | 27,000 t |
| Internal structure (decks, bulkheads, framing) | 28,000 t |
| Ship reactor + direct conversion (~100 GW fusion) | 8,000 t |
| Thermal management (tin loops, headers) | 8,000 t |
| Collectors (scoop coils, not IS) | 14,000 t |
| Stardrive modules (IS boxes, feeds) | 52,000 t |
| Drive mounts (A-frames, belt rings, torque box) | 6,000 t |
| Weapons systems | 10,000 t |
| Sensors, communications, computing | 5,000 t |
| Life support (sized for ~10 crew) | 500 t |
| Hangar structures and launch systems | 3,000 t |
| Miscellaneous (wiring, plumbing, fittings) | 3,000 t |

### Operating Mass (~312,000 tonnes)

Ready to fly — fueled, shielded, crewed, but no cargo:

| Component | Mass |
|-----------|------|
| Empty mass | 165,000 t |
| Whipple shield water (barrel: radiation + impact stuffing) | 40,000 t |
| LH₂ (torch fuel, remass, supplementary shield) | 100,000 t |
| D-³He reactor fuel | 2,000 t |
| Atmosphere (hab air + reserves) | 20 t |
| Water and consumables | 100 t |
| Small craft (shuttles, fighters, transports) | 5,000 t |
| Crew (~10 people) | ~1 t |

### Loaded Mass (up to ~520,000 tonnes)

Operating mass plus cargo. **Cargo capacity: ~206,000 tonnes.**

There is no separate drive-fuel bunker. The torch drinks the same LH₂
the scoop puts in the tanks. A hull breach still spills cold hydrogen.
Nothing here has to be *contained*.

Cargo includes: cold sleep pods with passengers, black-boxed technology,
trade goods, raw materials, colonization equipment — whatever the mission
requires. Actual loaded mass varies:

- Trade run: ~470,000 t
- Military patrol: ~380,000 t (light load)
- Colony expedition: ~520,000 t (maximum)


## Power: The Fusion Reactor

The ship's power plant is a D-³He (deuterium-helium-3) fusion reactor
rated at **~100 GW** fusion. It is not the drive. The torch handles
propulsion and dumps almost all of its waste into the plume. This
plant is hotel, scoop cryo, fabrication, sensors, and the FEL/weapons
bus.

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
The scoop's superconducting coils, once charged, want cryo, not a
power station. The FEL is GW-class at 0.10c — a 1-second, 1 GW pulse
is a gigajoule, and a sand grain is ~0.7 GJ. Weapons charge
capacitors off the same bus. Fabrication can draw hard for a while.
A 500 GW plant was a leftover from when someone thought the drive
needed a hotel that large. It did not. Keep a capacitor farm if you
want a bright, short shout. Do not haul a power station to feed it.

The **light show is the point, in combat.** Even 37 GW of waste, if
you light every zone, is a beacon. Run cool and dark at cruise. Light
panels when you need the watts. Shut the ones that face a contact.

³He at rated is ~5.4 t/year. The 2,000 t tank is now centuries at
redline, forever at cruise. The skim still matters because cruise in
the Bubble is break-even at a few percent. The tank is insurance, not
the weekly shop.


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
**centuries** (~5.4 t/year of ³He). The skim still sets the cruise
budget in the Bubble, not the bunker.

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
holds, tanks, and habitat stay Whipple and water, or insulated
cryogen. You do not run 1800 K skin over LH₂ or over a sleeper
container. The 2 m barrel that was water on those stations is
vacuum and MLI next to the hot gores.

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
*not* burning, or is burning away from you. The A-frames stand the
nozzles ~50 m off the skin so a collimated plume misses the
gores; at combat divergence the aft-most gores valve off because
the plume is already heating them.

No deployables. Zoning is valves. The radiating skin is the hull.
Its mass is already on the hull line; the thermal budget is the
tin loop and the headers.

### Midcourse Turnaround

The ship never flips, so there is no flip thermal sequence — the old
multi-day cool-the-radiator, rebuild-the-shield ordeal is simply gone. At
midcourse the scoop becomes a magnetic parachute and the ship brakes for
years on ISM drag; the boxes reconfigure to fire forward only when it
is time to burn the last of the speed off (see The Stardrive). The tail
radiates throughout; the nose shields throughout. Nothing structural
on the ship itself moves, and there is no vulnerable window — the
armor is permanent, not a coating that has to be regrown.

The one thing the crew would feel, if any were awake, is that
thrust-gravity reverses: "down" points sternward on the accelerating half
and bow-ward on the decelerating half, with a brief weightless coast
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

Two systems. They do not share a vendor.

**The stardrive** is an Interstellar Systems black box. You pipe liquid
hydrogen in. A fusion torch comes out. That is the entire published
interface. Catalyzed p-p — the secret — is inside. There is no ram,
no pellet, no onboard igniter farm. Full stop.

**The collectors** are not IS's. They are a magnetic scoop any decent
yard can build and maintain: ionize, funnel, tank. They fill the LH₂
the torch drinks. They also glance dust and, reshaped, parachute
against the ISM. See The Collector.

The torch does not care whether the hydrogen came from a dock, a
comet, or a century of scooping. In-system and interstellar are the
same engine on a different budget.

### Two Knobs, and Combat Lives Between Them

**Exhaust velocity (Isp).** Fuel and remass are the same fluid. Fuse a
larger fraction and \(v_e\) climbs (lean, long, gentle). Fuse a little
and throw a lot of cold hydrogen with it (rich, hard, short). Cruise
runs lean. A fight runs rich.

**Power (overdraw).** The nozzles are rated for **3.2 PW** continuously.
You can run them hotter at rising risk: a **soft redline at 2× (6.4 PW)**,
sustainable for minutes, and a **hard limit at 3.5× (11 PW)**, good for
seconds. Past that the exhaust plasma overwhelms the nozzle field and
burns through the throat, or the coils quench. Overdraw buys thrust
*without* spending your range — you pay in nozzle life instead.

Acceleration is **a = 2·P / (m·v_e)**. At the same 3.2 PW that cruises a
loaded hull at 0.1 G:

| Mode | Exhaust vel | Accel (~490 kt) | Full-tank endurance | Δv from whole tank |
|------|------------:|----------------:|--------------------:|-------------------:|
| Cruise (lean) | 13,100 km/s | 0.1 G | ~30 days | ~2,900 km/s |
| Hard maneuver | 1,300 km/s | 1 G | ~7 hours | ~290 km/s |
| Combat | 435 km/s | 3 G | ~50 min | ~97 km/s |
| Violent | 260 km/s | 5 G | ~18 min | ~58 km/s |

Read the last column and wince: hard acceleration is **Δv-catastrophic**.
Empty the whole tank at 5 G and you've changed velocity by 58 km/s; spend
it gently and the same mass buys 2,900. Every combat jink is Δv you needed
to get home.

### The Combat Redline

Maximum acceleration is **~9 G**, reached with overdraw and a rich mixture
together — and it is a *convergent* redline. Three independent systems hit
their limit at the same number:

- **Crew** — in immersion couches with pharmacological support (the same
  tech as the cold-sleep pods), the crew take ~9 G briefly, ~5 G sustained.
  Bare, it's ~5 G.
- **Nozzle** — the 3.5× overdraw burn-through / quench limit.
- **Reaction mass** — the tanks empty in minutes at these rates.

Below 9 G you have margin on all three; at 9 G you have margin on none, and
battle damage lowers whichever gives first. Combat is a three-resource
economy — **nozzle life, Δv-efficiency, reaction mass** — traded live. The
ship handles like a supertanker that can throw one hard punch: it lunges at
several G, but each lunge spends stamina it needs to get home. The agile
work belongs to the fighters (100 t, ~5,000× lighter), which pull tens of G
on the same drive tech. The capital ship punches; the fighters knife-fight.

### Performance Envelope

| Loaded mass | Available Δv | 0.1G endurance |
|-------------|-------------|----------------|
| ~520,000 t (max cargo) | ~2,800 km/s | ~31 days |
| ~400,000 t | ~3,700 km/s | ~24 days |
| ~330,000 t (light) | ~5,200 km/s | ~18 days |

In-system transits still take days; the ship is a kilometre long and
masses hundreds of thousands of tonnes. The drive changed; the heft
did not.

### Port Call — the Tank Is a Reserve

Interstellar climb and braking are scoop-fed. The 100,000 t tank is
what you spend when the scoop is off: in-system, near traffic, in a
fight, and on the first bit of a departure into thin air.

A clean port call — approach, dock, depart, light the scoop — barely
dents it. The old 2,300 km/s "catch tax" is gone; there is no second
engine to hand off to. Combat, or a long scoop-off burn, is what
empties the reserve. At max cargo the tank is still ~2,900 km/s if
you spend it lean, ~58 km/s if you spend it at 5 G. Same table as
above. Same wince.

**The trade tension** is no longer "buy deuterium or gamble." Hydrogen
is everywhere. The tension is **whether the tank is full when you
light the scoop.** Arrive from a crossing and you are full. A clean
port call barely touches the reserve. A fight can empty it. Depart
combat-dry into the Bubble and you are a rocket until income
catches the burn — a long, slow, embarrassing climb. Linger in a
filament, buy LH₂ at the dock, or leave light. Deuterium is a
reactor story. It is not a drive story.


## The Stardrive

The stardrive is a black-boxed module built by Interstellar Systems.
Its internals are proprietary, with multiple layers of tamper
protection. IS does not sell the scoop. IS does not maintain the
scoop. IS sells the box.

What is known: you connect an LH₂ feed. The box produces a fusion
torch. The process fuses plain protium via the proton-proton chain,
which conventional physics considers far too slow for engineering
use. How they solved the rate is the most closely guarded secret in
human civilization.

That is the whole machine. It is not a ramjet. Collection is
someone else's hardware. Insert hydrogen, extract thrust.

### Why This Matters

Normal D-D or D-³He fusion can only burn deuterium or helium-3 —
trace in the ISM (deuterium is ~20 ppm of the hydrogen). The torch
fuses **protium**, so every kilogram the collector tanks is fuel
and remass. That is the difference between a ship limited to a
tank of Δv and one that can cruise at a tenth of lightspeed.

The secret is the *rate*. Catalyzed p-p does not increase the
yield. The energy per nucleon is still ~6.7 MeV, \(v_e \approx
0.12c\) when you fuse what you throw. A static magnetic field does
no work. The collector steers mass into the tanks and, when you
leave it on while you burn, you still pay \(\dot{m} v\) of drag
for the privilege. Net thrust with scoop and torch in steady state
is \(\dot{m}(v_e - v)\). The wall is still \(v = v_e\). There is
no field geometry that retires it.

With the scoop **off**, the torch is a pure rocket. No ram drag,
no income. \(\Delta v = v_e \ln(m_0/m_f)\). The 100,000 t tank on
a 500 kt hull is ~8,000 km/s at the theoretical ceiling — a
generous port, a lost star. You need the collector to climb.

### Acceleration Profile

Thrust is \(\dot{m}(v_e - v)\) with \(\dot{m} \propto v\). Collection grows
with speed; net thrust dies as \(v\) approaches \(v_e\). The curve is a
hump that peaks near 0.06c and is zero at 0.12c. The table below is
disk-average ISM (1 atom/cm³) — a fat local cloud. In the Local Bubble,
divide collection and accel by ten. See Speed and the Forward Environment.

| Ship velocity | Collection (disk) | Accel (500 kt) | Accel (250 kt) |
|---------------|-------------------|----------------|----------------|
| 0.003c (900 km/s) | 0.93 kg/s | 0.0066 G | 0.013 G |
| 0.03c | 9.3 kg/s | 0.051 G | 0.10 G |
| 0.06c — peak | 19 kg/s | 0.068 G | 0.14 G |
| 0.08c | 25 kg/s | 0.060 G | 0.12 G |
| **0.10c — cruise** | **31 kg/s** | **0.038 G** | **0.076 G** |
| 0.11c | 34 kg/s | 0.021 G | 0.042 G |
| 0.12c — wall | 37 kg/s | 0 | 0 |

Sluggish at the bottom, a modest shove in the middle, nothing at the
wall. The first few thousand km/s are tanked, scoop off. Then you
live on income. A dry tank at the bottom of a Bubble climb is the
new "missed the catch."

### Deceleration — Parachute, then Torch

Near cruise, \(v \approx v_e\): a reverse burn has almost nothing to
give, and firing the exhaust into the scoop field is a good way to
thrash the magnetosphere. So the first brake is the **collector**.
The coils reshape from funnel to parachute. Incoming ISM reflects;
the ship takes the recoil. Drag scales as \(\dot{m}\,v \propto v^2\).
From 0.10c the sail has leverage the rocket has just lost.

Once speed is well below the asymptote you command the boxes to
reverse — internally, on IS's schedule, hours — and the torch
finishes the descent on tankage. The collectors stay pointed at the
ISM. The ship never turns over. The nose stays a shield; the tail
stays a radiator. The A-frames do not move.

A box that will not reverse no longer means "you cannot stop." The
sail still dumps speed. It does mean the last burn is improvised,
and you may arrive hot. See Maintenance. The collectors are not on
that interval. A yard that cannot touch an IS box can still patch
a coil.

### Maintenance

The **box** requires servicing by Interstellar Systems every ~60
light-years of travel. The collectors do not. The inspection is
opaque — IS tells you what needs doing and what it costs, never the
details of what they found. The cost varies unpredictably.

Skipping maintenance is possible but not advisable. Stardrives that
exceed their service interval have been known to fail in "energetic"
ways — a drive that throws 500,000 tonnes at a tenth of \(c\) does not
fail quietly. No captain pushes the interval twice. Sixty light-years at
cruise is six hundred years of flight; the interval is a fluence and wear
budget, not a calendar.

IS maintains service facilities in a handful of systems across
inhabited space. This constrains route planning — your circuit must
pass within ~30 light-years of a facility, because you need to get
there and back before the interval expires. See
[Trade](trade.md#interstellar-systems-and-the-shape-of-civilization)
for the strategic implications.


## Cruise Operations

During interstellar cruise at 0.10c the ramscoop is both the drive's
intake and the ship's refinery. What it gathers, and how long a tank
takes to fill, is a density question. Numbers below are **Local Bubble**
(~0.1 atoms/cm³) first — that is the map — with a dense local cloud
(1 atom/cm³) in parentheses.

- **Reaction mass (LH₂)**: Collection ~3.1 kg/s (31 kg/s). The 100,000 t
  remass tanks refill in **~1–2 years** in the Bubble (~a month in a fat
  cloud). Ships still arrive with full tanks on any hop long enough to
  matter.
- **Hull armor**: Needs no in-flight replenishment — the carbon Whipple hull
  doesn't ablate, and its water stuffing is sealed behind it (topped up at
  ports, not scooped). Losing the scoop no longer threatens the shield,
  only the fuel supply.
- **D-³He reactor fuel**: Deuterium skims freely (~20 ppm of ISM hydrogen).
  Helium-3 is scarcer: the field is optimized for protons, He²⁺ has half
  the charge-to-mass ratio, and the Bubble is thin. Cruise skim is
  **~0.5 t/year of ³He** in the Bubble (~5 t/year in a dense cloud). The
  reactor at full power burns ~27 t/year, so collection never covers
  redline. At a deep-cruise few percent of rated power, Bubble skim is
  roughly break-even or a slight deficit. There is no comfortable surplus
  to bank. **³He is a port commodity** — gas-giant atmosphere mining —
  and a long cruise in thin air spends the tank you bought at the last
  dock.
- **Torch hydrogen**: the same LH₂ the remass tanks already are.
  The scoop *is* the refuel. There is no deuterium bunker for the
  drive. Reactor D is a separate, small account (~20 ppm skim,
  plus whatever you buy).

The crew is in cold sleep during transit. Ship AI handles navigation,
scoop-field shaping, and resource management autonomously. An AI captain
who stays awake lives the crossing.


## The Collector

The scoop is **not** an Interstellar Systems product. Superconducting
coils on the bow — yards build them, crews patch them, no 60 ly
pilgrimage — throw a magnetic field ~14,000 km ahead. The field is
the size of a planet; the hardware you can see is three rings around
the nose and a throat on the axis. It ionizes interstellar hydrogen
and funnels it into the **tanks**. The torch never sees the ISM. It
sees LH₂.

### Collection Rate

**ṁ = π × r² × ρ × v**

Where:
- r = scoop radius (~14,000 km = 1.4 × 10⁷ m)
- ρ = ISM density
- v = ship velocity

At 0.10c and disk-average density (1 atom/cm³ = 1.67 × 10⁻²¹ kg/m³):

> π × (1.4×10⁷)² × 1.67×10⁻²¹ × 3.0×10⁷ ≈ **31 kg/s**

The Local Bubble, which contains the entire 30 ly inhabited sphere, is
~0.1 atoms/cm³: **~3.1 kg/s** at cruise. Local clouds and filaments move
that number by a factor of a few. Spiral-arm densities are not on this
map. Density sets time-to-cruise, dust load, and ³He skim — not the
0.12c wall.

The same coils do three jobs: **funnel** (into the tanks), **glance**
(deflect what you do not want, cheaply), and **parachute** (reflect
the ISM and brake). They are not a radiation shield for the crew;
4.7 MeV protons stop in the Whipple. They are not an accelerator; a
static B field does no work. They are not a drive. Lose the scoop
and you still have a torch and whatever is in the tank. Lose the
box and you have a very expensive balloon of hydrogen.


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
