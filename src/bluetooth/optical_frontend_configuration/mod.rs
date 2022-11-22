use std::sync::{Arc, RwLock};

use bluedroid::gatt_server::{Characteristic, Service};
use bluedroid::utilities::{AttributePermissions, BleUuid, CharacteristicProperties};

pub struct OpticalFrontendConfigurationServiceContainer {
    pub(crate) service: Arc<RwLock<Service>>,
    pub(crate) ambient_adc_conversion_end_characteristic: Arc<RwLock<Characteristic>>,
    pub(crate) ambient_adc_conversion_start_characteristic: Arc<RwLock<Characteristic>>,
    pub(crate) ambient_adc_reset_end_characteristic: Arc<RwLock<Characteristic>>,
    pub(crate) ambient_adc_reset_start_characteristic: Arc<RwLock<Characteristic>>,
    pub(crate) ambient_sample_end_characteristic: Arc<RwLock<Characteristic>>,
    pub(crate) ambient_sample_start_characteristic: Arc<RwLock<Characteristic>>,
    pub(crate) dynamic_power_down_end_characteristic: Arc<RwLock<Characteristic>>,
    pub(crate) dynamic_power_down_start_characteristic: Arc<RwLock<Characteristic>>,
    pub(crate) led1_adc_conversion_end_characteristic: Arc<RwLock<Characteristic>>,
    pub(crate) led1_adc_conversion_start_characteristic: Arc<RwLock<Characteristic>>,
    pub(crate) led1_adc_reset_end_characteristic: Arc<RwLock<Characteristic>>,
    pub(crate) led1_adc_reset_start_characteristic: Arc<RwLock<Characteristic>>,
    pub(crate) led1_lighting_end_characteristic: Arc<RwLock<Characteristic>>,
    pub(crate) led1_lighting_start_characteristic: Arc<RwLock<Characteristic>>,
    pub(crate) led1_current_characteristic: Arc<RwLock<Characteristic>>,
    pub(crate) led1_sample_end_characteristic: Arc<RwLock<Characteristic>>,
    pub(crate) led1_sample_start_characteristic: Arc<RwLock<Characteristic>>,
    pub(crate) led2_adc_conversion_end_characteristic: Arc<RwLock<Characteristic>>,
    pub(crate) led2_adc_conversion_start_characteristic: Arc<RwLock<Characteristic>>,
    pub(crate) led2_adc_reset_end_characteristic: Arc<RwLock<Characteristic>>,
    pub(crate) led2_adc_reset_start_characteristic: Arc<RwLock<Characteristic>>,
    pub(crate) led2_lighting_end_characteristic: Arc<RwLock<Characteristic>>,
    pub(crate) led2_lighting_start_characteristic: Arc<RwLock<Characteristic>>,
    pub(crate) led2_current_characteristic: Arc<RwLock<Characteristic>>,
    pub(crate) led2_sample_end_characteristic: Arc<RwLock<Characteristic>>,
    pub(crate) led2_sample_start_characteristic: Arc<RwLock<Characteristic>>,
    pub(crate) led3_adc_conversion_end_characteristic: Arc<RwLock<Characteristic>>,
    pub(crate) led3_adc_conversion_start_characteristic: Arc<RwLock<Characteristic>>,
    pub(crate) led3_adc_reset_end_characteristic: Arc<RwLock<Characteristic>>,
    pub(crate) led3_adc_reset_start_characteristic: Arc<RwLock<Characteristic>>,
    pub(crate) led3_lighting_end_characteristic: Arc<RwLock<Characteristic>>,
    pub(crate) led3_lighting_start_characteristic: Arc<RwLock<Characteristic>>,
    pub(crate) led3_current_characteristic: Arc<RwLock<Characteristic>>,
    pub(crate) led3_sample_end_characteristic: Arc<RwLock<Characteristic>>,
    pub(crate) led3_sample_start_characteristic: Arc<RwLock<Characteristic>>,
    pub(crate) tia_capacitor_1_characteristic: Arc<RwLock<Characteristic>>,
    pub(crate) tia_capacitor_2_characteristic: Arc<RwLock<Characteristic>>,
    pub(crate) tia_resistor_1_characteristic: Arc<RwLock<Characteristic>>,
    pub(crate) tia_resistor_2_characteristic: Arc<RwLock<Characteristic>>,
    pub(crate) total_window_length_characteristic: Arc<RwLock<Characteristic>>,
}

