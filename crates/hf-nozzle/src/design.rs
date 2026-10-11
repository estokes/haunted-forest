//! A parametric nozzle: coaxial rings (the first is the mirror toward the
//! ship), a saddle-pair sweeper around the axis, spokes from the boom tip to
//! the mirror ring, and the ship at the far end of the boom. Specs come in as JSON with every field defaulted.

use crate::{
    field::{Coils, Loop, Part, Wire},
    track::{self, Body, Run, Ship, C},
};
use anyhow::{bail, Result};
use glam::DVec3;
use serde::{Deserialize, Serialize};
use std::{f64::consts::PI, time::Instant};

const RHO_CNT: f64 = 1400.0; // conductor-plus-structure density, kg/m^3
const SIG_ALLOW: f64 = 25e9; // allowable stress, Pa
const MASS_MARGIN: f64 = 10.0; // virial minimum to a real magnet
const WIRE_R: f64 = 0.1; // conductor radius for inductance, m

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct RingSpec {
    pub r: f64,
    /// Ring plane distance toward the ship.
    pub z: f64,
    /// Current relative to the other rings; the set is scaled to make b0.
    pub i_rel: f64,
    /// Tilt of the ring's axis from +z toward +y.
    pub tilt_deg: f64,
}

impl Default for RingSpec {
    fn default() -> Self {
        Self { r: 535.0, z: 800.0, i_rel: 1.0, tilt_deg: 0.0 }
    }
}

/// Two saddle coils on a cylinder of radius a from z1 to z2, legs 60 and 120
/// degrees either side of +x, making a transverse field b along +x on the
/// axis midway between z1 and z2.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct SweeperSpec {
    pub a: f64,
    pub z1: f64,
    pub z2: f64,
    pub b: f64,
    pub n_arc: usize,
}

