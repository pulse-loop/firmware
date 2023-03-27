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

// If using the `binstart` feature of `esp-idf-sys`, always keep this module imported.
use esp_idf_sys::{self as _, esp_get_free_heap_size, esp_get_free_internal_heap_size};
use static_fir::FirFilter;
use uom::si::{electric_potential::microvolt, f32::ElectricPotential};

mod bluetooth;
mod optical;

fn main() {
    // Temporary. Will disappear once ESP-IDF 4.4 is released, but for now it is necessary to call this function once,
    // or else some patches to the runtime implemented by esp-idf-sys might not link properly.
    esp_idf_sys::link_patches();

    // Initialise logger.
    esp_idf_svc::log::EspLogger::initialize_default();
    log::info!("Logger initialised.");

    let peripherals = Peripherals::take().unwrap();
    let config = Config::new().baudrate(400.kHz().into());

    // Initialise frontend.
    let i2c = I2cDriver::new(
        peripherals.i2c0,
        peripherals.pins.gpio3,
        peripherals.pins.gpio2,
        &config,
    )
    .expect("Failed to initialise I2C bus.");

    let mut interrupt_pin = PinDriver::input(peripherals.pins.gpio4).unwrap();
    let ble_api = Arc::new(RwLock::new(bluetooth::BluetoothAPI::initialise()));

    optical::initialise(i2c, &mut interrupt_pin, ble_api.clone());

    // The latest data that will be sent to the application.
    let latest_data: Arc<Mutex<optical::data_sending::RawData>> =
        Arc::new(Mutex::new(optical::data_sending::RawData::default()));

    let ble_api_for_notify = ble_api;
    let latest_data_for_notify = latest_data.clone();
    thread::spawn(move || {
        optical::data_sending::notify_task(ble_api_for_notify, latest_data_for_notify)
    });

    let builder = thread::Builder::new()
        .name("data_reading".to_string())
        .stack_size(1024 * 10);

    builder
        .spawn(move || {
            let mut dc_filter = [
                FirFilter::<optical::signal_processing::DcFir>::new(),
                FirFilter::<optical::signal_processing::DcFir>::new(),
                FirFilter::<optical::signal_processing::DcFir>::new(),
            ];
            let mut ac_filter = [
                FirFilter::<optical::signal_processing::AcFir>::new(),
                FirFilter::<optical::signal_processing::AcFir>::new(),
                FirFilter::<optical::signal_processing::AcFir>::new(),
            ];
            let mut calibrator = [
                optical::calibration::Calibrator::new(800.0,
                    || {
                        optical::FRONTEND
                            .lock()
                            .unwrap()
                            .as_mut()
                            .unwrap()
                            .get_led1_current()
                            .unwrap()
                    },
                    |current| {
                        optical::FRONTEND
                            .lock()
                            .unwrap()
                            .as_mut()
                            .unwrap()
                            .set_led1_current(current)
                            .unwrap()
                    },
                    || {
                        optical::FRONTEND
                            .lock()
                            .unwrap()
                            .as_mut()
                            .unwrap()
                            .get_offset_led1_current()
                            .unwrap()
                    },
                    |current| {
                        optical::FRONTEND
                            .lock()
                            .unwrap()
                            .as_mut()
                            .unwrap()
                            .set_offset_led1_current(current)
                            .unwrap()
                    },
                    || {
                        optical::FRONTEND
                            .lock()
                            .unwrap()
                            .as_mut()
                            .unwrap()
                            .get_tia_resistor1()
                            .unwrap()
                    },
                ),
                optical::calibration::Calibrator::new(800.0,
                    || {
                        optical::FRONTEND
                            .lock()
                            .unwrap()
                            .as_mut()
                            .unwrap()
                            .get_led2_current()
                            .unwrap()
                    },
                    |current| {
                        optical::FRONTEND
                            .lock()
                            .unwrap()
                            .as_mut()
                            .unwrap()
                            .set_led2_current(current)
                            .unwrap()
                    },
                    || {
                        optical::FRONTEND
                            .lock()
                            .unwrap()
                            .as_mut()
                            .unwrap()
                            .get_offset_led2_current()
                            .unwrap()
                    },
                    |current| {
                        optical::FRONTEND
                            .lock()
                            .unwrap()
                            .as_mut()
                            .unwrap()
                            .set_offset_led2_current(current)
                            .unwrap()
                    },
                    || {
                        optical::FRONTEND
                            .lock()
                            .unwrap()
                            .as_mut()
                            .unwrap()
                            .get_tia_resistor2()
                            .unwrap()
                    },
                ),
                optical::calibration::Calibrator::new(200.0,
                    || {
                        optical::FRONTEND
                            .lock()
                            .unwrap()
                            .as_mut()
                            .unwrap()
                            .get_led3_current()
                            .unwrap()
                    },
                    |current| {
                        optical::FRONTEND
                            .lock()
                            .unwrap()
                            .as_mut()
                            .unwrap()
                            .set_led3_current(current)
                            .unwrap()
                    },
                    || {
                        optical::FRONTEND
                            .lock()
                            .unwrap()
                            .as_mut()
                            .unwrap()
                            .get_offset_led3_current()
                            .unwrap()
                    },
                    |current| {
                        optical::FRONTEND
                            .lock()
                            .unwrap()
                            .as_mut()
                            .unwrap()
                            .set_offset_led3_current(current)
                            .unwrap()
                    },
                    || {
                        optical::FRONTEND
                            .lock()
                            .unwrap()
                            .as_mut()
                            .unwrap()
                            .get_tia_resistor2()
                            .unwrap()
                    },
                ),
            ];
            let mut critical_history = [
                optical::signal_processing::CriticalHistory::new(),
                optical::signal_processing::CriticalHistory::new(),
                optical::signal_processing::CriticalHistory::new(),
            ];
            let mut averaged_data = (0, optical::data_sending::RawData::default());
            optical::data_reading::reading_task(move |raw_data| {
                // Average the data over 10 samples.
                if averaged_data.0 < 10 {
                    averaged_data.0 += 1;
                    averaged_data.1 += raw_data;
                } else {
                    averaged_data.1 /= 10;

                    let mut averaged_data_iterator = averaged_data.1.into_iter();

                    // Skip the ambient light.
                    let ambient = averaged_data_iterator.next().unwrap();

                    // Iterate over the three leds.
                    for (i, led) in averaged_data_iterator.enumerate().take(1) {
                        // Calibrate dc.
                        calibrator[i].calibrate_dc(ElectricPotential::new::<microvolt>(led as f32));

                        // Filter dc data (lowpass).
                        let dc_data = dc_filter[i].feed(led as f32) as i32;

                        // Filter ac data (bandpass).
                        let ac_data = ac_filter[i].feed(led as f32) as i32;

                        // Find critical values
                        // match optical::signal_processing::find_critical_value(ac_data, &mut critical_history[0]) {
                        //     optical::signal_processing::CriticalValue::Maximum(_,_ ) => {
                        //         log::info!("Maximum");
                        //     }
                        //     optical::signal_processing::CriticalValue::Minimum(_,_ ) => {
                        //         log::info!("Minimum");
                        //     }
                        //     optical::signal_processing::CriticalValue::None => {}
                        // }

                        latest_data.lock().unwrap().ambient_reading = led;
                        latest_data.lock().unwrap().led1_reading = dc_data;
                        latest_data.lock().unwrap().led2_reading = ac_data;
                    }

                    // Send data to the application.

                    averaged_data = (0, optical::data_sending::RawData::default());
                };
            })
        })
        .unwrap();

    loop {
        thread::sleep(Duration::from_millis(1000));

        unsafe {
            let x = esp_get_free_heap_size();
            let y = esp_get_free_internal_heap_size();
            log::info!("Free heap: {} bytes, free internal heap: {} bytes", x, y);
        }
    }
}
