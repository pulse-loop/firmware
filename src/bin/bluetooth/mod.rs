//! This module is a container for various Bluetooth LE services and utilities.

// Default configuration data.
mod configuration;

// Handlers.
mod gap_event_handler;
mod gatts_event_handler;

// Services.
pub mod services;

// Initialisation.
mod initialisation;

pub use initialisation::initialise;
