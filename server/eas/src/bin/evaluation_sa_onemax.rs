use std::time::Instant;

use eas::algorithms::simulated_annealing::{CoolingSchedule, SimulatedAnnealing};
use eas::algorithms::EvolutionaryAlgorithmCore;
use eas::fitness::one_max::OneMax;
use eas::mutation::SingleBitflip;
use rand::rng;

fn main() {
    let runs_per_size = 100;
    let problem_size = vec![250, 500, 1000, 2000];
    let cooling_rates = vec![0, 1, 5, 10];
    let mut averages = vec![];

    let start = Instant::now();

    for c in cooling_rates {
        let mut c_averages = vec![];
        for n in &problem_size {
            let mut average = 0.0;
            for _ in 0..runs_per_size {
                let iterations = if c == 0 {
                    rls_onemax_optimize(*n)
                } else {
                    sa_onemax_optimize(*n, c as f64)
                };
                println!("n={} c={} i={}", n, c, iterations);
                average += (iterations as f64) / (runs_per_size as f64);
            }
            c_averages.push((n, average));
        }
        averages.push((c, c_averages));
    }

    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);
    println!("Runs per size: {:?}", runs_per_size);

    println!("n, c, average");
    for (c, avg) in &averages {
        for (n, a) in avg {
            println!("{}, {}, {:.2}", n, c, a);
        }
    }

    println!("Tikz coordinate format:");


    for (c, avg) in &averages {
        println!("\nCooling rate: {}", c);
        for (n, a) in avg {
            print!("({}, {:.2})", n, a);
        }
    }
}

fn rls_onemax_optimize(size: usize) -> u64 {
    let c = CoolingSchedule::new_static(0.0);
    let mut rls = SimulatedAnnealing::new(size, SingleBitflip, OneMax, c, &mut rng());
    loop {
        rls.iterate(&mut rng());
        if rls.current_fitness() as usize == size {
            break;
        }
    }
    rls.iterations()
}

fn sa_onemax_optimize(size: usize, c: f64) -> u64 {
    let c = CoolingSchedule::new_default_bitstring(size as u64, c);
    let mut rls = SimulatedAnnealing::new(size, SingleBitflip, OneMax, c, &mut rng());
    loop {
        rls.iterate(&mut rng());
        if rls.current_fitness() as usize == size {
            break;
        }
    }
    rls.iterations()
}
