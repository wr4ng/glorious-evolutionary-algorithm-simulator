use std::time::Instant;

use eas::algorithms::mmas::MMASbs;
use eas::{algorithms::AlgorithmCore, fitness::leading_ones::LeadingOnes};
use rand::{Rng, SeedableRng};
use rand_pcg::Pcg64;

const REPETITIONS: usize = 50;
const SEED: u64 = 19658696;

fn main() {
    let problem_sizes = vec![50, 100, 200, 400, 600, 800, 1000];
    let mut averages = vec![];
    let mut rng = Pcg64::seed_from_u64(SEED);

    let start = Instant::now();

    for n in problem_sizes {
        let mut avg = 0.0;

        for _ in 0..REPETITIONS {
            let iterations = mmas_leadingones_optimize(n, &mut rng);
            avg += (iterations as f64) / (REPETITIONS as f64);
        }
        averages.push((n, avg));
    }

    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);
    println!("Repetitions: {}", REPETITIONS);
    println!("Seed: {}", SEED);
    println!("Results:");

    for (n, avg) in averages {
        print!("({}, {}) ", n, avg);
    }
    println!("");
}

fn mmas_leadingones_optimize<R: Rng>(size: usize, rng: &mut R) -> u64 {
    let mut mmas = MMASbs::new(LeadingOnes, size, 1, 1.0, 0.5, rng);
    while mmas.current_fitness() != size as f64 {
        mmas.iterate(rng);
    }
    mmas.iterations()
}
