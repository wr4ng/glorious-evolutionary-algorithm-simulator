use rand::Rng;

use crate::{rng::MyRng, search_space::SearchSpace};

pub mod mmas;
pub mod one_plus_one_ea;
pub mod simulated_annealing;

#[derive(Debug)]
pub struct SimulationState<S: SearchSpace> {
    pub iteration: u64,
    pub current_solution: S,
    pub current_fitness: f64,
}

// The core algorithm trait implemented by all implemented algorithms
pub trait AlgorithmCore {
    fn iterate<R: MyRng>(&mut self, rng: &mut R);
    fn current_fitness(&self) -> f64;
    fn iterations(&self) -> u64;
    fn status_json(&self) -> serde_json::Value;
}

// Trait used by the server implementation,
// requiring the types to be able to be sent safely between threads.
// Also uses a fixed RNG to avoid generic methods, allowing for the creating of trait objects
pub trait Algorithm<R: Rng>: Send {
    fn iterate(&mut self, rng: &mut R);
    fn current_fitness(&self) -> f64;
    fn iterations(&self) -> u64;
    fn status_json(&self) -> serde_json::Value;
}

// Algorithm is implemented for types implementing AlgorithmCore, using a fixed RNG
impl<T: AlgorithmCore + Send, R: Rng> Algorithm<R> for T {
    fn iterate(&mut self, rng: &mut R) {
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