impl OpticalFrontendConfigurationServiceContainer {
    #[allow(clippy::too_many_lines)]
    pub(crate) fn initialise() -> Self {
        #[rustfmt::skip]
        let characteristic_list: [(&str, &str); 40] = [
            ("9B6AF28C-9558-49ED-844B-06E7B8B0E6C3", "Ambient ADC conversion end"),
            ("66DC5EDA-B89E-43D5-B940-13E29A468C77", "Ambient ADC conversion start"),
            ("E9AB33D4-DA9C-4424-851A-16CF66AA08C0", "Ambient ADC reset end"),
            ("FD7FAFE2-4464-4F8C-A55C-79E45FB916B3", "Ambient ADC reset start"),
            ("83C29A09-B658-4316-A1FD-D8AD97C02F80", "Ambient sample end"),
            ("C35EBCC5-BCA4-4716-90E3-37B94D9AD6FF", "Ambient sample start"),
            ("BC276997-792F-4391-9371-78F1C1038DB7", "Dynamic power-down end"),
            ("0B68331C-B628-4D81-BBDB-47B79EA2430E", "Dynamic power-down start"),
            ("C455472B-4231-4EF7-A3BD-A1AE2676F9D2", "LED1 ADC conversion end"),
            ("ED9B9EE3-AAFE-4034-8C07-1D7F495288B1", "LED1 ADC conversion start"),
            ("7049E8C6-A0CE-4380-8186-1B7CD34179ED", "LED1 ADC reset end"),
            ("C8B42A6D-ECFC-40E8-8E3C-F5876EE749A3", "LED1 ADC reset start"),
            ("9C678B37-F3AA-4B8C-AFD5-10B4694E49C7", "LED1 lighting end"),
            ("F02C15DF-14F1-4872-BE99-33EE64F0E0B3", "LED1 lighting start"),
            ("A20B7943-5E1D-4053-8C4E-CD44463F460D", "LED1 current"),
            ("F60A8B03-FAB1-433D-9D9E-8722DF003329", "LED1 sample end"),
            ("FB219512-DC81-461A-B558-FE6E310E9333", "LED1 sample start"),
            ("40314C89-306E-47F0-AE1F-C5DDD8C0CDDD", "LED2 ADC conversion end"),
            ("160CC306-3CA6-4BF5-AC0B-85443F3CFC6B", "LED2 ADC conversion start"),
            ("34D6F164-543F-49F4-B0F1-6E68DC4CEE6B", "LED2 ADC reset end"),
            ("E34424D5-68DA-467F-93FE-BE49F19FAB0E", "LED2 ADC reset start"),
            ("B85968BA-FB52-46E8-81A5-0F837BF3D6EB", "LED2 lighting end"),
            ("F710D5DC-2655-42D6-97AA-7A5FDF0285C8", "LED2 lighting start"),
            ("29CA51A3-B33B-44FD-853C-00FE8827ADC4", "LED2 current"),
            ("F752142C-5BFC-4274-9044-E81D3F2F274A", "LED2 sample end"),
            ("38644B85-3D2E-4D31-9679-06C9EB6BAC2D", "LED2 sample start"),
            ("7C2A9A6F-95EB-45ED-B7E1-BB290F7853ED", "LED3 ADC conversion end"),
            ("C03D3143-E6B6-49AB-85FC-EEED3A43B530", "LED3 ADC conversion start"),
            ("A7D441AA-C456-4CBF-A0B9-84DBF33934EF", "LED3 ADC reset end"),
            ("536D72C8-DFF0-4E38-93F7-7F376316EA8D", "LED3 ADC reset start"),
            ("5B7F9859-092B-43D4-AC6B-AC9DD4742AB2", "LED3 lighting end"),
            ("0B098015-110E-487E-AAE9-BEA1ED1F54A0", "LED3 lighting start"),
            ("F7535ED9-CB9F-469A-817E-1635DC3B68B0", "LED3 current"),
            ("249782EC-004B-4A3D-9608-5143E69AB294", "LED3 sample end"),
            ("733C5AED-D3B3-4F65-8898-6EA37DA30F71", "LED3 sample start"),
            ("08B3B8E9-D3AD-48EB-B93B-AF4D3695F05C", "TIA capacitor 1"),
            ("740669DF-57D3-4147-87B4-DC302512F20A", "TIA capacitor 2"),
            ("81831E3A-917E-4252-9C16-42BA8FF3F47A", "TIA resistor 1"),
            ("A3F694D1-C378-4124-BF56-468DFAFF14E6", "TIA resistor 2"),
            ("B904BD23-6082-4507-8BD2-7333EF6A2726", "Total window length"),
        ];

        let mut characteristics: Vec<Arc<RwLock<Characteristic>>> = vec![];

        let mut service = Service::new(BleUuid::from_uuid128_string(
            "C8F276D4-E0DD-4660-8070-619FF734134B",
        ))
        .name("Optical frontend configuration")
        .primary()
        .clone();

        for item in characteristic_list {
            let characteristic = Characteristic::new(BleUuid::from_uuid128_string(item.0))
                .name(item.1)
                .show_name()
                .permissions(AttributePermissions::new().read().write())
                .properties(CharacteristicProperties::new().read().write())
                .max_value_length(4)
                .build();

            service.characteristic(&characteristic);
            characteristics.push(characteristic);
        }

        let service = service.build();

        Self {
            service,
            ambient_adc_conversion_end_characteristic: characteristics[0].clone(),
            ambient_adc_conversion_start_characteristic: characteristics[1].clone(),
            ambient_adc_reset_end_characteristic: characteristics[2].clone(),
            ambient_adc_reset_start_characteristic: characteristics[3].clone(),
            ambient_sample_end_characteristic: characteristics[4].clone(),
            ambient_sample_start_characteristic: characteristics[5].clone(),
            dynamic_power_down_end_characteristic: characteristics[6].clone(),
            dynamic_power_down_start_characteristic: characteristics[7].clone(),
            led1_adc_conversion_end_characteristic: characteristics[8].clone(),
            led1_adc_conversion_start_characteristic: characteristics[9].clone(),
            led1_adc_reset_end_characteristic: characteristics[10].clone(),
            led1_adc_reset_start_characteristic: characteristics[11].clone(),
            led1_lighting_end_characteristic: characteristics[12].clone(),
            led1_lighting_start_characteristic: characteristics[13].clone(),
            led1_current_characteristic: characteristics[14].clone(),
            led1_sample_end_characteristic: characteristics[15].clone(),
            led1_sample_start_characteristic: characteristics[16].clone(),
            led2_adc_conversion_end_characteristic: characteristics[17].clone(),
            led2_adc_conversion_start_characteristic: characteristics[18].clone(),
            led2_adc_reset_end_characteristic: characteristics[19].clone(),
            led2_adc_reset_start_characteristic: characteristics[20].clone(),
            led2_lighting_end_characteristic: characteristics[21].clone(),
            led2_lighting_start_characteristic: characteristics[22].clone(),
            led2_current_characteristic: characteristics[23].clone(),
            led2_sample_end_characteristic: characteristics[24].clone(),
            led2_sample_start_characteristic: characteristics[25].clone(),
            led3_adc_conversion_end_characteristic: characteristics[26].clone(),
            led3_adc_conversion_start_characteristic: characteristics[27].clone(),
            led3_adc_reset_end_characteristic: characteristics[28].clone(),
            led3_adc_reset_start_characteristic: characteristics[29].clone(),
            led3_lighting_end_characteristic: characteristics[30].clone(),
            led3_lighting_start_characteristic: characteristics[31].clone(),
            led3_current_characteristic: characteristics[32].clone(),
            led3_sample_end_characteristic: characteristics[33].clone(),
            led3_sample_start_characteristic: characteristics[34].clone(),
            tia_capacitor_1_characteristic: characteristics[35].clone(),
            tia_capacitor_2_characteristic: characteristics[36].clone(),
            tia_resistor_1_characteristic: characteristics[37].clone(),
            tia_resistor_2_characteristic: characteristics[38].clone(),
            total_window_length_characteristic: characteristics[39].clone(),
        }
    }
}
