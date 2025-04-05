use std::path;

use crate::fitness;

use super::search_space::{Bitstring, Permutation, SearchSpace};



pub trait Pheromone<S: SearchSpace> {
	fn apply(&mut self, path: &Vec<S>, fitness_values: &Vec<f64>); //change fit to usize
	fn decrease(&mut self, decrease_factor: f64);
	fn pheromones(&self, node: usize) -> &Vec<f64>;
}

// Ant System ----
pub struct AS {
	pheromones: Vec<Vec<f64>>
}

impl AS {
	fn new(x: usize,y: usize) -> Self {
		AS { pheromones: vec![vec![0.0;y]; x]}
	}
}

impl Pheromone<Permutation> for AS {
	fn apply(&mut self, paths: &Vec<Permutation>, fitness_values: &Vec<f64>){
		for path in 0..paths.len() {
			let p = paths[path].permutation();
			let fitness = fitness_values[path];
			let len = p.len();
			
			// Edge case
			self.pheromones[p[len-1]][p[0]] += 1.0/fitness;
			self.pheromones[p[0]][p[len-1]] += 1.0/fitness;

			for step in 0..(len-1) {
				self.pheromones[p[step]][p[step+1]] += 1.0/fitness;
				self.pheromones[p[step + 1]][p[step]] += 1.0/fitness;
			}
		}
	}
	
	fn decrease(&mut self, decrease_factor: f64) {
		for i in 0..self.pheromones.len() {
			for ii in 0..self.pheromones.len() {
				self.pheromones[i][ii] *= decrease_factor;
			}
		}
	}
	
	fn pheromones(&self, node: usize) -> &Vec<f64> {
			&self.pheromones[node]
		}
		
}

impl Pheromone<Bitstring> for AS {
	fn apply(&mut self, paths: &Vec<Bitstring>, fitness: &Vec<f64>) {
		for i in 0.. paths.len(){
			let p = paths[i].bits();
			let len = p.len();
			for i in 0..len {
				self.pheromones[i][if p[i] {1} else {0}] += 1.0/fitness[i]
			}
		}
	}
	
	fn decrease(&mut self, decrease_factor: f64) {
		for i in 0..self.pheromones.len() {
			for ii in 0..=1 {
				self.pheromones[i][ii] *= decrease_factor;
			}
		}
	}
		
	fn pheromones(&self, node: usize) -> &Vec<f64> {
		&self.pheromones[node]
	}

}

// MMAS -----
pub struct MMAS {
	pheromones: Vec<Vec<f64>>,
	max: f64,
	min: f64,
}

impl MMAS {
	fn new(x: usize,y: usize, max: f64, min: f64) -> Self {
		MMAS { pheromones: vec![vec![min;y]; x], max, min}
	}

	fn clamp(&mut self){
		for i in 0..self.pheromones.len(){
			for ii in 0..self.pheromones[i].len(){
				if self.pheromones[i][ii] < self.min {
					self.pheromones[i][ii] = self.min
				}
				else if self.pheromones[i][ii] > self.max {
					self.pheromones[i][ii] = self.max
				}
			}
		}
	}
}

impl Pheromone<Bitstring> for MMAS{
	fn apply(&mut self, paths: &Vec<Bitstring>, fitness: &Vec<f64>) {
		for i in 0.. paths.len(){
			let p = paths[i].bits();
			let len = p.len();
			for i in 0..len {
				self.pheromones[i][if p[i] {1} else {0}] += 1.0/fitness[i]
			}
		}
		self.clamp();
	}

	fn decrease(&mut self, decrease_factor: f64) {
		for i in 0..self.pheromones.len() {
			for ii in 0..2 {
				self.pheromones[i][ii] *= decrease_factor;
			}
		}
	}

	fn pheromones(&self, node: usize) -> &Vec<f64> {
		&self.pheromones[node]
	}
}
impl Pheromone<Permutation> for MMAS{
	fn apply(&mut self, paths: &Vec<Permutation>, fitness: &Vec<f64>) {
		for path in 0..paths.len() {
			let p = paths[path].permutation();
			let fitness = fitness[path];
			let len = p.len();
			
			// Edge case
			self.pheromones[p[len-1]][p[0]] += 1.0/fitness;
			self.pheromones[p[0]][p[len-1]] += 1.0/fitness;

			for step in 0..(len-1) {
				self.pheromones[p[step]][p[step+1]] += 1.0/fitness;
				self.pheromones[p[step + 1]][p[step]] += 1.0/fitness;
			}
		}
		self.clamp();
	}

