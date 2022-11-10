use std::sync::{RwLock, Arc};

use bluedroid::gatt_server::{Characteristic, Service};
use bluedroid::utilities::{BleUuid, AttributePermissions, CharacteristicProperties};

pub(crate) struct PulseLoopServiceContainer {
    pub(crate) service: Arc<RwLock<Service>>,
    pub(crate) api_version_characteristic: Arc<RwLock<Characteristic>>,
}

impl PulseLoopServiceContainer {
    pub(crate) fn initialise() -> Self {
        let api_version_characteristic = Characteristic::new(BleUuid::from_uuid128_string(
            "1852299D-AE64-4E4F-B915-CB37E7FD57C9",
        ))
        .name("API Version")
        .show_name()
        .permissions(AttributePermissions::new().read())
        .properties(CharacteristicProperties::new().read())
        .set_value([1])
        .build();

        let service = Service::new(BleUuid::from_uuid128_string(
            "68D68245-CFD8-4A1C-9858-B27ABC4C382E",
        ))
        .name("pulse.loop")
        .primary()
        .characteristic(&api_version_characteristic)
        .build();

        Self {
            service,
            api_version_characteristic,
        }
    }
}
