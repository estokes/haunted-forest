# Small Craft

Couriers cross between stars and never come near a planet. Everything
else that flies in a system — shuttles, fighters, missiles, tugs — is a
small craft. This doc covers the chemical tier: craft that burn **dust**,
the same IS metal the couriers carry, as a chemical fuel — and the
cruiser, which flies on the [ICF drive](icf-drive.md).

Numbers here are first-order, from energy balances and textbook engine
physics, not from a model.

## The Fuel

Metal that reverts to molecular hydrogen releases **216 MJ/kg** (see
[Ship Systems — The Metal](ship-systems.md#the-metal)). It does so with
no oxidizer: heat a grain past ~1000 K and it gives up its energy by
itself. In air the hydrogen it becomes can then burn as well.

| Fuel | Energy | Density | Needs |
|---|---|---|---|
| Jet fuel | 43 MJ/kg | 0.8 t/m³ | oxygen |
| Liquid hydrogen | 120 MJ/kg | 0.07 t/m³ | oxygen, cryogenics |
| **Dust, in any gas** | **216 MJ/kg** | **0.6 t/m³ packed** | nothing |
| **Dust, in oxygen air** | **~336 MJ/kg** | | oxygen for the second half |

Dust is the best chemical fuel there is, eight times jet fuel by mass,
dense, warm, and pumpable by magnets. It works in any atmosphere,
breathable or not. It is also IS's, so every shuttle and fighter is on
the same fuel leash as the couriers.

Every kilogram of it is ~50 kg of TNT waiting for heat. A light
shuttle's tanks hold a quarter of a kiloton; a heavy shuttle's, a few
kilotons.

## Dust Rockets

Pure dust reverting in a chamber runs at **~6,000 K** and gives ~1,700 s
of specific impulse (Silvera & Cole, 2010). No wall likes 6,000 K. Small
craft carry **ammonia** alongside the dust, for three jobs:

- **Coolant.** Liquid ammonia runs through the walls of every hot part,
  boils, and cracks into nitrogen and hydrogen, soaking up ~12 MJ/kg on
  its way to 2,500 K. Then it is injected as propellant, so the heat is
  not lost. Its products do not attack carbon; steam would.
- **Diluent.** Mixed into the gas, it pulls the temperature down, but
  slowly. Dust carries so much energy that a 1:1 mix still runs above
  4,000 K, and holding the gas to 3,000 K takes 12 kg of ammonia per
  kilogram of dust.
- **Fuel.** In air, cracked ammonia burns (18.6 MJ/kg), so it is not dead
  weight.

It is everywhere — outer-system ices, or made from air, water, and
power — and it stays liquid at modest pressure, with no cryogenics.

| Mix, dust : ammonia | Gas | Vacuum Isp | Used for |
|---|---|---|---|
| Pure dust | ~6,000 K | ~1,700 s | military performance; needs an exotic chamber |
| 1 : 1 | above 4,000 K | ~1,200 s | the standard rocket mix, with ammonia film along the walls |
| 1 : 2 | above 4,000 K | ~1,000 s | cheap lift |
| 1 : 5.6 | 5,000 K | — | the turbine gas of the hybrid engine |
| 1 : 12 | 3,000 K | — | |

(Temperatures from an energy balance on JANAF data; above 4,000 K
hydrogen dissociates and a chemistry model is needed. Isp scaled from
pure dust by energy per kilogram.)

## The Hybrid Engine

Atmosphere-capable craft fly one engine in three modes. It is an
**electric air turbo-rocket**: motors drive the fan and compressor, and
the power comes from a **turbopump**, a turbogenerator that runs in gas
from the dust **reverter** and never touches air. Nothing turns in the
hot air downstream.

CNT is ideal for the fan and compressor — cold, and strong enough for
very high tip speeds, so a few light stages give a high pressure ratio.
It is the wrong thing to put in hot air: carbon oxidizes above ~900 K.
The turbopump's gas is hydrogen and nitrogen, with no oxygen in it, so
its blades can be CNT too.

### The Turbopump

- **Feed**: dust slurried in liquid ammonia, pumped to ~50 MPa by the
  turbopump's own pump. More ammonia joins it at the reverter, after
  cooling the hot section.
- **Reverter**: the dust reverts, the ammonia cracks, and the gas leaves
  at **5,000 K**, with 5.6 kg of ammonia to each kilogram of dust. It is
  ~90% hydrogen by moles.
- **Turbine**: several stages expand the gas ~33:1, down to the main
  combustor's pressure. The stator vanes inject more dust, which reverts
  in the passage and lifts the gas back to 5,000 K before each rotor, so
  the turbine expands at nearly constant temperature. Dust can do this
  because it needs no oxidizer.
- **Blades**: CNT, under an insulating refractory-carbide skin, cooled
  from inside by ammonia. The stators return their coolant to the
  reverter's feed; the rotors bleed theirs out of their trailing edges.
- **Generator**: superconducting, on the turbopump shaft, at ~100 kW/kg.
- **Exhaust**: hydrogen and nitrogen at ~5,000 K and ~1.5 MPa, into the
  main combustor.

**Each megawatt of electricity costs ~2.4 kg of ammonia a minute**, and
a little dust. The exhaust burns downstream, so its energy is not lost;
the ammonia is the price of the work, because the turbine can only take
so much work from each kilogram of gas.

**Why 5,000 K.** The blade's CNT survives ~3,000 K. Its skin, of the
highest-melting materials known, survives ~4,000 K. Cooling lets the gas
run ~1,000 K hotter than the skin, as today's turbines run hotter than
their coatings and rockets run far hotter than their walls. Hydrogen and
nitrogen are nearly transparent, so the blades take little radiant heat.
Hotter would need a skin near 5,000 K, which no known material survives.
At 3,000 K the same turbine would need twice the ammonia per megawatt.

### The Air Path

- **Fan and compressor**: CNT blades, each rotor on its own motor at its
  own speed. Fan pressure ratio ~3.5, overall ~12, bypass ~0.4. As Mach
  rises the compressor slows, to keep its discharge below ~850 K.
- **Main combustor**: compressed air meets the turbopump's exhaust and,
  if it has oxygen, burns it; more dust can be injected directly. No
  turbine follows it, so it can run near stoichiometric.
- **Reheat**: the bypass air joins the core flow, and secondary dust and
  ammonia injectors burn the bypass air's oxygen.
- **Cooling**: the whole hot section is ammonia-cooled, and that
  ammonia, cracked and hot, feeds the reverter.

### Modes

| Mode | Speed | Isp, all propellant | Doors |
|---|---|---|---|
| **Turbine** | 0 to Mach ~3.5 | ~6,500–10,000 s | fan and compressor in the flow |
| **Ramjet** | Mach ~3.5 to ~6 | ~8,000 s | fan face closed; inlet air runs down the bypass duct and through doors into the main combustor |
| **Rocket** | above Mach ~6, and in space | 1,000–1,700 s | all closed; the main combustor and reheat are the chamber, and the turbopump runs only for the electricity needed |

In ramjet mode the stopped compressor sits beside duct walls at
~1,300 K, so its cavity is purged with cracked ammonia: hot CNT cannot
burn in it. In an atmosphere with no oxygen the engine still works, as a
pure heat-addition jet, at about two thirds of the thrust.

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
  ~30–45 kg of propellant a minute, most of it ammonia.
- **Backup**: the rocket can land the shuttle if the fans fail, at ~1 t of
  propellant a minute and a scorched landing site.

| | Mass |
|---|---|
| Structure, engine, fans, heat shield | ~7.5 t |
| Payload: crew and cargo | ~2–3 t |
| Propellant to orbit (air to Mach 6, then 1:1) | ~10 t, half dust, half ammonia |
| **Loaded** | **~20 t** |

## Fighter

**About 7.5 t empty and ~12 t loaded, with the lines of a Mirage 2000:**
a tailless delta, one fin, half-cone side inlets, one engine, but with a
broader forward fuselage to hold a lift fan. The inlet cones translate,
as the Mirage's do, but much further, to set up the inlet for the
ramjet. The delta's big wing gives it a low landing speed when it uses a
strip.

| Engine | |
|---|---|
| Airflow | ~90 kg/s |
| Thrust, sea level | ~105 kN dry, ~140 kN with reheat |
| Fan and compressor | 27 MW at full thrust |
| Turbopump | ~35 MW, enough for full thrust and the FEL together |
| Electric machines | ~0.75 t: generator ~350 kg, engine motors ~270 kg, lift-fan motor ~130 kg |

**It lifts the way the F-35B does**: an electric ~1.5 m fan behind the
cockpit, with doors above and below and a vane box under it, and the
main engine's nozzle swivelled straight down on a three-bearing duct.
The nozzle sits far aft on a tailless delta, so the fan carries about
two thirds of the weight.

| In the hover | Thrust | Power |
|---|---|---|
| Lift fan | ~75 kN | ~13 MW |
| Engine, nozzle down | ~40 kN | ~10 MW |
| **Total** | **~115 kN** | **~25 MW, ~60 kg of ammonia a minute** |

- **No shaft, no clutch.** The F-35B drives its fan from the engine
  through a driveshaft and clutch; this fan is wired to the turbopump.
  Pitch in the hover is a change of motor torque, in milliseconds.
- **Roll** comes from the wingtip thrusters the fighter uses in space, as
  the Harrier used wingtip puffers. Yaw comes from the fan's vanes and
  the nozzle.
- **Downwash**: ~190 m/s of cool air from the fan, and a ~1,500 K jet
  from the nozzle. The fighter lands on a pad, or on ground it can afford
  to scorch.
- **The wings are tanks.** One fan in the fuselage instead of three
  leaves the wing roots free.
- **A fan or the turbopump lost in the hover** is a hard upset, as it was
  for every lift-fan aircraft. The seat goes.

| | Mass |
|---|---|
| Empty | ~7.5 t |
| Weapons | ~1.5 t |
| Dust | ~1 t |
| Ammonia | ~2 t |
| **Loaded** | **~12 t** |

- **Range**: at Mach 0.9 it cruises on ~0.14 kg/s of propellant, about
  2,000 km per tonne, three to four times a Mirage 2000's. On internal
  tanks that is thousands of kilometres.
- **Rocket mode**: ~3 km/s from the internal tanks — zoom climbs,
  suborbital hops, a strike from above the atmosphere. Orbit needs
  external tanks with ~5 t more dust.
- **Ramjet mode**: Mach 6, briefly. The delta is a sound shape for it;
  the airframe's heat is the limit.

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
turbopump, and ten minutes of fire costs ~200 kg of ammonia and ~40 kg
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
- A chemistry model for the dust-ammonia mixes and the air-breathing
  modes, with dissociation.
- Heat transfer to the turbopump's blades at 5,000 K, and the ammonia it
  takes to carry it.
- The fighter's airframe heating at Mach 6.
- Antimatter-boosted dust (~3,000 s, gas core, a gamma lamp) and an
  open-ended fusion mirror for in-system craft.
- Armor and tank insulation against FELs, worked through.
