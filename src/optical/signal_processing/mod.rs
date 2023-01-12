use queues::{CircularBuffer, IsQueue};

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
    pub(crate) fn new(window_size: usize) -> ProcessingHistory {
        ProcessingHistory {
            window: CircularBuffer::new(window_size),
            window_size,
            distribution: histogram::Histogram::new(1024, -1.2, 1.2),
            previous_element: None,
            previous_previous_element: None,
        }
    }

    /// Update the window and the distribution of the processing history.
    pub(crate) fn update(&mut self, el: f32) {
        let old_el = self.window.add(el).unwrap(); // Add never fails on CircularBuffer.

        self.distribution.increment(el);

        if let Some(old_el) = old_el {
            self.distribution.decrement(old_el);
        }
    }
}

pub(crate) fn find_critical_value(el: f32, history: &mut ProcessingHistory) -> CriticalValue {
    // TODO: Finish.
    if let Some(previous_el) = history.previous_element {
    } else {
        // The window was empty.
        history.previous_element = Some(el);
    }

    CriticalValue::None
}
