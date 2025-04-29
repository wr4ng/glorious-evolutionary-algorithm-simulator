use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use eas::{
    algorithms::{
        EvolutionaryAlgorithm,
        one_plus_one_ea::OnePlusOneEA,
        simulated_annealing::{DefaultBitstringSchedule, DefaultTSPSchedule, SimulatedAnnealing},
    },
    fitness::{leading_ones::LeadingOnes, one_max::OneMax, tsp::TSP},
    mutation::{Bitflip, SingleBitflip, TwoOpt},
};
use rand::rng;

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

pub fn create_ea(task: &Task) -> Result<Box<dyn EvolutionaryAlgorithm + Send>, CreateError> {
    match task.algorithm {
        Algorithm::OnePlusOneEA => create_oneplusone_runner(&task.problem),
        Algorithm::SimulatedAnnealing { cooling_rate } => {
            create_sa_runner(&task.problem, cooling_rate)
        }
        Algorithm::ACO => Err(CreateError::NotImplemented),
    }
}

pub fn create_oneplusone_runner(
    problem: &Problem,
) -> Result<Box<dyn EvolutionaryAlgorithm + Send>, CreateError> {
    Ok(match problem {
        Problem::OneMax { bitstring_size } => Box::new(OnePlusOneEA::new(
            *bitstring_size,
            Bitflip,
            OneMax,
            &mut rng(),
        )),
        Problem::LeadingOnes { bitstring_size } => Box::new(OnePlusOneEA::new(
            *bitstring_size,
            Bitflip,
            LeadingOnes,
            &mut rng(),
        )),
        Problem::TSP { tsp_instance } => {
            let tsp = TSP::from_euc2d(&tsp_instance).ok_or(CreateError::InvalidTSP)?;
            Box::new(OnePlusOneEA::new(tsp.num_cities(), TwoOpt, tsp, &mut rng()))
        }
    })
}

pub fn create_sa_runner(
    problem: &Problem,
    cooling_rate: f64,
) -> Result<Box<dyn EvolutionaryAlgorithm + Send>, CreateError> {
    Ok(match problem {
        Problem::OneMax { bitstring_size } => {
            let c = DefaultBitstringSchedule::new(*bitstring_size as u64, cooling_rate);
            Box::new(SimulatedAnnealing::new(
                *bitstring_size,
                SingleBitflip,
                OneMax,
                c,
                &mut rng(),
            ))
        }
        Problem::LeadingOnes { bitstring_size } => {
            let c = DefaultBitstringSchedule::new(*bitstring_size as u64, cooling_rate);
            Box::new(SimulatedAnnealing::new(
                *bitstring_size,
                SingleBitflip,
                LeadingOnes,
                c,
                &mut rng(),
            ))
        }
        Problem::TSP { tsp_instance } => {
            let tsp = TSP::from_euc2d(&tsp_instance).ok_or(CreateError::InvalidTSP)?;
            let c = DefaultTSPSchedule::new(tsp.num_cities() as u64, cooling_rate);
            Box::new(SimulatedAnnealing::new(
                tsp.num_cities(),
                TwoOpt,
                tsp,
                c,
                &mut rng(),
            ))
        }
    })
}
