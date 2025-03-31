use eas::{
    algorithms::{EvolutionaryAlgorithm, one_plus_one_ea::OnePlusOneEA},
    fitness::{leading_ones::LeadingOnes, one_max::OneMax, tsp::TSP},
    mutation::{Bitflip, TwoOpt},
};
use rand::rng;

use crate::{Algorithm, CreateTaskRequest, Problem};

pub fn create_ea(request: CreateTaskRequest) -> Option<Box<dyn EvolutionaryAlgorithm + Send>> {
    match request.problem {
        Problem::OneMax => match request.algorithm {
            Algorithm::OnePlusOneEA => Some(Box::new(OnePlusOneEA::new(
                request.bitstring_size? as usize,
                Bitflip,
                OneMax,
                &mut rng(),
            ))),
            Algorithm::SimulatedAnnealing => todo!(),
            Algorithm::ACO => todo!(),
        },
        Problem::LeadingOnes => match request.algorithm {
            Algorithm::OnePlusOneEA => Some(Box::new(OnePlusOneEA::new(
                request.bitstring_size? as usize,
                Bitflip,
                LeadingOnes,
                &mut rng(),
            ))),
            Algorithm::SimulatedAnnealing => todo!(),
            Algorithm::ACO => todo!(),
        },
        Problem::TSP => {
            let tsp = TSP::from_euc2d(&request.tsp_instance?)?;
            match request.algorithm {
                Algorithm::OnePlusOneEA => Some(Box::new(OnePlusOneEA::new(
                    tsp.num_cities(),
                    TwoOpt,
                    tsp,
                    &mut rng(),
                ))),
                Algorithm::SimulatedAnnealing => todo!(),
                Algorithm::ACO => todo!(),
            }
        }
    }
}
