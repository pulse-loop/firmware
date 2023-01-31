use std::{
    sync::atomic::AtomicBool,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use afe4404::{device::AFE4404, modes::ThreeLedsMode, value_reading::Readings};
use uom::si::electric_potential::microvolt;

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

/// This function should be called in a separate thread to get readings from the AFE4404.
pub fn reading_task(data: Arc<Mutex<super::data_sending::AggregatedData>>) {
    let queue_size = 512;
    let critical_thresholds = (20, 80); // The lower and upper thresholds in percentage for filtering out critical values.

    let history = Arc::new(Mutex::new([
        super::signal_processing::ProcessingHistory::new(queue_size),
        super::signal_processing::ProcessingHistory::new(queue_size),
        super::signal_processing::ProcessingHistory::new(queue_size),
        super::signal_processing::ProcessingHistory::new(queue_size),
    ]));

    loop {
        thread::sleep(Duration::from_millis(1));

        let data = data.clone();
        let history = history.clone();
        request_readings(
            super::FRONTEND.lock().unwrap().as_mut().unwrap(),
            move |readings_frontend| {
                if let (Ok(mut data), Ok(mut history)) = (data.lock(), history.lock()) {
                    let ambient_converted_reading =
                        readings_frontend.ambient().get::<microvolt>().round() as i32;
                    let led1_converted_reading =
                        readings_frontend.led1().get::<microvolt>().round() as i32;
                    let led2_converted_reading =
                        readings_frontend.led2().get::<microvolt>().round() as i32;
                    let led3_converted_reading =
                        readings_frontend.led3().get::<microvolt>().round() as i32;

                    data.ambient_reading = history[0]
                        .previous_element
                        .unwrap_or(ambient_converted_reading); // The critical value refers to the previous element.
                    data.led1_reading = history[1]
                        .previous_element
                        .unwrap_or(led1_converted_reading);
                    data.led2_reading = history[2]
                        .previous_element
                        .unwrap_or(led2_converted_reading);
                    data.led3_reading = history[3]
                        .previous_element
                        .unwrap_or(led3_converted_reading);

                    super::signal_processing::find_critical_value(
                        ambient_converted_reading,
                        &mut history[0],
                    );
                    data.led1_critical_value = super::signal_processing::find_critical_value(
                        led1_converted_reading,
                        &mut history[1],
                    );
                    data.led2_critical_value = super::signal_processing::find_critical_value(
                        led2_converted_reading,
                        &mut history[2],
                    );
                    data.led3_critical_value = super::signal_processing::find_critical_value(
                        led3_converted_reading,
                        &mut history[3],
                    );

                    data.ambient_lower_threshold =
                        history[0].distribution.percentile(critical_thresholds.0);
                    data.ambient_upper_threshold = 
                        history[0].distribution.percentile(critical_thresholds.1);
                    data.led1_lower_threshold =
                        history[1].distribution.percentile(critical_thresholds.0);
                    data.led1_upper_threshold =
                        history[1].distribution.percentile(critical_thresholds.1);
                    data.led2_lower_threshold =
                        history[2].distribution.percentile(critical_thresholds.0);
                    data.led2_upper_threshold =
                        history[2].distribution.percentile(critical_thresholds.1);
                    data.led3_lower_threshold =
                        history[3].distribution.percentile(critical_thresholds.0);
                    data.led3_upper_threshold =
                        history[3].distribution.percentile(critical_thresholds.1);
                }
            },
        );
    }
}
