use server::fitness::one_max::OneMax;
use server::mutation::NaiveBitflip;
use server::EvolutionaryAlgorithm;
use server::OnePlusOneEA;

fn main() {
    //TODO: Example
    let mut rng = rand::rng();
    let mut ea = OnePlusOneEA::new(8, NaiveBitflip, OneMax, &mut rng);
    println!("Initial state: {:?}", ea.state);
    for _ in 0..10 {
        let _ = ea.iterate(&mut rng);
    }
    println!("10 iterations: {:?}", ea.state);
}
