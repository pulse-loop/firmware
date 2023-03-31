use std::sync::{Arc, RwLock};

use bluedroid::gatt_server::{Characteristic, Service};
use bluedroid::utilities::{AttributePermissions, BleUuid, CharacteristicProperties};

pub struct SensorDataServiceContainer {
    pub(crate) service: Arc<RwLock<Service>>,
    pub(crate) raw_optical_data_characteristic: Arc<RwLock<Characteristic>>,
    pub(crate) filtered_optical_data_characteristic: Arc<RwLock<Characteristic>>,
}

impl SensorDataServiceContainer {
    pub(crate) fn initialise() -> Self {
        let raw_optical_data_characteristic = Characteristic::new(BleUuid::from_uuid128_string(
            "26CB3CCA-F22E-4179-8125-55874E9153AD",
        ))
        .name("Raw optical data")
        .show_name()
        .permissions(AttributePermissions::new().read())
        .properties(CharacteristicProperties::new().read().notify())
        .max_value_length(16)
        .build();

        let filtered_optical_data_characteristic = Characteristic::new(BleUuid::from_uuid128_string(
            "BDC0FC52-797B-4065-AABA-DC394F1DD0FD",
        ))
        .name("Filtered optical data")
        .show_name()
        .permissions(AttributePermissions::new().read())
        .properties(CharacteristicProperties::new().read().notify())
        .max_value_length(24)
        .build();

        let service = Service::new(BleUuid::from_uuid128_string(
            "272DF1F7-9D28-4B8C-86F6-30DB30ACE42C",
        ))
        .name("Sensor data")
        .primary()
        .characteristic(&raw_optical_data_characteristic)
        .characteristic(&filtered_optical_data_characteristic)
        .build();

        Self {
            service,
            raw_optical_data_characteristic,
            filtered_optical_data_characteristic,
        }
    }
}
