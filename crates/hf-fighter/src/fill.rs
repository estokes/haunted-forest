//! What a tank holds: dust shot of two sizes packed to 80%, its gaps
//! filled first by the liquid, the rest of the liquid beside it.

/// Solid dust, kg/m3.
pub const DUST_SOLID: f64 = 700.;
pub const PACKING: f64 = 0.80;

/// m3 of tank for `dust` kg of shot and `liquid` kg of a liquid of
/// density `rho`.
pub fn volume(dust: f64, liquid: f64, rho: f64) -> f64 {
    let region = dust / DUST_SOLID / PACKING;
    let gaps = region * (1. - PACKING);
    region + (liquid / rho - gaps).max(0.)
}

/// Dust and liquid together, kg, per m3 when the liquid just fills the
/// gaps.
pub fn filled_density(rho: f64) -> f64 {
    DUST_SOLID * PACKING + rho * (1. - PACKING)
}
