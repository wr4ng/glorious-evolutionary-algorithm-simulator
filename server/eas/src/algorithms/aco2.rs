use std::vec;

use super::{EvolutionaryAlgorithmCore, SimulationState};
use crate::{
    fitness::FitnessFunction,
    rng::MyRng,
    search_space::{Bitstring, SearchSpace}
};

pub struct ACO2<S: SearchSpace, F: FitnessFunction<S>, T: Tau<S, F, O>, E: Eta, C: Constructor<S>, O: Translator<S>> {
    pub state: SimulationState<S>,
	fitness_function: F,
	pheromone: T,
    heuristic: E,
    constructor: C,
    translator: O,
    size: usize,
    ants: usize
}

impl<S, F, T, E, C, O> ACO2<S, F, T, E, C, O>
where 
    S: SearchSpace,
    F: FitnessFunction<S>,
    T: Tau<S, F, O>,
    E: Eta,
    C: Constructor<S>,
    O: Translator<S>
{
    pub fn new<R: MyRng>(size: usize, fitness_function: F, pheromone: T, heuristic: E, constructor: C, translator: O, ants: usize, mut rng: R) -> Self {
        let current_solution = S::new_random(size, &mut rng);
        let current_fitness = fitness_function.evaluate(&current_solution);
        ACO2 {
            state: SimulationState {
                iteration: 0,
                current_solution,
                current_fitness,
            },
            fitness_function,
            pheromone,
            heuristic,
            constructor,
            translator,
            size,
            ants,
        }
    }
}

impl <F, S, T, E, C, O>EvolutionaryAlgorithmCore for ACO2<S, F, T, E, C, O>
where 
    F: FitnessFunction<S>,
    S: SearchSpace,
    T: Tau<S,F,O>,
    E: Eta,
    C: Constructor<S>,
    O: Translator<S>,
{
    fn iterate<R: MyRng>(&mut self, rng: &mut R){
        let mut paths = vec![vec![0; self.size]; self.ants];
        for ant in 0..self.ants{
            paths[ant] = self.constructor.construct(self.pheromone.tau(), self.heuristic.eta(), false, rng);
        }
        self.pheromone.update(&paths, &self.fitness_function, &self.translator);
        //TODO update state
    }
    
    fn current_fitness(&self) -> f64 {
        self.state.current_fitness
    }
    
    fn iterations(&self) -> u64 {
        self.state.iteration
    }
    
    fn status_json(&self) -> serde_json::Value {
        todo!()
    }
}

pub struct Graph {
    graph: Vec<Vec<(usize, f64)>>,
}

impl Graph {
    fn len(&self) -> usize {
        self.graph.len()
    }

    fn weights_at(&self, node: usize) -> Vec<f64>{
        let mut weights = Vec::<f64>::with_capacity(self.graph[node].len());
        for i in 0..self.graph[node].len(){
            weights[i] = self.graph[node][i].1
        }
        weights
    }
}

pub trait Tau<S,F, T>
where
    S: SearchSpace,
    F: FitnessFunction<S>,
    T: Translator<S>,
    {
    fn init(len: usize, wid: usize, undirected: bool, evap_factor: f64, elitism_value: usize) -> Self;
    fn init_from_graph(graph: &Graph, undirected: bool, evap_factor: f64, elitism_value: usize) -> Self;
    fn apply(&mut self, path: &Vec<usize>, fitness: f64);
    fn decrease(&mut self);
    fn update(&mut self, paths: &Vec<Vec<usize>>, fitness_function: &F, translator: &T);
    fn tau(&self) -> &Vec<Vec<(usize, f64)>>;
}

pub struct AS{
    pheromones: Vec<Vec<(usize, f64)>>,
    best: Vec<(Vec<usize>,f64)>,
    undirected: bool,
    evap_factor: f64,
}

