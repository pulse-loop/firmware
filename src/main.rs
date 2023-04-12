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
    let latest_raw_data: Arc<Mutex<optical::data_sending::RawData>> =
        Arc::new(Mutex::new(optical::data_sending::RawData::default()));
    let latest_filtered_data: Arc<Mutex<optical::data_sending::FilteredData>> =
        Arc::new(Mutex::new(optical::data_sending::FilteredData::default()));

    let ble_api_for_notify = ble_api;
    let latest_data_for_notify = latest_raw_data.clone();
    let latest_filtered_data_for_notify = latest_filtered_data.clone();
    thread::spawn(move || {
        optical::data_sending::notify_task(
            ble_api_for_notify,
            latest_data_for_notify,
            latest_filtered_data_for_notify,
        )
    });

    let builder = thread::Builder::new()
        .name("data_reading".to_string())
        .stack_size(1024 * 10);

    builder
        .spawn(move || {
            let calibrators: [&Arc<Mutex<Option<optical::calibration::Calibrator>>>; 3] = [
                &optical::CALIBRATOR_LED1,
                &optical::CALIBRATOR_LED2,
                &optical::CALIBRATOR_LED3,
            ];
            let mut dc_filter = [
                FirFilter::<optical::signal_processing::filters::DcFir>::new(),
                FirFilter::<optical::signal_processing::filters::DcFir>::new(),
                FirFilter::<optical::signal_processing::filters::DcFir>::new(),
            ];
            let mut ac_filter = [
                FirFilter::<optical::signal_processing::filters::AcFir>::new(),
                FirFilter::<optical::signal_processing::filters::AcFir>::new(),
                FirFilter::<optical::signal_processing::filters::AcFir>::new(),
            ];
            let mut critical_history = [
                optical::signal_processing::CriticalHistory::new(),
                optical::signal_processing::CriticalHistory::new(),
                optical::signal_processing::CriticalHistory::new(),
            ];
            let mut averaged_data = (0, optical::data_sending::RawData::default());
            let mut frontend_set_up_timer = optical::timer::Timer::new(200); // Corresponds to the time needed, after any change to the frontend settings, for high-accuracy data.
            let mut filter_plus_frontend_set_up_timer = optical::timer::Timer::new(3300 + 200); // Corresponds to the time needed for the filters to settle plus the time needed for high-accuracy data.
            let mut previous_maximum: [Option<(i32, u128)>; 3] = [None; 3];
            optical::data_reading::reading_task(move |raw_data| {
                // Average the data over 10 samples.
                if averaged_data.0 < 1 {
                    averaged_data.0 += 1;
                    averaged_data.1 += raw_data;
                } else {
                    averaged_data.1 /= 1;

                    let mut averaged_data_iterator = averaged_data.1.into_iter();

                    // Skip the ambient light.
                    let ambient = averaged_data_iterator.next().unwrap();

                    // Iterate over the three leds.
                    for (i, led) in averaged_data_iterator.enumerate() {
                        // Calibrate.
                        if calibrators[i]
                            .lock()
                            .unwrap()
                            .as_mut()
                            .unwrap()
                            .calibrate_dc(ElectricPotential::new::<microvolt>(led as f32))
                        {
                            frontend_set_up_timer.reset();
                            filter_plus_frontend_set_up_timer.reset();
                        }

                        // Process data.
                        if frontend_set_up_timer.is_expired() {
                            // TODO: Normalise data (led / (2*R) - ambient / (2*R)).

                            // Filter dc data (lowpass).
                            let dc_data = dc_filter[i].feed(led as f32) as i32;

                            // Filter ac data (bandpass).
                            let ac_data = ac_filter[i].feed(led as f32) as i32;

                            // Find critical values
                            if filter_plus_frontend_set_up_timer.is_expired() {
                                match optical::signal_processing::find_critical_value(
                                    ac_data,
                                    &mut critical_history[i],
                                ) {
                                    optical::signal_processing::CriticalValue::Maximum(
                                        amplitude,
                                        time,
                                    ) => {
                                        if let Some(previous_maximum) = previous_maximum[i] {
                                            let rr = time - previous_maximum.1;
                                            log::info!("RR{}: {} ms", i, rr);
                                        }
                                        previous_maximum[i] = Some((amplitude, time));
                                    }
                                    optical::signal_processing::CriticalValue::Minimum(
                                        amplitude,
                                        _time,
                                    ) => {
                                        if let Some(previous_maximum) = previous_maximum[i] {
                                            let ac = previous_maximum.0 - amplitude;
                                            let dc = dc_data;
                                            log::info!("AC{}: {} DC{}: {}", i, ac, i, dc);
                                        }
                                    }
                                    optical::signal_processing::CriticalValue::None => {}
                                }
                            }

                            // Send filtered data to the application.
                            latest_filtered_data.lock().unwrap()[i] = (dc_data, ac_data);
                        }
                    }

                    // Send raw data to the application.
                    *latest_raw_data.lock().unwrap() = averaged_data.1;

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
