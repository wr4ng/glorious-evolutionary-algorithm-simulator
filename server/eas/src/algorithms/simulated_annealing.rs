use rand::{rngs::ThreadRng, Rng};
use serde_json::json;

use crate::{fitness::FitnessFunction, mutation::Mutation, search_space::SearchSpace};

use super::{EvolutionaryAlgorithm, SimulationState};

pub trait CoolingSchedule {
    fn temperature(&self, t: u64) -> f64;
}

// Default general cooling schedule for bitstring problems
// T(1) = n^3 where n should be initialized to the length of the bitstring (size)
// T(t) = n^3 * (1 - 1/cn)^t where c is a parameter given to the cooling schedule.
// A constructor is provided to calculate c such that T(t') = 1 for some value t'
pub struct DefaultBitstringSchedule {
    size: u64,
    c: f64,
}

impl DefaultBitstringSchedule {
    pub fn new(size: u64, c: f64) -> Self {
        Self { size, c }
    }

    pub fn from_max_iterations(size: u64, max_iterations: u64) -> Self {
        // -1 / (n * ((1/n^3)^(1/t) - 1))
        let c = -1.0
            / (size as f64
                * (((1.0 / ((size as f64).powi(3))).powf(1.0 / max_iterations as f64)) - 1.0));
        Self { size, c }
    }
}

impl CoolingSchedule for DefaultBitstringSchedule {
    fn temperature(&self, t: u64) -> f64 {
        if t == 1 {
            return self.size.pow(3) as f64;
        }
        // n^3 * (1 - 1/cn)^t
        self.size.pow(3) as f64 * (1.0 - 1.0 / (self.c * self.size as f64)).powi(t as i32)
    }
}

// Default cooling schedule for TSP (and other permutation based problems)
// T(1) = n^3 where n should be initialized to the number of vertices (permutation length)
// T(t) = n^3 * (1 - 1/cn^2)^t where c is a parameter given to the cooling schedule.
// A constructor is provided to calculate c such that T(t') = 1 for some value t'
pub struct DefaultTSPSchedule {
    size: u64,
    c: f64,
}

impl DefaultTSPSchedule {
    pub fn new(size: u64, c: f64) -> Self {
        Self { size, c }
    }

    pub fn from_max_iterations(size: u64, max_iterations: u64) -> Self {
        // -1 / (n^2 * ((1/n^3)^(1/t) - 1))
        let c = -1.0
            / (size.pow(2) as f64
                * (((1.0 / ((size as f64).powi(3))).powf(1.0 / max_iterations as f64)) - 1.0));
        Self { size, c }
    }
}

impl CoolingSchedule for DefaultTSPSchedule {
    fn temperature(&self, t: u64) -> f64 {
        if t == 1 {
            return self.size.pow(3) as f64;
        }
        // n^3 * (1 - 1/cn^2)^t
        self.size.pow(3) as f64 * (1.0 - 1.0 / (self.c * (self.size).pow(2) as f64)).powi(t as i32)
    }
}

pub struct SimulatedAnnealing<S, F, M, C>
where
    S: SearchSpace,
    F: FitnessFunction<S>,
    M: Mutation<S>,
    C: CoolingSchedule,
{
    pub state: SimulationState<S>,
    fitness: F,
    mutator: M,
    cooling: C,
}

impl<S, F, M, C> SimulatedAnnealing<S, F, M, C>
where
    S: SearchSpace,
    F: FitnessFunction<S>,
    M: Mutation<S>,
    C: CoolingSchedule,
{
    pub fn new<R: Rng>(size: usize, mutator: M, fitness: F, cooling: C, mut rng: R) -> Self {
        let current_solution = S::new_random(size, &mut rng);
        let current_fitness = fitness.evaluate(&current_solution);
        SimulatedAnnealing {
            state: SimulationState {
                iteration: 0,
                current_solution,
                current_fitness,
            },
            fitness,
            mutator,
            cooling,
        }
    }

    pub fn current_temperature(&self) -> f64 {
        self.cooling.temperature(self.state.iteration)
    }
}

impl<S, F, M, C> EvolutionaryAlgorithm for SimulatedAnnealing<S, F, M, C>
where
    S: SearchSpace,
    F: FitnessFunction<S>,
    M: Mutation<S>,
    C: CoolingSchedule,
{
    fn iterate(&mut self, rng: &mut ThreadRng) {
        let neighbor = self.mutator.apply(&self.state.current_solution, rng);
        let neighbor_fitness = self.fitness.evaluate(&neighbor);

        let fitness_cmp = self
            .fitness
            .compare(neighbor_fitness, self.state.current_fitness);

        if fitness_cmp == std::cmp::Ordering::Greater {
            self.state.current_solution = neighbor;
            self.state.current_fitness = neighbor_fitness
        } else {
            let difference = (neighbor_fitness - self.state.current_fitness).abs();
            let temp = self.cooling.temperature(self.state.iteration);
            let accept_probability = (-difference / temp).exp();
            let accept = rng.random_bool(accept_probability);
            if accept {
                self.state.current_solution = neighbor;
                self.state.current_fitness = neighbor_fitness
            }
        }
        self.state.iteration += 1;
    }

    fn current_fitness(&self) -> f64 {
        self.state.current_fitness
    }

    fn status_json(&self) -> serde_json::Value {
        json!({
            "current_fitness": &self.state.current_fitness
        })
    }
}

//TODO: Test simulated annealing
