//! The fighter as a flying machine: an Earth-like planet, its aerodynamics
//! and entry heating, the climb to orbit, entry from it, and short takeoff
//! and landing on a dirt strip. The engine is `hf-engine`'s. See
//! book/small-craft.md.

pub mod aero;
pub mod ascent;
pub mod entry;
pub mod fill;
pub mod heat;
pub mod optim;
pub mod planet;
pub mod strip;
