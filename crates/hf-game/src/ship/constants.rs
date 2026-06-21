//! Shared physical constants and reconciled rated values for the reactor and
//! thermal systems. Numbers reconcile to book/ship-systems.md. Plasma-specific
//! constants (Bosch-Hale reactivity tables, bremsstrahlung, ion masses) live in
//! reactor.rs; these are the values used by more than one system.

/// Stefan-Boltzmann constant [W/m^2/K^4].
pub const SIGMA_SB: f64 = 5.670374419e-8;
/// Electron volt [J].
pub const EV_J: f64 = 1.602176634e-19;
/// Atomic mass unit [kg].
pub const ATOMIC_MASS_KG: f64 = 1.66053907e-27;
/// Seconds in a Julian year.
pub const SECONDS_PER_YEAR: f64 = 3.15576e7;

/// Energy released per D-He3 reaction (18.35 MeV, all charged) [J].
pub const Q_DHE3_J: f64 = 18.35e6 * EV_J;
/// Mass of a helium-3 nucleus [kg].
pub const HE3_MASS_KG: f64 = 3.016 * ATOMIC_MASS_KG;

/// Reactor rating: fusion power at the redline operating point [W].
pub const RATED_FUSION_W: f64 = 500.0e9;
/// Reactor rating: direct-converted electrical output at the redline [W].
pub const RATED_ELECTRICAL_W: f64 = 315.0e9;
/// Reactor rating: waste heat to the tin loop at the redline [W].
pub const RATED_WASTE_HEAT_W: f64 = 185.0e9;

/// Cruise electrical demand — the ship loafing along on antimatter containment,
/// life support, compute, and sensors. The reactor is demand-driven and idles
/// here; 500 GW is the redline it reaches only under combat or AI surge load.
pub const CRUISE_DEMAND_W: f64 = 18.0e9;

/// Soft ceiling the auto-controller drives toward at full thermal headroom.
/// Raised from 540 GW so the thermal cascade engages (warn band near 649 GW
/// fusion) before the reactor's own cap binds.
pub const P_REACTOR_CEILING: f64 = 700.0e9;

/// Liquid tin: melting point [K].
pub const TIN_LIQUIDUS_K: f64 = 505.0;
/// Liquid tin: boiling point [K].
pub const TIN_BOILING_K: f64 = 2875.0;
/// Tin loop design temperature [K].
pub const TIN_DESIGN_K: f64 = 1800.0;

/// Area of one endcap radiator panel [m^2].
pub const RADIATOR_AREA_M2: f64 = 283_000.0;
/// Radiator emissivity.
pub const RADIATOR_EMISSIVITY: f64 = 0.9;

/// Radiative rejection capacity of `n` active endcaps at temperature `t_k` [W].
pub fn radiator_capacity_w(t_k: f64, n: u8) -> f64 {
    RADIATOR_EMISSIVITY * SIGMA_SB * RADIATOR_AREA_M2 * t_k.powi(4) * n as f64
}

/// Aneutronic CEILING on He3 consumption: attributes *all* fusion power to the
/// D-He3 channel (the book's ~27 t/yr figure). The true burn is lower because
/// D-D side reactions carry some of the power; for fuel accounting publish the
/// reactor's actual `R_DHe3 * HE3_MASS_KG` instead of this.
pub fn he3_burn_ceiling_kg_s(fusion_w: f64) -> f64 {
    (fusion_w / Q_DHE3_J) * HE3_MASS_KG
}
