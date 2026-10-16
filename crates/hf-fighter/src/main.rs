//! Close the fighter's down-and-back mission on an Earth-like planet: from
//! orbit, deorbit, entry, a sortie, a landing on a 300 m dirt strip, and
//! back to orbit on what it brought down, clean but for its laser. Prints
//! what the tanks should hold for a fixed volume — which liquid, and how
//! dust-rich the rocket burns — then the mission on the best of them: the
//! budget, the strip, the climb and the entry, and how they move with the
//! assumptions.

use anyhow::Result;
use hf_engine::{
    blade::{blade_heat, coolant_heat},
    cycle::{EngineSpec, Point},
    gas::Diluent,
    parts::{Turbopump, TurbopumpSpec, G0},
};
use hf_fighter::{
    aero::{Aero, Canard, WAVE},
    ascent::{Ascent, AscentSpec, Table},
    entry::{tank_soak, EntryReport, EntrySpec, Peak},
    fill::{filled_density, volume},
    heat::Insulation,
    planet::{air, Planet, EARTH},
    strip::StripSpec,
};
use rayon::prelude::*;

/// The liquids compared, and the rocket mixes, kg of liquid per kg of dust.
const LIQUIDS: [Diluent; 4] =
    [Diluent::Ammonia, Diluent::Aqueous(0.33), Diluent::Aqueous(0.15), Diluent::WATER];
const MIXES: [f64; 7] = [0.05, 0.1, 0.15, 0.2, 0.25, 0.35, 0.5];
/// The rocket mode's thrust, chamber pressure and exit.
const ROCKET: (f64, f64, f64) = (250e3, 4e6, 0.6);

#[derive(Clone, Copy)]
struct Vehicle {
    /// Empty, with the laser.
    empty: f64,
    tanks_dry: f64,
    /// Propellant kept in orbit, kg.
    reserve: f64,
    /// Propellant for the sortie before landing, kg.
    sortie: f64,
    tankage: f64,
    /// Wing area, m2.
    wing: f64,
    /// Zero-lift drag area the aircraft adds to the polar's, D/q, m2: its
    /// skin beyond the 14.2 m airframe the polar was set for, at ~0.0035
    /// of friction per m2, and its wing-root bays.
    drag_area: f64,
    /// Wave drag at Mach 1, D/q, m2.
    wave: f64,
    /// The wing's leading-edge sweep, degrees, and aspect ratio.
    sweep: f64,
    aspect: f64,
    /// Close-coupled canards, or tailless with strakes.
    canard: Option<Canard>,
    /// Centre of gravity, m from the nose, empty and with every tank full.
    cg: (f64, f64),
    /// Belly area over tanks, m2, and the heat each m2 takes through
    /// entry, J.
    belly_tanks: f64,
    soak: f64,
}

impl Default for Vehicle {
    fn default() -> Self {
        Self {
            empty: 6380.,
            tanks_dry: 0.,
            reserve: 150.,
            sortie: 500.,
            tankage: 25.1,
            wing: 55.37,
            drag_area: 0.220,
            wave: WAVE,
            sweep: 68.,
            aspect: 1.21,
            canard: Some(Canard::default()),
            cg: (12.47, 11.77),
            belly_tanks: 15.,
            soak: 1.6e6,
        }
    }
}

impl Vehicle {
    fn aero(&self) -> Aero {
        let a = Aero {
            wing: self.wing,
            drag_area: self.drag_area,
            wave: self.wave,
            sweep: self.sweep,
            aspect: self.aspect,
            cg: (self.cg.0 + self.cg.1) / 2.,
            ..Aero::default()
        };
        match self.canard {
            Some(c) => Aero { canard: Some(c), ..a },
            None => a.tailless(),
        }
    }

    /// Centre of gravity at `mass` kg, the tanks filling evenly.
    fn cg_at(&self, mass: f64) -> f64 {
        let full = self.empty + self.tankage * 700.;
        let t = ((mass - self.empty) / (full - self.empty)).clamp(0., 1.);
        self.cg.0 + (self.cg.1 - self.cg.0) * t
    }

