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
of specific impulse (Silvera & Cole, 2010). No wall likes 6,000 K. So
small-craft rockets dilute the dust with **ammonia**:

- Ammonia cracks into nitrogen and hydrogen, which absorbs heat
  (~2.7 MJ/kg) and pulls the chamber temperature down.
- Its products are hydrogen and nitrogen: light, and they do not attack
  the carbon the engines are built from. Water would work, but steam eats
  carbon at temperature.
- In air, ammonia is a fuel too (18.6 MJ/kg), so it is not dead weight.
- It is everywhere — outer-system ices, or made from air, water, and
  power — and it stays liquid at modest pressure, with no cryogenics.

The dilution trades exhaust speed for IS dust, and ammonia is cheap and
local:

| Mix, dust : ammonia | Chamber | Vacuum Isp | Used for |
|---|---|---|---|
| Pure dust | ~6,000 K | ~1,700 s | military performance; needs an exotic chamber |
| 1 : 1 | ~3,500 K | ~1,200 s | the standard rocket mix |
| 1 : 2 | ~2,600 K | ~1,000 s | cheap lift, hover-adjacent work |
| ~1 : 3 | ~2,000 K | — | the gas that drives turbines |

(Scaled from the pure-dust figure by energy per kilogram; a chemistry
model would refine them.)

## The Hybrid Engine

Atmosphere-capable craft fly one engine in three modes. It is an **air
turbo-rocket**: a carbon-nanotube compressor on a shaft, driven by a
turbine that runs in gas from the dust **reverter**, never in air.

CNT is ideal for the compressor — cold, and strong enough for very high
tip speeds, so a few light stages give a high pressure ratio. It is the
wrong thing to put in hot air: carbon oxidizes above ~900 K. In this
engine the turbine sees only hot hydrogen and nitrogen from a ~1:3
dust-ammonia mix at ~2,000 K, under carbide coatings. Its exhaust then
burns with the compressed air in the combustor.

| Mode | Speed | Fuel efficiency | How |
|---|---|---|---|
| **Turbine** | 0 to ~Mach 3.5 | ~10,000 s | compressor in the flow; reverter gas drives the turbine and burns with the air |
| **Ramjet** | Mach ~3.5 to ~6 | ~8,000 s | compressor bypassed; reverter gas burns in the duct |
| **Rocket** | above Mach ~6, and in space | 1,000–1,700 s | inlet closed; the reverter is the chamber |

The air-breathing figures scale hydrogen engines (~4,000 s turbojet,
~3,000 s ramjet) by the dust's energy per kilogram. In an atmosphere with
no oxygen the engine still works, as a pure heat-addition jet, at about
two thirds of that.

There is no scramjet stage. The rocket mode is good enough that one would
save only another ~10% of dust for a great deal of complexity.

Above Mach ~3.5 the compressor discharge gets too hot for bare carbon,
which is where the compressor is bypassed anyway.

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
- **Drive**: a superconducting generator on the main engine's shaft feeds
  superconducting motors in the fans. With superconductors that work to
  ~1000 K there are no cryogenics and the machines are light. Differences
  in fan thrust hold attitude in the hover without reaction jets, and
  losing one fan is not fatal. Hovering takes 10–16 MW, which the engine
  makes for ~10–20 kg of propellant a minute.
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
a tailless delta, one fin, half-cone side inlets, one engine. The inlet
cones translate, as the Mirage's do, but much further, to set up the
inlet for the ramjet. The delta's big wing gives it a low landing speed
when it uses a strip.

**Vertical lift comes from three buried fans in a triangle around the
centre of gravity**: one ~1.3 m fan behind the cockpit and one ~1.6 m fan
in each wing root, each with doors above and below. The Ryan XV-5
Vertifan flew this layout in 1964. The fans are driven electrically like
the light shuttle's, from the engine's generator, with the motors in the
duct rings, so a fan is thin enough for the delta's root. Together they
lift ~120 kN from ~5.4 m² of disk, for ~16 MW.

- **Control in the hover**: pitch by trading thrust between the nose fan
  and the wing fans, roll by trading it between the wings, yaw by louvres
  in the fan exits. The main engine idles straight aft.
- **Downwash**: ~190 m/s of cool air — hard on a surface, but not hot.
- **The centre fuselage** stays free for tanks and an internal bay.
- **A fan lost in the hover** is a hard upset, as it was for every
  lift-fan aircraft. The seat goes.

| | Mass |
|---|---|
| Empty | ~7.5 t |
| Weapons | ~1.5 t |
| Dust | ~1 t |
| Ammonia | ~2 t |
| **Loaded** | **~12 t** |

- **Range**: the tanks hold about three times the energy of a Mirage
  2000's internal fuel. In turbine mode that is thousands of kilometres.
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

Power is what separates the sizes. A dust-fired turbogenerator burns
~150 kg of dust for ten minutes of a fighter's fire. A gunboat carries an
annihilation APU, like a courier's, for endurance.

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
takes — 1 AU in ~25 days — and its small craft give it the gees and the
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
  modes.
- The fighter's airframe heating at Mach 6.
- Antimatter-boosted dust (~3,000 s, gas core, a gamma lamp) and an
  open-ended fusion mirror for in-system craft.
- Armor and tank insulation against FELs, worked through.
