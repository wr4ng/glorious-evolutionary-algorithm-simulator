use std::time::Instant;

use eas::algorithms::{mmas::{MMAStsp, PheromoneUpdateStrategy}, AlgorithmCore};
use rand::{Rng, SeedableRng};
use rand_pcg::Pcg64;
use eas::fitness::tsp::TSP;

const REPETITIONS: usize = 30;
const MAX_ITERATIONS: u64 = 1500;
const SEED: u64 = 24355423;
const INTERVAL: u64 = 10;

fn main() {
    // OPTIONS:
    let mut rng = Pcg64::seed_from_u64(SEED);

    let start = Instant::now();

	let mut fitness_times_ff = vec![0.0;(MAX_ITERATIONS/INTERVAL) as usize];
    let mut finals_ff = Vec::with_capacity(REPETITIONS);

	for _ in 0..REPETITIONS{
		let tsp = TSP::from_euc2d("NAME : bier127
COMMENT : 127 Biergaerten in Augsburg (Juenger/Reinelt)
TYPE : TSP
DIMENSION : 127
EDGE_WEIGHT_TYPE : EUC_2D
NODE_COORD_SECTION
   1   9860  14152
   2   9396  14616
   3  11252  14848
   4  11020  13456
   5   9512  15776
   6  10788  13804
   7  10208  14384
   8  11600  13456
   9  11252  14036
  10  10672  15080
  11  11136  14152
  12   9860  13108
  13  10092  14964
  14   9512  13340
  15  10556  13688
  16   9628  14036
  17  10904  13108
  18  11368  12644
  19  11252  13340
  20  10672  13340
  21  11020  13108
  22  11020  13340
  23  11136  13572
  24  11020  13688
  25   8468  11136
  26   8932  12064
  27   9512  12412
  28   7772  11020
  29   8352  10672
  30   9164  12876
  31   9744  12528
  32   8352  10324
  33   8236  11020
  34   8468  12876
  35   8700  14036
  36   8932  13688
  37   9048  13804
  38   8468  12296
  39   8352  12644
  40   8236  13572
  41   9164  13340
  42   8004  12760
  43   8584  13108
  44   7772  14732
  45   7540  15080
  46   7424  17516
  47   8352  17052
  48   7540  16820
  49   7888  17168
  50   9744  15196
  51   9164  14964
  52   9744  16240
  53   7888  16936
  54   8236  15428
  55   9512  17400
  56   9164  16008
  57   8700  15312
  58  11716  16008
  59  12992  14964
  60  12412  14964
  61  12296  15312
  62  12528  15196
  63  15312   6612
  64  11716  16124
  65  11600  19720
  66  10324  17516
  67  12412  13340
  68  12876  12180
  69  13688  10904
  70  13688  11716
  71  13688  12528
  72  11484  13224
  73  12296  12760
  74  12064  12528
  75  12644  10556
  76  11832  11252
  77  11368  12296
  78  11136  11020
  79  10556  11948
  80  10324  11716
  81  11484   9512
  82  11484   7540
  83  11020   7424
  84  11484   9744
  85  16936  12180
  86  17052  12064
  87  16936  11832
  88  17052  11600
  89  13804  18792
  90  12064  14964
  91  12180  15544
  92  14152  18908
  93   5104  14616
  94   6496  17168
  95   5684  13224
  96  15660  10788
  97   5336  10324
  98    812   6264
  99  14384  20184
 100  11252  15776
 101   9744   3132
 102  10904   3480
 103   7308  14848
 104  16472  16472
 105  10440  14036
 106  10672  13804
 107   1160  18560
 108  10788  13572
 109  15660  11368
 110  15544  12760
 111   5336  18908
 112   6264  19140
 113  11832  17516
 114  10672  14152
 115  10208  15196
 116  12180  14848
 117  11020  10208
 118   7656  17052
 119  16240   8352
 120  10440  14732
 121   9164  15544
 122   8004  11020
 123   5684  11948
 124   9512  16472
 125  13688  17516
 126  11484   8468
 127   3248  14152
EOF").unwrap();
			let fit = mmas_tsp_optimize_beta(tsp, 5.0, PheromoneUpdateStrategy::BestSoFar, &mut rng);
			for i in 0..fit.len() {
				fitness_times_ff[i] += fit[i]
			}
			finals_ff.push(fit[fit.len()-1]);
		}	

    // Size bounds Fitness update
	let mut fitness_times_sf = vec![0.0;(MAX_ITERATIONS/INTERVAL) as usize];
    let mut finals_sf = Vec::with_capacity(REPETITIONS);

	for _ in 0..REPETITIONS{
		let tsp = TSP::from_euc2d("NAME : bier127
COMMENT : 127 Biergaerten in Augsburg (Juenger/Reinelt)
TYPE : TSP
DIMENSION : 127
EDGE_WEIGHT_TYPE : EUC_2D
NODE_COORD_SECTION
   1   9860  14152
   2   9396  14616
   3  11252  14848
   4  11020  13456
   5   9512  15776
   6  10788  13804
   7  10208  14384
   8  11600  13456
   9  11252  14036
  10  10672  15080
  11  11136  14152
  12   9860  13108
  13  10092  14964
  14   9512  13340
  15  10556  13688
  16   9628  14036
  17  10904  13108
  18  11368  12644
  19  11252  13340
  20  10672  13340
  21  11020  13108
  22  11020  13340
  23  11136  13572
  24  11020  13688
  25   8468  11136
  26   8932  12064
  27   9512  12412
  28   7772  11020
  29   8352  10672
  30   9164  12876
  31   9744  12528
  32   8352  10324
  33   8236  11020
  34   8468  12876
  35   8700  14036
  36   8932  13688
  37   9048  13804
  38   8468  12296
  39   8352  12644
  40   8236  13572
  41   9164  13340
  42   8004  12760
  43   8584  13108
  44   7772  14732
  45   7540  15080
  46   7424  17516
  47   8352  17052
  48   7540  16820
  49   7888  17168
  50   9744  15196
  51   9164  14964
  52   9744  16240
  53   7888  16936
  54   8236  15428
  55   9512  17400
  56   9164  16008
  57   8700  15312
  58  11716  16008
  59  12992  14964
  60  12412  14964
  61  12296  15312
  62  12528  15196
  63  15312   6612
  64  11716  16124
  65  11600  19720
  66  10324  17516
  67  12412  13340
  68  12876  12180
  69  13688  10904
  70  13688  11716
  71  13688  12528
  72  11484  13224
  73  12296  12760
  74  12064  12528
  75  12644  10556
  76  11832  11252
  77  11368  12296
  78  11136  11020
  79  10556  11948
  80  10324  11716
  81  11484   9512
  82  11484   7540
  83  11020   7424
  84  11484   9744
  85  16936  12180
  86  17052  12064
  87  16936  11832
  88  17052  11600
  89  13804  18792
  90  12064  14964
  91  12180  15544
  92  14152  18908
  93   5104  14616
  94   6496  17168
  95   5684  13224
  96  15660  10788
  97   5336  10324
  98    812   6264
  99  14384  20184
 100  11252  15776
 101   9744   3132
 102  10904   3480
 103   7308  14848
 104  16472  16472
 105  10440  14036
 106  10672  13804
 107   1160  18560
 108  10788  13572
 109  15660  11368
 110  15544  12760
 111   5336  18908
 112   6264  19140
 113  11832  17516
 114  10672  14152
 115  10208  15196
 116  12180  14848
 117  11020  10208
 118   7656  17052
 119  16240   8352
 120  10440  14732
 121   9164  15544
 122   8004  11020
 123   5684  11948
 124   9512  16472
 125  13688  17516
 126  11484   8468
 127   3248  14152
EOF").unwrap();
			let fit = mmas_tsp_optimize_q(tsp, (1.0 - 1.0/127.0) / (1.0/(0.02* 135737.0)), &mut rng);
			for i in 0..fit.len() {
				fitness_times_sf[i] += fit[i]
			}
			finals_sf.push(fit[fit.len()-1]);
		}	
    // Size based
	let mut fitness_times_sr = vec![0.0;(MAX_ITERATIONS/INTERVAL) as usize];
    let mut finals_sr = Vec::with_capacity(REPETITIONS);

	for _ in 0..REPETITIONS{
		let tsp = TSP::from_euc2d("NAME : bier127
COMMENT : 127 Biergaerten in Augsburg (Juenger/Reinelt)
TYPE : TSP
DIMENSION : 127
EDGE_WEIGHT_TYPE : EUC_2D
NODE_COORD_SECTION
   1   9860  14152
   2   9396  14616
   3  11252  14848
   4  11020  13456
   5   9512  15776
   6  10788  13804
   7  10208  14384
   8  11600  13456
   9  11252  14036
  10  10672  15080
  11  11136  14152
  12   9860  13108
  13  10092  14964
  14   9512  13340
  15  10556  13688
  16   9628  14036
  17  10904  13108
  18  11368  12644
  19  11252  13340
  20  10672  13340
  21  11020  13108
  22  11020  13340
  23  11136  13572
  24  11020  13688
  25   8468  11136
  26   8932  12064
  27   9512  12412
  28   7772  11020
  29   8352  10672
  30   9164  12876
  31   9744  12528
  32   8352  10324
  33   8236  11020
  34   8468  12876
  35   8700  14036
  36   8932  13688
  37   9048  13804
  38   8468  12296
  39   8352  12644
  40   8236  13572
  41   9164  13340
  42   8004  12760
  43   8584  13108
  44   7772  14732
  45   7540  15080
  46   7424  17516
  47   8352  17052
  48   7540  16820
  49   7888  17168
  50   9744  15196
  51   9164  14964
  52   9744  16240
  53   7888  16936
  54   8236  15428
  55   9512  17400
  56   9164  16008
  57   8700  15312
  58  11716  16008
  59  12992  14964
  60  12412  14964
  61  12296  15312
  62  12528  15196
  63  15312   6612
  64  11716  16124
  65  11600  19720
  66  10324  17516
  67  12412  13340
  68  12876  12180
  69  13688  10904
  70  13688  11716
  71  13688  12528
  72  11484  13224
  73  12296  12760
  74  12064  12528
  75  12644  10556
  76  11832  11252
  77  11368  12296
  78  11136  11020
  79  10556  11948
  80  10324  11716
  81  11484   9512
  82  11484   7540
  83  11020   7424
  84  11484   9744
  85  16936  12180
  86  17052  12064
  87  16936  11832
  88  17052  11600
  89  13804  18792
  90  12064  14964
  91  12180  15544
  92  14152  18908
  93   5104  14616
  94   6496  17168
  95   5684  13224
  96  15660  10788
  97   5336  10324
  98    812   6264
  99  14384  20184
 100  11252  15776
 101   9744   3132
 102  10904   3480
 103   7308  14848
 104  16472  16472
 105  10440  14036
 106  10672  13804
 107   1160  18560
 108  10788  13572
 109  15660  11368
 110  15544  12760
 111   5336  18908
 112   6264  19140
 113  11832  17516
 114  10672  14152
 115  10208  15196
 116  12180  14848
 117  11020  10208
 118   7656  17052
 119  16240   8352
 120  10440  14732
 121   9164  15544
 122   8004  11020
 123   5684  11948
 124   9512  16472
 125  13688  17516
 126  11484   8468
 127   3248  14152
EOF").unwrap();
			let fit = mmas_tsp_optimize_rho(tsp, 0.05, &mut rng);
			for i in 0..fit.len() {
				fitness_times_sr[i] += fit[i]
			}
			finals_sr.push(fit[fit.len()-1]);
		}	


    println!("MMAS Fitness based, GenerationBest:");
    print_graph(fitness_times_ff);
    print_boxplot(finals_ff);
    println!("MMAS size-fitness, GenerationBest:");
    print_graph(fitness_times_sf);
    print_boxplot(finals_sf);
    println!("MMAS size-rho, GenerationBest:");
    print_graph(fitness_times_sr);
    print_boxplot(finals_sr);

	let duration = start.elapsed();
    println!("Duration: {:?}", duration);

}

fn mmas_tsp_optimize_beta<R: Rng>(tsp: TSP, beta: f64, update_strat: PheromoneUpdateStrategy, rng: &mut R) -> Vec<f64> {
	let size = tsp.num_cities();
	let mut mmas = MMAStsp::new(tsp.distances(), tsp, size, size, 1.0, beta, 0.02, update_strat, true, 0.05, 1.0, rng);
	let mut fitnesses = Vec::with_capacity((MAX_ITERATIONS/INTERVAL) as usize);
	for i in 0..MAX_ITERATIONS {
		mmas.iterate(rng);
		if i % INTERVAL == 0 {
			fitnesses.push(mmas.current_fitness());
		}
		if mmas.current_fitness() == 118282.0{
			break;
		}
	}
	while fitnesses.len() != (MAX_ITERATIONS/INTERVAL) as usize {fitnesses.push(118282.0);}
	fitnesses
}

fn mmas_tsp_optimize_q<R: Rng>(tsp: TSP, q: f64, rng: &mut R) -> Vec<f64> {
	let size = tsp.num_cities();
	let mut mmas = MMAStsp::new(tsp.distances(), tsp, size, size, 1.0, 5.0, 0.02, PheromoneUpdateStrategy::GenerationBest, true, 0.0, q, rng);
	let mut fitnesses = Vec::with_capacity((MAX_ITERATIONS/INTERVAL) as usize);
	for i in 0..MAX_ITERATIONS {
		mmas.iterate(rng);
		if i % INTERVAL == 0 {
			fitnesses.push(mmas.current_fitness());
		}
		if mmas.current_fitness() == 118282.0{
			break;
		}
	}
	while fitnesses.len() != (MAX_ITERATIONS/INTERVAL) as usize {fitnesses.push(118282.0);}
	fitnesses
}

fn mmas_tsp_optimize_rho<R: Rng>(tsp: TSP, rho: f64, rng: &mut R) -> Vec<f64> {
	let size = tsp.num_cities();
	let mut mmas = MMAStsp::new(tsp.distances(), tsp, size, size, 1.0, 5.0, rho, PheromoneUpdateStrategy::GenerationBest, true, 0.0, 0.0, rng);
	let mut fitnesses = Vec::with_capacity((MAX_ITERATIONS/INTERVAL) as usize);
	for i in 0..MAX_ITERATIONS {
		mmas.iterate(rng);
		if i % INTERVAL == 0 {
			fitnesses.push(mmas.current_fitness());
		}
		if mmas.current_fitness() == 118282.0{
			break;
		}
	}
	while fitnesses.len() != (MAX_ITERATIONS/INTERVAL) as usize {fitnesses.push(118282.0);}
	fitnesses
}

fn print_graph(values: Vec<f64>) {
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
fn print_boxplot(values: Vec<f64>) {
    for f in &values {
        print!("{}\\\\ ", f);
    }
    println!("\n");
}