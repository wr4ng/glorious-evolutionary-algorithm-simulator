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
const MAX_MS: u64 = 150;
const SEED: u64 = 111222333;

fn main() {
    let mut rng = Pcg64::seed_from_u64(SEED);

    let mut fitness_ea = Vec::with_capacity(REPETITIONS);
    for _ in 0..REPETITIONS {
        let tsp = TSP::from_euc2d(include_str!("./berlin52.tsp")).unwrap();
        let start = Instant::now();
        let mut ea = OnePlusOneEA::new(tsp.num_cities(), TwoOpt, tsp, &mut rng);
        while start.elapsed() < Duration::from_millis(MAX_MS) {
            ea.iterate(&mut rng);
        }
        fitness_ea.push(ea.current_fitness());
    }

    println!("(1+1) EA: {}", average(&fitness_ea));
    for x in fitness_ea {
        print!("{}\\\\ ", x);
    }
    println!("\n");

    let mut fitness_sa = Vec::with_capacity(REPETITIONS);
    for _ in 0..REPETITIONS {
        let tsp = TSP::from_euc2d(include_str!("./berlin52.tsp")).unwrap();
        let start = Instant::now();
        let cooling = CoolingSchedule::new_default_tsp(tsp.num_cities() as u64, 30.0);
        let mut sa = SimulatedAnnealing::new(tsp.num_cities(), TwoOpt, tsp, cooling, &mut rng);
        while start.elapsed() < Duration::from_millis(MAX_MS) {
            sa.iterate(&mut rng);
        }
        fitness_sa.push(sa.current_fitness());
    }

    println!("Simulated Annealing: {}", average(&fitness_sa));
    for x in fitness_sa {
        print!("{}\\\\ ", x);
    }
    println!("\n");

    let mut fitness_mmas_1 = Vec::with_capacity(REPETITIONS);
    for _ in 0..REPETITIONS {
        let tsp = TSP::from_euc2d(include_str!("./berlin52.tsp")).unwrap();
        let start = Instant::now();
        let size = tsp.num_cities();
        let mut mmas = MMAStsp::new(
            tsp.distances(),
            tsp,
            size,
            size,
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

    println!(
        "MMAS Size borders + fitness deposit: {}",
        average(&fitness_mmas_1)
    );
    for x in fitness_mmas_1 {
        print!("{}\\\\ ", x);
    }
    println!("\n");

    let mut fitness_mmas_2 = Vec::with_capacity(REPETITIONS);
    for _ in 0..REPETITIONS {
        let tsp = TSP::from_euc2d(include_str!("./berlin52.tsp")).unwrap();
        let start = Instant::now();
        let size = tsp.num_cities();
        let mut mmas = MMAStsp::new(
            tsp.distances(),
            tsp,
            size,
            size,
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

    println!(
        "MMAS Fitness border + fitness deposit: {}",
        average(&fitness_mmas_2)
    );
    for x in fitness_mmas_2 {
        print!("{}\\\\ ", x);
    }
    println!("\n");

    let mut fitness_mmas_3 = Vec::with_capacity(REPETITIONS);
    for _ in 0..REPETITIONS {
        let tsp = TSP::from_euc2d(include_str!("./berlin52.tsp")).unwrap();
        let size = tsp.num_cities();
        let start = Instant::now();
        let mut mmas = MMAStsp::new(
            tsp.distances(),
            tsp,
            size,
            size,
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

    println!(
        "MMAS Size border + rho deposit: {}",
        average(&fitness_mmas_3)
    );
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
