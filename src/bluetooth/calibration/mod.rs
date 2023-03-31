use std::sync::{Arc, RwLock};

use bluedroid::gatt_server::{Characteristic, Service};
use bluedroid::utilities::{AttributePermissions, BleUuid, CharacteristicProperties};
use log::warn;

pub struct CalibrationServiceContainer {
    pub(crate) service: Arc<RwLock<Service>>,
    pub(crate) led1_current_min: Arc<RwLock<Characteristic>>,
    pub(crate) led1_current_max: Arc<RwLock<Characteristic>>,
    pub(crate) led1_adc_set_point: Arc<RwLock<Characteristic>>,
    pub(crate) led1_adc_working_threshold: Arc<RwLock<Characteristic>>,
    pub(crate) led1_alpha: Arc<RwLock<Characteristic>>,
    pub(crate) led2_current_min: Arc<RwLock<Characteristic>>,
    pub(crate) led2_current_max: Arc<RwLock<Characteristic>>,
    pub(crate) led2_adc_set_point: Arc<RwLock<Characteristic>>,
    pub(crate) led2_adc_working_threshold: Arc<RwLock<Characteristic>>,
    pub(crate) led2_alpha: Arc<RwLock<Characteristic>>,
    pub(crate) led3_current_min: Arc<RwLock<Characteristic>>,
    pub(crate) led3_current_max: Arc<RwLock<Characteristic>>,
    pub(crate) led3_adc_set_point: Arc<RwLock<Characteristic>>,
    pub(crate) led3_adc_working_threshold: Arc<RwLock<Characteristic>>,
    pub(crate) led3_alpha: Arc<RwLock<Characteristic>>,
}

impl CalibrationServiceContainer {
    #[allow(clippy::too_many_lines)]
    pub(crate) fn initialise() -> Self {
        #[rustfmt::skip]
        let characteristic_list: [(&str, &str); 15] = [
            ("2043264C-C1A8-4A62-8FDE-525BE380AA13", "LED1 current min"),
            ("71F1573E-DB0D-4B52-9E9F-AA505719D41D", "LED1 current max"),
            ("9B98BA9A-9EEA-40F6-87F4-53BF2BB19699", "LED1 adc set point"),
            ("41A91B62-9FB2-41E3-906A-E24697D938D5", "LED1 adc working threshold"),
            ("A01B4911-9CA4-4E51-A484-C0E5E962FDA6", "LED1 alpha"),
            ("9621CF82-87A9-4794-AB81-7BAC475574BD", "LED2 current min"),
            ("2EB0E60C-B688-479A-AC80-D196F3146FD0", "LED2 current max"),
            ("BA113050-05DC-4A44-B4EF-7DBF10E74171", "LED2 adc set point"),
            ("43C5ECAD-63F4-42A8-A3AE-7F799FF6B01B", "LED2 adc working threshold"),
            ("1E33ED6E-1EB1-4738-9BAA-6A617BECB801", "LED2 alpha"),
            ("B7FF9A50-9954-4E5E-AD49-1A1925C51C33", "LED3 current min"),
            ("EB28857B-622F-42D8-B304-F7CCAE955EC0", "LED3 current max"),
            ("4D149938-C228-4345-B41C-26CDFF119B41", "LED3 adc set point"),
            ("337F34FC-E9A3-4BEC-817D-2E194D60E0B6", "LED3 adc working threshold"),
            ("A067A9B6-5395-448B-90D5-B243FE8E120D", "LED3 alpha"),
        ];

        let mut characteristics: Vec<Arc<RwLock<Characteristic>>> = vec![];

        let mut service = Service::new(BleUuid::from_uuid128_string(
            "0E87EDC7-757C-49BA-87A8-F1EA1053F4C1",
        ))
        .name("Calibration")
        .primary()
        .clone();

        for item in characteristic_list {
            let characteristic = Characteristic::new(BleUuid::from_uuid128_string(item.0))
                .name(item.1)
                .show_name()
                .permissions(AttributePermissions::new().read().write())
                .properties(CharacteristicProperties::new().read().write())
                .on_read(|_| {
                    warn!("Read not implemented.");
                    vec![0x00]
                })
                .max_value_length(4)
                .build();

            service.characteristic(&characteristic);
            characteristics.push(characteristic);
        }

        let service = service.build();

        Self {
            service,
            led1_current_min: characteristics[0].clone(),
            led1_current_max: characteristics[1].clone(),
            led1_adc_set_point: characteristics[2].clone(),
            led1_adc_working_threshold: characteristics[3].clone(),
            led1_alpha: characteristics[4].clone(),
            led2_current_min: characteristics[5].clone(),
            led2_current_max: characteristics[6].clone(),
            led2_adc_set_point: characteristics[7].clone(),
            led2_adc_working_threshold: characteristics[8].clone(),
            led2_alpha: characteristics[9].clone(),
            led3_current_min: characteristics[10].clone(),
            led3_current_max: characteristics[11].clone(),
            led3_adc_set_point: characteristics[12].clone(),
            led3_adc_working_threshold: characteristics[13].clone(),
            led3_alpha: characteristics[14].clone(),
        }
    }
}
