use std::sync::{Arc, RwLock};

use bluedroid::gatt_server::{Characteristic, Service};
use bluedroid::utilities::{AttributePermissions, BleUuid, CharacteristicProperties};

pub struct ResultsServiceContainer {
    pub(crate) service: Arc<RwLock<Service>>,
    pub(crate) blood_oxygen_saturation_characteristic: Arc<RwLock<Characteristic>>,
    pub(crate) heart_rate_characteristic: Arc<RwLock<Characteristic>>,
    pub(crate) led1_perfusion_index_characteristic: Arc<RwLock<Characteristic>>,
    pub(crate) led2_perfusion_index_characteristic: Arc<RwLock<Characteristic>>,
    pub(crate) led3_perfusion_index_characteristic: Arc<RwLock<Characteristic>>,
    pub(crate) wrist_presence_characteristic: Arc<RwLock<Characteristic>>,
}

impl ResultsServiceContainer {
    pub(crate) fn initialise() -> Self {
        let characteristic_list: [(&str, &str, u16); 6] = [
            (
                "0776731C-A5F8-4B40-9500-E4F97F5958D9",
                "Blood oxygen saturation",
                4,
            ),
            ("D8CE0238-F60C-4C1D-908F-5554760AA1D6", "Heart rate", 4),
            (
                "459CAB03-5240-4837-9742-B71A5D8112A3",
                "LED1 perfusion index",
                4,
            ),
            (
                "32D616C9-5721-4BF0-B5F3-B709C45225EE",
                "LED2 perfusion index",
                4,
            ),
            (
                "C11839D6-50E7-4210-AD45-E44C5AB085AC",
                "LED3 perfusion index",
                4,
            ),
            ("9439189D-C1C2-4970-BD64-B9F1932F159F", "Wrist presence", 1),
        ];

        let mut characteristics: Vec<Arc<RwLock<Characteristic>>> = vec![];

        let mut service = Service::new(BleUuid::from_uuid128_string(
            "5BE2E901-D0EC-4A5F-9488-3C80CE223852",
        ))
        .name("Results")
        .primary()
        .clone();

        for item in characteristic_list {
            let characteristic = Characteristic::new(BleUuid::from_uuid128_string(item.0))
                .name(item.1)
                .show_name()
                .permissions(AttributePermissions::new().read())
                .properties(CharacteristicProperties::new().read().notify())
                .max_value_length(item.2)
                .build();

            service.characteristic(&characteristic);
            characteristics.push(characteristic);
        }

        let service = service.build();

        Self {
            service,
            blood_oxygen_saturation_characteristic: characteristics[0].clone(),
            heart_rate_characteristic: characteristics[1].clone(),
            led1_perfusion_index_characteristic: characteristics[2].clone(),
            led2_perfusion_index_characteristic: characteristics[3].clone(),
            led3_perfusion_index_characteristic: characteristics[4].clone(),
            wrist_presence_characteristic: characteristics[5].clone(),
        }
    }
}
