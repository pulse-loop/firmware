use std::{
    sync::{Arc, Mutex, RwLock},
    thread,
    time::Duration,
};

use afe4404::{led_current::LedCurrentConfiguration, modes::ThreeLedsMode};
use esp_idf_hal::{
    gpio::PinDriver,
    i2c::{config::Config, I2cDriver},
    peripherals::Peripherals,
    prelude::*,
};

// If using the `binstart` feature of `esp-idf-sys`, always keep this module imported.
use esp_idf_sys::{self as _, esp_get_free_heap_size, esp_get_free_internal_heap_size};

use static_fir::FirFilter;
use uom::si::{
    electric_current::{microampere, milliampere},
    electrical_resistance::ohm,
    f32::{ElectricCurrent, ElectricalResistance},
};

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
    let mut offset_currents = optical::calibration::offset_measuring::OffsetCurrents::new();

    optical::initialise(
        i2c,
        &mut interrupt_pin,
        ble_api.clone(),
        &mut offset_currents,
    );

    // The latest data that will be sent to the application.
    let latest_raw_data: Arc<Mutex<optical::data_sending::RawData>> =
        Arc::new(Mutex::new(optical::data_sending::RawData::default()));
    let latest_filtered_data: Arc<Mutex<optical::data_sending::FilteredData>> =
        Arc::new(Mutex::new(optical::data_sending::FilteredData::default()));
    let latest_results: Arc<Mutex<optical::data_sending::Results>> =
        Arc::new(Mutex::new(optical::data_sending::Results::default()));

    let ble_api_for_notify = ble_api.clone();
    let latest_data_for_notify = latest_raw_data.clone();
    let latest_filtered_data_for_notify = latest_filtered_data.clone();
    let latest_results_for_notify = latest_results.clone();
    thread::spawn(move || {
        optical::data_sending::notify_task(
            ble_api_for_notify,
            latest_data_for_notify,
            latest_filtered_data_for_notify,
            latest_results_for_notify,
        )
    });

    let builder = thread::Builder::new()
        .name("data_reading".to_string())
        .stack_size(1024 * 16);

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

            let mut hr_median_filter: median::Filter<u128> = median::Filter::new(21);
            let mut r_median_filter: median::Filter<f32> = median::Filter::new(21);

            let mut critical_history = optical::signal_processing::CriticalHistory::new();
            let mut previous_maximum: Option<(f32, u128)> = None;

            let mut frontend_set_up_timer = optical::timer::Timer::new(200); // Corresponds to the time needed, after any change to the frontend settings, for high-accuracy data.
            let mut filter_plus_frontend_set_up_timer =
                optical::timer::Timer::new(85 * 50 + 200 + 200); // Corresponds to the time needed for the filters to settle plus the time needed for high-accuracy data.
            let mut threshold_timer = optical::timer::Timer::new(5000); // The timer that resets the crossing threshold.

            let mut ir_offset_current = ElectricCurrent::new::<microampere>(-7.0);
            let resistors = [
                ElectricalResistance::new::<ohm>(optical::RESISTOR1),
                ElectricalResistance::new::<ohm>(optical::RESISTOR2),
                ElectricalResistance::new::<ohm>(optical::RESISTOR2),
            ];

            let mut green_deviation =
                crate::optical::signal_processing::standard_deviation::MovingStandardDeviation::new(
                    200,
                );
            let mut red_deviation =
                crate::optical::signal_processing::standard_deviation::MovingStandardDeviation::new(
                    200,
                );
            let mut ir_deviation =
                crate::optical::signal_processing::standard_deviation::MovingStandardDeviation::new(
                    200,
                );

            let mut r = 0.0;
            let mut i = 0;

            optical::data_reading::reading_task(move |raw_data| {
                let mut raw_data_iterator = raw_data.into_iter();

                // Read the ambient light.
                let ambient = raw_data_iterator.next().unwrap();
                let ambient_current =
                    ambient / (2.0 * ElectricalResistance::new::<ohm>(optical::RESISTOR1));

                // Read the IR LED (LED 3).
                let ir = raw_data.led3;
                let ir_current = ir / (2.0 * ElectricalResistance::new::<ohm>(optical::RESISTOR2))
                    - offset_currents.accurate(ir_offset_current)
                    - ambient_current;

                // Check if the wrist is present with the IR LED (LED 3) and the ambient light.
                if ambient_current < ElectricCurrent::new::<microampere>(1.0)
                    && ir_current > ElectricCurrent::new::<microampere>(10.0)
                {
                    // Wrist is present.
                    latest_results.lock().unwrap().wrist_presence = true;

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
                            let offset_current = offset_currents.accurate(
                                calibrators[i]
                                    .lock()
                                    .unwrap()
                                    .as_mut()
                                    .unwrap()
                                    .offset_current,
                            );
                            let photodiode_current = led / (2.0 * resistors[i]) - offset_current;
                            let refined_current = photodiode_current - ambient_current;

                            // Update IR offset current for wrist detection.
                            // This is done in order to avoid reading the offset from the AFE4404.
                            if i == 2 {
                                ir_offset_current = offset_current;
                            }

                            // Filter dc data (lowpass).
                            let dc_data = dc_filter[i].feed(refined_current.value);

                            // Filter ac data (bandpass).
                            let ac_data = ac_filter[i].feed(refined_current.value);

                            // Send filtered data to the application.
                            latest_filtered_data.lock().unwrap()[i] = (dc_data, ac_data);
                        }
                    }

                    if filter_plus_frontend_set_up_timer.is_expired() {
                        // SpO2 calculation.
                        let (
                            green_ac_amplitude,
                            green_dc_amplitude,
                            red_ac_amplitude,
                            red_dc_amplitude,
                            ir_ac_amplitude,
                            ir_dc_amplitude,
                        ) = if let Ok(filtered_data) = latest_filtered_data.lock() {
                            (
                                green_deviation.push(filtered_data[0].1),
                                filtered_data[0].0,
                                red_deviation.push(filtered_data[1].1),
                                filtered_data[1].0,
                                ir_deviation.push(filtered_data[2].1),
                                filtered_data[2].0,
                            )
                        } else {
                            (0.0, 0.0, 0.0, 0.0, 0.0, 0.0)
                        };

                        if let Ok(mut results) = latest_results.lock() {
                            results.green_pi = green_ac_amplitude / green_dc_amplitude * 100.0;
                            results.red_pi = red_ac_amplitude / red_dc_amplitude * 100.0;
                            results.ir_pi = ir_ac_amplitude / ir_dc_amplitude * 100.0;

                            r += results.red_pi / results.ir_pi;
                            i += 1;

                            if i == 10 {
                                results.spo2 = r / 10.0;
                                // results.spo2 = r_median_filter.consume(r / 10.0);

                                r = 0.0;
                                i = 0;
                                log::info!("R: {}", results.spo2);
                            }
                        }

                        // Heart rate calculation.
                        if threshold_timer.is_expired() {
                            critical_history.crossing_threshold = 0.0;
                        }
                        match optical::signal_processing::find_critical_value(
                            latest_filtered_data.lock().unwrap()[0].1,
                            &mut critical_history,
                        ) {
                            optical::signal_processing::CriticalValue::Maximum(amplitude, time) => {
                                if let Some(previous_maximum) = previous_maximum {
                                    let rr = time - previous_maximum.1;
                                    if rr > 250 && rr < 2000 {
                                        let rr = hr_median_filter.consume(rr);
                                        let heart_rate = 60_000.0 / rr as f32;
                                        ble_api
                                            .write()
                                            .unwrap()
                                            .results
                                            .heart_rate_characteristic
                                            .write()
                                            .unwrap()
                                            .set_value(heart_rate.to_le_bytes());
                                        log::info!("HR: {} bpm", heart_rate);
                                    } else {
                                        log::error!("Wrong RR: {} ms", rr);
                                    }
                                }

                                previous_maximum = Some((amplitude, time));
                            }
                            optical::signal_processing::CriticalValue::Minimum(
                                amplitude,
                                _time,
                            ) => {
                                if let Some(previous_maximum) = previous_maximum {
                                    let ac = previous_maximum.0 - amplitude;

                                    // Update crossing threshold.
                                    critical_history.crossing_threshold = -ac * 0.15;
                                    threshold_timer.reset();
                                }
                            }
                            optical::signal_processing::CriticalValue::None => {}
                        }
                    }
                } else {
                    // Writst is not present.
                    latest_results.lock().unwrap().wrist_presence = false;
                    log::info!("Wrist not detected.");

                    // Reset the critical history crossing threshold.
                    critical_history.crossing_threshold = 0.0;

                    // Turn off the LEDs, wait for some time then check wrist presence with IR LED.
                    optical::FRONTEND
                        .lock()
                        .unwrap()
                        .as_mut()
                        .unwrap()
                        .set_leds_current(&LedCurrentConfiguration::<ThreeLedsMode>::new(
                            ElectricCurrent::new::<milliampere>(0.0),
                            ElectricCurrent::new::<milliampere>(0.0),
                            ElectricCurrent::new::<milliampere>(0.0),
                        ))
                        .expect("Cannot turn off LEDs.");

                    thread::sleep(Duration::from_millis(2000));

                    // Turn on the IR LED and set the offset current.
                    let mut ir_max_current = Default::default();
                    let mut ir_min_offset_current = Default::default();
                    if let Ok(mut ir_calibrator) = calibrators[2].lock() {
                        if let Some(ir_calibrator) = ir_calibrator.as_mut() {
                            ir_max_current = *ir_calibrator.led_current_max();
                            ir_min_offset_current = *ir_calibrator.offset_current_min();
                        }
                    };
                    if let Ok(mut frontend) = optical::FRONTEND.lock() {
                        if let Some(frontend) = frontend.as_mut() {
                            frontend
                                .set_led3_current(ir_max_current)
                                .expect("Cannot turn on LED3.");
                            frontend
                                .set_offset_led3_current(ir_min_offset_current)
                                .expect("Cannot set LED3 offset current.");
                        }
                    }
                    ir_offset_current = ir_min_offset_current;

                    // Wait for the IR LED to turn on.
                    thread::sleep(Duration::from_millis(200));
                }

                // Send raw data to the application.
                *latest_raw_data.lock().unwrap() = raw_data;
                if let Ok(mut thresholds) = latest_filtered_data.lock() {
                    thresholds.led1_threshold = critical_history.crossing_threshold;
                }
            })
        })
        .unwrap();

    loop {
        thread::sleep(Duration::from_millis(1000));

        unsafe {
            let x = esp_get_free_heap_size();
            let y = esp_get_free_internal_heap_size();
            // log::info!("Free heap: {} bytes, free internal heap: {} bytes", x, y);
        }
    }
}
