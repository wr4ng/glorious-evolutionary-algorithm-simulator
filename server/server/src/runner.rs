use eas::{
    algorithms::{EvolutionaryAlgorithm, one_plus_one_ea::OnePlusOneEA},
    fitness::{leading_ones::LeadingOnes, one_max::OneMax},
    mutation::Bitflip,
    search_space::Bitstring,
};
use rand::Rng;

pub enum Runner {
    OnePlusOneOneMax(OnePlusOneEA<Bitstring, OneMax, Bitflip>),
    OnePlusOneLeadingOnes(OnePlusOneEA<Bitstring, LeadingOnes, Bitflip>),
}

impl Runner {
    pub fn iterate<R: Rng>(&mut self, r: &mut R) {
        match self {
            Runner::OnePlusOneOneMax(ea) => ea.iterate(r),
            Runner::OnePlusOneLeadingOnes(ea) => ea.iterate(r),
        };
    }

    pub fn current_fitness(&self) -> f64 {
        match self {
            Runner::OnePlusOneOneMax(ea) => ea.state.current_fitness,
            Runner::OnePlusOneLeadingOnes(ea) => ea.state.current_fitness,
        }
    }
}
