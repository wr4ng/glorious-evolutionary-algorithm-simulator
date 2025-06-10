use super::FitnessFunction;
use crate::search_space::Bitstring;

// Implementation of the OneMax pseudo-boolean function
pub struct OneMax;

impl FitnessFunction<Bitstring> for OneMax {
    fn evaluate(&self, instance: &Bitstring) -> f64 {
        instance
            .bits()
            .iter()
            .fold(0_f64, |acc, &b| if b { acc + 1_f64 } else { acc })
    }
    fn is_maximizing(&self) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one_max() {
        // (bitstring, fitness)
        let testcases = vec![
            ("0000", 0.0),
            ("1001010101", 5.0),
            ("1111111111", 10.0),
            ("1111100000", 5.0),
            ("0000011111", 5.0),
        ];

        for t in testcases {
            let b = Bitstring::from_bitstring(t.0).unwrap();
            let got = OneMax::evaluate(&OneMax, &b);
            assert_eq!(
                got, t.1,
                "expected fitness of {} but got {} on \"{}\"",
                t.1, got, t.0
            );
        }
    }
}
