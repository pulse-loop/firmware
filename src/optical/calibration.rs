use uom::si::{
    electric_current::{milliampere, nanoampere},
    electrical_resistance::kiloohm,
    f32::ElectricCurrent,
};

pub(crate) struct Calibrator {
    // Voltages are expressed in microvolts, currents in nanoamperes and resistors in kiloohms.

    // Afe4404 values.
    led_current_min: i64,
    led_current_max: i64,
    offset_current_min: i64,
    offset_current_max: i64,

    // Dc calibration.
    alpha: i64, // Skin coefficient, alpha = i_led / i_photodiode.
    set_point: i64,
    working_threshold: i64,
}

impl Calibrator {
    pub(crate) fn new() -> Self {
        Calibrator {
            // TODO: Change to optimal initial value.
            led_current_min: 2_000_000,   // nA.
            led_current_max: 100_000_000, // nA.
            offset_current_min: -7_000,   // nA.
            offset_current_max: 7_000,    // nA.
            alpha: 800,
            set_point: 0,               // µV.
            working_threshold: 400_000, // µV.
        }
    }

    // Calibrates the DC component of the signal by changing the LED current and the offset current.
    // The calibration is firstly performed on the LED current for larger changes, then on the offset current for better accuracy.
    // TODO: Handle unwrap fails.
    pub(crate) fn calibrate_dc(&mut self, sample: i64) {
        // Calibrate only if the sample is out of the working threshold.
        if sample.abs() > self.working_threshold {
            // Acquire the frontend lock.
            if let Ok(mut frontend) = super::FRONTEND.lock() {
                if let Some(frontend) = frontend.as_mut() {
                    // Get the led current and the offset current from the frontend.
                    let mut led_current =
                        frontend.get_led1_current().unwrap().get::<nanoampere>() as i64;
                    let offset_current = frontend
                        .get_offset_led1_current()
                        .unwrap()
                        .get::<nanoampere>() as i64;

                    // The error between the set point and the sample converted in the current seen by the photodiode.
                    let error = (self.set_point - sample)
                        / (2 * frontend.get_tia_resistor1().unwrap().get::<kiloohm>() as i64); // TODO: Handle resistor1 and resitor2.

                    // Calculate the led current.
                    let requested_led_current = led_current + self.alpha * (error + offset_current);

                    led_current = frontend
                        .set_led1_current(ElectricCurrent::new::<nanoampere>(
                            requested_led_current.clamp(self.led_current_min, self.led_current_max)
                                as f32,
                        ))
                        .unwrap()
                        .get::<nanoampere>() as i64;

                    // Calculate the offset current.
                    let requested_offset_current =
                        (requested_led_current - led_current) / self.alpha;

                    frontend
                        .set_offset_led1_current(ElectricCurrent::new::<nanoampere>(
                            requested_offset_current
                                .clamp(self.offset_current_min, self.offset_current_max)
                                as f32,
                        ))
                        .unwrap();

                    log::info!(
                        "Calibrated DC: requested_led_current = {} nA, led_current = {} nA, offset_current = {} nA",
                        requested_led_current,
                        led_current,
                        offset_current
                    );
                }
            }
        }
    }
}
