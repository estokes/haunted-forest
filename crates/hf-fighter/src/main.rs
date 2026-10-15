//! Close the fighter's down-and-back mission on an Earth-like planet: from
//! orbit, deorbit, entry, a sortie, a landing on a 300 m dirt strip, and
//! back to orbit on what it brought down, clean but for its laser. Prints
//! the mass and volume budget, the strip, the climb and the entry, and how
//! they move with the assumptions.

use anyhow::Result;
use hf_engine::{cycle::EngineSpec, parts::G0};
use hf_fighter::{
    aero::Aero,
    ascent::{Ascent, AscentSpec, Table},
    entry::{tank_soak, EntryReport, EntrySpec, Peak},
    heat::Insulation,
    planet::EARTH,
    strip::StripSpec,
};

/// Packed two-size shot (packing 0.80) with ammonia in the voids, kg/m3.
const SLURRY: f64 = 696.;
const AMMONIA_BOILOFF: f64 = 1.17e6;

#[derive(Clone, Copy)]
struct Vehicle {
    /// Empty, with the laser.
    empty: f64,
    tanks_dry: f64,
    /// Propellant kept in orbit, kg.
    reserve: f64,
    /// Propellant for the sortie before landing, kg.
    sortie: f64,
    internal: f64,
    conformal: f64,
    /// Belly area over tanks, m2.
    belly_tanks: f64,
}

impl Default for Vehicle {
    fn default() -> Self {
        Self {
            empty: 7500.,
            tanks_dry: 600.,
            reserve: 150.,
            sortie: 500.,
            internal: 4.6,
            conformal: 7.5,
            belly_tanks: 15.,
        }
    }
}

struct Closure {
    orbit_mass: f64,
    takeoff: f64,
    ascent: Ascent,
    entry_mass: f64,
    deorbit: f64,
    aboard: f64,
    capacity: f64,
}

fn close(
    v: &Vehicle,
    spec: &AscentSpec,
    table: &Table,
    entry: &EntrySpec,
) -> Result<Closure> {
    let orbit_mass = v.empty + v.tanks_dry + v.reserve;
    let takeoff = hf_engine::gas::brent(
        |m| Ok(spec.fly(&EARTH, &Aero::default(), table, m).final_mass - orbit_mass),
        orbit_mass + 2000.,
        40000.,
        1e-6,
    )?;
    let ascent = spec.fly(&EARTH, &Aero::default(), table, takeoff);
    let entry_mass = takeoff + v.sortie;
    let dv = entry.fly(&EARTH, &Aero::default()).deorbit_dv;
    let deorbit = entry_mass * ((dv / (spec.rocket_isp * G0)).exp() - 1.);
    let aboard = entry_mass + deorbit - v.empty - v.tanks_dry;
    Ok(Closure {
        orbit_mass,
        takeoff,
        ascent,
        entry_mass,
        deorbit,
        aboard,
        capacity: (v.internal + v.conformal) * SLURRY,
    })
}

fn peak(name: &str, p: &Peak) {
    println!(
        "    {name:28} {:5.0} K  {:6.2} MW/m2  at {:4.0} km, {:4.0} m/s, t {:4.0} s; {:5.0} MJ/m2 over entry",
        p.temperature,
        p.flux / 1e6,
        p.altitude / 1e3,
        p.speed,
        p.time,
        p.load / 1e6
    );
}

fn report_entry(e: &EntryReport, spec: &EntrySpec) {
    println!(
        "  deorbit {:.0} m/s from {:.0} km to a {:.0} km periapsis; interface at {:.0} km, {:.0} m/s, {:.2} deg",
        e.deorbit_dv,
        spec.orbit / 1e3,
        spec.periapsis / 1e3,
        spec.interface / 1e3,
        e.interface_speed,
        e.interface_gamma.to_degrees()
    );
    println!(
        "  {:.0} s to Mach {:.1} at {:.0} km, {:.0} km downrange; peak {:.2} g, {:.0} kPa",
        e.duration,
        e.end_mach,
        e.end_altitude / 1e3,
        e.downrange / 1e3,
        e.peak_g,
        e.peak_q / 1e3
    );
    peak(&format!("nose, r {:.0} cm", spec.nose_radius * 100.), &e.nose);
    peak(&format!("leading edge, r {:.0} cm", spec.edge_radius * 100.), &e.edge);
    for (x, p) in spec.stations.iter().zip(&e.stations) {
        peak(&format!("windward, {x} m back"), p);
    }
    peak("lee side, 6 m back", &e.lee);
    println!("    windward flow turbulent from t {:.0} s", e.turbulent_from);
}

