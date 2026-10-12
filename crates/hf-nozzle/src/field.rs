//! Magnetic fields of circular loops (exact, by elliptic integrals) and
//! straight wire segments (exact Biot-Savart), with clearance and stored
//! energy for the same conductors.

use glam::DVec3;
use serde::Serialize;
use std::f64::consts::PI;

pub const MU0: f64 = 4e-7 * PI;

/// What a conductor or strut is, for attributing particle hits.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub enum Part {
    Ring,
    Sweeper,
    Spoke,
}

impl Part {
    pub const ALL: [Part; 3] = [Part::Ring, Part::Sweeper, Part::Spoke];
}

#[derive(Clone, Debug)]
pub struct Loop {
    pub r: f64,
    pub center: DVec3,
    pub i: f64,
    u: DVec3,
    v: DVec3,
    n: DVec3,
}

impl Loop {
    pub fn new(r: f64, center: DVec3, axis: DVec3, i: f64) -> Self {
        let n = axis.normalize();
        let t = if n.x.abs() < 0.9 { DVec3::X } else { DVec3::Y };
        let u = n.cross(t).normalize();
        Self { r, center, i, u, v: n.cross(u), n }
    }

    pub fn with_current(self, i: f64) -> Self {
        Self { i, ..self }
    }

    /// A point on the loop at azimuth phi in its own frame.
    pub fn at(&self, phi: f64) -> DVec3 {
        self.center + self.r * (phi.cos() * self.u + phi.sin() * self.v)
    }

    fn local(&self, x: DVec3) -> (f64, f64, f64) {
        let d = x - self.center;
        (d.dot(self.u), d.dot(self.v), d.dot(self.n))
    }

    fn field(&self, x: DVec3) -> DVec3 {
        let (lx, ly, lz) = self.local(x);
        let rho = lx.hypot(ly);
        let (br, bz) = loop_field(self.r, self.i, rho, lz);
        let radial =
            if rho > 1e-9 { (lx * self.u + ly * self.v) / rho } else { DVec3::ZERO };
        br * radial + bz * self.n
    }

    fn distance(&self, x: DVec3) -> f64 {
        let (lx, ly, lz) = self.local(x);
        (lx.hypot(ly) - self.r).hypot(lz)
    }
}

/// A polyline carrying current from its first point to its last; i = 0 is a
/// bare strut.
#[derive(Clone, Debug)]
pub struct Wire {
    pub pts: Vec<DVec3>,
    pub i: f64,
    pub part: Part,
}

#[derive(Clone, Copy, Debug)]
struct Seg {
    a: DVec3,
    b: DVec3,
    i: f64,
    part: Part,
}

impl Seg {
    fn distance(&self, x: DVec3) -> f64 {
        let ab = self.b - self.a;
        let t = ((x - self.a).dot(ab) / ab.length_squared()).clamp(0.0, 1.0);
        (x - (self.a + t * ab)).length()
    }

    fn field(&self, x: DVec3) -> DVec3 {
        let a = self.a - x;
        let c = self.b - x;
        let (an, cn) = (a.length(), c.length());
        let den = (an * cn * (an * cn + a.dot(c))).max(1e-9);
        MU0 / (4.0 * PI) * self.i * (an + cn) / den * a.cross(c)
    }
}

#[derive(Clone, Debug)]
pub struct Coils {
    pub loops: Vec<(Loop, Part)>,
    segs: Vec<Seg>,
    live: Vec<Seg>,
    box_lo: DVec3,
    box_hi: DVec3,
}

/// Margin around the wires' bounding box inside which clearance is exact.
const BOX_MARGIN: f64 = 50.0;

impl Coils {
    pub fn new(loops: Vec<(Loop, Part)>, wires: &[Wire]) -> Self {
        let segs: Vec<Seg> = wires
            .iter()
            .flat_map(|w| {
                w.pts.windows(2).map(|p| Seg { a: p[0], b: p[1], i: w.i, part: w.part })
            })
            .collect();
        let live = segs.iter().copied().filter(|s| s.i != 0.0).collect();
        let (mut lo, mut hi) = (DVec3::INFINITY, DVec3::NEG_INFINITY);
        for s in &segs {
            lo = lo.min(s.a).min(s.b);
            hi = hi.max(s.a).max(s.b);
        }
        let m = DVec3::splat(BOX_MARGIN);
        Self { loops, segs, live, box_lo: lo - m, box_hi: hi + m }
    }

