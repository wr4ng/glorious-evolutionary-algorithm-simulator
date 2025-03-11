use rand::{seq::SliceRandom, Rng};
use std::fmt::Debug;

pub trait SearchSpace: Debug + Clone {
    fn new_random<R: Rng>(size: usize, rng: &mut R) -> Self;
    fn size(&self) -> usize;
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

    pub fn flip(&mut self, i: usize) {
        self.bits[i] = !self.bits[i];
    }

    //TODO: Should be Result<Self, {some error}>
    pub fn from_bitstring(s: &str) -> Option<Self> {
        let bitstring = s
            .chars()
            .map(|c| match c {
                '0' => Some(false),
                '1' => Some(true),
                _ => None,
            })
            .collect::<Option<Vec<_>>>();

        bitstring.map(|bits| Self { bits })
    }
}

impl SearchSpace for Bitstring {
    fn new_random<R: Rng>(size: usize, rng: &mut R) -> Self {
        let mut bits = vec![false; size];
        for bit in bits.iter_mut() {
            *bit = rng.random();
        }
        Bitstring { bits }
    }

    fn size(&self) -> usize {
        self.bits.len()
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

    pub fn new(permutation: Vec<usize>) -> Self {
        Permutation { permutation }
    }
}

impl SearchSpace for Permutation {
    fn new_random<R: Rng>(size: usize, rng: &mut R) -> Self {
        let mut perm = (0..size).collect::<Vec<_>>();
        perm.shuffle(rng);
        Permutation { permutation: perm }
    }

    fn size(&self) -> usize {
        self.permutation.len()
    }
}
