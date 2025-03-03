#![allow(dead_code)]

use rand::{seq::SliceRandom, Rng};

struct Bitstring {
    bits: Vec<bool>,
}

struct Permutation {
    permutation: Vec<usize>,
}

//TODO: Maybe rename to instance? or solution?
// Need clone + debug
trait SearchSpace {
    fn new_random<R: Rng>(size: usize, rng: &mut R) -> Self;
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

impl SearchSpace for Permutation {
    fn new_random<R: Rng>(size: usize, rng: &mut R) -> Self {
        let mut perm = (0..size).collect::<Vec<_>>();
        perm.shuffle(rng);
        return Permutation { permutation: perm };
    }
}

trait FitnessFunction<T: SearchSpace> {
    fn fitness(&self, instance: &T) -> usize;
    fn is_maximizing(&self) -> bool;
}

struct OneMax;

impl FitnessFunction<Bitstring> for OneMax {
    fn fitness(&self, instance: &Bitstring) -> usize {
        return instance
            .bits
            .iter()
            .fold(0, |acc, b| if *b { acc + 1 } else { acc });
    }
    fn is_maximizing(&self) -> bool {
        true
    }
}

//TODO:
trait EvolutionaryAlgorithm<S: SearchSpace, F: FitnessFunction<S>> {
    //
    fn iterate(&mut self) -> &SimulationState<S>;
}
//- function to run a single iteration
//- function to create a problem instance from provided params (given all are available)
//  fn fromParams(&Params) -> Option<Self>;

//TODO:
// struct TSP
// determine how we want to store distances (probably distance matrix to support non-euclidian
// instances
// impl Problem<Permutation> for TSP {}
// impl TSP { from_EUC2D() -> TSP }

// struct OnePlusOneEA<P, S> where P: Problem<S> {
//     problem: P
// }
//
// impl OnePlusOneEA {
//
// }

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
        // Save previous fitness
        // Create new offspring
        let offspring = self.mutator.apply(&mut self.state.current_solution);
        let new_fitness = self.fitness_function.fitness(&offspring);
        // Compare with previous fitness (EvolutionData struct?) and possible swap internal
        // solution...
        //
        // Update simulation state...
        &self.state
    }
}

// Shared state between all simulations. Probably the type to send to client
struct SimulationState<S: SearchSpace> {
    iteration: usize,
    current_solution: S,
    current_fitness: usize, //TODO: f64?
}