    fn entry(&self) -> EntrySpec {
        EntrySpec { sweep: self.sweep.to_radians(), ..EntrySpec::default() }
    }
}

/// The liquid's share of the propellant at cruise, kg per kg of dust, and
/// the cruise's km per tonne and per m3 of tank: Mach 0.9 at 11 km, fan at
/// full ratio, on the first of a few settings with the thrust for it.
fn cruise(engine: &EngineSpec, mass: f64) -> Result<(f64, f64, f64)> {
    let at = air(11000.);
    let v = 0.9 * at.sound;
    let q = 0.5 * at.rho * v * v;
    let drag = Aero::default().drag(0.9, q, mass * 9.80665);
    let rho = engine.turbopump.diluent.density();
    for (comp, t4) in [(1., None), (1.5, None), (1.5, Some(1500.)), (2.5, Some(2000.))] {
        let pt = Point {
            altitude: 11000.,
            mach: 0.9,
            fan_pr: 3.5,
            compressor_pr: comp,
            t4,
            ..Default::default()
        };
        let Ok(k) = engine.throttle_for(&pt, drag) else { continue };
        let r = engine.evaluate(&Point { throttle: k, ..pt })?;
        return Ok((
            r.diluent / r.dust,
            v / (r.diluent + r.dust),
            v / 1e3 / volume(r.dust, r.diluent, rho),
        ));
    }
    anyhow::bail!("no setting cruises at {mass} kg")
}

/// Where the fighter flies the mission, and how: on which planet, whether
/// it climbs back out from a standing start (it landed) or from flight at
/// Mach `from` (it never landed), and any braking burn before entry.
#[derive(Clone, Copy)]
struct Mission {
    planet: Planet,
    from: f64,
    braking: f64,
}

impl Default for Mission {
    fn default() -> Self {
        Self { planet: EARTH, from: 0., braking: 0. }
    }
}

struct Closure {
    orbit_mass: f64,
    takeoff: f64,
    ascent: Ascent,
    entry_mass: f64,
    deorbit: f64,
    dust: f64,
    liquid: f64,
    volume: f64,
}

/// The climb back to orbit: the mass it starts at, and how it goes.
fn climb(
    v: &Vehicle,
    spec: &AscentSpec,
    table: &Table,
    mission: &Mission,
) -> Result<(f64, Ascent)> {
    let orbit_mass = v.empty + v.tanks_dry + v.reserve;
    let aero = v.aero();
    let fly = |m| spec.fly(&mission.planet, &aero, table, m, mission.from);
    let takeoff = hf_engine::gas::brent(
        |m| Ok(fly(m).final_mass - orbit_mass),
        orbit_mass + 500.,
        60000.,
        1e-6,
    )?;
    Ok((takeoff, fly(takeoff)))
}

/// The whole mission on a climb already flown.
fn close_on(
    v: &Vehicle,
    spec: &AscentSpec,
    sortie_mix: f64,
    mission: &Mission,
    (takeoff, ascent): (f64, Ascent),
) -> Closure {
    let aero = v.aero();
    let entry_mass = takeoff + v.sortie;
    let dv =
        EntrySpec::default().fly(&mission.planet, &aero).deorbit_dv + mission.braking;
    let deorbit = entry_mass * ((dv / (spec.rocket_isp * G0)).exp() - 1.);
    let d = spec.engine.turbopump.diluent;
    let rocket = ascent.rocket.propellant + deorbit + v.reserve;
    let r = spec.mix;
    let dust = ascent.air_dust + rocket / (1. + r) + v.sortie / (1. + sortie_mix);
    let liquid = ascent.air_diluent
        + rocket * r / (1. + r)
        + v.sortie * sortie_mix / (1. + sortie_mix)
        + v.soak * v.belly_tanks / d.latent();
    Closure {
        orbit_mass: v.empty + v.tanks_dry + v.reserve,
        takeoff,
        ascent,
        entry_mass,
        deorbit,
        dust,
        liquid,
        volume: volume(dust, liquid, d.density()),
    }
}

