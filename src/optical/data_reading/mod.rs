use std::sync::atomic::AtomicBool;

use afe4404::{AFE4404, afe4404::ThreeLedsMode, high_level::value_reading::Readings};


pub static DATA_READY: AtomicBool = AtomicBool::new(false);

pub fn get_sample_blocking<I2C>(
    frontend: &mut AFE4404<I2C, ThreeLedsMode>,
    iterations: u8,
) -> Result<Readings<ThreeLedsMode>, ()>
where
    I2C: embedded_hal::i2c::blocking::I2c,
{
    let mut i = 0;
    while i < iterations {
        if DATA_READY.load(std::sync::atomic::Ordering::Relaxed) {
            DATA_READY.store(false, std::sync::atomic::Ordering::Relaxed); // Prevent readings overlapping.
            let current_readings = frontend.read();
            if !DATA_READY.load(std::sync::atomic::Ordering::Relaxed) {
                if let Ok(readings) = current_readings {
                    return Ok(readings);
                }
            }
            i += 1;
        }
    }
    Err(())
}