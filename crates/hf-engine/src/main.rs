//! Evaluate the fighter's engine against the numbers in book/small-craft.md
//! and print the comparison.
//!
//! `hf-engine [-|FILE]`: with no path the default engine spec, `-` a spec
//! on stdin, else one from FILE. Missing fields take defaults.

use anyhow::{Context, Result};
use hf_engine::{
    atmos::{altitude_for_q, isa, Air},
    blade::blade_heat,
    cycle::{rocket, EngineSpec, Mode, Point, PointReport},
    gas::{eq_hp, Feed},
    hover::{calibrated_efficiency, lift_fan, Airframe},
    parts::{Turbopump, TurbopumpSpec, G0},
    thermo::sp,
};
use rayon::prelude::*;
use std::{fs::File, io};

/// The turbopump's rating, W.
const TURBOPUMP_MAX: f64 = 35e6;

fn read_spec() -> Result<EngineSpec> {
    Ok(match std::env::args().skip(1).find(|a| !a.starts_with("--")).as_deref() {
        None => EngineSpec::default(),
        Some("-") => serde_json::from_reader(io::stdin()).context("spec on stdin")?,
        Some(path) => serde_json::from_reader(File::open(path).context(path.to_owned())?)
            .with_context(|| format!("spec in {path}"))?,
    })
}

fn row(name: &str, r: &PointReport) {
    println!(
        "  {name:30} {:6.1} kN {:5.1} MW  air {:5.1}  NH3 {:6.3} dust {:6.3} kg/s  Isp {:6.0} s  dust {:6.0} s  T3 {:4.0} T4 {:4.0} T7 {:4.0} K  jet {:4.0} m/s  tp {:3.0}%",
        r.thrust / 1e3,
        r.electric / 1e6,
        r.airflow,
        r.nh3,
        r.dust,
        r.isp,
        r.isp_dust,
        r.t3,
        r.t4,
        r.t7,
        r.v_jet,
        100. * r.turbopump_share,
    );
}

fn chemistry() -> Result<()> {
    println!("== Reverter and rocket chemistry (equilibrium, with dissociation)");
    for (x, p) in
        [(0., 10e6), (1., 10e6), (2., 10e6), (3.28, 50e6), (5.6, 50e6), (12., 50e6)]
    {
        let g =
            eq_hp(&Feed::blend(&[(1., Feed::dust()), (x, Feed::ammonia(0.))]), p, None)?;
        println!(
            "  dust:ammonia 1:{x:<5} at {:3.0} MPa: {:5.0} K, {:4.1}% H atoms, {:.2} g/mol",
            p / 1e6,
            g.t,
            100. * g.x(sp::H),
            g.mw() * 1e3
        );
    }
    for tit in [3000., 5000.] {
        for p in [10e6, 50e6] {
            let (x, g) =
                TurbopumpSpec { tit, p_reverter: p, ..Default::default() }.reverter()?;
            println!(
                "  {tit:.0} K at {:.0} MPa takes 1:{x:.2}: {:.0}% H atoms, {:.0}% H2, {:.0}% N2",
                p / 1e6,
                100. * g.x(sp::H),
                100. * g.x(sp::H2),
                100. * g.x(sp::N2)
            );
        }
    }
    println!("  rocket, vacuum Isp, shifting equilibrium, ideal nozzle:");
    for (x, pc, e) in [
        (0., 10e6, 100.),
        (1., 10e6, 100.),
        (2., 10e6, 100.),
        (2., 2e6, 5.),
        (2., 2e6, 18.),
        (1., 2e6, 18.),
    ] {
        let r = rocket(x, pc, e)?;
        println!(
            "    1:{x} at {:2.0} MPa, area ratio {e:3}: {:4.0} K chamber, {:4.0} s",
            pc / 1e6,
            r.chamber_t,
            r.isp_vacuum
        );
    }
    Ok(())
}

fn turbopumps() -> Result<()> {
    println!("== Turbopump, 10 MPa reverter to 1.5 MPa, per MW-minute of electricity");
    for tit in [3000., 4000., 5000.] {
        for (stages, reheat, carrier) in
            [(6, false, 0.56), (6, true, 0.56), (6, true, 0.)]
        {
            let s = TurbopumpSpec { tit, stages, reheat, carrier, ..Default::default() };
            let s = TurbopumpSpec { p_reverter: 10e6, ..s };
            let k = s.per_kg(1.5e6)?;
            let label = match (reheat, carrier) {
                (false, _) => "no reheat",
                (true, 0.) => "reheat, dry dust",
                _ => "reheat, slurry",
            };
            println!(
                "  {tit:.0} K, {label:17}: {:4.2} kg ammonia, {:4.2} kg dust, exhaust {:4.0} K",
                Turbopump::nh3_per_mw_min(&k),
                k.dust / k.electric * 6e7,
                k.exhaust.gas.t
            );
        }
    }
    Ok(())
}

