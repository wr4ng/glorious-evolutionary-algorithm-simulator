use crate::{rng::MyRng, search_space::SearchSpace};
use rand::rngs::ThreadRng;
use rand_xoshiro::Xoshiro128PlusPlus;

pub mod one_plus_one_ea;
pub mod simulated_annealing;

// Shared state between all simulations. Probably the type to send to client
#[derive(Debug)]
pub struct SimulationState<S: SearchSpace> {
    pub iteration: u64,
    pub current_solution: S,
    pub current_fitness: f64,
}

pub trait EvolutionaryAlgorithmCore {
    fn iterate<R: MyRng>(&mut self, rng: &mut R);
    fn current_fitness(&self) -> f64;
    fn iterations(&self) -> u64;
    fn status_json(&self) -> serde_json::Value;
}

pub trait EvolutionaryAlgorithm<R>: Send {
    fn iterate(&mut self, rng: &mut R);
    fn current_fitness(&self) -> f64;
    fn iterations(&self) -> u64;
    fn status_json(&self) -> serde_json::Value;
}

impl<T: EvolutionaryAlgorithmCore + Send> EvolutionaryAlgorithm<ThreadRng> for T {
    fn iterate(&mut self, rng: &mut ThreadRng) {
        self.iterate(rng);
    }

    fn current_fitness(&self) -> f64 {
        self.current_fitness()
    }

    fn iterations(&self) -> u64 {
        self.iterations()
    }

    fn status_json(&self) -> serde_json::Value {
        self.status_json()
    }
}

impl<T: EvolutionaryAlgorithmCore + Send> EvolutionaryAlgorithm<Xoshiro128PlusPlus> for T {
    fn iterate(&mut self, rng: &mut Xoshiro128PlusPlus) {
        self.iterate(rng);
    }

    fn current_fitness(&self) -> f64 {
        self.current_fitness()
    }

    fn iterations(&self) -> u64 {
        self.iterations()
    }

    fn status_json(&self) -> serde_json::Value {
        self.status_json()
    }
}
