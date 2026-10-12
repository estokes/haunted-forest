# Small Craft

Couriers cross between stars and never come near a planet. Everything
else that flies in a system — shuttles, fighters, missiles, tugs — is a
small craft. This doc covers the chemical tier: craft that burn **dust**,
the same IS metal the couriers carry, as a chemical fuel. Fusion and
antimatter drives for small craft are later work.

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

**Vertical lift comes from two fans buried in the centre fuselage**,
either side of the centre of gravity, with doors above and below. The
Mirage IIIV did the same in 1965 with eight small lift jets. The fans are
~1.5 m across, driven electrically like the light shuttle's, from the
engine's generator; together they lift ~120 kN, and louvres in their
exits give control in the hover. The main engine idles straight aft. The
fans' jet is ~250 m/s of cool air: hard on a surface, but not hot.

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

## Vacuum Fighters and Missiles

To come: short-range vacuum fighters and medium-range missiles, on rocket
mode alone — pure dust for performance, how much Δv a fight takes, and
what it means to fight a courier at milligees.

## Open Questions

- Chamber design for pure dust at ~6,000 K.
- A chemistry model for the dust-ammonia mixes and the air-breathing
  modes.
- The fighter's airframe heating at Mach 6.
- Antimatter-boosted dust (~3,000 s, gas core, a gamma lamp), fusion, and
  ICF drives for small craft.