	fn decrease(&mut self, decrease_factor: f64) {
		for i in 0..self.pheromones.len() {
			for ii in 0..=1 {
				self.pheromones[i][ii] *= decrease_factor;
			}
		}
	}

	fn pheromones(&self, node: usize) -> &Vec<f64> {
		&self.pheromones[node]
	}
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn bitstring_pheromone_update() {
//         let expected_values = vec![vec![0.0, 1.0/64.0];5];
// 		let fit = 64; 
// 		let path = Bitstring::new(vec![true; 5]);
// 		let mut p: AS = AS::new(5, 2);
// 		p.apply(&path, fit as f64);
// 		for i in 0..5{
// 			assert_eq!(*<AS as Pheromone<Bitstring>>::pheromones(&p, i), expected_values[i]);
// 		}
//     }
// 	#[test]
//     fn bitstring_pheromone_update_false() {
//         let expected_values = vec![vec![1.0/64.0, 0.0];5];
// 		let fit = 64; 
// 		let path = Bitstring::new(vec![false; 5]);
// 		let mut p: AS = AS::new(5, 2);
// 		p.apply(&path, fit as f64);
// 		for i in 0..5{
// 			assert_eq!(*<AS as Pheromone<Bitstring>>::pheromones(&p, i), expected_values[i]);
// 		}
//     }
// 	#[test]
//     fn bitstring_pheromone_decrease() {
//         let expected_values = vec![vec![1.0/64.0 * 0.9, 0.0];5];
// 		let fit = 64; 
// 		let path = Bitstring::new(vec![false; 5]);
// 		let mut p: AS = AS::new(5, 2);
// 		p.apply(&path, fit as f64);
// 		<AS as Pheromone<Bitstring>>::decrease(&mut p, 0.9 as f64);
// 		for i in 0..5{
// 			assert_eq!(*<AS as Pheromone<Bitstring>>::pheromones(&p, i), expected_values[i]);
// 		}
//     }
// 	#[test]
//     fn permutation_pheromone_update() {
//         let expected_values = vec![vec![1.0/64.0, 0.0];5];
// 		let fit = 64; 
// 		let path = Bitstring::new(vec![false; 5]);
// 		let mut p: AS = AS::new(5, 2);
// 		p.apply(&path, fit as f64);
// 		for i in 0..5{
// 			assert_eq!(*<AS as Pheromone<Bitstring>>::pheromones(&p, i), expected_values[i]);
// 		}
//     }
// 	#[test]
//     fn permutation_pheromone_decrease() {
//         let expected_values = vec![vec![1.0/64.0 * 0.9, 0.0];5];
// 		let fit = 64; 
// 		let path = Bitstring::new(vec![false; 5]);
// 		let mut p: AS = AS::new(5, 2);
// 		p.apply(&path, fit as f64);
// 		<AS as Pheromone<Bitstring>>::decrease(&mut p, 0.9 as f64);
// 		for i in 0..5{
// 			assert_eq!(*<AS as Pheromone<Bitstring>>::pheromones(&p, i), expected_values[i]);
// 		}
//     }
// 	#[test]
//     fn MMAS_clamp_min() {
// 		let expected_values = vec![vec![0.2,0.2];5];
		
// 		let mut p: MMAS = MMAS::new(5, 2, 1.2, 0.2);
// 		<MMAS as Pheromone<Bitstring>>::decrease(&mut p, 0.9 as f64);
// 		p.clamp();
// 		for i in 0..5{
// 			assert_eq!(*<MMAS as Pheromone<Bitstring>>::pheromones(&p, i), expected_values[i]);
// 		}
//     }
	
// 	#[test]
//     fn MMAS_clamp_max() {
// 		let expected_values = vec![vec![0.2,1.0];5];
// 		let fit = 1.0; 
// 		let path = Bitstring::new(vec![true; 5]);

// 		let mut p: MMAS = MMAS::new(5, 2, 1.0, 0.2);
// 		<MMAS as Pheromone<Bitstring>>::decrease(&mut p, 0.9);
// 		p.apply(&path, fit);
// 		for i in 0..5{
// 			assert_eq!(*<MMAS as Pheromone<Bitstring>>::pheromones(&p, i), expected_values[i]);
// 		}
//     }
// }