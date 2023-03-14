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
use esp_idf_sys::{
    self as _, adc_channel_t_ADC_CHANNEL_5, esp_get_free_heap_size, esp_get_free_internal_heap_size,
};
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

    let mut dc_filter = FirFilter::<optical::signal_processing::DcFir>::new();
    let mut ac_filter = FirFilter::<optical::signal_processing::AcFir>::new();
    let mut average = (0, 0);
    thread::spawn(move || {
        optical::data_reading::reading_task(move |raw| {
            if average.0 < 10 {
                average.0 += 1;
                average.1 += raw.led1_reading;
            } else {
                average.1 /= average.0;

                // Calibrate dc.
                optical::calibration::dc_calibration(average.1);

                // Filter dc data (lowpass).
                let dc_data = dc_filter.feed(average.1 as f32) as i32;

                // Filter ac data (bandpass).
                let ac_data = ac_filter.feed(average.1 as f32) as i32;

                // Send data to the application.
                // *latest_data.lock().unwrap() = raw;
                latest_data.lock().unwrap().led1_reading = average.1;
                latest_data.lock().unwrap().led2_reading = dc_data;
                latest_data.lock().unwrap().led3_reading = ac_data;

                average = (0, 0);
            }
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
