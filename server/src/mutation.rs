use super::search_space::{Bitstring, Permutation, SearchSpace};
use rand::Rng;
use std::ops::Range;

pub trait Mutation<S: SearchSpace> {
    fn apply<R: EARng>(&self, solution: &S, rng: &mut R) -> S;
}

pub struct NaiveBitflip;

impl Mutation<Bitstring> for NaiveBitflip {
    fn apply<R: EARng>(&self, solution: &Bitstring, rng: &mut R) -> Bitstring {
        let bits = solution
            .bits()
            .iter()
            .map(|&b| {
                let to_flip = rng.random_ratio(1, solution.bits().len() as u32);
                to_flip ^ b
            })
            .collect::<Vec<_>>();

        Bitstring::new(bits)
    }
}

pub struct Bitflip;

impl Mutation<Bitstring> for Bitflip {
    fn apply<R: EARng>(&self, solution: &Bitstring, rng: &mut R) -> Bitstring {
        let mut result = solution.clone();
        let p = 1.0 / solution.bits().len() as f64;
        let mut i = 0;
        i += rng.sample_geometric(p) as usize;

        while i < solution.bits().len() {
            result.flip(i);
            i += 1;
            i += rng.sample_geometric(p) as usize;
        }
        result
    }
}

//TODO: pub struct SingleBitflip (used by Simulated Annealing)

pub struct TwoOpt;

impl Mutation<Permutation> for TwoOpt {
    fn apply<R: EARng>(&self, solution: &Permutation, rng: &mut R) -> Permutation {
        let previous = solution.permutation();
        let a = rng.random_range(0..previous.len());
        let mut b = a;
        while b == a {
            b = rng.random_range(0..previous.len());
        }
        let (v1, v2) = if a > b { (b, a) } else { (a, b) };
        let mut result = Vec::with_capacity(previous.len());
        for v in &previous[0..=v1] {
            result.push(*v);
        }
        for i in ((v1 + 1)..=v2).rev() {
            result.push(previous[i]);
        }
        for v in &previous[(v2 + 1)..] {
            result.push(*v);
        }
        Permutation::new(result)
    }
}

//TODO: pub struct ThreeOpt;
//      impl Mutation<Permutation> for ThreeOpt { ... }

pub trait EARng {
    fn random_ratio(&mut self, numerator: u32, denominator: u32) -> bool;
    fn random_range(&mut self, range: Range<usize>) -> usize;
    fn sample_geometric(&mut self, p: f64) -> u64;
}

impl<T: Rng> EARng for T {
    fn random_ratio(&mut self, numerator: u32, denominator: u32) -> bool {
        Rng::random_ratio(self, numerator, denominator)
    }

    fn random_range(&mut self, range: Range<usize>) -> usize {
        Rng::random_range(self, range)
    }

    // Sample a value from a geometric distribution with success probablity p,
    // using inverse CDF method
    fn sample_geometric(&mut self, p: f64) -> u64 {
        let rand: f64 = Rng::random_range(self, 0.0..1.0);
        (rand.log2() / (1.0 - p).log2()).floor() as u64
    }
}

#[derive(Default)]
struct MockRng {
    random_ratio_values: Vec<bool>,
    random_ratio_index: usize,

    random_range_values: Vec<usize>,
    random_range_index: usize,

    random_geometric_values: Vec<u64>,
    random_geometric_index: usize,
}

impl MockRng {
    fn new_ratio(values: Vec<bool>) -> Self {
        MockRng {
            random_ratio_values: values,
            random_ratio_index: 0,
            ..Default::default()
        }
    }

    fn new_range(values: Vec<usize>) -> Self {
        MockRng {
            random_range_values: values,
            random_range_index: 0,
            ..Default::default()
        }
    }

    fn new_geometric(values: Vec<u64>) -> Self {
        MockRng {
            random_geometric_values: values,
            random_geometric_index: 0,
            ..Default::default()
        }
    }
}

impl EARng for MockRng {
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

#[cfg(test)]
mod tests {
    use super::*;

    fn bitstring_to_bools(s: &str) -> Vec<bool> {
        s.chars()
            .map(|c| match c {
                '0' => false,
                '1' => true,
                _ => panic!("invalid character: {}", c),
            })
            .collect()
    }

    #[test]
    fn test_naive_bitflip() {
        // (input, flips, expected)
        let testcases = vec![
            ("00000000", "10001001", "10001001"),
            ("1001010110", "1111111111", "0110101001"),
            ("101010110101010", "000000000000000", "101010110101010"),
        ];

        for t in testcases {
            let bitstring = Bitstring::from_bitstring(t.0).unwrap();
            let mut mock_rng = MockRng::new_ratio(bitstring_to_bools(t.1));
            let got = NaiveBitflip::apply(&NaiveBitflip, &bitstring, &mut mock_rng);
            assert_eq!(*got.bits(), bitstring_to_bools(t.2))
        }
    }

    #[test]
    fn test_bitflip() {
        // (input, [flip distance(s)], expected)
        let testcases = vec![
            ("0000000000", vec![4, 10], "0000100000"),
            ("0000000000", vec![1, 2, 10], "0100100000"),
            ("0000000000", vec![0, 0, 0, 10], "1110000000"),
            ("0000000000", vec![10], "0000000000"),
        ];

        for t in testcases {
            let bitstring = Bitstring::from_bitstring(t.0).unwrap();
            let mut mock_rng = MockRng::new_geometric(t.1);
            let got = Bitflip.apply(&bitstring, &mut mock_rng);
            assert_eq!(*got.bits(), bitstring_to_bools(t.2))
        }
    }

    #[test]
    fn test_two_opt() {
        let initial = Permutation::new(vec![0, 1, 4, 3, 2, 5, 6, 7]);
        let mut mock_rng = MockRng::new_range(vec![1, 4]);
        let result = TwoOpt::apply(&TwoOpt, &initial, &mut mock_rng);
        assert_eq!(*result.permutation(), vec![0, 1, 2, 3, 4, 5, 6, 7])
    }
}
