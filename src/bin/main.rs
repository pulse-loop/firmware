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

//! Firmware for the pulse.loop wrist pulse oximeter.
//!
//! This is the main file of the firmware.

use std::{
    thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

// If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use esp_idf_sys as _;
use smart_leds::{
    brightness, gamma,
    hsv::{hsv2rgb, Hsv},
    SmartLedsWrite,
};
use ws2812_esp32_rmt_driver::Ws2812Esp32Rmt;

use bluedroid;

mod bluetooth_services;

fn main() {
    // Temporary. Will disappear once ESP-IDF 4.4 is released, but for now it is necessary to call this function once,
    // or else some patches to the runtime implemented by esp-idf-sys might not link properly.
    esp_idf_sys::link_patches();

    // TODO: Use ESP-IDF logging!
    println!("Hello, world!");

    let mut led = Ws2812Esp32Rmt::new(0, 8).expect("Cannot get RGB led interface over RMT.");

    bluedroid::initialise();

    thread::spawn(move || loop {
        let now = SystemTime::now();
        let time_since_epoch = now.duration_since(UNIX_EPOCH).expect("Time went backwards");
        let millis = time_since_epoch.as_millis();

        let color = [hsv2rgb(Hsv {
            hue: ((millis / 4) % 255) as u8,
            sat: 255,
            val: 255,
        })];
        led.write(gamma(brightness(color.iter().copied(), 50)))
            .expect("Cannot write data to LED.");
        thread::sleep(Duration::from_millis(10));
    });
}
