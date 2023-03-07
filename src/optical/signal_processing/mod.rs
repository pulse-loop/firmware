use queues::{CircularBuffer, IsQueue};

mod histogram;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) enum CriticalValue {
    #[default]
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
    pub(crate) window: CircularBuffer<i32>,
    pub(crate) window_size: usize,
    /// A histogram of the last `window_size` elements.
    pub(crate) distribution: histogram::Histogram,
    /// The last element added to the window.
    pub(crate) previous_element: Option<i32>,
    /// The second last element added to the window.
    pub(crate) previous_previous_element: Option<i32>,
}

impl ProcessingHistory {
    pub(crate) fn new(window_size: usize, distribution_bins: usize) -> ProcessingHistory {
        ProcessingHistory {
            window: CircularBuffer::new(window_size),
            window_size,
            distribution: histogram::Histogram::new(distribution_bins, -1_200_000, 1_200_000),
            previous_element: None,
            previous_previous_element: None,
        }
    }

    /// Update the window and the distribution of the processing history.
    pub(crate) fn update(&mut self, el: i32) {
        let old_el = self.window.add(el).unwrap(); // Add never fails on CircularBuffer.

        self.distribution.increment(el);

        if let Some(old_el) = old_el {
            self.distribution.decrement(old_el);
        }
    }
}

/// Returns the critical value of the element preceding the last one added to the window.
pub(crate) fn find_critical_value(el: i32, history: &mut ProcessingHistory) -> CriticalValue {
    let hysteresis = 100;
    let value: CriticalValue = if let (Some(previous_el), Some(previous_previous_el)) =
        (history.previous_element, history.previous_previous_element)
    {
        if previous_el < previous_previous_el - hysteresis && previous_el < el - hysteresis {
            CriticalValue::Minimum
        } else if previous_el > previous_previous_el + hysteresis && previous_el > el + hysteresis {
            CriticalValue::Maximum
        } else {
            CriticalValue::None
        }
    } else {
        // The window was empty.
        CriticalValue::None
    };

    // Update the previous elements.
    if history.previous_element.is_some() && history.previous_element.unwrap() == el {
        // The element is the same as the previous one, don't update the previous elements or information will be lost.
        return value;
    }
    history.previous_previous_element = history.previous_element;
    history.previous_element = Some(el);

    value
}
