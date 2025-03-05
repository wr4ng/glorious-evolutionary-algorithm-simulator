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

    fn bitstring_to_bools(s: &str) -> Vec<bool> {
        s.chars()
            .map(|c| match c {
                '0' => false,
                '1' => true,
                _ => panic!("invalid character: {}", c),
            })
            .collect()
    }

    impl Bitstring {
        fn from_str(s: &str) -> Self {
            Bitstring::new(bitstring_to_bools(s))
        }
    }

    struct TestCase<'a> {
        input: &'a str,
        flips: &'a str,
        expected: &'a str,
    }

    #[test]
    fn test_naive_bitflip() {
        let testcases = vec![
            TestCase {
                input: "00000000",
                flips: "10001001",
                expected: "10001001",
            },
            TestCase {
                input: "1001010110",
                flips: "1111111111",
                expected: "0110101001",
            },
            TestCase {
                input: "10101011010101010101011",
                flips: "00000000000000000000000",
                expected: "10101011010101010101011",
            },
        ];

        for t in testcases {
            let bitstring = Bitstring::from_str(t.input);
            let mut mock_rng = MockRng::new_ratio(bitstring_to_bools(t.flips));
            let got = NaiveBitflip::apply(&NaiveBitflip, &bitstring, &mut mock_rng);
            assert_eq!(*got.bits(), bitstring_to_bools(t.expected))
        }
    }
}
