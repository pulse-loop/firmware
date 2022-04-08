//! Bluetooth module.
//!
//! This module abstracts the unsafe Bluetooth LE API in a safe and rusty way.
//! It is divided in application profiles.

mod initialisation;
pub use initialisation::initialise;

mod applications;
