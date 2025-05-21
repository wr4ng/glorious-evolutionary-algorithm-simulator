use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use eas::{
    algorithms::{
        mmas::{MMASbs, MMAStsp}, one_plus_one_ea::OnePlusOneEA, simulated_annealing::{CoolingSchedule, SimulatedAnnealing}, EvolutionaryAlgorithm
    },
    fitness::{leading_ones::LeadingOnes, one_max::OneMax, tsp::TSP},
    mutation::{Bitflip, SingleBitflip, TwoOpt},
};
use rand::Rng;
use rand_pcg::Pcg64;

use crate::{Algorithm, Problem, Task};

#[derive(Debug)]
pub enum CreateError {
    InvalidTSP,
    NotImplemented,
}

impl IntoResponse for CreateError {
    fn into_response(self) -> Response {
        match self {
            CreateError::InvalidTSP => {
                (StatusCode::BAD_REQUEST, "invalid tsp instance".to_string())
            }
            CreateError::NotImplemented => (StatusCode::BAD_REQUEST, "not implemented".to_string()),
        }
        .into_response()
    }
}

pub fn create_ea<R: Rng>(
    task: &Task,
    rng: &mut R
) -> Result<Box<dyn EvolutionaryAlgorithm<Pcg64>>, CreateError> {
    match task.algorithm {
        Algorithm::OnePlusOneEA => create_oneplusone_runner(&task.problem, rng),
        Algorithm::SimulatedAnnealing { cooling_schedule } => {
            create_sa_runner(&task.problem, rng, cooling_schedule)
        }
        Algorithm::ACO {alpha, beta, evap_factor, ants} => create_aco_runner(&task.problem, alpha, beta, evap_factor, ants),
    }
}

pub fn create_oneplusone_runner<R: Rng>(
    problem: &Problem,
    rng: &mut R
) -> Result<Box<dyn EvolutionaryAlgorithm<Pcg64>>, CreateError> {
    Ok(match problem {
        Problem::OneMax { bitstring_size } => Box::new(OnePlusOneEA::new(
            *bitstring_size,
            Bitflip,
            OneMax,
            rng,
        )),
        Problem::LeadingOnes { bitstring_size } => Box::new(OnePlusOneEA::new(
            *bitstring_size,
            Bitflip,
            LeadingOnes,
            rng,
        )),
        Problem::TSP { tsp_instance } => {
            let tsp = TSP::from_euc2d(&tsp_instance).ok_or(CreateError::InvalidTSP)?;
            Box::new(OnePlusOneEA::new(tsp.num_cities(), TwoOpt, tsp, rng))
        }
    })
}

pub fn create_sa_runner<R: Rng>(
    problem: &Problem,
    rng: &mut R,
    cooling_schedule: crate::CoolingSchedule,
) -> Result<Box<dyn EvolutionaryAlgorithm<Pcg64>>, CreateError> {
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
        Problem::TSP { tsp_instance } => {
            let tsp = TSP::from_euc2d(&tsp_instance).ok_or(CreateError::InvalidTSP)?;
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

pub fn create_aco_runner(
    problem: &Problem, alpha: f64, beta: f64, evap_factor: f64, ants: usize
) -> Result<Box<dyn EvolutionaryAlgorithm + Send>, CreateError> {
    Ok(match problem {
        Problem::OneMax { bitstring_size } => Box::new(MMASbs::new(
            OneMax,
            *bitstring_size,
            ants,
            alpha,
            evap_factor,
            &mut rng(),
        )),
        Problem::LeadingOnes { bitstring_size } => Box::new(MMASbs::new(
            OneMax,
            *bitstring_size,
            ants,
            alpha,
            evap_factor,
            &mut rng(),
        )),
        Problem::TSP { tsp_instance } => {
            let tsp = TSP::from_euc2d(&tsp_instance).ok_or(CreateError::InvalidTSP)?;
            let size = tsp.num_cities();
            Box::new(MMAStsp::new(
                tsp.distances(),
                tsp,
                size,
                ants,
                alpha,
                beta,
                evap_factor,
                &mut rng(),))
        }  
    })
}