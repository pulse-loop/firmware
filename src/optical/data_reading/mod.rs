use std::sync::{atomic::AtomicBool};

use afe4404::{device::AFE4404, modes::ThreeLedsMode, value_reading::Readings};

pub static DATA_READY: AtomicBool = AtomicBool::new(false);

pub fn get_sample<I2C, CB>(
    frontend: &mut AFE4404<I2C, ThreeLedsMode>,
    mut completion: CB
) where
    I2C: embedded_hal::i2c::I2c,
    CB: FnMut(Readings<ThreeLedsMode>) + 'static,
{
    if DATA_READY.load(std::sync::atomic::Ordering::Relaxed) {
        DATA_READY.store(false, std::sync::atomic::Ordering::Relaxed); // Prevent readings overlapping.
        let current_readings = frontend.read();
        if !DATA_READY.load(std::sync::atomic::Ordering::Relaxed) {
            if let Ok(readings) = current_readings {
                completion(readings);
            }
            else {
                log::error!("Error reading from AFE4404: {:?}", current_readings);
            }
        }
        else {
            log::warn!("Readings have overlapped.");
        }
    }
}
