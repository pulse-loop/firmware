use uom::si::{
    electric_current::{microampere, milliampere},
    electric_potential::millivolt,
    f32::{ElectricCurrent, ElectricPotential, ElectricalResistance},
};

pub(crate) struct Calibrator {
    // Afe4404 values.
    led_current_min: ElectricCurrent,
    led_current_max: ElectricCurrent,
    offset_current_min: ElectricCurrent,
    offset_current_max: ElectricCurrent,

    // Dc calibration.
    alpha: f32, // The skin reflectance parameter (alpha = i_led / i_photodiode).
    offset_current_set_point: ElectricCurrent, // In order to turn on the LED, set a negative offset.
    adc_set_point: ElectricPotential,
    adc_working_threshold: ElectricPotential, // Around the adc_set_point.

    // Frontend functions.
    get_led_current: Box<dyn Fn() -> ElectricCurrent>,
    set_led_current: Box<dyn Fn(ElectricCurrent) -> ElectricCurrent>,
    get_offset_current: Box<dyn Fn() -> ElectricCurrent>,
    set_offset_current: Box<dyn Fn(ElectricCurrent) -> ElectricCurrent>,
    get_resistor: Box<dyn Fn() -> ElectricalResistance>,
}

unsafe impl Send for Calibrator {}
unsafe impl Sync for Calibrator {}

impl Calibrator {
    /// Creates a new `Calibrator`.
    pub(crate) fn new<GLC, SLC, GOC, SOC, GR>(
        alpha: f32,
        get_led_current: GLC,
        set_led_current: SLC,
        get_offset_current: GOC,
        set_offset_current: SOC,
        get_resistor: GR,
    ) -> Self
    where
        GLC: Fn() -> ElectricCurrent + 'static,
        SLC: Fn(ElectricCurrent) -> ElectricCurrent + 'static,
        GOC: Fn() -> ElectricCurrent + 'static,
        SOC: Fn(ElectricCurrent) -> ElectricCurrent + 'static,
        GR: Fn() -> ElectricalResistance + 'static,
    {
        let calibrator = Calibrator {
            // TODO: Change to optimal initial value.
            led_current_min: ElectricCurrent::new::<milliampere>(0.0),
            led_current_max: ElectricCurrent::new::<milliampere>(50.0),
            offset_current_min: ElectricCurrent::new::<microampere>(-7.0),
            offset_current_max: ElectricCurrent::new::<microampere>(7.0),
            alpha,
            offset_current_set_point: ElectricCurrent::new::<microampere>(-6.5),
            adc_set_point: ElectricPotential::new::<millivolt>(500.0),
            adc_working_threshold: ElectricPotential::new::<millivolt>(300.0),
            get_led_current: Box::new(get_led_current),
            set_led_current: Box::new(set_led_current),
            get_offset_current: Box::new(get_offset_current),
            set_offset_current: Box::new(set_offset_current),
            get_resistor: Box::new(get_resistor),
        };

        (calibrator.set_led_current)(calibrator.led_current_min);
        (calibrator.set_offset_current)(calibrator.offset_current_min);

        calibrator
    }

    /// Gets an immutable reference of the minimum led current.
    pub(crate) fn led_current_min(&self) -> &ElectricCurrent {
        &self.led_current_min
    }

    /// Gets an immutable reference of the maximum led current.
    pub(crate) fn led_current_max(&self) -> &ElectricCurrent {
        &self.led_current_max
    }

    /// Gets an immutable reference of the minimum offset current.
    pub(crate) fn offset_current_min(&self) -> &ElectricCurrent {
        &self.offset_current_min
    }

    /// Gets an immutable reference of the maximum offset current.
    pub(crate) fn offset_current_max(&self) -> &ElectricCurrent {
        &self.offset_current_max
    }

    /// Gets an immutable reference of the skin reflectance parameter alpha.
    pub(crate) fn alpha(&self) -> &f32 {
        &self.alpha
    }

