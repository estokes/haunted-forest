# The fighter's equipment, by its parts (book/small-craft.md). Pure Python:
# modeling/fighter.py takes its masses from here.
#
# Near-ideal materials, as for the structure: CNT at 1.4 t/m3 for anything
# cold enough, carbide-skinned CNT for anything hot, and electric machines
# at 100 kW/kg. At those strengths rotors, casings and pressure walls are
# set by minimum gauge and by aerodynamics, not by stress, so most of the
# mass is in the electric machines and what feeds them. Each item has a
# central mass and a high one, the high for what today's practice would
# add back: more conservative gauges, bearings, accessories, margins.

import math

RHO = 1400.0                # CNT, kg/m3
GAUGE = 0.6e-3              # m
MACHINE_KW_KG = 100.0       # superconducting motors and generators
CONVERTER_KW_KG = 250.0     # power electronics, per stage of conversion

TURBOPUMP_MW = 50.0
ENGINE_MOTORS_MW = 45.0     # what the fan and compressor want at Mach 2
LIFT_FAN_MW = 30.0
LASER_MW = 8.0              # electricity into the laser
MAX_LANDING_KG = 21000.0


def decoy():
    """A space decoy: kg of dust, film, and the whole, and the burn it can
    fly. It matches the fighter's acceleration in rocket mode with a small
    dust engine of the fighter's exhaust speed, and its plume's light with
    the same engine overdriven: the fighter's exhaust, particle-laden,
    leaves a 0.6 m2 exit near 2,300 K, ~1 MW as a black body and some
    10 MW from the plume beyond, so the decoy burns ~10 MW of dust's heat.
    Its radar and lidar return is a film shell it unfolds to the
    fighter's size, and a radar repeater."""
    burn_s, accel, ve = 60.0, 3.0 * 9.80665, 14e3
    light_w, dust_j = 10e6, 210e6        # W to show; J per kg of dust (a tonne is ~50 t of TNT)
    dry = 4.0 + 3.0 + 1.5 + 6.0           # engine, electronics and repeater, film, canister and ejector
    light = light_w * burn_s / dust_j
    whole = dry + light
    for _ in range(20):                   # the propellant for its own Δv, at the fighter's exhaust speed
        prop = whole * (1 - math.exp(-accel * burn_s / ve))
        whole = dry + light + prop
    return {"dust_light": light, "propellant": prop, "film": 1.5, "kg": whole, "burn_s": burn_s,
            "dv": accel * burn_s}


DECOY_KG = decoy()["kg"]


def skin(area, layers=2, gauge=GAUGE, core=1.3):
    """A sandwich wall of `area` m2, kg."""
    return RHO * area * layers * gauge * core


def rotor_stage(d_tip, hub_ratio, blades, aspect=2.0, tc=0.05):
    """One rotor and its stator: blades of CNT at a mean thickness of `tc`
    of chord, a disk as heavy as its blades (strength is nearly free), the
    vanes as heavy again."""
    span = d_tip / 2 * (1 - hub_ratio)
    chord = span / aspect
    blade = span * chord * tc * chord * 0.7 * RHO
    return 3 * blades * blade


