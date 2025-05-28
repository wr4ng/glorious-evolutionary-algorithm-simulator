use serde_json::json;

use crate::{fitness::FitnessFunction, mutation::Mutation, rng::MyRng, search_space::SearchSpace};

use super::{EvolutionaryAlgorithmCore, SimulationState};

pub enum CoolingSchedule {
    Static(f64),
    Exponential(f64, f64),
}

impl CoolingSchedule {
    pub fn temperature(&self, t: u64) -> f64 {
        match self {
            CoolingSchedule::Static(static_temp) => *static_temp,
            CoolingSchedule::Exponential(initial, alpha) => initial * alpha.powi(t as i32),
        }
    }

    pub fn new_static(temperature: f64) -> Self {
        CoolingSchedule::Static(temperature)
    }

    pub fn new_default_bitstring(size: u64, c: f64) -> Self {
        let initial_temp = size.pow(3) as f64; // T(0) = n^3
        let alpha = 1.0 - 1.0 / (c * size as f64); // alpha = 1 - 1/cn
        CoolingSchedule::Exponential(initial_temp, alpha)
    }

    pub fn new_default_tsp(size: u64, c: f64) -> Self {
        let initial_temp = size.pow(3) as f64; // T(0) = n^3
        let alpha = 1.0 - 1.0 / (c * size.pow(2) as f64); // alpha = 1 - 1/cn^2
        CoolingSchedule::Exponential(initial_temp, alpha)
    }

    pub fn from_max_iterations_tsp(size: u64, max_iterations: u64) -> Self {
        // c = -1 / (n^2 * ((1/n^3)^(1/t) - 1))
        let c = -1.0
            / (size.pow(2) as f64
                * (((1.0 / ((size as f64).powi(3))).powf(1.0 / max_iterations as f64)) - 1.0));
        Self::new_default_tsp(size, c)
    }

    pub fn from_max_iterations_bitstring(size: u64, max_iterations: u64) -> Self {
        // c = -1 / (n * ((1/n^3)^(1/t) - 1))
        let c = -1.0
            / (size as f64
                * (((1.0 / ((size as f64).powi(3))).powf(1.0 / max_iterations as f64)) - 1.0));
        Self::new_default_tsp(size, c)
    }
}

pub struct SimulatedAnnealing<S, F, M>
where
    S: SearchSpace,
    F: FitnessFunction<S>,
    M: Mutation<S>,
{
    pub state: SimulationState<S>,
    fitness: F,
    mutator: M,
    cooling: CoolingSchedule,
}

impl<S, F, M> SimulatedAnnealing<S, F, M>
where
    S: SearchSpace,
    F: FitnessFunction<S>,
    M: Mutation<S>,
{
    pub fn new<R: MyRng>(
        size: usize,
        mutator: M,
        fitness: F,
        cooling: CoolingSchedule,
        rng: &mut R,
    ) -> Self {
        let current_solution = S::new_random(size, rng);
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

impl<S, F, M> EvolutionaryAlgorithmCore for SimulatedAnnealing<S, F, M>
where
    S: SearchSpace,
    F: FitnessFunction<S>,
    M: Mutation<S>,
{
    fn iterate<R: MyRng>(&mut self, rng: &mut R) {
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
            // Account for NaN when temp gets too close to 0
            if accept_probability.is_finite() {
                let accept = rng.random_bool(accept_probability);
                if accept {
                    self.state.current_solution = neighbor;
                    self.state.current_fitness = neighbor_fitness
                }
            }
        }
        self.state.iteration += 1;
    }

    fn current_fitness(&self) -> f64 {
        self.state.current_fitness
    }

    fn iterations(&self) -> u64 {
        self.state.iteration
    }

    fn status_json(&self) -> serde_json::Value {
        json!({
            "iterations": self.state.iteration,
            "current_fitness": self.state.current_fitness,
            "current_solution": self.state.current_solution.to_string(),
            "temperature": format!("{:.5}", self.current_temperature()),
        })
    }
}

//TODO: Test simulated annealing
