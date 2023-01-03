use std::{
    sync::{Arc, Mutex, RwLock},
    thread,
    time::Duration,
};

use uom::si::{electric_potential::volt, f32::ElectricPotential};

/// This funtion should be called in a separate thread to send the averaged readings from the AFE4404.
pub fn notify_with_averaged_readings_loop(
    ble_api: Arc<RwLock<crate::bluetooth::BluetoothAPI>>,
    averaged_readings: Arc<Mutex<[ElectricPotential; 5]>>,
    n: Arc<Mutex<u32>>,
) {
    let mut time = std::time::Instant::now();
    loop {
        thread::sleep(Duration::from_millis(10));

        if time.elapsed().as_millis() > 50 && *n.lock().unwrap() > 0 {
            if let (Ok(ble_api), Ok(mut n), Ok(mut averaged_readings)) =
                (ble_api.write(), n.lock(), averaged_readings.lock())
            {
                ble_api
                    .raw_sensor_data
                    .ambient_reading_characteristic
                    .write()
                    .unwrap()
                    .set_value((averaged_readings[0] / (*n as f32)).value.to_le_bytes());
                ble_api
                    .raw_sensor_data
                    .led1_minus_ambient_reading_characteristic
                    .write()
                    .unwrap()
                    .set_value((averaged_readings[1] / (*n as f32)).value.to_le_bytes());
                ble_api
                    .raw_sensor_data
                    .led1_reading_characteristic
                    .write()
                    .unwrap()
                    .set_value((averaged_readings[2] / (*n as f32)).value.to_le_bytes());
                ble_api
                    .raw_sensor_data
                    .led2_reading_characteristic
                    .write()
                    .unwrap()
                    .set_value((averaged_readings[3] / (*n as f32)).value.to_le_bytes());
                ble_api
                    .raw_sensor_data
                    .led3_reading_characteristic
                    .write()
                    .unwrap()
                    .set_value((averaged_readings[4] / (*n as f32)).value.to_le_bytes());

                *averaged_readings = [ElectricPotential::new::<volt>(0.0); 5];
                *n = 0;
                time = std::time::Instant::now();
            }
        }
    }
}
