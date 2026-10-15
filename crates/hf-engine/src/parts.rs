//! Engine components. A `Stream` is a mass flow at its stagnation state;
//! burners and the mixer reach chemical equilibrium, compressors keep
//! their composition frozen, and expansions (turbine, nozzles) shift
//! equilibrium as they go.

use crate::gas::{brent, eq_hp, Diluent, Feed, Gas};
use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug)]
pub struct Stream {
    /// kg/s
    pub m: f64,
    /// Stagnation state.
    pub gas: Gas,
}

impl Stream {
    pub fn part(&self) -> (f64, Feed) {
        (self.m, self.gas.feed())
    }
}

/// Dust slurried in its diluent, `carrier` kg of diluent per kg of dust.
#[derive(Clone, Copy, Debug)]
pub struct Slurry {
    pub diluent: Diluent,
    pub carrier: f64,
}

impl Slurry {
    /// Per kg of slurry.
    pub fn feed(self) -> Feed {
        Feed::blend(&[(1., Feed::dust()), (self.carrier, self.diluent.feed(0.))])
    }
}

/// Compress frozen-composition gas by `pr` at isentropic efficiency `eta`;
/// returns the delivered gas and the work, J/kg.
pub fn compress(g: Gas, pr: f64, eta: f64) -> Result<(Gas, f64)> {
    if pr <= 1. {
        return Ok((g, 0.));
    }
    let p = g.p * pr;
    let hs = g.frozen_s(g.s(), p)?.h();
    let w = (hs - g.h()) / eta;
    Ok((g.frozen_h(g.h() + w, p)?, w))
}

/// Burn `parts` to equilibrium at `p`.
pub fn burn(parts: &[(f64, Feed)], p: f64) -> Result<Stream> {
    let m = parts.iter().map(|p| p.0).sum();
    Ok(Stream { m, gas: eq_hp(&Feed::blend(parts), p, None)? })
}

/// Burn `parts` at `p`, adding slurry until the gas reaches `t`. Returns the
/// stream and the dust added, kg/s. No slurry if the parts are already as
/// hot.
pub fn burn_to(
    parts: &[(f64, Feed)],
    p: f64,
    t: f64,
    slurry: Slurry,
) -> Result<(Stream, f64)> {
    let base = burn(parts, p)?;
    if base.gas.t >= t {
        return Ok((base, 0.));
    }
    let (m, feed, fuel, carrier) =
        (base.m, Feed::blend(parts), slurry.feed(), slurry.carrier);
    let with = |d: f64| burn(&[(m, feed), (d * (1. + carrier), fuel)], p);
    let d = brent(|d| Ok(with(d)?.gas.t - t), 0., base.m, 1e-6)?;
    Ok((with(d)?, d))
}

/// Fully expanded exit velocity and exit gas from stagnation state `g` to
/// `p`, with shifting equilibrium and kinetic-energy efficiency `eta`.
pub fn expand(g: &Gas, p: f64, eta: f64) -> Result<(f64, Gas)> {
    if p >= g.p {
        return Ok((0., *g));
    }
    let ge = g.eq_s(g.s(), p)?;
    Ok(((2. * eta * (g.h() - ge.h())).sqrt(), ge))
}

