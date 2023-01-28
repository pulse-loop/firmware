use std::sync::{Arc, RwLock};

use bluedroid::gatt_server::{Characteristic, Service};
use bluedroid::utilities::{AttributePermissions, BleUuid, CharacteristicProperties};

pub struct RawSensorDataServiceContainer {
    pub(crate) service: Arc<RwLock<Service>>,
    pub(crate) aggregated_data_characteristic: Arc<RwLock<Characteristic>>,
}

impl RawSensorDataServiceContainer {
    pub(crate) fn initialise() -> Self {
        let aggregated_data_characteristic = Characteristic::new(BleUuid::from_uuid128_string(
            "26CB3CCA-F22E-4179-8125-55874E9153AD",
        ))
        .name("Aggregated data")
        .show_name()
        .permissions(AttributePermissions::new().read())
        .properties(CharacteristicProperties::new().read().notify())
        .max_value_length(49)
        .build();

        let service = Service::new(BleUuid::from_uuid128_string(
            "272DF1F7-9D28-4B8C-86F6-30DB30ACE42C",
        ))
        .name("Raw sensor data")
        .primary()
        .characteristic(&aggregated_data_characteristic)
        .build();

        Self {
            service,
            aggregated_data_characteristic,
        }
    }
}
