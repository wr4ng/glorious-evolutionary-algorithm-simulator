use super::search_space::{Bitstring, Permutation, SearchSpace};



pub trait Pheromone<S: SearchSpace> {
	fn apply(&mut self, path: &S, fitness: f64); //change fit to usize
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
	
	// TODO WHY WONT BURROW?
	fn pheromones(&self, node: usize) -> &Vec<f64> {
			&self.pheromones[node]
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

impl Pheromone<Bitstring> for MMAS{
	fn apply(&mut self, path: &Bitstring, fitness: f64) {
		todo!()
	}

	fn decrease(&mut self, decrease_factor: f64) {
		todo!()
	}

	fn pheromones(&self, node: usize) -> &Vec<f64> {
		&self.pheromones[node]
	}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bitstring_pheromone_update() {
        let expected_values = vec![vec![0.0, 1.0/64.0];5];
		let fit = 64; 
		let path = Bitstring::new(vec![true; 5]);
		let mut p: AS = AS::new(5, 2);
		p.apply(&path, fit as f64);
		for i in 0..5{
			assert_eq!(*<AS as Pheromone<Bitstring>>::pheromones(&p, i), expected_values[i]);
		}
    }
	#[test]
    fn bitstring_pheromone_update_false() {
        let expected_values = vec![vec![1.0/64.0, 0.0];5];
		let fit = 64; 
		let path = Bitstring::new(vec![false; 5]);
		let mut p: AS = AS::new(5, 2);
		p.apply(&path, fit as f64);
		for i in 0..5{
			assert_eq!(*<AS as Pheromone<Bitstring>>::pheromones(&p, i), expected_values[i]);
		}
    }
	#[test]
    fn bitstring_pheromone_decrease() {
        let expected_values = vec![vec![1.0/64.0 * 0.9, 0.0];5];
		let fit = 64; 
		let path = Bitstring::new(vec![false; 5]);
		let mut p: AS = AS::new(5, 2);
		p.apply(&path, fit as f64);
		<AS as Pheromone<Bitstring>>::decrease(&mut p, 0.9 as f64);
		for i in 0..5{
			assert_eq!(*<AS as Pheromone<Bitstring>>::pheromones(&p, i), expected_values[i]);
		}
    }
	#[test]
    fn permutation_pheromone_update() {
        let expected_values = vec![vec![1.0/64.0, 0.0];5];
		let fit = 64; 
		let path = Bitstring::new(vec![false; 5]);
		let mut p: AS = AS::new(5, 2);
		p.apply(&path, fit as f64);
		for i in 0..5{
			assert_eq!(*<AS as Pheromone<Bitstring>>::pheromones(&p, i), expected_values[i]);
		}
    }
	#[test]
    fn permutation_pheromone_decrease() {
        let expected_values = vec![vec![1.0/64.0 * 0.9, 0.0];5];
		let fit = 64; 
		let path = Bitstring::new(vec![false; 5]);
		let mut p: AS = AS::new(5, 2);
		p.apply(&path, fit as f64);
		<AS as Pheromone<Bitstring>>::decrease(&mut p, 0.9 as f64);
		for i in 0..5{
			assert_eq!(*<AS as Pheromone<Bitstring>>::pheromones(&p, i), expected_values[i]);
		}
    }

}