use std::sync::{Arc, Mutex};

use afe4404::{device::AFE4404, modes::ThreeLedsMode};
use esp_idf_hal::i2c::I2cDriver;
use uom::si::{
    electric_current::ampere,
    electric_potential::volt,
    f32::{ElectricCurrent, ElectricPotential, Time},
    time::second,
};
use core::convert::TryInto;

macro_rules! attach_char {
    // Otical frontend uom f32 value.
    (optical frontend, $ble_characteristic:expr, $frontend:ident, $setter:ident, $getter:ident, $quantity:ident, $unit:ident) => {
        log::info!("Attaching {}.", stringify!($ble_characteristic));

        $ble_characteristic
            .write()
            .unwrap()
            .on_write(move |value, _| {
                let mut slice: [u8; 4] = [0; 4];
                slice.copy_from_slice(&value[..4]);
                let value = f32::from_le_bytes(slice);

                log::info!("Setting {} to {}", stringify!($ble_characteristic), value);

                let result = $frontend
                    .lock()
                    .unwrap()
                    .as_mut()
                    .unwrap()
                    .$setter($quantity::new::<$unit>(value));

                match result {
                    Ok(result) => {
                        log::info!("{} set to {:?}", stringify!($ble_characteristic), result);
                    }
                    Err(e) => {
                        log::error!("Error setting {}: {:?}", stringify!($ble_characteristic), e);
                    }
                }
            });

        $ble_characteristic.write().unwrap().on_read(move |_| {
            let result = $frontend.lock().unwrap().as_mut().unwrap().$getter();

            match result {
                Ok(result) => {
                    log::info!("{} is {:?}", stringify!($ble_characteristic), result);
                    result.get::<$unit>().to_le_bytes().to_vec()
                }
                Err(e) => {
                    log::error!("Error getting {}: {:?}", stringify!($ble_characteristic), e);
                    vec![]
                }
            }
        });
    };

    // Otical frontend afe4404 u8 value.
    (optical frontend enum, $ble_characteristic:expr, $frontend:ident, $setter:ident, $getter:ident) => {
        log::info!("Attaching {}.", stringify!($ble_characteristic));

        $ble_characteristic
            .write()
            .unwrap()
            .on_write(move |value, _| {
                let value = value[0];

                log::info!("Setting {} to {}", stringify!($ble_characteristic), value);

                let result = $frontend
                    .lock()
                    .unwrap()
                    .as_mut()
                    .unwrap()
                    .$setter(value.try_into().expect("Unable to convert u8 to enum."));

                match result {
                    Ok(()) => {
                        log::info!("{} set to {}", stringify!($ble_characteristic), value);
                    }
                    Err(e) => {
                        log::error!("Error setting {}: {:?}", stringify!($ble_characteristic), e);
                    }
                }
            });

        $ble_characteristic.write().unwrap().on_read(move |_| {
            let result = $frontend.lock().unwrap().as_mut().unwrap().$getter();

            match result {
                Ok(result) => {
                    let result: u8 = result.try_into().expect("Unable to convert enum to u8."); // Convert to u8.
                    log::info!("{} is {}", stringify!($ble_characteristic), result);
                    vec![result]
                }
                Err(e) => {
                    log::error!("Error getting {}: {:?}", stringify!($ble_characteristic), e);
                    vec![]
                }
            }
        });
    };

    // Optical frontend u8 value.
    (optical frontend, $ble_characteristic:expr, $frontend:ident, $setter:ident, $getter:ident) => {
        log::info!("Attaching {}.", stringify!($ble_characteristic));

        $ble_characteristic
            .write()
            .unwrap()
            .on_write(move |value, _| {
                let value = value[0];

                log::info!("Setting {} to {}", stringify!($ble_characteristic), value);

                let result = $frontend.lock().unwrap().as_mut().unwrap().$setter(value);

                match result {
                    Ok(result) => {
                        log::info!("{} set to {:?}", stringify!($ble_characteristic), result);
                    }
                    Err(e) => {
                        log::error!("Error setting {}: {:?}", stringify!($ble_characteristic), e);
                    }
                }
            });

        $ble_characteristic.write().unwrap().on_read(move |_| {
            let result = $frontend.lock().unwrap().as_mut().unwrap().$getter();

            match result {
                Ok(result) => {
                    log::info!("{} is {:?}", stringify!($ble_characteristic), result);
                    vec![result]
                }
                Err(e) => {
                    log::error!("Error getting {}: {:?}", stringify!($ble_characteristic), e);
                    vec![]
                }
            }
        });
    };

    // Optical calibration uom f32 value.
    (optical calibration, $ble_characteristic:expr, $calibrator:ident, $setter:ident, $getter:ident, $quantity:ident, $unit:ident) => {
        log::info!("Attaching {}.", stringify!($ble_characteristic));

        $ble_characteristic
            .write()
            .unwrap()
            .on_write(move |value, _| {
                let mut slice: [u8; 4] = [0; 4];
                slice.copy_from_slice(&value[..4]);
                let value = f32::from_le_bytes(slice);

                log::info!("Setting {} to {}", stringify!($ble_characteristic), value);

                *$calibrator.lock().unwrap().as_mut().unwrap().$setter() =
                    $quantity::new::<$unit>(value);

                log::info!(
                    "{} set to {:?}",
                    stringify!($ble_characteristic),
                    $quantity::new::<$unit>(value)
                );
            });

        $ble_characteristic.write().unwrap().on_read(move |_| {
            let value = *$calibrator.lock().unwrap().as_mut().unwrap().$getter();

            log::info!("{} is {:?}", stringify!($ble_characteristic), value);

            value.get::<$unit>().to_le_bytes().to_vec()
        });
    };

    // Optical calibration f32 value.
    (optical calibration, $ble_characteristic:expr, $calibrator:ident, $setter:ident, $getter:ident) => {
        log::info!("Attaching {}.", stringify!($ble_characteristic));

        $ble_characteristic
            .write()
            .unwrap()
            .on_write(move |value, _| {
                let mut slice: [u8; 4] = [0; 4];
                slice.copy_from_slice(&value[..4]);
                let value = f32::from_le_bytes(slice);

                log::info!("Setting {} to {}", stringify!($ble_characteristic), value);

                *$calibrator.lock().unwrap().as_mut().unwrap().$setter() = value;

                log::info!("{} set to {}", stringify!($ble_characteristic), value);
            });

        $ble_characteristic.write().unwrap().on_read(move |_| {
            let value = *$calibrator.lock().unwrap().as_mut().unwrap().$getter();

            log::info!("{} is {}", stringify!($ble_characteristic), value);

            value.to_le_bytes().to_vec()
        });
    };
}

