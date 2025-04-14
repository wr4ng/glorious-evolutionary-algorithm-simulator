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

use crate::{Algorithm, CreateTaskRequest, Problem, TSPMutator};

#[derive(Debug)]
pub enum CreateError {
    MissingValue(String),
    InvalidTSP,
    NotImplemented,
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
            CreateError::NotImplemented => (StatusCode::BAD_REQUEST, "not implemented".to_string()),
        }
        .into_response()
    }
}

pub fn create_ea(
    request: CreateTaskRequest,
) -> Result<Box<dyn EvolutionaryAlgorithm + Send>, CreateError> {
    match request.algorithm {
        Algorithm::OnePlusOneEA => create_oneplusone_runner(
            request.problem,
            request.bitstring_size,
            request.tsp_instance,
            request.tsp_mutator,
        ),
        Algorithm::SimulatedAnnealing => create_sa_runner(
            request.problem,
            request.bitstring_size,
            request.tsp_instance,
            request.tsp_mutator,
            request.stop_cond.max_iterations,
        ),
        Algorithm::ACO => todo!(),
    }
}

pub fn create_oneplusone_runner(
    problem: Problem,
    bitstring_size: Option<usize>,
    tsp_instance: Option<String>,
    tsp_mutator: Option<TSPMutator>,
) -> Result<Box<dyn EvolutionaryAlgorithm + Send>, CreateError> {
    Ok(match problem {
        Problem::OneMax => {
            let bitstring_size = bitstring_size.ok_or(CreateError::MissingValue("".to_string()))?;
            Box::new(OnePlusOneEA::new(
                bitstring_size,
                Bitflip,
                OneMax,
                &mut rng(),
            ))
        }
        Problem::LeadingOnes => {
            let bitstring_size = bitstring_size.ok_or(CreateError::MissingValue("".to_string()))?;
            Box::new(OnePlusOneEA::new(
                bitstring_size,
                Bitflip,
                LeadingOnes,
                &mut rng(),
            ))
        }
        Problem::TSP => {
            let tsp = TSP::from_euc2d(
                &tsp_instance.ok_or(CreateError::MissingValue("tsp_instance".to_string()))?,
            )
            .ok_or(CreateError::InvalidTSP)?;
            let mutator =
                tsp_mutator.ok_or(CreateError::MissingValue("tsp_mutator".to_string()))?;
            match mutator {
                TSPMutator::TwoOpt => {
                    Box::new(OnePlusOneEA::new(tsp.num_cities(), TwoOpt, tsp, &mut rng()))
                }
                TSPMutator::ThreeOpt => return Err(CreateError::NotImplemented),
            }
        }
    })
}

pub fn create_sa_runner(
    problem: Problem,
    bitstring_size: Option<usize>,
    tsp_instance: Option<String>,
    tsp_mutator: Option<TSPMutator>,
    max_iterations: u64,
) -> Result<Box<dyn EvolutionaryAlgorithm + Send>, CreateError> {
    Ok(match problem {
        Problem::OneMax => {
            let bitstring_size = bitstring_size.ok_or(CreateError::MissingValue("".to_string()))?;
            let c = DefaultBitstringSchedule::from_max_iterations(
                bitstring_size as u64,
                max_iterations,
            );
            Box::new(SimulatedAnnealing::new(
                bitstring_size,
                SingleBitflip,
                OneMax,
                c,
                &mut rng(),
            ))
        }
        Problem::LeadingOnes => {
            let bitstring_size = bitstring_size.ok_or(CreateError::MissingValue("".to_string()))?;
            let c = DefaultBitstringSchedule::from_max_iterations(
                bitstring_size as u64,
                max_iterations,
            );
            Box::new(SimulatedAnnealing::new(
                bitstring_size,
                SingleBitflip,
                LeadingOnes,
                c,
                &mut rng(),
            ))
        }
        Problem::TSP => {
            let tsp = TSP::from_euc2d(
                &tsp_instance.ok_or(CreateError::MissingValue("tsp_instance".to_string()))?,
            )
            .ok_or(CreateError::InvalidTSP)?;
            let c =
                DefaultTSPSchedule::from_max_iterations(tsp.num_cities() as u64, max_iterations);
            let mutator =
                tsp_mutator.ok_or(CreateError::MissingValue("tsp_mutator".to_string()))?;
            match mutator {
                TSPMutator::TwoOpt => Box::new(SimulatedAnnealing::new(
                    tsp.num_cities(),
                    TwoOpt,
                    tsp,
                    c,
                    &mut rng(),
                )),
                TSPMutator::ThreeOpt => return Err(CreateError::NotImplemented),
            }
        }
    })
}
