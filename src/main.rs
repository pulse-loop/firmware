use std::time::Duration;
use embedded_hal::delay::blocking::DelayUs;

use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use esp_idf_hal::{self, rmt::{Pulse, PinState, FixedLengthSignal}, delay::Ets};
use esp_idf_hal::rmt::Signal;

fn main() {
    // Temporary. Will disappear once ESP-IDF 4.4 is released, but for now it is necessary to call this function once,
    // or else some patches to the runtime implemented by esp-idf-sys might not link properly.
    esp_idf_sys::link_patches();

    println!("Hello, world!");

    let dp = esp_idf_hal::peripherals::Peripherals::take().expect("Cannot take ESP32 peripherals.");
    let gpio8 = dp.pins.gpio8.into_output().expect("Cannot put GPIO8 into output mode.");

    println!("GPIO conf complete!");

    let rmt_channel = dp.rmt.channel0;
    let rmt_config = esp_idf_hal::rmt::config::TransmitConfig::new().clock_divider(1);
    let mut rmt_tx = esp_idf_hal::rmt::Transmit::new(gpio8, rmt_channel, &rmt_config).expect("Cannot configure RMT transmitter.");

    println!("RMT conf complete!");

    let rgbs = [0xff0000, 0xffff00, 0x00ffff, 0x00ff00, 0xa000ff];
    loop {
        for rgb in rgbs {
            let ticks_hz = rmt_tx.counter_clock().expect("Cannot get RMT counter clock.");

            println!("Ticker is at {freq}.", freq = ticks_hz);

            let t0h = Pulse::new_with_duration(ticks_hz, PinState::High, &Duration::from_nanos(350)).unwrap();
            let t0l = Pulse::new_with_duration(ticks_hz, PinState::Low, &Duration::from_nanos(800)).unwrap();
            let t1h = Pulse::new_with_duration(ticks_hz, PinState::High, &Duration::from_nanos(700)).unwrap();
            let t1l = Pulse::new_with_duration(ticks_hz, PinState::Low, &Duration::from_nanos(600)).unwrap();

            let mut signal = FixedLengthSignal::<24>::new();
            for i in 0..24 {
                let bit = 2_u32.pow(i) & rgb != 0;
                let (high_pulse, low_pulse) = if bit { (t1h, t1l) } else { (t0h, t0l) };
                signal.set(i as usize, &(high_pulse, low_pulse)).unwrap();
            }

            rmt_tx.start_blocking(&signal).expect("Cannot send RMT signal");
            Ets.delay_ms(1000).unwrap();
        }
    }
}