fn close(
    v: &Vehicle,
    spec: &AscentSpec,
    table: &Table,
    sortie_mix: f64,
) -> Result<Closure> {
    let m = Mission::default();
    Ok(close_on(v, spec, sortie_mix, &m, climb(v, spec, table, &m)?))
}

/// On worlds of Earth's density and heavier gravity, the most the fighter
/// can spend down in the air and still climb back to orbit, flying from a
/// landing or never landing; and the braking burn its entry needs to keep
/// its nose below NOSE_MAX.
fn worlds(v: &Vehicle) -> Result<()> {
    const NOSE_MAX: f64 = 2900.;
    let engine = EngineSpec::default();
    let cr = cruise(&engine, 11000.)?;
    let spec = AscentSpec::new(engine, ROCKET.0, ROCKET.1, ROCKET.2, 0.1)?;
    let table = spec.engine_table();
    let curve = StripSpec::engine_curve(&spec.engine);
    let aero = v.aero();
    println!(
        "== The sortie budget: what is left of {:.1} m3 for the work down in the air, empty {:.0} kg",
        v.tankage, v.empty
    );
    println!("  g     radius   orbit    mission      climb from  budget        braking  nose     landing");
    let mut g = 1.0;
    while g < 2.05 {
        let planet = Planet::earthlike(g);
        for (name, from) in [("lands", 0.0), ("never lands", 0.95)] {
            let mut mission = Mission { planet, from, braking: 0. };
            let c = match climb(v, &spec, &table, &mission) {
                Ok(c) => c,
                Err(_) => {
                    println!("  {g:.1}   {name}: cannot climb to orbit");
                    continue;
                }
            };
            let mut budget = 0.;
            let mut nose = 0.;
            let mut ok = true;
            let mut hot_limited = false;
            for _ in 0..3 {
                let at = |s: f64| {
                    close_on(
                        &Vehicle { sortie: s, ..*v },
                        &spec,
                        cr.0,
                        &mission,
                        c.clone(),
                    )
                };
                if at(0.).volume > v.tankage {
                    ok = false;
                    break;
                }
                budget = hf_engine::gas::brent(
                    |s| Ok(at(s).volume - v.tankage),
                    0.,
                    30000.,
                    1e-6,
                )?;
                let entry = at(budget).entry_mass;
                let fly = |b: f64| {
                    EntrySpec { mass: entry, braking: b, ..v.entry() }
                        .fly(&planet, &aero)
                };
                nose = fly(mission.braking).nose.temperature;
                if nose <= NOSE_MAX + 5.
                    && (mission.braking == 0. || nose >= NOSE_MAX - 30.)
                {
                    break;
                }
                mission.braking = if fly(0.).nose.temperature <= NOSE_MAX {
                    0.
                } else {
                    match hf_engine::gas::brent(
                        |b| Ok(fly(b).nose.temperature - NOSE_MAX),
                        0.,
                        6000.,
                        1e-4,
                    ) {
                        Ok(b) => b,
                        Err(_) => {
                            let hot = |s: f64| {
                                Ok(EntrySpec { mass: at(s).entry_mass, ..v.entry() }
                                    .fly(&planet, &aero)
                                    .nose
                                    .temperature
                                    - NOSE_MAX)
                            };
                            match hf_engine::gas::brent(hot, 0., budget, 1e-3) {
                                Ok(s) => {
                                    budget = s;
                                    nose = NOSE_MAX;
                                    mission.braking = 0.;
                                    hot_limited = true;
                                }
                                Err(_) => ok = false,
                            }
                            break;
                        }
                    }
                };
            }
            let landing = if hot_limited {
                "entry heat limits the budget".to_string()
            } else if from > 0. {
                "-".to_string()
            } else {
                let strip = StripSpec { gravity: 9.80665 * g, ..StripSpec::default() };
                let to = strip.takeoff(c.0, &curve);
                if to.roll < 1. {
                    "vertical".to_string()
                } else if to.roll <= 300. {
                    format!("{:.0} m roll", to.roll)
                } else {
                    "needs a runway".to_string()
                }
            };
            let orbit = planet.circular_speed(200e3);
            if !ok {
                let why = if nose > NOSE_MAX {
                    "no budget: no braking burn keeps the nose cool"
                } else {
                    "no budget: the climb alone overfills the tanks"
                };
                println!(
                    "  {g:.1}  {:6.0} km {:4.1} km/s {name:12} {:5.1} t      {why}",
                    planet.radius / 1e3,
                    orbit / 1e3,
                    c.0 / 1e3
                );
                continue;
            }
            println!(
                "  {g:.1}  {:6.0} km {:4.1} km/s {name:12} {:5.1} t    {:5.2} t (~{:5.0} km) {:5.0} m/s {:5.0} K  {landing}",
                planet.radius / 1e3,
                orbit / 1e3,
                c.0 / 1e3,
                budget / 1e3,
                budget / 1e3 * 2200. / g,
                mission.braking,
                nose
            );
        }
        g += 0.1;
    }
    Ok(())
}

