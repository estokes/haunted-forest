# Ship Systems — Engineering Reference

## Hull and Structure

The ship is a slender cylinder — roughly 1 km long and 30 m in radius (a
60 m diameter, a ~17:1 needle). Facing the interstellar medium head-on at a
fraction of light speed, a small frontal cross-section is worth far more
than interior convenience, so the hull is long and narrow. It is
constructed from carbon nanotube composites and graphene sheets — materials
with tensile strengths of 50-100 GPa at densities of ~1,400 kg/m³.

For perspective: the structural stress at the base of a 1 km column under
1G, at the ship's average density (~120 kg/m³ at operating mass), is about
1.2 MPa. CNT handles 50 GPa. The structure could withstand tens of thousands
of G before failing; the drive's 9G redline is nothing to the frame — the
limits are the crew and the nozzles, not the structure.

The load-bearing structure — a carbon-nanosheet inner hull plus the ship's
longitudinal frame — is a small fraction of the mass. The heavy part of the
hull is the multi-layer, water-stuffed Whipple armor wrapped around it (see
Hull Armor and Radiation Protection); the inner shell mainly holds pressure
and carries thrust.

Most of the interior volume is **unpressurized**: cargo bays, hangars,
engineering spaces, fuel tankage. Pressurized habitable space for the crew
of ~10 is a tiny fraction of the total — a few hundred cubic meters of
quarters, a bridge, a workshop. The ship is essentially a massive
automated cargo and exploration vessel with a small human crew making
decisions.


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

     [ Water-Whipple nose cap ]      ← always faces the ISM
     [ Cargo bays / hangars    ]
     [ LH₂ tanks               ]
     [ Crew / sensitive cargo  ]
     [ LH₂ tanks               ]
     [ Cargo bays / hangars    ]
     [ Ship reactor / engineering ]
     [ Radiator tail           ]      ← always in the wake → cold space
     [ Conventional engines    ]

  Stardrive nacelles ride on booms alongside the hull, each a lockable
  turret: nozzle aft to accelerate, nozzle forward to decelerate.