/// Mix a core and a bypass stream at constant static pressure, set by the
/// bypass flowing at Mach `bypass_mach`. Momentum and energy are
/// conserved; the difference in the streams' speeds is lost to mixing.
/// Returns the mixed stream at its stagnation state.
pub fn mix(core: &Stream, bypass: &Stream, bypass_mach: f64) -> Result<Stream> {
    let gb0 = bypass.gas;
    let p_mix = gb0.p
        / (1. + (gb0.gamma() - 1.) / 2. * bypass_mach.powi(2))
            .powf(gb0.gamma() / (gb0.gamma() - 1.));
    if p_mix >= core.gas.p {
        bail!(
            "core at {:.0} kPa cannot enter a mixer at {:.0} kPa",
            core.gas.p / 1e3,
            p_mix / 1e3
        );
    }
    let gc = core.gas.eq_s(core.gas.s(), p_mix)?;
    let vc = (2. * (core.gas.h() - gc.h())).sqrt();
    let gb = gb0.frozen_s(gb0.s(), p_mix)?;
    let vb = (2. * (gb0.h() - gb.h())).sqrt();
    let m = core.m + bypass.m;
    let vm = (core.m * vc + bypass.m * vb) / m;
    let mixed = Feed::blend(&[core.part(), bypass.part()]);
    let h0 = mixed.h;
    let stat = eq_hp(&Feed { b: mixed.b, h: h0 - vm * vm / 2. }, p_mix, Some(&gc))?;
    let s = stat.s();
    let mut warm = stat;
    let lnp = brent(
        |lnp| {
            warm = warm.eq_s(s, lnp.exp())?;
            Ok(warm.h() - h0)
        },
        p_mix.ln(),
        (p_mix * 100.).ln(),
        1e-9,
    )?;
    Ok(Stream { m, gas: warm.eq_s(s, lnp.exp())? })
}

/// How far below the temperature an endless flood of slurry would reach a
/// reheat stage aims, K.
const REHEAT_MARGIN: f64 = 150.;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct TurbopumpSpec {
    /// Turbine inlet temperature, K; every stage starts at it with reheat.
    pub tit: f64,
    /// Reverter pressure, Pa.
    pub p_reverter: f64,
    pub stages: usize,
    /// Isentropic efficiency of each stage.
    pub eta_stage: f64,
    /// Dust slurry injected through the stator rows to bring each stage
    /// back to `tit`.
    pub reheat: bool,
    pub diluent: Diluent,
    /// kg of diluent carrying each kg of slurried dust.
    pub carrier: f64,
    /// J per kg of diluent picked up cooling the hot section before it
    /// reaches the reverter.
    pub regen_heat: f64,
    pub eta_generator: f64,
    pub eta_pump: f64,
    /// kg/m3
    pub slurry_density: f64,
}

impl Default for TurbopumpSpec {
    fn default() -> Self {
        Self {
            tit: 5000.,
            p_reverter: 10e6,
            stages: 6,
            eta_stage: 0.88,
            reheat: false,
            diluent: Diluent::Ammonia,
            carrier: 0.56,
            regen_heat: 0.,
            eta_generator: 0.98,
            eta_pump: 0.75,
            slurry_density: 700.,
        }
    }
}

pub const MAX_STAGES: usize = 16;

#[derive(Clone, Copy, Debug, Default)]
pub struct Stage {
    pub inlet: Gas,
    pub m: f64,
    pub work: f64,
}

/// What a turbopump consumes and delivers, per kg of gas leaving its
/// reverter.
#[derive(Clone, Copy, Debug)]
pub struct TurbopumpPerKg {
    /// kg of diluent per kg of dust in the reverter.
    pub mix: f64,
    /// Each stage's gas entering its rotor (after reheat), its flow, and
    /// its work, J; the first `n_stages` are filled.
    pub stages: [Stage; MAX_STAGES],
    pub n_stages: usize,
    pub diluent: f64,
    pub dust: f64,
    /// Electricity, J.
    pub electric: f64,
    /// Exhaust at the turbine exit.
    pub exhaust: Stream,
}

impl TurbopumpSpec {
    pub fn slurry(&self) -> Slurry {
        Slurry { diluent: self.diluent, carrier: self.carrier }
    }

