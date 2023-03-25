use std::sync::{Arc, Mutex, RwLock};

use esp_idf_hal::{
    gpio::{Input, Pin, PinDriver},
    i2c::I2cDriver,
};

use uom::si::{
    electric_current::milliampere,
    f32::{ElectricCurrent, Frequency, Time},
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

lazy_static::lazy_static! {
    pub static ref FRONTEND: Arc<Mutex<Option<AFE4404<I2cDriver<'static>, ThreeLedsMode>>>> = Arc::new(Mutex::new(None));
}

/// Initialises the `FRONTEND` with default values.
pub fn initialise<P: Pin>(
    i2c: I2cDriver<'static>,
    interrupt_pin: &mut PinDriver<P, Input>,
    ble_api: Arc<RwLock<BluetoothAPI>>,
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
                .set_clock_source(ClockConfiguration::Internal)
                .expect("Cannot set clock source.");

            frontend
                .set_measurement_window(&MeasurementWindowConfiguration::<ThreeLedsMode>::new(
                    Time::new::<microsecond>(10_000.0),
                    ActiveTiming::<ThreeLedsMode>::new(
                        LedTiming {
                            lighting_st: Time::new::<microsecond>(200.5),
                            lighting_end: Time::new::<microsecond>(300.25),
                            sample_st: Time::new::<microsecond>(225.5),
                            sample_end: Time::new::<microsecond>(300.25),
                            reset_st: Time::new::<microsecond>(634.75),
                            reset_end: Time::new::<microsecond>(636.25),
                            conv_st: Time::new::<microsecond>(636.75),
                            conv_end: Time::new::<microsecond>(901.5),
                        },
                        LedTiming {
                            lighting_st: Time::new::<microsecond>(0.0),
                            lighting_end: Time::new::<microsecond>(99.75),
                            sample_st: Time::new::<microsecond>(25.0),
                            sample_end: Time::new::<microsecond>(99.75),
                            reset_st: Time::new::<microsecond>(100.25),
                            reset_end: Time::new::<microsecond>(101.75),
                            conv_st: Time::new::<microsecond>(102.25),
                            conv_end: Time::new::<microsecond>(367.0),
                        },
                        LedTiming {
                            lighting_st: Time::new::<microsecond>(100.25),
                            lighting_end: Time::new::<microsecond>(200.0),
                            sample_st: Time::new::<microsecond>(125.25),
                            sample_end: Time::new::<microsecond>(200.0),
                            reset_st: Time::new::<microsecond>(367.5),
                            reset_end: Time::new::<microsecond>(369.0),
                            conv_st: Time::new::<microsecond>(369.5),
                            conv_end: Time::new::<microsecond>(634.25),
                        },
                        AmbientTiming {
                            sample_st: Time::new::<microsecond>(325.75),
                            sample_end: Time::new::<microsecond>(400.5),
                            reset_st: Time::new::<microsecond>(902.0),
                            reset_end: Time::new::<microsecond>(903.5),
                            conv_st: Time::new::<microsecond>(904.0),
                            conv_end: Time::new::<microsecond>(1168.75),
                        },
                    ),
                    PowerDownTiming {
                        power_down_st: Time::new::<microsecond>(1368.75),
                        power_down_end: Time::new::<microsecond>(9799.75),
                    },
                ))
                .expect("Cannot set timing window.");
        }
    }

    // Bluetooth.
    crate::optical::char_control::attach_optical_frontend_chars(
        &FRONTEND,
        &mut ble_api.write().unwrap(),
    );

    ble_api.read().unwrap().start();
}
