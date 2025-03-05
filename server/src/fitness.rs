use super::search_space::{Bitstring, SearchSpace};

pub trait FitnessFunction<T: SearchSpace> {
    fn evaluate(&self, instance: &T) -> f64;
    fn is_maximizing(&self) -> bool;

    fn compare(&self, a: f64, b: f64) -> std::cmp::Ordering {
        if self.is_maximizing() {
            a.total_cmp(&b)
        } else {
            a.total_cmp(&b).reverse()
        }
    }
}

pub struct OneMax;

impl FitnessFunction<Bitstring> for OneMax {
    fn evaluate(&self, instance: &Bitstring) -> f64 {
        return instance
            .bits()
            .iter()
            .fold(0_f64, |acc, b| if *b { acc + 1_f64 } else { acc });
    }
    fn is_maximizing(&self) -> bool {
        true
    }
}

//TODO: struct LeadingOnes;
//TODO: struct TSP {}
// determine how we want to store distances (probably distance matrix to support non-euclidian
// instances
// impl FitnessFunction<Permutation> for TSP {}
// impl TSP { from_EUC2D() -> TSP }
