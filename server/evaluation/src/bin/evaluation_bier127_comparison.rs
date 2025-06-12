use std::time::{Duration, Instant};

use eas::{
    algorithms::{
        AlgorithmCore,
        mmas::{MMAStsp, PheromoneUpdateStrategy},
        one_plus_one_ea::OnePlusOneEA,
        simulated_annealing::{CoolingSchedule, SimulatedAnnealing},
    },
    fitness::tsp::TSP,
    mutation::TwoOpt,
};
use rand::SeedableRng;
use rand_pcg::Pcg64;

const REPETITIONS: usize = 100;
const MAX_MS: u64 = 1000;
const SEED: u64 = 1253132;

fn main() {
    let mut rng = Pcg64::seed_from_u64(SEED);

    let mut fitness_ea = Vec::with_capacity(REPETITIONS);
    for _ in 0..REPETITIONS {
        let tsp = TSP::from_euc2d(include_str!("./bier127.tsp")).unwrap();
        let start = Instant::now();
        let mut ea = OnePlusOneEA::new(tsp.num_cities(), TwoOpt, tsp, &mut rng);
        while start.elapsed() < Duration::from_millis(MAX_MS) {
            ea.iterate(&mut rng);
        }
        fitness_ea.push(ea.current_fitness());
    }

    let optimum_ea = fitness_ea
        .iter()
        .fold(0, |acc, x| if *x == 118282.0 { acc + 1 } else { acc });
    println!("(1+1) EA:");
    println!("Avg: {}", average(&fitness_ea));
    println!("Optimum: {}", optimum_ea);
    for x in fitness_ea {
        print!("{}\\\\ ", x);
    }
    println!("\n");

    let mut fitness_sa = Vec::with_capacity(REPETITIONS);
    for _ in 0..REPETITIONS {
        let tsp = TSP::from_euc2d(include_str!("./bier127.tsp")).unwrap();
        let start = Instant::now();
        let cooling = CoolingSchedule::new_default_tsp(tsp.num_cities() as u64, 20.0);
        let mut sa = SimulatedAnnealing::new(tsp.num_cities(), TwoOpt, tsp, cooling, &mut rng);
        while start.elapsed() < Duration::from_millis(MAX_MS) {
            sa.iterate(&mut rng);
        }
        fitness_sa.push(sa.current_fitness());
    }

    let optimum_sa = fitness_sa
        .iter()
        .fold(0, |acc, x| if *x == 118282.0 { acc + 1 } else { acc });
    println!("Simulated Annealing:");
    println!("Avg: {}", average(&fitness_sa));
    println!("Optimum: {}", optimum_sa);
    for x in fitness_sa {
        print!("{}\\\\ ", x);
    }
    println!("\n");

    let mut fitness_mmas_1 = Vec::with_capacity(REPETITIONS);
    for _ in 0..REPETITIONS {
        let tsp = TSP::from_euc2d(include_str!("./bier127.tsp")).unwrap();
        let start = Instant::now();
        let size = tsp.num_cities();
        let mut mmas = MMAStsp::new(
            tsp.distances(),
            tsp,
            size,
            30,
            1.0,
            5.0,
            0.05,
            PheromoneUpdateStrategy::GenerationBest,
            true,
            0.0,
            100.0,
            &mut rng,
        );
        while start.elapsed() < Duration::from_millis(MAX_MS) {
            mmas.iterate(&mut rng);
        }
        fitness_mmas_1.push(mmas.current_fitness());
    }

    let optimum_mmas_1 = fitness_mmas_1
        .iter()
        .fold(0, |acc, x| if *x == 118282.0 { acc + 1 } else { acc });
    println!("MMAS Size borders + fitness deposit:");
    println!("Avg: {}", average(&fitness_mmas_1));
    println!("Optimum: {}", optimum_mmas_1);
    for x in fitness_mmas_1 {
        print!("{}\\\\ ", x);
    }
    println!("\n");

    let mut fitness_mmas_2 = Vec::with_capacity(REPETITIONS);
    for _ in 0..REPETITIONS {
        let tsp = TSP::from_euc2d(include_str!("./bier127.tsp")).unwrap();
        let start = Instant::now();
        let size = tsp.num_cities();
        let mut mmas = MMAStsp::new(
            tsp.distances(),
            tsp,
            size,
            30,
            1.0,
            5.0,
            0.05,
            PheromoneUpdateStrategy::GenerationBest,
            true,
            0.05,
            1.0,
            &mut rng,
        );
        while start.elapsed() < Duration::from_millis(MAX_MS) {
            mmas.iterate(&mut rng);
        }
        fitness_mmas_2.push(mmas.current_fitness());
    }

    let optimum_mmas_2 = fitness_mmas_2
        .iter()
        .fold(0, |acc, x| if *x == 118282.0 { acc + 1 } else { acc });
    println!("MMAS Fitness border + fitness deposit:");
    println!("Avg: {}", average(&fitness_mmas_2));
    println!("Optimum: {}", optimum_mmas_2);
    for x in fitness_mmas_2 {
        print!("{}\\\\ ", x);
    }
    println!("\n");

    let mut fitness_mmas_3 = Vec::with_capacity(REPETITIONS);
    for _ in 0..REPETITIONS {
        let tsp = TSP::from_euc2d(include_str!("./bier127.tsp")).unwrap();
        let size = tsp.num_cities();
        let start = Instant::now();
        let mut mmas = MMAStsp::new(
            tsp.distances(),
            tsp,
            size,
            30,
            1.0,
            5.0,
            0.05,
            PheromoneUpdateStrategy::GenerationBest,
            true,
            0.0,
            0.0,
            &mut rng,
        );
        while start.elapsed() < Duration::from_millis(MAX_MS) {
            mmas.iterate(&mut rng);
        }
        fitness_mmas_3.push(mmas.current_fitness());
    }

    let optimum_mmas_3 = fitness_mmas_3
        .iter()
        .fold(0, |acc, x| if *x == 118282.0 { acc + 1 } else { acc });
    println!("MMAS Size border + rho deposit:");
    println!("Avg: {}", average(&fitness_mmas_3));
    println!("Optimum: {}", optimum_mmas_3);
    for x in fitness_mmas_3 {
        print!("{}\\\\ ", x);
    }
    println!("\n");
}

fn average(numbers: &[f64]) -> f64 {
    let sum: f64 = numbers.iter().sum();
    let count = numbers.len() as f64;
    sum / count
}
