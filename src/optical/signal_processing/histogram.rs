pub(crate) struct Histogram {
    // The number of elements in the histogram.
    count: usize,
    // The number of bins in the histogram.
    bins: usize,
    // The minimum value of the histogram.
    min: f32,
    // The maximum value of the histogram.
    max: f32,
    // The width of each bin.
    bin_width: f32,
    // The number of elements in each bin.
    bin_counts: Vec<usize>,
}

impl Histogram {
    /// Creates a new histogram with the given number of bins and the given range.
    pub(crate) fn new(bins: usize, min: f32, max: f32) -> Histogram {
        Histogram {
            count: 0,
            bins,
            min,
            max,
            bin_width: (max - min) / bins as f32,
            bin_counts: vec![0; bins],
        }
    }

    /// Adds an element to the histogram.
    pub(crate) fn increment(&mut self, value: f32) {
        let bin = ((value - self.min) / self.bin_width) as usize;
        self.bin_counts[bin] += 1;
        self.count += 1;
    }

    /// Removes an element from the histogram.
    pub(crate) fn decrement(&mut self, value: f32) {
        let bin = ((value - self.min) / self.bin_width) as usize;
        self.bin_counts[bin] -= 1;
        self.count -= 1;
    }

    /// Returns the number of elements in the histogram.
    pub(crate) fn count(&self) -> usize {
        self.count
    }

    /// Returns the given percentile expressed in the range [0, 1].
    pub(crate) fn percentile(&self, percentile: f32) -> f32 {
        let mut count = 0;
        let mut bin = 0;
        let target = (percentile * self.count as f32) as usize;
        while count < target {
            count += self.bin_counts[bin];
            bin += 1;
        }
        self.min + (bin as f32 * self.bin_width)
    }
}
