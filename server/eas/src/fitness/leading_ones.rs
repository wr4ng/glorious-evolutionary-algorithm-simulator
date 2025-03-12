use super::FitnessFunction;
use crate::search_space::Bitstring;

pub struct LeadingOnes;

impl FitnessFunction<Bitstring> for LeadingOnes {
    fn evaluate(&self, instance: &Bitstring) -> f64 {
        let mut leading_ones = 0;
        for &b in instance.bits() {
            if b {
                leading_ones += 1;
            } else {
                break;
            }
        }
        leading_ones as f64
    }
    fn is_maximizing(&self) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_leading_ones() {
        // (bitstring, fitness)
        let testcases = vec![
            ("0000", 0.0),
            ("1001010101", 1.0),
            ("1111111111", 10.0),
            ("1111100000", 5.0),
            ("0000011111", 0.0),
        ];

        for t in testcases {
            let b = Bitstring::from_bitstring(t.0).unwrap();
            let got = LeadingOnes::evaluate(&LeadingOnes, &b);
            assert_eq!(
                got, t.1,
                "expected fitness of {} but got {} on \"{}\"",
                t.1, got, t.0
            );
        }
    }
}
