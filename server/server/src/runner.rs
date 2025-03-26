use eas::{
    algorithms::{EvolutionaryAlgorithm, one_plus_one_ea::OnePlusOneEA},
    fitness::{leading_ones::LeadingOnes, one_max::OneMax, tsp::TSP},
    mutation::{Bitflip, TwoOpt},
    search_space::{Bitstring, Permutation},
};
use rand::{Rng, rng};

use crate::{Algorithm, CreateTaskRequest, Problem};

pub enum Runner {
    OnePlusOneOneMax(OnePlusOneEA<Bitstring, OneMax, Bitflip>),
    OnePlusOneTSPTwoOpt(OnePlusOneEA<Permutation, TSP, TwoOpt>),
    OnePlusOneLeadingOnes(OnePlusOneEA<Bitstring, LeadingOnes, Bitflip>),
}

impl Runner {
    pub fn create(request: CreateTaskRequest) -> Option<Self> {
        match request.problem {
            Problem::OneMax => match request.algorithm {
                Algorithm::OnePlusOneEA => Some(Self::OnePlusOneOneMax(OnePlusOneEA::new(
                    request.bitstring_size? as usize,
                    Bitflip,
                    OneMax,
                    &mut rng(),
                ))),
                Algorithm::SimulatedAnnealing => todo!(),
                Algorithm::ACO => todo!(),
            },
            Problem::LeadingOnes => match request.algorithm {
                Algorithm::OnePlusOneEA => Some(Self::OnePlusOneLeadingOnes(OnePlusOneEA::new(
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
                    Algorithm::OnePlusOneEA => Some(Self::OnePlusOneTSPTwoOpt(OnePlusOneEA::new(
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

    pub fn iterate<R: Rng>(&mut self, r: &mut R) {
        match self {
            Runner::OnePlusOneOneMax(ea) => {
                ea.iterate(r);
            }
            Runner::OnePlusOneTSPTwoOpt(ea) => {
                ea.iterate(r);
            }
            Runner::OnePlusOneLeadingOnes(ea) => {
                ea.iterate(r);
            }
        };
    }

    pub fn current_fitness(&self) -> f64 {
        match self {
            Runner::OnePlusOneOneMax(ea) => ea.state.current_fitness,
            Runner::OnePlusOneTSPTwoOpt(ea) => ea.state.current_fitness,
            Runner::OnePlusOneLeadingOnes(ea) => ea.state.current_fitness,
        }
    }
}
