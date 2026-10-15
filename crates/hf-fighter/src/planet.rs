//! A planet: gravity, rotation, and its air, the US76 atmosphere's
//! stretched or squeezed by its gravity.

use hf_engine::atmos::isa;

#[derive(Clone, Copy, Debug)]
pub struct Planet {
    pub radius: f64,
    /// Gravitational parameter, m3/s2.
    pub mu: f64,
    /// Rotation, rad/s.
    pub omega: f64,
    /// Its air at height z is Earth's at z times this.
    pub air_scale: f64,
}

pub const EARTH: Planet =
    Planet { radius: 6.371e6, mu: 3.986004418e14, omega: 7.2921159e-5, air_scale: 1.0 };

impl Planet {
    /// A world of Earth's density and day with `g` times Earth's surface
    /// gravity: g times Earth's radius, and Earth's surface air thinning
    /// with height g times as fast, its scale height shrunk by its gravity.
    pub fn earthlike(g: f64) -> Planet {
        Planet {
            radius: EARTH.radius * g,
            mu: EARTH.mu * g.powi(3),
            omega: EARTH.omega,
            air_scale: g,
        }
    }

    pub fn air(&self, z: f64) -> Air {
        air(z * self.air_scale)
    }

    pub fn circular_speed(&self, altitude: f64) -> f64 {
        (self.mu / (self.radius + altitude)).sqrt()
    }

    pub fn surface_gravity(&self) -> f64 {
        self.mu / (self.radius * self.radius)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Air {
    pub t: f64,
    pub p: f64,
    pub rho: f64,
    pub sound: f64,
    /// Pa s, Sutherland.
    pub viscosity: f64,
}

pub fn air(z: f64) -> Air {
    let (t, p) = isa(z.max(0.));
    Air {
        t,
        p,
        rho: p / (287.053 * t),
        sound: (1.4 * 287.053 * t).sqrt(),
        viscosity: 1.458e-6 * t.powf(1.5) / (t + 110.4),
    }
}

/// An inertial state in the plane of an equatorial, eastward track: radius,
/// angle, and radial and tangential velocity.
#[derive(Clone, Copy, Debug)]
pub struct State {
    pub r: f64,
    pub theta: f64,
    pub vr: f64,
    pub vt: f64,
}

impl State {
    pub fn altitude(&self, planet: &Planet) -> f64 {
        self.r - planet.radius
    }

    /// Velocity relative to the rotating air: radial, tangential.
    pub fn relative(&self, planet: &Planet) -> (f64, f64) {
        (self.vr, self.vt - planet.omega * self.r)
    }

    pub fn airspeed(&self, planet: &Planet) -> f64 {
        let (a, b) = self.relative(planet);
        a.hypot(b)
    }

    /// Flight-path angle relative to the air.
    pub fn gamma(&self, planet: &Planet) -> f64 {
        let (a, b) = self.relative(planet);
        a.atan2(b)
    }

    /// Apoapsis and periapsis radii of the osculating orbit.
    pub fn apsides(&self, planet: &Planet) -> (f64, f64) {
        let v2 = self.vr * self.vr + self.vt * self.vt;
        let energy = v2 / 2. - planet.mu / self.r;
        let h = self.r * self.vt;
        let a = -planet.mu / (2. * energy);
        let e = (1. + 2. * energy * h * h / (planet.mu * planet.mu)).max(0.).sqrt();
        if energy >= 0. {
            (f64::INFINITY, a * (1. - e))
        } else {
            (a * (1. + e), a * (1. - e))
        }
    }

    /// Advance by `dt` under acceleration `acc(state) -> (radial, tangential)`
    /// besides gravity, fourth-order Runge-Kutta.
    pub fn step(
        self,
        planet: &Planet,
        dt: f64,
        acc: impl Fn(&State) -> (f64, f64),
    ) -> State {
        let deriv = |s: &State| -> [f64; 4] {
            let (ar, at) = acc(s);
            [
                s.vr,
                s.vt / s.r,
                s.vt * s.vt / s.r - planet.mu / (s.r * s.r) + ar,
                -s.vr * s.vt / s.r + at,
            ]
        };
        let add = |s: &State, d: &[f64; 4], h: f64| State {
            r: s.r + d[0] * h,
            theta: s.theta + d[1] * h,
            vr: s.vr + d[2] * h,
            vt: s.vt + d[3] * h,
        };
        let k1 = deriv(&self);
        let k2 = deriv(&add(&self, &k1, dt / 2.));
        let k3 = deriv(&add(&self, &k2, dt / 2.));
        let k4 = deriv(&add(&self, &k3, dt));
        let mut d = [0.; 4];
        for i in 0..4 {
            d[i] = (k1[i] + 2. * k2[i] + 2. * k3[i] + k4[i]) / 6.;
        }
        add(&self, &d, dt)
    }
}
