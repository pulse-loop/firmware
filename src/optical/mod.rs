use std::sync::{Arc, Mutex, RwLock};

use esp_idf_hal::{
    gpio::{Input, Pin, PinDriver},
    i2c::I2cDriver,
};

use uom::si::{
    capacitance::picofarad,
    electric_current::milliampere,
    electrical_resistance::ohm,
    f32::{Capacitance, ElectricCurrent, ElectricalResistance, Frequency, Time},
    frequency::megahertz,
    time::microsecond,
};

use afe4404::{
    device::AFE4404,
    led_current::LedCurrentConfiguration,
    modes::ThreeLedsMode,
    {
        clock::ClockConfiguration,
        measurement_window::{
            ActiveTiming, AmbientTiming, LedTiming, MeasurementWindowConfiguration, PowerDownTiming,
        },
    },
};

use crate::bluetooth::BluetoothAPI;

pub(crate) mod calibration;
pub(crate) mod char_control;
pub(crate) mod data_reading;
pub(crate) mod data_sending;
pub(crate) mod signal_processing;
pub(crate) mod timer;

lazy_static::lazy_static! {
    pub static ref FRONTEND: Arc<Mutex<Option<AFE4404<I2cDriver<'static>, ThreeLedsMode>>>> = Arc::new(Mutex::new(None));
    pub(crate) static ref CALIBRATOR_LED1: Arc<Mutex<Option<calibration::Calibrator>>> = Arc::new(Mutex::new(None));
    pub(crate) static ref CALIBRATOR_LED2: Arc<Mutex<Option<calibration::Calibrator>>> = Arc::new(Mutex::new(None));
    pub(crate) static ref CALIBRATOR_LED3: Arc<Mutex<Option<calibration::Calibrator>>> = Arc::new(Mutex::new(None));
}

// Afe4404 constants.
pub(crate) static RESISTOR1: f32 = 500e3;
pub(crate) static RESISTOR2: f32 = 10e3;

