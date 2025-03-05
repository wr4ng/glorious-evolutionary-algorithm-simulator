use rand::Rng;

use super::search_space::{Bitstring, SearchSpace};

pub trait Mutation<S: SearchSpace> {
    fn apply<R: EARng>(&self, solution: &S, rng: &mut R) -> S;
}

pub struct NaiveBitflip;

impl Mutation<Bitstring> for NaiveBitflip {
    fn apply<R: EARng>(&self, solution: &Bitstring, rng: &mut R) -> Bitstring {
        let bits = solution
            .bits()
            .iter()
            .map(|b| {
                let flip = rng.random_ratio(1, solution.bits().len() as u32);
                if flip {
                    !*b
                } else {
                    *b
                }
            })
            .collect::<Vec<_>>();

        Bitstring::new(bits)
    }
}
//TODO: pub struct TwoOpt;
//      impl Mutation<Permutation> for TwoOpt { ... }
//TODO: pub struct ThreeOpt;
//      impl Mutation<Permutation> for ThreeOpt { ... }

pub trait EARng {
    fn random_ratio(&mut self, numerator: u32, denominator: u32) -> bool;
}

impl<T: Rng> EARng for T {
    fn random_ratio(&mut self, numerator: u32, denominator: u32) -> bool {
        Rng::random_ratio(self, numerator, denominator)
    }
}

struct MockRng {
    random_ratio_values: Vec<bool>,
    random_ratio_index: usize,
}

impl MockRng {
    fn new_ratio(values: Vec<bool>) -> Self {
        MockRng {
            random_ratio_values: values,
            random_ratio_index: 0,
        }
    }
}

impl EARng for MockRng {
    fn random_ratio(&mut self, _: u32, _: u32) -> bool {
        let value = self.random_ratio_values[self.random_ratio_index];
        self.random_ratio_index += 1;
        value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_naive_bitflip() {
        let bitstring = Bitstring::new(vec![false; 10]);
        let mut flips = vec![false; 10];
        flips[2] = true;
        flips[6] = true;
        let mut rng = MockRng::new_ratio(flips.clone());
        let flipped = NaiveBitflip::apply(&NaiveBitflip, &bitstring, &mut rng);
        assert_eq!(*flipped.bits(), flips);
    }
}
