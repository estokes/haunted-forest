//! Beamed-core antimatter nozzle model: coil fields, charged-pion tracking
//! with decays, and a parametric design scored for thrust, leaks and magnet
//! mass. See book/courier.md.

pub mod design;
pub mod field;
pub mod track;

pub use design::{Report, Spec};
