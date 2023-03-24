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

    thread::spawn(move || {
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
            optical::calibration::Calibrator::new(),
            optical::calibration::Calibrator::new(),
            optical::calibration::Calibrator::new(),
        ];
        let mut critical_history = [
            optical::signal_processing::CriticalHistory::new(),
            optical::signal_processing::CriticalHistory::new(),
            optical::signal_processing::CriticalHistory::new(),
        ];
        let mut average = (0, optical::data_sending::RawData::default());
        log::info!("Started data reading task.");
        optical::data_reading::reading_task(move |raw_data| {
            // Average the data over 10 samples.
            log::info!("0 - Start");
            if average.0 < 10 {
                average.0 += 1;
                average.1 += raw_data;
            } else {
                average.1 /= 10;
                log::info!("1 - Average");

                let mut average_iterator = average.1.into_iter();

                // Skip the ambient light.
                let ambient = average_iterator.next().unwrap();
                log::info!("2 - Ambient");

                // Iterate over the three leds.
                for led in average_iterator {
                    log::info!("3 - For");
                    // Calibrate dc.
                    calibrator[0].calibrate_dc(led as i64);
                    log::info!("4 - Calibrate DC");

                    // Filter dc data (lowpass).
                    let dc_data = dc_filter[0].feed(led as f32) as i32;
                    log::info!("5 - Filter DC");

                    // Filter ac data (bandpass).
                    let ac_data = ac_filter[0].feed(led as f32) as i32;
                    log::info!("6 - Filter AC");

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
                }
                // Send data to the application.
                // *latest_data.lock().unwrap() = average.1;

                average = (0, optical::data_sending::RawData::default());
            };
        })
    });

    loop {
        thread::sleep(Duration::from_millis(1000));

        unsafe {
            let x = esp_get_free_heap_size();
            let y = esp_get_free_internal_heap_size();
            log::info!("Free heap: {} bytes, free internal heap: {} bytes", x, y);
        }
    }
}
