#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::cargo)]
#![allow(clippy::multiple_crate_versions)]
// Documentation lints
#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]
#![warn(invalid_doc_attributes)]
#![warn(rustdoc::all)]

//! BlueDroid support crate for ESP32.
//!
//! This module abstracts the unsafe Bluetooth LE API in a safe and rusty way.

// Initialisation
mod initialisation;
pub use initialisation::initialise;

// Configuration table
mod configuration;
pub use configuration::Configuration;

// Handlers
mod gap_event_handler;
mod gatts_event_handler;

use lazy_static::lazy_static;
use std::sync::Mutex;

// Configuration storage
lazy_static! {
    /// The configuration storage.
    static ref CONFIG: Mutex<Option<Configuration >> = Mutex::new(None);
}
