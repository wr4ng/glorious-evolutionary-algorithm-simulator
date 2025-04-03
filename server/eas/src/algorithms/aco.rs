use super::{EvolutionaryAlgorithm, SimulationState};
use crate::{
    fitness::FitnessFunction, pheromone::Pheromone, search_space::{Bitstring, Permutation, SearchSpace}
};
use rand::Rng;

pub struct ACO<S: SearchSpace, F: FitnessFunction<S>, P: Pheromone<S>> {
	pub state: SimulationState<S>,
	fitness_function: F,
	pheromone: P,
    decrease_factor: f64,
    ants: usize,
    size: usize,
    alpha: f64,
    beta: f64,
}

impl<S, F, P> ACO<S, F, P>
where
	S: SearchSpace,
	F: FitnessFunction<S>,
	P: Pheromone<S>,
{
	pub fn new<R: Rng>(size: usize, fitness_function: F, pheromone: P, ants: usize, decrease_factor: f64, alpha: f64,
        beta: f64, mut rng: R) -> Self {
		let current_solution = S::new_random(size, &mut rng);
        let current_fitness = fitness_function.evaluate(&current_solution);
        ACO {
            state: SimulationState {
                iteration: 0,
                current_solution,
                current_fitness,
            },
            fitness_function,
            pheromone,
            decrease_factor,
            ants,
            size,
            alpha,
            beta,   
        }
	}

    pub fn update_pheromones(&mut self, paths: Vec<S>){
        self.pheromone.decrease(self.decrease_factor);
        for path in paths {
            let fit = self.fitness_function.evaluate(&path);

            self.pheromone.apply(&path, fit);

            let fit_cmp = self
            .fitness_function.compare(fit, self.state.current_fitness);

            if fit_cmp == std::cmp::Ordering::Greater {
                self.state.current_solution = path;
                self.state.current_fitness = fit;
            }
        }
    }
}

impl<F, P> EvolutionaryAlgorithm<Bitstring, F> for ACO<Bitstring, F, P>
where
    F: FitnessFunction<Bitstring>,
    P: Pheromone<Bitstring>,
{
    fn iterate<R: Rng>(&mut self, rng: &mut R) -> &SimulationState<Bitstring> {
        // Generate paths
        let mut paths = Vec::with_capacity(self.ants);
        let half:f64 = 1.0/2.0;
        for ant in 0..self.ants{
            let mut values = Vec::<f64>::with_capacity(2);
            let mut path = Vec::<bool>::with_capacity(self.size);
            for step in 0..self.size{
                let weights = self.pheromone.pheromones(step);
                let mut total_weight = 0.0;
                for i in 0..=1 { //TODO clone
                    let val = weights[i].powf(self.alpha) + half.powf(self.beta);
                    total_weight += val;
                    values[i] = val;
                }
                let num = rng.random_range(0.0..=1.0);
                if values[0]/total_weight > num{
                    path[step] = false
                }
                else {
                    path[step] = true
                }
            }
            paths[ant] = Bitstring::new(path);
        }

        // Update pheromones
        self.update_pheromones(paths);
        
        self.state.iteration += 1;
        &self.state
    }
}

impl<F, P> EvolutionaryAlgorithm<Permutation, F> for ACO<Permutation, F, P>
where
    F: FitnessFunction<Permutation>,
    P: Pheromone<Permutation>,
{
    fn iterate<R: Rng>(&mut self, rng: &mut R) -> &SimulationState<Permutation> {
        // Generate paths
        let mut paths = Vec::with_capacity(self.ants);
        let mut visited = vec![false; self.size];
        let inv_size: f64 = 1.0 / self.size as f64;
        for ant in 0..self.ants{
            let mut path = Vec::<usize>::with_capacity(self.size);
            let mut current_node = 0;
            visited[0] = true;
            let mut possible_nexts = Vec::<usize>::with_capacity(self.size + 1);
            let mut possible_weights = Vec::<f64>::with_capacity(self.size + 1); //possible fenwick tree
            for step in 0..self.size{
                let weights = self.pheromone.pheromones(current_node);
                let mut total_weight = 0.0;
                let mut added: usize = 1;
                for node in 0..self.size{
                    if visited[node] {continue;}
                    possible_nexts[added] = node;
                    let val = weights[node].powf(self.alpha) + inv_size.powf(self.beta);
                    total_weight += val;
                    possible_weights[added] = val;
                    added += 1
                } 
                let num = rng.random_range(0.0..=1.0);
                let mut find_node = 1;
                while possible_weights[find_node] / total_weight < num{
                    find_node += 1;
                }
                current_node = possible_nexts[find_node];
                path[step] = current_node;
                visited[current_node] = true;
            }
            paths[ant] = Permutation::new(path);
        }

        // Update pheromones
        self.update_pheromones(paths);
        
        self.state.iteration += 1;
        &self.state
    }
}
