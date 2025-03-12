use crate::fitness::FitnessFunction;
use crate::search_space::SearchSpace;
use rand::Rng;

pub mod one_plus_one_ea;
pub mod simulated_annealing;

// Shared state between all simulations. Probably the type to send to client
#[derive(Debug)]
pub struct SimulationState<S: SearchSpace> {
    pub iteration: usize,
    pub current_solution: S,
    pub current_fitness: f64,
}

pub trait EvolutionaryAlgorithm<S: SearchSpace, F: FitnessFunction<S>> {
    fn iterate<R: Rng>(&mut self, rng: &mut R) -> &SimulationState<S>;
}
