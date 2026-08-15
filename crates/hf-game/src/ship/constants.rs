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
pub const RATED_FUSION_W: f64 = 100.0e9;
/// Reactor rating: direct-converted electrical output at the redline [W].
pub const RATED_ELECTRICAL_W: f64 = 63.0e9;
/// Reactor rating: waste heat to the tin loop at the redline [W].
pub const RATED_WASTE_HEAT_W: f64 = 37.0e9;

/// Cruise electrical demand — hab, cryo, compute, sensors. The reactor is
/// demand-driven and idles here; 100 GW fusion is combat/FEL redline.
pub const CRUISE_DEMAND_W: f64 = 6.0e9;

/// Soft ceiling the auto-controller drives toward at full thermal headroom.
/// 1.4× rated so a zoned-down radiator can saturate before the cap binds.
pub const P_REACTOR_CEILING: f64 = 140.0e9;

/// Liquid tin: melting point [K].
pub const TIN_LIQUIDUS_K: f64 = 505.0;
/// Liquid tin: boiling point [K].
pub const TIN_BOILING_K: f64 = 2875.0;
/// Tin loop design temperature at rated load, all zones lit [K].
pub const TIN_DESIGN_K: f64 = 1200.0;

/// Total aft radiating area, all zones live [m^2].
pub const RADIATOR_AREA_M2: f64 = 500_000.0;
/// Independently valveable radiator zones (aft faces + vanes).
pub const N_RADIATOR_ZONES: usize = 8;
/// Radiator emissivity.
pub const RADIATOR_EMISSIVITY: f64 = 0.9;

/// Radiative rejection capacity of `n_live` zones at temperature `t_k` [W].
pub fn radiator_capacity_w(t_k: f64, n_live: u8) -> f64 {
    let frac = n_live as f64 / N_RADIATOR_ZONES as f64;
    RADIATOR_EMISSIVITY * SIGMA_SB * RADIATOR_AREA_M2 * frac * t_k.powi(4)
}

/// Aneutronic CEILING on He3 consumption: attributes *all* fusion power to the
/// D-He3 channel (the book's ~5.4 t/yr figure at 100 GW). The true burn is
/// lower because D-D side reactions carry some of the power; for fuel
/// accounting publish the reactor's actual `R_DHe3 * HE3_MASS_KG` instead.
pub fn he3_burn_ceiling_kg_s(fusion_w: f64) -> f64 {
    (fusion_w / Q_DHE3_J) * HE3_MASS_KG
}
