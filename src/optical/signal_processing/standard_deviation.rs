pub(crate) struct MovingStandardDeviation {
    pub(crate) values: Vec<f32>,
    pub(crate) next_element: usize,
    pub(crate) size: usize,
    pub(crate) quadratic_sum: f32,
}

impl MovingStandardDeviation {
    pub(crate) fn new(size: usize) -> Self {
        Self {
            values: Vec::with_capacity(size),
            next_element: 0,
            size,
            quadratic_sum: 0.0,
        }
    }

    // Returns the standard deviation of the values in the buffer.
    pub(crate) fn push(&mut self, value: f32) -> f32 {
        if self.values.len() < self.size {
            self.values.push(value.powi(2));
        } else {
            self.quadratic_sum -= self.values[self.next_element];
            self.values[self.next_element] = value.powi(2);
        }

        self.next_element = (self.next_element + 1) % self.size;
        self.quadratic_sum += value.powi(2);

        (self.quadratic_sum / self.values.len() as f32).sqrt()
    }
}