fn main() -> Result<()> {
    let planet = &EARTH;
    let aero = Aero::default();
    let v = Vehicle::default();
    let spec = AscentSpec::new(EngineSpec::default(), 250e3, 4e6, 0.6)?;
    let table = spec.engine_table();
    let strip = StripSpec::default();
    let curve = StripSpec::engine_curve(&spec.engine);
    let entry_spec = EntrySpec::default();

    println!("== Down and back, Earth-like, clean but for the laser");
    println!(
        "  rocket mode: {:.0} kN vacuum at 4 MPa, area ratio {:.1} in a {:.2} m2 exit, {:.0} s on 1:0.24 slurry",
        spec.rocket_thrust / 1e3,
        spec.area_ratio,
        spec.exit_area,
        spec.rocket_isp
    );
    let c = close(&v, &spec, &table, &entry_spec)?;
    let a = &c.ascent;
    println!(
        "  in orbit, empty: {:.0} kg ({:.0} empty, {:.0} tanks, {:.0} reserve)",
        c.orbit_mass, v.empty, v.tanks_dry, v.reserve
    );
    println!(
        "  takeoff from the strip: {:.0} kg; entry: {:.0} kg; before the deorbit burn: {:.0} kg",
        c.takeoff,
        c.entry_mass,
        c.entry_mass + c.deorbit
    );
    println!(
        "  propellant aboard leaving orbit: {:.0} kg = {:.1} m3 of {:.1} m3 ({:.0}% of capacity, {:.1} m3 spare)",
        c.aboard,
        c.aboard / SLURRY,
        v.internal + v.conformal,
        100. * c.aboard / c.capacity,
        (c.capacity - c.aboard) / SLURRY
    );

    println!("== Climb, from {:.0} kg", c.takeoff);
    for s in &a.segments {
        println!(
            "    Mach {:4.2} at {:5.0} m: {:15} {:5.0} kN against {:4.0} kN",
            s.0,
            s.1,
            s.2,
            s.3 / 1e3,
            s.4 / 1e3
        );
    }
    println!(
        "  air-breathing to Mach {:.2} at {:.0} km: {:.0} s, {:.0} kg ({:.0} ammonia, {:.0} dust)",
        a.handover_mach,
        a.handover_altitude / 1e3,
        a.air_time,
        a.air_diluent + a.air_dust,
        a.air_diluent,
        a.air_dust
    );
    let b = &a.rocket;
    println!(
        "  rocket: {:.0} s, {:.0} kg, circularising {:.0} m/s; {:.2} km/s in all; peak {:.1} g, {:.0} kPa",
        b.burn_time,
        b.propellant,
        b.circularise_dv,
        b.dv / 1e3,
        b.max_g,
        b.max_q / 1e3
    );
    println!(
        "  heating on the way up: nose {:.0} K at {:.0} km and {:.0} m/s, leading edge {:.0} K",
        b.nose_t,
        b.nose_at.0 / 1e3,
        b.nose_at.1,
        b.edge_t
    );

    println!(
        "== The strip: 300 m of hard dirt (rolling {}, braking {})",
        strip.rolling, strip.braking
    );
    let to = strip.takeoff(c.takeoff, &curve);
    println!(
        "  takeoff at {:.0} kg: {:.0} m, lifting off at {:.0} m/s; engine {:.1} MW, fan {:.1} MW, nozzle {:.0} deg down",
        c.takeoff,
        to.roll,
        to.speed,
        to.engine_power / 1e6,
        to.fan_power / 1e6,
        to.nozzle_angle.to_degrees()
    );
    let heaviest = strip.heaviest(300., &curve)?;
    println!("  heaviest takeoff in 300 m: {heaviest:.0} kg");
    let land = strip.land(c.takeoff, &curve);
    println!(
        "  landing at {:.0} kg: touchdown {:.0} m/s, roll {:.0} m",
        c.takeoff, land.touchdown, land.roll
    );
    for (name, s) in [
        ("fan motor 20 MW", StripSpec { fan_max: 20e6, ..StripSpec::default() }),
        (
            "soft dirt, rolling 0.08, braking 0.2",
            StripSpec { rolling: 0.08, braking: 0.2, ..StripSpec::default() },
        ),
        ("wing CL 0.8 on the run", StripSpec { cl_ground: 0.8, ..StripSpec::default() }),
    ] {
        let t = s.takeoff(c.takeoff, &curve);
        let l = s.land(c.takeoff, &curve);
        println!(
            "    {name:38}: takeoff {:4.0} m, heaviest in 300 m {:5.0} kg; landing touchdown {:3.0} m/s, roll {:4.0} m",
            t.roll,
            s.heaviest(300., &curve).unwrap_or(f64::NAN),
            l.touchdown,
            l.roll
        );
    }

    println!(
        "== Entry at {:.0} kg ({:.0} kg/m2 of wing)",
        c.entry_mass,
        c.entry_mass / aero.wing
    );
    let es = EntrySpec { mass: c.entry_mass, ..EntrySpec::default() };
    let e = es.fly(planet, &aero);
    report_entry(&e, &es);
    println!(
        "  belly tanks behind the 6 m station's skin, held at 300 K by boiling ammonia:"
    );
    for t in [0.03, 0.05, 0.10] {
        let ins = Insulation { thickness: t, ..Insulation::default() };
        let s = tank_soak(&e, &ins, 300., 8000., 0.85);
        println!(
            "    {:2.0} cm of insulation ({:.0} kg/m2): {:5.1} MJ/m2 into the tank, peak {:5.0} W/m2 at t {:4.0} s; {:4.0} kg of ammonia boiled off under {} m2",
            t * 100.,
            t * ins.density,
            s.heat / 1e6,
            s.peak_flux,
            s.peak_time,
            s.heat * v.belly_tanks / AMMONIA_BOILOFF,
            v.belly_tanks
        );
    }
    println!("  sharper and blunter:");
    for (rn, re) in [(0.03, 0.01), (0.05, 0.02), (0.2, 0.05)] {
        let s = EntrySpec {
            mass: c.entry_mass,
            nose_radius: rn,
            edge_radius: re,
            ..EntrySpec::default()
        };
        let r = s.fly(planet, &aero);
        println!(
            "    nose {:2.0} cm: {:4.0} K; edge {:1.0} cm: {:4.0} K",
            rn * 100.,
            r.nose.temperature,
            re * 100.,
            r.edge.temperature
        );
    }
    for (name, s) in [
        (
            "steeper: periapsis 0 km",
            EntrySpec { mass: c.entry_mass, periapsis: 0., ..EntrySpec::default() },
        ),
        (
            "angle of attack 30 deg",
            EntrySpec {
                mass: c.entry_mass,
                alpha_high: 30f64.to_radians(),
                ..EntrySpec::default()
            },
        ),
        (
            "early transition, Re 5e5",
            EntrySpec { mass: c.entry_mass, transition: 5e5, ..EntrySpec::default() },
        ),
    ] {
        let r = s.fly(planet, &aero);
        println!(
            "    {name:28}: nose {:4.0} K, edge {:4.0} K, windward 1 m {:4.0} K, 6 m {:4.0} K, peak {:.1} g",
            r.nose.temperature,
            r.edge.temperature,
            r.stations[0].temperature,
            r.stations[2].temperature,
            r.peak_g
        );
    }

    println!("== Sensitivity of the budget");
    for (name, vv) in [
        ("empty +1 t", Vehicle { empty: v.empty + 1000., ..v }),
        ("empty +2 t", Vehicle { empty: v.empty + 2000., ..v }),
        ("sortie 1.5 t", Vehicle { sortie: 1500., ..v }),
    ] {
        let c = close(&vv, &spec, &table, &entry_spec)?;
        println!(
            "    {name:14}: takeoff {:5.0} kg, entry {:5.0} kg, aboard {:4.1} m3 of {:.1} ({:3.0}%), 300 m takeoff {}",
            c.takeoff,
            c.entry_mass,
            c.aboard / SLURRY,
            vv.internal + vv.conformal,
            100. * c.aboard / c.capacity,
            if strip.takeoff(c.takeoff, &curve).roll <= 300. { "yes" } else { "no" }
        );
    }
    for (name, s) in [
        ("rocket 2 MPa", AscentSpec::new(EngineSpec::default(), 250e3, 2e6, 0.6)?),
        ("rocket 180 kN", AscentSpec::new(EngineSpec::default(), 180e3, 4e6, 0.6)?),
        ("rocket 350 kN", AscentSpec::new(EngineSpec::default(), 350e3, 4e6, 0.6)?),
    ] {
        let t = s.engine_table();
        let c = close(&v, &s, &t, &entry_spec)?;
        println!(
            "    {name:14}: {:4.0} s; takeoff {:5.0} kg, aboard {:4.1} m3 ({:3.0}%), rocket {:.2} km/s",
            s.rocket_isp,
            c.takeoff,
            c.aboard / SLURRY,
            100. * c.aboard / c.capacity,
            c.ascent.rocket.dv / 1e3
        );
    }
    Ok(())
}
