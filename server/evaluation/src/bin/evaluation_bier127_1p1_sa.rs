use std::time::Instant;

use eas::algorithms::AlgorithmCore;
use eas::algorithms::one_plus_one_ea::OnePlusOneEA;
use eas::algorithms::simulated_annealing::{CoolingSchedule, SimulatedAnnealing};
use eas::fitness::tsp::TSP;
use eas::mutation::TwoOpt;
use rand::{Rng, SeedableRng};
use rand_pcg::Pcg64;

const REPETITIONS: usize = 100;
const MAX_ITERATIONS: u64 = 500_000;
const SEED: u64 = 123123123;

fn main() {
    // OPTIONS:
    let c_values = vec![1.25, 1.5, 1.75, 2.0, 2.25, 2.5, 2.75, 3.0, 3.25];

    let mut rng = Pcg64::seed_from_u64(SEED);

    let start = Instant::now();

    // RUN (1+1) EA
    let mut fitness_1p1 = Vec::with_capacity(REPETITIONS);

    for _ in 0..REPETITIONS {
        let tsp = TSP::from_euc2d(include_str!("./bier127.tsp")).unwrap();
        let fitness = run_oneplusone(tsp, MAX_ITERATIONS, &mut rng);
        fitness_1p1.push(fitness);
    }

    // RUN RLS
    let mut fitness_rls = Vec::with_capacity(REPETITIONS);

    for _ in 0..REPETITIONS {
        let tsp = TSP::from_euc2d(include_str!("./bier127.tsp")).unwrap();
        let fitness = run_rls(tsp, MAX_ITERATIONS, &mut rng);
        fitness_rls.push(fitness);
    }

    // RUN SA with different c values
    let mut fitness_sa = vec![Vec::with_capacity(REPETITIONS); c_values.len()];

    for (i, c) in c_values.iter().enumerate() {
        for _ in 0..REPETITIONS {
            let tsp = TSP::from_euc2d(include_str!("./bier127.tsp")).unwrap();
            let fitness = run_sa(tsp, *c, MAX_ITERATIONS, &mut rng);
            fitness_sa[i].push(fitness);
        }
    }

    println!("Repetitions: {}", REPETITIONS);
    println!("Max iterations: {}", MAX_ITERATIONS);
    println!("Seed: {}\n", SEED);

    println!("(1+1) EA:");
    print_results(fitness_1p1);

    println!("RLS:");
    print_results(fitness_rls);

    for (i, results) in fitness_sa.into_iter().enumerate() {
        println!("SA (c = {}):", c_values[i]);
        print_results(results);
    }

    let duration = start.elapsed();
    println!("Duration: {:?}", duration);
}

fn run_oneplusone<R: Rng>(tsp: TSP, max_iterations: u64, rng: &mut R) -> f64 {
    let mut ea = OnePlusOneEA::new(tsp.num_cities(), TwoOpt, tsp, rng);
    for _ in 0..max_iterations {
        ea.iterate(rng);
    }
    ea.current_fitness()
}

fn run_rls<R: Rng>(tsp: TSP, max_iterations: u64, rng: &mut R) -> f64 {
    let c = CoolingSchedule::new_static(0.0);
    let mut rls = SimulatedAnnealing::new(tsp.num_cities(), TwoOpt, tsp, c, rng);
    for _ in 0..max_iterations {
        rls.iterate(rng);
    }
    rls.current_fitness()
}

fn run_sa<R: Rng>(tsp: TSP, c: f64, max_iterations: u64, rng: &mut R) -> f64 {
    let c = CoolingSchedule::new_default_tsp(tsp.num_cities() as u64, c);
    let mut sa = SimulatedAnnealing::new(tsp.num_cities(), TwoOpt, tsp, c, rng);
    for _ in 0..max_iterations {
        sa.iterate(rng);
    }
    sa.current_fitness()
}

fn print_results(values: Vec<f64>) {
    // Compute average
    let average = values.iter().sum::<f64>() / (REPETITIONS as f64);
    println!("Average: {}", average);

    // Compute times optimum found
    let optimum_found: u64 = values
        .iter()
        .map(|v| if *v == 118282.0 { 1 } else { 0 })
        .sum();
    println!("Optimum found: {}", optimum_found);

    // Print values
    for f in &values {
        print!("{}\\\\ ", f);
    }
    println!("\n");
}