    /// Gets an immutable reference of the offset current set point.
    pub(crate) fn offset_current_set_point(&self) -> &ElectricCurrent {
        &self.offset_current_set_point
    }

    /// Gets an immutable reference of the adc set point.
    pub(crate) fn adc_set_point(&self) -> &ElectricPotential {
        &self.adc_set_point
    }

    /// Gets an immutable reference of the adc working threshold.
    pub(crate) fn adc_working_threshold(&self) -> &ElectricPotential {
        &self.adc_working_threshold
    }

    /// Gets a mutable reference of the minimum led current.
    pub(crate) fn led_current_min_mut(&mut self) -> &mut ElectricCurrent {
        &mut self.led_current_min
    }

    /// Gets a mutable reference of the maximum led current.
    pub(crate) fn led_current_max_mut(&mut self) -> &mut ElectricCurrent {
        &mut self.led_current_max
    }

    /// Gets a mutable reference of the minimum offset current.
    pub(crate) fn offset_current_min_mut(&mut self) -> &mut ElectricCurrent {
        &mut self.offset_current_min
    }

    /// Gets a mutable reference of the maximum offset current.
    pub(crate) fn offset_current_max_mut(&mut self) -> &mut ElectricCurrent {
        &mut self.offset_current_max
    }

    /// Gets a mutable reference of the skin reflectance parameter alpha.
    pub(crate) fn alpha_mut(&mut self) -> &mut f32 {
        &mut self.alpha
    }

    /// Gets a mutable reference of the offset current set point.
    pub(crate) fn offset_current_set_point_mut(&mut self) -> &mut ElectricCurrent {
        &mut self.offset_current_set_point
    }

    /// Gets a mutable reference of the adc set point.
    pub(crate) fn adc_set_point_mut(&mut self) -> &mut ElectricPotential {
        &mut self.adc_set_point
    }

    /// Gets a mutable reference of the adc working threshold.
    pub(crate) fn adc_working_threshold_mut(&mut self) -> &mut ElectricPotential {
        &mut self.adc_working_threshold
    }

    /// Calibrates the DC component of the signal by changing the LED current and the offset current.
    /// The calibration is firstly performed on the LED current for larger changes, then on the offset current for better accuracy.
    /// Returns true if the calibration was performed, false otherwise.
    // TODO: Handle unwrap fails.
    pub(crate) fn calibrate_dc(&mut self, sample: ElectricPotential) -> bool {
        // Calibrate only if the sample is out of the working threshold.
        if sample < self.adc_set_point - self.adc_working_threshold
            || sample > self.adc_set_point + self.adc_working_threshold
        {
            // Get the led current and the offset current from the frontend.
            let mut led_current = (self.get_led_current)();
            let mut offset_current = (self.get_offset_current)();

            // The error between the set point and the sample converted in the current seen by the photodiode.
            let error = (self.adc_set_point - sample) / (2.0 * (self.get_resistor)());

            // Calculate the requested led current.
            let requested_led_current =
                led_current + self.alpha * (error - self.offset_current_set_point + offset_current);

            led_current = (self.set_led_current)(if requested_led_current < self.led_current_min {
                self.led_current_min
            } else if requested_led_current > self.led_current_max {
                self.led_current_max
            } else {
                requested_led_current
            });

            // Calculate the requested offset current.
            let requested_offset_current =
                self.offset_current_set_point + (requested_led_current - led_current) / self.alpha;

            offset_current =
                (self.set_offset_current)(if requested_offset_current < self.offset_current_min {
                    log::warn!("Offset too low");
                    self.offset_current_min
                } else if requested_offset_current > self.offset_current_max {
                    log::warn!("Offset too high");
                    self.offset_current_max
                } else {
                    requested_offset_current
                });

            log::info!(
                "Calibrated DC {:?} {:?}, {:?}, {:?}",
                error,
                requested_led_current,
                led_current,
                offset_current,
            );
            true
        } else {
            false
        }
    }
}
