use super::{EvolutionaryAlgorithm, SimulationState};
use crate::{
    fitness::FitnessFunction,
    mutation::Mutation,
    search_space::{Bitstring, Permutation, SearchSpace},
};
use rand::Rng;

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

// Implementation of (1+1) EA for a given fitness function and mutation operating on bitstrings.
// Here the mutation is applied once each iteration.
impl<F, M> EvolutionaryAlgorithm<Bitstring, F> for OnePlusOneEA<Bitstring, F, M>
where
    F: FitnessFunction<Bitstring>,
    M: Mutation<Bitstring>,
{
    fn iterate<R: Rng>(&mut self, rng: &mut R) -> &SimulationState<Bitstring> {
        let offspring = self.mutator.apply(&self.state.current_solution, rng);

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

//TODO:
// Implementation of (1+1) EA for a given fitness function and mutation operating on permutations.
// Here the mutation is applied x times each iteration where x ~ Poisson(1).
impl<F, M> EvolutionaryAlgorithm<Permutation, F> for OnePlusOneEA<Permutation, F, M>
where
    F: FitnessFunction<Permutation>,
    M: Mutation<Permutation>,
{
    fn iterate<R: Rng>(&mut self, _: &mut R) -> &SimulationState<Permutation> {
        todo!()
    }
}
