use super::search_space::{Bitstring, SearchSpace};

pub trait FitnessFunction<T: SearchSpace> {
    fn evaluate(&self, instance: &T) -> f64;
    fn is_maximizing(&self) -> bool;

    fn compare(&self, a: f64, b: f64) -> std::cmp::Ordering {
        if self.is_maximizing() {
            a.total_cmp(&b)
        } else {
            a.total_cmp(&b).reverse()
        }
    }
}

pub struct OneMax;

impl FitnessFunction<Bitstring> for OneMax {
    fn evaluate(&self, instance: &Bitstring) -> f64 {
        return instance
            .bits()
            .iter()
            .fold(0_f64, |acc, b| if *b { acc + 1_f64 } else { acc });
    }
    fn is_maximizing(&self) -> bool {
        true
    }
}

//TODO: struct LeadingOnes;
//TODO: struct TSP {}
// determine how we want to store distances (probably distance matrix to support non-euclidian
// instances
// impl FitnessFunction<Permutation> for TSP {}
// impl TSP { from_EUC2D() -> TSP }

#[cfg(test)]
mod tests {
    use super::*;

    impl Bitstring {
        fn from_bitstring(bitstring: &str) -> Self {
            Bitstring::new(bitstring.chars().map(|c| match c {
                '0' => false,
                '1' => true,
                _ => panic!("invalid character: {}", c),
            })
            .collect())
        }
    }

    #[test]
    fn test_one_max() {
        // (bitstring, ones)
        let testcases = vec![
            ("0000", 0.0),
            ("1001010101", 5.0),
            ("1111111111", 10.0),
            ("1111100000", 5.0),
            ("0000011111", 5.0),
        ];

        for t in testcases {
            let b = Bitstring::from_bitstring(t.0);
            let got = OneMax::evaluate(&OneMax, &b);
            assert_eq!(got, t.1, "expected fitness of {} but got {} on \"{}\"", t.1, got, t.0);
        }
    }
}
