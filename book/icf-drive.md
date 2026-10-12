# ICF Drive

The in-system drive for warships and big in-system craft: deuterium–
helium-3 pellets fired at up to a hundred a second, compressed and lit by
FELs, their blasts turned into thrust by a magnetic ring. It is what
gives a cruiser the hundreds of km/s a system takes. Numbers are
first-order; the pellet gain is an assumption, stated below.

## Why This Drive

- **Deuterium–helium-3** releases 3.5×10¹⁴ J/kg and is nearly aneutronic:
  its products are a 14.7 MeV proton and a 3.6 MeV alpha, both charged,
  so a magnet can push on them. Side reactions between deuterium nuclei
  put ~3% of the yield into neutrons. Deuterium–tritium ignites far more
  easily but puts 80% into neutrons; pure deuterium needs no helium-3 but
  puts about a third there.
- **Helium-3** is already the station economy's fuel (see [Station
  Power](station-power.md)), mined from gas giants. A warship burns what
  a city burns.
- **FEL drivers** are already everywhere in the setting. No IS anti-dust
  is needed to fly a cruiser — only to fill its magazine.

**The assumption:** each pellet returns ~200 times the driver energy that
lights it. Deuterium–helium-3 ignition is far harder than
deuterium–tritium: hotter, denser. Physics allows it, and six centuries
of work is the setting's premise. It is stated here the way the metal is.

## How It Works

1. **A pellet** — deuterium and helium-3, about a tenth of a gram, of
   which ~30% burns — is fired to the burn point on the drive's axis.
2. **A jacket** of frozen water or ammonia, whichever is local, sits a
   centimetre or so around the capsule, with windows for the drivers. The
   jacket is the propellant: its mass sets the exhaust speed. It has to be
   that close: the 14.7 MeV fusion protons stop only in ~0.25 g/cm² of
   water, and a 10 g jacket 1.4 cm in radius gives ~1.3 g/cm². A loose
   puff spread over metres would let them fly through (see The Blast). The
   jacket also soaks up X-rays before they reach the ring.
3. **FEL drivers** on the ring compress the pellet and light it:
   ~50 MJ in, ~10 GJ out (~2.4 t of TNT).
4. **The blast**, pellet and jacket together, expands as a plasma ball into
   the field of a superconducting **ring** ahead of it. The plasma pushes
   the field, the field pushes the ring, and the plasma leaves astern.
5. **Induction coils** take back some of the plasma's work on the field:
   enough to power the drivers and the ship.

At up to 100 pulses a second that is **1 TW** of fusion. About 0.6 TW
reaches the jet; the rest goes to neutrons, X-rays, and the ~10% of the
plasma's momentum the nozzle does not turn astern. Each blast clears the
nozzle in about a millisecond, well inside the 10 ms between pulses.

## The Knob

Jacket mass per pulse trades exhaust speed for thrust. Each pulse gives
~0.9 × √(2 × 7.7 GJ × jacket mass) of impulse (see The Blast):

| Exhaust speed | Thrust | Jacket per pulse | Propellant flow |
|---|---|---|---|
| ~11,000 km/s (no jacket) | 0.11 MN | — | 10 g/s |
| ~1,100 km/s | 1.1 MN | 10 g | 1 kg/s |
| ~350 km/s | 3.5 MN | 100 g | 10 kg/s |
| ~110 km/s | 11 MN | 1 kg | 100 kg/s |

The ship cruises on helium-3 and sprints on water.

## The Ring

- **~200 m across, ~25 MA**, set **~100 m ahead of the burn point** (see
  The Blast for why). It makes ~56 mT at the burn point, stores ~200 GJ,
  and needs ~120 t of magnet. As a superconductor it keeps its flux: its
  current rises ~2% as each blast squeezes the field.
- **X-rays** are maybe a fifth of each pulse. Soft X-rays stop in the
  first micrometres of tungsten; to keep each pulse below ~30 kJ/m² and
  not ablate it, nothing sits closer than ~75 m. The conductor is ~141 m
  from the burn point: ~8 kJ/m² a pulse, ~0.8 MW/m² on average, on cooled
  tungsten tiles.
- **Neutrons** reach the ring unshaded: ~150 MW of them. The
  superconductors run at ~900 K, which may anneal their radiation damage
  as they go — to be checked.

## Nothing on the Axis — Again

