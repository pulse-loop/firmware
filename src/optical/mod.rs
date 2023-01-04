use esp_idf_hal::i2c::I2cDriver;

use uom::si::{
    f32::{Frequency, Time},
    frequency::megahertz,
    time::microsecond,
};

use afe4404::{
    device::AFE4404,
    modes::ThreeLedsMode,
    {
        clock::ClockConfiguration,
        measurement_window::{
            ActiveTiming, AmbientTiming, LedTiming, MeasurementWindowConfiguration, PowerDownTiming,
        },
    },
};

pub(crate) mod char_control;
pub(crate) mod data_reading;
pub(crate) mod data_sending;

/// Initialises the `FRONTEND` with default values.
pub fn initialise_afe4404(i2c: I2cDriver<'static>) {
    *crate::FRONTEND.lock().unwrap() = Some(AFE4404::with_three_leds(
        i2c,
        0x58u8,
        Frequency::new::<megahertz>(4.0),
    ));

    crate::FRONTEND
        .lock()
        .unwrap()
        .as_mut()
        .unwrap()
        .sw_reset()
        .expect("Cannot reset the afe.");

    crate::FRONTEND
        .lock()
        .unwrap()
        .as_mut()
        .unwrap()
        .set_clock_source(ClockConfiguration::Internal)
        .expect("Cannot set clock source.");

    crate::FRONTEND
        .lock()
        .unwrap()
        .as_mut()
        .unwrap()
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
