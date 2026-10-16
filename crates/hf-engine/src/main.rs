//! Evaluate the fighter's engine against the numbers in book/small-craft.md,
//! with ammonia and with methane as the diluent, and print the comparison.
//!
//! `hf-engine [-|FILE] [--blades] [--flowpath [--fan-rotors N] [--counter]
//! [--fan-tip U] [--hub-tip H] [--json OUT]]`: with no path the default engine spec, `-`
//! a spec on stdin, else one from FILE. Missing fields take defaults.
//! `--blades` sweeps the turbopump blades' heat load instead; `--flowpath`
//! designs the air path, and writes it as JSON to OUT for
//! modeling/engine.py.

use anyhow::{Context, Result};
use hf_engine::{
    atmos::{altitude_for_q, isa, Air},
    blade::{blade_heat, coolant_heat, BladeHeat},
    cycle::{rocket, EngineSpec, Mode, Point, PointReport},
    flowpath::{FlowpathSpec, MACHINE_DENSITY, MOTOR_SPECIFIC},
    gas::{eq_hp, Diluent, Feed, Gas},
    hover::{calibrated_efficiency, lift_fan, Airframe},
    parts::{Turbopump, TurbopumpSpec, G0},
    thermo::{sp, NS, THERMO},
};
use rayon::prelude::*;
use std::{f64::consts::PI, fs::File, io};

/// The turbopump's rating, W.
const TURBOPUMP_MAX: f64 = 35e6;
/// Packed dust, kg/m3.
const DUST_DENSITY: f64 = 600.;
/// Above this, methane in a cooling passage cracks and lays down coke.
const COKING_T: f64 = 900.;

/// Flags that take a value.
const VALUED: [&str; 4] = ["--fan-rotors", "--fan-tip", "--hub-tip", "--json"];

fn flag(name: &str) -> Option<String> {
    let args: Vec<String> = std::env::args().collect();
    args.iter().position(|a| a == name).and_then(|i| args.get(i + 1).cloned())
}

fn read_spec() -> Result<EngineSpec> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let path = args
        .iter()
        .enumerate()
        .find(|(i, a)| !a.starts_with("--") && (*i == 0 || !VALUED.contains(&args[i - 1].as_str())))
        .map(|(_, a)| a.as_str());
    Ok(match path {
        None => EngineSpec::default(),
        Some("-") => serde_json::from_reader(io::stdin()).context("spec on stdin")?,
        Some(path) => serde_json::from_reader(File::open(path).context(path.to_owned())?)
            .with_context(|| format!("spec in {path}"))?,
    })
}

fn with_diluent(spec: &EngineSpec, diluent: Diluent) -> EngineSpec {
    EngineSpec {
        turbopump: TurbopumpSpec { diluent, ..spec.turbopump.clone() },
        ..spec.clone()
    }
}

fn row(name: &str, r: &PointReport) {
    println!(
        "  {name:34} {:6.1} kN {:5.1} MW  air {:5.1}  diluent {:6.3} dust {:6.3} kg/s  Isp {:6.0} s  dust {:6.0} s  T3 {:4.0} T4 {:4.0} T7 {:4.0} K  jet {:4.0} m/s  soot {:4.1}%",
        r.thrust / 1e3,
        r.electric / 1e6,
        r.airflow,
        r.diluent,
        r.dust,
        r.isp,
        r.isp_dust,
        r.t3,
        r.t4,
        r.t7,
        r.v_jet,
        100. * r.soot,
    );
}

/// J/kg a diluent absorbs from its tank to `t` at `p`: as a frozen gas, or
/// cracked to equilibrium.
fn coolant_dh(d: Diluent, p: f64, t: f64, cracked: bool) -> Result<f64> {
    if cracked {
        return coolant_heat(d, p, t);
    }
    let liquid = d.feed(0.);
    let hot = {
        let j = match d {
            Diluent::Ammonia => sp::NH3,
            Diluent::Methane => sp::CH4,
            Diluent::Aqueous(_) => sp::H2O,
        };
        let mut n = [0.; NS];
        n[j] = 1. / THERMO.gas[j].mw;
        Gas { n, t, p, ..Default::default() }
    };
    Ok(hot.h() - liquid.h)
}

