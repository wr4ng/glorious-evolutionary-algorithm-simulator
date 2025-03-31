use super::search_space::{Bitstring, Permutation, SearchSpace};



pub trait Pheromone<S: SearchSpace> {
	fn apply(&mut self, path: &S, fitness: f64);
	fn decrease(&mut self, decrease_factor: f64);
	fn new(&self, size: usize) -> Self;
	fn pheromones(&self, node: usize) -> Vec<f64>;
}

// Ant System ----
pub struct AS {
	pheromones: Vec<Vec<f64>>
}

impl Pheromone<Permutation> for AS {
	fn apply(&mut self, path: &Permutation, fitness: f64){
		let p = path.permutation();
		let len = p.len();
		// TODO hardcode edge case
		for step in 0..len {
			self.pheromones[p[step]][p[(step+1) % len]] += 1.0/fitness;
			self.pheromones[p[(step+1) % len]][p[step]] += 1.0/fitness;
		}
	}
	
	fn decrease(&mut self, decrease_factor: f64) {
		for i in 0..self.pheromones.len() {
			for ii in 0..self.pheromones.len() {
				self.pheromones[i][ii] *= decrease_factor;
			}
		}
	}
	
	fn new(&self, size: usize) -> Self {
			AS { pheromones: vec![vec![0.0; 2]; size] }
		}
		
	// TODO WHY WONT BURROW?
	fn pheromones(&self, node: usize) -> Vec<f64> {
			self.pheromones[node].clone()
		}
		
}

impl Pheromone<Bitstring> for AS {
	fn apply(&mut self, path: &Bitstring, fitness: f64) {
		let p = path.bits();
		let len = p.len();
		for i in 0..len {
			self.pheromones[i][if p[i] {1} else {0}] += 1.0/fitness
		}
	}
	
	fn decrease(&mut self, decrease_factor: f64) {
		for i in 0..self.pheromones.len() {
			for ii in 0..=1 {
				self.pheromones[i][ii] *= decrease_factor;
			}
		}
	}
	
	fn new(&self, size: usize) -> Self {
		AS { pheromones: vec![vec![0.0; size]; size] }
		}
		
	fn pheromones(&self, node: usize) -> Vec<f64> {
		self.pheromones[node].clone()
	}

}

// MMAS -----
pub struct MMAS {
	pheromones: Vec<Vec<f64>>,
	top: f64,
	bot: f64,
}

impl Pheromone<Bitstring> for MMAS{
	fn apply(&mut self, path: &Bitstring, fitness: f64) {
		todo!()
	}

	fn decrease(&mut self, decrease_factor: f64) {
		todo!()
	}

	fn new(&self, size: usize) -> Self {
		MMAS { pheromones: vec![vec![0.0; 2]; size], top: 1.0, bot: 0.1 } //TODO changability of top and bot
	}

	fn pheromones(&self, node: usize) -> Vec<f64> {
		self.pheromones[node].clone()
	}
}