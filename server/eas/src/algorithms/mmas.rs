use super::{AlgorithmCore, SimulationState};
use crate::{
    fitness::FitnessFunction,
    rng::MyRng,
    search_space::{Bitstring, Permutation, SearchSpace},
};
use serde_json::json;
use std::{usize, vec};

pub struct MMAStsp<F: FitnessFunction<Permutation>> {
    pub state: SimulationState<Permutation>,
    fitness_function: F,
    pheromone: Vec<Vec<f64>>,
    heuristic: Vec<Vec<f64>>,
    size: usize,
    ants: usize,
    alpha: f64,
    beta: f64,
    evap_factor: f64,
    update_strategy: PheromoneUpdateStrategy,
    t_min: f64,
    t_max: f64,
    q: f64,
    probabilities: Vec<Vec<f64>>,
}
impl<F> MMAStsp<F>
where
    F: FitnessFunction<Permutation>,
{
    pub fn new<R: MyRng>(
        graph: Vec<Vec<f64>>,
        fitness_function: F,
        size: usize,
        ants: usize,
        alpha: f64,
        beta: f64,
        evap_factor: f64,
        update_strategy: PheromoneUpdateStrategy,
        near_neigh: bool,
        p_best: f64,
        q: f64,
        rng: &mut R,
    ) -> Self {
        let path = if near_neigh {
            Self::nearest_neighbor(&graph, size)
        } else {
            Permutation::new_random(size, rng)
        };
        let current_solution = path;
        let current_fitness = fitness_function.evaluate(&current_solution);

        let (t_min, t_max) = if p_best == 0.0 {
            (1.0 / ((size * size) as f64), 1.0 - 1.0 / (size as f64))
        } else {
            (
                1.0 / (evap_factor) * 1.0 / current_fitness
                    * (1.0 - p_best.powf(1.0 / size as f64))
                    / (((size as f64) / 2.0 - 1.0) * p_best.powf(1.0 / size as f64)),
                1.0 / (evap_factor) * 1.0 / current_fitness,
            )
        };

        let pheromone = vec![vec![t_max; size]; size];
        let mut heuristic = vec![vec![0.0; size]; size];
        for i in 0..size {
            for ii in i..size {
                let val = 1.0 / graph[i][ii];
                heuristic[i][ii] = val;
                heuristic[ii][i] = val;
            }
        }
        let probabilities = vec![vec![0.0;size];size];
        MMAStsp {
            state: SimulationState {
                iteration: 0,
                current_solution,
                current_fitness,
            },
            fitness_function,
            pheromone,
            heuristic,
            size,
            ants,
            alpha,
            beta,
            evap_factor,
            update_strategy,
            t_min,
            t_max,
            q,
            probabilities,
        }
    }

    fn nearest_neighbor(graph: &Vec<Vec<f64>>, size: usize) -> Permutation {
        let mut path = Vec::<usize>::with_capacity(size);
        let mut visited = vec![false; size];
        visited[0] = true;
        let mut neighbors = &graph[0];
        while path.len() != size {
            let mut min_neighbor = (f64::MAX, 0); //TODO
            for i in 0..neighbors.len() {
                if visited[i] {
                    continue;
                }
                if neighbors[i] < min_neighbor.0 {
                    min_neighbor = (neighbors[i], i)
                }
            }
            visited[min_neighbor.1] = true;
            neighbors = &graph[min_neighbor.1];
            path.push(min_neighbor.1);
        }

        Permutation::new(path)
    }

    fn probability(&mut self, i: usize, j: usize) -> f64{
        if self.probabilities[i][j] == 0.0 {
            self.probabilities[i][j] = self.pheromone[i][j].powf(self.alpha)
            * self.heuristic[i][j].powf(self.beta);
            self.probabilities[j][i] = self.probabilities[i][j];
        }
        self.probabilities[i][j]
    }

    fn construct<R: MyRng>(&mut self, rng: &mut R) -> Permutation {
        let start_node = rng.random_range(0..self.size);
        let mut path = Vec::<usize>::with_capacity(self.size);
        let mut visited = vec![false; self.size];
        let mut neighbors = vec![0; self.size];
        let mut neighbor_weights = vec![0.0; self.size];

        let mut current_node = start_node;
        visited[current_node] = true;
        path.push(current_node);

        for _ in 0..self.size - 1 {
            let mut added = 1;
            for i in 0..self.size {
                if visited[i] {
                    continue;
                }
                neighbors[added] = i;
                neighbor_weights[added] = self.probability(current_node, i)
                    + neighbor_weights[added - 1];
                added += 1;
            }
            let total_weight = neighbor_weights[added - 1];
            let choice = rng.random_range_float(0.0..1.0);
            let mut find = 1;
            while neighbor_weights[find] / total_weight <= choice {
                find += 1
            }
            current_node = neighbors[find];
            path.push(current_node);
            visited[current_node] = true;
        }
        Permutation::new(path)
    }

    fn decrease(&mut self) {
        for i in 0..self.size {
            for ii in 0..self.size {
                self.pheromone[i][ii] = self
                    .t_min
                    .max(self.pheromone[i][ii] * (1.0 - self.evap_factor));
                self.probabilities[i][ii] = 0.0;
            }
        }
    }

    fn update(&mut self, paths: &Vec<Permutation>) {
        let mut generation_best = (f64::MAX, Permutation::new(vec![0]));
        for path in paths {
            // Check if there is a new better solution
            let fit_val = self.fitness_function.evaluate(path);
            if self.fitness_function.compare(fit_val, generation_best.0)
                == std::cmp::Ordering::Greater
            {
                generation_best = (fit_val, path.clone());
            }
        }
        if self
            .fitness_function
            .compare(generation_best.0, self.state.current_fitness)
            == std::cmp::Ordering::Greater
        {
            (self.state.current_fitness, self.state.current_solution) = generation_best.clone();
        }

        match self.update_strategy {
            PheromoneUpdateStrategy::AllAnts => {
                for path in paths {
                    self.apply(path);
                }
            }
            PheromoneUpdateStrategy::GenerationBest => {
                self.apply(&generation_best.1.clone());
            }
            PheromoneUpdateStrategy::BestSoFar => {
                self.apply(&self.state.current_solution.clone());
            }
        }
    }

    fn apply(&mut self, p: &Permutation) {
        let perm = p.permutation();
        let p_val = if self.q == 0.0 {
            self.evap_factor
        } else {
            self.q / self.fitness_function.evaluate(p)
        };
        self.pheromone[perm[0]][perm[perm.len() - 1]] = self
            .t_max
            .min(self.pheromone[perm[0]][perm[perm.len() - 1]] + p_val);
        self.pheromone[perm[perm.len() - 1]][perm[0]] =
            self.pheromone[perm[0]][perm[perm.len() - 1]];
        for i in 1..self.size {
            self.pheromone[perm[i]][perm[i - 1]] =
                self.t_max.min(self.pheromone[perm[i]][perm[i - 1]] + p_val);
            self.pheromone[perm[i - 1]][perm[i]] = self.pheromone[perm[i]][perm[i - 1]]
        }
    }
}

