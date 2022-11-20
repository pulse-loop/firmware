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

use std::{thread, time::Duration};

// If using the `binstart` feature of `esp-idf-sys`, always keep this module imported.
use esp_idf_sys::{self as _, esp_get_free_heap_size, esp_get_free_internal_heap_size};

#[path = "../bluetooth/mod.rs"]
mod bluetooth;

fn main() {
    // Temporary. Will disappear once ESP-IDF 4.4 is released, but for now it is necessary to call this function once,
    // or else some patches to the runtime implemented by esp-idf-sys might not link properly.
    esp_idf_sys::link_patches();

    // Initialise logger.
    esp_idf_svc::log::EspLogger::initialize_default();
    log::info!("Logger initialised.");

    let ble_api = bluetooth::initialise();

    // Fake data generation.
    #[allow(clippy::cast_possible_truncation, clippy::cast_precision_loss)]
    std::thread::spawn(move || {
        loop {
            std::thread::sleep(std::time::Duration::from_millis(100));
            let now = std::time::SystemTime::now();
            let milliseconds: f32 = (now
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis() % 1_000_000_000) as f32;
            let seconds = milliseconds / 1000.0;

            // Sine wave
            let ambient_value = seconds.sin().mul_add(100.0, 100.0);
            let led1_minus_ambient_value = ambient_value + 50.0;
            let led1_value = seconds.cos().mul_add(100.0, 100.0);
            let led2_value = seconds % 10.0;
            let led3_value = (seconds * 3.0).sin() * led2_value;

            ble_api
                .raw_sensor_data
                .ambient_reading_characteristic
                .write()
                .unwrap()
                .set_value(ambient_value.to_le_bytes());
            ble_api
                .raw_sensor_data
                .led1_minus_ambient_reading_characteristic
                .write()
                .unwrap()
                .set_value(led1_minus_ambient_value.to_le_bytes());
            ble_api
                .raw_sensor_data
                .led1_reading_characteristic
                .write()
                .unwrap()
                .set_value(led1_value.to_le_bytes());
            ble_api
                .raw_sensor_data
                .led2_reading_characteristic
                .write()
                .unwrap()
                .set_value(led2_value.to_le_bytes());
            ble_api
                .raw_sensor_data
                .led3_reading_characteristic
                .write()
                .unwrap()
                .set_value(led3_value.to_le_bytes());
            std::thread::yield_now();
        }
    });

    thread::spawn(|| loop {
        thread::sleep(Duration::from_millis(500));

        unsafe {
            let x = esp_get_free_heap_size();
            let y = esp_get_free_internal_heap_size();
            log::info!("Free heap: {} bytes, free internal heap: {} bytes", x, y);
        }
    });
}
