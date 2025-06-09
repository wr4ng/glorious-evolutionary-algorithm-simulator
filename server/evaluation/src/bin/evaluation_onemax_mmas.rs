use std::time::Instant;

use eas::algorithms::AlgorithmCore;
use eas::algorithms::mmas::MMASbs;
use eas::fitness::one_max::OneMax;
use rand::{Rng, SeedableRng};
use rand_pcg::Pcg64;

const REPETITIONS: usize = 50;
const SEED: u64 = 111222111;

fn main() {
    let problem_sizes = vec![100, 200, 500, 1000, 1500, 2000, 2500, 3000, 3500, 4000];
    let mut averages = vec![];
    let mut rng = Pcg64::seed_from_u64(SEED);

    let start = Instant::now();

    for n in problem_sizes {
        let mut avg = 0.0;

        for _ in 0..REPETITIONS {
            let iterations = mmas_onemax_optimize(n, &mut rng);
            avg += (iterations as f64) / (REPETITIONS as f64);
        }
        averages.push((n, avg));
    }

    let duration = start.elapsed();
    println!("Time elapsed: {:?}\n", duration);

    for (n, avg) in averages {
        print!("({}, {}) ", n, avg);
    }
    println!("");
}

fn mmas_onemax_optimize<R: Rng>(size: usize, rng: &mut R) -> u64 {
    let mut mmas = MMASbs::new(OneMax, size, 1, 1.0, 1.0, rng);
    while mmas.current_fitness() != size as f64 {
        mmas.iterate(rng);
    }
    mmas.iterations()
}
