use std::f32::consts::PI;

// Used to perform a dot product between a sine wave and a signal.
#[derive(Debug)]
pub struct SineProduct {
    pub(crate) sine_frequency: f32,
    pub(crate) sampling_period: f32,
    pub(crate) window_period: f32,
    pub(crate) time: f32,
    pub(crate) result: f32,

    pub(crate) s: f32,
}

impl SineProduct {
    pub fn new(sine_frequency: f32, sampling_period: f32, window_period: f32) -> Self {
        Self {
            sine_frequency,
            sampling_period,
            window_period,
            time: 0.0,
            result: 0.0,

            s: 0.0,
        }
    }

    pub fn process(&mut self, signal: f32) -> Option<f32> {
        self.s = (2.0 * PI * self.sine_frequency * self.time).sin();
        self.result = signal * self.s;
        self.time += self.sampling_period;

        if self.time >= self.window_period {
            Some(self.result)
        } else {
            None
        }
    }

    pub fn reset(&mut self, sine_frequency: f32) {
        self.sine_frequency = sine_frequency;
        self.time = 0.0;
        self.result = 0.0;
    }
}
