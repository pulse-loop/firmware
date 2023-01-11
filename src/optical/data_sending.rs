use std::{
    sync::{Arc, Mutex, RwLock},
    thread,
    time::Duration,
};

use uom::si::{f32::ElectricPotential, electric_potential::volt};

/// This struct contains all the data that is sent via notifications.
pub struct AggregatedData {
    pub(crate) ambient_reading: ElectricPotential,
    pub(crate) led1_reading: ElectricPotential,
    pub(crate) led2_reading: ElectricPotential,
    pub(crate) led3_reading: ElectricPotential,
    pub(crate) ambient_lower_threshold: ElectricPotential,
    pub(crate) ambient_upper_threshold: ElectricPotential,
    pub(crate) led1_lower_threshold: ElectricPotential,
    pub(crate) led1_upper_threshold: ElectricPotential,
    pub(crate) led2_lower_threshold: ElectricPotential,
    pub(crate) led2_upper_threshold: ElectricPotential,
    pub(crate) led3_lower_threshold: ElectricPotential,
    pub(crate) led3_upper_threshold: ElectricPotential,
    pub(crate) led1_critical_value: super::signal_processing::CriticalValue,
    pub(crate) led2_critical_value: super::signal_processing::CriticalValue,
    pub(crate) led3_critical_value: super::signal_processing::CriticalValue,
}

impl AggregatedData {
    pub fn new() -> Self {
        Self {
            ambient_reading: ElectricPotential::new::<volt>(0.0),
            led1_reading: ElectricPotential::new::<volt>(0.0),
            led2_reading: ElectricPotential::new::<volt>(0.0),
            led3_reading: ElectricPotential::new::<volt>(0.0),
            ambient_lower_threshold: ElectricPotential::new::<volt>(0.0),
            ambient_upper_threshold: ElectricPotential::new::<volt>(0.0),
            led1_lower_threshold: ElectricPotential::new::<volt>(0.0),
            led1_upper_threshold: ElectricPotential::new::<volt>(0.0),
            led2_lower_threshold: ElectricPotential::new::<volt>(0.0),
            led2_upper_threshold: ElectricPotential::new::<volt>(0.0),
            led3_lower_threshold: ElectricPotential::new::<volt>(0.0),
            led3_upper_threshold: ElectricPotential::new::<volt>(0.0),
            led1_critical_value: super::signal_processing::CriticalValue::None,
            led2_critical_value: super::signal_processing::CriticalValue::None,
            led3_critical_value: super::signal_processing::CriticalValue::None,
        }
    }

    pub fn serialise(&self) -> [u8; 49] {
        let mut data = [0; 49];

        data[0..4].copy_from_slice(&self.ambient_reading.value.to_le_bytes());
        data[4..8].copy_from_slice(&self.led1_reading.value.to_le_bytes());
        data[8..12].copy_from_slice(&self.led2_reading.value.to_le_bytes());
        data[12..16].copy_from_slice(&self.led3_reading.value.to_le_bytes());
        data[16..20].copy_from_slice(&self.ambient_lower_threshold.value.to_le_bytes());
        data[20..24].copy_from_slice(&self.ambient_upper_threshold.value.to_le_bytes());
        data[24..28].copy_from_slice(&self.led1_lower_threshold.value.to_le_bytes());
        data[28..32].copy_from_slice(&self.led1_upper_threshold.value.to_le_bytes());
        data[32..36].copy_from_slice(&self.led2_lower_threshold.value.to_le_bytes());
        data[36..40].copy_from_slice(&self.led2_upper_threshold.value.to_le_bytes());
        data[40..44].copy_from_slice(&self.led3_lower_threshold.value.to_le_bytes());
        data[44..48].copy_from_slice(&self.led3_upper_threshold.value.to_le_bytes());
        data[48] = 0b00000000; // TODO: Add critical value conversion.

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
            if let (Ok(ble_api), Ok(readings)) = (ble_api.write(), readings.lock()) {
                // ble_api
                //     .raw_sensor_data
                //     .ambient_reading_characteristic
                //     .write()
                //     .unwrap()
                //     .set_value(readings[0].value.to_le_bytes());
                // ble_api
                //     .raw_sensor_data
                //     .led1_minus_ambient_reading_characteristic
                //     .write()
                //     .unwrap()
                //     .set_value(readings[1].value.to_le_bytes());
                // ble_api
                //     .raw_sensor_data
                //     .led1_reading_characteristic
                //     .write()
                //     .unwrap()
                //     .set_value(readings[2].value.to_le_bytes());
                // ble_api
                //     .raw_sensor_data
                //     .led2_reading_characteristic
                //     .write()
                //     .unwrap()
                //     .set_value(readings[3].value.to_le_bytes());
                // ble_api
                //     .raw_sensor_data
                //     .led3_reading_characteristic
                //     .write()
                //     .unwrap()
                //     .set_value(readings[4].value.to_le_bytes());

                ble_api
                    .raw_sensor_data
                    .aggregated_data_characteristic
                    .write()
                    .unwrap()
                    .set_value(readings.serialise());

                time = std::time::Instant::now();
            }
        }
    }
}
