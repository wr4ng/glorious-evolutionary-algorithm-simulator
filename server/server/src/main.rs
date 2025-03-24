use axum::{Json, Router, extract::State, http::StatusCode, routing::get};
use eas::{
    algorithms::{EvolutionaryAlgorithm, one_plus_one_ea::OnePlusOneEA},
    fitness::{leading_ones::LeadingOnes, one_max::OneMax},
    mutation::Bitflip,
};
use rand::rng;
use runner::Runner;
use serde::{Deserialize, Serialize};
use std::{
    collections::VecDeque,
    sync::{Arc, RwLock},
    thread,
};
use uuid::Uuid;

mod runner;

#[derive()]
struct AppState {
    queue: RwLock<VecDeque<Task>>,
    finished: RwLock<Vec<TaskResult>>,
}

type SharedState = Arc<AppState>;

#[tokio::main]
async fn main() {
    let state: SharedState = Arc::new(AppState {
        queue: RwLock::new(VecDeque::new()),
        finished: RwLock::new(Vec::new()),
    });

    // build our application with a single route
    let app = Router::new()
        .route("/tasks", get(get_tasks).post(create_task))
        .with_state(state);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn create_task(
    State(state): State<SharedState>,
    Json(request): Json<CreateTaskRequest>,
) -> (StatusCode, Result<(), String>) {
    let task = Task {
        id: Uuid::new_v4(),
        algorithm: request.algorithm,
        problem: request.problem,
        stop_cond: request.stop_cond.clone(),
        task_param: request.task_param,
        parameters: request.parameters,
        body: request.body,
    };

    state
        .queue
        .write()
        .expect("RWLock is poisoned")
        .push_back(task);

    if !request.stop_cond.is_valid() {
        return (
            StatusCode::BAD_REQUEST,
            Err("invalid stop condition".to_string()),
        );
    }

    thread::spawn(move || {
        //TODO: Extract into function that returns Option<Runner> or Result<Runner,
        //ValidationError> or similar
        let mut r: Runner = match request.algorithm {
            Algorithm::OnePlusOneEA => match request.problem {
                Problem::OneMax => {
                    Runner::OnePlusOneOneMax(OnePlusOneEA::new(100, Bitflip, OneMax, rng()))
                }
                Problem::LeadingOnes => Runner::OnePlusOneLeadingOnes(OnePlusOneEA::new(
                    100,
                    Bitflip,
                    LeadingOnes,
                    rng(),
                )),
                Problem::TSP => todo!(),
            },
            Algorithm::SimulatedAnnealing => todo!(),
            Algorithm::ACO => todo!(),
        };
        println!("initial: {}", r.current_fitness());
        for _ in 0..1_000 {
            r.iterate(&mut rng());
        }
        println!("result: {}", r.current_fitness());
    });

    (StatusCode::CREATED, Ok(()))
}

async fn get_tasks(State(state): State<SharedState>) -> (StatusCode, Json<TasksReturn>) {
    let tasks = state
        .queue
        .read()
        .expect("RWLock is poisoned")
        .clone()
        .into_iter()
        .collect();
    let finished = state.finished.read().expect("RWLock is poisoned").clone();
    let returns = TasksReturn {
        queued: tasks,
        finished,
    };
    (StatusCode::ACCEPTED, Json(returns))
}

#[derive(Deserialize)]
struct CreateTaskRequest {
    algorithm: Algorithm,
    problem: Problem,
    stop_cond: StopCondition,
    task_param: TaskParameters,
    parameters: AlgoParameters,
    body: String,
}

#[derive(Serialize, Clone)]
struct Task {
    id: Uuid,
    algorithm: Algorithm,
    problem: Problem,
    stop_cond: StopCondition,
    task_param: TaskParameters,
    parameters: AlgoParameters,
    body: String,
}

#[derive(Deserialize, Serialize, Clone, Copy)]
enum Algorithm {
    OnePlusOneEA,
    SimulatedAnnealing,
    ACO,
}
#[derive(Deserialize, Serialize, Clone, Copy)]
enum Problem {
    OneMax,
    LeadingOnes,
    TSP,
}

#[derive(Deserialize, Serialize, Clone)]
struct StopCondition {
    max_time: Option<usize>,
    max_iterations: Option<usize>,
    max_iterations_since_improvement: Option<usize>,
    requested_fitness: Option<usize>,
}

impl StopCondition {
    fn is_valid(&self) -> bool {
        return self.max_time.is_some() || self.max_iterations.is_some();
    }
}

#[derive(Deserialize, Serialize, Clone)]
struct TaskParameters {
    repetitions: Option<usize>,
    data_interval: Option<usize>,
    measure_time: Option<bool>,
    save_running_solutions: Option<bool>,
    save_running_fitness: Option<bool>,
}

#[derive(Deserialize, Serialize, Clone)]
struct AlgoParameters {
    //TODO
}

#[derive(Serialize, Clone)]
struct TaskResult {
    id: usize,
    algorithm: Algorithm,
    problem: Problem,
}

#[derive(Serialize)]
struct TasksReturn {
    queued: Vec<Task>,
    finished: Vec<TaskResult>,
}
