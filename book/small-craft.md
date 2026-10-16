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

**About 6.4 t empty, 20 m long, a eurodelta:** a 68° delta set low, its
underside flush with a flat belly, close-coupled canards on the intake
trunks as on the Rafale; one fin with a pod at its tip; half-cone inlets
on splitter plates standing off the flanks; one engine; a long round
radome and, where the canopy would be, a low hump over the pilot's
capsule; a body broader amidships, where the inlet ducts pass round a
lift fan. The inlet cones translate, as the Mirage's do, but much
further, to set up the inlet for the ramjet. `modeling/fighter.py`
builds it in Blender, places what it carries, and works out its volumes,
masses, balance and wave drag; `modeling/fighter_look.py` dresses and
renders it.

| | |
|---|---|
| Length, span | 20 m, 8.2 m |
| Wing | 55 m², a 68° cropped delta, slatted; 70 m² of planform with the body |
| Canards | ~6 m², all-moving, 50° |
| Inside the skin | ~54 m³, ~195 m² of skin |
| Tanks | 24.7 m³: forward 5.7, middle 7.4, around the engine 9.5, wing box 2.2 |
| Bays | two, in the wing roots, for four missiles |
| Empty | ~6.4 t (~10.1 t if its equipment came out as heavy as today's practice) |
| Full internal tanks and four missiles | ~24 t |

**From the nose back**: the radar and avionics in the radome; the
pilot's capsule under its hump, where a cockpit would be, with the nose
gear and the capsule's reservoir below it and the forward tank round it;
the inlets; the lift fan's shaft, 1.5 m across and skin to skin, 8.3 m
back, with the inlet ducts curving round it; the middle tank behind it,
holding in its upper part the space decoys under dorsal doors and in its
lower the jammers, dropped through the belly; the laser's dorsal
director, and the laser along the spine over the engine face, with the
ventral director stowed under it; the engine, from 13 m back, with the
turbopump and its electronics beside it, dense and so aft, and the main
gear stowed under them; tanks wrapped round all of it and in the wing
box; the missile bays in the wing roots; the chaff and flare dispensers
flanking the engine; the towed decoys in the fin root, the jammer's
antennas in the fin-tip pod; the swivel nozzle at the tail.

**Why 20 m.** Wave drag goes with the square of the volume over the
fourth power of the length. At the Mirage's 14.2 m, the fighter held
12.7 m³ of tanks in ~40 m³ inside, fat for its length, and its Mach-1
wave drag was ~1.9 m² of D/q; slimming it to area-rule it cost tank
faster than it saved drag. Stretched to 20 m, with the same sections,
wing and equipment, it holds 24.3 m³ and its wave drag halves to
~1.0 m², twice a Sears-Haack body's of the same length and volume. The
stretch costs ~0.15 t: with near-ideal materials the longer skin and
structure are minimum gauge too. It nearly triples the sortie budget.
The wing's section is thickest at 40% of its chord: its trailing edge is
straight, so a section thick to the back would stack the whole span's
thickness in the last two metres and pile area on the tail. The mission
sim takes the wave drag measured from the model's cross-sections: ~1.0 m²
of D/q as the airframe stands.

**The wing.** At 45 m² the wing was loaded to ~490 kg/m² just after
entry, worse than an F-16; it is 55 m², loaded ~290 kg/m² mid-sortie, a
Mirage 2000's. Bigger still turned better subsonic but held less
supersonic, its volume adding wave drag, and at 75 m² the fighter could
not hold 1 g at Mach 1.5 and 11 km. Its sweep came from a sweep of
sweeps, 52° to 72° at the same area: less sweep, more aspect ratio,
turns better subsonic and sustained; more sweep turns better supersonic
and instantaneously, carries less wave drag, and heats its leading edge
less on entry. The sortie budget hardly cared. The usual price of high
sweep, a nose-high, fast landing, is nothing to an aircraft that lands
on its lift fan. 68° is the trade, at an aspect ratio of 1.2: ~20% less
sustained turn subsonic than at 58°, ~30% more supersonic, ~20% more
instantaneous. Two slat segments a side, set by the flight computer,
hold the flow on the leading edge to a higher angle of attack, which
keeps the drag due to lift down in a hard turn.

**Canards.** Close-coupled, all-moving, on the intake trunks just behind
the lips, ~6 m², 11% of the wing, as on the Rafale and the Typhoon. A
tailless delta trims with its elevons, which push down when the centre
of gravity is ahead of the wing's lift and cost lift and drag; canards
trim lifting, and their vortices keep the wing's flow attached to a
higher angle. They let the aircraft be balanced well unstable, as the
eurodeltas are, the flight computer flying it and fuel moved between
tanks setting the centre of gravity for each regime. Against the same
aircraft tailless, with strakes on its intakes and the same freedom to
move its fuel, they buy ~5% more turn subsonic, sustained and
instantaneous, and ~15% more instantaneous supersonic, for ~0.18 t and
~0.3 t of sortie budget. At 4 m² they were too small: supersonic, with
the wing's lift moved aft, they stalled before the wing did. In entry
they turn edge-on to the flow.

**Turning**, mid-sortie at 16 t, from `hf-fighter --turn`:

| | Instantaneous | Sustained |
|---|---|---|
| Mach 0.5 at 3 km | ~6.9 g, 23°/s | ~3.1 g |
| Mach 0.9 at 3 km | ~18.8 g, 36°/s, the structure's limit | ~5.5 g |
| Mach 0.9 at 10 km | ~8.4 g, 18°/s | ~2.3 g |
| Mach 1.5 at 11 km | ~17.4 g, 22°/s | ~3.5 g |
| Mach 2 at 15 km | ~16.5 g, 16°/s | ~3.0 g |

Light, at 11 t, it reaches the 27 g its structure allows at that weight
at Mach 0.9 low down, and sustains ~8 g. The turns are bound by thrust
and by a low-aspect delta's drag due to lift, not by the pilot. The
maximum lift coefficients behind the instantaneous turns, ~1.6 subsonic
and ~1.4 supersonic, trimmed, are estimates for CFD to settle.

**The intakes** are the Mirage's: a half-round capture with a half-cone
shock body, its flat side a splitter plate standing 7 cm off the flank.
The air slowed against the forebody's skin, ~7 cm deep 7 m back at
Mach 1.5 (~0.37 x/Re^0.2), passes between plate and fuselage instead of
into the engine, and a diverter wedge behind the lip carries the intake
and spills that air above and below. The capture and the lip are all
the intake shows the flow: a blunt face round it would be drag, and a
step in the area distribution.

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
| Laser and its two directors | ~0.65 t |
| Inlets, ducts, swivel nozzle | ~0.2 t |
| Primary structure and tank walls | ~0.5 t |
| Skin coating, insulation, equipment mounts | ~0.52 t |
| Radar, avionics, sensor windows, flight controls, RCS, thermal, electrical, propellant system, bays | ~0.75 t |
| Canards and slats | ~0.22 t |
| Countermeasures and space decoys | ~0.38 t |
| **Empty** | **~6.4 t** |

The equipment is the uncertain part: built to today's practice, with
heavier gauges, bearings, accessories and margins, it would weigh ~3.7 t
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
and the nozzle another ~55–67 kN, so the fighter hovers at up to
~18–19 t; with every tank full, ~23 t, it rolls or fills partly.

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
wing's lift acts ~12.3 m back, supersonic ~13.0 m; the canards bring the
whole aircraft's ~0.65 m forward of that. Full, the centre of gravity is
at ~11.7 m; empty, ~12.3 m. The forward tank, ~3.9 t of propellant
~6 m ahead of the centre of gravity, is the trim tank, as Concorde's
aft tanks were: draining it moves the centre of gravity aft by ~1.1 m,
for supersonic flight and for entry, and filling it moves it forward,
which the hover likes. Dust and ammonia can be moved between any of the
tanks in flight. In the hover the nozzle carries 0.45–0.75 of the fan's
thrust, depending on the load.

**Seeing.** The pilot sees all round, as an F-35's does: eleven
wide-angle imaging and infrared windows, each seeing 65° either side of
where it looks, cover every direction, the airframe's shadow included;
two low on the tail look under the long wing; the four on the nose are faceted,
angled forward, the nose being too slender for a flush window to see
ahead, and the tail is watched from the fin-tip pod. The same windows
warn of missiles, by the heat of their motors and of their noses. Each
of the laser's two 0.5 m directors is also its telescope, near a
microradian, which is a few centimetres at 20 km above the turbulence
and the beam's own adaptive optics clearing what is below it: the
dorsal one reaches the whole sky above and a sixth of the sky below,
the ventral one, let down from the belly, the whole of the ground, so
the fighter strikes the ground without rolling over to look.

**Countermeasures in air**, where the engine cannot be hidden: a jammer
with its antennas in the fin-tip pod, as on the Mirage; two dispensers,
~120 cartridges of chaff and flares; six expendable jammers, ~1 m and
12 kg, dropped through the belly to fly on batteries ahead of or beside
the fighter, small cousins of today's MALD; and three jammers towed on
fibre out of the fin root, above the plume. Together ~0.3 t.

**Decoys in space.** Four, ~20 kg each, fired from under dorsal doors
in any direction while the fighter burns. Each flies a burn like the
fighter's: the rocket's particle-laden plume leaves a 0.6 m² exit near
2,300 K, ~1 MW as a black body and of order 10 MW from the plume
beyond, so a decoy burns ~3 kg of dust for light over a minute; ~2 kg
more drives its own small engine at the fighter's exhaust speed, so it
accelerates at the fighter's ~3 g, 1.8 km/s in the minute; and it
unfolds ~1.5 kg of film to the fighter's size, with a radar repeater,
for radar and lidar. At range, where the plume is a point, it is the
fighter. Up close its plume is too small and its spectrum not quite
right, and every burn with decoys shows a watcher more of the
difference: they buy a few burns, not a campaign.

**Weapons.** The laser, its main weapon against the ground as in the
air, through either director; and four dust missiles, ~90 kg each,
2.4 m by 0.18 m, two to a bay in each wing root, reaching ~200 km.
Nothing hangs outside: pylons in the flow would burn on entry and cost
cruise. Stealth missions carry the laser alone. In air the laser is a
line-of-sight weapon, strongest in clear high air and crippled by cloud
and low altitude; the missiles work in any weather and past the
horizon. In space the laser is heat-limited: it cannot fire
continuously. The ventral director rides behind belly doors through
entry, out of the windward flow.

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
always wants more. On an Earth-like world, with its 24.7 m³ full, the
climb back costs ~5.4 t of propellant from an 11.9 t start, and the
deorbit burn a little; **~11.5 t is left for the sortie**, a day of
subsonic cruise.

| | Mass |
|---|---|
| In orbit, empty, with a 150 kg reserve | ~6.5 t |
| Climb back to orbit, from the ground or from flight | ~5.4 t |
| Sortie budget | ~11.5 t |
| Entry, budget unspent | ~23.4 t, ~420 kg/m² of wing |

It loads the climb's propellant aft first and keeps the forward tank for
the sortie, so that it enters with its centre of gravity near the
hypersonic centre of lift and its elevons nearly neutral, out of the
worst heat.

**The climb.** Turbine mode with reheat to Mach ~2.2 at 15 km, its
compressor unloading as the air warms, ~0.4 t in six minutes. Then the
rocket: 250 kN from the main combustor as a 4 MPa chamber, through the
nozzle opened to an area ratio of ~18 in a 0.6 m² exit. It burns
dust-rich, about one part ammonia to seven of dust, at ~1,430 s and a
~5,200 K chamber: ~5.1 t over four and a half minutes and the burn to
circularise, 8.1 km/s in all, peaking near 3.7 g. At 4 MPa a first-order
Bartz estimate puts the throat at a few tens of MW/m², well short of a
rocket's usual.

The ramjet pays only by a little against a rocket of ~1,430 s, and
drag decides it: at the 14.2 m airframe's ~1.9 m² of wave drag the
climb left the air at Mach 0.95; at ~1.0 m², before trim was in the
model, it rode the ramjet to Mach ~4 first; with the canards' trim drag
it leaves the air at Mach 2.2. A tenth of a square metre of drag either
way tips it, and the difference is ~0.3 t.

**Entry.** A 50 m/s deorbit burn; then belly-first at 40° angle of
attack, banking to hold an equilibrium glide, as the Shuttle did. At
that angle the fin is in the wing's shadow, and the reaction jets give
yaw until the fighter is low and slow; a Mirage 2000 flies to 30°.
With the budget spent, peak heating is near 70 km and 6.2 km/s, ~1.1 g
at worst, ~50 minutes and up to 17,000 km of glide to Mach 2; with it
unspent, ~23 t, the nose runs ~2,220 K:

| Surface | Peak |
|---|---|
| Nose, 10 cm radius | ~2,060 K, ~0.9 MW/m² |
| Wing leading edge, 3 cm, at 68° | ~1,980 K |
| Belly, 1 m back | ~1,350 K |
| Belly, 6–10 m back | ~1,100 K (~1,450 K if its flow trips early) |
| Back | ~500 K |

The blade skin, good to ~4,000 K, non-catalytic and proof against hot
oxygen, makes it a hot structure that needs nothing after a flight: no
ablator, no tiles. It even leaves the fighter its sharp edges; a 3 cm
nose would run ~2,400–2,500 K. At 30° the nose runs ~100 K hotter. The
climb, leaving the air at Mach 2.2, arcs its rocket burn lower and runs
the nose to ~2,250 K, hotter than entry. The belly's bay doors
and gear doors seal against hot gas, as the Shuttle's gear doors did.
Behind 5 cm of insulation, ~8 kg/m², a belly tank takes ~1.5 MJ/m²
through the whole entry and the hours after, which boils off ~15–20 kg
of its ammonia.

**The strip.** At 11.9 t, and at anything up to ~18–19 t, the fighter
hovers off it and lands on it vertically: the 30 MW lift fan and the
nozzle carry its weight with the engine on the rest of the turbopump.
From 300 m of dirt it takes off at up to ~21.9 t, short of full tanks.

**Harder worlds.** On worlds of Earth's density, heavier and larger,
with Earth's air at the surface thinning faster with height:

| Surface gravity | Orbital speed | Sortie budget | Nose on entry | Landing |
|---|---|---|---|---|
| 1.0 g | 7.8 km/s | ~11.5–11.6 t | ~2,220 K | vertical |
| 1.1 g | 8.6 km/s | ~10.7–10.9 t | ~2,370 K | vertical |
| 1.2 g | 9.4 km/s | ~9.9–10.0 t | ~2,510 K | vertical |
| 1.3 g | 10.2 km/s | ~8.7–8.9 t | ~2,650 K | vertical |
| 1.4 g | 11.0 km/s | ~6.9–7.6 t | ~2,780 K | a runway, or never lands |
| **1.5 g** | **11.7 km/s** | **~7.4–8.6 t, as much as entry allows** | **2,900 K** | **a runway, or never lands** |
| 1.6 g | 12.5 km/s | ~0.8–1.0 t, as much as entry allows | 2,900 K | |
| 1.7 g and up | 13.3 km/s and up | none: brought down, the climb's own propellant runs entry too hot | | |

Landing or not changes the budget by only ~0.1–0.7 t. **About 1.5 g is
the practical limit**, with most of a day of work down in the air,
though from 1.4 g its takeoff for the climb, at 15–16 t, wants more than
300 m of dirt; at 1.6 g entry heat, not the tanks, holds the budget to
a tonne; beyond, the fighter goes down one way, or refuels on the
ground. No braking burn helps: the heavier it comes down, the hotter.

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

On a world harder than ~1.6 g, more than ~12.5 km/s to orbit, there is no
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
- The fighter's aerodynamics, by CFD once the shape settles: the
  outer lines exported from `modeling/fighter.py`; vortex-lattice and
  panel runs (OpenVSP's VSPAERO) for lift, drag due to lift and trim;
  Euler and RANS (SU2) for maximum lift and where the leading-edge
  vortices burst, what the slats and canards add, the transonic drag rise
  and wave drag, the inlets and their diverters, drag above Mach 2 and
  where the ramjet stops holding level flight, and heating in entry, the
  canards edge-on to the flow included.
- The fighter's equipment beyond first order: built to today's practice
  it would weigh ~3.5 t more, and every tonne of it costs the sortie
  budget ~0.8 t.
- The light a space decoy must make to pass for the fighter's plume:
  ~10 MW is an order of magnitude, from a particle-laden exhaust at
  ~2,300 K.
- The breathing liquid: the body's density, and lungs that clear of it
  fast.
- Takeoff from dirt: debris thrown by the lift fan and nozzle, and hot
  gas ingested, neither modelled.
- The fighter's airframe heating at Mach 6.
- Antimatter-boosted dust (~3,000 s, gas core, a gamma lamp) and an
  open-ended fusion mirror for in-system craft.
- Armor and tank insulation against FELs, worked through.
