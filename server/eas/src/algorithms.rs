use crate::{rng::MyRng, search_space::SearchSpace};
use rand::rngs::ThreadRng;
use rand_pcg::Pcg64;

pub mod one_plus_one_ea;
pub mod simulated_annealing;
pub mod mmas;

#[derive(Debug)]
pub struct SimulationState<S: SearchSpace> {
    pub iteration: u64,
    pub current_solution: S,
    pub current_fitness: f64,
}

pub trait AlgorithmCore {
    fn iterate<R: MyRng>(&mut self, rng: &mut R);
    fn current_fitness(&self) -> f64;
    fn iterations(&self) -> u64;
    fn status_json(&self) -> serde_json::Value;
}

pub trait Algorithm<R>: Send {
    fn iterate(&mut self, rng: &mut R);
    fn current_fitness(&self) -> f64;
    fn iterations(&self) -> u64;
    fn status_json(&self) -> serde_json::Value;
}

impl<T: AlgorithmCore + Send> Algorithm<ThreadRng> for T {
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

impl<T: AlgorithmCore + Send> Algorithm<Pcg64> for T {
    fn iterate(&mut self, rng: &mut Pcg64) {
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
