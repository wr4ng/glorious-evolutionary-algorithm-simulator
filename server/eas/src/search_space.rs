use std::fmt::Debug;

use crate::rng::MyRng;

// Main search space trait used as solution candidate for algorithms
pub trait SearchSpace: Debug + Clone {
    fn new_random<R: MyRng>(size: usize, rng: &mut R) -> Self;
    fn size(&self) -> usize;
    fn to_string(&self) -> String;
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
    fn new_random<R: MyRng>(size: usize, rng: &mut R) -> Self {
        let mut bits = vec![false; size];
        for bit in bits.iter_mut() {
            *bit = rng.random();
        }
        Bitstring { bits }
    }

    fn size(&self) -> usize {
        self.bits.len()
    }

    fn to_string(&self) -> String {
        self.bits
            .iter()
            .map(|&b| if b { "1" } else { "0" })
            .collect()
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
    fn new_random<R: MyRng>(size: usize, rng: &mut R) -> Self {
        let mut perm = (0..size).collect::<Vec<_>>();
        rng.shuffle_vec(&mut perm);
        Permutation { permutation: perm }
    }

    fn size(&self) -> usize {
        self.permutation.len()
    }

    fn to_string(&self) -> String {
        self.permutation
            .iter()
            .map(|&v| v.to_string())
            .collect::<Vec<_>>()
            .join(", ")
    }
}
