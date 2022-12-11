use std::sync::{Arc, Mutex};

use afe4404::{device::AFE4404, modes::ThreeLedsMode};
use esp_idf_hal::i2c::I2cDriver;
use log::info;
use uom::si::{f32::ElectricCurrent, electric_current::milliampere};

pub(crate) fn attach_optical_frontend_chars(frontend: &'static Arc<Mutex<Option<AFE4404<I2cDriver, ThreeLedsMode>>>>, ble_api: &mut crate::bluetooth::BluetoothAPI) {
    ble_api.optical_frontend_configuration.led1_lighting_start_characteristic.write().unwrap().on_write(move |value, _| {
        let mut slice: [u8; 4] = [0; 4];
        slice.copy_from_slice(&value[..4]);
        let value = f32::from_le_bytes(slice);

        info!("Setting LED1 lighting start to {} mA", value);

        let result = frontend.lock().unwrap().as_mut().unwrap().set_led1_current(ElectricCurrent::new::<milliampere>(value));

        match result {
            Ok(result) => {
                info!("LED1 lighting start set to {:?}", result);
            },
            Err(e) => {
                info!("Error setting LED1 lighting start: {:?}", e);
            },
        }
    });

    ble_api.optical_frontend_configuration.led1_lighting_start_characteristic.write().unwrap().on_read(move |_| {
        let result = frontend.lock().unwrap().as_mut().unwrap().get_led1_current();

        match result {
            Ok(result) => {
                info!("LED1 lighting start is {:?}", result);
                result.value.to_le_bytes().to_vec()
            },
            Err(e) => {
                info!("Error getting LED1 lighting start: {:?}", e);
                vec![]
            },
        }
    });
}