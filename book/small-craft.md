# Small Craft

Couriers cross between stars and never come near a planet. Everything
else that flies in a system — shuttles, fighters, missiles, tugs — is a
small craft. This doc covers the chemical tier: craft that burn **dust**,
the same IS metal the couriers carry, as a chemical fuel — and the
cruiser, which flies on the [ICF drive](icf-drive.md).

Numbers here are first-order, from energy balances and textbook engine
physics. The dust chemistry and the hybrid engine's come from
`crates/hf-engine` (`cargo run -p hf-engine`): NASA thermodynamic data,
chemical equilibrium with dissociation (checked against Cantera), and a
cycle model of the engine.

## The Fuel

Metal that reverts to molecular hydrogen releases **216 MJ/kg** (see
[Ship Systems — The Metal](ship-systems.md#the-metal)). It does so with
no oxidizer: heat a grain past ~1000 K and it gives up its energy by
itself. In air the hydrogen it becomes can then burn as well.

| Fuel | Energy | Density | Needs |
|---|---|---|---|
| Jet fuel | 43 MJ/kg | 0.8 t/m³ | oxygen |
| Liquid hydrogen | 120 MJ/kg | 0.07 t/m³ | oxygen, cryogenics |
| **Dust, in any gas** | **216 MJ/kg** | **0.56 t/m³ packed** | nothing |
| **Dust, in oxygen air** | **~336 MJ/kg** | | oxygen for the second half |

Dust is the best chemical fuel there is, eight times jet fuel by mass,
dense, warm, and pumpable by magnets. It works in any atmosphere,
breathable or not. It is also IS's, so every shuttle and fighter is on
the same fuel leash as the couriers.

Every kilogram of it is ~50 kg of TNT waiting for heat. A light
shuttle's tanks hold a quarter of a kiloton; a heavy shuttle's, a few
kilotons.

## Dust Rockets

Pure dust reverting in a chamber runs at **~5,600 K** and gives ~1,700 s
of specific impulse (Silvera & Cole, 2010, put it near 6,000 K). No wall
likes 5,600 K. Small craft carry **ammonia** alongside the dust, for
three jobs:

- **Coolant.** Liquid ammonia runs through the walls of every hot part,
  boils, and cracks into nitrogen and hydrogen, soaking up ~12 MJ/kg on
  its way to 2,500 K. Then it is injected as propellant, so the heat is
  not lost. Its products do not attack carbon; steam would.
- **Diluent.** Mixed into the gas, it pulls the temperature down, but
  slowly. Dust carries so much energy that a 1:1 mix still runs at
  5,200 K, and holding the gas to 3,000 K takes 11 kg of ammonia per
  kilogram of dust. Above ~4,000 K much of the energy sits in hydrogen
  split into atoms, which recombine as the gas expands and give it back.
- **Fuel.** In air, cracked ammonia burns (18.6 MJ/kg), so it is not dead
  weight.

It is everywhere — outer-system ices, or made from air, water, and
power — and it stays liquid at modest pressure, with no cryogenics.

| Mix, dust : ammonia | Gas at 10 MPa | Vacuum Isp | Used for |
|---|---|---|---|
| Pure dust | ~5,600 K | ~1,700 s | military performance; needs an exotic chamber |
| 1 : 1 | ~5,200 K | ~1,200 s | the standard rocket mix, with ammonia film along the walls |
| 1 : 1.65 | 5,000 K | — | the turbine gas of the hybrid engine |
| 1 : 2 | ~4,900 K | ~1,000 s | cheap lift |
| 1 : 11 | 3,000 K | — | |

(Equilibrium temperatures; hotter at higher pressure, where less hydrogen
splits. The Isp is ~90% of the ideal of an area-ratio-100 nozzle:
1,840, 1,330 and 1,110 s.)

### Ammonia or Methane

Liquid methane could do ammonia's jobs. It is a quarter hydrogen by mass
against ammonia's sixth, it soaks up more heat cracking, it burns in air
at 50 MJ/kg against 18.6, and it is as common. Side by side in the
fighter's engine:

| | Ammonia | Methane |
|---|---|---|
| Liquid | 0.68 t/m³ at 240 K; storable under ~10 bar | 0.42 t/m³ at 112 K; cryogenic |
| For 5,000 K, per kg of dust | 1.65 kg | 0.96 kg |
| Turbopump, per MW-minute | 1.9 kg, and 1.1 kg of dust | 1.2 kg, and 1.2 kg of dust |
| Sea level, dry | 115 kN at 9,700 s | 121 kN at 12,700 s |
| Cruise | 2,200 km per tonne; 1,400 km per m³ of tank | 2,900 km per tonne; 1,430 km per m³ |
| Hover, a minute | 20 kg and 12 kg of dust | 13 kg and 13 kg of dust |
| Rocket, 1:1, vacuum, ideal | 1,330 s, clean | 1,310 s, the exhaust 30% soot |
| Cooling the turbine | cracks cleanly to gas; ~60% of its flow carries the blades' heat | cracks to soot: kept below the ~900 K where it cokes its passages, all of it could carry a fifth of the heat |

Methane is a third better by mass and no better by volume, and a fighter
runs short of volume first. It cannot cool the turbopump, which is what
the engine's temperature rests on. Cooling the blades with ammonia and
diluting with methane would save about a quarter of the diluent's mass,
for a second fluid in a cryogenic tank. Its rocket plume is sooty: two-
phase losses the ideal Isp leaves out, and a bright infrared signature.
In its favour, its gas is saturated with carbon, so hot hydrogen in it
does not eat carbon blades; and at 5,000 K, or once it burns in air, it
holds no soot.

Small craft run on ammonia. Methane suits craft short of mass rather
than volume, with no turbine to cool.

## The Hybrid Engine

Atmosphere-capable craft fly one engine in three modes. It is an
**electric air turbo-rocket**: motors drive the fan and compressor, and
the power comes from a **turbopump**, a turbogenerator that runs in gas
from the dust **reverter** and never touches air. Nothing turns in the
hot air downstream.

![The electric air turbo-rocket](img/hybrid-engine.svg)

CNT is ideal for the fan and compressor — cold, and strong enough for
very high tip speeds, so a few light stages give a high pressure ratio.
It is the wrong thing to put in hot air: carbon oxidizes above ~900 K.
The turbopump's gas is hydrogen and nitrogen, with no oxygen in it, so
its blades can be CNT too.

### The Turbopump

- **Feed**: dust slurried in liquid ammonia, pumped to ~10 MPa by the
  turbopump's own pump. More ammonia joins it at the reverter, after
  cooling the turbine's blades and the hot section.
- **Reverter**: the dust reverts, the ammonia cracks, and the gas leaves
  at **5,000 K**, with 1.65 kg of ammonia to each kilogram of dust.
  Nearly half its molecules are hydrogen atoms; they recombine through
  the turbine, and much of its work comes from them.
- **Turbine**: six stages expand the gas ~7:1, down to the main
  combustor's pressure. It is small and fast: ~7 cm in radius at
  ~190,000 rpm, its first blades 7 mm long.
- **Blades**: CNT, under an insulating refractory-carbide skin, cooled
  from inside by ammonia. The stators return their coolant to the
  reverter's feed; the rotors bleed theirs out of their trailing edges.
- **Generator**: superconducting, on the turbopump shaft, at ~100 kW/kg.
- **Exhaust**: hydrogen and nitrogen at ~4,000 K and ~1.5 MPa, into the
  main combustor.

**Each megawatt of electricity costs ~1.9 kg of ammonia and ~1.1 kg of
dust a minute.** The exhaust burns downstream, so its energy is not
lost; the ammonia is the price of the work, because the turbine can only
take so much work from each kilogram of gas.

**Why 5,000 K.** The blade's CNT survives ~3,000 K. Its skin, of the
highest-melting materials known, survives ~4,000 K. Cooling holds the
gas ~1,000 K hotter than the skin, as today's turbines run hotter than
their coatings and rockets run far hotter than their walls. Two things
make that possible:

- **The skin is non-catalytic**: hydrogen atoms that reach it do not
  recombine on it, so it does not take their energy. A catalytic skin
  would take twice the heat.
- **The reverter runs at 10 MPa**, not higher. Dense hot hydrogen
  carries heat into a surface very well: at 50 MPa the first blades
  would take ~500 MW/m², and at 10 MPa they take ~90 MW/m², what a
  rocket's throat takes. The whole turbine sheds ~9 MW into its blades,
  and ~60% of its own ammonia, heated to 2,500 K, carries that away.

Hydrogen and nitrogen are nearly transparent, so the blades take little
radiant heat. Hotter would need a skin near 5,000 K, which no known
material survives. At 3,000 K the turbine would need nearly six times
the ammonia per megawatt.

There is no reheat between the turbine's stages. Dust needs no oxidizer,
so stator vanes could inject it and lift the gas back to 5,000 K before
each rotor; at 10 MPa that saves only ~15% of the ammonia, for twice the
dust.

### The Air Path

- **Fan and compressor**: CNT blades, each rotor on its own motor at its
  own speed. Fan pressure ratio ~3.5, overall ~12, bypass ~0.4. As Mach
  rises the compressor slows, to keep its discharge below ~850 K. The
  fan's blades are variable-pitch, so it can pass full airflow at a low
  pressure ratio.
- **Main combustor**: compressed air meets the turbopump's exhaust and,
  if it has oxygen, burns it; more dust can be injected directly. No
  turbine follows it, so it can run near stoichiometric.
- **Reheat**: the bypass air joins the core flow at the bypass's
  pressure, and secondary dust and ammonia injectors burn the bypass
  air's oxygen. Mixed, the two streams give 2–4% more thrust than
  separate nozzles would.
- **Cooling**: the whole hot section is ammonia-cooled, and that
  ammonia, cracked and hot, feeds the reverter.

### Modes

| Mode | Speed | Isp, all propellant | Doors |
|---|---|---|---|
| **Turbine** | 0 to Mach ~3.5 | ~6,500–13,000 s | fan and compressor in the flow |
| **Ramjet** | Mach ~3.5 to ~5 | ~7,500–8,800 s | fan face closed; doors in the inlet duct ahead of it open into the bypass duct, which opens through more doors into the main combustor |
| **Rocket** | above Mach ~5, and in space | 900–1,700 s | all closed; the main combustor and reheat are the chamber, and the turbopump runs only for the electricity needed |

Turbine mode's best is cruise; its worst is Mach 2, where ram doubles
the airflow and the fan and compressor would want 49 MW. The ramjet holds
level flight to about Mach 4 with its burner at 3,000–3,500 K; beyond
that drag outgrows it, and Mach 6 is a dash on the rocket.

In ramjet mode the stopped compressor sits beside duct walls at
1,100–1,800 K, so its cavity is purged with cracked ammonia: hot CNT
cannot burn in it. In an atmosphere with no oxygen the engine still
works, as a pure heat-addition jet: it makes the same thrust by burning
more dust, at 70–80% of the fuel efficiency.

The rocket mode is a poor rocket. The main combustor is a low-pressure
chamber, ~2 MPa, and the aircraft nozzle opens to an area ratio of only
~5–20, so a 1:2 mix gives 900–1,000 s, not 1,100.

There is no scramjet stage. The rocket mode is good enough that one would
save only another ~10% of dust for a great deal of complexity.

**Why bother breathing air:** orbit does not need it — even a pure dust
rocket reaches orbit in one stage with a mass ratio under 2. Air
breathing roughly halves the IS dust each ascent burns, gives
atmospheric craft range and loiter, and keeps a 3,500 K hydrogen jet away
from the ground.

## Heavy Shuttle

About 100 t dry with its payload, ~190 t at liftoff. It flies the hybrid
to Mach ~6 and rockets the rest of the way to orbit, and it lands on its
rocket, so it needs a real pad: hardened, remote, with a water deluge to
take the heat and the noise. To low orbit:

| Profile | Dust per ascent | Ammonia |
|---|---|---|
| Pure dust rocket from the ground | ~87 t (~4.5 kt of TNT) | — |
| 1:1 rocket from the ground | ~72 t | ~72 t |
| Air to Mach 6, then pure dust | ~58 t | — |
| **Air to Mach 6, then 1:1** | **~46 t** | **~43 t** |

Heavy shuttles work between ports' pads and their stations. None reaches a
parked courier, tens of light-seconds out; transfer craft do that leg.

## Light Shuttle

About **20 t fully loaded**, built to land almost anywhere.

- **Lift**: two to four **deployable fans** with folding CNT blades swing
  out of bays on arms once the shuttle is subsonic. Together they sweep
  ~75 m², about three times what fans buried in the body could, so the
  downwash is ~65 m/s of cool air: rough on loose ground, but no pad, no
  deluge, no fire.
- **Drive**: the engine's turbopump feeds superconducting motors in the
  fans. With superconductors that work to ~1000 K there are no
  cryogenics and the machines are light. Differences in fan thrust hold
  attitude in the hover without reaction jets, and losing one fan is not
  fatal. Hovering takes 10–16 MW, which the turbopump makes for
  ~30–50 kg of propellant a minute, 60% of it ammonia.
- **Backup**: the rocket can land the shuttle if the fans fail, at ~1 t of
  propellant a minute and a scorched landing site.

| | Mass |
|---|---|
| Structure, engine, fans, heat shield | ~7.5 t |
| Payload: crew and cargo | ~2–3 t |
| Propellant to orbit (air to Mach 6, then 1:1) | ~10 t, half dust, half ammonia |
| **Loaded** | **~20 t** |

## Fighter

**About 5.5 t empty, 14.2 m long, with the lines of a Mirage 2000:** a
tailless delta set low, its underside flush with a flat belly; one fin,
half-cone inlets in D-shaped trunks on the flanks, one engine; a round
radome and, where the canopy would be, a low hump over the pilot's
capsule; but a body broader amidships, where the inlet ducts pass round a
lift fan. The inlet cones translate, as the Mirage's do, but much
further, to set up the inlet for the ramjet. `modeling/fighter.py`
builds it in Blender, places what it carries, and works out its volumes,
masses, balance and wave drag; `modeling/fighter_look.py` dresses and
renders it.

| | |
|---|---|
| Length, span | 14.2 m, 9.2 m |
| Wing | 45 m², a 58° cropped delta; 55 m² of planform with the body |
| Inside the skin | ~40 m³, ~144 m² of skin |
| Tanks | 12.7 m³: forward 4.3, around the engine 6.3, wing box 2.1 |
| Bays | two, in the wing roots, for four missiles |
| Empty | ~5.5 t (~8.7 t if its equipment came out as heavy as today's practice) |
| Full internal tanks and four missiles | ~14.7 t |

**From the nose back**: the radar and avionics in the radome; the
pilot's capsule under its hump, where a cockpit would be, with the nose
gear and the capsule's reservoir below it and the forward tank round it;
the inlets; the lift fan's shaft, 1.5 m across and skin to skin, 6.4 m
back, with the inlet ducts curving round it; the laser's turret just
behind it, and the laser along the spine over the engine face; the
engine, from 8.6 m to the tail, with the turbopump and its electronics
beside it, dense and so aft, and the main gear stowed under them; tanks
wrapped round all of it and in the wing box; the missile bays in the
wing roots; the swivel nozzle at the tail.

**Shape and wave drag.** The wing's section is thickest at 40% of its
chord, not half: its trailing edge is straight, so a section thick to
the back would stack the whole span's thickness in the last two metres
and pile area on the tail. Measured from the model's cross-sections, the
Mach-1 wave drag is ~1.9 m² of D/q, 1.75 times a Sears-Haack body's of
the same length and volume, a little better than a usual fighter's. It
is high in absolute terms because the fighter is fat for its length,
~40 m³ inside to a Mirage's ~25, and wave drag goes with the square of
volume; the volume is the tanks. Area ruling harder costs tank faster than it saves drag:
a 4.5% wing would save little and lose ~0.56 m³ of tank and the bays;
moving the saddle tanks forward to slim the tail lost 1.5 m³ for almost
nothing. The mission sim takes the measured figure.

**The pilot rides a capsule, not a cockpit.** There is no canopy: the
pilot sees through the sensors and speaks through a BCI. The capsule,
~0.6 m³ and ~0.5 t with its pilot, holds ~0.25 m³ of an oxygenated liquid
of the body's own density, which the pilot breathes. With no air in the
lungs or anywhere else in the body, high g has nothing to collapse;
bone, denser than the rest, is what is left to limit it, near 20–25 g
sustained. The liquid needs no pressurisation, and life support is a
loop through an oxygenator and a CO₂ scrubber. The pilot boards through
a hatch in the spine, by a built-in ladder, with no one to help: the
capsule fills, takes the pilot through the change to liquid breathing,
and on landing drains to a reservoir, tilts the pilot head down and
clears the lungs. The liquid has to clear fast by itself, or a pilot
alone on a colony world is in trouble. The capsule is also the ejection
seat: it leaves upward through the hatch, and someone in liquid survives
a far harder kick than someone strapped to a seat.

**Structure.** Near-ideal CNT, ~30 GPa in tension and ~8 GPa in
compression, knocked down by a third, sized from the loads by
`modeling/fighter_structure.py`: 20 g limit and 30 g ultimate at 15 t,
less when heavier. The loads almost never ask for more than minimum
gauge, 0.6 mm (1 mm on the belly, against gravel off dirt strips): the
wing root would need 0.36 mm. The primary structure is ~0.3 t; the rest
of the airframe's mass is its carbide skin, its insulation, its gear and
the mounts for its equipment. What sets it is minimum gauge: 2 mm
everywhere would add ~0.8 t.

The 20 g is the instantaneous turn's: lifting 20 g at ~15 t takes
~70 kPa of dynamic pressure, Mach ~1 at sea level or ~2 at 11 km, and
there the drag is several times the engine's thrust, so the fighter
sheds speed at ~5 g while it pulls. Sustained turns are thrust-limited,
~6–8 g. At 20 g in a fast pull the rotors twist, the turbopump at
~190,000 rpm and the fan and compressor with it: their bearings and
mounts are sized by gyroscopic loads.

Its equipment, from `modeling/fighter_equipment.py`, is mostly electric
machines and what feeds them: with near-ideal materials, rotors, casings
and walls are set by minimum gauge and aerodynamics, and come out light.

| Empty | Mass |
|---|---|
| Engine: motors for ~45 MW (~450 kg), fan, compressor, casings, hot section, nozzle | ~0.75 t |
| Turbopump and its 50 MW generator | ~0.57 t |
| Power electronics: ~125 MW of conversion | ~0.53 t |
| Capsule and pilot | ~0.48 t |
| Lift fan and its 30 MW motor | ~0.45 t |
| Landing gear, for dirt | ~0.42 t |
| Laser and its turret | ~0.53 t |
| Inlets, ducts, swivel nozzle | ~0.2 t |
| Primary structure and tank walls | ~0.37 t |
| Skin coating, insulation, equipment mounts | ~0.45 t |
| Radar, avionics, flight controls, RCS, thermal, electrical, propellant system, bays | ~0.75 t |
| **Empty** | **~5.5 t** |

The equipment is the uncertain part: built to today's practice, with
heavier gauges, bearings, accessories and margins, it would weigh ~3 t
more.

| Engine | |
|---|---|
| Airflow | ~90 kg/s |
| Thrust, sea level | ~115 kN dry, ~135 kN with reheat |
| Fan and compressor | 26 MW at full thrust at sea level; ~49 MW at Mach 2 |
| Turbopump | 50 MW: full thrust at Mach 2, or full thrust at sea level with the laser |
| Electric machines | ~1.25 t: generator ~500 kg, engine motors ~450 kg, lift-fan motor ~300 kg |

**It lifts the way the F-35B does**: an electric ~1.5 m fan behind the
nose, with doors above and below and a vane box under it, and the main
engine's nozzle swivelled straight down on a three-bearing duct. Its
motor is rated 30 MW: it lifts ~122 kN, at the F-35B fan's disk loading,
and the nozzle another ~67 kN, so the fighter hovers at up to ~19 t,
fully fuelled.

| In the hover, at ~11.5 t | Thrust | Power |
|---|---|---|
| Lift fan | ~75 kN | ~14 MW |
| Engine, nozzle down | ~40 kN | ~5 MW |
| **Total** | **~115 kN** | **~20 MW, ~20 kg of ammonia and ~12 kg of dust a minute** |

In the hover the engine opens its fan's pitch to a pressure ratio of
~1.3 at nearly full airflow and runs its compressor at ~1.5, so the
nozzle passes ~80 kg/s at ~950 K and ~490 m/s. At full pressure ratios
it would make the same thrust from a quarter of the air, at ~3,200 K;
each rotor on its own motor at its own speed, with its own pitch, is
what makes the difference.

- **No shaft, no clutch.** The F-35B drives its fan from the engine
  through a driveshaft and clutch; this fan is wired to the turbopump.
  Pitch in the hover is a change of motor torque, in milliseconds.
- **Roll** comes from the wingtip thrusters the fighter uses in space, as
  the Harrier used wingtip puffers. Yaw comes from the fan's vanes and
  the nozzle.
- **Downwash**: ~190 m/s of cool air from the fan, and a ~950 K jet
  from the nozzle. The fighter lands on a pad, or on ground it can afford
  to scorch.
- **A fan or the turbopump lost in the hover** is a hard upset, as it was
  for every lift-fan aircraft. The capsule goes.

**Balance.** The fighter is unstable by design and flown by wire, so its
centre of gravity is set for trim drag, not stability. Subsonic, the
wing's lift acts ~8.8 m back; supersonic and hypersonic, near the
planform's centroid, ~9.4 m. Full, the centre of gravity is at ~8.6 m;
empty, ~8.9 m. The forward tank, ~3 t of propellant 4.2 m ahead of the
centre of gravity, is the trim tank, as Concorde's aft tanks were:
draining it moves the centre of gravity aft by ~1.1 m, for supersonic
flight and for entry, and filling it moves it forward, which the hover
likes. The rest of the tankage sits at or behind the centre of lift, and
dust and ammonia can be moved between tanks in flight. In the hover the
nozzle carries 0.4–0.85 of the fan's thrust, depending on the load.

**Weapons.** The laser, and four dust missiles, ~90 kg each, 2.4 m by
0.18 m, two to a bay in each wing root, reaching ~200 km. Nothing hangs
outside: pylons in the flow would burn on entry and cost cruise. Stealth
missions carry the laser alone. In air the laser is a line-of-sight
weapon, strongest in clear high air and crippled by cloud and low
altitude; the missiles work in any weather and past the horizon. In
space the laser is heat-limited: it cannot fire continuously.

- **Range**: at Mach 0.9 it cruises on ~0.12 kg/s of propellant, its fan
  at full ratio and its compressor idle: about 2,200 km per tonne, three
  times a Mirage 2000's. On internal tanks that is thousands of
  kilometres.
- **Rocket mode**: ~3 km/s on a few tonnes of internal propellant — zoom
  climbs, suborbital hops, a strike from above the atmosphere — and
  orbit on full tanks.
- **Ramjet mode**: level flight to about Mach 4, and Mach 6 briefly
  on the rocket. The delta is a sound shape for it; the airframe's heat
  is the limit.

### Down and Back

The fighter's hardest mission: from orbit, down through entry, a sortie
in the air, and back up to orbit on what it brought down, with no help
on the ground. It may land on 300 m of dirt and take off again, or never
land at all and climb back out from flight. It flies it clean but for
its laser. `crates/hf-fighter` (`cargo run -p hf-fighter`) flies it:
entry and ascent over a rotating planet, the engine from `hf-engine`,
and the strip.

**The tanks** hold **dust shot with ammonia in the gaps**: two sizes of
grain packed to 80%, 0.56 t/m³ of dust, the gaps a fifth of the space.
The engine draws dust and ammonia separately, each at the rate its
regime wants. Everything is inside: tanks hung under the wings would sit
in the hottest flow, and the shocks off their pylons would burn
through, as the X-15A-2's nearly did at Mach 6.7.

**The sortie budget.** What the mission is worth is how much the
fighter can spend down in the air before it must climb back: a planner
always wants more. On an Earth-like world, with its 12.7 m³ full, the
climb back costs ~4.7 t of propellant from a 10.3 t start, and the
deorbit burn a little; **~3.9 t is left for the sortie**, some nine hours
of subsonic cruise.

| | Mass |
|---|---|
| In orbit, empty, with a 150 kg reserve | ~5.6 t |
| Climb back to orbit, from the ground or from flight | ~4.7 t |
| Sortie budget | ~3.9 t |
| Entry, budget unspent | ~14.2 t, ~315 kg/m² of wing |

It loads the climb's propellant aft first and keeps the forward tank for
the sortie, so that it enters with its centre of gravity near the
hypersonic centre of lift and its elevons nearly neutral, out of the
worst heat.

**The climb.** Turbine mode to Mach 0.95 at 10 km, ~0.1 t in a minute
and a half; then the rocket: 250 kN from the main combustor as a 4 MPa
chamber, through the nozzle opened to an area ratio of ~18 in a 0.6 m²
exit. It burns dust-rich, about one part ammonia to seven of dust, at
~1,430 s and a ~5,200 K chamber: ~4.7 t over about three minutes and
the burn to circularise at 200 km, 8.5 km/s in all, peaking near 3.8 g.
At 4 MPa a first-order Bartz estimate puts the throat at a few tens of
MW/m², well short of a rocket's usual.

The ramjet does not pay on the way up. Against a rocket of ~1,430 s it
wins only by a little, and the fighter's wave drag takes that: the
climb optimiser leaves the air at Mach 0.95 at the measured ~1.9 m² of
D/q, goes on to Mach 2.2 at ~1.4 m², and only below ~1 m² rides the
ramjet to Mach ~4.5 at 23 km first, which would save ~0.5 t. A fighter
area-ruled that well would be longer, or carry less.

**Entry.** A 50 m/s deorbit burn; then belly-first at 40° angle of
attack, banking to hold an equilibrium glide, as the Shuttle did. At
that angle the fin is in the wing's shadow, and the reaction jets give
yaw until the fighter is low and slow; a Mirage 2000 flies to 30°.
With the budget unspent, peak heating is near 70 km and 6.2 km/s, ~1.1 g
at worst, ~50 minutes and up to 17,000 km of glide to Mach 2:

| Surface | Peak |
|---|---|
| Nose, 10 cm radius | ~2,150 K, ~1 MW/m² |
| Wing leading edge, 3 cm | ~2,100 K |
| Belly, 1 m back | ~1,350 K |
| Belly, 6–10 m back | ~1,100 K (~1,450 K if its flow trips early) |
| Back | ~500 K |

The blade skin, good to ~4,000 K, non-catalytic and proof against hot
oxygen, makes it a hot structure that needs nothing after a flight: no
ablator, no tiles. It even leaves the fighter its sharp edges; a 3 cm
nose would run ~2,400–2,500 K. At 30° the nose runs ~100 K hotter. The
climb is gentler, the nose peaking near 1,800 K. The belly's bay doors
and gear doors seal against hot gas, as the Shuttle's gear doors did.
Behind 5 cm of insulation, ~8 kg/m², a belly tank takes ~1.5 MJ/m²
through the whole entry and the hours after, which boils off ~15–20 kg
of its ammonia.

**The strip.** At 10.3 t, and at anything up to ~19 t, the fighter hovers
off it and lands on it vertically: the 30 MW lift fan and the nozzle
carry its weight with the engine on the rest of the turbopump.

**Harder worlds.** On worlds of Earth's density, heavier and larger,
with Earth's air at the surface thinning faster with height:

| Surface gravity | Orbital speed | Sortie budget | Nose on entry | Landing |
|---|---|---|---|---|
| 1.0 g | 7.8 km/s | ~3.9 t | ~2,150 K | vertical |
| 1.1 g | 8.6 km/s | ~3.2–3.3 t | ~2,300 K | vertical |
| 1.2 g | 9.4 km/s | ~2.5–2.6 t | ~2,400 K | vertical |
| **1.3 g** | **10.2 km/s** | **~1.6–1.8 t** | **~2,550 K** | **vertical** |
| 1.4 g and up | 11.0 km/s and up | none: the climb alone overfills the tanks | | |

Landing or not changes the budget by only ~0.1–0.2 t: the climb from
the ground is cheap next to the rocket's. **About 1.3 g is the practical
limit**, with a few hours of work down in the air; beyond it the fighter
goes down one way, or refuels on the ground. Entry never needs a braking
burn while the climb is possible: the nose stays under ~2,600 K to
1.3 g.

**Water in the gaps.** Water is denser than ammonia, 1.0 t/m³ against
0.68, and the skin takes hot oxygen anyway, so it was tried in the
gaps, pure and as ammonia-water, holding the volume fixed (at an earlier,
heavier estimate of the fighter, 8.3 t empty, with a 0.5 t sortie):

| Liquid | Down and back takes | Takeoff | All 12.7 m³ on the rocket | Cruise, per m³ | Freezes |
|---|---|---|---|---|---|
| Ammonia | 10.2 m³ | 14.6 t | 9.7 km/s | 1,300 km | 195 K |
| 33% ammonia-water | 10.0 m³ | 15.0 t | 9.7 km/s | 1,370 km | 173 K |
| Water | 9.9 m³ | 15.2 t | 9.8 km/s | 1,380 km | 273 K |

Water buys ~3% of volume and ~1% of rocket Δv, for ~0.6 t more at
takeoff: in the air it is dead weight where ammonia is a fuel, and the
turbopump needs ~25% more of it per megawatt. The mission has volume to
spare, and mass is what binds, so the fighter carries ammonia. Water
also freezes in a cold tank, and steam eats carbon in the cooling
passages unless they are skinned too.

On a world harder than ~1.3 g, more than ~10 km/s to orbit, there is no
down and back: it goes down one way, or refuels on the ground.

## Logistics and Austere Operations

The bulk consumable is **ammonia**, not dust: a fighter sortie burns
about a tonne of dust and a couple of tonnes of ammonia. A forward base
can make ammonia from air, water, and power; in a pinch the engines run
on water as the diluent, at the cost of turbine coating life.

The dangerous consumable is the dust. A fighter's tonne is ~50 t of TNT;
a forward base holding 100 t holds ~5 kt. Dust stores are cold, buried,
spread out, and guarded, and they are the first thing an attacker looks
for.

And all of it comes from IS. A system whose depot runs dry grounds its
shuttles and fighters along with its couriers.

## Vacuum Combat

Fights between small craft are short, local, and decided at range.

- **There are no dogfights.** Engagements open at hundreds to thousands
  of kilometres. An FEL's light crosses 1,000 km in 3 ms; nothing dodges
  it. A craft survives a laser by being out of range, unseen, or armored.
- **Seeing is the first fight.** See [Sensors and Stealth](sensors.md).
- **Gees are for geometry.** High acceleration changes where you will be
  before the shooting starts, and spoils a missile's last seconds. It
  does nothing against a beam.
- **Δv is the budget, and it is small.** A dust rocket carries 10–20
  km/s. Crossing a system takes tens of km/s and months, so small craft
  fight near a planet, a station, or a carrier, where relative speeds are
  low. A fighter keeps half its Δv to get home; a missile spends all of it
  in one pass.
- **The tanks are the weak point.** An FEL need not reach anything vital.
  Heat a dust tank past ~1000 K and the craft destroys itself. Every
  small craft insulates and armors its tanks first, and missiles, which
  cannot, die easily.
- **A person decides.** The Compact forbids sub-threshold AIs to make a
  lethal decision ([History](history.md#sub-threshold-ais)), and
  light-lag keeps that person close: a pilot a light-second from the
  fight is a second behind it. Sub-threshold systems fly, aim, and track;
  a crew member pulls every trigger.

### FEL Ranges

A beam spreads with distance, so the director's aperture sets the range:

| Craft | Director | Beam | Lethal to soft targets | Blinding |
|---|---|---|---|---|
| Fighter | ~0.5 m | ~2 MW | ~100–300 km | ~1,000 km |
| Gunboat | ~1.5 m | ~20 MW | ~1,000 km | several thousand km |

Power is what separates the sizes. A fighter's FEL draws ~8 MW from its
turbopump, and ten minutes of fire costs ~150 kg of ammonia and ~90 kg
of dust. A gunboat carries an annihilation APU, like a courier's, for
endurance.

## The Anti-Dust Coilgun

Anti-grains superconduct, so a field gradient pushes them without
touching them. A ~10 T coilgun takes a 4 mm anti-grain to ~20 km/s in
about 10 m. It must be a coilgun: an anti-grain that touched a rail
would annihilate in the barrel.

- **Each grain is ~1 kt** when it meets matter, against ~1 kg of TNT for
  the same grain of plain dust at that speed.
- **It cannot be shot down cheaply.** An FEL heats the grain, it reverts
  to gas, and the gas keeps coming, spreading as it flies. Caught at
  100 km, the cloud is tens of kilometres across by arrival and harmless.
  Caught close, the target still takes the kiloton.
- **The magazine is a bomb.** 100 g of anti-dust is ~4,300 rounds and
  ~4 Mt, held off the walls by a magnet. A gunboat that loses its
  magazine's field is gone, and so is everything near it.

## Missiles

Dust rockets with no crew: mostly propellant, pure dust, hundreds of gees.

| | |
|---|---|
| Mass | ~0.2–2 t |
| Mass ratio | 5–10 |
| Δv | ~25–40 km/s, all spent in one pass |
| Acceleration | 50–200 G |
| Guidance | radar, IR, optical; a crew member designates the target at launch |

- **The fuel is the warhead.** Leftover dust reverts on impact at
  216 MJ/kg, and above ~20 km/s of closing speed the impact itself brings
  more. Plain missiles disable.
- **Some carry an anti-dust warhead**: a grain or a few at the nose, in a
  tiny trap. One grain is 1 kt, and its gamma flash gives an unshielded
  crew ~50 Gy at 1 km, so a near miss kills. The trap needs power the
  whole time: **a hot missile must stay plugged in**, and one whose trap
  fails goes off in the magazine.
- **Getting through is the problem.** Crossing a gunboat's 1,000 km kill
  range at 30 km/s takes ~33 seconds under fire. Missiles work by
  numbers, decoys, and insulated tanks.
- **Targets are often easy to see.** A burning courier is a gamma lamp
  across the system, and dust engines leave bright plumes. Hard targets
  run cold, with their radiators edge-on.

## Space-Superiority Fighter

| | |
|---|---|
| Mass | ~13 t loaded, ~6 t dry |
| Crew | one |
| Propellant | ~6 t of pure dust (~300 t of TNT), ~1 t of ammonia |
| Δv | ~10 km/s on the rocket, ~90 m/s of cold gas |
| Acceleration | 3–5 G sustained, 9 G briefly |
| Power | dust-fired turbogenerator |
| Weapons | FEL (~2 MW beam), 4–6 missiles |
| Endurance | hours |

Short-range and fast, it fights near its base or its carrier: screening
stations and ports, killing missiles, and contesting space against other
fighters. Its Δv covers one engagement and the trip home. Its ammonia
lets it run dark for a few hours and jink on cold gas (see
[Sensors — Stealth](sensors.md#stealth)).

## Patrol Gunboat

| | |
|---|---|
| Mass | ~300 t loaded, ~90 t dry |
| Crew | ~5, plus ~8 marines |
| Propellant | ~200 t of pure dust (~10 kt of TNT), ~10 t of ammonia |
| Δv | ~18 km/s on the rocket, ~35 m/s of cold gas |
| Acceleration | 1–2 G |
| Power | annihilation APU (~50 MW thermal, ~15 MW electric) for endurance; dust turbogenerator and capacitors for the FEL |
| Weapons | FEL (~20 MW beam), ~20 missiles, anti-dust coilgun |
| Endurance | weeks |

The gunboat is the working warship of a system: patrol, customs,
quarantine, escort, and anti-piracy. Its marines board **shuttles and
in-system craft**, matching speed and going aboard through a lock or the
hull. Interstellar couriers are not boarded in the ordinary way — their
engines and stores make that a different kind of operation.

Its staying power is the APU: weeks of hotel and sensor power on grams of
antimatter, so it can loiter where a fighter would have to go home. Its
risk is its magazine.

## Cruiser

A scaled-up gunboat with an **ICF drive** and a **hangar** for fighters
and a few gunboats: a ring 200 m across with three pods riding it, ~5 kt
dry and ~9 kt loaded. The drive gives it the hundreds of km/s a system
takes — 1 AU in ~24 days — and its small craft give it the gees and the
last few km/s of a fight. Its drive's FELs double as a main battery that
reaches ~10,000 km. See [ICF Drive](icf-drive.md#the-cruiser).

## Fighting a Courier

A courier cannot dodge, but it is a hard thing to approach:

- **A burning courier is surrounded by a kill zone.** Its 180 TW of gammas
  give an unshielded crew a lethal dose in ~8 seconds at 100 km, ~13
  minutes at 1,000 km, and about a day at 10,000 km. Shielding against
  200 MeV gammas is too heavy for a small craft.
- **The way in is a plate's shadow.** Each pod's plate casts a dark cone
  outward, ~1.2 km wide at 100 km. An attacker riding it arrives at the
  pod's far end. Otherwise, wait until the courier throttles down near a
  port.
- **Its exhaust leads while it brakes.** An arriving courier points a
  0.28c particle beam at what lies ahead. Nobody attacks it head-on.
- **It is only catchable near the ends of a crossing.** Braking at
  ~2.8 mG it sheds its last 15 km/s over about two weeks and ~10 million
  km on the way to its parking point, and departs the same way. Those
  corridors are where escorts fly and pirates wait.
- **You disable a courier, you do not destroy it.** Its antimatter store
  is lethal out to several light-seconds, attackers included. The targets
  are the ring's tile and helium segments, which throttle the engine, the
  sensor packages, and the fins. The store is everyone's hostage.

## Open Questions

- Chamber design for pure dust at ~6,000 K.
- A real heat-transfer model of the turbopump's blades: the first-order
  one puts ~90 MW/m² on the first stage, and the skin's catalytic
  behaviour doubles or halves it.
- The fighter's drag above Mach 2 and its inlet's size, which decide
  where the ramjet stops holding level flight: settled with the full
  aircraft design.
- The fighter's equipment beyond first order: built to today's practice
  it would weigh ~3 t more, and every tonne of it costs the sortie
  budget ~0.8 t.
- The breathing liquid: the body's density, and lungs that clear of it
  fast.
- Takeoff from dirt: debris thrown by the lift fan and nozzle, and hot
  gas ingested, neither modelled.
- The fighter's airframe heating at Mach 6.
- Antimatter-boosted dust (~3,000 s, gas core, a gamma lamp) and an
  open-ended fusion mirror for in-system craft.
- Armor and tank insulation against FELs, worked through.
