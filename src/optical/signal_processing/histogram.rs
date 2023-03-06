pub(crate) struct Histogram {
    // The number of elements in the histogram.
    count: usize,
    // The number of bins in the histogram.
    bins: usize,
    // The minimum value of the histogram.
    min: i32,
    // The maximum value of the histogram.
    max: i32,
    // The width of each bin.
    bin_width: i32,
    // The number of elements in each bin.
    bin_counts: Vec<usize>,
}

impl Histogram {
    /// Creates a new histogram with the given number of bins and the given range.
    pub(crate) fn new(bins: usize, min: i32, max: i32) -> Histogram {
        Histogram {
            count: 0,
            bins,
            min,
            max,
            bin_width: (max - min) / (bins - 1) as i32,
            bin_counts: vec![0; bins],
        }
    }

    /// Adds an element to the histogram.
    pub(crate) fn increment(&mut self, value: i32) {
        let bin = ((value - self.min) / self.bin_width) as usize;
        self.bin_counts[bin] += 1;
        self.count += 1;
    }

    /// Removes an element from the histogram.
    pub(crate) fn decrement(&mut self, value: i32) {
        let bin = ((value - self.min) / self.bin_width) as usize;
        self.bin_counts[bin] -= 1;
        self.count -= 1;
    }

    /// Returns the number of elements in the histogram.
    pub(crate) fn count(&self) -> usize {
        self.count
    }

    /// Returns the given percentile expressed in the range [0, 100].
    pub(crate) fn percentile(&self, percentile: usize) -> i32 {
        let mut count = 0;
        let mut bin = 0;
        let target = percentile * self.count / 100;
        while count < target {
            count += self.bin_counts[bin];
            bin += 1;
        }
        self.min + (bin as i32 * self.bin_width)
    }

    /// Returns the given percentile expressed in the range [0, 100] with finer granularity, assuming a constant distribution within each bin.
    pub(crate) fn percentile_fine(&self, percentile: usize) -> i32 {
        let mut count = 0;
        let mut bin = 0;
        let target = percentile * self.count / 100;
        while count < target {
            count += self.bin_counts[bin];
            bin += 1;
        }
        // let bin_start = self.min + (bin as i32 * self.bin_width);
        // let bin_count = self.bin_counts[bin];
        // let bin_target = target - (count - bin_count);
        // let bin_percentile = bin_target * 100 / bin_count;
        // bin_start + (bin_percentile as i32 * self.bin_width / 100)
        self.min + (bin as i32 * self.bin_width) - ((count - target) as i32 * self.bin_width / self.bin_counts[bin] as i32)
    }
}