impl<F> AlgorithmCore for MMAStsp<F>
where
    F: FitnessFunction<Permutation>,
{
    fn iterate<R: MyRng>(&mut self, rng: &mut R) {
        let mut paths = Vec::<Permutation>::with_capacity(self.ants);

        // Construct(G,a)
        for _ in 0..self.ants {
            paths.push(self.construct(rng));
        }
        // Update pheromones
        self.decrease();
        self.update(&paths);

        // Update iteration
        self.state.iteration += 1;
    }

    fn current_fitness(&self) -> f64 {
        self.state.current_fitness
    }

    fn iterations(&self) -> u64 {
        self.state.iteration
    }

    fn status_json(&self) -> serde_json::Value {
        json!({
            "iterations": self.state.iteration,
            "current_fitness": self.state.current_fitness,
            "current_solution": self.state.current_solution.to_string(),
            "pheromones": self.pheromone,
            "t_max" : self.t_max,
            "t_min" : self.t_min,
        })
    }
}

pub struct MMASbs<F: FitnessFunction<Bitstring>> {
    pub state: SimulationState<Bitstring>,
    fitness_function: F,
    pheromone: Vec<Vec<f64>>,
    size: usize,
    ants: usize,
    alpha: f64,
    evap_factor: f64,
    t_min: f64,
    t_max: f64,
}
impl<F> MMASbs<F>
where
    F: FitnessFunction<Bitstring>,
{
    pub fn new<R: MyRng>(
        fitness_function: F,
        size: usize,
        ants: usize,
        alpha: f64,
        evap_factor: f64,
        rng: &mut R,
    ) -> Self {
        let path = Bitstring::new_random(size, rng);
        let current_solution = path;
        let current_fitness = fitness_function.evaluate(&current_solution);
        let t_min = 1.0 / (size as f64);
        let t_max = 1.0 - 1.0 / (size as f64);

        let pheromone = vec![vec![0.5; 2]; size];

        MMASbs {
            state: SimulationState {
                iteration: 0,
                current_solution,
                current_fitness,
            },
            fitness_function,
            pheromone,
            size,
            ants,
            alpha,
            evap_factor,
            t_min,
            t_max,
        }
    }

    fn construct<R: MyRng>(&self, rng: &mut R) -> Bitstring {
        let mut path = vec![false; self.size];

        for step in 0..self.size {
            let r = rng.random_range_float(0.0..1.0);
            path[step] = r < self.pheromone[step][0].powf(self.alpha)/(self.pheromone[step][0].powf(self.alpha) + self.pheromone[step][1].powf(self.alpha))
        }
        Bitstring::new(path)
    }

    fn update(&mut self, paths: &Vec<Bitstring>) {
        for path in paths {
            // Check if there is a new better solution
            let fit_val = self.fitness_function.evaluate(path);
            if self
                .fitness_function
                .compare(fit_val, self.state.current_fitness)
                == std::cmp::Ordering::Greater
            {
                self.state.current_fitness = fit_val;
                self.state.current_solution = path.clone();
            }

            // Apply new pheromones
            let bits = self.state.current_solution.bits();
            for i in 0..self.size {
                if bits[i] {
                    self.pheromone[i][0] = ((1.0 - self.evap_factor) * self.pheromone[i][0] + self.evap_factor).min(self.t_max);
                    self.pheromone[i][1] = ((1.0 - self.evap_factor) * self.pheromone[i][1]).max(self.t_min);
                } else {
                    self.pheromone[i][1] = ((1.0 - self.evap_factor) * self.pheromone[i][1] + self.evap_factor).min(self.t_max);
                    self.pheromone[i][0] = ((1.0 - self.evap_factor) * self.pheromone[i][0]).max(self.t_min);
                }
            }
        }
    }
}

impl<F> AlgorithmCore for MMASbs<F>
where
    F: FitnessFunction<Bitstring>,
{
    fn iterate<R: MyRng>(&mut self, rng: &mut R) {
        let mut paths = Vec::<Bitstring>::with_capacity(self.ants);

        // Construct(G,a)
        for _ in 0..self.ants {
            paths.push(self.construct(rng));
        }
        // Daemon action

        // Update pheromones
        self.update(&paths);

        // Update iteration
        self.state.iteration += 1;
    }

    fn current_fitness(&self) -> f64 {
        self.state.current_fitness
    }

    fn iterations(&self) -> u64 {
        self.state.iteration
    }

    fn status_json(&self) -> serde_json::Value {
        json!({
            "iterations": self.state.iteration,
            "current_fitness": self.state.current_fitness,
            "current_solution": self.state.current_solution.to_string()
        })
    }
}

pub enum PheromoneUpdateStrategy {
    BestSoFar,
    GenerationBest,
    AllAnts,
}
