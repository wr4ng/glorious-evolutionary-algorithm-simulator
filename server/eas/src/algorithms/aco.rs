// use super::{EvolutionaryAlgorithm, SimulationState};
// use crate::{
//     fitness::FitnessFunction, pheromone::Pheromone, search_space::{Bitstring, Permutation, SearchSpace}
// };
// use rand::Rng;

// pub struct ACO<S: SearchSpace, F: FitnessFunction<S>, P: Pheromone<S>> {
// 	pub state: SimulationState<S>,
// 	fitness_function: F,
// 	pheromone: P,
//     decrease_factor: f64,
//     ants: usize,
// <<<<<<< HEAD
//     size: usize,
//     alpha: f64,
//     beta: f64,
// =======
//     size: usize
// >>>>>>> 7051a0ca54b7980bdace0939ab5924a00bb4d9de
// }

// impl<S, F, P> ACO<S, F, P>
// where
// 	S: SearchSpace,
// 	F: FitnessFunction<S>,
// 	P: Pheromone<S>,
// {
// <<<<<<< HEAD
// 	pub fn new<R: Rng>(size: usize, fitness_function: F, pheromone: P, ants: usize, decrease_factor: f64, alpha: f64,
//         beta: f64, mut rng: R) -> Self {
// =======
// 	pub fn new<R: Rng>(size: usize, fitness_function: F, pheromone: P, ants: usize, decrease_factor: f64, mut rng: R) -> Self {
// >>>>>>> 7051a0ca54b7980bdace0939ab5924a00bb4d9de
// 		let current_solution = S::new_random(size, &mut rng);
//         let current_fitness = fitness_function.evaluate(&current_solution);
//         ACO {
//             state: SimulationState {
//                 iteration: 0,
//                 current_solution,
//                 current_fitness,
//             },
//             fitness_function,
//             pheromone,
//             decrease_factor,
//             ants,
// <<<<<<< HEAD
//             size,
//             alpha,
//             beta,   
//         }
// 	}

//     pub fn update_pheromones(&mut self, paths: &Vec<S>){
//         self.pheromone.decrease(self.decrease_factor);
//         let mut fitness_values = Vec::<f64>::with_capacity(self.ants);
//         for path in 0..paths.len() {
//             let fit = self.fitness_function.evaluate(&paths[path]);
//             fitness_values[path] = fit;

//             let fit_cmp = self
//             .fitness_function.compare(fit, self.state.current_fitness);
        
//             if fit_cmp == std::cmp::Ordering::Greater {
//                 self.state.current_solution = paths[path].clone();
//                 self.state.current_fitness = fit;
//             }
//         }
//     self.pheromone.apply(paths, &fitness_values);
// =======
//             size,   
//         }
// 	}

//     pub fn update_pheromones(&mut self, paths: Vec<S>){
//         self.pheromone.decrease(self.decrease_factor);
//         for path in paths {
//             let fit = self.fitness_function.evaluate(&path);

//             self.pheromone.apply(&path, fit);

//             let fit_cmp = self
//             .fitness_function.compare(fit, self.state.current_fitness);

//             if fit_cmp == std::cmp::Ordering::Greater {
//                 self.state.current_solution = path;
//                 self.state.current_fitness = fit;
//             }
//         }
// >>>>>>> 7051a0ca54b7980bdace0939ab5924a00bb4d9de
//     }
// }

