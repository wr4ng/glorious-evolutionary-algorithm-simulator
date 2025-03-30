use crate::search_space::SearchSpace;
use rand::rngs::ThreadRng;

pub mod one_plus_one_ea;
pub mod simulated_annealing;

// Shared state between all simulations. Probably the type to send to client
#[derive(Debug)]
pub struct SimulationState<S: SearchSpace> {
    pub iteration: u64,
    pub current_solution: S,
    pub current_fitness: f64,
}

pub trait EvolutionaryAlgorithm {
    fn iterate(&mut self, rng: &mut ThreadRng);
    fn current_fitness(&self) -> f64;
}
