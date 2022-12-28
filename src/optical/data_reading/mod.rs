use std::sync::atomic::AtomicBool;

use afe4404::{device::AFE4404, modes::ThreeLedsMode, value_reading::Readings};

pub static DATA_READY: AtomicBool = AtomicBool::new(false);

pub fn get_sample_blocking<I2C>(
    frontend: &mut AFE4404<I2C, ThreeLedsMode>,
    max_iterations: u8,
) -> Result<Readings<ThreeLedsMode>, ()>
where
    I2C: embedded_hal::i2c::I2c,
{
    let mut i = 0;
    while i < max_iterations {
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
