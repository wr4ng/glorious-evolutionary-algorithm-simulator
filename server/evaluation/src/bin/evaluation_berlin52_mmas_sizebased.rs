use std::time::Instant;

use eas::algorithms::{mmas::{MMAStsp, PheromoneUpdateStrategy}, AlgorithmCore};
use rand::{Rng, SeedableRng};
use rand_pcg::Pcg64;
use eas::fitness::tsp::TSP;

const REPETITIONS: usize = 30;
const MAX_ITERATIONS: u64 = 1000;
const SEED: u64 = 123456789;
const INTERVAL: u64 = 10;

fn main() {
    // OPTIONS:
    let mut rng = Pcg64::seed_from_u64(SEED);
	let rhos = vec![0.02, 0.05, 0.1, 0.2, 0.5, 0.75];

    let start = Instant::now();

	let mut fitness_times = Vec::with_capacity(rhos.len());
	let mut optimal_finds = Vec::with_capacity(rhos.len());

    
    for rho in rhos.clone() {
		let mut fitnesses = vec![0.0; (MAX_ITERATIONS/INTERVAL) as usize];
		let mut optimals = 0;
		for _ in 0..REPETITIONS{
			let tsp = TSP::from_euc2d("NAME: berlin52
TYPE: TSP
COMMENT: 52 locations in Berlin (Groetschel)
DIMENSION: 52
EDGE_WEIGHT_TYPE: EUC_2D
NODE_COORD_SECTION
1 565.0 575.0
2 25.0 185.0
3 345.0 750.0
4 945.0 685.0
5 845.0 655.0
6 880.0 660.0
7 25.0 230.0
8 525.0 1000.0
9 580.0 1175.0
10 650.0 1130.0
11 1605.0 620.0
12 1220.0 580.0
13 1465.0 200.0
14 1530.0 5.0
15 845.0 680.0
16 725.0 370.0
17 145.0 665.0
18 415.0 635.0
19 510.0 875.0
20 560.0 365.0
21 300.0 465.0
22 520.0 585.0
23 480.0 415.0
24 835.0 625.0
25 975.0 580.0
26 1215.0 245.0
27 1320.0 315.0
28 1250.0 400.0
29 660.0 180.0
30 410.0 250.0
31 420.0 555.0
32 575.0 665.0
33 1150.0 1160.0
34 700.0 580.0
35 685.0 595.0
36 685.0 610.0
37 770.0 610.0
38 795.0 645.0
39 720.0 635.0
40 760.0 650.0
41 475.0 960.0
42 95.0 260.0
43 875.0 920.0
44 700.0 500.0
45 555.0 815.0
46 830.0 485.0
47 1170.0 65.0
48 830.0 610.0
49 605.0 625.0
50 595.0 360.0
51 1340.0 725.0
52 1740.0 245.0
EOF

").unwrap();
			let fit = mmas_tsp_optimize_rho(tsp, rho,&mut rng);
			for i in 0..fit.len() {
				fitnesses[i] += fit[i]
			}
			if fit[fit.len()-1] == 7542.0 {optimals += 1}
		}
		fitness_times.push(fitnesses);
		optimal_finds.push(optimals);
	}

	for (i, results) in fitness_times.into_iter().enumerate() {
        println!("MMAS Fitness based, GenerationBest (rho = {}):", rhos[i]);
		println!("Found optimal solution {} times", optimal_finds[i]);
        print_results(results);
    }

	let duration = start.elapsed();
    println!("Duration: {:?}", duration);

}

fn mmas_tsp_optimize_rho<R: Rng>(tsp: TSP, rho: f64, rng: &mut R) -> Vec<f64> {
	let size = tsp.num_cities();
	let mut mmas = MMAStsp::new(tsp.distances(), tsp, size, 52, 1.0, 4.0, rho, PheromoneUpdateStrategy::GenerationBest, true, 0.0, 0.0, rng);
	let mut fitnesses = Vec::with_capacity((MAX_ITERATIONS/INTERVAL) as usize);
	for i in 0..MAX_ITERATIONS {
		mmas.iterate(rng);
		if i % INTERVAL == 0 {
			fitnesses.push(mmas.current_fitness());
		}
		if mmas.current_fitness() == 7542.0{
			break;
		}
	}
	while fitnesses.len() != (MAX_ITERATIONS/INTERVAL) as usize {fitnesses.push(7542.0);}
	fitnesses
}

fn print_results(values: Vec<f64>) {
	let mut graph= Vec::with_capacity((MAX_ITERATIONS/INTERVAL) as usize);
	for i in 0..values.len(){
		graph.push((i*(INTERVAL as usize), values[i]/(REPETITIONS as f64)))
	}

    // Print graf points
    for f in &graph {
		print!("({},{}) ", f.0, f.1);
    }
    println!("\n");
}