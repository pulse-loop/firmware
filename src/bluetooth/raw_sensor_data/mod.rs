use std::sync::{Arc, RwLock};

use bluedroid::gatt_server::{Characteristic, Service};
use bluedroid::utilities::{AttributePermissions, BleUuid, CharacteristicProperties};

pub struct RawSensorDataServiceContainer {
    pub(crate) service: Arc<RwLock<Service>>,
    pub(crate) ambient_reading_characteristic: Arc<RwLock<Characteristic>>,
    pub(crate) led1_minus_ambient_reading_characteristic: Arc<RwLock<Characteristic>>,
    pub(crate) led1_reading_characteristic: Arc<RwLock<Characteristic>>,
    pub(crate) led2_reading_characteristic: Arc<RwLock<Characteristic>>,
    pub(crate) led3_reading_characteristic: Arc<RwLock<Characteristic>>,
}

impl RawSensorDataServiceContainer {
    pub(crate) fn initialise() -> Self {
        let ambient_reading_characteristic = Characteristic::new(BleUuid::from_uuid128_string(
            "33EAF25F-7A5C-4327-A95B-B602DA54C443",
        ))
        .name("Ambient reading")
        .show_name()
        .permissions(AttributePermissions::new().read())
        .properties(CharacteristicProperties::new().read().notify())
        .max_value_length(4)
        .build();

        let led1_minus_ambient_reading_characteristic = Characteristic::new(
            BleUuid::from_uuid128_string("CF66D344-584D-4E67-AC30-17D28B099A30"),
        )
        .name("LED1 - ambient reading")
        .show_name()
        .permissions(AttributePermissions::new().read())
        .properties(CharacteristicProperties::new().read().notify())
        .max_value_length(4)
        .build();

        let led1_reading_characteristic = Characteristic::new(BleUuid::from_uuid128_string(
            "05500B81-516D-4BD9-95BA-C0B87C911DDB",
        ))
        .name("LED1 reading")
        .show_name()
        .permissions(AttributePermissions::new().read())
        .properties(CharacteristicProperties::new().read().notify())
        .max_value_length(4)
        .build();

        let led2_reading_characteristic = Characteristic::new(BleUuid::from_uuid128_string(
            "A93B639D-8A8D-43EA-8A5A-8175D7C09E0B",
        ))
        .name("LED2 reading")
        .show_name()
        .permissions(AttributePermissions::new().read())
        .properties(CharacteristicProperties::new().read().notify())
        .max_value_length(4)
        .build();

        let led3_reading_characteristic = Characteristic::new(BleUuid::from_uuid128_string(
            "C0A12246-79E4-4BD7-8A4F-B841D5590F70",
        ))
        .name("LED3 reading")
        .show_name()
        .permissions(AttributePermissions::new().read())
        .properties(CharacteristicProperties::new().read().notify())
        .max_value_length(4)
        .build();

        let service = Service::new(BleUuid::from_uuid128_string(
            "272DF1F7-9D28-4B8C-86F6-30DB30ACE42C",
        ))
        .name("Raw sensor data")
        .primary()
        .characteristic(&led1_reading_characteristic)
        .characteristic(&ambient_reading_characteristic)
        .characteristic(&led1_minus_ambient_reading_characteristic)
        .characteristic(&led2_reading_characteristic)
        .characteristic(&led3_reading_characteristic)
        .build();

        Self {
            service,
            ambient_reading_characteristic,
            led1_minus_ambient_reading_characteristic,
            led1_reading_characteristic,
            led2_reading_characteristic,
            led3_reading_characteristic,
        }
    }
}
