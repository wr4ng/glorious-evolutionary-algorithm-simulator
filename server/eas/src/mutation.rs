use super::rng::MyRng;
use super::search_space::{Bitstring, Permutation, SearchSpace};

pub trait Mutation<S: SearchSpace> {
    fn apply<R: MyRng>(&self, solution: &S, rng: &mut R) -> S;
}

pub struct NaiveBitflip;

impl Mutation<Bitstring> for NaiveBitflip {
    fn apply<R: MyRng>(&self, solution: &Bitstring, rng: &mut R) -> Bitstring {
        let bits = solution
            .bits()
            .iter()
            .map(|&b| {
                let to_flip = rng.random_ratio(1, solution.size() as u32);
                to_flip ^ b
            })
            .collect::<Vec<_>>();

        Bitstring::new(bits)
    }
}

pub struct Bitflip;

impl Mutation<Bitstring> for Bitflip {
    fn apply<R: MyRng>(&self, solution: &Bitstring, rng: &mut R) -> Bitstring {
        let mut result = solution.clone();
        let p = 1.0 / solution.size() as f64;
        let mut i = 0;
        i += rng.sample_geometric(p) as usize;

        while i < solution.size() {
            result.flip(i);
            i += 1;
            i += rng.sample_geometric(p) as usize;
        }
        result
    }
}

pub struct SingleBitflip;

impl Mutation<Bitstring> for SingleBitflip {
    fn apply<R: MyRng>(&self, solution: &Bitstring, rng: &mut R) -> Bitstring {
        let mut result = solution.clone();
        let i = rng.random_range(0..solution.size());
        result.flip(i);
        result
    }
}

pub struct TwoOpt;

impl Mutation<Permutation> for TwoOpt {
    fn apply<R: MyRng>(&self, solution: &Permutation, rng: &mut R) -> Permutation {
        let previous = solution.permutation();
        let a = rng.random_range(0..previous.len());
        let mut b = a;
        while b == a {
            b = rng.random_range(0..previous.len());
        }
        let result = two_opt(previous, a, b);
        Permutation::new(result)
    }
}

fn two_opt(previous: &Vec<usize>, a: usize, b: usize) -> Vec<usize> {
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
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rng::MockRng;

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
            let got = NaiveBitflip.apply(&bitstring, &mut mock_rng);
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
    fn test_single_bitflip() {
        // (input, flip, expected)
        let testcases = vec![
            ("00000000", 7, "00000001"),
            ("1001010110", 3, "1000010110"),
            ("101010110101010", 0, "001010110101010"),
        ];

        for t in testcases {
            let bitstring = Bitstring::from_bitstring(t.0).unwrap();
            let mut mock_rng = MockRng::new_range(vec![t.1]);
            let got = SingleBitflip.apply(&bitstring, &mut mock_rng);
            assert_eq!(*got.bits(), bitstring_to_bools(t.2))
        }
    }

    #[test]
    fn test_two_opt() {
        let initial = Permutation::new(vec![0, 1, 4, 3, 2, 5, 6, 7]);
        let mut mock_rng = MockRng::new_range(vec![1, 4]);
        let result = TwoOpt.apply(&initial, &mut mock_rng);
        assert_eq!(*result.permutation(), vec![0, 1, 2, 3, 4, 5, 6, 7])
    }
}