    pub fn field(&self, x: DVec3) -> DVec3 {
        let mut b = DVec3::ZERO;
        for (l, _) in &self.loops {
            b += l.field(x);
        }
        for s in &self.live {
            b += s.field(x);
        }
        b
    }

    /// Distance to the nearest conductor or strut, and what it is. Outside
    /// the wires' bounding box the wires are reported no nearer than the
    /// box, which is a lower bound.
    pub fn clearance(&self, x: DVec3) -> (f64, Part) {
        let mut best = (f64::INFINITY, Part::Ring);
        for (l, part) in &self.loops {
            let d = l.distance(x);
            if d < best.0 {
                best = (d, *part);
            }
        }
        if self.segs.is_empty() {
            return best;
        }
        let gap = (self.box_lo - x).max(x - self.box_hi).max(DVec3::ZERO);
        if gap != DVec3::ZERO {
            let d = gap.length() + BOX_MARGIN;
            if d < best.0 {
                best = (d, self.segs[0].part);
            }
            return best;
        }
        for s in &self.segs {
            let d = s.distance(x);
            if d < best.0 {
                best = (d, s.part);
            }
        }
        best
    }

    /// Current in the first wire of a part.
    pub fn current(&self, part: Part) -> f64 {
        self.segs.iter().find(|s| s.part == part).map_or(0.0, |s| s.i)
    }

    /// Closest approach of any conductor or strut to a point.
    pub fn closest(&self, x: DVec3) -> f64 {
        let l = self.loops.iter().map(|(l, _)| l.distance(x));
        let s = self.segs.iter().map(|s| s.distance(x));
        l.chain(s).fold(f64::INFINITY, f64::min)
    }

    /// Conductors cut into straight pieces no longer than h: (midpoint, dl, I).
    fn pieces(&self, h: f64) -> Vec<(DVec3, DVec3, f64)> {
        let mut out = Vec::new();
        for (l, _) in &self.loops {
            let k = ((2.0 * PI * l.r / h) as usize).max(16);
            for j in 0..k {
                let p0 = l.at(2.0 * PI * j as f64 / k as f64);
                let p1 = l.at(2.0 * PI * (j + 1) as f64 / k as f64);
                out.push((0.5 * (p0 + p1), p1 - p0, l.i));
            }
        }
        for s in &self.live {
            let k = (((s.b - s.a).length() / h).ceil() as usize).max(1);
            let d = (s.b - s.a) / k as f64;
            for j in 0..k {
                out.push((s.a + (j as f64 + 0.5) * d, d, s.i));
            }
        }
        out
    }

    /// Gamma power, as a fraction of the total, falling on conductors whose
    /// cross-section is proportional to their current and whose total mass
    /// is `mass` at density `rho`: the load their shadowing tiles must take.
    pub fn shadow_fraction(&self, source: DVec3, mass: f64, rho: f64) -> f64 {
        let p = self.pieces(5.0);
        let amp_m: f64 = p.iter().map(|(_, dl, i)| i.abs() * dl.length()).sum();
        let area_per_amp = mass / rho / amp_m;
        p.iter()
            .map(|(m, dl, i)| {
                let d = *m - source;
                let r2 = d.length_squared();
                let width = (4.0 * area_per_amp * i.abs() / PI).sqrt();
                let edge_on = dl.normalize().cross(d / r2.sqrt()).length();
                width * dl.length() * edge_on / (4.0 * PI * r2)
            })
            .sum()
    }

