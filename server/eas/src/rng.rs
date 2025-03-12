use std::ops::Range;

pub trait Rng {
    fn random_ratio(&mut self, numerator: u32, denominator: u32) -> bool;
    fn random_range(&mut self, range: Range<usize>) -> usize;
    fn sample_geometric(&mut self, p: f64) -> u64;
}

impl<T: rand::Rng> Rng for T {
    fn random_ratio(&mut self, numerator: u32, denominator: u32) -> bool {
        rand::Rng::random_ratio(self, numerator, denominator)
    }

    fn random_range(&mut self, range: Range<usize>) -> usize {
        rand::Rng::random_range(self, range)
    }

    // Sample a value from a geometric distribution with success probablity p,
    // using inverse CDF method
    fn sample_geometric(&mut self, p: f64) -> u64 {
        let rand: f64 = rand::Rng::random_range(self, 0.0..1.0);
        (rand.log2() / (1.0 - p).log2()).floor() as u64
    }
}

#[derive(Default)]
pub struct MockRng {
    random_ratio_values: Vec<bool>,
    random_ratio_index: usize,

    random_range_values: Vec<usize>,
    random_range_index: usize,

    random_geometric_values: Vec<u64>,
    random_geometric_index: usize,
}

#[cfg(test)]
impl MockRng {
    pub fn new_ratio(values: Vec<bool>) -> Self {
        MockRng {
            random_ratio_values: values,
            random_ratio_index: 0,
            ..Default::default()
        }
    }

    pub fn new_range(values: Vec<usize>) -> Self {
        MockRng {
            random_range_values: values,
            random_range_index: 0,
            ..Default::default()
        }
    }

    pub fn new_geometric(values: Vec<u64>) -> Self {
        MockRng {
            random_geometric_values: values,
            random_geometric_index: 0,
            ..Default::default()
        }
    }
}

impl Rng for MockRng {
    fn random_ratio(&mut self, _: u32, _: u32) -> bool {
        let value = self.random_ratio_values[self.random_ratio_index];
        self.random_ratio_index += 1;
        value
    }

    fn random_range(&mut self, _: Range<usize>) -> usize {
        let value = self.random_range_values[self.random_range_index];
        self.random_range_index += 1;
        value
    }

    fn sample_geometric(&mut self, _: f64) -> u64 {
        let value = self.random_geometric_values[self.random_geometric_index];
        self.random_geometric_index += 1;
        value
    }
}