/// What the closing comparison prints for each diluent.
#[derive(Default)]
struct Summary {
    reverter_mix: f64,
    reverter_soot: f64,
    per_mw: (f64, f64),
    exhaust_soot: f64,
    sls: (f64, f64, f64),
    n2_isp_ratio: f64,
    m2_thrust: f64,
    cruise: (f64, f64, f64),
    hover: (f64, f64, f64),
    fel: (f64, f64),
    rocket: (f64, f64, f64),
    blades: (f64, f64, f64),
}

fn chemistry(d: Diluent, sum: &mut Summary) -> Result<()> {
    let name = d;
    println!("== Dust and {name}: equilibrium, with dissociation and soot");
    for (x, p) in [(1., 10e6), (2., 10e6)] {
        let g = eq_hp(&Feed::blend(&[(1., Feed::dust()), (x, d.feed(0.))]), p, None)?;
        println!(
            "  dust:{name} 1:{x} at {:.0} MPa: {:5.0} K, {:4.1}% H atoms, {:.2} g/mol, {:.1}% soot",
            p / 1e6,
            g.t,
            100. * g.x(sp::H),
            g.mw() * 1e3,
            100. * g.soot()
        );
    }
    for tit in [3000., 5000.] {
        let (x, g) =
            TurbopumpSpec { tit, diluent: d, ..Default::default() }.reverter()?;
        println!(
            "  {tit:.0} K at 10 MPa takes 1:{x:.2}: {:.0}% H atoms, {:.0}% H2, {:.1}% soot",
            100. * g.x(sp::H),
            100. * g.x(sp::H2),
            100. * g.soot()
        );
        if tit == 5000. {
            sum.reverter_mix = x;
            sum.reverter_soot = g.soot();
        }
    }
    println!("  rocket, vacuum Isp, shifting equilibrium, ideal nozzle, soot riding with the gas:");
    for (x, pc, e) in [(1., 10e6, 100.), (2., 10e6, 100.), (1., 2e6, 18.), (2., 2e6, 18.)]
    {
        let r = rocket(d, x, pc, e)?;
        println!(
            "    1:{x} at {:2.0} MPa, area ratio {e:3}: {:4.0} K chamber, {:4.0} s, {:4.1}% soot at the exit",
            pc / 1e6,
            r.chamber_t,
            r.isp_vacuum,
            100. * r.soot
        );
        if x == 1. && pc == 10e6 {
            sum.rocket.0 = r.isp_vacuum;
            sum.rocket.2 = r.soot;
        }
        if x == 2. && pc == 2e6 {
            sum.rocket.1 = r.isp_vacuum;
        }
    }
    Ok(())
}

fn turbopumps(d: Diluent, sum: &mut Summary) -> Result<()> {
    println!("== Turbopump on {}, 10 MPa into 1.5 MPa, per MW-minute", d);
    for (tit, reheat) in [(3000., false), (4000., false), (5000., false), (5000., true)] {
        let s = TurbopumpSpec { tit, reheat, diluent: d, ..Default::default() };
        let k = s.per_kg(1.5e6)?;
        println!(
            "  {tit:.0} K{:15}: {:5.2} kg {}, {:4.2} kg dust, exhaust {:4.0} K, {:4.1}% soot",
            if reheat { ", stage reheat" } else { "" },
            Turbopump::diluent_per_mw_min(&k),
            d,
            Turbopump::dust_per_mw_min(&k),
            k.exhaust.gas.t,
            100. * k.exhaust.gas.soot()
        );
        if tit == 5000. && !reheat {
            sum.per_mw =
                (Turbopump::diluent_per_mw_min(&k), Turbopump::dust_per_mw_min(&k));
            sum.exhaust_soot = k.exhaust.gas.soot();
        }
    }
    Ok(())
}

