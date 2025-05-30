use eas::algorithms::simulated_annealing::{CoolingSchedule, SimulatedAnnealing};
use eas::algorithms::Algorithm;
use eas::fitness::tsp::TSP;
use eas::mutation::TwoOpt;

fn main() {
    let mut rng = rand::rng();
    let n = 1_000_000;
    let tsp = TSP::from_euc2d(include_str!("./../fitness/berlin52.tsp")).unwrap();
    let cooling = CoolingSchedule::from_max_iterations_bitstring(tsp.num_vertices(), n);
    let mut sa =
        SimulatedAnnealing::new(tsp.num_vertices() as usize, TwoOpt, tsp, cooling, &mut rng);

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
    }
}
