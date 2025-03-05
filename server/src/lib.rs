#![allow(dead_code)]

use fitness::FitnessFunction;
use mutation::Mutation;
use rand::Rng;
use search_space::SearchSpace;
use std::fmt::Debug;

pub mod fitness;
pub mod mutation;
pub mod search_space;

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

pub struct OnePlusOneEA<S: SearchSpace, F: FitnessFunction<S>, M: Mutation<S>> {
    pub state: SimulationState<S>,
    fitness_function: F,
    mutator: M,
}

impl<S, F, M> OnePlusOneEA<S, F, M>
where
    S: SearchSpace,
    F: FitnessFunction<S>,
    M: Mutation<S>,
{
    pub fn new<R: Rng>(size: usize, mutator: M, fitness_function: F, mut rng: R) -> Self {
        let current_solution = S::new_random(size, &mut rng);
        let current_fitness = fitness_function.evaluate(&current_solution);
        OnePlusOneEA {
            state: SimulationState {
                iteration: 0,
                current_solution,
                current_fitness,
            },
            fitness_function,
            mutator,
        }
    }
}

impl<S, F, M> EvolutionaryAlgorithm<S, F> for OnePlusOneEA<S, F, M>
where
    S: SearchSpace,
    F: FitnessFunction<S>,
    M: Mutation<S>,
{
    fn iterate<R: Rng>(&mut self, rng: &mut R) -> &SimulationState<S> {
        let offspring = self.mutator.apply(&mut self.state.current_solution, rng);

        let new_fitness = self.fitness_function.evaluate(&offspring);

        self.state.iteration += 1;

        let fitness_order = self
            .fitness_function
            .compare(new_fitness, self.state.current_fitness);

        if fitness_order == std::cmp::Ordering::Greater {
            self.state.current_fitness = new_fitness;
            self.state.current_solution = offspring;
        }

        &self.state
    }
}
