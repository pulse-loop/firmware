use std::{
    sync::{Arc, Mutex, RwLock},
    thread,
    time::Duration,
};

use uom::si::f32::ElectricPotential;

/// This funtion should be called in a separate thread to send the readings from the AFE4404.
pub fn notify_task(
    ble_api: Arc<RwLock<crate::bluetooth::BluetoothAPI>>,
    readings: Arc<Mutex<[ElectricPotential; 5]>>,
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

                let aggregated_data: [u8; 100] = [0b11010000 ; 100];
                ble_api.raw_sensor_data.aggregated_data_characteristic.write().unwrap().set_value(aggregated_data);

                time = std::time::Instant::now();
            }
        }
    }
}