fn engine(name: &str, spec: &EngineSpec) -> Result<()> {
    println!("== Engine, {name}");
    let sls = Point::default();
    let dry = spec.evaluate(&sls)?;
    row("sea level, max dry", &dry);
    row(
        "sea level, separate nozzles",
        &spec.evaluate(&Point { mixed: false, ..sls.clone() })?,
    );
    for t in [2000., 2400.] {
        row(
            &format!("sea level, main burner {t:.0} K"),
            &spec.evaluate(&Point { t4: Some(t), ..sls.clone() })?,
        );
    }
    row(
        "sea level, reheat 2400 K",
        &spec.evaluate(&Point { t4: Some(2400.), t7: Some(2400.), ..sls.clone() })?,
    );
    let n2 = Point { air: Air::Nitrogen, ..sls.clone() };
    let n2_dry = spec.evaluate(&n2)?;
    row("nitrogen air, max dry", &n2_dry);
    row(
        "nitrogen air, main burner 2400 K",
        &spec.evaluate(&Point { t4: Some(2400.), ..n2.clone() })?,
    );
    row(
        "earth air, main burner 2400 K",
        &spec.evaluate(&Point { t4: Some(2400.), ..sls.clone() })?,
    );
    let af = Airframe::default();
    let m2 = Point { altitude: 11000., mach: 2., ..Default::default() };
    row("Mach 2, 11 km, max dry", &spec.evaluate(&m2)?);
    let capped = spec.within_power(&m2, TURBOPUMP_MAX - 8e6)?;
    row("Mach 2, 11 km, dry, FEL on, 35 MW", &spec.evaluate(&capped)?);
    let m2r = Point { t4: Some(2400.), t7: Some(2400.), ..m2 };
    row("Mach 2, 11 km, reheat 2400 K", &spec.evaluate(&m2r)?);
    row(
        "Mach 2, reheat, 35 MW",
        &spec.evaluate(&spec.within_power(&m2r, TURBOPUMP_MAX)?)?,
    );
    println!(
        "  drag at Mach 2, 11 km, 11 t: {:.0} kN",
        af.drag(11000., 11000., 2.) / 1e3
    );
    println!(
        "  high Mach at q = 50 kPa (inlet capture {} m2), drag at 10 t:",
        spec.capture
    );
    for m in [2.5, 3.5] {
        let z = altitude_for_q(m, 50e3);
        // The compressor is unloaded to keep its delivery below ~850 K.
        let pt = Point {
            altitude: z,
            mach: m,
            fan_pr: 1.6,
            compressor_pr: 1.,
            t4: Some(2400.),
            t7: Some(2400.),
            ..Default::default()
        };
        row(
            &format!("turbine, Mach {m}, {:.0} km, fan 1.6", z / 1e3),
            &spec.evaluate(&pt)?,
        );
    }
    for m in [3.5, 4.5, 6.] {
        let z = altitude_for_q(m, 50e3);
        for t4 in [2400., 3000., 3500.] {
            let pt = Point {
                altitude: z,
                mach: m,
                mode: Mode::Ramjet,
                t4: Some(t4),
                ..Default::default()
            };
            row(
                &format!("ramjet, Mach {m}, {:.0} km, {t4:.0} K", z / 1e3),
                &spec.evaluate(&pt)?,
            );
        }
        println!("    drag {:.0} kN", af.drag(10000., z, m) / 1e3);
    }
    Ok(())
}

/// The setting that cruises at `thrust` on the least propellant, over a
/// grid of fan and compressor ratios, burner temperatures and mixing.
fn best_cruise(
    spec: &EngineSpec,
    base: &Point,
    thrust: f64,
) -> Option<(Point, PointReport)> {
    let mut grid = Vec::new();
    for fan in [1.4, 1.7, 2.0, 2.5, 3.0, 3.5] {
        for comp in [1.0, 1.5, 2.0, 2.5, 3.5] {
            for t4 in [None, Some(1200.), Some(1500.), Some(1800.)] {
                for mixed in [true, false] {
                    grid.push(Point {
                        fan_pr: fan,
                        compressor_pr: comp,
                        t4,
                        mixed,
                        ..base.clone()
                    });
                }
            }
        }
    }
    grid.into_par_iter()
        .filter_map(|pt| {
            let k = spec.throttle_for(&pt, thrust).ok()?;
            let pt = Point { throttle: k, ..pt };
            let r = spec.evaluate(&pt).ok()?;
            Some((pt, r))
        })
        .max_by(|a, b| a.1.isp.total_cmp(&b.1.isp))
}

