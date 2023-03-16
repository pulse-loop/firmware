use uom::si::{electric_current::nanoampere, f32::ElectricCurrent};

pub(crate) struct Calibration {
    // Voltages are expressed in microvolts, currents in nanoamperes and resistors in kiloohms.
    // Afe4404 values.
    resistor: i32,
    led_current: i32,
    led_min_current: i32,
    led_max_current: i32,
    offset_current: i32,
    offset_min_current: i32,
    offset_max_current: i32,

    // Dc calibration.
    alpha: i32,
    set_point: i32,
    working_threshold: i32,
}

impl Calibration {
    pub(crate) fn new() -> Self {
        Calibration {
            // TODO: Change to optimal initial value.
            resistor: 10,                 // kΩ
            led_current: 0,               // nA
            led_min_current: 0,           // nA
            led_max_current: 100_000_000, // nA
            offset_current: 0,            // nA
            offset_min_current: -7_000,   // nA
            offset_max_current: 7_000,    // nA
            alpha: 40_000,
            set_point: 0,               // µV
            working_threshold: 800_000, // µV
        }
    }

    pub(crate) fn calibrate_dc(&mut self, data: i32) {
        log::info!("1 - Calibrating DC: {} µV", data);
        if data.abs() > self.working_threshold {
            let error = (self.set_point - data) / (2 * self.resistor);
            log::info!("2 - Error: {} nA", error);

            // if let Ok(led_current) = frontend.set_led1_current(ElectricCurrent::new::<nanoampere>(
            //     (self.led_current + error) as f32,
            // )) {
            //     self.led_current += led_current.get::<nanoampere>() as i32;
            //     log::info!("Led current: {} nA", self.led_current);
            // } else {
            //     log::error!("Failed to set led current.");
            // }
            // log::info!("3 - END");
        }
        log::info!("4 - END");
    }
}
