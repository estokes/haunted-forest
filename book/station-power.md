# Station Power — the D-³He Plant

Stations and colonies run on a **~100 GW D-³He fusion reactor**. It used
to be a ship's hotel plant. Couriers now make their electricity from
their engine's tile heat (see [Ship Systems — Power](ship-systems.md#power)),
and only so many people want to live next to kilotonnes of antimatter. So
the fusion plant lives where people do: on stations, at colonies, at the
industrial yards that build couriers.

The game's reactor and thermal simulation (`crates/hf-game/src/ship/reactor.rs`)
models this plant: 100 GW fusion, a 70 m burning column, a tin loop, and
eight zoned radiators.


## The Reactor

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

That is a city's industry, a shipyard's fabricators, a station's
defenses, and its people, with room to spare.

**Waste heat is a signature.** Even 37 GW, every zone lit, is a beacon.
Run cool and dark when you can. Light panels when you need the watts.
Shut the ones that face a contact.

³He at rated is ~5.4 t/year. A station keeps a stock of years to
centuries. Systems with gas giants mine their own; systems without buy
it, and one courier's payload of ³He runs a plant for ~150 years (see
[Trade](trade.md#elemental-commodities)).


## Confinement and Throttle

### The Tandem Mirror

The reactor confines its plasma in a **tandem magnetic mirror**: a straight
magnetic bottle roughly **70 m** long, with the field intensified at each
end into a magnetic "choke" that reflects escaping particles back toward the
middle. The burning plasma is a thin column about **3 m** in radius — some
**2,000 m³** at full length — held at **9 T** in the central cell, rising to
~27 T at the end chokes (mirror ratio ≈ 3).

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
| Redline | 100 GW | ~70 m (full) | Peak load. β riding toward 0.75; every radiator zone lit, or fewer zones running hotter. |
| Self-sustaining | 10–100 GW (10–100%) | ~7–70 m | Everyday range. Ignited at constant density; overhead turns marginal below ~10 GW. |
| Aux-driven / lean | 3–10 GW | at the ~7 m geometric floor | Standby minimum. Density dropped, aux heating holds temperature. Delicate. |
| Shutdown | < ~3 GW | cold | Overhead exceeds output — cold-and-dark, cold-start to relight. |
| Overdraw | 100–140 GW | ~70 m (full) | Surge past rated: density pushed up, β climbing. Thermally loud. You are a lighthouse. |

The **top** is a hard wall with two independent guards: the plasma's own
stability (beta past 0.90 → flute/interchange disruption) and thermal rejection
(the radiators saturate; see Thermal Management). The **bottom** is soft and
economic rather than physical: the burning column shrinks but the plug drive
and coolant pumps do not, so at a few percent power the fixed
overhead is a large fraction of output, and below ~3 GW it exceeds output
entirely. There is no point idling there — the reactor is shut down and later
cold-started. (There is also a hard geometric floor: the central cell cannot be
shorter than the plug and transition regions.)

Running lean stretches the ³He stock: at a few percent power the reactor sips a
small fraction of its rated fuel.

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
- **Plug collapse** — loss of electrostatic-plug drive (damage, heater
  failure) drops the end barrier; central-cell ions pour out the loss cone, and
  the plasma drains, cools, and quenches. The most likely way to lose a
  reactor on a long, quiet watch, when the plugs run for months.
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
to space. Radiation is the only option.

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
reactor, converters, and radiators sit together, away from the habitat.
Pump power is a fraction of a GW at this scale.

The loop does not have a single design temperature. Light load sits near
1300 K. Rated load is hotter. A zoned-down posture, half the
area shut, runs hotter still. Tin does not care.

### Radiators, Zoned

The radiators are about **63,000 m²** (ε ≈ 0.9), in **eight zones** with
eight valves, matching the plant sim. Shut the ones that face a contact,
or the ones that face a habitat. The remaining panels run hotter, or
you throttle.

| Load | All zones lit | Character |
|------|----------------|-----------|
| Light load ~8–12 GW | ~1260–1390 K | A dull glow. |
| Rated 37 GW | ~1850 K | A lantern. Graphite/C-C panels. |
| Overdraw ~50 GW | ~2000 K | You are a lighthouse. |
| Rated, half the zones shut | ~2200 K | Soft limit. Throttle or open the dark zones. |

T⁴ buys the watts back; tin boils at 2875 K. No deployables: zoning is
valves.