/// Initialises the `FRONTEND` with default values.
pub(crate) fn initialise<P: Pin>(
    i2c: I2cDriver<'static>,
    interrupt_pin: &mut PinDriver<P, Input>,
    ble_api: Arc<RwLock<BluetoothAPI>>,
    offset_currents: &mut calibration::offset_measuring::OffsetCurrents,
) {
    // Interrupt pin.
    interrupt_pin
        .set_interrupt_type(esp_idf_hal::gpio::InterruptType::PosEdge)
        .unwrap();

    unsafe {
        interrupt_pin
            .subscribe(|| {
                data_reading::DATA_READY.store(true, std::sync::atomic::Ordering::Relaxed);
            })
            .unwrap();
    }

    // Frontend.
    *FRONTEND.lock().unwrap() = Some(AFE4404::with_three_leds(
        i2c,
        0x58u8,
        Frequency::new::<megahertz>(4.0),
    ));

    if let Ok(mut frontend) = FRONTEND.lock() {
        if let Some(frontend) = frontend.as_mut() {
            frontend.sw_reset().expect("Cannot reset the afe.");

            frontend
                .set_leds_current(&LedCurrentConfiguration::<ThreeLedsMode>::new(
                    ElectricCurrent::new::<milliampere>(0.0),
                    ElectricCurrent::new::<milliampere>(0.0),
                    ElectricCurrent::new::<milliampere>(0.0),
                ))
                .expect("Cannot set LEDs current.");

            frontend
                .set_tia_resistor1(ElectricalResistance::new::<ohm>(RESISTOR1))
                .expect("Cannot set TIA resistor 1.");
            frontend
                .set_tia_resistor2(ElectricalResistance::new::<ohm>(RESISTOR2))
                .expect("Cannot set TIA resistor 2.");
            frontend
                .set_tia_capacitor1(Capacitance::new::<picofarad>(2.5))
                .expect("Cannot set TIA capacitor 1.");
            frontend
                .set_tia_capacitor2(Capacitance::new::<picofarad>(2.5))
                .expect("Cannot set TIA capacitor 2.");

            frontend
                .set_clock_source(ClockConfiguration::Internal)
                .expect("Cannot set clock source.");

            frontend
                .set_measurement_window(&MeasurementWindowConfiguration::<ThreeLedsMode>::new(
                    Time::new::<microsecond>(50_000.0),
                    ActiveTiming::<ThreeLedsMode>::new(
                        LedTiming {
                            lighting_st: Time::new::<microsecond>(600.0),
                            lighting_end: Time::new::<microsecond>(890.0),
                            sample_st: Time::new::<microsecond>(680.0),
                            sample_end: Time::new::<microsecond>(890.0),
                            reset_st: Time::new::<microsecond>(3200.0),
                            reset_end: Time::new::<microsecond>(3208.0),
                            conv_st: Time::new::<microsecond>(3210.0),
                            conv_end: Time::new::<microsecond>(3690.0),
                        },
                        LedTiming {
                            lighting_st: Time::new::<microsecond>(0.0),
                            lighting_end: Time::new::<microsecond>(290.0),
                            sample_st: Time::new::<microsecond>(80.0),
                            sample_end: Time::new::<microsecond>(290.0),
                            reset_st: Time::new::<microsecond>(2200.0),
                            reset_end: Time::new::<microsecond>(2208.0),
                            conv_st: Time::new::<microsecond>(2210.0),
                            conv_end: Time::new::<microsecond>(2690.0),
                        },
                        LedTiming {
                            lighting_st: Time::new::<microsecond>(300.0),
                            lighting_end: Time::new::<microsecond>(590.0),
                            sample_st: Time::new::<microsecond>(380.0),
                            sample_end: Time::new::<microsecond>(590.0),
                            reset_st: Time::new::<microsecond>(2700.0),
                            reset_end: Time::new::<microsecond>(2708.0),
                            conv_st: Time::new::<microsecond>(2710.0),
                            conv_end: Time::new::<microsecond>(3190.0),
                        },
                        AmbientTiming {
                            sample_st: Time::new::<microsecond>(980.0),
                            sample_end: Time::new::<microsecond>(1190.0),
                            reset_st: Time::new::<microsecond>(3700.0),
                            reset_end: Time::new::<microsecond>(3708.0),
                            conv_st: Time::new::<microsecond>(3710.0),
                            conv_end: Time::new::<microsecond>(4190.0),
                        },
                    ),
                    PowerDownTiming {
                        power_down_st: Time::new::<microsecond>(4400.0),
                        power_down_end: Time::new::<microsecond>(49_800.0),
                    },
                ))
                .expect("Cannot set timing window.");

            frontend.set_averaging(8).expect("Cannot set averaging.");
        }
    }

    // Calibration.
    *CALIBRATOR_LED1.lock().unwrap() = Some(calibration::Calibrator::new(
        12000.0,
        || {
            FRONTEND
                .lock()
                .unwrap()
                .as_mut()
                .unwrap()
                .get_led1_current()
                .unwrap()
        },
        |current| {
            FRONTEND
                .lock()
                .unwrap()
                .as_mut()
                .unwrap()
                .set_led1_current(current)
                .unwrap()
        },
        || {
            FRONTEND
                .lock()
                .unwrap()
                .as_mut()
                .unwrap()
                .get_offset_led1_current()
                .unwrap()
        },
        |current| {
            FRONTEND
                .lock()
                .unwrap()
                .as_mut()
                .unwrap()
                .set_offset_led1_current(current)
                .unwrap()
        },
        || {
            FRONTEND
                .lock()
                .unwrap()
                .as_mut()
                .unwrap()
                .get_tia_resistor1()
                .unwrap()
        },
    ));
    *CALIBRATOR_LED2.lock().unwrap() = Some(calibration::Calibrator::new(
        350.0,
        || {
            FRONTEND
                .lock()
                .unwrap()
                .as_mut()
                .unwrap()
                .get_led2_current()
                .unwrap()
        },
        |current| {
            FRONTEND
                .lock()
                .unwrap()
                .as_mut()
                .unwrap()
                .set_led2_current(current)
                .unwrap()
        },
        || {
            FRONTEND
                .lock()
                .unwrap()
                .as_mut()
                .unwrap()
                .get_offset_led2_current()
                .unwrap()
        },
        |current| {
            FRONTEND
                .lock()
                .unwrap()
                .as_mut()
                .unwrap()
                .set_offset_led2_current(current)
                .unwrap()
        },
        || {
            FRONTEND
                .lock()
                .unwrap()
                .as_mut()
                .unwrap()
                .get_tia_resistor2()
                .unwrap()
        },
    ));
    *CALIBRATOR_LED3.lock().unwrap() = Some(calibration::Calibrator::new(
        350.0,
        || {
            FRONTEND
                .lock()
                .unwrap()
                .as_mut()
                .unwrap()
                .get_led3_current()
                .unwrap()
        },
        |current| {
            FRONTEND
                .lock()
                .unwrap()
                .as_mut()
                .unwrap()
                .set_led3_current(current)
                .unwrap()
        },
        || {
            FRONTEND
                .lock()
                .unwrap()
                .as_mut()
                .unwrap()
                .get_offset_led3_current()
                .unwrap()
        },
        |current| {
            FRONTEND
                .lock()
                .unwrap()
                .as_mut()
                .unwrap()
                .set_offset_led3_current(current)
                .unwrap()
        },
        || {
            FRONTEND
                .lock()
                .unwrap()
                .as_mut()
                .unwrap()
                .get_tia_resistor2()
                .unwrap()
        },
    ));

    // Measure accurate offset currents.
    offset_currents.measure();

    // Bluetooth.
    crate::optical::char_control::attach_optical_frontend_chars(
        &FRONTEND,
        &mut ble_api.write().unwrap(),
    );
    crate::optical::char_control::attach_optical_calibration_chars(
        &CALIBRATOR_LED1,
        &CALIBRATOR_LED2,
        &CALIBRATOR_LED3,
        &mut ble_api.write().unwrap(),
    );

    ble_api.read().unwrap().start();
}
