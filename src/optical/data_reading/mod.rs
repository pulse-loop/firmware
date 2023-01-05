use std::{
    sync::atomic::AtomicBool,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use afe4404::{device::AFE4404, modes::ThreeLedsMode, value_reading::Readings};
use uom::si::f32::ElectricPotential;

/// This is a flag that is set to true when the AFE4404 has new readings.
pub static DATA_READY: AtomicBool = AtomicBool::new(false);

/// Gets the readings from the AFE4404 and calls the completion callback with them.
/// If the readings are not ready or overlap with previous readings, the callback is not called.
fn get_readings<I2C, CB>(frontend: &mut AFE4404<I2C, ThreeLedsMode>, mut completion: CB)
where
    I2C: embedded_hal::i2c::I2c,
    CB: FnMut(Readings<ThreeLedsMode>) + 'static,
{
    if DATA_READY.load(std::sync::atomic::Ordering::Relaxed) {
        DATA_READY.store(false, std::sync::atomic::Ordering::Relaxed); // Prevent readings overlapping.
        let current_readings = frontend.read();
        if !DATA_READY.load(std::sync::atomic::Ordering::Relaxed) {
            if let Ok(readings) = current_readings {
                completion(readings);
            } else {
                log::error!("Error reading from AFE4404: {:?}", current_readings);
            }
        } else {
            log::warn!("Readings have overlapped.");
        }
    }
}

/// This function should be called in a separate thread to get readings from the AFE4404 and add them to the averaged readings array.
/// To calculate the average, divide each element of the array by the number of readings
pub fn get_averaged_readings_loop(
    averaged_readings: Arc<Mutex<[ElectricPotential; 5]>>,
    n: Arc<Mutex<u32>>,
) {
    loop {
        thread::sleep(Duration::from_millis(1));

        let averaged_readings_for_read = averaged_readings.clone();
        let n_for_read = n.clone();
        get_readings(
            super::FRONTEND.lock().unwrap().as_mut().unwrap(),
            move |readings| {
                if let (Ok(mut n), Ok(mut averaged_readings)) =
                    (n_for_read.lock(), averaged_readings_for_read.lock())
                {
                    *n += 1;
                    averaged_readings[0] += *readings.ambient();
                    averaged_readings[1] += *readings.led1_minus_ambient();
                    averaged_readings[2] += *readings.led1();
                    averaged_readings[3] += *readings.led2();
                    averaged_readings[4] += *readings.led3();
                }
            },
        );
    }
}