/// One liquid in the tanks: what it does in each regime, and the mission
/// on its best rocket mix.
struct Fill {
    diluent: Diluent,
    per_mw: (f64, f64),
    blades: f64,
    sls: (f64, f64),
    cruise: (f64, f64, f64),
    ramjet: f64,
    /// (mix, Isp, chamber K, delta-v from the full tankage on the empty
    /// orbital mass, mission volume) for each mix.
    mixes: Vec<(f64, f64, f64, f64, f64)>,
    best: (f64, Closure),
}

fn fill(v: &Vehicle, d: Diluent) -> Result<Fill> {
    let engine = EngineSpec {
        turbopump: TurbopumpSpec { diluent: d, ..TurbopumpSpec::default() },
        ..EngineSpec::default()
    };
    let k = engine.turbopump.per_kg(1.5e6)?;
    let per_mw = (Turbopump::diluent_per_mw_min(&k), Turbopump::dust_per_mw_min(&k));
    let dh = coolant_heat(d, engine.turbopump.p_reverter, 2500.)?;
    let b = blade_heat(&k, 50e6 / k.electric, 4000., dh, false)?;
    let sls = engine.evaluate(&Point::default())?;
    let ramjet = engine.evaluate(&Point {
        altitude: hf_engine::atmos::altitude_for_q(4.5, 50e3),
        mach: 4.5,
        mode: hf_engine::cycle::Mode::Ramjet,
        t4: Some(3000.),
        ..Default::default()
    })?;
    let cr = cruise(&engine, 11000.)?;
    let table = AscentSpec::new(engine.clone(), ROCKET.0, ROCKET.1, ROCKET.2, 0.25)?
        .engine_table();
    let orbit = v.empty + v.tanks_dry + v.reserve;
    let rho = d.density();
    let runs: Vec<_> = MIXES
        .par_iter()
        .filter_map(|&r| {
            let spec =
                AscentSpec::new(engine.clone(), ROCKET.0, ROCKET.1, ROCKET.2, r).ok()?;
            // All the tankage burnt on the rocket at this mix.
            let per_m3 = (1. + r) / volume(1., r, rho);
            let dv = spec.rocket_isp * G0 * (1. + per_m3 * v.tankage / orbit).ln();
            let c = close(v, &spec, &table, cr.0)
                .map_err(|e| eprintln!("{d} 1:{r}: {e}"))
                .ok()?;
            Some((r, spec.rocket_isp, spec.chamber_t, dv, c))
        })
        .collect();
    let mixes =
        runs.iter().map(|(r, isp, t, dv, c)| (*r, *isp, *t, *dv, c.volume)).collect();
    let best = runs
        .into_iter()
        .min_by(|a, b| a.4.volume.total_cmp(&b.4.volume))
        .map(|x| (x.0, x.4))
        .ok_or_else(|| anyhow::anyhow!("no mix closes for {d}"))?;
    Ok(Fill {
        diluent: d,
        per_mw,
        blades: b.heat / b.capacity,
        sls: (sls.thrust, sls.isp),
        cruise: cr,
        ramjet: ramjet.isp,
        mixes,
        best,
    })
}

