use axum::{
    extract::State, routing::{get, post}, Json, Router
};
use schedule::create_task_schedule;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use tokio::sync::broadcast::Sender;
use tower_http::cors::{Any, CorsLayer};
use uuid::Uuid;
use ws::get_websocket_updates;

mod create;
mod schedule;
mod ws;

#[derive()]
struct AppState {
    results: Vec<ScheduleResult>,
    schedule_channels: HashMap<Uuid, Sender<serde_json::Value>>,
}

type SharedState = Arc<Mutex<AppState>>;

#[tokio::main]
async fn main() {
    let state: SharedState = Arc::new(Mutex::new(AppState {
        results: Vec::new(),
        schedule_channels: HashMap::new(),
    }));

    //TODO: Proberly handle CORS
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // build our application with a single route
    let app = Router::new()
        .route("/ping", get(ping_handler))
        .route("/results", get(get_results))
        .route("/schedules", post(create_task_schedule))
        .route("/ws/{id}", get(get_websocket_updates))
        .layer(cors)
        .with_state(state);

    // run our app with hyper, listening globally on port 3000
    println!("listining at 0.0.0.0:3000...");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn ping_handler() -> String {
    "pong".to_owned()
}

async fn get_results(State(state): State<SharedState>) -> Json<Vec<ScheduleResult>> {
    let state = state.lock().expect("failed to aquire mutex");
    let results = state.results.clone();
    Json(results)
}

#[derive(Serialize, Clone, Debug)]
struct ScheduleResult {
    results: Vec<TaskResult>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
struct Task {
    algorithm: Algorithm,
    problem: Problem,
    stop_cond: StopCondition,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
struct StopCondition {
    max_iterations: u64,
    optimal_fitness: Option<f64>,
}

#[derive(Serialize, Clone, Debug)]
struct TaskResult {
    task: Task,
    fitness: f64,
    iterations: u64,
}

#[derive(Deserialize, Serialize, Clone, Copy, Debug)]
#[serde(tag = "type")]
enum Algorithm {
    OnePlusOneEA,
    SimulatedAnnealing { cooling_schedule: CoolingSchedule },
    ACO,
}

#[derive(Deserialize, Serialize, Clone, Copy, Debug)]
#[serde(tag = "type")]
enum CoolingSchedule {
    Static { temperature: f64 },
    Exponential { cooling_rate: f64 },
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "type")]
enum Problem {
    OneMax { bitstring_size: usize },
    LeadingOnes { bitstring_size: usize },
    TSP { tsp_instance: String },
}