pub(crate) fn attach_optical_frontend_chars(
    frontend: &'static Arc<Mutex<Option<AFE4404<I2cDriver, ThreeLedsMode>>>>,
    ble_api: &mut crate::bluetooth::BluetoothAPI,
) {
    attach_char!(
        optical frontend,
        (ble_api
            .optical_frontend_configuration
            .adc_averages_characteristic),
        frontend,
        set_averaging,
        get_averaging
    );
    attach_char!(
        optical frontend,
        (ble_api
            .optical_frontend_configuration
            .ambient_adc_conversion_end_characteristic),
        frontend,
        set_ambient_conv_end,
        get_ambient_conv_end,
        Time,
        second
    );
    attach_char!(
        optical frontend,
        (ble_api
            .optical_frontend_configuration
            .ambient_adc_conversion_start_characteristic),
        frontend,
        set_ambient_conv_st,
        get_ambient_conv_st,
        Time,
        second
    );
    attach_char!(
        optical frontend,
        (ble_api
            .optical_frontend_configuration
            .ambient_adc_reset_end_characteristic),
        frontend,
        set_ambient_reset_end,
        get_ambient_reset_end,
        Time,
        second
    );
    attach_char!(
        optical frontend,
        (ble_api
            .optical_frontend_configuration
            .ambient_adc_reset_start_characteristic),
        frontend,
        set_ambient_reset_st,
        get_ambient_reset_st,
        Time,
        second
    );
    attach_char!(
        optical frontend,
        (ble_api
            .optical_frontend_configuration
            .ambient_sample_end_characteristic),
        frontend,
        set_ambient_sample_end,
        get_ambient_sample_end,
        Time,
        second
    );
    attach_char!(
        optical frontend,
        (ble_api
            .optical_frontend_configuration
            .ambient_sample_start_characteristic),
        frontend,
        set_ambient_sample_st,
        get_ambient_sample_st,
        Time,
        second
    );
    attach_char!(
        optical frontend,
        (ble_api
            .optical_frontend_configuration
            .ambient_offset_current_characteristic),
        frontend,
        set_offset_amb_current,
        get_offset_amb_current,
        ElectricCurrent,
        ampere
    );
    attach_char!(
        optical frontend,
        (ble_api
            .optical_frontend_configuration
            .decimation_factor_characteristic),
        frontend,
        set_decimation,
        get_decimation
    );
    attach_char!(
        optical frontend,
        (ble_api
            .optical_frontend_configuration
            .dynamic_power_down_end_characteristic),
        frontend,
        set_dynamic_power_down_end,
        get_dynamic_power_down_end,
        Time,
        second
    );
    attach_char!(
        optical frontend,
        (ble_api
            .optical_frontend_configuration
            .dynamic_power_down_start_characteristic),
        frontend,
        set_dynamic_power_down_st,
        get_dynamic_power_down_st,
        Time,
        second
    );
    attach_char!(
        optical frontend,
        (ble_api
            .optical_frontend_configuration
            .led1_adc_conversion_end_characteristic),
        frontend,
        set_led1_conv_end,
        get_led1_conv_end,
        Time,
        second
    );
    attach_char!(
        optical frontend,
        (ble_api
            .optical_frontend_configuration
            .led1_adc_conversion_start_characteristic),
        frontend,
        set_led1_conv_st,
        get_led1_conv_st,
        Time,
        second
    );
    attach_char!(
        optical frontend,
        (ble_api
            .optical_frontend_configuration
            .led1_adc_reset_end_characteristic),
        frontend,
        set_led1_reset_end,
        get_led1_reset_end,
        Time,
        second
    );
    attach_char!(
        optical frontend,
        (ble_api
            .optical_frontend_configuration
            .led1_adc_reset_start_characteristic),
        frontend,
        set_led1_reset_st,
        get_led1_reset_st,
        Time,
        second
    );
    attach_char!(
        optical frontend,
        (ble_api
            .optical_frontend_configuration
            .led1_lighting_end_characteristic),
        frontend,
        set_led1_lighting_end,
        get_led1_lighting_end,
        Time,
        second
    );
    attach_char!(
        optical frontend,
        (ble_api
            .optical_frontend_configuration
            .led1_lighting_start_characteristic),
        frontend,
        set_led1_lighting_st,
        get_led1_lighting_st,
        Time,
        second
    );
    attach_char!(
        optical frontend,
        (ble_api
            .optical_frontend_configuration
            .led1_current_characteristic),
        frontend,
        set_led1_current,
        get_led1_current,
        ElectricCurrent,
        ampere
    );
    attach_char!(
        optical frontend,
        (ble_api
            .optical_frontend_configuration
            .led1_sample_end_characteristic),
        frontend,
        set_led1_sample_end,
        get_led1_sample_end,
        Time,
        second
    );
    attach_char!(
        optical frontend,
        (ble_api
            .optical_frontend_configuration
            .led1_sample_start_characteristic),
        frontend,
        set_led1_sample_st,
        get_led1_sample_st,
        Time,
        second
    );
    attach_char!(
        optical frontend,
        (ble_api
            .optical_frontend_configuration
            .led1_offset_current_characteristic),
        frontend,
        set_offset_led1_current,
        get_offset_led1_current,
        ElectricCurrent,
        ampere
    );
    attach_char!(
        optical frontend,
        (ble_api
            .optical_frontend_configuration
            .led2_adc_conversion_end_characteristic),
        frontend,
        set_led2_conv_end,
        get_led2_conv_end,
        Time,
        second
    );
    attach_char!(
        optical frontend,
        (ble_api
            .optical_frontend_configuration
            .led2_adc_conversion_start_characteristic),
        frontend,
        set_led2_conv_st,
        get_led2_conv_st,
        Time,
        second
    );
    attach_char!(
        optical frontend,
        (ble_api
            .optical_frontend_configuration
            .led2_adc_reset_end_characteristic),
        frontend,
        set_led2_reset_end,
        get_led2_reset_end,
        Time,
        second
    );
    attach_char!(
        optical frontend,
        (ble_api
            .optical_frontend_configuration
            .led2_adc_reset_start_characteristic),
        frontend,
        set_led2_reset_st,
        get_led2_reset_st,
        Time,
        second
    );
    attach_char!(
        optical frontend,
        (ble_api
            .optical_frontend_configuration
            .led2_lighting_end_characteristic),
        frontend,
        set_led2_lighting_end,
        get_led2_lighting_end,
        Time,
        second
    );
    attach_char!(
        optical frontend,
        (ble_api
            .optical_frontend_configuration
            .led2_lighting_start_characteristic),
        frontend,
        set_led2_lighting_st,
        get_led2_lighting_st,
        Time,
        second
    );
    attach_char!(
        optical frontend,
        (ble_api
            .optical_frontend_configuration
            .led2_current_characteristic),
        frontend,
        set_led2_current,
        get_led2_current,
        ElectricCurrent,
        ampere
    );
    attach_char!(
        optical frontend,
        (ble_api
            .optical_frontend_configuration
            .led2_sample_end_characteristic),
        frontend,
        set_led2_sample_end,
        get_led2_sample_end,
        Time,
        second
    );
    attach_char!(
        optical frontend,
        (ble_api
            .optical_frontend_configuration
            .led2_sample_start_characteristic),
        frontend,
        set_led2_sample_st,
        get_led2_sample_st,
        Time,
        second
    );
    attach_char!(
        optical frontend,
        (ble_api
            .optical_frontend_configuration
            .led2_offset_current_characteristic),
        frontend,
        set_offset_led2_current,
        get_offset_led2_current,
        ElectricCurrent,
        ampere
    );
    attach_char!(
        optical frontend,
        (ble_api
            .optical_frontend_configuration
            .led3_adc_conversion_end_characteristic),
        frontend,
        set_led3_conv_end,
        get_led3_conv_end,
        Time,
        second
    );
    attach_char!(
        optical frontend,
        (ble_api
            .optical_frontend_configuration
            .led3_adc_conversion_start_characteristic),
        frontend,
        set_led3_conv_st,
        get_led3_conv_st,
        Time,
        second
    );
    attach_char!(
        optical frontend,
        (ble_api
            .optical_frontend_configuration
            .led3_adc_reset_end_characteristic),
        frontend,
        set_led3_reset_end,
        get_led3_reset_end,
        Time,
        second
    );
    attach_char!(
        optical frontend,
        (ble_api
            .optical_frontend_configuration
            .led3_adc_reset_start_characteristic),
        frontend,
        set_led3_reset_st,
        get_led3_reset_st,
        Time,
        second
    );
    attach_char!(
        optical frontend,
        (ble_api
            .optical_frontend_configuration
            .led3_lighting_end_characteristic),
        frontend,
        set_led3_lighting_end,
        get_led3_lighting_end,
        Time,
        second
    );
    attach_char!(
        optical frontend,
        (ble_api
            .optical_frontend_configuration
            .led3_lighting_start_characteristic),
        frontend,
        set_led3_lighting_st,
        get_led3_lighting_st,
        Time,
        second
    );
    attach_char!(
        optical frontend,
        (ble_api
            .optical_frontend_configuration
            .led3_current_characteristic),
        frontend,
        set_led3_current,
        get_led3_current,
        ElectricCurrent,
        ampere
    );
    attach_char!(
        optical frontend,
        (ble_api
            .optical_frontend_configuration
            .led3_sample_end_characteristic),
        frontend,
        set_led3_sample_end,
        get_led3_sample_end,
        Time,
        second
    );
    attach_char!(
        optical frontend,
        (ble_api
            .optical_frontend_configuration
            .led3_sample_start_characteristic),
        frontend,
        set_led3_sample_st,
        get_led3_sample_st,
        Time,
        second
    );
    attach_char!(
        optical frontend,
        (ble_api
            .optical_frontend_configuration
            .led3_offset_current_characteristic),
        frontend,
        set_offset_led3_current,
        get_offset_led3_current,
        ElectricCurrent,
        ampere
    );
    attach_char!(
        optical frontend enum,
        (ble_api
            .optical_frontend_configuration
            .tia_capacitor_1_characteristic),
        frontend,
        set_tia_capacitor1_enum,
        get_tia_capacitor1_enum
    );
    attach_char!(
        optical frontend enum,
        (ble_api
            .optical_frontend_configuration
            .tia_capacitor_2_characteristic),
        frontend,
        set_tia_capacitor2_enum,
        get_tia_capacitor2_enum
    );
    attach_char!(
        optical frontend enum,
        (ble_api
            .optical_frontend_configuration
            .tia_resistor_1_characteristic),
        frontend,
        set_tia_resistor1_enum,
        get_tia_resistor1_enum
    );
    attach_char!(
        optical frontend enum,
        (ble_api
            .optical_frontend_configuration
            .tia_resistor_2_characteristic),
        frontend,
        set_tia_resistor2_enum,
        get_tia_resistor2_enum
    );
    attach_char!(
        optical frontend,
        (ble_api
            .optical_frontend_configuration
            .total_window_length_characteristic),
        frontend,
        set_window_period,
        get_window_period,
        Time,
        second
    );
}

