use std::{
    sync::{Arc, Mutex, RwLock},
    thread,
    time::Duration,
};

/// This struct contains all the data that is sent via notifications.
/// All the voltages are expressed in microvolts.
#[derive(Debug, Default, Clone, Copy)]
pub struct AggregatedData {
    // TODO: Remove unused fields.
    pub(crate) ambient_reading: i32,
    pub(crate) led1_reading: i32,
    pub(crate) led2_reading: i32,
    pub(crate) led3_reading: i32,
    pub(crate) ambient_lower_threshold: i32,
    pub(crate) ambient_upper_threshold: i32,
    pub(crate) led1_lower_threshold: i32,
    pub(crate) led1_upper_threshold: i32,
    pub(crate) led2_lower_threshold: i32,
    pub(crate) led2_upper_threshold: i32,
    pub(crate) led3_lower_threshold: i32,
    pub(crate) led3_upper_threshold: i32,
    pub(crate) led1_critical_value: super::signal_processing::CriticalValue,
    pub(crate) led2_critical_value: super::signal_processing::CriticalValue,
    pub(crate) led3_critical_value: super::signal_processing::CriticalValue,
}

impl AggregatedData {
    pub fn serialise(&self) -> [u8; 49] {
        let mut data = [0; 49];

        data[0..4].copy_from_slice(&self.ambient_reading.to_le_bytes());
        data[4..8].copy_from_slice(&self.led1_reading.to_le_bytes());
        data[8..12].copy_from_slice(&self.led2_reading.to_le_bytes());
        data[12..16].copy_from_slice(&self.led3_reading.to_le_bytes());
        data[16..20].copy_from_slice(&self.ambient_lower_threshold.to_le_bytes());
        data[20..24].copy_from_slice(&self.ambient_upper_threshold.to_le_bytes());
        data[24..28].copy_from_slice(&self.led1_lower_threshold.to_le_bytes());
        data[28..32].copy_from_slice(&self.led1_upper_threshold.to_le_bytes());
        data[32..36].copy_from_slice(&self.led2_lower_threshold.to_le_bytes());
        data[36..40].copy_from_slice(&self.led2_upper_threshold.to_le_bytes());
        data[40..44].copy_from_slice(&self.led3_lower_threshold.to_le_bytes());
        data[44..48].copy_from_slice(&self.led3_upper_threshold.to_le_bytes());
        data[48] = (self.led1_critical_value.into_bits() << 4)
            | (self.led2_critical_value.into_bits() << 2)
            | (self.led3_critical_value.into_bits());

        data
    }
}

/// This funtion should be called in a separate thread to send the readings from the AFE4404.
pub fn notify_task(
    ble_api: &crate::bluetooth::BluetoothAPI,
    raw_data: Arc<Mutex<RawData>>,
    filtered_data: Arc<Mutex<FilteredData>>,
    calibration_receiver: std::sync::mpsc::Receiver<(f32, f32)>,
) {
    let mut notify_timer = super::timer::Timer::new(50);
    let mut latest_calibration_data: Option<(f32, f32)> = None;
    loop {
        latest_calibration_data = calibration_receiver.try_recv().map(Some).unwrap_or(None);

        if notify_timer.is_expired() {
            if let (Ok(raw_data), Ok(filtered_data)) = (raw_data.lock(), filtered_data.lock()) {
                thread::sleep(Duration::from_millis(5));

                ble_api
                    .raw_sensor_data
                    .aggregated_data_characteristic
                    .write()
                    .unwrap()
                    .set_value(raw_data.serialise());

                thread::sleep(Duration::from_millis(5));

                ble_api
                    .sensor_data
                    .filtered_optical_data_characteristic
                    .write()
                    .unwrap()
                    .set_value(filtered_data.serialise());

                // #[cfg(feature = "notify-calibration")]
                // if let Some(latest_calibration_data) = latest_calibration_data {
                //     thread::sleep(Duration::from_millis(5));

                //     ble_api
                //         .optical_frontend_configuration
                //         .led1_current_characteristic
                //         .write()
                //         .unwrap()
                //         .set_value(latest_calibration_data.0.to_le_bytes());

                //     thread::sleep(Duration::from_millis(5));

                //     ble_api
                //         .optical_frontend_configuration
                //         .led1_offset_current_characteristic
                //         .write()
                //         .unwrap()
                //         .set_value(latest_calibration_data.1.to_le_bytes());
                // }
                notify_timer.reset();
            }
        }
    }
}
