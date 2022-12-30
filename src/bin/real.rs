use std::{
    sync::{Arc, Mutex, RwLock},
    thread,
    time::Duration,
};

use esp_idf_hal::{
    gpio::PinDriver,
    i2c::{config::Config, I2cDriver},
    peripherals::Peripherals,
    prelude::*,
};

use uom::si::{
    electric_potential::volt,
    f32::{ElectricPotential, Frequency, Time},
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

lazy_static::lazy_static! {
    static ref FRONTEND: Arc<Mutex<Option<AFE4404<I2cDriver<'static>, ThreeLedsMode>>>> = Arc::new(Mutex::new(None));
}

#[path = "../bluetooth/mod.rs"]
mod bluetooth;
#[path = "../optical/mod.rs"]
mod optical;
use optical::data_reading::{get_sample, DATA_READY};

fn main() {
    // Temporary. Will disappear once ESP-IDF 4.4 is released, but for now it is necessary to call this function once,
    // or else some patches to the runtime implemented by esp-idf-sys might not link properly.
    esp_idf_sys::link_patches();

    // Initialise logger.
    esp_idf_svc::log::EspLogger::initialize_default();
    log::info!("Logger initialised.");

    let peripherals = Peripherals::take().unwrap();
    let config = Config::new().baudrate(400.kHz().into());

    let i2c = I2cDriver::new(
        peripherals.i2c0,
        peripherals.pins.gpio3,
        peripherals.pins.gpio2,
        &config,
    )
    .expect("Failed to initialize I2C bus.");

    let mut interrupt_pin = PinDriver::input(peripherals.pins.gpio4).unwrap();

    let frontend = AFE4404::with_three_leds(i2c, 0x58u8, Frequency::new::<megahertz>(4.0));

    *FRONTEND.lock().unwrap() = Some(frontend);

    let ble_api = Arc::new(RwLock::new(bluetooth::BluetoothAPI::initialise()));

    FRONTEND
        .lock()
        .unwrap()
        .as_mut()
        .unwrap()
        .sw_reset()
        .expect("Cannot reset the afe");

    FRONTEND
        .lock()
        .unwrap()
        .as_mut()
        .unwrap()
        .set_clock_source(ClockConfiguration::Internal)
        .expect("Cannot set clock source");

    FRONTEND
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
        thread::sleep(Duration::from_millis(1000));

        unsafe {
            let x = esp_get_free_heap_size();
            let y = esp_get_free_internal_heap_size();
            log::info!("Free heap: {} bytes, free internal heap: {} bytes", x, y);
        }
    });

    ble_api.read().unwrap().start();

    crate::optical::char_control::attach_optical_frontend_chars(
        &FRONTEND,
        &mut ble_api.write().unwrap(),
    );

    let averaged_readings: Arc<Mutex<[ElectricPotential; 5]>> =
        Arc::new(Mutex::new([ElectricPotential::new::<volt>(0.0); 5]));
    let n = Arc::new(Mutex::new(0));


    let ble_api_for_notify = ble_api;
    let averaged_readings_for_notify = averaged_readings.clone();
    let n_for_notify = n.clone();
    thread::spawn(move || {
        let mut time = std::time::Instant::now();
        loop {
            thread::sleep(Duration::from_millis(10));

            if time.elapsed().as_millis() > 50 && *n_for_notify.lock().unwrap() > 0 {
                if let (Ok(ble_api), Ok(mut n), Ok(mut averaged_readings)) = (
                    ble_api_for_notify.write(),
                    n_for_notify.lock(),
                    averaged_readings_for_notify.lock(),
                ) {
                    ble_api
                        .raw_sensor_data
                        .ambient_reading_characteristic
                        .write()
                        .unwrap()
                        .set_value((averaged_readings[0] / (*n as f32)).value.to_le_bytes());
                    ble_api
                        .raw_sensor_data
                        .led1_minus_ambient_reading_characteristic
                        .write()
                        .unwrap()
                        .set_value((averaged_readings[1] / (*n as f32)).value.to_le_bytes());
                    ble_api
                        .raw_sensor_data
                        .led1_reading_characteristic
                        .write()
                        .unwrap()
                        .set_value((averaged_readings[2] / (*n as f32)).value.to_le_bytes());
                    ble_api
                        .raw_sensor_data
                        .led2_reading_characteristic
                        .write()
                        .unwrap()
                        .set_value((averaged_readings[3] / (*n as f32)).value.to_le_bytes());
                    ble_api
                        .raw_sensor_data
                        .led3_reading_characteristic
                        .write()
                        .unwrap()
                        .set_value((averaged_readings[4] / (*n as f32)).value.to_le_bytes());

                    *averaged_readings = [ElectricPotential::new::<volt>(0.0); 5];
                    *n = 0;
                    time = std::time::Instant::now();
                }
            }
        }
    });

    loop {
        thread::sleep(Duration::from_millis(1));

        let averaged_readings_for_read = averaged_readings.clone();
        let n_for_read = n.clone();

        get_sample(
            FRONTEND.lock().unwrap().as_mut().unwrap(),
            move |readings| {
                if let (Ok(mut n), Ok(mut averaged_readings)) = (n_for_read.lock(), averaged_readings_for_read.lock())
                {
                    *n += 1;
                    averaged_readings[0] += *readings.ambient();
                    averaged_readings[1] += *readings.led1_minus_ambient();
                    averaged_readings[2] += *readings.led1();
                    averaged_readings[3] += *readings.led2();
                    averaged_readings[4] += *readings.led3();
                }
            },
        );
    }
}