fn engine(spec: &EngineSpec, sum: &mut Summary) -> Result<()> {
    println!("== Engine on {}", spec.turbopump.diluent);
    let sls = Point::default();
    let dry = spec.evaluate(&sls)?;
    row("sea level, max dry", &dry);
    let reheat =
        spec.evaluate(&Point { t4: Some(2400.), t7: Some(2400.), ..sls.clone() })?;
    row("sea level, reheat 2400 K", &reheat);
    sum.sls = (dry.thrust, reheat.thrust, dry.isp);
    let n2 = spec.evaluate(&Point { air: Air::Nitrogen, ..sls.clone() })?;
    row("nitrogen air, max dry", &n2);
    row(
        "nitrogen air, main burner 2400 K",
        &spec.evaluate(&Point { air: Air::Nitrogen, t4: Some(2400.), ..sls.clone() })?,
    );
    sum.n2_isp_ratio = n2.isp / dry.isp;
    let af = Airframe::default();
    let m2 = Point { altitude: 11000., mach: 2., ..Default::default() };
    row(
        "Mach 2, 11 km, dry, FEL on, 35 MW",
        &spec.evaluate(&spec.within_power(&m2, TURBOPUMP_MAX - 8e6)?)?,
    );
    let m2r = Point { t4: Some(2400.), t7: Some(2400.), ..m2 };
    let m2r = spec.evaluate(&spec.within_power(&m2r, TURBOPUMP_MAX)?)?;
    row("Mach 2, reheat, 35 MW", &m2r);
    sum.m2_thrust = m2r.thrust;
    println!(
        "    drag at Mach 2, 11 km, 11 t: {:.0} kN",
        af.drag(11000., 11000., 2.) / 1e3
    );
    for m in [3.5, 4.5] {
        let z = altitude_for_q(m, 50e3);
        let pt = Point {
            altitude: z,
            mach: m,
            mode: Mode::Ramjet,
            t4: Some(3000.),
            ..Default::default()
        };
        row(
            &format!("ramjet, Mach {m}, {:.0} km, 3000 K", z / 1e3),
            &spec.evaluate(&pt)?,
        );
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

fn mission(spec: &EngineSpec, sum: &mut Summary) -> Result<()> {
    let d = spec.turbopump.diluent;
    println!("== Cruise on {}, Mach 0.9 at 11 km, 11 t, Mirage-like drag", d);
    let v = 0.9 * (1.4 * 287.053 * isa(11000.).0).sqrt();
    let drag = Airframe::default().drag(11000., 11000., 0.9);
    let base = Point { altitude: 11000., mach: 0.9, ..Default::default() };
    if let Some((pt, r)) = best_cruise(spec, &base, drag) {
        row(
            &format!(
                "fan {} comp {} t4 {:?} mixed {}",
                pt.fan_pr, pt.compressor_pr, pt.t4, pt.mixed
            ),
            &r,
        );
        let km_per_t = v / (r.diluent + r.dust);
        let km_per_m3 = v / 1e3 / (r.diluent / d.density() + r.dust / DUST_DENSITY);
        // Mirage 2000 for comparison: M53-P2 at ~0.09 kg/N/h in cruise, on
        // jet fuel at 800 kg/m3.
        let mirage_flow = drag * 0.09 / 3600.;
        println!(
            "  {km_per_t:.0} km per tonne ({:.1}x a Mirage 2000), {km_per_m3:.0} km per m3 of tank ({:.1}x); {:.1} kg of {} per kg of dust",
            km_per_t / (v / mirage_flow),
            km_per_m3 / (v / 1e3 / (mirage_flow / 800.)),
            r.diluent / r.dust,
            d
        );
        sum.cruise = (km_per_t, km_per_m3, r.diluent / r.dust);
    }
    Ok(())
}

fn hover(spec: &EngineSpec, sum: &mut Summary) -> Result<()> {
    println!(
        "== Hover on {}, sea level (lift fan calibrated on the F-35B: {:.2} of ducted ideal)",
        spec.turbopump.diluent,
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
    let base = Point { extra_power: fan_el, ..Default::default() };
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
            row(
                &format!(
                    "coolest, {pitch} pitch: fan {} comp {}",
                    pt.fan_pr, pt.compressor_pr
                ),
                &r,
            );
            println!(
                "    hover total {:.1} MW: {:.0} kg {} and {:.0} kg dust a minute",
                r.electric / 1e6,
                r.diluent * 60.,
                spec.turbopump.diluent,
                r.dust * 60.
            );
            if pitch == "variable" {
                sum.hover = (r.diluent * 60., r.dust * 60., r.t7);
            }
        }
    }
    Ok(())
}

fn loads(spec: &EngineSpec, sum: &mut Summary) -> Result<()> {
    let d = spec.turbopump.diluent;
    println!("== Electric loads on {} (turbopump into a 1.5 MPa combustor)", d);
    let k = spec.turbopump.per_kg(1.5e6)?;
    for (name, mw, minutes) in [
        ("FEL, 10 min", 8., 10.),
        ("light shuttle hover, 1 min", 10., 1.),
        ("light shuttle hover, 1 min", 16., 1.),
    ] {
        let tp = Turbopump::scale(k, mw * 1e6);
        println!(
            "  {name} at {mw} MW: {:.0} kg {}, {:.0} kg dust",
            tp.diluent * 60. * minutes,
            d,
            tp.dust * 60. * minutes
        );
        if mw == 8. {
            sum.fel = (tp.diluent * 600., tp.dust * 600.);
        }
    }
    let r = rocket(d, 2., 2e6, 18.)?;
    println!(
        "  fighter rocket mode, 3 t of 1:2 from 12 t at {:.0} s: {:.1} km/s",
        r.isp_vacuum,
        r.isp_vacuum * G0 * (12f64 / 9.).ln() / 1e3
    );
    Ok(())
}

/// The turbopump's blades at its rating, into 1.5 MPa, non-catalytic
/// surfaces at 4,000 K cooled by diluent cracked to 2,500 K; its gas flow,
/// kg/s, and the J/kg that coolant takes.
fn turbopump_blades(spec: &EngineSpec) -> Result<(BladeHeat, f64, f64)> {
    let k = spec.turbopump.per_kg(1.5e6)?;
    let m_gas = TURBOPUMP_MAX / k.electric;
    let cracked = coolant_dh(spec.turbopump.diluent, spec.turbopump.p_reverter, 2500., true)?;
    Ok((blade_heat(&k, m_gas, 4000., cracked, false)?, m_gas, cracked))
}

fn blades(spec: &EngineSpec, sum: &mut Summary) -> Result<()> {
    let d = spec.turbopump.diluent;
    let p = spec.turbopump.p_reverter;
    println!(
        "== Turbopump blades on {} at {:.0} MW into 1.5 MPa: non-catalytic surfaces at 4,000 K",
        d,
        TURBOPUMP_MAX / 1e6
    );
    let (b, _, cracked) = turbopump_blades(spec)?;
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
    let frozen = coolant_dh(d, p, COKING_T, false)?;
    let to = |dh: f64| 100. * b.heat / (b.capacity / cracked * dh);
    println!(
        "  heat {:.1} MW; {} cracked to 2,500 K takes {:.1} MJ/kg ({:.0}% of it needed), uncracked to {COKING_T:.0} K takes {:.1} MJ/kg ({:.0}%)",
        b.heat / 1e6,
        d,
        cracked / 1e6,
        to(cracked),
        frozen / 1e6,
        to(frozen)
    );
    sum.blades = (b.stages[0].flux, to(cracked), to(frozen));
    Ok(())
}

/// Where cooling can hold the gas above the blade surface, at 35 MW into
/// 1.5 MPa without reheat.
fn blade_envelope(spec: &EngineSpec) -> Result<()> {
    println!("== Blade envelope, 35 MW: first-stage flux, turbine heat / diluent cracked to 2,500 K");
    for d in Diluent::ALL {
        for p_rev in [50e6, 20e6, 10e6] {
            for (tit, wall) in [(5000., 4000.), (4500., 4000.), (4200., 4000.)] {
                for catalytic in [true, false] {
                    let tp = TurbopumpSpec {
                        tit,
                        p_reverter: p_rev,
                        diluent: d,
                        ..spec.turbopump.clone()
                    };
                    let k = tp.per_kg(1.5e6)?;
                    let dh = coolant_dh(d, p_rev, 2500., true)?;
                    let b =
                        blade_heat(&k, TURBOPUMP_MAX / k.electric, wall, dh, catalytic)?;
                    println!(
                        "  {:8} {:2.0} MPa, gas {tit:.0} K over {wall:.0} K, {:13}: {:6.0} MW/m2, {:3.0}%, {:.2} kg/MW-min, {:4.0} krpm",
                        d,
                        p_rev / 1e6,
                        if catalytic { "catalytic" } else { "non-catalytic" },
                        b.stages[0].flux / 1e6,
                        100. * b.heat / b.capacity,
                        Turbopump::diluent_per_mw_min(&k),
                        b.rpm / 1e3,
                    );
                }
            }
        }
    }
    Ok(())
}

fn compare(sums: &[(Diluent, Summary)]) {
    println!("== Ammonia against methane");
    print!("  {:46}", "");
    for (d, _) in sums {
        print!(" {:>24}", d);
    }
    println!();
    let line = |name: &str, f: &dyn Fn(&Diluent, &Summary) -> String| {
        print!("  {name:46}");
        for (d, s) in sums {
            print!(" {:>24}", f(d, s));
        }
        println!();
    };
    line("liquid kg/m3, stored at", &|d, _| {
        format!("{:.0}, {:.0} K", d.density(), d.storage())
    });
    line("reverter at 5,000 K: dust:diluent, soot", &|_, s| {
        format!("1:{:.2}, {:.1}%", s.reverter_mix, 100. * s.reverter_soot)
    });
    line("per MW-minute: diluent, dust kg", &|_, s| {
        format!("{:.2}, {:.2}", s.per_mw.0, s.per_mw.1)
    });
    line("turbopump exhaust soot", &|_, s| format!("{:.1}%", 100. * s.exhaust_soot));
    line("sea level dry / reheat kN, dry Isp s", &|_, s| {
        format!("{:.0} / {:.0}, {:.0}", s.sls.0 / 1e3, s.sls.1 / 1e3, s.sls.2)
    });
    line("nitrogen air Isp / earth air", &|_, s| format!("{:.2}", s.n2_isp_ratio));
    line("Mach 2 reheat on 35 MW, kN", &|_, s| format!("{:.0}", s.m2_thrust / 1e3));
    line("cruise km per tonne, per m3, diluent:dust", &|_, s| {
        format!("{:.0}, {:.0}, {:.1}", s.cruise.0, s.cruise.1, s.cruise.2)
    });
    line("hover kg/min diluent, dust; jet K", &|_, s| {
        format!("{:.0}, {:.0}; {:.0}", s.hover.0, s.hover.1, s.hover.2)
    });
    line("FEL 10 min kg diluent, dust", &|_, s| {
        format!("{:.0}, {:.0}", s.fel.0, s.fel.1)
    });
    line("rocket 1:1 10 MPa / 1:2 2 MPa s, soot", &|_, s| {
        format!("{:.0} / {:.0}, {:.0}%", s.rocket.0, s.rocket.1, 100. * s.rocket.2)
    });
    line("blades MW/m2; coolant used cracked, uncracked", &|_, s| {
        format!("{:.0}; {:.0}%, {:.0}%", s.blades.0 / 1e6, s.blades.1, s.blades.2)
    });
}

fn flowpath(spec: &EngineSpec) -> Result<()> {
    let mut fp = FlowpathSpec::default();
    if let Some(n) = flag("--fan-rotors") {
        fp.fan_rotors = n.parse()?;
    }
    if let Some(u) = flag("--fan-tip") {
        fp.fan_tip_speed = u.parse()?;
    }
    if let Some(h) = flag("--hub-tip") {
        fp.fan_hub_tip = h.parse()?;
    }
    fp.counter_rotating = std::env::args().any(|a| a == "--counter");
    let f = fp.design(spec)?;
    println!(
        "== Air path: {} fan rotor{}{}, tip {:.0} m/s; {} compressor stages at {:.0} m/s",
        fp.fan_rotors,
        if fp.fan_rotors > 1 { "s" } else { "" },
        if fp.counter_rotating { ", counter-rotating" } else { "" },
        fp.fan_tip_speed,
        fp.compressor_stages,
        fp.compressor_speed
    );
    println!(
        "  {:10} {:>4} {:>7} {:>6} {:>6} {:>6} {:>13} {:>13} {:>16} {:>15} {:>15} {:>8}",
        "row", "Z", "rpm", "chord", "x le", "x te", "hub in/out", "tip in/out", "rel Mach h/m/t", "DF h/m/t", "stagger h/m/t", "ds J/kgK"
    );
    for r in &f.rows {
        let kind = match (r.component, r.spin) {
            (c, 0) => format!("{c:?} S"),
            (c, s) => format!("{c:?} R{}", if s < 0 { "-" } else { "" }),
        };
        let [h, m, t] = r.sections;
        println!(
            "  {kind:10} {:4} {:7.0} {:6.3} {:6.3} {:6.3} {:6.3}/{:6.3} {:6.3}/{:6.3} {:5.2}/{:4.2}/{:4.2} {:4.2}/{:4.2}/{:4.2} {:4.0}/{:4.0}/{:4.0} {:8.2}",
            r.blades, r.rpm, r.chord, r.x_le, r.x_te, r.hub.0, r.hub.1, r.tip.0, r.tip.1,
            h.mach, m.mach, t.mach, h.diffusion, m.diffusion, t.diffusion,
            h.stagger, m.stagger, t.stagger, r.ds
        );
    }
    println!(
        "  efficiency by the meanline's losses: fan {:.3} (cycle assumes {:.2}), compressor {:.3} ({:.2})",
        f.eta.0, spec.eta_fan, f.eta.1, spec.eta_compressor
    );
    println!(
        "  splitter at x {:.2} m, r {:.3}; combustor x {:.2}-{:.2}, r {:.3}-{:.3}; bypass duct r {:.3}-{:.3}",
        f.splitter.0, f.splitter.1, f.combustor.x0, f.combustor.x1, f.combustor.r_in, f.combustor.r_out,
        f.bypass.r_in, f.bypass.r_out
    );
    println!(
        "  mixer at x {:.2}, cone/lobes/wall r {:.3}/{:.3}/{:.3}; reheat to x {:.2}, r {:.3}; nozzle throat/exit r dry {:.3}/{:.3}, reheat {:.3}/{:.3}",
        f.mixer.x, f.mixer.r_cone, f.mixer.r_lobes, f.mixer.r_out, f.reheat.x1, f.reheat.r_out, f.nozzle.0.throat, f.nozzle.0.exit,
        f.nozzle.1.throat, f.nozzle.1.exit
    );
    println!(
        "  hover: first rotor at {:.2} of design speed, pitch {:+.1} deg at mean, {:+.1} at tip",
        f.pitch.speed, f.pitch.mean, f.pitch.tip
    );
    println!(
        "  motors, kg: {}",
        f.motors.iter().map(|m| format!("{:.0} ({:.2} m)", m.kg, m.x1 - m.x0)).collect::<Vec<_>>().join(", ")
    );
    let (b, m_gas, _) = turbopump_blades(spec)?;
    let reverter = spec.turbopump.reverter()?.1;
    let generator = TURBOPUMP_MAX / MOTOR_SPECIFIC;
    let turbopump = TurbopumpGeometry {
        mean_radius: b.mean_radius,
        rpm: b.rpm,
        blades: b.stages[..b.n_stages].iter().map(|s| s.blade).collect(),
        generator,
        generator_length: generator / MACHINE_DENSITY / (PI * GENERATOR_RADIUS * GENERATOR_RADIUS),
        generator_radius: GENERATOR_RADIUS,
        reverter: m_gas * REVERTER_RESIDENCE / reverter.rho(),
        flow: m_gas,
    };
    println!(
        "  turbopump: {} stages at r {:.0} mm, {:.0} rpm, blades {:.1}-{:.1} mm; generator {:.0} kg; reverter {:.1} L for {:.2} kg/s",
        turbopump.blades.len(),
        turbopump.mean_radius * 1e3,
        turbopump.rpm,
        turbopump.blades[0] * 1e3,
        turbopump.blades[turbopump.blades.len() - 1] * 1e3,
        turbopump.generator,
        turbopump.reverter * 1e3,
        turbopump.flow
    );
    if let Some(out) = flag("--json") {
        serde_json::to_writer_pretty(
            File::create(&out).context(out.clone())?,
            &serde_json::json!({ "air": f, "turbopump": turbopump }),
        )?;
    }
    Ok(())
}

/// The reverter holds its gas this long, s. The generator's radius, m.
const REVERTER_RESIDENCE: f64 = 3e-3;
const GENERATOR_RADIUS: f64 = 0.16;

#[derive(serde::Serialize)]
struct TurbopumpGeometry {
    mean_radius: f64,
    rpm: f64,
    /// Blade height of each stage, m.
    blades: Vec<f64>,
    /// kg; m, m
    generator: f64,
    generator_length: f64,
    generator_radius: f64,
    /// m3
    reverter: f64,
    /// kg/s
    flow: f64,
}

fn main() -> Result<()> {
    let spec = read_spec()?;
    if std::env::args().any(|a| a == "--blades") {
        return blade_envelope(&spec);
    }
    if std::env::args().any(|a| a == "--flowpath") {
        return flowpath(&spec);
    }
    let mut sums = Vec::new();
    for d in Diluent::ALL {
        let spec = with_diluent(&spec, d);
        let mut sum = Summary::default();
        chemistry(d, &mut sum)?;
        turbopumps(d, &mut sum)?;
        engine(&spec, &mut sum)?;
        mission(&spec, &mut sum)?;
        hover(&spec, &mut sum)?;
        loads(&spec, &mut sum)?;
        blades(&spec, &mut sum)?;
        sums.push((d, sum));
    }
    compare(&sums);
    Ok(())
}
