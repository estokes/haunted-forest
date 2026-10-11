//! Beamed-core antimatter nozzle model: coil fields, charged-pion tracking
//! with decays, and a parametric design scored for thrust, leaks and magnet
//! mass, and the cosmic-ray shelter the ring's field gives the pods. See
//! book/courier.md.

pub mod design;
pub mod field;
pub mod shelter;
pub mod track;

pub use design::{Report, Spec};
pub use shelter::{ShelterReport, ShelterSpec};