    /// W = 1/2 sum I_p I_q M_pq over pieces: Neumann midpoint rule between
    /// pieces; the diagonal holds the straight-piece self inductance plus the
    /// midpoint rule's shortfall against its two touching neighbours.
    pub fn stored_energy(&self, wire_r: f64, h: f64) -> f64 {
        let p = self.pieces(h);
        let mut w = 0.0;
        for (j, (mj, dj, ij)) in p.iter().enumerate() {
            let l = dj.length();
            let self_l = MU0 * l / (2.0 * PI) * ((2.0 * l / wire_r).ln() - 1.0)
                + MU0 * l / (4.0 * PI) * 2.0 * (2.0 * 2f64.ln() - 1.0);
            let mut acc = self_l * ij;
            for (k, (mk, dk, ik)) in p.iter().enumerate() {
                if k != j {
                    acc += MU0 / (4.0 * PI) * dj.dot(*dk) / (*mj - *mk).length() * ik;
                }
            }
            w += 0.5 * ij * acc;
        }
        w
    }
}

/// Complete elliptic integrals K(m), E(m) by the arithmetic-geometric mean.
pub(crate) fn ellipke(m: f64) -> (f64, f64) {
    let (mut a, mut b) = (1.0, (1.0 - m).sqrt());
    let (mut s, mut p2) = (0.5 * m, 0.5);
    for _ in 0..12 {
        let c = 0.5 * (a - b);
        (a, b) = (0.5 * (a + b), (a * b).sqrt());
        p2 *= 2.0;
        s += p2 * c * c;
    }
    let k = PI / (2.0 * a);
    (k, k * (1.0 - s))
}

/// Flux function psi = rho A_phi of a circular loop of radius r in the z = 0
/// plane carrying i, at (rho, z): the flux through the circle of radius rho
/// there is 2 pi psi, and B_rho = -dpsi/dz / rho, B_z = dpsi/drho / rho.
pub(crate) fn loop_psi(r: f64, i: f64, rho: f64, z: f64) -> f64 {
    let d2 = (r + rho) * (r + rho) + z * z;
    let m = (4.0 * r * rho / d2).clamp(0.0, 1.0 - 1e-15);
    let (k, e) = ellipke(m);
    MU0 * i / (2.0 * PI) * d2.sqrt() * ((1.0 - 0.5 * m) * k - e)
}