def items():
    """name -> (central kg, high kg, what it is)."""
    out = {}
    # Engine: two fan stages and three compressor stages on their own
    # motors, the casings, the ammonia-cooled hot section and the nozzle.
    fan = 2 * rotor_stage(0.80, 0.35, 24)
    comp = 3 * rotor_stage(0.62, 0.55, 40)
    casings = skin(math.pi * 0.95 * 1.6, layers=2) * 2.0  # with blade containment
    hot = skin(math.pi * 1.0 * 2.6, layers=2) + 1.0 * math.pi * 1.0 * 2.6 + 40  # coated walls, jacket, injectors
    nozzle = 60 + 20  # petals to an area ratio of ~18, and their actuators
    doors = 50        # fan-face, ramjet entry and bypass doors, actuators
    controls = 40     # pitch-change, valves, sensors, controllers
    core = fan + comp + casings + hot + nozzle + doors + controls
    out["engine"] = (core, 2.0 * core, f"fan, compressor, casings, hot section, nozzle, doors ({core:.0f} kg)")
    motors = ENGINE_MOTORS_MW * 1e3 / MACHINE_KW_KG
    out["engine_motors"] = (motors, 1.3 * motors, f"{ENGINE_MOTORS_MW:.0f} MW at {MACHINE_KW_KG:.0f} kW/kg")
    out["swivel_nozzle"] = (110, 220, "three-bearing duct, 1.0 m across, its bearings and drives")
    # Turbopump: the generator; a 6-stage turbine 7 cm in radius; a 10 MPa
    # reverter of a few litres; a slurry pump; magnetic bearings; housing.
    gen = TURBOPUMP_MW * 1e3 / MACHINE_KW_KG
    out["turbopump"] = (gen + 70, 1.3 * gen + 150, f"generator {gen:.0f} kg, turbine, reverter, pump, bearings")
    conv = (TURBOPUMP_MW + ENGINE_MOTORS_MW + LIFT_FAN_MW) * 1e3 / CONVERTER_KW_KG
    out["power_electronics"] = (conv + 30, 2.0 * conv, f"{TURBOPUMP_MW + ENGINE_MOTORS_MW + LIFT_FAN_MW:.0f} MW of conversion, and the bus")
    fan_motor = LIFT_FAN_MW * 1e3 / MACHINE_KW_KG
    lift = rotor_stage(1.5, 0.3, 18, aspect=3.0) * 2 + skin(math.pi * 1.55 * 0.5) + 30 + 2 * skin(1.8) + 20
    out["lift_fan"] = (fan_motor + lift, 1.3 * fan_motor + 2 * lift, f"motor {fan_motor:.0f} kg, rotors, duct, vane box, doors")
    out["inlets_and_ducts"] = (2 * skin(9.0) + 2 * 15 + 20, 250, "two ducts, translating cones, actuators")
    # Gear for dirt, at its heaviest landing: today ~3.5% of that mass in
    # steel and titanium; struts of near-ideal material, wheels, tyres and
    # brakes much as now.
    out["landing_gear"] = (0.020 * MAX_LANDING_KG, 0.035 * MAX_LANDING_KG, "struts, wheels, tyres, brakes, ~2% of 21 t")
    out["capsule"] = (100 + 250 + 50 + 40 + 40, 550, "pilot, ~0.25 m3 of liquid, shell, escape motor and recovery, life support, BCI")
    # The laser: electricity in at LASER_MW; its power conversion, the
    # accelerator and its magnets, the undulator and optics, shielding for
    # the pilot from the electron beam, and the turret.
    laser = LASER_MW * 1e3 / CONVERTER_KW_KG + 150 + 80 + 150
    out["laser"] = (laser, 1000, "conversion, accelerator, undulator, optics, shielding")
    out["laser_turret"] = (120, 200, "0.5 m director, its drives and fast optics")
    out["radar"] = (80, 120, "")
    out["avionics"] = (80, 120, "")
    out["flight_controls_and_rcs"] = (120, 220, "elevon and fin actuators, ~16 reaction thrusters and their feeds")
    out["thermal"] = (100, 200, "heat exchangers to the bypass air, loops")
    out["electrical"] = (100, 150, "wiring, starting power")
    out["propellant_system"] = (150, 250, "magnetic dust feed, ammonia pumps, transfer between tanks, gauging")
    out["bays"] = (80, 120, "two bays' doors, ejectors and hot seals")
    out["slats_and_strakes"] = (55, 90, "four leading-edge slat segments and their actuators; two strakes")
    out["canards"] = (165, 270, "two all-moving canards, ~3 m2 each, their pivots and actuators")
    # Sensors: six or more wide-angle windows round the airframe, imaging
    # and IR, for the pilot's view all round and missile warning; the
    # laser's 0.5 m directors double as the targeting telescopes.
    out["sensor_windows"] = (40, 70, "~11 wide-angle imaging and IR windows, all round, missile warning")
    out["laser_turret_ventral"] = (120, 200, "a second director under the belly, retracting behind doors for entry")
    # Countermeasures in the air, where the engine cannot be hidden.
    out["ew"] = (60, 100, "receivers and jammer, its antennas in a fin-tip pod as on the Mirage")
    out["chaff_flares"] = (70, 100, "two dispensers, ~120 cartridges between them")
    out["jammer_pods"] = (6 * 12 + 15, 6 * 18 + 25, "six expendable jammers, ~1 m and 12 kg each, flying on batteries, and their launcher")
    out["towed_decoys"] = (3 * 3 + 15, 3 * 5 + 25, "three towed jammers on fibre, and the reel")
    # In space: decoys that fly a burn like the fighter's (decoys below).
    out["space_decoys"] = (4 * DECOY_KG + 20, 4 * DECOY_KG * 1.4 + 30, f"four, {DECOY_KG:.0f} kg each, and their tubes")
    return out


def total(which=0):
    return sum(v[which] for v in items().values())


if __name__ == "__main__":
    for name, (c, h, what) in sorted(items().items(), key=lambda kv: -kv[1][0]):
        print(f"{name:24} {c:6.0f} {h:6.0f}  {what}")
    print(f"{'total':24} {total(0):6.0f} {total(1):6.0f}")
