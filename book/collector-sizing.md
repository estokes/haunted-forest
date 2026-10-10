# Collector Sizing — What a Real Magnetic Scoop Takes

> **Outcome (2026-10-10):** the scoop is dropped. The ship carries
> metastable metallic hydrogen that only IS makes; see
> [Ship Systems — The Fuel](ship-systems.md#the-fuel--metallic-hydrogen).
> This doc stays as the reason why.

2026-10-10. Follows review findings P1–P2
([review-2026-09-22.md](review-2026-09-22.md)). Numbers come from
`modeling/collector_sizing.py`. Speeds are 0.10c and the math is
non-relativistic unless stated; at 0.1c the error is under 1%.

Assumptions:
- Ship: 470 kt (trade run).
- Superconductor: REBCO-class HTS. The engineering current density
  J_e is the current divided by the whole tape cross-section,
  including substrate and copper.
  - Today, cold (20–30 K) and at low field: J_e ≈ 10⁹ A/m².
  - "10× better tape": J_e ≈ 10¹⁰ A/m².
  - Conductor density: 8,000 kg/m³.
- Structure, given as specific strength σ/ρ:
  - Today's best fibre (Zylon or Dyneema class): 3.5 MJ/kg.
  - Near-ideal CNT, ~45 GPa: 35 MJ/kg.
- Gas, as densities of *charged* particles. The scoop catches only
  those (§2.4).
  - Bible figure: 0.1 cm⁻³, treated as if fully ionized.
  - Warm local cloud: 0.07 cm⁻³ of ions, at 7,000 K.
  - Hot Bubble gas: 0.005 cm⁻³, at 10⁶ K.


## 1. Bottom line

- **The coils must be about as big as the area they catch from.**
  That means a deployed hoop thousands of km across. Bow coils
  can't do it. A dipole's field falls as 1/r³, and the field does
  not steer a 4.7 MeV proton at all beyond ~1.6 km from a 43 m,
  20 T coil. That coil catches about **1 kg per year**.
- **The limit is magnetic flux, not superconductor field.**
  - Every kg/s you catch drags ~1.2×10⁸ Wb of field into the ship
    (at 0.1 cm⁻³), whatever the speed.
  - Conductor mass is proportional to that flux divided by J_e.
  - A stronger magnet does not help. The mirror effect caps the
    throat field at **~2 T** in warm gas and **~3 mT** in hot gas,
    far below what HTS can reach.
- **The bible's collector (3.1 kg/s, 14 kt booked) is 100–1000× too
  light.** The mouth hoop alone is ~1 Mt of conductor, even with tape
  10× better than today's, plus ~0.9 Mt of CNT to hold it together.
- **Stopping the stream is the part that can't be modelled.**
  - Thermally, 3% of 1.4 PW is 42 TW.
  - Geometrically, any converter has to be as big as the scoop
    mouth (§4.2).
- **The intake should never be stopped. Burn it in passing.** That
  removes the 0.12c wall and keeps the petawatts inside the IS box.
- **Time dilation is out of reach at the IS power rating, whatever
  the scoop.** It takes 65 years at 3.2 PW to reach 0.5c (γ = 1.15).


## 2. The four physics constraints

### 2.1 The ion scale: there is a minimum scoop

A field only funnels a plasma *collectively* when the scoop is much
larger than the ion inertial length d_i = c/ω_pi:
- 720 km at 0.1 cm⁻³.
- 3,200 km at 0.005 cm⁻³.

Below that size the electrons are steered but the ions punch through
to the Störmer radius. Simulations of small magnetospheres (lunar
magnetic anomalies, magnetic-sail studies) show this sharp
transition.

Converting the mouth radius into intake rate gives:

> R/d_i ≈ 11 · √(ṁ / 1 kg/s) at 0.10c, independent of density.

So **a scoop under ~1 kg/s barely works as a funnel**, in any gas. A
smaller collector does not degrade gracefully. It stops working.

### 2.2 Pressure: flux per kilogram

The field holds the flow where B²/2μ₀ ≥ ρv², which gives
B_c = v√(2μ₀ρ):
- ~600 nT at 0.1 cm⁻³ and 0.1c.
- 137 nT in hot gas.

Mouth area times B_c is the flux:

> Φ = ṁ · √(2μ₀/ρ)  — **the speed cancels.**

That is 1.2×10⁸ Wb per kg/s at 0.1 cm⁻³, and 4.5× more in hot gas.
Flux is conserved along the funnel, so whatever area the captured
stream reaches, its field is Φ/area.

### 2.3 Mirror: HTS doesn't help

A magnetized proton keeps v⊥²/B constant. As the funnel squeezes,
the field rises from B_c to B_t, and a proton whose initial pitch
angle exceeds √(B_c/B_t) bounces back out.

The incoming gas has thermal spread v_th/v:
- 2.5×10⁻⁴ for warm gas.
- 3×10⁻³ for hot gas.

Letting 90% through requires:

> R_throat / R_mouth ≥ 2.15 · v_th/v  →  B_t ≤ B_c·(R_mouth/R_throat)²

That caps the throat field at **2 T in warm gas and 3 mT in hot
gas**. Above that, a stronger magnet only reflects more of the
catch. The field lines at the mouth also have to be parallel to the
flow to within ~10⁻⁴ rad over the whole disk.

Schattschneider & Jackson ("The Fishback ramjet revisited", Acta
Astronautica, 2022) tracked single particles through a real
solenoid funnel and hit the same wall from the other side. For 11 MN
at 1 cm⁻³ and 0.56c they needed:
- A mouth ~1,800 km in radius.
- A funnel **~1.6×10⁸ km long**, because the field has to change
  slowly enough for protons to follow it.

The collective picture used here is the optimistic case.

### 2.4 Ionization: the scoop only catches ions

The bible says the field "ionizes interstellar hydrogen". It can't.
A static field does no work, and neutral atoms ignore it.

The local cloud the Sun sits in is roughly ⅓ ionized. Photoionizing
the column ahead of a 14,000 km mouth, even generously, takes
**≥ 6×10¹⁷ W**. The cross-section is so small that UV goes straight
through.

So budget on **ion** density: ~0.07 cm⁻³ in warm clouds, 0.005 cm⁻³
in hot gas. This affects the review's P12 Bubble-density point too.


## 3. Size and mass

The minimum is one hoop at the mouth, with its weakest field (at the
centre) equal to B_c:
- Conductor: M = 4ρ_c·Φ/(μ₀J_e).
- Structure: whatever carries the hoop tension, μ₀I²/4π·(ln 8R/b − 1).

A real funnel needs more winding between the mouth and the throat:
- Several times this for a short, dish-like funnel.
- The funnel's length-to-radius ratio times this for a
  Schattschneider-type long funnel.

| ṁ kg/s | gas | mouth km | throat km (≤ B_t) | hoop conductor kt, J_e 10⁹ / 10¹⁰ | hoop structure kt, fibre / CNT |
|---|---|---|---|---|---|
| 0.3 | bible 0.1 | 4,400 | 2.4 (2 T) | 940 / 94 | 240 / 24 |
| 1 | bible 0.1 | 8,000 | 4.3 (2 T) | 3,100 / 310 | 1,500 / 150 |
| 3.1 | bible 0.1 | 14,000 | 7.6 (2 T) | 9,700 / 970 | 8,600 / 860 |
| 5 | bible 0.1 | 17,800 | 9.7 (2 T) | 15,600 / 1,560 | 17,900 / 1,800 |
| 1 | warm ions | 9,500 | 5.2 (1.7 T) | 3,700 / 370 | 1,900 / 190 |
| 3.1 | warm ions | 16,800 | 9.1 (1.7 T) | 11,600 / 1,160 | 10,400 / 1,040 |
| 1 | hot bubble | 35,600 | 232 (3 mT) | 14,000 / 1,400 | 7,400 / 750 |
| 3.1 | hot bubble | 62,700 | 409 (3 mT) | 43,000 / 4,300 | 42,000 / 4,200 |

Notes:
- **Conductor mass depends on flux, not on coil size.** A bigger,
  weaker hoop uses the same tape and *less* structure. That is why
  the answer is "huge and thin", not "small and strong".
- The hoop current is 8–17 MA. Packed into a thin cable, its own
  field at the cable surface is tens of tesla. The cable has to be
  spread out, so add jacket mass.
- At 1 kg/s in warm gas, the best case is ~0.5 Mt of conductor plus
  structure for the mouth alone. That is about one ship's mass. Hot
  Bubble gas is 4× worse, and the map is partly hot gas.
- The bible books 14 kt.

### What the mouth gets you even without intake

The mouth hoop is also the **parachute** (a magnetic sail) and the
**glance** deflector. Those jobs are honest physics with no throat
and no stopping problem:
- Drag is ρv²·πR², 93 MN at 0.1c for the 14,000 km mouth, which is
  0.02 G on 470 kt.
- Drag falls as v², so braking from 0.10c takes ~11 years to reach
  0.03c and ~43 years to reach 0.01c. The torch finishes from
  around 0.03c, as the bible already says.


## 4. The energy question

### 4.1 Stopping the stream can't be done thermally

At 0.10c, stopping one kilogram releases ½v² = 4.5×10¹⁴ J. That is
**70% of what fusing it yields**.

| ṁ | ram power | waste at η = 97% | at 99% |
|---|---|---|---|
| 0.3 kg/s | 135 TW | 4 TW | 1.3 TW |
| 3.1 kg/s | 1.4 PW | 42 TW | 14 TW |

The radiator handles 37 GW. Keeping converter waste to 10 GW at
3.1 kg/s would take η ≈ 99.9993%.

At relativistic speed it gets worse. The converter has to beat
1 − ε/(γ−1) just to break even on energy:
- 85% at 0.3c.
- 95% at 0.5c.
- 98.6% at 0.75c.

### 4.2 The converter can't be compact either

Both known ways to decelerate a plasma stream need hardware the size
of the mouth.

- **Electrostatic** (a retarding potential, as in the reactor's
  direct converter).
  - The electrons have to come off first. Otherwise the same 4.7 MV
    that stops each proton speeds up an electron, the two land on
    the same plate, and you collect heat, not current.
  - Electrons and protons can only be pulled apart over about one
    proton gyroradius, and the captured stream is many gyroradii
    wide at every point along the funnel. That is what being
    captured means (§2.1).
  - Separating them means unwinding the field back to mouth scale.
  - The stream also shields itself; its Debye length is metres.
- **Inductive** (the plasma pushes a field back, and the coil
  current changes).
  - This needs the plasma's pressure to be comparable to the field
    pressure, i.e. β ~ 1. The stream's β relative to the scoop
    field is ≈ B_c/B: ~1 at the mouth and smaller everywhere
    inward.
  - Only a region where the field is ~B_c can take the stream's
    energy, and by flux conservation that region is mouth-sized.

This revises the review. The "electrostatic decelerator, waste heat
sets cruise" fix is withdrawn.

### 4.3 The two options

**Option 1: we model the converter, and the IS drive takes
electrical power.**
- Section 4.2 means "our" hardware would be a mouth-sized converter
  handling PW, with an electron-separation problem nobody knows how
  to solve.
- That is a worse hand-wave than a black box, because it pretends
  to be engineering.
- The other motivation doesn't hold either. The reactor's 63 GW_e
  is ~2–3% of torch power, which is not a thrust boost worth
  modelling.
- What 63 GW_e *can* do in-system is electric thrusters. At
  50 km/s exhaust that is ~2.5 MN of lateral or attitude thrust,
  about 5 mm/s² on 470 kt. That fills the review's P7 hole (no RCS)
  and gives the reactor a real job.
- Recommendation: reject Option 1 as the propulsion path. Keep the
  "reactor feeds the RCS" idea.

**Option 2: pass-through, inside the IS box.**
- The funnelled stream enters the box at v and leaves faster. It is
  never stopped, so there are no petawatts in our hardware and no
  converter.
- The black box's new claim is "it burns a passing stream." That is
  the same family as the claim it already makes, catalysed p-p, and
  it stays inside the box.
- An ideal pass-through ramjet has no wall. Exhaust speed is
  √(v² + 2εc²) and thrust is ṁ(v_ex − v).
- Acceleration is constant at ρAεc²/M until fusion power reaches the
  nozzle rating, then falls as 1/v.

| ṁ at 0.1c | thrust | fusion power | accel, 470 kt |
|---|---|---|---|
| 0.3 kg/s | 5 MN | 0.19 PW | 1.1 mG |
| 1 kg/s | 17 MN | 0.64 PW | 3.6 mG |
| 3.1 kg/s | 52 MN | 2.0 PW | 11 mG |
| 5 kg/s | 83 MN | 3.2 PW (nozzle limit) | 18 mG |

Compare the bible's current cruise figure, 4 mG at 3.1 kg/s.

Two consequences:
- **The box's intake has to accept the flux tube.** At the
  mirror-capped ≤ 2 T, the throat is 4–8 km in radius for
  1–3 kg/s. Either the IS intake is km-scale, which is our funnel
  feeding a big box, or the box also swallows the funnel's inner
  stages, which is a bigger box claim. This is the one remaining
  real choice.
- **The tanks don't refill at cruise.** Banking a kilogram costs
  ½v², the same stopping problem. Scooped fuel is burned, not
  stored. Tanks refill at ports, and possibly late in braking, but
  by then ṁ ∝ v is tiny. The bible's "1–2 year refill" goes away
  unless the box also banks.


## 5. Time dilation

Pass-through removes the wall but not the power bill. For an ideal
ramjet, fusion power = thrust × v. Reaching speed v costs
(γ−1)Mc² no matter what the scoop does.

| speed | γ | years at 3.2 PW | years at 11 PW | power for 1 G at that speed |
|---|---|---|---|---|
| 0.1c | 1.005 | 2 | 0.6 | 138 PW |
| 0.3c | 1.048 | 20 | 6 | 415 PW |
| 0.5c | 1.155 | 65 | 19 | 690 PW |
| 0.75c | 1.51 | 214 | 62 | 1.0 EW |
| 0.87c | 2.03 | 430 | 125 | 1.2 EW |

These are lower bounds: the burn has to fit within the nozzle
rating, and the scoop also has to supply ṁ = P/εc², 5 kg/s at
3.2 PW.

Meaningful dilation (γ ≳ 1.5 reached within years) needs either:
- **~100× the IS power rating.** The plume then becomes a weapon
  that reaches across a solar system, which is a setting decision.
- **~100× less ship.**

The 0.12c wall is gone, but on today's numbers the practical cruise
is "however long you are willing to burn": ~0.2–0.3c over a couple
of decades at full rating.


## 6. Decisions

1. **Throat: our funnel, or IS's intake?** With pass-through, this is
   the only open placement question (§4.3).
2. **Who makes the mouth hoop?**
   - **Yard-built**, with honest mass: ~0.5–2 Mt at 1–3 kg/s with
     optimistic materials, which is more than the ship. It doubles
     as the parachute and glance deflector.
   - **IS sells the tape and fibre**: a materials gimme. It stays
     localized and physical, because the flux and the energy really
     are out there. Only J_e and σ/ρ are magic.
3. **Intake target.** Below ~1 kg/s the scoop stops working (§2.1),
   so there is no small version.
4. **Time dilation.** If you want it, the IS power rating has to go
   up by ~100×, with the consequences for the plume.

Bible edits once you decide:
- Delete the 0.12c "conservation wall" (lines 249–258, 1170–1177).
- Collector mass: 14 kt → real figure (line 752).
- "Ionizes interstellar hydrogen" (line 1291).
- "1–2 yr refill" (line 1259).
- Rework the acceleration tables (303–314, 1192–1200) as pass-through.
