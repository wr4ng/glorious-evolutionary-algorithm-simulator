use rand::{seq::SliceRandom, Rng};
use std::fmt::Debug;

pub trait SearchSpace: Debug + Clone {
    fn new_random<R: Rng>(size: usize, rng: &mut R) -> Self;
}

#[derive(Debug, Clone)]
pub struct Bitstring {
    bits: Vec<bool>,
}

impl Bitstring {
    pub fn bits(&self) -> &Vec<bool> {
        &self.bits
    }

    pub fn new(bits: Vec<bool>) -> Self {
        Bitstring { bits }
    }
}

impl SearchSpace for Bitstring {
    fn new_random<R: Rng>(size: usize, rng: &mut R) -> Self {
        let mut bits = vec![false; size];
        for i in 0..size {
            bits[i] = rng.random();
        }
        Bitstring { bits }
    }
}

#[derive(Debug, Clone)]
pub struct Permutation {
    permutation: Vec<usize>,
}

impl Permutation {
    pub fn permutation(&self) -> &Vec<usize> {
        &self.permutation
    }
}

impl SearchSpace for Permutation {
    fn new_random<R: Rng>(size: usize, rng: &mut R) -> Self {
        let mut perm = (0..size).collect::<Vec<_>>();
        perm.shuffle(rng);
        return Permutation { permutation: perm };
    }
}
