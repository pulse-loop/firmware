use std::{
    sync::atomic::AtomicBool,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use afe4404::{device::AFE4404, modes::ThreeLedsMode, value_reading::Readings};

/// This is a flag that is set to true when the AFE4404 has new readings.
pub static DATA_READY: AtomicBool = AtomicBool::new(false);

/// Gets the readings from the AFE4404 and calls the completion callback with them.
/// If the readings are not ready or overlap with previous readings, the callback is not called.
fn request_readings<I2C, CB>(frontend: &mut AFE4404<I2C, ThreeLedsMode>, mut completion: CB)
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
            // DATA_READY was set to true again during frontend.read(), so the readings overlapped.
            log::warn!("Readings have overlapped.");
        }
    }
}

/// This function should be called in a separate thread to get readings from the AFE4404 and add them to the averaged readings array.
/// To calculate the average, divide each element of the array by the number of readings
pub fn reading_task(readings: Arc<Mutex<super::data_sending::AggregatedData>>) {
    loop {
        thread::sleep(Duration::from_millis(1));

        let readings = readings.clone();
        request_readings(
            super::FRONTEND.lock().unwrap().as_mut().unwrap(),
            move |readings_frontend| {
                if let Ok(mut readings) = readings.lock() {
                    readings.ambient_reading = *readings_frontend.ambient();
                    readings.led1_reading = *readings_frontend.led1();
                    readings.led2_reading = *readings_frontend.led2();
                    readings.led3_reading = *readings_frontend.led3();
                }
            },
        );
    }
}
