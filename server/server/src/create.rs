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
        Algorithm::OnePlusOneEA => create_oneplusone_runner(request.problem, request.tsp_mutator),
        Algorithm::SimulatedAnnealing { cooling_rate } => {
            create_sa_runner(request.problem, request.tsp_mutator, cooling_rate)
        }
        Algorithm::ACO => Err(CreateError::NotImplemented),
    }
}

pub fn create_oneplusone_runner(
    problem: Problem,
    tsp_mutator: Option<TSPMutator>,
) -> Result<Box<dyn EvolutionaryAlgorithm + Send>, CreateError> {
    Ok(match problem {
        Problem::OneMax { bitstring_size } => Box::new(OnePlusOneEA::new(
            bitstring_size,
            Bitflip,
            OneMax,
            &mut rng(),
        )),
        Problem::LeadingOnes { bitstring_size } => Box::new(OnePlusOneEA::new(
            bitstring_size,
            Bitflip,
            LeadingOnes,
            &mut rng(),
        )),
        Problem::TSP { tsp_instance } => {
            let tsp = TSP::from_euc2d(&tsp_instance).ok_or(CreateError::InvalidTSP)?;
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
    tsp_mutator: Option<TSPMutator>,
    cooling_rate: f64,
) -> Result<Box<dyn EvolutionaryAlgorithm + Send>, CreateError> {
    Ok(match problem {
        Problem::OneMax { bitstring_size } => {
            let c = DefaultBitstringSchedule::new(bitstring_size as u64, cooling_rate);
            Box::new(SimulatedAnnealing::new(
                bitstring_size,
                SingleBitflip,
                OneMax,
                c,
                &mut rng(),
            ))
        }
        Problem::LeadingOnes { bitstring_size } => {
            let c = DefaultBitstringSchedule::new(bitstring_size as u64, cooling_rate);
            Box::new(SimulatedAnnealing::new(
                bitstring_size,
                SingleBitflip,
                LeadingOnes,
                c,
                &mut rng(),
            ))
        }
        Problem::TSP { tsp_instance } => {
            let tsp = TSP::from_euc2d(&tsp_instance).ok_or(CreateError::InvalidTSP)?;
            let c = DefaultTSPSchedule::new(tsp.num_cities() as u64, cooling_rate);
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
