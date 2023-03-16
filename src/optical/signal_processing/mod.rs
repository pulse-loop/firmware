use queues::{CircularBuffer, IsQueue};
use static_fir::impl_fir;

mod histogram;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) enum CriticalValue {
    #[default]
    None,
    Minimum(i32, u128),
    Maximum(i32, u128),
}

impl CriticalValue {
    pub(crate) fn into_bits(&self) -> u8 {
        match self {
            CriticalValue::None => 0b00000000,
            CriticalValue::Minimum(_, _) => 0b00000001,
            CriticalValue::Maximum(_, _) => 0b00000010,
        }
    }
}

// pub(crate) struct ProcessingHistory {
//     /// A sized queue of the last `window_size` elements.
//     pub(crate) window: CircularBuffer<i32>,
//     pub(crate) window_size: usize,
//     /// A histogram of the last `window_size` elements.
//     pub(crate) distribution: histogram::Histogram,
//     /// The last element added to the window.
//     pub(crate) previous_element: Option<i32>,
//     /// The second last element added to the window.
//     pub(crate) previous_previous_element: Option<i32>,
// }

// impl ProcessingHistory {
//     pub(crate) fn new(window_size: usize, distribution_bins: usize) -> ProcessingHistory {
//         ProcessingHistory {
//             window: CircularBuffer::new(window_size),
//             window_size,
//             distribution: histogram::Histogram::new(distribution_bins, -1_200_000, 1_200_000),
//             previous_element: None,
//             previous_previous_element: None,
//         }
//     }

//     /// Update the window and the distribution of the processing history.
//     pub(crate) fn update(&mut self, el: i32) {
//         let old_el = self.window.add(el).unwrap(); // Add never fails on CircularBuffer.

//         self.distribution.increment(el);

//         if let Some(old_el) = old_el {
//             self.distribution.decrement(old_el);
//         }
//     }
// }

/// Returns the critical value of the element preceding the last one added to the window.
// pub(crate) fn find_critical_value(el: i32, history: &mut ProcessingHistory) -> CriticalValue {
//     let hysteresis = 100;
//     let value: CriticalValue = if let (Some(previous_el), Some(previous_previous_el)) =
//         (history.previous_element, history.previous_previous_element)
//     {
//         if previous_el < previous_previous_el - hysteresis && previous_el < el - hysteresis {
//             CriticalValue::Minimum
//         } else if previous_el > previous_previous_el + hysteresis && previous_el > el + hysteresis {
//             CriticalValue::Maximum
//         } else {
//             CriticalValue::None
//         }
//     } else {
//         // The window was empty.
//         CriticalValue::None
//     };

//     // Update the previous elements.
//     if history.previous_element.is_some() && history.previous_element.unwrap() == el {
//         // The element is the same as the previous one, don't update the previous elements or information will be lost.
//         return value;
//     }
//     history.previous_previous_element = history.previous_element;
//     history.previous_element = Some(el);

//     value
// }

// Implement a low-pass filter for the DC component.
// Passband: 0 - 0.3 Hz, ripple: 0.06 dB
// Stopband: 0.8 - 5 Hz, attenuation: -22.24 dB
impl_fir!(
    DcFir,
    f32,
    33,
    [
        0.033686230350082585,
        -0.010642447087272166,
        -0.012813731629770824,
        -0.015887455303791324,
        -0.018536806202827574,
        -0.019364143460885164,
        -0.017080379868972623,
        -0.010724076937264609,
        0.00015361692942768187,
        0.0153753114647108,
        0.034111713562928206,
        0.05494760855744724,
        0.07603485264975439,
        0.09531783587412412,
        0.11080271852376086,
        0.1208289965464284,
        0.12429891622440617,
        0.1208289965464284,
        0.11080271852376086,
        0.09531783587412412,
        0.07603485264975439,
        0.05494760855744724,
        0.034111713562928206,
        0.0153753114647108,
        0.00015361692942768187,
        -0.010724076937264609,
        -0.017080379868972623,
        -0.019364143460885164,
        -0.018536806202827574,
        -0.015887455303791324,
        -0.012813731629770824,
        -0.010642447087272166,
        0.033686230350082585
    ]
);

// Implement a band-pass filter for the AC component.
// Stopband: 0 - 0.1 Hz, attenuation: -60.76 dB
// Passband: 0.5 - 4 Hz, ripple: 3.81 dB
// Stopband: 4.3 - 5 Hz, attenuation: -40.76 dB
impl_fir!(
    AcFir,
    f32,
    33,
    [
        0.06974076415112264,
        0.030276242654535573,
        -0.056117818250729194,
        0.011192297410823544,
        -0.021962057743598634,
        -0.014309791580197795,
        0.008324646277687258,
        -0.04925212809464995,
        0.013471582602332218,
        -0.06518619115131469,
        -0.03237592537499673,
        -0.03472154196202484,
        -0.12562517860174727,
        0.032067041436089995,
        -0.22191523843341426,
        0.08750037721667803,
        0.7372133213412866,
        0.08750037721667803,
        -0.22191523843341426,
        0.032067041436089995,
        -0.12562517860174727,
        -0.03472154196202484,
        -0.03237592537499673,
        -0.06518619115131469,
        0.013471582602332218,
        -0.04925212809464995,
        0.008324646277687258,
        -0.014309791580197795,
        -0.021962057743598634,
        0.011192297410823544,
        -0.056117818250729194,
        0.030276242654535573,
        0.06974076415112264
    ]
);

pub(crate) struct CriticalHistory {
    pub(crate) max: (i32, u128),
    pub(crate) min: (i32, u128),
    pub(crate) is_positive: bool,
    pub(crate) time: std::time::Instant,
}

impl CriticalHistory {
    pub(crate) fn new() -> Self {
        Self {
            max: (0, 0),
            min: (0, 0),
            is_positive: true,
            time: std::time::Instant::now(),
        }
    }
}

pub(crate) fn find_critical_value(element: i32, history: &mut CriticalHistory) -> CriticalValue {
    let critical;

    if element > history.max.0 {
        history.max = (element, history.time.elapsed().as_millis());
    } else if element < history.min.0 {
        history.min = (element, history.time.elapsed().as_millis());
    }

    let is_positive = element > 0;
    if history.is_positive != is_positive {
        // The signal slope has changed.
        if history.is_positive {
            // The signal was positive, the maximum is the critical value.
            critical = CriticalValue::Maximum(history.max.0, history.max.1);
            history.max.1 = 0;
        } else {
            // The signal was negative, the minimum is the critical value.
            critical = CriticalValue::Minimum(history.min.0, history.min.1);
            history.min.0 = 0;
        }
        // Update the sign of the signal.
        history.is_positive = is_positive;
    } else {
        critical = CriticalValue::None;
    }

    critical
}
