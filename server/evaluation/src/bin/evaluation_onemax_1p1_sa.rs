use std::time::Instant;

use eas::algorithms::AlgorithmCore;
use eas::algorithms::one_plus_one_ea::OnePlusOneEA;
use eas::algorithms::simulated_annealing::{CoolingSchedule, SimulatedAnnealing};
use eas::fitness::one_max::OneMax;
use eas::mutation::{Bitflip, SingleBitflip};
use rand::rng;

fn main() {
    let runs_per_size = 100;
    let problem_size = vec![100, 200, 500, 1000, 1500, 2000, 2500, 3000, 3500, 4000];
    let mut averages = vec![];

    let start = Instant::now();

    for n in problem_size {
        let mut average_1p1 = 0.0;
        let mut average_rls = 0.0;
        for _ in 0..runs_per_size {
            let iterations_1p1 = oneplusone_onemax_optimize(n);
            let iterations_rls = rls_onemax_optimize(n);
            average_1p1 += (iterations_1p1 as f64) / (runs_per_size as f64);
            average_rls += (iterations_rls as f64) / (runs_per_size as f64);
        }
        averages.push((n, average_1p1, average_rls));
    }

    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);
    println!("Runs per size: {:?}", runs_per_size);

    println!("n, average (1+1) EA, average RLS");
    for (n, a, b) in &averages {
        println!("{}, {:.2}, {:.2}", n, a, b);
    }

    println!("Tikz coordinate format:");
    println!("(1+1) EA:");
    for (n, a, _) in &averages {
        print!("({}, {:.2})", n, a);
    }
    println!("\nRLS:");
    for (n, _, b) in &averages {
        print!("({}, {:.2})", n, b);
    }
}

fn oneplusone_onemax_optimize(size: usize) -> u64 {
    let mut ea = OnePlusOneEA::new(size, Bitflip, OneMax, &mut rng());
    loop {
        ea.iterate(&mut rng());
        if ea.current_fitness() as usize == size {
            break;
        }
    }
    ea.iterations()
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
