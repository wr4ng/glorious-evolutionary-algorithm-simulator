use super::search_space::SearchSpace;

pub mod leading_ones;
pub mod one_max;
pub mod tsp;

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
