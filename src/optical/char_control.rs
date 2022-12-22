use std::sync::{Arc, Mutex};

use afe4404::{device::AFE4404, modes::ThreeLedsMode};
use esp_idf_hal::i2c::I2cDriver;
use log::info;
use uom::si::{
    capacitance::picofarad,
    electric_current::milliampere,
    electrical_resistance::ohm,
    f32::{Capacitance, ElectricCurrent, ElectricalResistance, Time},
    time::microsecond,
};

macro_rules! attach_char {
    ($ble_characteristic:expr, $frontend:ident, $setter:ident, $getter:ident, $quantity:ident, $unit:ident) => {
        $ble_characteristic
            .write()
            .unwrap()
            .on_write(move |value, _| {
                let mut slice: [u8; 4] = [0; 4];
                slice.copy_from_slice(&value[..4]);
                let value = f32::from_le_bytes(slice);

                info!("Setting {} to {}", stringify!($ble_characteristic), value);

                let result = $frontend
                    .lock()
                    .unwrap()
                    .as_mut()
                    .unwrap()
                    .$setter($quantity::new::<$unit>(value));

                match result {
                    Ok(result) => {
                        info!("{} set to {:?}", stringify!($ble_characteristic), result);
                    }
                    Err(e) => {
                        info!("Error setting {}: {:?}", stringify!($ble_characteristic), e);
                    }
                }
            });

        $ble_characteristic.write().unwrap().on_read(move |_| {
            let result = $frontend.lock().unwrap().as_mut().unwrap().$getter();

            match result {
                Ok(result) => {
                    info!("{} is {:?}", stringify!($ble_characteristic), result);
                    result.value.to_le_bytes().to_vec()
                }
                Err(e) => {
                    info!("Error getting {}: {:?}", stringify!($ble_characteristic), e);
                    vec![]
                }
            }
        });
    };

    ($ble_characteristic:expr, $frontend:ident, $setter:ident, $getter:ident) => {};
}

pub(crate) fn attach_optical_frontend_chars(
    frontend: &'static Arc<Mutex<Option<AFE4404<I2cDriver, ThreeLedsMode>>>>,
    ble_api: &mut crate::bluetooth::BluetoothAPI,
) {
    // LED1 chars.
    attach_char!(
        (ble_api
            .optical_frontend_configuration
            .led1_lighting_start_characteristic),
        frontend,
        set_led1_lighting_st,
        get_led1_lighting_st,
        Time,
        microsecond
    );
    attach_char!(
        (ble_api
            .optical_frontend_configuration
            .led1_lighting_end_characteristic),
        frontend,
        set_led1_lighting_end,
        get_led1_lighting_end,
        Time,
        microsecond
    );
    attach_char!(
        (ble_api
            .optical_frontend_configuration
            .led1_sample_start_characteristic),
        frontend,
        set_led1_sample_st,
        get_led1_sample_st,
        Time,
        microsecond
    );
    attach_char!(
        (ble_api
            .optical_frontend_configuration
            .led1_sample_end_characteristic),
        frontend,
        set_led1_sample_end,
        get_led1_sample_end,
        Time,
        microsecond
    );
    attach_char!(
        (ble_api
            .optical_frontend_configuration
            .led1_adc_reset_start_characteristic),
        frontend,
        set_led1_reset_st,
        get_led1_reset_st,
        Time,
        microsecond
    );
    attach_char!(
        (ble_api
            .optical_frontend_configuration
            .led1_adc_reset_end_characteristic),
        frontend,
        set_led1_reset_end,
        get_led1_reset_end,
        Time,
        microsecond
    );
    attach_char!(
        (ble_api
            .optical_frontend_configuration
            .led1_adc_conversion_start_characteristic),
        frontend,
        set_led1_conv_st,
        get_led1_conv_st,
        Time,
        microsecond
    );
    attach_char!(
        (ble_api
            .optical_frontend_configuration
            .led1_adc_conversion_end_characteristic),
        frontend,
        set_led1_conv_end,
        get_led1_conv_end,
        Time,
        microsecond
    );
    attach_char!(
        (ble_api
            .optical_frontend_configuration
            .led1_current_characteristic),
        frontend,
        set_led1_current,
        get_led1_current,
        ElectricCurrent,
        milliampere
    );

    // LED2 chars.
    attach_char!(
        (ble_api
            .optical_frontend_configuration
            .led2_lighting_start_characteristic),
        frontend,
        set_led2_lighting_st,
        get_led2_lighting_st,
        Time,
        microsecond
    );
    attach_char!(
        (ble_api
            .optical_frontend_configuration
            .led2_lighting_end_characteristic),
        frontend,
        set_led2_lighting_end,
        get_led2_lighting_end,
        Time,
        microsecond
    );
    attach_char!(
        (ble_api
            .optical_frontend_configuration
            .led2_sample_start_characteristic),
        frontend,
        set_led2_sample_st,
        get_led2_sample_st,
        Time,
        microsecond
    );
    attach_char!(
        (ble_api
            .optical_frontend_configuration
            .led2_sample_end_characteristic),
        frontend,
        set_led2_sample_end,
        get_led2_sample_end,
        Time,
        microsecond
    );
    attach_char!(
        (ble_api
            .optical_frontend_configuration
            .led2_adc_reset_start_characteristic),
        frontend,
        set_led2_reset_st,
        get_led2_reset_st,
        Time,
        microsecond
    );
    attach_char!(
        (ble_api
            .optical_frontend_configuration
            .led2_adc_reset_end_characteristic),
        frontend,
        set_led2_reset_end,
        get_led2_reset_end,
        Time,
        microsecond
    );
    attach_char!(
        (ble_api
            .optical_frontend_configuration
            .led2_adc_conversion_start_characteristic),
        frontend,
        set_led2_conv_st,
        get_led2_conv_st,
        Time,
        microsecond
    );
    attach_char!(
        (ble_api
            .optical_frontend_configuration
            .led2_adc_conversion_end_characteristic),
        frontend,
        set_led2_conv_end,
        get_led2_conv_end,
        Time,
        microsecond
    );
    attach_char!(
        (ble_api
            .optical_frontend_configuration
            .led2_current_characteristic),
        frontend,
        set_led2_current,
        get_led2_current,
        ElectricCurrent,
        milliampere
    );

    // LED3 chars.
    // Ambient chars.
    // Dynamic power-down chars.
    // Window length char.

    // TIA chars.
    attach_char!(
        (ble_api
            .optical_frontend_configuration
            .tia_resistor_1_characteristic),
        frontend,
        set_tia_resistor1,
        get_tia_resistor1,
        ElectricalResistance,
        ohm
    );
    attach_char!(
        (ble_api
            .optical_frontend_configuration
            .tia_resistor_2_characteristic),
        frontend,
        set_tia_resistor2,
        get_tia_resistor2,
        ElectricalResistance,
        ohm
    );
    attach_char!(
        (ble_api
            .optical_frontend_configuration
            .tia_capacitor_1_characteristic),
        frontend,
        set_tia_capacitor1,
        get_tia_capacitor1,
        Capacitance,
        picofarad
    );
    attach_char!(
        (ble_api
            .optical_frontend_configuration
            .tia_capacitor_2_characteristic),
        frontend,
        set_tia_capacitor2,
        get_tia_capacitor2,
        Capacitance,
        picofarad
    );
}