```

The ship is **directionally specialized and never flips**: the nose always
meets the ISM, the tail always radiates into the wake, and the reactor sits
aft beside the radiator and the conventional engines. The stardrive modules
ride in nacelles offset from the hull on structural booms, which isolates
their waste heat and radiation; each nacelle is a lockable two-position
turret (see The Stardrive). To decelerate at midcourse, the drive cuts out,
the ship coasts a few minutes while the nacelles swing 180°, and the drive
relights facing the other way. The scoop keeps collecting ISM hydrogen
throughout; the exhaust, far faster than the ISM, blasts clear without being
recollected.

Because the *ship* never turns over, **thrust-gravity reverses at
midcourse**. On the outbound accelerating half the drive pushes the ship
forward, so "down" is toward the **tail**; after the nacelle-flip the drive
decelerates and "down" is toward the **nose**, with a brief weightless coast
between. **The floor becomes the ceiling halfway through the trip.** The
crew rides interstellar cruise in cold sleep, so no one walks the decks
during the swap, and the pods, tankage, and fluid systems are built to work
under thrust from either direction. But anyone awake and active during a
burn — an in-flight emergency, a mid-cruise wake-up — finds the ship's
usable "down" set entirely by which half of the voyage they're in, with a
space laid out for the outbound floor now hanging overhead on the way home.


## Cargo — The Hold

This is what the ship is *for*. Strip away the drive, the shield, the reactor,
and their only job is to carry the hold between stars. Two cargo bays — one
forward of the crew core, one aft — run about 300 m each and together are
roughly **60% of the ship's length**. Everything else is in service of them.

### The Standard Container

Cargo is fully containerized in a single standard box: **12 × 2.5 × 2.5 m**,
about 75 m³, ~20 t loaded — a deliberate descendant of the twentieth-century
intermodal container, because a standard that already worked for four hundred
years isn't worth reinventing. One size means one set of rails, one handling
system, one lock. A full hold is on the order of **7,500 containers** (~150,000
t of cargo); the rack is sized for a little more, to leave slack for trim.

### The Rack — a Hard Disk Stood on Its Axis

Gravity aboard runs *along* the long axis (the drive thrusts that way) and
**reverses at midcourse** — see Longitudinal Layout. That single fact shapes
the whole rack. The cargo rails run fore-and-aft, parallel to gravity; each
container clamps to an **inner and an outer rail** and rides the gap between
them.

Those rails hang on a polar skeleton that, seen down the ship's axis, looks
like a hard disk: **radial support trusses** split each bay into eight
**sectors** (the spokes), **concentric arc rings** mark the boundary of each
**track**, and the rails run between the rings. Seven tracks nest from the core
out to the inner hull; because the outer tracks have more circumference they
hold more boxes — one or two per sector at the cramped inner rings, six at the
outermost. The trusses bond directly to the central core and to the inner hull,
grown from the same carbon-nanotube stock and fused to both. It's all open
framework: a container slides straight along its rails and out through the open
rings, so no box ever has to be dug out from behind another.

### What Actually Holds It Up

Under a 9 g combat burn a 20 t container weighs **1.77 MN** — and there are
thousands. If each box hung from its own rails, every rail would carry a whole
fore-and-aft column, buckling would force them absurdly thick, and the rack
alone would out-mass the entire hull. So the boxes **stack against the bay-end
bulkheads** under thrust, the way cargo has always ridden down onto a floor,
and the bulkheads carry that load into the ship's spine. The rails don't hold
the weight — they hold *position*: lateral restraint plus a positive lock so
nothing drifts during the weightless coast when the ship flips. The
unglamorous **bay-end bulkheads, not the elegant rack, are the primary
structure of the hold.**

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


## Hull Armor and Radiation Protection

### The Problem

At 0.5c, each ISM hydrogen atom carries ~117 MeV of kinetic energy —
comparable to a cosmic ray proton. The raw particle flux on the forward
face is about 2.8 kW/m². Unshielded, this would cook the crew in hours.
And the ISM is ~1% dust: a neutral grain the magnetic field can't touch
arrives as a hypervelocity impactor (see Active Debris Defense).

### The Solution (three layers)

1. **Magnetic deflection**: The bussard scoop's magnetic field extends
   thousands of kilometers ahead of the ship, deflecting ~95% of ionized
   ISM particles — and most photoelectrically-charged dust — before they
   reach the hull.

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
individual atoms and ions, ~117 MeV apiece at 0.5c (see Hull Armor), and the hull's
bumper layers shatter the smallest grains. But a **grain** with real mass is another
matter. The ISM is ~1% dust by mass: silicate and carbonaceous
motes from sub-micron up through the rare sand- and pebble-sized debris of
shattered worlds. A neutral grain isn't deflected by the scoop's field, and above
a few micrometres no passive hull can absorb a concentrated impactor. At 0.5c the kinetic energy of even a
speck is absurd (KE ≈ (γ−1)mc², γ ≈ 1.155):

| Grain | Mass | KE at 0.5c | Equivalent | Handled by |
|-------|------|-----------|------------|------------|
| 1 µm mote | ~1.6 pg | ~22 J | a hard flick | Whipple hull |
| 10 µm dust | ~1.6 ng | ~22 kJ | a rifle round | Whipple hull, marginally |
| 100 µm fine sand | ~1.6 µg | ~22 MJ | ~5 kg TNT | **active defense** |
| 1 mm coarse sand | ~1.6 mg | ~22 GJ | ~5 t TNT | **avoidance** |
| 1 cm pebble | ~1.6 g | ~22 TJ | ~5 kt (nuclear) | **avoid at all costs** |

Nothing larger than a molecule may strike the hull at cruise. A grain of sand
is a tactical munition; a pebble is a warhead. The ship cannot tank these — it
must never let them arrive intact.

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

### Detecting at Relativistic Closing Speed

The closing speed is 1.5×10⁸ m/s, and active ranging is a race against its own
echo. A pulse to a grain at range R and back takes 2R/(c+v); during that time
the grain closes from R to **R·(c−v)/(c+v) — to one third of the detection range
at 0.5c, before the echo even returns.** Detect at 150,000 km and you learn of
it at 50,000 km with ~0.3 s to act; everything closer is effectively undetectable
by round-trip means. The forward sky is also aberrated (threats compressed into a
forward cone) and Doppler-shifted by √((1+β)/(1−β)) ≈ 1.7×, so the whole suite is
tuned for a blueshifted, forward-bunched world.

The suite is multi-modal, layered by wavelength because no single band sees
everything:

- **Long-wave radar** for rare large bodies at long range — big cross-section,
  enough resolution to flag a threat and start a course solution minutes to hours
  out.
- **mm- and sub-mm radar** for sand-grade grains close in — small targets need
  short wavelengths; the Rayleigh-suppressed return is brutally weak, so this is a
  high-power, short-range band.
- **LIDAR** for precise track and aimpoint on grains already flagged — narrow
  beam, must be cued, but pins the solution the laser needs.
- **Multi-frequency coverage** so grains that are radar-dark in one band
  (absorption features, size resonances) light up in another.
- **Passive forward-staring IR/optical** — the only channel with no round-trip
  penalty. Grains entering the bow wave (the scoop's compressed plasma and the
  forward radiation field) flash-heat and ablate *before* arrival; their own
  emission is one-way light, so the passive channel often sees a threat sooner
  than the radar chasing it.

### Interdiction — Spread It, Don't Destroy It

The interdictor is a **free-electron laser array** — tunable across wavelengths
to match grain composition and punch through the bow plasma, with multiple
independently-steered beam directors to engage many threats at once. The
crucial, counterintuitive physics: **the laser cannot remove the grain's kinetic
energy.** Momentum is conserved; 22 MJ of forward-directed energy is still 22 MJ
after vaporization. What it *can* do is spread it: a concentrated point impact
drills through ice and hull, but the same energy arriving as a diffuse plasma
cloud fanned across a square meter of the carbon hull merely craters a scar. So the FEL's
job is to detonate each grain at standoff and let the passive shield take the
diffuse remnant — **converting an impactor into an ablation load.** Where range
and time allow it does better: an asymmetric ablation jet gives the grain a
sideways kick (the rocket effect off its own vaporizing face) and walks it clear
of the hull — far cheaper than destroying it.

The array is, by temperament, **deeply anti-social.** As a sub-threshold ship
intelligence (see [Narrative Design](narrative.md#sub-threshold-ai-interactions))
it has settled into a personality defined almost entirely by ordnance: its sense
of a quarter well spent correlates directly with tonnage vaporized, and it does
not love long clean stretches of empty medium. The crew learns to read its mood
the way you'd read a bored predator.

### Ionize and Scoop — the Threat as Fuel

There's a better outcome than spreading a grain onto the ice, and it falls out of
a system the ship already carries. The bussard scoop funnels **charged** matter
into the collector — that is its day job. So if the FEL doesn't merely vaporize a
grain but *ionizes the plume*, the resulting plasma is charged, and the same field
that deflects ISM ions bends the debris plasma off the hull and **into the
collector throat.** The impactor isn't neutralized, it's *eaten* — redirected into
the reaction-mass and fuel stream. This is deflection in the truest sense: the
grain never reaches the hull, and its mass is on the books.

The physics constrains it. A solid grain, however charged, has far too little
charge-to-mass ratio for the field to bend — its gyroradius dwarfs the scoop. You
must vaporize it to a plasma first, so the field acts on individual ions (high
q/m) rather than a massive solid. And heavy debris ions (silicon, iron) carry
lower q/m than the hydrogen the scoop is tuned for, so they funnel less cleanly: a
fraction makes the collector, the rest still ends up as a diffuse ablation load on
the ice. As fuel it's second-rate — the stardrive fuses hydrogen, not metal — but
any mass is reaction mass, and the hydrogen/helium fraction is real fuel. So the
interdiction hierarchy is: **ionize-and-scoop where the geometry allows (threat →
fuel), vaporize-and-spread as the fallback (threat → ablation), avoid what's too
large for either.**

This sharpens the route tension: a dirty region is more dangerous *and* more
fuel-rich. More grains means more impacts to interdict and more power on the laser
bus — but also more mass funneled into the tanks. Flying the dusty lane is a
gamble that pays in propellant, if the FEL keeps up.

### Where It Couples

The FEL is not free. It draws from the reactor's ~315 GW electrical bus, so a
debris-heavy passage is a *power* event — beams up means less for everything
else, and the generation losses land as waste heat on the same tin loop that
cools the reactor, nudging the whole ship toward its thermal limit (see Thermal
Management). Conversely, every beam that leaves the hull *exports* energy to
space — the debris laser is one of the few loads that radiates its draw outward
rather than dumping it inboard. And detection coverage is finite: the crew (or
the AI) allocates sensor power and standoff across the forward cone, and the
route planner weighs known dust density against schedule — because the cheapest
defense against a pebble is to never fly at it.

### Open Tensions

- **The undetectable close-in fraction.** Round-trip ranging can't see the last
  few tens of thousands of km; the passive forward-stare and a standoff kill-zone
  cover some of it, and the residual is an accepted, ice-absorbed hit rate. How
  high that runs in a dirty region is a real operational risk.
- **The pebble you can't dodge.** At 0.1 G and a few thousand km/s of in-system
  Δv, evading a body spotted light-minutes out is marginal. Avoidance is mostly a
  *navigation*-layer problem — route around dense regions — not a last-second
  maneuver.


## Mass Budget

### Empty Mass (~165,000 tonnes)

The ship with nothing in it — hull, structure, permanently installed
systems:

| Component | Mass |
|-----------|------|
| Hull (multi-layer carbon-nanosheet Whipple armor) | 20,000 t |
| Internal structure (decks, bulkheads, framing) | 20,000 t |
| Ship reactor + direct conversion (~500 GW fusion) | 25,000 t |
| Thermal management (tin loops, tail radiator) | 8,000 t |
| ICF drive: igniter banks, reaction chamber, magnetic nozzles, neutron shield (~25 TW) | 20,000 t |
| Stardrive nacelles (black-boxed, incl. structure) | 50,000 t |
| Weapons systems | 10,000 t |
| Sensors, communications, computing | 5,000 t |
| Life support (sized for ~10 crew) | 500 t |
| Hangar structures and launch systems | 3,000 t |
| Miscellaneous (wiring, plumbing, fittings) | 3,000 t |

### Operating Mass (~350,000 tonnes)

Ready to fly — fueled, shielded, crewed, but no cargo:

| Component | Mass |
|-----------|------|
| Empty mass | 165,000 t |
| Whipple shield water (barrel: radiation + impact stuffing) | 30,000 t |
| Reaction mass (LH₂ — drive fuel + supplementary shield) | 100,000 t |
| Drive fuel (deuterium, distributed tanks) | 45,000 t |
| D-³He reactor fuel | 2,000 t |
| Atmosphere (crew areas only) | 500 t |
| Water and consumables | 100 t |
| Small craft (shuttles, fighters, transports) | 5,000 t |
| Crew (~10 people) | ~1 t |

### Loaded Mass (up to ~500,000 tonnes)

Operating mass plus cargo. **Cargo capacity: ~150,000 tonnes.**

The ICF drive's deuterium fuel is a real slice of the mass budget — the
price of fusion's modest energy density, and the reason cargo capacity sits
where it does. The designers took the trade gladly: the ship carries cheap,
inert fuel instead of anything that has to be *contained*.

Cargo includes: cold sleep pods with passengers, black-boxed technology,
trade goods, raw materials, colonization equipment — whatever the mission
requires. Actual loaded mass varies:

- Trade run: ~420,000 t
- Military patrol: ~370,000 t (light load)
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

The reactor is **not** involved in propulsion. The ICF drive has its own
dedicated drive reactors (~25 TW total) feeding its igniter banks. The
stardrive is an entirely separate black-boxed system.

315 GW is enormously overprovisioned for 10 crew — modern human
civilization uses ~18 TW for 8 billion people. The surplus covers weapons
charge, fabrication, cryogenics, and the ship's hotel load.


## Confinement and Throttle

### The Tandem Mirror

The reactor confines its plasma in a **tandem magnetic mirror**: a straight
magnetic bottle roughly **350 m** long running down the ship's spine, with the
field intensified at each end into a magnetic "choke" that reflects escaping
particles back toward the middle. The burning plasma is a thin column about
**3 m** in radius — some **10,000 m³** at full length — held at **9 T** in the
central cell, rising to ~27 T at the end chokes (mirror ratio ≈ 3).

D-³He forces this choice. Because the fuel is nearly aneutronic, its fusion
power density is low and bremsstrahlung is always trying to outrun it, so the
plasma must run at high pressure relative to its confining field — high **beta**
(β, the ratio of plasma pressure to magnetic pressure). The mirror is a
naturally high-β machine; at the 500 GW operating point it runs **β ≈ 0.57**,
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

> P_fusion ≈ 500 GW × (L_active / 354 m)

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
| Redline | 500 GW | ~354 m (full) | Combat / peak. β riding toward 0.75; radiators saturated. Two guards lit. |
| Self-sustaining | 10–500 GW (2–100%) | ~7–354 m | Everyday range. Ignited at constant density across the whole span; overhead turns marginal below ~5% (25 GW). |
| Aux-driven / lean | 3–10 GW (0.6–2%) | at the ~7 m geometric floor | Deepest idle and the deep-cruise minimum. The column can't shorten further, so density is dropped with aux heating holding temperature. Delicate. |
| Shutdown | < ~3 GW | cold | Overhead exceeds output — cold-and-dark, cold-start to relight. |
| Overdraw | 500–700 GW | ~354 m (full) | Combat surge past rated: length maxed, density pushed up, β climbing toward the disruption limit. Thermally brutal. |

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
across interstellar transit times. A ship that could only run at its 500 GW
rating would drink its fuel in months.

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

With the ship directionally specialized, the reactor sits aft alongside the
tail radiator and the conventional engines, so the tin loop is **short** —
reactor to radiator is a few decks, not a kilometer of ~1800 K piping down
the hull. That long run was a real cost of the old flip design (radiators at
both ends, reactor amidships); specializing the ship retires it. The loop
still wants good insulation and the ~1 GW tin pump, but it no longer
threatens to bake the habitable volume along its length.

### The Tail Radiator

The ship radiates its waste heat from the **tail** — a radiating endcap of
~283,000 m² that always faces the ship's wake. At 1800 K it radiates:

> P = ε × σ × A × T⁴ = 0.9 × 5.67×10⁻⁸ × 283,000 × 1800⁴ ≈ **152 GW**

That's short of the 185 GW rated load, so the tail runs a little hotter —
~1900 K clears 185 GW, and at 2000 K it reaches 231 GW. Because the ship is
directionally specialized, the nose is a *shield*, not a second radiator, so
this single tail face does all the rejecting. That is the price of not
flipping: where the old two-endcap design had ~460 GW of in-system capacity,
the tail alone is the ceiling — and **peak heat rejection, not a spare
endcap, is what saturates first under combat overdraw**, feeding straight
into the reactor's thermal-cascade guard.

**Wake placement**: the tail always faces cold space behind the direction
of travel, so it radiates freely on every phase of the trip. No shadow
choreography, no role-swapping — the nose shields and the tail radiates,
always, even across the midcourse turnaround (the nacelles flip, not the
ship).

No deployable mechanisms. No moving parts on the radiator itself.

### Midcourse Turnaround

The ship never flips, so there is no flip thermal sequence — the old
multi-day cool-the-radiator, rebuild-the-shield ordeal is simply gone. At
midcourse the drive cuts out, the ship coasts a few minutes while the
stardrive nacelles slew 180° from accelerate-lock to decelerate-lock (see
The Stardrive), and the drive relights facing the other way. The tail
radiates throughout; the nose shields throughout. Nothing structural on the
ship itself moves, and there is no vulnerable window — the armor is
permanent, not a coating that has to be regrown.

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


## Conventional Propulsion

For in-system operations — maneuvering, combat, docking, and boosting to
stardrive catch speed — the ship uses an **inertial-confinement fusion
(ICF) pulse drive**.

Dedicated drive reactors (~25 TW) power the igniter banks — lasers and ion
beams that compress and light **deuterium fuel pellets** hundreds of times
a second. Each microdetonation is caught and turned by a magnetic nozzle.
The drive doesn't hum; it hammers. Inert LH₂ injected into each pulse
augments and cools the exhaust — which is how thrust and exhaust velocity
are traded against each other.

Deuterium is the fuel because it's cheap and everywhere. Helium-3 is far
too scarce to feed a drive — the reactor alone nearly exhausts the scoop's
³He — so the drive burns plain deuterium and pays for it in neutrons: the
D-D reactions make the drive the ship's neutron source. The nacelle throat
is shielded, activates over its service life, and dumps its neutron heat
into the thermal system.

Nothing here is to be feared in a hull breach. A hit to a fuel tank spills
cold deuterium and hydrogen. The worst the drive itself can do is stop, or
burn through a nozzle — bad, but survivable.

### Two Knobs, and Combat Lives Between Them

**Exhaust velocity (Isp).** At fixed power, throwing reaction mass *slower*
buys *more thrust* — the fundamental thrust-versus-range trade. Cruise runs
lean: high exhaust velocity, gentle thrust, best range. A fight runs rich:
low exhaust velocity, hard thrust, ruinous range.

**Power (overdraw).** The nozzles are rated for **3.2 PW** continuously.
You can run them hotter — bigger pulses, higher rep rate — at rising risk:
a **soft redline at 2× (6.4 PW)**, sustainable for minutes, and a **hard
limit at 3.5× (11 PW)**, good for seconds. Past that the exhaust plasma
overwhelms the nozzle field and burns through the physical throat, or the
nozzle coils quench. Overdraw buys thrust *without* spending your range —
you pay in nozzle life instead.

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
| ~490,000 t (max cargo) | ~2,900 km/s | ~30 days |
| ~400,000 t | ~3,700 km/s | ~24 days |
| ~330,000 t (light) | ~5,200 km/s | ~18 days |

In-system transits still take days; the ship is a kilometer long and masses
hundreds of thousands of tonnes. The drive changed; the heft did not.

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

At maximum cargo (~490 kt, ~2,900 km/s available), this leaves only ~600
km/s of margin — enough for a clean port call, but combat or unexpected
maneuvering eats into it fast. At lighter loads the margin is comfortable.

**The trade tension**: Arrive loaded with cargo, sell it, depart light. You
have plenty of Δv to leave. But if you want to depart loaded — having
bought cargo to fill the hold — you either refuel at the station or trust
your margin. Deuterium and reaction mass are cheap and available at most
ports, but topping up still costs time and money, and a lean departure is a
gamble.


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

### Nacelle-Flip Deceleration

At the midpoint of an interstellar transit the ship decelerates — but it
never turns over. Each stardrive nacelle is a **lockable two-position
turret** on its boom: nozzle aft to accelerate, nozzle forward to
decelerate. At turnaround the drive cuts out, the ship coasts for a few
minutes while the nacelles swing 180° and relock, and the drive relights
pointing the other way. The scoop keeps collecting ISM hydrogen throughout;
the exhaust, far faster than the ISM, blasts clear without being
recollected. The ship decelerates on free ISM hydrogen down to ~900 km/s,
where the stardrive hands off to the conventional engines for final
approach.

This retires the old flip-and-burn. Turning a 500,000-tonne hull over was a
multi-day ordeal; slewing two nacelles during a drive-off coast takes
minutes, and the specialized nose and tail never swap jobs. The cost is a
pair of big turret bearings and their rotary utility feeds — and a sharp
new failure mode: **a jammed turret at turnaround means you cannot
reconfigure to decelerate**, and you arrive at your destination system at a
fraction of *c* with no way to stop. See Maintenance.

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
- **Hull armor**: Needs no in-flight replenishment — the carbon Whipple hull
  doesn't ablate, and its water stuffing is sealed behind it (topped up at
  ports, not scooped). Losing the scoop no longer threatens the shield,
  only the fuel supply.
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
- **Drive deuterium**: The scoop can't skim deuterium at drive scale —
  it's only ~20 ppm of ISM hydrogen, a trickle that barely covers the
  reactor. The drive banks a modest reserve in transit and loads the bulk
  at destination, extracted from local water or gas-giant hydrogen.
  Deuterium is cheap and available almost everywhere, so the drive is never
  fuel-starved the way a scarce-fuel ship would be — but topping up is a
  routine part of every port call.

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
- **Active combat defense**: Active Debris Defense (above) handles natural
  hazards; defending against a hostile actor's weapons, saturation, and
  countermeasures is unspecified — see Weapons systems
- **Sensors and communications**: Relativistic effects on sensing
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
