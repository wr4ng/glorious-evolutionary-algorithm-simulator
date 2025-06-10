use std::ops::Range;

use rand::{seq::SliceRandom, Rng};

// RNG trait with functions used by algorithms.
// Allows mocking the RNG passed to algorithm implementations for testing.
pub trait MyRng {
    fn random(&mut self) -> bool;
    fn random_bool(&mut self, probability: f64) -> bool;
    fn random_ratio(&mut self, numerator: u32, denominator: u32) -> bool;
    fn random_range(&mut self, range: Range<usize>) -> usize;
    fn random_range_float(&mut self, range: Range<f64>) -> f64;
    fn sample_geometric(&mut self, p: f64) -> u64;
    fn sample_poisson(&mut self) -> u64;
    fn shuffle_vec<T>(&mut self, v: &mut Vec<T>);
}

impl<T: Rng> MyRng for T {
    fn random(&mut self) -> bool {
        self.random()
    }

    fn random_bool(&mut self, probability: f64) -> bool {
        self.random_bool(probability)
    }

    fn random_ratio(&mut self, numerator: u32, denominator: u32) -> bool {
        self.random_ratio(numerator, denominator)
    }

    fn random_range(&mut self, range: Range<usize>) -> usize {
        self.random_range(range)
    }

    // Sample a value from a geometric distribution with success probablity p,
    // using inverse CDF method
    fn sample_geometric(&mut self, p: f64) -> u64 {
        let rand: f64 = self.random_range(0.0..1.0);
        (rand.log2() / (1.0 - p).log2()).floor() as u64
    }

    // Sample a value from a Poisson distribution with mean 1 using Knuth's method
    fn sample_poisson(&mut self) -> u64 {
        let l = (-1.0_f64).exp(); // e^(-λ)
        let mut k = 0;
        let mut p = 1.0;
        loop {
            let u: f64 = self.random_range(0.0..1.0);
            p *= u;
            if p < l {
                return k;
            }
            k += 1;
        }
    }

    fn shuffle_vec<I>(&mut self, v: &mut Vec<I>) {
        v.shuffle(self);
    }

    fn random_range_float(&mut self, range: Range<f64>) -> f64 {
        self.random_range(range)
    }
}

// Mock implementation of MyRng,
// allowing for setting random numbers returned by each function
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

impl MyRng for MockRng {
    fn random(&mut self) -> bool {
        todo!()
    }

    fn random_bool(&mut self, _: f64) -> bool {
        todo!()
    }

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

    fn sample_poisson(&mut self) -> u64 {
        todo!()
    }

    fn shuffle_vec<T>(&mut self, _: &mut Vec<T>) {
        todo!()
    }

    fn random_range_float(&mut self, _: Range<f64>) -> f64 {
        todo!()
    }
}