fn mission(spec: &EngineSpec) -> Result<()> {
    println!("== Cruise, Mach 0.9 at 11 km, 11 t, Mirage-like drag");
    let t = isa(11000.).0;
    let v = 0.9 * (1.4 * 287.053 * t).sqrt();
    let drag = Airframe::default().drag(11000., 11000., 0.9);
    let base = Point { altitude: 11000., mach: 0.9, ..Default::default() };
    match best_cruise(spec, &base, drag) {
        Some((pt, r)) => {
            row(
                &format!(
                    "fan {} comp {} t4 {:?} mixed {}",
                    pt.fan_pr, pt.compressor_pr, pt.t4, pt.mixed
                ),
                &r,
            );
            let km_per_t = v / (r.nh3 + r.dust);
            // Mirage 2000 for comparison: M53-P2 at ~0.09 kg/N/h in cruise.
            let mirage = v / (drag * 0.09 / 3600.);
            println!(
                "  drag {:.1} kN; {km_per_t:.0} km per tonne of propellant; a Mirage 2000 at TSFC 0.09 kg/N/h: {mirage:.0} km/t ({:.1}x)",
                drag / 1e3,
                km_per_t / mirage
            );
        }
        None => println!("  no setting cruises"),
    }
    Ok(())
}

fn hover(spec: &EngineSpec) -> Result<()> {
    println!(
        "== Hover, sea level (lift fan calibrated on the F-35B: {:.2} of ducted ideal)",
        calibrated_efficiency()
    );
    let (fan_thrust, rear) = (75e3, 40e3);
    let (shaft, jet) = lift_fan(fan_thrust, 1.5);
    let fan_el = shaft / spec.eta_motor;
    println!(
        "  lift fan, 1.5 m, {:.0} kN: {:.1} MW electric, {jet:.0} m/s downwash",
        fan_thrust / 1e3,
        fan_el / 1e6
    );
    let show = |name: &str, r: &PointReport| {
        row(name, r);
        println!(
            "    hover total {:.1} MW: {:.0} kg ammonia and {:.0} kg dust a minute",
            r.electric / 1e6,
            r.nh3 * 60.,
            r.dust * 60.
        );
    };
    let base = Point { extra_power: fan_el, ..Default::default() };
    let k = spec.throttle_for(&base, rear)?;
    show(
        &format!("rear {:.0} kN, full ratios", rear / 1e3),
        &spec.evaluate(&Point { throttle: k, ..base.clone() })?,
    );
    // The coolest rear jet within the turbopump's rating.
    let mut grid = Vec::new();
    for fan in [1.3, 1.6, 2.0, 2.5, 3.5] {
        for comp in [1.0, 1.5, 2.5, 3.5] {
            for mixed in [true, false] {
                grid.push(Point {
                    fan_pr: fan,
                    compressor_pr: comp,
                    mixed,
                    ..base.clone()
                });
            }
        }
    }
    // A fixed-pitch fan slowed to a lower ratio passes less air: on a
    // speed line, roughly flow ~ speed and (ratio - 1) ~ speed^2. A
    // variable-pitch fan is free of that.
    for pitch in ["fixed", "variable"] {
        let coolest = grid
            .par_iter()
            .filter_map(|pt| {
                let k = spec.throttle_for(pt, rear).ok()?;
                let map = ((pt.fan_pr - 1.) / (spec.fan_pr - 1.)).sqrt();
                if pitch == "fixed" && k > map {
                    return None;
                }
                let pt = Point { throttle: k, ..pt.clone() };
                let r = spec.evaluate(&pt).ok()?;
                (r.electric <= TURBOPUMP_MAX).then_some((pt, r))
            })
            .min_by(|a, b| a.1.t7.total_cmp(&b.1.t7));
        if let Some((pt, r)) = coolest {
            show(
                &format!(
                    "coolest, {pitch} pitch: fan {} comp {} mix {}",
                    pt.fan_pr, pt.compressor_pr, pt.mixed
                ),
                &r,
            );
        }
    }
    Ok(())
}