    /// The reverter's gas: `mix` kg of diluent per kg of dust burnt to
    /// `tit` at `p_reverter`.
    pub fn reverter(&self) -> Result<(f64, Gas)> {
        let feed = |x: f64| {
            Feed::blend(&[(1., Feed::dust()), (x, self.diluent.feed(self.regen_heat))])
        };
        let t = |x: f64| -> Result<f64> { Ok(eq_hp(&feed(x), self.p_reverter, None)?.t) };
        if t(0.)? < self.tit {
            bail!("pure dust does not reach {} K", self.tit);
        }
        let x = brent(|x| Ok(t(x)? - self.tit), 0., 100., 1e-7)?;
        Ok((x, eq_hp(&feed(x), self.p_reverter, None)?))
    }

    pub fn per_kg(&self, p_exit: f64) -> Result<TurbopumpPerKg> {
        let (x, mut g) = self.reverter()?;
        let mut m = 1.;
        let (mut dust, mut diluent) = (1. / (1. + x), x / (1. + x));
        let pr = (self.p_reverter / p_exit).powf(1. / self.stages as f64);
        let fuel = self.slurry().feed();
        if self.stages > MAX_STAGES {
            bail!("at most {MAX_STAGES} stages");
        }
        let mut work = 0.;
        let mut stages = [Stage::default(); MAX_STAGES];
        for k in 0..self.stages {
            if k > 0 && self.reheat {
                let heated = |d: f64| -> Result<Gas> {
                    let f =
                        Feed::blend(&[(m, g.feed()), (d * (1. + self.carrier), fuel)]);
                    eq_hp(&f, g.p, Some(&g))
                };
                // Dissociation caps what slurry can reach at low pressure.
                let target = self.tit.min(heated(50. * m)?.t - REHEAT_MARGIN);
                if target > g.t {
                    let d = brent(|d| Ok(heated(d)?.t - target), 0., 50. * m, 1e-7)?;
                    g = heated(d)?;
                    m += d * (1. + self.carrier);
                    dust += d;
                    diluent += d * self.carrier;
                }
            }
            let inlet = g;
            let p = g.p / pr;
            let h_in = g.h();
            let hs = g.eq_s(g.s(), p)?.h();
            let h_out = h_in - self.eta_stage * (h_in - hs);
            g = g.eq_h(h_out, p)?;
            work += m * (h_in - h_out);
            stages[k] = Stage { inlet, m, work: m * (h_in - h_out) };
        }
        let pump = (dust + diluent) * (self.p_reverter - 0.5e6)
            / self.slurry_density
            / self.eta_pump;
        Ok(TurbopumpPerKg {
            mix: x,
            stages,
            n_stages: self.stages,
            diluent,
            dust,
            electric: (work - pump) * self.eta_generator,
            exhaust: Stream { m, gas: g },
        })
    }
}

/// A turbopump making `electric` W into a combustor at `p_exit`.
#[derive(Clone, Copy, Debug)]
pub struct Turbopump {
    pub diluent: f64,
    pub dust: f64,
    pub exhaust: Stream,
    pub per_kg: TurbopumpPerKg,
}

impl Turbopump {
    pub fn run(spec: &TurbopumpSpec, electric: f64, p_exit: f64) -> Result<Self> {
        let per_kg = spec.per_kg(p_exit)?;
        Ok(Self::scale(per_kg, electric))
    }

    pub fn scale(per_kg: TurbopumpPerKg, electric: f64) -> Self {
        let k = electric / per_kg.electric;
        Self {
            diluent: per_kg.diluent * k,
            dust: per_kg.dust * k,
            exhaust: Stream { m: per_kg.exhaust.m * k, gas: per_kg.exhaust.gas },
            per_kg,
        }
    }

    /// kg of diluent per MW-minute of electricity.
    pub fn diluent_per_mw_min(per_kg: &TurbopumpPerKg) -> f64 {
        per_kg.diluent / per_kg.electric * 6e7
    }

    /// kg of dust per MW-minute of electricity.
    pub fn dust_per_mw_min(per_kg: &TurbopumpPerKg) -> f64 {
        per_kg.dust / per_kg.electric * 6e7
    }
}

/// Standard gravity.
pub const G0: f64 = 9.80665;
