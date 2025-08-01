use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use eas::{
    algorithms::{
        Algorithm,
        mmas::{MMASbs, MMAStsp, PheromoneUpdateStrategy},
        one_plus_one_ea::OnePlusOneEA,
        simulated_annealing::{CoolingSchedule, SimulatedAnnealing},
    },
    fitness::{leading_ones::LeadingOnes, one_max::OneMax, tsp::TSP},
    mutation::{Bitflip, SingleBitflip, TwoOpt},
};
use rand::Rng;
use rand_pcg::Pcg64;

use crate::{AlgorithmConfig, Problem, Task, UpdateStrategy};

#[derive(Debug)]
pub enum CreateError {
    InvalidTSP,
}

impl IntoResponse for CreateError {
    fn into_response(self) -> Response {
        match self {
            CreateError::InvalidTSP => {
                (StatusCode::BAD_REQUEST, "invalid tsp instance".to_string())
            }
        }
        .into_response()
    }
}

// Create a Algorithm trait object to allow iterating any implemented algorithm
// Returns a CreateError if an invalid TSP instance is provided
pub fn create_ea<R: Rng>(
    task: &Task,
    rng: &mut R,
) -> Result<Box<dyn Algorithm<Pcg64>>, CreateError> {
    match task.algorithm {
        AlgorithmConfig::OnePlusOneEA => create_oneplusone_runner(&task.problem, rng),
        AlgorithmConfig::SimulatedAnnealing { cooling_schedule } => {
            create_sa_runner(&task.problem, rng, cooling_schedule)
        }
        AlgorithmConfig::ACO {
            alpha,
            beta,
            evap_factor,
            ants,
            p_best,
            q,
            nn,
            update_strategy,
        } => create_aco_runner(
            &task.problem,
            alpha,
            beta,
            evap_factor,
            ants,
            p_best.unwrap_or(0.0),
            q.unwrap_or(0.0),
            nn,
            update_strategy,
            rng,
        ),
    }
}

// Create a (1+1) EA instance given a problem
pub fn create_oneplusone_runner<R: Rng>(
    problem: &Problem,
    rng: &mut R,
) -> Result<Box<dyn Algorithm<Pcg64>>, CreateError> {
    Ok(match problem {
        Problem::OneMax { bitstring_size } => {
            Box::new(OnePlusOneEA::new(*bitstring_size, Bitflip, OneMax, rng))
        }
        Problem::LeadingOnes { bitstring_size } => Box::new(OnePlusOneEA::new(
            *bitstring_size,
            Bitflip,
            LeadingOnes,
            rng,
        )),
        Problem::TSP {
            tsp_instance,
            tsp_name: _,
        } => {
            let tsp = TSP::from_euc2d(tsp_instance).ok_or(CreateError::InvalidTSP)?;
            Box::new(OnePlusOneEA::new(tsp.num_cities(), TwoOpt, tsp, rng))
        }
    })
}

// Create a Simulated Annealing instance, given provided problem and cooling schedule
pub fn create_sa_runner<R: Rng>(
    problem: &Problem,
    rng: &mut R,
    cooling_schedule: crate::CoolingSchedule,
) -> Result<Box<dyn Algorithm<Pcg64>>, CreateError> {
    Ok(match problem {
        Problem::OneMax { bitstring_size } => {
            let c = match cooling_schedule {
                crate::CoolingSchedule::Static { temperature } => {
                    CoolingSchedule::new_static(temperature)
                }
                crate::CoolingSchedule::Exponential { cooling_rate } => {
                    CoolingSchedule::new_default_bitstring(*bitstring_size as u64, cooling_rate)
                }
            };
            Box::new(SimulatedAnnealing::new(
                *bitstring_size,
                SingleBitflip,
                OneMax,
                c,
                rng,
            ))
        }
        Problem::LeadingOnes { bitstring_size } => {
            let c = match cooling_schedule {
                crate::CoolingSchedule::Static { temperature } => {
                    CoolingSchedule::new_static(temperature)
                }
                crate::CoolingSchedule::Exponential { cooling_rate } => {
                    CoolingSchedule::new_default_bitstring(*bitstring_size as u64, cooling_rate)
                }
            };
            Box::new(SimulatedAnnealing::new(
                *bitstring_size,
                SingleBitflip,
                LeadingOnes,
                c,
                rng,
            ))
        }
        Problem::TSP {
            tsp_instance,
            tsp_name: _,
        } => {
            let tsp = TSP::from_euc2d(tsp_instance).ok_or(CreateError::InvalidTSP)?;
            let c = match cooling_schedule {
                crate::CoolingSchedule::Static { temperature } => {
                    CoolingSchedule::new_static(temperature)
                }
                crate::CoolingSchedule::Exponential { cooling_rate } => {
                    CoolingSchedule::new_default_tsp(tsp.num_cities() as u64, cooling_rate)
                }
            };
            Box::new(SimulatedAnnealing::new(
                tsp.num_cities(),
                TwoOpt,
                tsp,
                c,
                rng,
            ))
        }
    })
}

// Create an MMAS instance given a problem and MMAS parameters
pub fn create_aco_runner<R: Rng>(
    problem: &Problem,
    alpha: f64,
    beta: f64,
    evap_factor: f64,
    ants: usize,
    p_best: f64,
    q: f64,
    nn: bool,
    strategy: UpdateStrategy,
    rng: &mut R,
) -> Result<Box<dyn Algorithm<Pcg64>>, CreateError> {
    Ok(match problem {
        Problem::OneMax { bitstring_size } => Box::new(MMASbs::new(
            OneMax,
            *bitstring_size,
            ants,
            alpha,
            evap_factor,
            rng,
        )),
        Problem::LeadingOnes { bitstring_size } => Box::new(MMASbs::new(
            LeadingOnes,
            *bitstring_size,
            ants,
            alpha,
            evap_factor,
            rng,
        )),
        Problem::TSP {
            tsp_instance,
            tsp_name: _,
        } => {
            let tsp = TSP::from_euc2d(tsp_instance).ok_or(CreateError::InvalidTSP)?;
            let size = tsp.num_cities();
            Box::new(MMAStsp::new(
                tsp.distances(),
                tsp,
                size,
                ants,
                alpha,
                beta,
                evap_factor,
                map_strategy(strategy),
                nn,
                p_best,
                q,
                rng,
            ))
        }
    })
}

fn map_strategy(strategy: UpdateStrategy) -> PheromoneUpdateStrategy {
    match strategy {
        UpdateStrategy::BestSoFar => PheromoneUpdateStrategy::BestSoFar,
        UpdateStrategy::GenerationBest => PheromoneUpdateStrategy::GenerationBest,
        UpdateStrategy::AllAnts => PheromoneUpdateStrategy::AllAnts,
    }
}