A ring turns plasma that comes at it slanting. Plasma coming straight
down the axis runs along the field and is not turned, so anything on the
axis ahead of the burn point would take a stream of fusion debris. The
courier's answer works here: **the hull rides the ring.** The fusion
products are far easier to turn than a courier's pions — ~0.56 T·m for
the 14.7 MeV protons, ~0.27 T·m for the alphas, against 3.1 T·m — and a
25 MA ring forbids them a wide region around its conductor (see
[Courier — The Ring Is a Shelter](courier.md#the-ring-is-a-shelter)).

The tracker in `crates/hf-nozzle`, run with fusion protons and alphas
from the burn point, confirms the shelter: no hits on the pods or the
conductor in 400,000 tracks.

## The Blast

Three models in `crates/hf-nozzle` follow one 10 GJ blast — 7.7 GJ of
plasma, 10 g of it — into the ring's field.

- **Single particles** (`hf-nozzle` with ion emission): late in the blast,
  when the plasma is thin, every ion follows field lines on its own. The
  ring is a magnetic mirror and the burn point sits in its weak field, so
  ions heading nearly straight ahead, inside the mirror's loss cone, leave
  forward through the bore. The tracker matches mirror theory to three
  decimal places.
- **A thin shell** (`hf-nozzle shell`): early in the blast the plasma is
  too dense to follow field lines and pushes the field aside as a
  conducting shell; superconducting rings keep their flux; fingers that
  run along the axis pinch off as jets. Checked against a superconducting
  sphere in a uniform field, and plasma and ring momenta cancel to 0.5%.
- **Hybrid** (`hf-nozzle hybrid`): ions as particles in three dimensions
  on an axisymmetric grid, electrons as a massless fluid, vacuum as a
  current-free field, a free-space edge. It follows the whole blast. Its
  energy exchange between ions and field is exact by construction and
  checked by a ledger; its blast in a uniform field stops within ~10% of
  the thin shell's radius; plasma and ring momenta agree to 2–10%.

The fraction of the blast's momentum turned astern:

| Burn point behind the ring | Single particles | Thin shell | Hybrid | Hybrid: escapes forward (mass / energy) |
|---|---|---|---|---|
| 60 m | 60% | 72% | 69% | 16% / 10% |
| **100 m** | 79% | 83% | **91%** | **1% / 2%** |
| 150 m | 89% | 76% | 84% | 3% / 3% |

The models agree that **~100 m is best**: closer, the weak mirror lets
the plasma leak forward; farther, the field at the burn point is too weak
to turn the blast before it spreads. The hybrid gives 90% on a finer grid
(4 m cells instead of 5 m). No run put plasma on the conductor.

**The jacket matters.** If the fusion protons and alphas keep their energy
— a loose puff that only the X-rays heat — the hybrid gives 84%, with 13%
of the energy leaving forward, mostly fast protons in the loss cone. Worse,
the impulse per pulse falls from ~11 kN·s to ~5.6 kN·s: the fast products
carry most of the energy but little of the momentum. A jacket close enough
to stop them doubles the thrust.

What the models leave out: they are axisymmetric, so the flute
instabilities that break a plasma's edge into fingers around the axis are
missing, and the hybrid drops the azimuthal field, a second-order effect
of the Hall term. Thrust is counted from the plasma's momentum; the
impulse on the ring, from the Maxwell stress around its conductor, comes
out 2–10% lower.

Each pod faces the burn point with a **neutron plate** — about a metre of
lithium hydride, boron, and steel, ~8 m in radius — and sits in its
shadow. At ~150 m a plate catches ~0.07% of the neutrons, ~20 MW, and
~0.15 GW of X-rays.

## The Cruiser

A cruiser is a gunboat scaled up around this drive: a ring 200 m across,
three pods on it, and a hangar.

| Part | Mass |
|---|---|
| Ring: magnet, tiles, frame, fins | ~0.5 kt |
| Driver FELs, capacitors, induction pickup | ~1 kt |
| Radiators (~9 GW of driver waste, ~0.4 GW of nuclear heating, at ~1,500 K) | ~0.2 kt |
| Three neutron plates | ~1.2 kt |
| Hangar: ~12 fighters and 3 gunboats | ~1.1 kt |
| Crew of ~60 and months of life support | ~0.3 kt |
| Structure and tanks | ~0.5 kt |
| Weapons, missiles, magazine | ~0.2 kt |
| **Dry** | **~5 kt** |
| Water or ammonia | ~4 kt |
| Deuterium and helium-3 | ~20 t |
| **Loaded** | **~9 kt** |

The pods are a command and crew pod, a hangar pod, and a stores and
weapons pod. A gunboat or fighter in the hangar is in its pod's shadow.

| Exhaust speed | Thrust | Acceleration, loaded to dry | Water lasts | Δv on all of it |
|---|---|---|---|---|
| ~1,100 km/s | 1.1 MN | 13–23 mG | ~46 days | ~650 km/s |
| ~350 km/s | 3.5 MN | 40–72 mG | ~4.6 days | ~210 km/s |
| ~110 km/s | 11 MN | 0.13–0.23 G | ~11 hours | ~65 km/s |

**1 AU**, burning the whole way at ~1,100 km/s: ~24 days, ~2.1 kt of
water, ~3.5 t of helium-3 — about eight months of a station plant's.
Every cruiser voyage competes with a city for fuel.

### The Main Battery

The drive's FELs deliver ~5 GW of light on average. Turned outward
through a few large directors instead of onto pellets, they are the
cruiser's main battery: 5 GW from a 5 m director is lethal to soft
targets out to ~10,000 km, ten times a gunboat's reach. A cruiser can
burn hard or shoot hard, not both.

## Hazards

- **Neutrons.** A burning cruiser emits ~30 GW of them in every
  direction: lethal to an unshielded person within ~100 km in minutes,
  ~1 Sv an hour at 1,000 km. Cruisers keep away from habitats, and
  throttle down to launch and recover small craft, which leave the shadow
  of their pod as they go.
- **X-rays**, mostly soft, stop in a hull's skin.
- **Pellets and jackets** are not dangerous to carry. The magazine is.

## Open Questions

- Flute instabilities at the blast's edge, which need a 3D model.
- The jacket: windows for the drivers, and how much of the fusion
  products' energy it really takes up.
- Collimation at jacket masses other than 10 g; the thin shell says it does
  not depend on mass at fixed energy.
- Pellet gain and driver energy for deuterium–helium-3.
- Superconductor life under ~150 MW of neutrons.
- Pellet manufacture aboard, and its cryogenics.
