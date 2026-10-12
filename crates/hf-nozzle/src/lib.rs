//! Beamed-core antimatter nozzle model: coil fields, charged-pion tracking
//! with decays, and a parametric design scored for thrust, leaks and magnet
//! mass, the cosmic-ray shelter the ring's field gives the pods, and a
//! thin-shell model of an ICF blast in a ring's field. See book/courier.md
//! and book/icf-drive.md.

pub mod design;
pub mod field;
pub mod shell;
pub mod shelter;
pub mod track;

pub use design::{Report, Spec};
pub use shell::{ShellReport, ShellSpec};
pub use shelter::{ShelterReport, ShelterSpec};
