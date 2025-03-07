use super::search_space::{Bitstring, Permutation, SearchSpace};

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
            .fold(0_f64, |acc, &b| if b { acc + 1_f64 } else { acc });
    }
    fn is_maximizing(&self) -> bool {
        true
    }
}

struct LeadingOnes;

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

pub struct TSP {
    distances: Vec<Vec<f64>>,
}

impl FitnessFunction<Permutation> for TSP {
    fn evaluate(&self, instance: &Permutation) -> f64 {
        let mut tour_length = 0.0;
        let p = instance.permutation();
        for pair in p.windows(2) {
            tour_length += self.distances[pair[0]][pair[1]];
        }
        // Add last distance
        tour_length += self.distances[p[p.len() - 1]][0];

        tour_length
    }

    fn is_maximizing(&self) -> bool {
        false
    }
}

impl TSP {
    pub fn new(distances: Vec<Vec<f64>>) -> Self {
        TSP { distances }
    }

    //TODO: Should probably be Result<TSP, TSPParseError> or something
    pub fn from_euc2d(input: &str) -> Option<TSP> {
        //TODO: Read values from metadata section. Verify EUC2D type
        let (_metadata, data) = match input.split_once("NODE_COORD_SECTION\n") {
            None => return None,
            Some((l, r)) => (l, r),
        };

        let coords_str = match data.split_once("\nEOF") {
            None => return None,
            Some((l, _)) => l,
        };

        let coords = coords_str
            .lines()
            .map(|l| {
                let mut parts = l.split(' ');
                let (index, x, y) = match (parts.next(), parts.next(), parts.next()) {
                    (Some(a), Some(b), Some(c)) => (a, b, c),
                    _ => return None,
                };
                let (index, x, y) =
                    match (index.parse::<usize>(), x.parse::<f64>(), y.parse::<f64>()) {
                        (Ok(a), Ok(b), Ok(c)) => (a, b, c),
                        _ => return None,
                    };
                Some((index, x, y))
            })
            .collect::<Option<Vec<_>>>();

        let Some(coords) = coords else {
            return None;
        };

        let mut distances = vec![vec![0.0; coords.len()]; coords.len()];

        for i in 0..coords.len() {
            for j in 0..coords.len() {
                //TODO: Calculate distance based on EUC2D
                let (_, x1, y1) = coords[i];
                let (_, x2, y2) = coords[j];

                let d = ((x1 - x2) / (y1 - y2)).sqrt();
                distances[i][j] = d;
            }
        }
        Some(TSP::new(distances))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl Bitstring {
        fn from_bitstring(bitstring: &str) -> Self {
            Bitstring::new(
                bitstring
                    .chars()
                    .map(|c| match c {
                        '0' => false,
                        '1' => true,
                        _ => panic!("invalid character: {}", c),
                    })
                    .collect(),
            )
        }
    }

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
            let b = Bitstring::from_bitstring(t.0);
            let got = OneMax::evaluate(&OneMax, &b);
            assert_eq!(
                got, t.1,
                "expected fitness of {} but got {} on \"{}\"",
                t.1, got, t.0
            );
        }
    }

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
            let b = Bitstring::from_bitstring(t.0);
            let got = LeadingOnes::evaluate(&LeadingOnes, &b);
            assert_eq!(
                got, t.1,
                "expected fitness of {} but got {} on \"{}\"",
                t.1, got, t.0
            );
        }
    }

    #[test]
    fn test_parse_berlin52() {
        let berlin52 = include_str!("./berlin52.tsp");
        let tsp = TSP::from_euc2d(&berlin52);
        assert!(tsp.is_some());
        //TODO: Assert distances using optimal tour
    }
}
