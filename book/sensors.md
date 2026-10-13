# Sensors and Stealth

How anything in this setting sees, and how it tries not to be seen. It
covers couriers, small craft, and stations. The courier's debris-defense
suite — the THz search, the pebble radar, the cooker — is worked through
in [Ship Systems — Active Debris Defense](ship-systems.md#active-debris-defense);
this doc reuses its radar scaling.

Ranges here are order-of-magnitude. The passive ones assume a cooled
1 m infrared telescope that detects ~10⁻¹⁷ W/m² in about a second.

## The Rules

1. **Burning is being seen.** Every engine in this setting is loud.
2. **Heat has to go somewhere.** A craft can hide its waste heat for a
   while, never for good.
3. **Going dark does not reset a track.** A craft that was tracked and
   then went cold is coasting on a path anyone can predict. The watcher
   points radar or lidar where it has to be.
4. **Going active announces you.** A radar is heard 10–100 times farther
   away than it can see, by anyone in its beam. A lidar is seen by what
   it hits.
5. **Every picture is old.** At 30 light-seconds, it is 30 seconds old.

## Passive

Passive sensors watch the whole spectrum, from radio to gamma rays, and
give nothing away.

| Source | Seen from about |
|---|---|
| Fighter running cold (~50 m² of skin at 250 K) | 15–20 light-seconds |
| Fighter on its rocket (a GW-class plume) | across the inner system |
| Gunboat on its APU, radiators showing | ~1 AU |
| The same, radiators edge-on to the watcher | ~0.1 AU |
| Courier coasting on its APU | ~1 AU |
| Courier burning: 38 GW of fins at 1,500–2,900 K, and 180 TW of gammas | anywhere in the system; a 100 m² gamma detector registers it from a light-day out |

- **Infrared detectors need ~30–80 K.** That is the one exception to "no
  cryogenics": sensors get small coolers or shaded radiators.
- **A dark craft blocks starlight.** Wide-field cameras watching for stars
  that blink out catch nearby ones.
- **Near a star, everything is lit.** Reflected sunlight shows hulls that
  run perfectly cold.
- **A courier's magnetic field is not a signature.** The ring makes ~2 pT
  at 1,000 km, under the ~5 nT background between planets.

### Gamma Lenses

No material reflects gamma rays. They oscillate far faster than any
electron can follow, so every material's refractive index differs from 1
by a few parts in a billion at 1 MeV. Head-on, a surface reflects ~10⁻¹⁸
of them; a plasma mirror for them would need ~10³⁹ electrons per m³,
denser than a white dwarf. Mirrors work only at grazing angles, ~80 µrad
off gold at 1 MeV, which suits telescopes and nothing else.

What does work is diffraction off crystal planes. A **Laue lens** is a
disc of crystal tiles, each tilted to bend one gamma line onto a detector
behind it: 511 keV, from positrons meeting electrons, bends ~6 mrad off
copper, so a 5 m lens focuses ~200 m behind itself. It catches tens of
percent of that one line, and sees where it comes from against a dim
background. Stations fly them as pairs, lens and detector in formation;
a cruiser can string one across its ring.

A lens does not find a burning courier; anything does. It tells drives
apart and finds the quiet ones:

- **What is burning.** Pion decay gives a courier a broad hump at ~70–300
  MeV. An ICF drive's neutrons, caught in hydrogen, glow at 2.2 MeV.
  Antimatter anywhere glows at 511 keV.
- **A running annihilation APU.** A gunboat's APU makes ~18 MW of gammas
  inside its shield. If even 1 kW of 511 keV leaks out, a lens with ~1 m²
  of effective area sees it from a light-second or two in an hour. A
  gunboat that wants to vanish shuts its APU down, not just its skin.

## Active

### Radar

Range grows only with the fourth root of target size and power (scaled
from the pebble radar in ship-systems.md):

| Radar | Stealthy fighter (~10 m²) | Gunboat (~1,000 m²) |
|---|---|---|
| Gunboat, 10 MW, 15 m dish | ~55,000 km | ~180,000 km |
| Cruiser, 100 MW, 20 m dish | ~0.8 light-seconds | ~2.5 light-seconds |

### Lidar and the Cooker

The FEL director does three jobs: weapon, lidar, and cooker.

- **Lidar**: a cued 1 GW pulse from a 1 m director tracks a stealthy
  fighter at 3–6 light-seconds with a precision radar cannot touch. It
  needs a cue: its beam is a metre wide at a light-second, so it cannot
  search.
- **The cooker**: flood a dark target with laser light and watch it
  warm and glow.
- **Neither is covert to its target.** A laser warning receiver sees the
  pulse that finds it, and the direction it came from. What lidar hides
  is the hunter from everyone else: its beam is a metre wide, so a third
  party sees nothing, and a lidar that searches and misses gives little
  away.

### Who Shouts

**The big ship shouts; the small craft listen.** A cruiser or gunboat
illuminates and accepts being seen. Silent fighters, spread out, receive
the echoes from other angles and stay hidden. They share what they see
over tight laser links, which are hard to intercept.

## Stealth

Hiding means three things: a cold face toward the watcher, a skin that
returns nothing, and somewhere to put the heat for a while.

### The Cold Face

The side of the craft facing the enemy is insulated and cold; it
radiates its heat from the far side.

- **Beam the heat.** Reflective louvres in front of the radiators (Winston
  cones) confine their emission to a cone pointed away from the threat.
  Confining it to a cone ~60° wide costs ~4× the radiator area, or ~1.4×
  the temperature.
- **It works against a known direction.** Pickets spread around the sky
  see the hot side. Several watchers defeat it.
- **A cold face in sunlight is not cold.** Near a star, a stealthy craft
  keeps the star behind it, or carries a shade.

### The Skin

- **Radar-absorbing skin**, with shaping, cuts a fighter's radar cross
  section 100–1,000 times. That divides radar range by only 3–6: the
  fourth root works for the hunter.
- **Spectrally selective.** The ideal skin is black at radar and laser
  wavelengths, so it returns no echo, and shiny in the thermal infrared,
  so it radiates little of its own heat. Carbon-nanotube composite is a
  conductor and reflects radar like metal, so the coating does the work.
  Super-black CNT coatings are no good here: they absorb everything, and
  so radiate in the infrared like a blackbody.

### Heat Sinks: Ammonia

Dust-burning craft already carry **ammonia**. It is also the best heat
sink they have:

- Boiling ammonia absorbs ~1.4 MJ/kg at about room temperature, so the
  interior's heat goes into it with no heat pump.
- The vapor vents through cold-gas thrusters at ~1 km/s. The heat leaves
  the ship as a cold, nearly invisible plume, and the craft gets
  **Δv as a by-product of hiding**.

| | Running dark | Ammonia | How long | Cold-gas Δv from it |
|---|---|---|---|---|
| Fighter | ~100 kW | ~1 t | ~4 hours | ~90 m/s |
| Gunboat, APU at idle | ~1 MW | ~10 t | ~4 hours | ~35 m/s |

The vents can fire in opposed pairs when the craft must not move.

**Heat pumps** let a craft store heat in a hotter sink — molten lithium
hydride at ~960 K holds far more per kilogram than boiling ammonia — and
radiate it later, fast, through small radiators. Lifting heat from 300 K
to 900 K costs roughly as much work as the heat moved, and that work is
heat too, so a pumped sink takes about twice what the craft makes. It is
the answer for a big source that cannot stop, like a gunboat's APU at
power, and a poor one for anything that can.

### Cold Gas Breaks the Track

A few tens of m/s does not take a craft anywhere. It does make it hard to
find. A tracker that lost a dark craft predicts where it must be; 30 m/s
of cold gas puts it ~100 km from that point an hour later, and the error
grows. A cued lidar, a metre wide at a light-second, cannot search that.
The hunter has to fall back on radar, and announce itself to everyone in
a wide volume, not just to the craft it is looking for.

### What Cannot Hide

A courier: a kilometre of ring, an APU dumping ~70 MW, and when it burns,
the brightest gamma source in the system. A courier's only stealth is
distance.

## Open Questions

- Sensor ranges done properly: detector noise, backgrounds, integration
  times.
- Station sensor arrays, which can be enormous.
- How much a vented ammonia cloud shows up to lidar and spectroscopy.
