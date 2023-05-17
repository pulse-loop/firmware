use std::{
    sync::atomic::AtomicBool,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use afe4404::{modes::ThreeLedsMode, value_reading::Readings};

use super::data_sending::RawData;

/// This is a flag that is set to true when the AFE4404 has new readings.
pub static DATA_READY: AtomicBool = AtomicBool::new(false);

/// Gets the readings from the AFE4404 and calls the completion callback with them.
/// If the readings are not ready or overlap with previous readings, the callback is not called.
fn request_readings<CB>(mut completion: CB)
where
    CB: FnMut(Readings<ThreeLedsMode>) + 'static,
{
    if DATA_READY.load(std::sync::atomic::Ordering::Relaxed) {
        DATA_READY.store(false, std::sync::atomic::Ordering::Relaxed); // Prevent readings overlapping.
        let current_readings = super::FRONTEND.lock().unwrap().as_mut().unwrap().read();
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

/// This function should be called in a separate thread to get readings from the AFE4404.
pub fn reading_task<CB>(callback: CB)
where
    CB: FnMut(RawData) + 'static,
{
    let cb = Arc::new(Mutex::new(callback));

    loop {
        let cb = cb.clone();

        thread::sleep(Duration::from_millis(1));

        let mut data: RawData = RawData::default();

        request_readings(move |readings_frontend| {
            // Convert the readings to microvolts as integers.
            data.ambient = *readings_frontend.ambient();
            data.led1 = *readings_frontend.led1();
            data.led2 = *readings_frontend.led2();
            data.led3 = *readings_frontend.led3();

            // Call the callback.
            let mut cb = cb.lock().unwrap();
            cb(data);
        });
    }
}
