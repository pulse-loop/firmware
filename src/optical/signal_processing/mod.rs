pub mod filters;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub(crate) enum CriticalValue {
    #[default]
    None,
    Minimum(f32, u128),
    Maximum(f32, u128),
}

pub(crate) struct CriticalHistory {
    pub(crate) max: (f32, u128),
    pub(crate) min: (f32, u128),
    pub(crate) is_positive: bool,
    pub(crate) time: std::time::Instant,
}

impl CriticalHistory {
    pub(crate) fn new() -> Self {
        Self {
            max: (0.0, 0),
            min: (0.0, 0),
            is_positive: true,
            time: std::time::Instant::now(),
        }
    }
}

pub(crate) fn find_critical_value(element: f32, history: &mut CriticalHistory) -> CriticalValue {
    let critical;

    if element > history.max.0 {
        history.max = (element, history.time.elapsed().as_millis());
    } else if element < history.min.0 {
        history.min = (element, history.time.elapsed().as_millis());
    }

    let is_positive = element > 0.0;
    if history.is_positive != is_positive {
        // The signal slope has changed.
        if history.is_positive {
            // The signal was positive, the maximum is the critical value.
            critical = CriticalValue::Maximum(history.max.0, history.max.1);
            history.max.0 = 0.0;
        } else {
            // The signal was negative, the minimum is the critical value.
            critical = CriticalValue::Minimum(history.min.0, history.min.1);
            history.min.0 = 0.0;
        }
        // Update the sign of the signal.
        history.is_positive = is_positive;
    } else {
        critical = CriticalValue::None;
    }

    critical
}
