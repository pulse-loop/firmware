use std::time::Instant;

/// A timer that can be used to measure time passed from its creation or last reset.
pub(crate) struct Timer {
    duration: u128,
    instant: Instant,
}

impl Timer {
    /// Starts a new timer with the given duration in milliseconds.
    pub(crate) fn new(milliseconds: u128) -> Self {
        Timer {
            duration: milliseconds,
            instant: Instant::now(),
        }
    }

    /// Resets the timer.
    pub(crate) fn reset(&mut self) {
        self.instant = Instant::now();
    }

    /// Checks if the timer has expired.
    pub(crate) fn is_expired(&self) -> bool {
        self.instant.elapsed().as_millis() >= self.duration
    }
}
