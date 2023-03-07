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
    ble_api: Arc<RwLock<crate::bluetooth::BluetoothAPI>>,
    readings: Arc<Mutex<AggregatedData>>,
) {
    let mut time = std::time::Instant::now();
    loop {
        thread::sleep(Duration::from_millis(10));

        if time.elapsed().as_millis() > 50 {
            if let (Ok(ble_api), Ok(mut readings)) = (ble_api.write(), readings.lock()) {
                ble_api
                    .raw_sensor_data
                    .aggregated_data_characteristic
                    .write()
                    .unwrap()
                    .set_value(readings.serialise());

                // Reset the critical values;
                readings.led1_critical_value = super::signal_processing::CriticalValue::None;
                readings.led2_critical_value = super::signal_processing::CriticalValue::None;
                readings.led3_critical_value = super::signal_processing::CriticalValue::None;

                time = std::time::Instant::now();
            }
        }
    }
}