impl Default for SweeperSpec {
    fn default() -> Self {
        Self { a: 12.0, z1: 975.0, z2: 995.0, b: 0.5, n_arc: 8 }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
/// Struts from a hub circle of radius hub_r at hub_z out to the mirror ring.
pub struct SpokeSpec {
    pub n: usize,
    pub phase_deg: f64,
    pub hub_r: f64,
    pub hub_z: f64,
}

impl Default for SpokeSpec {
    fn default() -> Self {
        Self { n: 4, phase_deg: 45.0, hub_r: 8.0, hub_z: 1000.0 }
    }
}

/// Where the ship's bodies sit.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Layout {
    /// One stack on the axis beyond the mirror ring, plate at z.
    Axial { z: f64, r: f64, length: f64 },
    /// n pods spaced around the mirror ring, each on the line from the
    /// reaction through the conductor, its plate `gap` beyond it.
    Pods { n: usize, r: f64, length: f64, gap: f64, phase_deg: f64 },
}

impl Default for Layout {
    fn default() -> Self {
        Layout::Pods { n: 2, r: 5.7, length: 220.0, gap: 20.0, phase_deg: 0.0 }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Spec {
    /// Gamma power, which with tile_mw_m2 sets how close structure may sit.
    pub gamma_tw: f64,
    pub tile_mw_m2: f64,
    /// |B| at the origin, T; the ring currents follow.
    pub b0: f64,
    pub rings: Vec<RingSpec>,
    pub sweeper: Option<SweeperSpec>,
    pub spokes: Option<SpokeSpec>,
    pub ship: Layout,
    /// Centre of mass, for torques.
    pub com: [f64; 3],
    pub run: Run,
}

impl Default for Spec {
    fn default() -> Self {
        Self {
            gamma_tw: 180.0,
            tile_mw_m2: 50.0,
            b0: 0.005,
            rings: vec![RingSpec::default()],
            sweeper: None,
            spokes: None,
            ship: Layout::default(),
            com: [0.0, 0.0, 800.0],
            run: Run::default(),
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct ByPart {
    pub ring: f64,
    pub sweeper: f64,
    pub spoke: f64,
}

#[derive(Clone, Debug, Serialize)]
pub struct Species {
    pub pion: f64,
    pub muon: f64,
    pub electron: f64,
}

#[derive(Clone, Debug, Serialize)]
pub struct Report {
    pub spec: Spec,
    pub magnet_t: f64,
    /// Gamma power falling on the conductors, GW.
    pub shadow_gw: f64,
    pub ring_ma: Vec<f64>,
    pub sweeper_ma: f64,
    /// Closest tiled structure may sit at r_min; the peak flux any conductor,
    /// strut or the boom tip sees.
    pub r_min: f64,
    pub peak_flux_mw_m2: f64,
    pub ve_eff: f64,
    pub n_per_gw: f64,
    /// Side thrust over axial thrust, x and y.
    pub side: [f64; 2],
    /// Torque about the centre of mass over axial thrust, about x and y, m.
    pub torque_arm: [f64; 2],
    pub plate_ppm: f64,
    pub side_ppm: f64,
    /// Tracks that struck the ship anywhere, for the statistics of the above.
    pub ship_hits: u64,
    /// Charged power striking the ship, ppm, by track::HIT_BIN_M of position
    /// along the struck body from its plate, and by species.
    pub hits_along_ppm: Vec<f64>,
    pub hits_species_ppm: Species,
    /// Gamma power on the ship's plates.
    pub plate_gw: f64,
    pub coil_ppm: ByPart,
    pub forward: f64,
    pub neutrino: f64,
    pub trapped: f64,
    pub secs: f64,
}

fn arc(a: f64, z: f64, phi0: f64, phi1: f64, n: usize) -> impl Iterator<Item = DVec3> {
    (0..=n).map(move |j| {
        let ph = phi0 + (phi1 - phi0) * j as f64 / n as f64;
        DVec3::new(a * ph.cos(), a * ph.sin(), z)
    })
}

fn saddle_pair(s: &SweeperSpec) -> Result<Vec<Wire>> {
    if !(s.a > 0.0 && s.z1 < s.z2 && s.n_arc > 0) {
        bail!("sweeper needs a > 0, z1 < z2 and n_arc > 0: {s:?}")
    }
    let t = PI / 3.0;
    let unit: Vec<Wire> = [(t, -t), (2.0 * t, 4.0 * t)]
        .into_iter()
        .map(|(p0, p1)| {
            let pts = std::iter::once(DVec3::new(s.a * p0.cos(), s.a * p0.sin(), s.z1))
                .chain(arc(s.a, s.z2, p0, p1, s.n_arc))
                .chain(arc(s.a, s.z1, p1, p0, s.n_arc))
                .collect();
            Wire { pts, i: 1.0, part: Part::Sweeper }
        })
        .collect();
    let bx = Coils::new(vec![], &unit).field(DVec3::new(0.0, 0.0, 0.5 * (s.z1 + s.z2))).x;
    Ok(unit.into_iter().map(|w| Wire { i: s.b / bx, ..w }).collect())
}

impl Spec {
    pub fn r_min(&self) -> f64 {
        (self.gamma_tw * 1e12 / (4.0 * PI * self.tile_mw_m2 * 1e6)).sqrt()
    }

    pub fn build(&self) -> Result<(Coils, Ship)> {
        if self.rings.is_empty() {
            bail!("a nozzle needs at least the mirror ring")
        }
        let unit: Vec<(Loop, Part)> = self
            .rings
            .iter()
            .map(|r| {
                let th = r.tilt_deg.to_radians();
                let axis = DVec3::new(0.0, th.sin(), th.cos());
                (Loop::new(r.r, DVec3::new(0.0, 0.0, r.z), axis, r.i_rel), Part::Ring)
            })
            .collect();
        let b = Coils::new(unit.clone(), &[]).field(DVec3::ZERO).length();
        let rings: Vec<(Loop, Part)> = unit
            .into_iter()
            .map(|(l, p)| {
                let i = l.i * self.b0 / b;
                (l.with_current(i), p)
            })
            .collect();
        let mirror = &rings[0].0;
        let mut wires = match &self.sweeper {
            Some(s) => saddle_pair(s)?,
            None => vec![],
        };
        if let Some(sp) = &self.spokes {
            wires.extend((0..sp.n).map(|k| {
                let ph = sp.phase_deg.to_radians() + 2.0 * PI * k as f64 / sp.n as f64;
                let hub = DVec3::new(sp.hub_r * ph.cos(), sp.hub_r * ph.sin(), sp.hub_z);
                Wire { pts: vec![hub, mirror.at(ph)], i: 0.0, part: Part::Spoke }
            }));
        }
        let bodies = match self.ship {
            Layout::Axial { z, r, length } => {
                vec![Body { a: [0.0, 0.0, z], b: [0.0, 0.0, z + length], r }]
            }
            Layout::Pods { n, r, length, gap, phase_deg } => (0..n)
                .map(|k| {
                    let p = mirror
                        .at(phase_deg.to_radians() + 2.0 * PI * k as f64 / n as f64);
                    let u = p.normalize();
                    let a = p + gap * u;
                    Body { a: a.into(), b: (a + length * u).into(), r }
                })
                .collect(),
        };
        Ok((Coils::new(rings, &wires), Ship { bodies }))
    }

    pub fn evaluate(&self) -> Result<Report> {
        let start = Instant::now();
        let (coils, ship) = self.build()?;
        let source = DVec3::from(self.run.source);
        let closest = coils.closest(source).min(ship.distance(source));
        let plate_w: f64 = ship
            .bodies
            .iter()
            .map(|b| {
                PI * b.r * b.r * self.gamma_tw * 1e12
                    / (4.0 * PI * (DVec3::from(b.a) - source).length_squared())
            })
            .sum();
        let t = track::simulate(&coils, &ship, &self.run);
        let e = track::e_ann(&self.run);
        let f = -(t.escape_p + t.nu_p) / e; // thrust per power, units of 1/c
        let torque = -(t.escape_l + t.nu_l) / e - DVec3::from(self.com).cross(f);
        let ppm = |x: f64| x / e * 1e6;
        let magnet_t =
            MASS_MARGIN * coils.stored_energy(WIRE_R, 5.0) * RHO_CNT / SIG_ALLOW / 1e3;
        let [ring, sweeper, spoke] = Part::ALL.map(|p| ppm(t.coil_e[p as usize]));
        Ok(Report {
            spec: self.clone(),
            magnet_t,
            shadow_gw: self.gamma_tw
                * 1e3
                * coils.shadow_fraction(source, magnet_t * 1e3, RHO_CNT),
            ring_ma: coils.loops.iter().map(|(l, _)| l.i / 1e6).collect(),
            sweeper_ma: self
                .sweeper
                .as_ref()
                .map_or(0.0, |_| coils.current(Part::Sweeper) / 1e6),
            r_min: self.r_min(),
            peak_flux_mw_m2: self.gamma_tw * 1e6 / (4.0 * PI * closest * closest),
            ve_eff: f.z,
            n_per_gw: f.z / C * 1e9,
            side: [f.x / f.z, f.y / f.z],
            torque_arm: [torque.x / f.z, torque.y / f.z],
            plate_ppm: ppm(t.plate_e),
            side_ppm: ppm(t.side_e),
            ship_hits: t.ship_n,
            hits_along_ppm: t.hit_along.iter().map(|&x| ppm(x)).collect(),
            hits_species_ppm: {
                let [pion, muon, electron] = t.hit_species.map(ppm);
                Species { pion, muon, electron }
            },
            plate_gw: plate_w / 1e9,
            coil_ppm: ByPart { ring, sweeper, spoke },
            forward: t.fwd_e / e,
            neutrino: t.nu_e / e,
            trapped: t.trapped_e / e,
            secs: start.elapsed().as_secs_f64(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn saddle_makes_its_field_and_little_on_axis_elsewhere() {
        let s = SweeperSpec::default();
        let c = Coils::new(vec![], &saddle_pair(&s).unwrap());
        let mid = c.field(DVec3::new(0.0, 0.0, 0.5 * (s.z1 + s.z2)));
        assert!(
            (mid.x - s.b).abs() < 1e-12 && mid.y.abs() < 1e-9 && mid.z.abs() < 1e-9,
            "{mid}"
        );
        assert!(c.field(DVec3::ZERO).length() < 1e-3 * s.b);
    }

    #[test]
    fn rings_scaled_to_b0() {
        let spec = Spec {
            rings: vec![
                RingSpec::default(),
                RingSpec { r: 800.0, z: -600.0, i_rel: 0.5, ..RingSpec::default() },
            ],
            ..Spec::default()
        };
        let (c, _) = spec.build().unwrap();
        let rings = Coils::new(c.loops.clone(), &[]);
        assert!((rings.field(DVec3::ZERO).length() - spec.b0).abs() < 1e-12);
        assert!((c.loops[1].0.i / c.loops[0].0.i - 0.5).abs() < 1e-12);
    }

    #[test]
    fn spec_fields_default() {
        let s: Spec = serde_json::from_str(
            r#"{"b0": 0.02, "rings": [{"z": 500}], "sweeper": null}"#,
        )
        .unwrap();
        assert_eq!(s.b0, 0.02);
        assert_eq!((s.rings[0].r, s.rings[0].z), (535.0, 500.0));
        assert!(matches!(s.ship, Layout::Pods { n: 2, .. }));
        let s: Spec = serde_json::from_str(
            r#"{"ship": {"axial": {"z": 1000, "r": 8, "length": 150}}}"#,
        )
        .unwrap();
        assert!(matches!(s.ship, Layout::Axial { .. }));
        assert!(s.sweeper.is_none() && s.spokes.is_none());
        let s: Spec = serde_json::from_str(r#"{"spokes": {}}"#).unwrap();
        assert_eq!(s.spokes.unwrap().n, 4);
    }
}
