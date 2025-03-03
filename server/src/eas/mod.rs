#![allow(dead_code)]

use rand::{seq::SliceRandom, Rng};
use std::fmt::Debug;

pub trait SearchSpace: Debug + Clone {
    fn new_random<R: Rng>(size: usize, rng: &mut R) -> Self;
}

#[derive(Debug, Clone)]
pub struct Bitstring {
    bits: Vec<bool>,
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

impl SearchSpace for Permutation {
    fn new_random<R: Rng>(size: usize, rng: &mut R) -> Self {
        let mut perm = (0..size).collect::<Vec<_>>();
        perm.shuffle(rng);
        return Permutation { permutation: perm };
    }
}

pub trait FitnessFunction<T: SearchSpace> {
    fn evaluate(&self, instance: &T) -> f64;
    fn is_maximizing(&self) -> bool;

    fn compare(&self, a: f64, b: f64) -> std::cmp::Ordering {
        if self.is_maximizing() {
            a.total_cmp(&b)
        } else {
            a.total_cmp(&b).reverse()
        }
    }
}

pub struct OneMax;

impl FitnessFunction<Bitstring> for OneMax {
    fn evaluate(&self, instance: &Bitstring) -> f64 {
        return instance
            .bits
            .iter()
            .fold(0_f64, |acc, b| if *b { acc + 1_f64 } else { acc });
    }
    fn is_maximizing(&self) -> bool {
        true
    }
}

//TODO: struct LeadingOnes;
//TODO: struct TSP {}
// determine how we want to store distances (probably distance matrix to support non-euclidian
// instances
// impl FitnessFunction<Permutation> for TSP {}
// impl TSP { from_EUC2D() -> TSP }

// Shared state between all simulations. Probably the type to send to client
#[derive(Debug)]
pub struct SimulationState<S: SearchSpace> {
    pub iteration: usize,
    pub current_solution: S,
    pub current_fitness: f64,
}

pub trait EvolutionaryAlgorithm<S: SearchSpace, F: FitnessFunction<S>> {
    fn iterate(&mut self) -> &SimulationState<S>;
}
//TODO:
//- function to run a single iteration
//- function to create a problem instance from provided params (given all are available)
//  fn fromParams(&Params) -> Option<Self>;

pub trait Mutation<S: SearchSpace> {
    fn apply<R: Rng>(&self, solution: &S, rng: &mut R) -> S;
}

pub struct NaiveBitflip;

impl Mutation<Bitstring> for NaiveBitflip {
    fn apply<R: Rng>(&self, solution: &Bitstring, rng: &mut R) -> Bitstring {
        let bits = solution
            .bits
            .iter()
            .map(|b| {
                let flip = rng.random_ratio(1, solution.bits.len() as u32);
                if flip {
                    !*b
                } else {
                    *b
                }
            })
            .collect::<Vec<_>>();

        Bitstring { bits }
    }
}

pub struct OnePlusOneEA<S: SearchSpace, F: FitnessFunction<S>, M: Mutation<S>, R> {
    pub state: SimulationState<S>,
    fitness_function: F,
    mutator: M,
    rng: R,
}

impl<S, F, M, R> OnePlusOneEA<S, F, M, R>
where
    S: SearchSpace,
    F: FitnessFunction<S>,
    M: Mutation<S>,
    R: Rng,
{
    pub fn new(size: usize, mutator: M, fitness_function: F, mut rng: R) -> Self {
        let current_solution = S::new_random(size, &mut rng);
        let current_fitness = fitness_function.evaluate(&current_solution);
        OnePlusOneEA {
            state: SimulationState {
                iteration: 0,
                current_solution,
                current_fitness,
            },
            fitness_function,
            mutator,
            rng,
        }
    }
}

impl<S, F, M, R> EvolutionaryAlgorithm<S, F> for OnePlusOneEA<S, F, M, R>
where
    S: SearchSpace,
    F: FitnessFunction<S>,
    M: Mutation<S>,
    R: Rng,
{
    fn iterate(&mut self) -> &SimulationState<S> {
        let offspring = self
            .mutator
            .apply(&mut self.state.current_solution, &mut self.rng);

        let new_fitness = self.fitness_function.evaluate(&offspring);

        self.state.iteration += 1;

        let fitness_order = self
            .fitness_function
            .compare(new_fitness, self.state.current_fitness);

        if fitness_order == std::cmp::Ordering::Greater {
            self.state.current_fitness = new_fitness;
            self.state.current_solution = offspring;
        }

        &self.state
    }
}