fn loads(spec: &EngineSpec) -> Result<()> {
    println!("== Electric loads (turbopump into a 1.5 MPa combustor)");
    let k = spec.turbopump.per_kg(1.5e6)?;
    for (name, mw, minutes) in [
        ("FEL, 10 min", 8., 10.),
        ("light shuttle hover, 1 min", 10., 1.),
        ("light shuttle hover, 1 min", 16., 1.),
    ] {
        let tp = Turbopump::scale(k, mw * 1e6);
        println!(
            "  {name} at {mw} MW: {:.0} kg ammonia, {:.0} kg dust",
            tp.nh3 * 60. * minutes,
            tp.dust * 60. * minutes
        );
    }
    let sls = spec.evaluate(&Point::default())?;
    let generator = sls.electric + 8e6;
    println!(
        "  machines at 100 kW/kg: generator {:.0} kg (max dry + FEL, {:.1} MW), engine motors {:.0} kg",
        generator / 1e5,
        generator / 1e6,
        (sls.electric - spec.systems) * spec.eta_motor / 1e5
    );
    let r = rocket(2., 2e6, 18.)?;
    println!(
        "  fighter rocket mode, 3 t of 1:2 from 12 t at {:.0} s: {:.1} km/s",
        r.isp_vacuum,
        r.isp_vacuum * G0 * (12f64 / 9.).ln() / 1e3
    );
    Ok(())
}

fn blades(spec: &EngineSpec) -> Result<()> {
    println!("== Turbopump blades at {:.0} MW into 1.5 MPa: non-catalytic surfaces at 4,000 K, ammonia out at 2,500 K", TURBOPUMP_MAX / 1e6);
    let k = spec.turbopump.per_kg(1.5e6)?;
    let m_gas = TURBOPUMP_MAX / k.electric;
    let b = blade_heat(&k, m_gas, 4000., 2500., false)?;
    println!("  mean radius {:.0} mm, {:.0} rpm", b.mean_radius * 1e3, b.rpm);
    for s in &b.stages[..b.n_stages] {
        println!(
            "    {:4.0} K {:5.1} MPa: blades {:5.1} mm, {:6.1} MW/m2, {:5.2} MW",
            s.t,
            s.p / 1e6,
            s.blade * 1e3,
            s.flux / 1e6,
            s.heat / 1e6
        );
    }
    println!(
        "  heat {:.1} MW against {:.1} MW the ammonia can take ({:.0}%); shaft work {:.1} MW",
        b.heat / 1e6,
        b.capacity / 1e6,
        100. * b.heat / b.capacity,
        b.work / 1e6
    );
    let first = b.stages[0].flux;
    println!("  first stage: a 1,000 K drop through a k = 1 W/m K skin needs it {:.0} um thick", 1000. / first * 1e6);
    Ok(())
}

/// Where cooling can hold the gas above the blade surface: the first
/// stage's flux and the whole turbine's heat against the ammonia's
/// capacity, without reheat, at 35 MW into 1.5 MPa.
fn blade_envelope(spec: &EngineSpec) -> Result<()> {
    println!("== Blade envelope, no reheat, 35 MW: first-stage flux, turbine heat / ammonia capacity");
    for p_rev in [50e6, 20e6, 10e6] {
        for (tit, wall) in [
            (5000., 4000.),
            (4500., 4000.),
            (4200., 4000.),
            (4100., 4000.),
            (4000., 3800.),
        ] {
            for catalytic in [true, false] {
                let tp = TurbopumpSpec {
                    tit,
                    p_reverter: p_rev,
                    reheat: false,
                    ..spec.turbopump.clone()
                };
                let k = tp.per_kg(1.5e6)?;
                let b =
                    blade_heat(&k, TURBOPUMP_MAX / k.electric, wall, 2500., catalytic)?;
                println!(
                    "  {:2.0} MPa, gas {tit:.0} K over {wall:.0} K, {:13}: {:6.0} MW/m2, {:3.0}%, {:.2} kg ammonia/MW-min, {:4.0} krpm, first blades {:.1} mm",
                    p_rev / 1e6,
                    if catalytic { "catalytic" } else { "non-catalytic" },
                    b.stages[0].flux / 1e6,
                    100. * b.heat / b.capacity,
                    Turbopump::nh3_per_mw_min(&k),
                    b.rpm / 1e3,
                    b.stages[0].blade * 1e3
                );
            }
        }
    }
    Ok(())
}

fn main() -> Result<()> {
    let spec = read_spec()?;
    if std::env::args().any(|a| a == "--blades") {
        return blade_envelope(&spec);
    }
    chemistry()?;
    turbopumps()?;
    engine("baseline", &spec)?;
    mission(&spec)?;
    hover(&spec)?;
    loads(&spec)?;
    blades(&spec)?;
    Ok(())
}