pub(crate) fn attach_optical_calibration_chars(
    calibrator1: &'static Arc<Mutex<Option<super::calibration::Calibrator>>>,
    calibrator2: &'static Arc<Mutex<Option<super::calibration::Calibrator>>>,
    calibrator3: &'static Arc<Mutex<Option<super::calibration::Calibrator>>>,
    ble_api: &mut crate::bluetooth::BluetoothAPI,
) {
    attach_char!(
        optical calibration,
        (ble_api.calibration.led1_current_min),
        calibrator1,
        led_current_min_mut,
        led_current_min,
        ElectricCurrent,
        ampere
    );
    attach_char!(
        optical calibration,
        (ble_api.calibration.led1_current_max),
        calibrator1,
        led_current_max_mut,
        led_current_max,
        ElectricCurrent,
        ampere
    );
    attach_char!(
        optical calibration,
        (ble_api.calibration.led1_adc_set_point),
        calibrator1,
        adc_set_point_mut,
        adc_set_point,
        ElectricPotential,
        volt
    );
    attach_char!(
        optical calibration,
        (ble_api.calibration.led1_adc_working_threshold),
        calibrator1,
        adc_working_threshold_mut,
        adc_working_threshold,
        ElectricPotential,
        volt
    );
    attach_char!(
        optical calibration,
        (ble_api.calibration.led1_alpha),
        calibrator1,
        alpha_mut,
        alpha
    );
    attach_char!(
        optical calibration,
        (ble_api.calibration.led2_current_min),
        calibrator2,
        led_current_min_mut,
        led_current_min,
        ElectricCurrent,
        ampere
    );
    attach_char!(
        optical calibration,
        (ble_api.calibration.led2_current_max),
        calibrator2,
        led_current_max_mut,
        led_current_max,
        ElectricCurrent,
        ampere
    );
    attach_char!(
        optical calibration,
        (ble_api.calibration.led2_adc_set_point),
        calibrator2,
        adc_set_point_mut,
        adc_set_point,
        ElectricPotential,
        volt
    );
    attach_char!(
        optical calibration,
        (ble_api.calibration.led2_adc_working_threshold),
        calibrator2,
        adc_working_threshold_mut,
        adc_working_threshold,
        ElectricPotential,
        volt
    );
    attach_char!(
        optical calibration,
        (ble_api.calibration.led2_alpha),
        calibrator2,
        alpha_mut,
        alpha
    );
    attach_char!(
        optical calibration,
        (ble_api.calibration.led3_current_min),
        calibrator3,
        led_current_min_mut,
        led_current_min,
        ElectricCurrent,
        ampere
    );
    attach_char!(
        optical calibration,
        (ble_api.calibration.led3_current_max),
        calibrator3,
        led_current_max_mut,
        led_current_max,
        ElectricCurrent,
        ampere
    );
    attach_char!(
        optical calibration,
        (ble_api.calibration.led3_adc_set_point),
        calibrator3,
        adc_set_point_mut,
        adc_set_point,
        ElectricPotential,
        volt
    );
    attach_char!(
        optical calibration,
        (ble_api.calibration.led3_adc_working_threshold),
        calibrator3,
        adc_working_threshold_mut,
        adc_working_threshold,
        ElectricPotential,
        volt
    );
    attach_char!(
        optical calibration,
        (ble_api.calibration.led3_alpha),
        calibrator3,
        alpha_mut,
        alpha
    );
}
