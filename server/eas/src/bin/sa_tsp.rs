use eas::algorithms::simulated_annealing::DefaultTSPSchedule;
use eas::algorithms::simulated_annealing::SimulatedAnnealing;
use eas::algorithms::EvolutionaryAlgorithm;
use eas::fitness::tsp::TSP;
use eas::mutation::TwoOpt;

fn main() {
    let mut rng = rand::rng();
    let n = 1_000_000;
    let tsp = TSP::from_euc2d(include_str!("./../fitness/berlin52.tsp")).unwrap();
    let cooling = DefaultTSPSchedule::from_max_iterations(tsp.num_vertices(), n);
    let mut sa =
        SimulatedAnnealing::new(tsp.num_vertices() as usize, TwoOpt, tsp, cooling, &mut rng);

    println!("Iteration, Fitness, Temperature");
    for i in 0..n {
        let _ = sa.iterate(&mut rng);
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
