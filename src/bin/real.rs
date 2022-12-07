use std::{thread, time::Duration};

use esp_idf_hal::{
    gpio::PinDriver,
    i2c::{config::Config, I2cDriver},
    peripherals::Peripherals,
    prelude::*,
};

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

// If using the `binstart` feature of `esp-idf-sys`, always keep this module imported.
use esp_idf_sys::{self as _, esp_get_free_heap_size, esp_get_free_internal_heap_size};

#[path = "../bluetooth/mod.rs"]
mod bluetooth;
#[path = "../optical/mod.rs"]
mod optical;

use optical::data_reading::{get_sample_blocking, DATA_READY};

fn main() {
    // Temporary. Will disappear once ESP-IDF 4.4 is released, but for now it is necessary to call this function once,
    // or else some patches to the runtime implemented by esp-idf-sys might not link properly.
    esp_idf_sys::link_patches();

    // Initialise logger.
    esp_idf_svc::log::EspLogger::initialize_default();
    log::info!("Logger initialised.");

    let peripherals = Peripherals::take().unwrap();
    let config = Config::new().baudrate(100.kHz().into());

    let i2c = I2cDriver::new(
        peripherals.i2c0,
        peripherals.pins.gpio3,
        peripherals.pins.gpio2,
        &config,
    )
    .expect("Failed to initialize I2C bus.");

    let mut interrupt_pin = PinDriver::input(peripherals.pins.gpio4).unwrap();

    let mut frontend = AFE4404::with_three_leds(i2c, 0x58u8, Frequency::new::<megahertz>(4.0));
    let ble_api = bluetooth::BluetoothAPI::initialise();

    frontend.sw_reset().expect("Cannot reset the afe");

    frontend
        .set_clock_source(ClockConfiguration::Internal)
        .expect("Cannot set clock source");

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
        .expect("Cannot set timing window");

    interrupt_pin
        .set_interrupt_type(esp_idf_hal::gpio::InterruptType::PosEdge)
        .unwrap();

    unsafe {
        interrupt_pin
            .subscribe(|| {
                DATA_READY.store(true, std::sync::atomic::Ordering::Relaxed);
            })
            .unwrap();
    }

    thread::spawn(|| loop {
        thread::sleep(Duration::from_millis(500));

        unsafe {
            let x = esp_get_free_heap_size();
            let y = esp_get_free_internal_heap_size();
            log::info!("Free heap: {} bytes, free internal heap: {} bytes", x, y);
        }
    });

    ble_api.start();

    loop {
        std::thread::sleep(std::time::Duration::from_millis(10));
        let readings = get_sample_blocking(&mut frontend, 5);
        match readings {
            Ok(readings) => {
                ble_api
                    .raw_sensor_data
                    .ambient_reading_characteristic
                    .write()
                    .unwrap()
                    .set_value(readings.ambient().value.to_le_bytes());
                ble_api
                    .raw_sensor_data
                    .led1_minus_ambient_reading_characteristic
                    .write()
                    .unwrap()
                    .set_value(readings.led1_minus_ambient().value.to_le_bytes());
                ble_api
                    .raw_sensor_data
                    .led1_reading_characteristic
                    .write()
                    .unwrap()
                    .set_value(readings.led1().value.to_le_bytes());
                ble_api
                    .raw_sensor_data
                    .led2_reading_characteristic
                    .write()
                    .unwrap()
                    .set_value(readings.led2().value.to_le_bytes());
                ble_api
                    .raw_sensor_data
                    .led3_reading_characteristic
                    .write()
                    .unwrap()
                    .set_value(readings.led3().value.to_le_bytes());
            }
            Err(e) => println!("Error: {:?}", e),
        }
    }
}
