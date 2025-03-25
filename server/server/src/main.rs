use axum::{Json, Router, extract::State, http::StatusCode, routing::get};
use rand::rng;
use runner::Runner;
use serde::{Deserialize, Serialize};
use tower_http::cors::CorsLayer;
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
        .layer(CorsLayer::permissive()) //TODO: Handle CORS
        .route("/tasks", get(get_tasks).post(create_task))
        .with_state(state);

    // run our app with hyper, listening globally on port 3000
    println!("listining at 0.0.0.0:3000...");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn create_task(
    State(state): State<SharedState>,
    Json(request): Json<CreateTaskRequest>,
) -> (StatusCode, Result<(), String>) {
    let id = Uuid::new_v4();

    let task = Task {
        id: id.clone(),
        algorithm: request.algorithm,
        problem: request.problem,
        stop_cond: request.stop_cond.clone(),
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

    let mut runner = match Runner::create(request.clone()) {
        Some(r) => r,
        None => {
            return (
                StatusCode::BAD_REQUEST,
                Err("invalid configuration".to_string()),
            );
        }
    };

    thread::spawn(move || {
        println!("initial: {}", runner.current_fitness());
        //TODO: Use request.stop_condition
        for _ in 0..1_000 {
            runner.iterate(&mut rng());
        }
        println!("result: {}", runner.current_fitness());

        state
            .finished
            .write()
            .expect("RWLock is poisoned")
            .push(TaskResult {
                id,
                algorithm: request.algorithm,
                problem: request.problem,
                final_fitness: runner.current_fitness(),
            });
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

#[derive(Deserialize, Clone)]
struct CreateTaskRequest {
    algorithm: Algorithm,
    problem: Problem,
    bitstring_size: Option<u32>,
    tsp_instance: Option<String>,
    stop_cond: StopCondition,
}

#[derive(Serialize, Clone)]
struct Task {
    id: Uuid,
    algorithm: Algorithm,
    problem: Problem,
    stop_cond: StopCondition,
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
        self.max_time.is_some() || self.max_iterations.is_some()
    }
}

#[derive(Serialize, Clone)]
struct TaskResult {
    id: Uuid,
    algorithm: Algorithm,
    problem: Problem,
    final_fitness: f64,
}

#[derive(Serialize)]
struct TasksReturn {
    queued: Vec<Task>,
    finished: Vec<TaskResult>,
}
