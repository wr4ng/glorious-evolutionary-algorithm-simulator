use rand::Rng;

use super::search_space::{Bitstring, SearchSpace};


pub trait Mutation<S: SearchSpace> {
    fn apply<R: Rng>(&self, solution: &S, rng: &mut R) -> S;
}

pub struct NaiveBitflip;

impl Mutation<Bitstring> for NaiveBitflip {
    fn apply<R: Rng>(&self, solution: &Bitstring, rng: &mut R) -> Bitstring {
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
