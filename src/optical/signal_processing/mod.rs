use std::sync::{Arc, Mutex};

use queues::{CircularBuffer, IsQueue};
use uom::si::{electric_potential::volt, f32::ElectricPotential};

mod histogram;

pub(crate) enum CriticalValue {
    None,
    Minimum,
    Maximum,
}

impl CriticalValue {
    pub(crate) fn into_bits(&self) -> u8 {
        match self {
            CriticalValue::None => 0b00000000,
            CriticalValue::Minimum => 0b00000001,
            CriticalValue::Maximum => 0b00000010,
        }
    }
}

pub(crate) struct ProcessingHistory {
    /// A sized queue of the last `window_size` elements.
    window: CircularBuffer<f32>,
    window_size: usize,
    /// A histogram of the last `window_size` elements.
    distribution: histogram::Histogram,
    /// The last element added to the window.
    previous_element: Option<f32>,
    /// The second last element added to the window.
    previous_previous_element: Option<f32>,
}

impl ProcessingHistory {
    fn new(window_size: usize) -> ProcessingHistory {
        ProcessingHistory {
            window: CircularBuffer::new(window_size),
            window_size,
            distribution: histogram::Histogram::new(64, -1.2, 1.2),
            previous_element: None,
            previous_previous_element: None,
        }
    }

    /// Update the window, the distribution and the previous elements of the processing history.
    fn update(&mut self, el: f32) {
        let old_el = self.window.add(el).unwrap(); // Add never fails on CircularBuffer.

        self.distribution.increment(el);

        if let Some(old_el) = old_el {
            self.distribution.decrement(old_el);
        }

        if self.previous_element.is_some() && self.previous_element.unwrap() == el {
            // The element is the same as the previous one, don't update the previous elements or information will be lost.
            return;
        }
        self.previous_previous_element = self.previous_element;
        self.previous_element = Some(el);
    }
}

/// Returns a tuple containing the critical value of the element preceding the last one added to the window and the relative element.
pub(crate) fn find_critical_value(el: f32, history: &mut ProcessingHistory) -> CriticalValue {
    let value: CriticalValue = if let (Some(previous_el), Some(previous_previous_el)) =
        (history.previous_element, history.previous_previous_element)
    {
        if previous_el < previous_previous_el && previous_el < el {
            CriticalValue::Minimum
        } else if previous_el > previous_previous_el && previous_el > el {
            CriticalValue::Maximum
        } else {
            CriticalValue::None
        }
    } else {
        // The window was empty.
        CriticalValue::None
    };

    history.update(el);

    value
}

pub fn processing_task(data: Arc<Mutex<super::data_sending::AggregatedData>>) {
    let queue_size = 512;

    let critical_thresholds = (0.2, 0.8); // The lower and upper thresholds for filtering out critical values.

    let mut history = [
        ProcessingHistory::new(queue_size),
        ProcessingHistory::new(queue_size),
        ProcessingHistory::new(queue_size),
    ];

    loop {
        // TODO: Update the led1, led2, led3 values to the previous one.
        // TODO: Different structs aggregated data and readings with indexes.
        if let Ok(mut data) = data.lock() {
            data.led1_critical_value =
                find_critical_value(data.led1_reading.value, &mut history[0]);
            data.led2_critical_value =
                find_critical_value(data.led2_reading.value, &mut history[1]);
            data.led3_critical_value =
                find_critical_value(data.led3_reading.value, &mut history[2]);

            data.led1_lower_threshold = ElectricPotential::new::<volt>(
                history[0].distribution.percentile(critical_thresholds.0),
            );
            data.led1_upper_threshold = ElectricPotential::new::<volt>(
                history[0].distribution.percentile(critical_thresholds.1),
            );
            data.led2_lower_threshold = ElectricPotential::new::<volt>(
                history[1].distribution.percentile(critical_thresholds.0),
            );
            data.led2_upper_threshold = ElectricPotential::new::<volt>(
                history[1].distribution.percentile(critical_thresholds.1),
            );
            data.led3_lower_threshold = ElectricPotential::new::<volt>(
                history[2].distribution.percentile(critical_thresholds.0),
            );
            data.led3_upper_threshold = ElectricPotential::new::<volt>(
                history[2].distribution.percentile(critical_thresholds.1),
            );

        }
    }
}
