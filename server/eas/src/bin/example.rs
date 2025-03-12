use eas::algorithms::{one_plus_one_ea::OnePlusOneEA, EvolutionaryAlgorithm};
use eas::fitness::one_max::OneMax;
use eas::mutation::Bitflip;

fn main() {
    //TODO: Example
    let mut rng = rand::rng();
    let mut ea = OnePlusOneEA::new(20, Bitflip, OneMax, &mut rng);
    println!("Initial state: {:?}", ea.state);
    let n = 100;
    for _ in 0..n {
        let _ = ea.iterate(&mut rng);
    }
    println!("{} iterations: {:?}", n, ea.state);
}
