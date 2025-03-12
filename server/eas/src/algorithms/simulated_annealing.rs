use rand::Rng;

use crate::{fitness::FitnessFunction, mutation::Mutation, search_space::SearchSpace};

use super::{EvolutionaryAlgorithm, SimulationState};

pub trait CoolingSchedule {
    fn temperature(&self, t: usize) -> f64;
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
}

impl<S, F, M, C> EvolutionaryAlgorithm<S, F> for SimulatedAnnealing<S, F, M, C>
where
    S: SearchSpace,
    F: FitnessFunction<S>,
    M: Mutation<S>,
    C: CoolingSchedule,
{
    fn iterate<R: Rng>(&mut self, rng: &mut R) -> &SimulationState<S> {
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
        &self.state
    }
}
