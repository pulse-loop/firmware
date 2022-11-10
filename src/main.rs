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

    let ble_api = bluetooth::initialise();

    std::thread::spawn(move || {
        loop {
            std::thread::sleep(std::time::Duration::from_millis(10));
            let now = std::time::SystemTime::now();
            let milliseconds: f64 = ((now.duration_since(std::time::UNIX_EPOCH).unwrap().as_millis() % u32::MAX as u128) as u32).into();
            let seconds = milliseconds / 1000.0;
            
            // Sine wave
            let ambient_value = seconds.sin() * 100.0;
            let led1_minus_ambient_value = ambient_value + 50.0;
            let led1_value = seconds.cos() * 100.0;
            let led2_value = seconds % 100.0;
            let led3_value = led1_value - led2_value;

            let ambient_value = ambient_value.trunc() as i32;
            let led1_minus_ambient_value = led1_minus_ambient_value.trunc() as i32;
            let led1_value = led1_value.trunc() as i32;
            let led2_value = led2_value.trunc() as i32;
            let led3_value = led3_value.trunc() as i32;

            ble_api.raw_sensor_data.ambient_reading_characteristic.write().unwrap().set_value(ambient_value.to_le_bytes());
            ble_api.raw_sensor_data.led1_minus_ambient_reading_characteristic.write().unwrap().set_value(led1_minus_ambient_value.to_le_bytes());
            ble_api.raw_sensor_data.led1_reading_characteristic.write().unwrap().set_value(led1_value.to_le_bytes());
            ble_api.raw_sensor_data.led2_reading_characteristic.write().unwrap().set_value(led2_value.to_le_bytes());
            ble_api.raw_sensor_data.led3_reading_characteristic.write().unwrap().set_value(led3_value.to_le_bytes());
            std::thread::yield_now();
        }
    });

    thread::spawn(|| loop {
        thread::sleep(Duration::from_millis(500));
    });
}
