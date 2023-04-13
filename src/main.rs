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
use uom::si::{electrical_resistance::kiloohm, f32::ElectricalResistance};

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
            let mut rr_average_filter = [
                FirFilter::<optical::signal_processing::filters::AverageFir>::new(),
                FirFilter::<optical::signal_processing::filters::AverageFir>::new(),
                FirFilter::<optical::signal_processing::filters::AverageFir>::new(),
            ];

            let mut critical_history = [
                optical::signal_processing::CriticalHistory::new(),
                optical::signal_processing::CriticalHistory::new(),
                optical::signal_processing::CriticalHistory::new(),
            ];
            let mut previous_maximum: [Option<(f32, u128)>; 3] = [None; 3];

            let mut frontend_set_up_timer = optical::timer::Timer::new(200); // Corresponds to the time needed, after any change to the frontend settings, for high-accuracy data.
            let mut filter_plus_frontend_set_up_timer = optical::timer::Timer::new(85 * 50 + 200); // Corresponds to the time needed for the filters to settle plus the time needed for high-accuracy data.

            optical::data_reading::reading_task(move |raw_data| {
                let mut raw_data_iterator = raw_data.into_iter();

                // Skip the ambient light.
                let ambient = raw_data_iterator.next().unwrap();

                // Iterate over the three leds.
                for (i, led) in raw_data_iterator.enumerate() {
                    // Calibrate.
                    if calibrators[i]
                        .lock()
                        .unwrap()
                        .as_mut()
                        .unwrap()
                        .calibrate_dc(led)
                    {
                        frontend_set_up_timer.reset();
                        filter_plus_frontend_set_up_timer.reset();
                    }

                    // Process data.
                    if frontend_set_up_timer.is_expired() {
                        // Convert the data into current and remove the ambient light.
                        let photodiode_current = led
                            / (2.0 * ElectricalResistance::new::<kiloohm>(1000.0))
                            - calibrators[i]
                                .lock()
                                .unwrap()
                                .as_mut()
                                .unwrap()
                                .offset_current;
                        let ambient_current =
                            ambient / (2.0 * ElectricalResistance::new::<kiloohm>(1000.0));
                        let refined_current = (photodiode_current - ambient_current).value;

                        // Filter dc data (lowpass).
                        let dc_data = dc_filter[i].feed(refined_current);

                        // Filter ac data (bandpass).
                        let ac_data = ac_filter[i].feed(refined_current);

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
                                        if rr > 250 && rr < 2000 {
                                            let averaged_rr = rr_average_filter[i].feed(rr as f32);
                                            if i == 0 {
                                                log::info!(
                                                    "BPM: {}, RR: {} ms",
                                                    60_000.0 / averaged_rr,
                                                    averaged_rr,
                                                );
                                            }
                                        } else {
                                            log::error!("RR{}: {} ms", i, rr);
                                        }
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
                                        let perfusion_index = ac / dc;
                                        log::info!("PI{}: {}", i, perfusion_index);
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
                *latest_raw_data.lock().unwrap() = raw_data;
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