fn report_fills(v: &Vehicle, fills: &[Fill]) {
    println!("== What fills {:.1} m3 of tank best: dust shot packed to 80%, a liquid in its gaps and beside it", v.tankage);
    for f in fills {
        let d = f.diluent;
        println!(
            "  {d}: {:.0} kg/m3, freezes at {:.0} K; gaps filled, {:.0} kg/m3 of tank",
            d.density(),
            d.freezing(),
            filled_density(d.density())
        );
        println!(
            "    turbopump at 5,000 K: {:.2} kg liquid and {:.2} kg dust per MW-minute; blade cooling takes {:.0}% of the liquid",
            f.per_mw.0,
            f.per_mw.1,
            100. * f.blades
        );
        println!(
            "    sea level dry {:.0} kN at {:.0} s; cruise {:.0} km/t, {:.0} km/m3, {:.2} kg liquid per kg dust; ramjet Mach 4.5 {:.0} s",
            f.sls.0 / 1e3,
            f.sls.1,
            f.cruise.1,
            f.cruise.2,
            f.cruise.0,
            f.ramjet
        );
        for (r, isp, t, dv, vol) in &f.mixes {
            println!(
                "    rocket 1:{r:<4}: {isp:4.0} s, chamber {t:4.0} K; all {:.1} m3 on the rocket: {:.2} km/s; down and back takes {vol:4.1} m3",
                v.tankage,
                dv / 1e3
            );
        }
        let (r, c) = &f.best;
        println!(
            "    best: 1:{r} — takeoff {:.0} kg, {:.0} kg dust and {:.0} kg liquid, {:.1} m3 of {:.1} ({:.1} spare)",
            c.takeoff,
            c.dust,
            c.liquid,
            c.volume,
            v.tankage,
            v.tankage - c.volume
        );
    }
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

fn mission(v: &Vehicle, f: &Fill) -> Result<()> {
    let planet = &EARTH;
    let aero = Aero::default();
    let d = f.diluent;
    let (mix, c) = &f.best;
    let engine = EngineSpec {
        turbopump: TurbopumpSpec { diluent: d, ..TurbopumpSpec::default() },
        ..EngineSpec::default()
    };
    let spec = AscentSpec::new(engine, ROCKET.0, ROCKET.1, ROCKET.2, *mix)?;
    let table = spec.engine_table();
    let strip = StripSpec::default();
    let curve = StripSpec::engine_curve(&spec.engine);

    println!(
        "== Down and back on {d}, rocket at 1:{mix}, Earth-like, clean but for the laser"
    );
    println!(
        "  rocket mode: {:.0} kN vacuum at {:.0} MPa, area ratio {:.1} in a {:.2} m2 exit, {:.0} s, chamber {:.0} K",
        spec.rocket_thrust / 1e3,
        ROCKET.1 / 1e6,
        spec.area_ratio,
        spec.exit_area,
        spec.rocket_isp,
        spec.chamber_t
    );
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
        "  leaving orbit: {:.0} kg of dust and {:.0} kg of {d}, {:.1} m3 of {:.1} ({:.1} spare)",
        c.dust,
        c.liquid,
        c.volume,
        v.tankage,
        v.tankage - c.volume
    );

    let a = &c.ascent;
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
        "  air-breathing to Mach {:.2} at {:.0} km: {:.0} s, {:.0} kg ({:.0} {d}, {:.0} dust)",
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
        "== The strip: 300 m of hard dirt (rolling {}, braking {}), lift-fan motor {:.0} MW",
        strip.rolling,
        strip.braking,
        strip.fan_max / 1e6
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
    println!("  heaviest takeoff in 300 m: {:.0} kg", strip.heaviest(300., &curve)?);
    let land = strip.land(c.takeoff, &curve);
    println!(
        "  landing at {:.0} kg: touchdown {:.0} m/s, roll {:.0} m",
        c.takeoff, land.touchdown, land.roll
    );
    for (name, s) in [
        (
            "35 MW turbopump, 20 MW lift fan",
            StripSpec { turbopump: 35e6, fan_max: 20e6, ..StripSpec::default() },
        ),
        (
            "soft dirt, rolling 0.08, braking 0.2",
            StripSpec { rolling: 0.08, braking: 0.2, ..StripSpec::default() },
        ),
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
    let es = EntrySpec { mass: c.entry_mass, ..v.entry() };
    let e = es.fly(planet, &aero);
    report_entry(&e, &es);
    println!("  belly tanks behind the 6 m station's skin, held at 300 K by their boiling liquid:");
    for t in [0.03, 0.05, 0.10] {
        let ins = Insulation { thickness: t, ..Insulation::default() };
        let s = tank_soak(&e, &ins, 300., 8000., 0.85);
        println!(
            "    {:2.0} cm of insulation ({:.0} kg/m2): {:5.1} MJ/m2 into the tank, peak {:5.0} W/m2 at t {:4.0} s; {:4.0} kg of {d} boiled off under {} m2",
            t * 100.,
            t * ins.density,
            s.heat / 1e6,
            s.peak_flux,
            s.peak_time,
            s.heat * v.belly_tanks / d.latent(),
            v.belly_tanks
        );
    }
    for (name, s) in [
        (
            "nose 3 cm, edge 1 cm",
            EntrySpec {
                mass: c.entry_mass,
                nose_radius: 0.03,
                edge_radius: 0.01,
                ..v.entry()
            },
        ),
        (
            "angle of attack 30 deg",
            EntrySpec {
                mass: c.entry_mass,
                alpha_high: 30f64.to_radians(),
                ..v.entry()
            },
        ),
        (
            "early transition, Re 5e5",
            EntrySpec { mass: c.entry_mass, transition: 5e5, ..v.entry() },
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
        ("empty +1 t", Vehicle { empty: v.empty + 1000., ..*v }),
        ("empty +2 t", Vehicle { empty: v.empty + 2000., ..*v }),
        ("sortie 1.5 t", Vehicle { sortie: 1500., ..*v }),
    ] {
        let c = close(&vv, &spec, &table, f.cruise.0)?;
        println!(
            "    {name:14}: takeoff {:5.0} kg, entry {:5.0} kg, {:4.1} m3 of {:.1}, 300 m takeoff {}",
            c.takeoff,
            c.entry_mass,
            c.volume,
            vv.tankage,
            if strip.takeoff(c.takeoff, &curve).roll <= 300. { "yes" } else { "no" }
        );
    }
    Ok(())
}

/// The heaviest empty aircraft the down-and-back mission tolerates on
/// ammonia: before the propellant outgrows the tankage, and before the
/// takeoff outgrows 300 m of dirt.
fn limits(v: &Vehicle) -> Result<()> {
    let engine = EngineSpec::default();
    let cr = cruise(&engine, 11000.)?;
    let spec = AscentSpec::new(engine, ROCKET.0, ROCKET.1, ROCKET.2, 0.1)?;
    let table = spec.engine_table();
    let strip = StripSpec::default();
    let curve = StripSpec::engine_curve(&spec.engine);
    let heaviest = strip.heaviest(300., &curve)?;
    let at = |empty: f64| close(&Vehicle { empty, ..*v }, &spec, &table, cr.0);
    let by_volume =
        hf_engine::gas::brent(|e| Ok(at(e)?.volume - v.tankage), 6000., 12000., 1e-5)?;
    let by_strip =
        hf_engine::gas::brent(|e| Ok(at(e)?.takeoff - heaviest), 6000., 12000., 1e-5)?;
    println!(
        "== Empty mass the mission tolerates, with {:.0} kg of tank structure and {:.0} kg kept in orbit",
        v.tanks_dry, v.reserve
    );
    println!("  propellant fills the {:.1} m3 at {by_volume:.0} kg empty", v.tankage);
    println!("  takeoff reaches the {heaviest:.0} kg the strip allows at {by_strip:.0} kg empty");
    Ok(())
}

/// The mission for an airframe of this wing area, empty mass (with the
/// pilot and internal tank walls), conformal-tank structure and tankage,
/// and turbopump and lift-fan ratings: `--size WING EMPTY TANKS_DRY
/// TANKAGE [TURBOPUMP_MW FAN_MW DRAG_AREA]`; with `--worlds`, its sortie
/// budget on each world instead.
fn size(v: &Vehicle, turbopump: f64, fan: f64) -> Result<()> {
    let engine = EngineSpec::default();
    let cr = cruise(&engine, 11000.)?;
    let mut spec = AscentSpec::new(engine, ROCKET.0, ROCKET.1, ROCKET.2, 0.1)?;
    spec.electric = turbopump;
    let table = spec.engine_table();
    let strip =
        StripSpec { wing: v.wing, turbopump, fan_max: fan, ..StripSpec::default() };
    let curve = StripSpec::engine_curve(&spec.engine);
    let c = close(v, &spec, &table, cr.0)?;
    let heaviest = strip.heaviest(300., &curve)?;
    let to = strip.takeoff(c.takeoff, &curve);
    let land = strip.land(c.takeoff, &curve);
    let e = EntrySpec { mass: c.entry_mass, ..v.entry() }
        .fly(&EARTH, &v.aero());
    println!(
        "{:.0}/{:.0} MW, wing {:.1} m2, empty {:.0} kg + {:.0} kg tanks: takeoff {:.0} kg (strip allows {heaviest:.0}), roll {:.0} m, landing roll {:.0} m; propellant {:.1} of {:.1} m3; entry {:.0} kg, nose {:.0} K",
        turbopump / 1e6,
        fan / 1e6,
        v.wing,
        v.empty,
        v.tanks_dry,
        c.takeoff,
        to.roll,
        land.roll,
        c.volume,
        v.tankage,
        c.entry_mass,
        e.nose.temperature
    );
    Ok(())
}

/// How hard the fighter turns at `mass` kg: instantaneous, on its
/// trimmed maximum lift or the structure's 20 g at 15 t, whichever binds;
/// and sustained, where full thrust meets the trimmed drag; in g and
/// degrees a second. Propellant moved between tanks puts the centre of
/// gravity where each turn wants it, within TRIM_RANGE of where the tanks
/// leave it.
fn turn(v: &Vehicle) -> Result<()> {
    const N_STRUCT: (f64, f64) = (20., 15000.);
    const TRIM_RANGE: f64 = 0.6;
    let spec = AscentSpec::new(EngineSpec::default(), ROCKET.0, ROCKET.1, ROCKET.2, 0.1)?;
    println!(
        "== Turning, {}, wing {:.0} m2 at {:.0}°: instantaneous and sustained, g and deg/s",
        if v.canard.is_some() { "canards" } else { "tailless" },
        v.wing,
        v.sweep
    );
    for mass in [11000., 16000., 22000.] {
        let aero = Aero { cg: v.cg_at(mass), ..v.aero() };
        println!("  {:.0} t, {:.0} kg/m2 of wing, centre of gravity {:.2} m", mass / 1e3, mass / v.wing, aero.cg);
        for (mach, z) in [(0.5, 3000.), (0.9, 3000.), (0.9, 10000.), (1.5, 11000.), (2.0, 15000.)] {
            let at = air(z);
            let speed = mach * at.sound;
            let q = 0.5 * at.rho * speed * speed;
            let weight = mass * 9.80665;
            let thrust = spec
                .settings(mach, z)
                .iter()
                .map(|(_, r)| r.thrust)
                .fold(0., f64::max);
            let rate = |n: f64| (9.80665 * (n * n - 1.).max(0.).sqrt() / speed).to_degrees();
            let at_cg = |cg: f64| {
                let a = Aero { cg, ..aero };
                let cl_max = a.trimmed_cl_max(mach);
                let inst = (q * v.wing * cl_max / weight).min(N_STRUCT.0 * N_STRUCT.1 / mass);
                let (mut lo, mut hi) = (0., cl_max);
                for _ in 0..60 {
                    let cl = (lo + hi) / 2.;
                    if a.drag(mach, q, q * v.wing * cl) < thrust {
                        lo = cl;
                    } else {
                        hi = cl;
                    }
                }
                (inst, (q * v.wing * lo / weight).min(inst), cl_max)
            };
            let cgs: Vec<f64> =
                (0..=24).map(|i| aero.cg - TRIM_RANGE + 2. * TRIM_RANGE * i as f64 / 24.).collect();
            let best = |k: usize| {
                cgs.iter()
                    .map(|&cg| (at_cg(cg), cg))
                    .max_by(|a, b| {
                        let f = |r: &((f64, f64, f64), f64)| [r.0.0, r.0.1][k];
                        f(a).total_cmp(&f(b))
                    })
                    .unwrap()
            };
            let ((inst, _, cl_max), cg_i) = best(0);
            let ((_, sus, _), cg_s) = best(1);
            println!(
                "    Mach {mach:.1} at {z:5.0} m: {inst:4.1} g {:4.1}/s (CLmax {cl_max:.2}, cg {cg_i:.2}), sustained {sus:4.1} g {:4.1}/s (cg {cg_s:.2})",
                rate(inst),
                rate(sus)
            );
        }
    }
    Ok(())
}

fn main() -> Result<()> {
    let mut v = Vehicle::default();
    let args: Vec<String> = std::env::args().collect();
    if args.iter().any(|a| a == "--tailless") {
        v.canard = None;
    }
    if let Some(i) = args.iter().position(|a| a == "--canard") {
        let n = |k: usize| -> Result<f64> {
            Ok(args
                .get(i + k)
                .ok_or_else(|| anyhow::anyhow!("--canard AREA ASPECT X_AC"))?
                .parse()?)
        };
        v.canard =
            Some(Canard { area: n(1)?, aspect: n(2)?, x_ac: n(3)?, ..Canard::default() });
    }
    if let Some(i) = args.iter().position(|a| a == "--cg") {
        let n = |k: usize| -> Result<f64> {
            Ok(args.get(i + k).ok_or_else(|| anyhow::anyhow!("--cg EMPTY FULL"))?.parse()?)
        };
        v.cg = (n(1)?, n(2)?);
    }
    if let Some(i) = args.iter().position(|a| a == "--planform") {
        let n = |k: usize| -> Result<f64> {
            Ok(args
                .get(i + k)
                .ok_or_else(|| anyhow::anyhow!("--planform SWEEP ASPECT"))?
                .parse()?)
        };
        v.sweep = n(1)?;
        v.aspect = n(2)?;
    }
    if let Some(i) = args.iter().position(|a| a == "--wave") {
        v.wave = args.get(i + 1).ok_or_else(|| anyhow::anyhow!("--wave D/Q"))?.parse()?;
    }
    if let Some(i) = args.iter().position(|a| a == "--size") {
        let n = |k: usize| -> Result<f64> {
            Ok(args
                .get(i + k)
                .ok_or_else(|| anyhow::anyhow!("--size WING EMPTY TANKS_DRY TANKAGE"))?
                .parse()?)
        };
        v = Vehicle {
            wing: n(1)?,
            empty: n(2)?,
            tanks_dry: n(3)?,
            tankage: n(4)?,
            drag_area: n(7).unwrap_or(v.drag_area),
            ..v
        };
        let (tp, fan) = (n(5).unwrap_or(50.) * 1e6, n(6).unwrap_or(30.) * 1e6);
        if !args.iter().any(|a| a == "--worlds" || a == "--turn") {
            return size(&v, tp, fan);
        }
    }
    if args.iter().any(|a| a == "--worlds") {
        return worlds(&v);
    }
    if args.iter().any(|a| a == "--turn") {
        return turn(&v);
    }
    if args.iter().any(|a| a == "--limits") {
        return limits(&v);
    }
    let fills: Vec<Fill> = LIQUIDS.iter().map(|&d| fill(&v, d)).collect::<Result<_>>()?;
    report_fills(&v, &fills);
    let best =
        fills.iter().min_by(|a, b| a.best.1.volume.total_cmp(&b.best.1.volume)).unwrap();
    mission(&v, best)?;
    if best.diluent != Diluent::Ammonia
        && let Some(nh3) = fills.iter().find(|f| f.diluent == Diluent::Ammonia)
    {
        mission(&v, nh3)?;
    }
    Ok(())
}
