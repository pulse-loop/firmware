use uom::si::{
    electric_current::{milliampere, nanoampere},
    electrical_resistance::kiloohm,
    f32::ElectricCurrent,
};

pub(crate) struct Calibration {
    // Voltages are expressed in microvolts, currents in nanoamperes and resistors in kiloohms.

    // Afe4404 values.
    led_current: i64, // Cached value of the led current.
    led_min_current: i64,
    led_max_current: i64,
    offset_current: i64, // Cached value of the offset current.
    offset_min_current: i64,
    offset_max_current: i64,

    // Dc calibration.
    alpha: i64, // Skin coefficient, alpha = i_led / i_photodiode.
    set_point: i64,
    working_threshold: i64,
}

impl Calibration {
    pub(crate) fn new() -> Self {
        Calibration {
            // TODO: Change to optimal initial value.
            led_current: 0,               // nA.
            led_min_current: 0,           // nA.
            led_max_current: 100_000_000, // nA.
            offset_current: 0,            // nA.
            offset_min_current: -7_000,   // nA.
            offset_max_current: 7_000,    // nA.
            alpha: 400,
            set_point: 0,               // µV.
            working_threshold: 800_000, // µV.
        }
    }

    // Calibrates the DC component of the signal by changing the LED current and the offset current.
    // The calibration is split proportionally between the LED current and the offset current.
    // TODO: Handle unwrap fails.
    pub(crate) fn calibrate_dc(&mut self, sample: i64) {
        // Calibrate only if the sample is out of the working threshold.
        if sample.abs() > self.working_threshold {
            // Acquire the frontend lock.
            if let Ok(mut frontend) = super::FRONTEND.lock() {
                if let Some(frontend) = frontend.as_mut() {
                    // The error between the set point and the sample converted in the current seen by the photodiode.
                    let error = (self.set_point - sample)
                        / (2 * frontend.get_tia_resistor1().unwrap().get::<kiloohm>() as i64); // TODO: Handle resistor1 and resitor2.

                    // The available LED current.
                    let available_led_current = if error.is_positive() {
                        self.led_max_current - self.led_current
                    } else {
                        self.led_current - self.led_min_current
                    };
                    // The available offset current.
                    let available_offset_current = if error.is_positive() {
                        self.offset_max_current - self.offset_current
                    } else {
                        self.offset_current - self.offset_min_current
                    };

                    // Calculate the part of the error that will be compensated by the LED current.
                    let led_error = (error * available_led_current)
                        / (available_led_current + available_offset_current * self.alpha);
                    // Calculate the part of the error that will be compensated by the offset current.
                    let offset_error = error - led_error;
                    log::info!(
                        "Error: {} nA, LED error: {} nA, alpha error: {} mA, offset error: {} nA",
                        error,
                        led_error,
                        led_error * self.alpha,
                        offset_error,
                    );

                    // Update the led current.
                    self.led_current = frontend
                        .set_led1_current(ElectricCurrent::new::<nanoampere>(
                            (self.led_current + self.alpha * led_error) as f32,
                        ))
                        .unwrap()
                        .get::<nanoampere>() as i64;
                    // Update the offset current.
                    self.offset_current = frontend
                        .set_offset_led1_current(ElectricCurrent::new::<nanoampere>(
                            (self.offset_current + offset_error) as f32,
                        ))
                        .unwrap()
                        .get::<nanoampere>() as i64;
                }
            }
        }
    }
}