impl <S,F,T> Tau<S,F,T> for AS
where 
    S: SearchSpace,
    F: FitnessFunction<S>,
    T: Translator<S>,
{
    fn init(len: usize, wid: usize, undirected: bool, evap_factor: f64, elitism_value: usize) -> Self {
        let mut p = vec![vec![(0,0.0); wid]; len];
        for i in 0..len{
            for ii in 0..wid{
                p[i][ii] = (ii, 0.0);
            }
        }
        let best = vec![(vec![0; wid], f64::MIN); elitism_value];
        AS {pheromones: p, best: best, undirected, evap_factor}
    }
    
    fn init_from_graph(g: &Graph, undirected: bool, evap_factor: f64, elitism_value: usize) -> Self {
        let mut pheromones = Vec::<Vec::<(usize, f64)>>::new();
        for node in 0..g.len(){
            let edges = Vec::<(usize, f64)>::with_capacity(g.graph[node].len());
            pheromones.push(edges);
        }
        let mut best = Vec::<(Vec::<usize>, f64)>::with_capacity(elitism_value);
        for i in 0..elitism_value{
            best[i] = (Vec::<usize>::with_capacity(g.len()),f64::MIN)
        }

        AS { pheromones, best, undirected, evap_factor}
    }
    
    fn apply(&mut self, path: &Vec<usize>, fitness: f64) {
        let val = 1.0/fitness;
        for node in 0.. path.len(){
            self.pheromones[node][path[node]].1 += val;
            if self.undirected{
                self.pheromones[path[node]][node].1 += val;
            }
        }
    }
    
    fn tau(&self) -> &Vec<Vec<(usize, f64)>> {
        &self.pheromones
    }
    
    fn update(&mut self, paths: &Vec<Vec<usize>>, fitness_function: &F, translator: &T) {
        for path in paths{
            let fitness = fitness_function.evaluate(&translator.translate(&path));
            if fitness > self.best[self.best.len()-1].1{
                let mut i = self.best.len() -1;
                while self.best[i].1 < fitness {
                    i -= 1;
                }
                for ii in self.best.len()-1..i{
                    self.best[ii].1 = self.best[ii-1].1;
                }
                self.best[i] = (path.clone(), fitness);
            }
            // TODO this might have to be borrowed
            for b in &self.best{
                let val = 1.0/b.1;
                for node in 0.. path.len(){
                    self.pheromones[node][path[node]].1 += val;
                    if self.undirected{
                        self.pheromones[path[node]][node].1 += val;
                    }
                }
            }
        };
    }

    fn decrease(&mut self) {
        for i in 0..self.pheromones.len(){
            for ii in 0..self.pheromones[i].len(){
                self.pheromones[i][ii].1 *= 1.0 - self.evap_factor;
            }
        }
    }
}

pub trait Eta {
    fn init_from_size(len: usize, wid: usize) -> Self;
    fn init_from_graph(graph: &Graph, is_constant: bool) -> Self;
    fn eta(&self) -> &Vec<Vec<f64>>;
}

pub struct Heuristic{
    heuristic: Vec<Vec<f64>>,
}

impl Eta for Heuristic{
    fn init_from_size(len: usize, wid: usize) -> Self {
        Heuristic {heuristic: vec![vec![1.0/(wid as f64); wid]; len] }
    }

    fn init_from_graph(g: &Graph, is_constant: bool) -> Self {
        let mut heuristic = Vec::<Vec::<f64>>::new();
        for node in 0..g.len(){
            let constant = 1.0/(g.graph[node].len() as f64);
            let edges = if is_constant {vec![constant; g.graph[node].len()]} else {g.weights_at(node)};
            heuristic.push(edges);
        }
        Heuristic { heuristic }
    }

    fn eta(&self) -> &Vec<Vec<f64>> {
        &self.heuristic
    }
}

pub trait Constructor<S>
where
    S: SearchSpace,
{
    fn construct<R: MyRng>(&self, tau: &Vec<Vec<(usize, f64)>>, eta: &Vec<Vec<f64>>, unique_visits: bool, rng: &mut R) -> Vec<usize>;
}

pub struct PartialSumConstructor {}

impl<S> Constructor<S> for PartialSumConstructor
where
    S: SearchSpace
{
    fn construct<R: MyRng>(&self, tau: &Vec<Vec<(usize, f64)>>, eta: &Vec<Vec<f64>>, unique_visits: bool, rng: &mut R) -> Vec<usize> {
        let length = tau.len();
        let mut path = Vec::<usize>::with_capacity(length);
        let mut visited = vec![false; length];
        let mut neighbors = Vec::<usize>::with_capacity(length+1);
        let mut neighbor_weights = Vec::<f64>::with_capacity(length+1);
        
        for step in 0..length {
            let mut added = 1; // sat to 1 to avoid index out of bounds for first element
            for i in 0..tau[step].len() {
                if unique_visits && visited[i] { continue; }
                neighbors[added] = tau[step][i].0;
                neighbor_weights[added] = tau[step][i].1 + eta[step][i] + neighbor_weights[added -1]; // previous weight to achieve partial sum
                added += 1;
            }
            let total_weight = neighbor_weights[added - 1]; // total weight is equal to last element in partial sum
            let choice = rng.random_range_float(0.0..1.0);
            let mut iter = 1;
            while neighbor_weights[iter] / total_weight < choice {
                iter += 1
            }
            path[step] = neighbors[iter];
            if unique_visits { visited[neighbors[iter]] = true}
        }
        path
    }
}

pub trait Translator<S>
where
    S: SearchSpace
{
    fn translate(&self, path: &Vec<usize>) -> S;
}

pub struct Temp{}

impl Translator<Bitstring> for Temp{
    fn translate(&self, path: &Vec<usize>) -> Bitstring{
        let mut translated = Vec::<bool>::with_capacity(path.len());
        for i in 0..path.len(){
            translated[i] = path[i] == 1;
        }
        Bitstring::new(translated)
    }
}