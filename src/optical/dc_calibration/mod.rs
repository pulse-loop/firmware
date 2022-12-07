use uom::si::{
    capacitance::picofarad,
    electric_current::milliampere,
    electric_potential::volt,
    electrical_resistance::kiloohm,
    f32::{Capacitance, ElectricCurrent, ElectricPotential, ElectricalResistance},
};

use afe4404::{
    device::AFE4404,
    modes::ThreeLedsMode,
    {
        led_current::LedCurrentConfiguration,
        tia::{CapacitorConfiguration, ResistorConfiguration},
    },
};

use crate::optical::data_reading::get_sample_blocking;

fn bisection(
    a: &mut ElectricCurrent,
    b: &mut ElectricCurrent,
    c: &mut ElectricCurrent,
    is_f_greater: bool,
) {
    if is_f_greater {
        *a = *c;
    } else {
        *b = *c;
    }
    *c = (*a + *b) / 2.0;
}

pub fn calibration_loop<I2C>(frontend: &mut AFE4404<I2C, ThreeLedsMode>)
where
    I2C: embedded_hal::i2c::I2c,
{
    struct Parameters {
        resistors: [ElectricalResistance; 8],
        capacitors: [Capacitance; 8],
        current_max_value: ElectricCurrent,
        voltage_max_value: ElectricPotential,
    }

    let parameters: Parameters = Parameters {
        resistors: [
            ElectricalResistance::new::<kiloohm>(10.0),
            ElectricalResistance::new::<kiloohm>(25.0),
            ElectricalResistance::new::<kiloohm>(50.0),
            ElectricalResistance::new::<kiloohm>(100.0),
            ElectricalResistance::new::<kiloohm>(250.0),
            ElectricalResistance::new::<kiloohm>(500.0),
            ElectricalResistance::new::<kiloohm>(1000.0),
            ElectricalResistance::new::<kiloohm>(2000.0),
        ],
        capacitors: [
            Capacitance::new::<picofarad>(2.5),
            Capacitance::new::<picofarad>(5.0),
            Capacitance::new::<picofarad>(7.5),
            Capacitance::new::<picofarad>(10.0),
            Capacitance::new::<picofarad>(17.5),
            Capacitance::new::<picofarad>(20.0),
            Capacitance::new::<picofarad>(22.5),
            Capacitance::new::<picofarad>(25.0),
        ],
        current_max_value: ElectricCurrent::new::<milliampere>(100.0),
        voltage_max_value: ElectricPotential::new::<volt>(1.0),
    };

    // Starting values.
    let mut best_resistors = ResistorConfiguration::<ThreeLedsMode>::new(
        parameters.resistors[0],
        parameters.resistors[0],
    );
    // TODO: Estimate best capacitor starting value.
    let best_capacitors = CapacitorConfiguration::<ThreeLedsMode>::new(
        parameters.capacitors[7],
        parameters.capacitors[7],
    );
    let mut best_currents;

    frontend.set_tia_resistors(&best_resistors).unwrap();
    frontend.set_tia_capacitors(&best_capacitors).unwrap();

    // Gain calibration loop.
    // Set the current to the maximum value.
    // Set resistors to minimum value and increase them until the voltage threshold is reached.
    let voltage_threshold = parameters.voltage_max_value * 0.8;
    best_currents = LedCurrentConfiguration::<ThreeLedsMode>::new(
        parameters.current_max_value,
        parameters.current_max_value,
        parameters.current_max_value,
    );
    frontend.set_leds_current(&best_currents).unwrap();

    for resistor in parameters.resistors.iter().skip(1) {
        std::thread::sleep(std::time::Duration::from_millis(200));

        let sample = get_sample_blocking(frontend, 10).unwrap();

        // Select greater resistors in case of current saturation.
        if sample.led1() < &voltage_threshold {
            *best_resistors.resistor1_mut() = *resistor;
        }
        if sample.led2() < &voltage_threshold || sample.led3() < &voltage_threshold {
            *best_resistors.resistor2_mut() = *resistor;
        }
        if sample.led1() >= &voltage_threshold
            && sample.led2() >= &voltage_threshold
            && sample.led3() >= &voltage_threshold
        {
            break;
        }

        frontend.set_tia_resistors(&best_resistors).unwrap();
        // TODO: Update capacitor values.
    }

    // Current calibration loop.
    // Using bisection method, find the current that makes the photodiode voltage reach 80% of the maximum value.
    let mut lower_current_bound = [ElectricCurrent::new::<milliampere>(0.0); 3];
    let mut upper_current_bound = [parameters.current_max_value; 3];
    let mut mid_current_bound = [parameters.current_max_value / 2.0; 3];
    *best_currents.led1_mut() = mid_current_bound[0];
    *best_currents.led2_mut() = mid_current_bound[1];
    *best_currents.led3_mut() = mid_current_bound[2];
    frontend.set_leds_current(&best_currents).unwrap();

    while upper_current_bound[0] - lower_current_bound[0] > ElectricCurrent::new::<milliampere>(0.8)
        || upper_current_bound[1] - lower_current_bound[1]
            > ElectricCurrent::new::<milliampere>(0.8)
        || upper_current_bound[2] - lower_current_bound[2]
            > ElectricCurrent::new::<milliampere>(0.8)
    {
        std::thread::sleep(std::time::Duration::from_millis(200));

        let sample = get_sample_blocking(frontend, 10).unwrap();

        if upper_current_bound[0] - lower_current_bound[0]
            > ElectricCurrent::new::<milliampere>(0.8)
        {
            bisection(
                &mut lower_current_bound[0],
                &mut upper_current_bound[0],
                &mut mid_current_bound[0],
                sample.led1() < &voltage_threshold,
            );
            *best_currents.led1_mut() = mid_current_bound[0];
        }
        if upper_current_bound[1] - lower_current_bound[1]
            > ElectricCurrent::new::<milliampere>(0.8)
        {
            bisection(
                &mut lower_current_bound[1],
                &mut upper_current_bound[1],
                &mut mid_current_bound[1],
                sample.led2() < &voltage_threshold,
            );
            *best_currents.led2_mut() = mid_current_bound[1];
        }
        if upper_current_bound[2] - lower_current_bound[2]
            > ElectricCurrent::new::<milliampere>(0.8)
        {
            bisection(
                &mut lower_current_bound[2],
                &mut upper_current_bound[2],
                &mut mid_current_bound[2],
                sample.led3() < &voltage_threshold,
            );
            *best_currents.led3_mut() = mid_current_bound[2];
        }

        best_currents = frontend.set_leds_current(&best_currents).unwrap();
    }
}
