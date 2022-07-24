//! This module is a container for various Bluetooth LE services and utilities.

macro_rules! leaky_box_raw {
    ($val:expr) => {
        Box::into_raw(Box::new($val))
    };
}

macro_rules! leaky_box_u8 {
    ($val:expr) => {
        leaky_box_raw!($val) as *mut u8
    };
}

macro_rules! leaky_box_be_bytes {
    ($val:expr) => {
        leaky_box_u8!($val.to_be_bytes())
    };
}

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
