use axum::{Json, Router, extract::State, http::StatusCode, routing::get};
use eas::{algorithms::{one_plus_one_ea::OnePlusOneEA, EvolutionaryAlgorithm}, fitness::one_max::OneMax, mutation::Bitflip};
use serde::{Deserialize, Serialize};
use std::{
    collections::VecDeque,
    sync::{Arc, RwLock},
    thread,
};
use uuid::Uuid;

#[derive()]
struct AppState {
    queue: RwLock<VecDeque<Task>>,
    finished: RwLock<Vec<Result>>,
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
) -> StatusCode {
    let task = Task {
        id: Uuid::new_v4(),
        algorithm: request.algorithm,
        problem: request.problem,
        stop_cond: request.stop_cond,
        task_param: request.task_param,
        parameters: request.parameters,
        body: request.body,
    };

    state
        .queue
        .write()
        .expect("RWLock is poisoned")
        .push_back(task);

    thread::spawn(move || {
        let mut ea = OnePlusOneEA::new(1000, Bitflip, OneMax, rand::rng());
        println!("initial: {}", ea.state.current_fitness);
        for _ in 0..100_000 {
            ea.iterate(&mut rand::rng());
        }
        println!("result: {}", ea.state.current_fitness);
    });

    StatusCode::CREATED
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

#[derive(Deserialize, Serialize, Clone)]
enum Algorithm {
    OnePlusOneEA,
    SimulatedAnnealing,
    ACO,
}
#[derive(Deserialize, Serialize, Clone)]
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
struct Result {
    id: usize,
    algorithm: Algorithm,
    problem: Problem,
}

#[derive(Serialize)]
struct TasksReturn {
    queued: Vec<Task>,
    finished: Vec<Result>,
}
