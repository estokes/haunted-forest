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
2. **A puff** of water or ammonia, whichever is local, is released around
   it. The puff is the propellant: its mass sets the exhaust speed. It
   also soaks up X-rays before they reach the ring.
3. **FEL drivers** on the ring compress the pellet and light it:
   ~50 MJ in, ~10 GJ out (~2.4 t of TNT).
4. **The blast**, pellet and puff together, expands as a plasma ball into
   the field of a superconducting **ring** ahead of it. The plasma pushes
   the field, the field pushes the ring, and the plasma leaves astern.
5. **Induction coils** take back some of the plasma's work on the field:
   enough to power the drivers and the ship.

At up to 100 pulses a second that is **1 TW** of fusion. About half
reaches the jet; the rest goes to neutrons, X-rays, and nozzle losses.

## The Knob

Puff mass per pulse trades exhaust speed for thrust:

| Exhaust speed | Thrust | Puff per pulse | Propellant flow |
|---|---|---|---|
| 10,000 km/s (no puff) | 0.1 MN | — | 10 g/s |
| 1,000 km/s | 1 MN | 10 g | 1 kg/s |
| 300 km/s | 3.3 MN | 110 g | 11 kg/s |
| 100 km/s | 10 MN | 1 kg | 100 kg/s |

The ship cruises on helium-3 and sprints on water.

## The Ring

- **~200 m across, ~25 MA**, set ~60 m ahead of the burn point. It makes
  ~0.1 T about 50 m from the conductor, enough to stop each blast's plasma
  ball within ~70 m. It stores ~200 GJ and needs ~120 t of magnet.
- **X-rays** are maybe a fifth of each pulse. Soft X-rays stop in the
  first micrometres of tungsten; to keep each pulse below ~30 kJ/m² and
  not ablate it, nothing sits closer than ~75 m. The ring is ~117 m from
  the burn point, where the average load is ~3 MW/m² on cooled tungsten
  tiles.
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

### The Forward Leak

The ring is a magnetic mirror, and the burn point sits in its weak
field. Particles that gyrate tightly — the puff ions, and even the
fusion protons, which circle in ~6 m at 0.1 T — follow field lines, and
the field lines through the burn point all pass through the ring's bore.
Those moving within the mirror's loss cone, nearly straight ahead, are
not turned back: they leave forward along the axis, a negative thrust
and a plume ahead of the ship. The tracker matches mirror theory to three
decimal places:

| Burn point behind the ring | Field there | Mirror ratio | Escapes forward | Momentum that becomes thrust |
|---|---|---|---|---|
| 60 m | 100 mT | 1.6 | 20% | 60% |
| 100 m | 56 mT | 2.8 | 10% | 79% |
| 150 m | 27 mT | 5.9 | 4.5% | 89% |
| 200 m | 14 mT | 11 | 2.4% | 91% |

Moving the burn point back fixes the leak, but weakens the field that has
to stop each blast's plasma ball before it reaches the ring. Early on the
blast is too dense to follow field lines: it pushes the field aside as a
whole. How these two trade off is what the plasma model has to answer.
Until then the design point stays at 60 m, with the leak as its known
cost.

Each pod faces the burn point with a **neutron plate** — about a metre of
lithium hydride, boron, and steel, ~8 m in radius — and sits in its
shadow. At ~117 m a plate catches ~0.1% of the neutrons, ~35 MW, and
~0.2 GW of X-rays.

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
| 1,000 km/s | 1 MN | 11–20 mG | ~46 days | ~590 km/s |
| 300 km/s | 3.3 MN | 38–68 mG | ~4 days | ~180 km/s |
| 100 km/s | 10 MN | 0.11–0.2 G | ~11 hours | ~60 km/s |

**1 AU**, burning the whole way at 1,000 km/s: ~25 days, ~2.2 kt of
water, ~3.7 t of helium-3 — about eight months of a station plant's.
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
- **Pellets and puffs** are not dangerous to carry. The magazine is.

## Open Questions

- The plasma ball's collective push on the field, and the burn point's
  best distance behind the ring: a hybrid particle-in-cell model, not the
  single-particle tracker.
- Pellet gain and driver energy for deuterium–helium-3.
- Superconductor life under ~150 MW of neutrons.
- Pellet manufacture aboard, and its cryogenics.
