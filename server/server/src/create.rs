use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use eas::{
    algorithms::{
        EvolutionaryAlgorithm,
        one_plus_one_ea::OnePlusOneEA,
        simulated_annealing::{DefaultTSPSchedule, SimulatedAnnealing},
    },
    fitness::{leading_ones::LeadingOnes, one_max::OneMax, tsp::TSP},
    mutation::{Bitflip, SingleBitflip, TwoOpt},
};
use rand::rng;

use crate::{Algorithm, CreateTaskRequest, Problem};

#[derive(Debug)]
pub enum CreateError {
    MissingValue(String),
    InvalidTSP,
}

impl IntoResponse for CreateError {
    fn into_response(self) -> Response {
        match self {
            CreateError::MissingValue(msg) => {
                (StatusCode::BAD_REQUEST, format!("missing value: {msg}"))
            }
            CreateError::InvalidTSP => {
                (StatusCode::BAD_REQUEST, "invalid tsp instance".to_string())
            }
        }
        .into_response()
    }
}

pub fn create_ea(
    request: CreateTaskRequest,
) -> Result<Box<dyn EvolutionaryAlgorithm + Send>, CreateError> {
    match request.problem {
        Problem::OneMax => match request.algorithm {
            Algorithm::OnePlusOneEA => Ok(Box::new(OnePlusOneEA::new(
                request
                    .bitstring_size
                    .ok_or(CreateError::MissingValue("bistring_size".to_string()))?
                    as usize,
                Bitflip,
                OneMax,
                &mut rng(),
            ))),
            Algorithm::SimulatedAnnealing => {
                let c = DefaultTSPSchedule::from_max_iterations(
                    request
                        .bitstring_size
                        .ok_or(CreateError::MissingValue("bistring_size".to_string()))?
                        as u64,
                    request.stop_cond.max_iterations,
                );
                Ok(Box::new(SimulatedAnnealing::new(
                    request
                        .bitstring_size
                        .ok_or(CreateError::MissingValue("bistring_size".to_string()))?
                        as usize,
                    SingleBitflip,
                    OneMax,
                    c,
                    &mut rng(),
                )))
            }
            Algorithm::ACO => todo!(),
        },
        Problem::LeadingOnes => match request.algorithm {
            Algorithm::OnePlusOneEA => Ok(Box::new(OnePlusOneEA::new(
                request
                    .bitstring_size
                    .ok_or(CreateError::MissingValue("bistring_size".to_string()))?
                    as usize,
                Bitflip,
                LeadingOnes,
                &mut rng(),
            ))),
            Algorithm::SimulatedAnnealing => {
                let c = DefaultTSPSchedule::from_max_iterations(
                    request
                        .bitstring_size
                        .ok_or(CreateError::MissingValue("bistring_size".to_string()))?
                        as u64,
                    request.stop_cond.max_iterations,
                );
                Ok(Box::new(SimulatedAnnealing::new(
                    request
                        .bitstring_size
                        .ok_or(CreateError::MissingValue("bistring_size".to_string()))?
                        as usize,
                    SingleBitflip,
                    LeadingOnes,
                    c,
                    &mut rng(),
                )))
            }
            Algorithm::ACO => todo!(),
        },
        Problem::TSP => {
            let tsp = TSP::from_euc2d(
                &request
                    .tsp_instance
                    .ok_or(CreateError::MissingValue("tsp_instance".to_string()))?,
            )
            .ok_or(CreateError::InvalidTSP)?;
            match request.algorithm {
                //TODO: Match on mutator
                Algorithm::OnePlusOneEA => Ok(Box::new(OnePlusOneEA::new(
                    tsp.num_cities(),
                    TwoOpt,
                    tsp,
                    &mut rng(),
                ))),
                Algorithm::SimulatedAnnealing => {
                    let c = DefaultTSPSchedule::from_max_iterations(
                        tsp.num_cities() as u64,
                        request.stop_cond.max_iterations,
                    );
                    Ok(Box::new(SimulatedAnnealing::new(
                        tsp.num_cities(),
                        TwoOpt,
                        tsp,
                        c,
                        &mut rng(),
                    )))
                }
                Algorithm::ACO => todo!(),
            }
        }
    }
}
