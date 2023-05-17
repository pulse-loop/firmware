// Due to the fact that the offset cancellation DAC can have device-to-device variations,
// the offset currents can be measured from the current device.

use afe4404::system::State;
use uom::si::{
    electric_current::microampere,
    electrical_resistance::ohm,
    f32::{ElectricCurrent, ElectricalResistance},
};

pub(crate) struct OffsetCurrents {
    currents: [ElectricCurrent; 31],
}

impl OffsetCurrents {
    pub(crate) fn new() -> Self {
        let mut currents: [ElectricCurrent; 31] = Default::default();
        for i in 0..31 {
            currents[i] = ElectricCurrent::new::<microampere>(7.0 / 15.0 * i as f32 - 7.0);
        }

        Self { currents }
    }

    pub(crate) fn accurate(&mut self, offset: ElectricCurrent) -> ElectricCurrent {
        let i = ((offset.get::<microampere>() + 7.0) / 7.0 * 15.0) as usize;
        self.currents[i]
    }

    pub(crate) fn measure(&mut self) {
        if let Ok(mut frontend) = crate::optical::FRONTEND.lock() {
            if let Some(frontend) = frontend.as_mut() {
                // Disconnect the photodiode.
                frontend
                    .set_photodiode(State::Disabled)
                    .expect("Failed disconnect the photodiode.");
                std::thread::sleep(std::time::Duration::from_millis(60));

                // Measure offset currents.
                for i in 0..31 {
                    frontend
                        .set_offset_led3_current(ElectricCurrent::new::<microampere>(
                            7.0 / 15.0 * i as f32 - 7.0,
                        ))
                        .expect("Failed to set offset current.");
                    std::thread::sleep(std::time::Duration::from_millis(60));

                    let voltage = *frontend
                        .read()
                        .expect("Failed to read offset current.")
                        .led3();
                    let current = voltage
                        / (2.0 * ElectricalResistance::new::<ohm>(crate::optical::RESISTOR2));

                    self.currents[i] = current;
                    log::info!("Offset current: {}", current.get::<microampere>());
                }

                // Reconnect the photodiode.
                frontend
                    .set_photodiode(State::Enabled)
                    .expect("Failed to reconnect the photodiode.");
            }
        }
    }
}
