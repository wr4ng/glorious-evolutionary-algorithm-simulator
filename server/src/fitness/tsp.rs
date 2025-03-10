use super::FitnessFunction;
use crate::search_space::Permutation;

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
        // Add last edge (end -> start)
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
                let d = ((x1 - x2).powi(2) + (y1 - y2).powi(2)).sqrt();
                distances[i][j] = d.round();
            }
        }
        Some(TSP::new(distances))
    }
}

fn parse_tour(input: &str) -> Option<Vec<usize>> {
    //TODO: Read values from metadata section. Verify Tour type
    let (_metadata, remaning) = match input.split_once("TOUR_SECTION\n") {
        None => return None,
        Some((l, r)) => (l, r),
    };

    let data = match remaning.split_once("\n-1\nEOF") {
        None => return None,
        Some((l, _)) => l,
    };

    data.lines()
        .map(|l| l.parse::<usize>().ok().map(|v| v - 1))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_berlin52() {
        let berlin52 = include_str!("./berlin52.tsp");
        let tsp = TSP::from_euc2d(&berlin52);
        assert!(tsp.is_some());
    }

    #[test]
    fn test_berlin52_optimal_tour() {
        let tsp = TSP::from_euc2d(include_str!("./berlin52.tsp")).unwrap();
        let optimal_tour = parse_tour(include_str!("./berlin52.opt.tour")).unwrap();
        let optimal_permutation = Permutation::new(optimal_tour);

        let optimal_fitness = tsp.evaluate(&optimal_permutation);
        assert_eq!(optimal_fitness, 7542.0);
    }
}
