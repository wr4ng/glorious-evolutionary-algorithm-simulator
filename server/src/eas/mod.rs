#![allow(dead_code)]

use rand::{seq::SliceRandom, Rng};
use std::fmt::Debug;

trait SearchSpace: Debug + Clone {
    fn new_random<R: Rng>(size: usize, rng: &mut R) -> Self;
}

#[derive(Debug, Clone)]
struct Bitstring {
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
struct Permutation {
    permutation: Vec<usize>,
}

impl SearchSpace for Permutation {
    fn new_random<R: Rng>(size: usize, rng: &mut R) -> Self {
        let mut perm = (0..size).collect::<Vec<_>>();
        perm.shuffle(rng);
        return Permutation { permutation: perm };
    }
}

trait FitnessFunction<T: SearchSpace> {
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

struct OneMax;

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
struct SimulationState<S: SearchSpace> {
    iteration: usize,
    current_solution: S,
    current_fitness: f64,
}

trait EvolutionaryAlgorithm<S: SearchSpace, F: FitnessFunction<S>> {
    fn iterate(&mut self) -> &SimulationState<S>;
}
//TODO:
//- function to run a single iteration
//- function to create a problem instance from provided params (given all are available)
//  fn fromParams(&Params) -> Option<Self>;

trait Mutation<S: SearchSpace> {
    fn apply(&self, solution: &S) -> S;
}

struct OnePlusOneEA<S: SearchSpace, F: FitnessFunction<S>, M: Mutation<S>> {
    state: SimulationState<S>,
    fitness_function: F,
    mutator: M,
}

impl<S, F, M> EvolutionaryAlgorithm<S, F> for OnePlusOneEA<S, F, M>
where
    S: SearchSpace,
    F: FitnessFunction<S>,
    M: Mutation<S>,
{
    fn iterate(&mut self) -> &SimulationState<S> {
        let offspring = self.mutator.apply(&mut self.state.current_solution);
        let new_fitness = self.fitness_function.evaluate(&offspring);

        self.state.iteration += 1;

        if self.fitness_function.compare(self.state.current_fitness, new_fitness) == std::cmp::Ordering::Greater {
            self.state.current_fitness = new_fitness;
            self.state.current_solution = offspring;
        }

        // Compare with previous fitness (EvolutionData struct?) and possible swap internal
        //
        // solution...
        //
        // Update simulation state...
        &self.state
    }
}
