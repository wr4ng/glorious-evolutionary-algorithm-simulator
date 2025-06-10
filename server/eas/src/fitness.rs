use super::search_space::SearchSpace;

pub mod leading_ones;
pub mod one_max;
pub mod tsp;

// Trait defining a fitness function given a search space
// Needs to be able to evaluate a search point and yield a fitness value
// Also defines whether the fitness function is maximizing or minimizing,
// which is used to be able to compare two fitness values and return an std::cmd::Ordering
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
