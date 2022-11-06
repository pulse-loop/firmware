#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::cargo)]
#![allow(clippy::multiple_crate_versions)]
// Documentation lints
// #![warn(missing_docs)]
// #![warn(clippy::missing_docs_in_private_items)]
#![warn(rustdoc::all)]

//! Firmware for the pulse.loop wrist pulse oximeter.
//!
//! This is the main file of the firmware.

extern crate core;

use esp_idf_hal::can::config::Filter;
use std::borrow::Borrow;
use std::{thread, time::Duration};

// If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use esp_idf_sys as _;

mod bluetooth;

fn main() {
    // Temporary. Will disappear once ESP-IDF 4.4 is released, but for now it is necessary to call this function once,
    // or else some patches to the runtime implemented by esp-idf-sys might not link properly.
    esp_idf_sys::link_patches();

    // Initialise logger.
    esp_idf_svc::log::EspLogger::initialize_default();
    log::info!("Logger initialised.");

    thread::spawn(|| loop {
        thread::sleep(Duration::from_millis(500));
    });
}