// impl<F, P> EvolutionaryAlgorithm<Bitstring, F> for ACO<Bitstring, F, P>
// where
//     F: FitnessFunction<Bitstring>,
//     P: Pheromone<Bitstring>,
// {
//     fn iterate<R: Rng>(&mut self, rng: &mut R) -> &SimulationState<Bitstring> {
//         // Generate paths
//         let mut paths = Vec::with_capacity(self.ants);
// <<<<<<< HEAD
//         let half:f64 = 1.0/2.0;
//         for ant in 0..self.ants{
//             let mut values = Vec::<f64>::with_capacity(2);
// =======
//         for ant in 0..self.ants{
// >>>>>>> 7051a0ca54b7980bdace0939ab5924a00bb4d9de
//             let mut path = Vec::<bool>::with_capacity(self.size);
//             for step in 0..self.size{
//                 let weights = self.pheromone.pheromones(step);
//                 let mut total_weight = 0.0;
// <<<<<<< HEAD
//                 for i in 0..=1 { //TODO clone
//                     let val = weights[i].powf(self.alpha) + half.powf(self.beta);
//                     total_weight += val;
//                     values[i] = val;
//                 }
//                 let num = rng.random_range(0.0..=1.0);
//                 if values[0]/total_weight > num{
// =======
//                 for w in weights.clone() { //TODO clone
//                     if w > 0.1 {    //TODO figue out constant
//                         total_weight += w;
//                     }
//                     else {
//                         total_weight += 0.1
//                     }
//                 }
//                 let num = rng.random_range(0.0..=1.0);
//                 if weights[0]/total_weight > num{
// >>>>>>> 7051a0ca54b7980bdace0939ab5924a00bb4d9de
//                     path[step] = false
//                 }
//                 else {
//                     path[step] = true
//                 }
//             }
//             paths[ant] = Bitstring::new(path);
//         }

//         // Update pheromones
// <<<<<<< HEAD
//         self.update_pheromones(&paths);
// =======
//         self.update_pheromones(paths);
// >>>>>>> 7051a0ca54b7980bdace0939ab5924a00bb4d9de
        
//         self.state.iteration += 1;
//         &self.state
//     }
// }

// impl<F, P> EvolutionaryAlgorithm<Permutation, F> for ACO<Permutation, F, P>
// where
//     F: FitnessFunction<Permutation>,
//     P: Pheromone<Permutation>,
// {
//     fn iterate<R: Rng>(&mut self, rng: &mut R) -> &SimulationState<Permutation> {
//         // Generate paths
//         let mut paths = Vec::with_capacity(self.ants);
//         let mut visited = vec![false; self.size];
// <<<<<<< HEAD
//         let inv_size: f64 = 1.0 / self.size as f64;
//         for ant in 0..self.ants{
//             let mut path = Vec::<usize>::with_capacity(self.size);
//             let mut current_node = 0;
//             visited[0] = true;
// =======
//         for ant in 0..self.ants{
//             let mut path = Vec::<usize>::with_capacity(self.size);
//             let mut current_node = 0;
// >>>>>>> 7051a0ca54b7980bdace0939ab5924a00bb4d9de
//             let mut possible_nexts = Vec::<usize>::with_capacity(self.size + 1);
//             let mut possible_weights = Vec::<f64>::with_capacity(self.size + 1); //possible fenwick tree
//             for step in 0..self.size{
//                 let weights = self.pheromone.pheromones(current_node);
//                 let mut total_weight = 0.0;
//                 let mut added: usize = 1;
//                 for node in 0..self.size{
//                     if visited[node] {continue;}
//                     possible_nexts[added] = node;
// <<<<<<< HEAD
//                     let val = weights[node].powf(self.alpha) + inv_size.powf(self.beta);
//                     total_weight += val;
//                     possible_weights[added] = val;
//                     added += 1
//                 } 
//                 let num = rng.random_range(0.0..=1.0);
//                 let mut find_node = 1;
// =======
//                     if weights[node] > 0.1 { //TODO replace in pheromones
//                         possible_weights[added] += possible_weights[added-1] + weights[node];
//                         total_weight += weights[node];
//                     }
//                     else {
//                         possible_weights[added] += possible_weights[added-1] + 0.1;
//                         total_weight += 0.1;
//                     }
//                     added += 1
//                 } 
//                 let num = rng.random_range(0.0..=1.0);
//                 let mut find_node = 0;
// >>>>>>> 7051a0ca54b7980bdace0939ab5924a00bb4d9de
//                 while possible_weights[find_node] / total_weight < num{
//                     find_node += 1;
//                 }
//                 current_node = possible_nexts[find_node];
//                 path[step] = current_node;
//                 visited[current_node] = true;
//             }
//             paths[ant] = Permutation::new(path);
//         }

//         // Update pheromones
// <<<<<<< HEAD
//         self.update_pheromones(&paths);
// =======
//         self.update_pheromones(paths);
// >>>>>>> 7051a0ca54b7980bdace0939ab5924a00bb4d9de
        
//         self.state.iteration += 1;
//         &self.state
//     }
// }
