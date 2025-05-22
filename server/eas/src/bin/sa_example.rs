use eas::algorithms::simulated_annealing::{CoolingSchedule, SimulatedAnnealing};
use eas::algorithms::EvolutionaryAlgorithm;
use eas::fitness::one_max::OneMax;
use eas::mutation::SingleBitflip;

fn main() {
    let mut rng = rand::rng();
    let size = 200;
    let n = 100_000;
    let cooling = CoolingSchedule::from_max_iterations_bitstring(size as u64, n);
    let mut sa = SimulatedAnnealing::new(size, SingleBitflip, OneMax, cooling, &mut rng);

    println!("Iteration, Fitness, Temperature");
    for i in 0..n {
        sa.iterate(&mut rng);
        if i % 1000 == 0 {
            println!(
                "{}, {}, {}",
                i,
                sa.state.current_fitness,
                sa.current_temperature()
            );
        }
        if sa.state.current_fitness == size as f64 {
            break;
        }
    }
}