/// (B_rho, B_z) of a circular loop of radius r in the z = 0 plane carrying i.
pub(crate) fn loop_field(r: f64, i: f64, rho: f64, z: f64) -> (f64, f64) {
    let s2 = r * r + rho * rho + z * z;
    let alpha2 = (s2 - 2.0 * r * rho).max(1e-12);
    let beta2 = s2 + 2.0 * r * rho;
    let beta = beta2.sqrt();
    let (k, e) = ellipke((1.0 - alpha2 / beta2).clamp(0.0, 1.0 - 1e-15));
    let c = MU0 * i / PI;
    let bz = c / (2.0 * alpha2 * beta) * ((r * r - rho * rho - z * z) * e + alpha2 * k);
    let br = if rho > 1e-9 {
        c * z / (2.0 * alpha2 * beta * rho)
            * ((r * r + rho * rho + z * z) * e - alpha2 * k)
    } else {
        0.0
    };
    (br, bz)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ring() -> Loop {
        Loop::new(535.0, DVec3::new(0.0, 0.0, 428.0), DVec3::Z, 1e6)
    }

    fn polygon(l: &Loop, k: usize) -> Wire {
        let pts = (0..=k).map(|j| l.at(2.0 * PI * j as f64 / k as f64)).collect();
        Wire { pts, i: l.i, part: Part::Ring }
    }

    const PROBES: [DVec3; 4] = [
        DVec3::new(10.0, 20.0, 30.0),
        DVec3::new(300.0, -100.0, 600.0),
        DVec3::ZERO,
        DVec3::new(-520.0, 40.0, 420.0),
    ];

    #[test]
    fn loop_flux_function_gives_its_field() {
        let (r, i) = (100.0, 1e6);
        for (rho, z) in [(30.0, 40.0), (150.0, -20.0), (99.0, 5.0), (0.5, 80.0)] {
            let h = 1e-3;
            let bz = (loop_psi(r, i, rho + h, z) - loop_psi(r, i, rho - h, z)) / (2.0 * h) / rho;
            let br = -(loop_psi(r, i, rho, z + h) - loop_psi(r, i, rho, z - h)) / (2.0 * h) / rho;
            let (er, ez) = loop_field(r, i, rho, z);
            let scale = er.hypot(ez);
            assert!((br - er).abs() < 1e-5 * scale && (bz - ez).abs() < 1e-5 * scale,
                "({rho}, {z}): ({br}, {bz}) vs ({er}, {ez})");
        }
        assert_eq!(loop_psi(r, i, 0.0, 10.0), 0.0);
    }

    #[test]
    fn loop_on_axis_matches_closed_form() {
        let l = ring();
        let z = 428.0;
        let want = MU0 * l.i * l.r.powi(2) / (2.0 * (l.r * l.r + z * z).powf(1.5));
        let got = Coils::new(vec![(l, Part::Ring)], &[]).field(DVec3::ZERO);
        assert!((got.z - want).abs() < 1e-9 * want, "{got} vs {want}");
        assert!(got.x.abs() + got.y.abs() < 1e-15);
    }

    #[test]
    fn loop_matches_fine_polygon_of_wires() {
        let l = ring();
        let exact = Coils::new(vec![(l.clone(), Part::Ring)], &[]);
        let poly = Coils::new(vec![], &[polygon(&l, 2000)]);
        for x in PROBES {
            let (a, b) = (exact.field(x), poly.field(x));
            assert!((a - b).length() < 2e-4 * a.length(), "{x}: {a} vs {b}");
        }
    }

    #[test]
    fn tilted_loop_is_rotated_field() {
        let rot = glam::DQuat::from_rotation_x(0.3);
        let l = ring();
        let t = Loop::new(l.r, rot * l.center, rot * DVec3::Z, l.i);
        let (a, b) = (
            Coils::new(vec![(l, Part::Ring)], &[]),
            Coils::new(vec![(t, Part::Ring)], &[]),
        );
        for x in PROBES {
            let (fa, fb) = (rot * a.field(x), b.field(rot * x));
            assert!((fa - fb).length() < 1e-12 * fa.length().max(1e-6), "{fa} vs {fb}");
        }
    }

    #[test]
    fn loop_inductance_matches_thin_ring_formula() {
        let l = ring();
        let want = MU0 * l.r * ((8.0 * l.r / 0.1).ln() - 2.0);
        let c = Coils::new(vec![(l.with_current(1.0), Part::Ring)], &[]);
        let got = 2.0 * c.stored_energy(0.1, 5.0);
        assert!((got - want).abs() < 0.01 * want, "{got} vs {want}");
    }

    #[test]
    fn rectangle_inductance_matches_rosa() {
        let (w, h, a) = (100.0, 400.0, 0.1);
        let pts = vec![
            DVec3::ZERO,
            DVec3::new(w, 0.0, 0.0),
            DVec3::new(w, h, 0.0),
            DVec3::new(0.0, h, 0.0),
            DVec3::ZERO,
        ];
        let want = MU0 / PI
            * (w * (2.0 * w / a).ln() + h * (2.0 * h / a).ln() + 2.0 * w.hypot(h)
                - w * (w / h).asinh()
                - h * (h / w).asinh()
                - 1.75 * (w + h));
        let c = Coils::new(vec![], &[Wire { pts, i: 1.0, part: Part::Sweeper }]);
        let got = 2.0 * c.stored_energy(a, 5.0);
        assert!((got - want).abs() < 0.05 * want, "{got} vs {want}");
    }

    #[test]
    fn clearance_finds_nearest_part() {
        let l = ring();
        let strut = Wire {
            pts: vec![DVec3::new(0.0, 0.0, 650.0), DVec3::new(0.0, 0.0, 700.0)],
            i: 0.0,
            part: Part::Spoke,
        };
        let c = Coils::new(vec![(l, Part::Ring)], &[strut]);
        let (d, p) = c.clearance(DVec3::new(5.0, 0.0, 660.0));
        assert!((d - 5.0).abs() < 1e-9 && p == Part::Spoke);
        let (d, p) = c.clearance(DVec3::new(535.0, 0.0, 431.0));
        assert!((d - 3.0).abs() < 1e-9 && p == Part::Ring);
        let (d, _) = c.clearance(DVec3::ZERO);
        assert!(d <= c.closest(DVec3::ZERO) + 1e-9);
    }
}
