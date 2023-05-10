use std::sync::{Arc, RwLock};

use bluedroid::gatt_server::{Characteristic, Service};
use bluedroid::utilities::{AttributePermissions, BleUuid, CharacteristicProperties};
use log::warn;

pub struct CalibrationServiceContainer {
    pub(crate) service: Arc<RwLock<Service>>,
    pub(crate) led1_current_min: Arc<RwLock<Characteristic>>,
    pub(crate) led1_current_max: Arc<RwLock<Characteristic>>,
    pub(crate) led1_offset_current_min: Arc<RwLock<Characteristic>>,
    pub(crate) led1_offset_current_max: Arc<RwLock<Characteristic>>,
    pub(crate) led1_offset_current_set_point: Arc<RwLock<Characteristic>>,
    pub(crate) led1_adc_set_point: Arc<RwLock<Characteristic>>,
    pub(crate) led1_adc_working_threshold: Arc<RwLock<Characteristic>>,
    pub(crate) led1_alpha: Arc<RwLock<Characteristic>>,
    pub(crate) led2_led3_current_min: Arc<RwLock<Characteristic>>,
    pub(crate) led2_led3_current_max: Arc<RwLock<Characteristic>>,
    pub(crate) led2_led3_offset_current_min: Arc<RwLock<Characteristic>>,
    pub(crate) led2_led3_offset_current_max: Arc<RwLock<Characteristic>>,
    pub(crate) led2_led3_offset_current_set_point: Arc<RwLock<Characteristic>>,
    pub(crate) led2_led3_adc_set_point: Arc<RwLock<Characteristic>>,
    pub(crate) led2_led3_adc_working_threshold: Arc<RwLock<Characteristic>>,
    pub(crate) led2_led3_alpha: Arc<RwLock<Characteristic>>,
}

impl CalibrationServiceContainer {
    #[allow(clippy::too_many_lines)]
    pub(crate) fn initialise() -> Self {
        #[rustfmt::skip]
        let characteristic_list: [(&str, &str); 16] = [
            ("2043264C-C1A8-4A62-8FDE-525BE380AA13", "LED1 current min"),
            ("71F1573E-DB0D-4B52-9E9F-AA505719D41D", "LED1 current max"),
            ("914E65A0-F10D-4E35-9705-424FBE514594", "LED1 offset current min"),
            ("0428B369-BD92-4625-BEF3-55B9C054411E", "LED1 offset current max"),
            ("BA6BFE73-1621-42CC-B792-AEE5BAAE57CD", "LED1 offset current set point"),
            ("9B98BA9A-9EEA-40F6-87F4-53BF2BB19699", "LED1 adc set point"),
            ("41A91B62-9FB2-41E3-906A-E24697D938D5", "LED1 adc working threshold"),
            ("A01B4911-9CA4-4E51-A484-C0E5E962FDA6", "LED1 alpha"),
            ("9621CF82-87A9-4794-AB81-7BAC475574BD", "LED2 LED3 current min"),
            ("2EB0E60C-B688-479A-AC80-D196F3146FD0", "LED2 LED3 current max"),
            ("913C4C37-63E9-49C4-9944-782DD702D503", "LED2 LED3 offset current min"),
            ("6F2BB2FE-6DB8-4D3B-8AA6-5D4845CFBFA2", "LED2 LED3 offset current max"),
            ("FDBB0D89-33B6-40E0-B7B5-1C5E74D3FB05", "LED2 LED3 offset current set point"),
            ("BA113050-05DC-4A44-B4EF-7DBF10E74171", "LED2 LED3 adc set point"),
            ("43C5ECAD-63F4-42A8-A3AE-7F799FF6B01B", "LED2 LED3 adc working threshold"),
            ("1E33ED6E-1EB1-4738-9BAA-6A617BECB801", "LED2 LED3 alpha"),
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
            led1_offset_current_min: characteristics[2].clone(),
            led1_offset_current_max: characteristics[3].clone(),
            led1_offset_current_set_point: characteristics[4].clone(),
            led1_adc_set_point: characteristics[5].clone(),
            led1_adc_working_threshold: characteristics[6].clone(),
            led1_alpha: characteristics[7].clone(),
            led2_led3_current_min: characteristics[8].clone(),
            led2_led3_current_max: characteristics[9].clone(),
            led2_led3_offset_current_min: characteristics[10].clone(),
            led2_led3_offset_current_max: characteristics[11].clone(),
            led2_led3_offset_current_set_point: characteristics[12].clone(),
            led2_led3_adc_set_point: characteristics[13].clone(),
            led2_led3_adc_working_threshold: characteristics[14].clone(),
            led2_led3_alpha: characteristics[15].clone(),
        }
    }
}
