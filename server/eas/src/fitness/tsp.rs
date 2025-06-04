use super::FitnessFunction;
use crate::search_space::Permutation;

pub struct TSP {
    distances: Vec<Vec<f64>>,
    vertices: u64,
}

impl FitnessFunction<Permutation> for TSP {
    fn evaluate(&self, instance: &Permutation) -> f64 {
        let mut tour_length = 0.0;
        let p = instance.permutation();
        for pair in p.windows(2) {
            tour_length += self.distances[pair[0]][pair[1]];
        }
        // Add last edge (end -> start)
        tour_length += self.distances[p[p.len() - 1]][p[0]];
        tour_length
    }

    fn is_maximizing(&self) -> bool {
        false
    }
}

impl TSP {
    pub fn new(distances: Vec<Vec<f64>>) -> Self {
        let vertices = distances.len() as u64;
        TSP {
            distances,
            vertices,
        }
    }

    pub fn num_vertices(&self) -> u64 {
        self.vertices
    }

    pub fn num_cities(&self) -> usize {
        self.distances.len()
    }

    pub fn distances(&self) -> Vec<Vec<f64>> {
        self.distances.clone()
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
                let mut parts = l.split_whitespace();
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

        let coords = coords?;

        let mut distances = vec![vec![0.0; coords.len()]; coords.len()];

        for i in 0..coords.len() {
            for j in 0..coords.len() {
                let (_, x1, y1) = coords[i];
                let (_, x2, y2) = coords[j];
                let d = ((x1 - x2).powi(2) + (y1 - y2).powi(2)).sqrt();
                distances[i][j] = d.round();
            }
        }
        Some(TSP::new(distances))
    }
}

#[cfg(test)]
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

    #[test]
    fn test_parse_bier127() {
        let berlin52 = include_str!("./bier127.tsp");
        let tsp = TSP::from_euc2d(&berlin52);
        assert!(tsp.is_some());
    }

    #[test]
    fn test_bier127_optimal_tour() {
        let tsp = TSP::from_euc2d(include_str!("./bier127.tsp")).unwrap();
        let optimal_tour = vec![
            48, 52, 47, 117, 45, 93, 111, 110, 106, 126, 92, 94, 122, 96, 97, 31, 28, 27, 121, 32,
            24, 25, 37, 38, 41, 33, 42, 39, 34, 36, 35, 40, 13, 11, 29, 26, 30, 79, 78, 76, 17, 20,
            16, 19, 107, 14, 105, 5, 113, 104, 6, 0, 15, 1, 50, 43, 102, 44, 53, 56, 120, 55, 123,
            51, 4, 49, 114, 12, 119, 9, 99, 63, 57, 90, 60, 61, 58, 59, 115, 89, 2, 10, 8, 23, 22,
            3, 21, 18, 71, 7, 66, 72, 73, 67, 70, 69, 68, 74, 75, 77, 116, 83, 80, 125, 81, 82,
            100, 101, 62, 118, 95, 108, 87, 86, 85, 84, 109, 103, 124, 88, 91, 98, 64, 112, 65, 54,
            46,
        ];
        let optimal_permutation = Permutation::new(optimal_tour);

        let optimal_fitness = tsp.evaluate(&optimal_permutation);
        assert_eq!(optimal_fitness, 118282.0);
    }
}
