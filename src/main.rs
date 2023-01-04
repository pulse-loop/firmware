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

use uom::si::{electric_potential::volt, f32::ElectricPotential};

use afe4404::{device::AFE4404, modes::ThreeLedsMode};

// If using the `binstart` feature of `esp-idf-sys`, always keep this module imported.
use esp_idf_sys::{self as _, esp_get_free_heap_size, esp_get_free_internal_heap_size};

lazy_static::lazy_static! {
    static ref FRONTEND: Arc<Mutex<Option<AFE4404<I2cDriver<'static>, ThreeLedsMode>>>> = Arc::new(Mutex::new(None));
}

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

    let i2c = I2cDriver::new(
        peripherals.i2c0,
        peripherals.pins.gpio3,
        peripherals.pins.gpio2,
        &config,
    )
    .expect("Failed to initialise I2C bus.");

    let mut interrupt_pin = PinDriver::input(peripherals.pins.gpio4).unwrap();

    let ble_api = Arc::new(RwLock::new(bluetooth::BluetoothAPI::initialise()));

    optical::initialise_afe4404(i2c);

    interrupt_pin
        .set_interrupt_type(esp_idf_hal::gpio::InterruptType::PosEdge)
        .unwrap();

    unsafe {
        interrupt_pin
            .subscribe(|| {
                optical::data_reading::DATA_READY.store(true, std::sync::atomic::Ordering::Relaxed);
            })
            .unwrap();
    }

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
        optical::data_sending::notify_with_averaged_readings_loop(
            ble_api_for_notify,
            averaged_readings_for_notify,
            n_for_notify,
        )
    });

    thread::spawn(move || optical::data_reading::get_averaged_readings_loop(averaged_readings, n));

    loop {
        thread::sleep(Duration::from_millis(1000));

        unsafe {
            let x = esp_get_free_heap_size();
            let y = esp_get_free_internal_heap_size();
            log::info!("Free heap: {} bytes, free internal heap: {} bytes", x, y);
        }
    }
}
